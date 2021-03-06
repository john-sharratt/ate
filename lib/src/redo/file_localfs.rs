#[allow(unused_imports)]
use log::{error, info, warn, debug};
use async_trait::async_trait;

#[cfg(feature = "caching")]
use cached::Cached;
use tokio::io::{Result};
use tokio::io::ErrorKind;
use bytes::Bytes;
#[cfg(feature = "caching")]
use cached::*;
use fxhash::{FxHashMap};
#[cfg(feature = "caching")]
use parking_lot::Mutex as MutexSync;

use crate::{crypto::*, redo::LogLookup};
use crate::event::*;
use crate::error::*;
use crate::spec::*;
use crate::loader::*;

use super::*;
use super::magic::*;
use super::archive::*;
use super::appender::*;

#[cfg(feature = "caching")]
pub(crate) struct LogFileCache
{
    pub(crate) flush: FxHashMap<AteHash, LoadData>,
    pub(crate) write: TimedSizedCache<AteHash, LoadData>,
    pub(crate) read: TimedSizedCache<AteHash, LoadData>,
}

pub(super) struct LogFileLocalFs
{
    pub(crate) path: String,
    pub(crate) temp: bool,
    pub(crate) lookup: FxHashMap<AteHash, LogLookup>,
    pub(crate) appender: LogAppender,
    pub(crate) archives: FxHashMap<u32, LogArchive>,
    #[cfg(feature = "caching")]
    pub(crate) cache: MutexSync<LogFileCache>,
}

impl LogFileLocalFs
{
    pub(super) async fn new(temp_file: bool, path_log: String, truncate: bool, _cache_size: usize, _cache_ttl: u64, header_bytes: Vec<u8>) -> Result<Box<LogFileLocalFs>>
    {
        info!("open at {}", path_log);
        
        // Load all the archives
        let mut archives = FxHashMap::default();
        let mut n = 0 as u32;
        
        loop
        {
            // If the next file does not exist then there are no more archives
            let test = format!("{}.{}", path_log.clone(), n + 1);
            if std::path::Path::new(test.as_str()).exists() == false {
                break;
            }

            // If its a temp file then fail as this would be unsupported behaviour
            if temp_file {
                return Err(tokio::io::Error::new(ErrorKind::AlreadyExists, "Can not start a temporary redo log when there are existing archives."));
            }
            
            // Add the file as pure archive with no appender
            archives.insert(n , LogArchive::new(path_log.clone(), n).await?);
            n = n + 1;
        }

        // Create the log appender
        let (appender, archive) = LogAppender::new(
            path_log.clone(),
            truncate,
            n,
            &header_bytes[..]
        ).await?;
        archives.insert(n, archive);

        // If we are temporary log file then kill the file
        if temp_file {
            let _ = std::fs::remove_file(appender.path());
        }
        
        // Log file
        let ret = LogFileLocalFs {
            path: path_log,
            temp: temp_file,
            lookup: FxHashMap::default(),
            appender,
            #[cfg(feature = "caching")]
            cache: MutexSync::new(LogFileCache {
                flush: FxHashMap::default(),
                read: TimedSizedCache::with_size_and_lifespan(_cache_size, _cache_ttl),
                write: TimedSizedCache::with_size_and_lifespan(_cache_size, _cache_ttl),
            }),
            archives,
        };

        Ok(Box::new(ret))
    }

    /// Read all the log files from all the archives including the current one representing the appender
    pub(super) async fn read_all(&mut self, mut loader: Box<impl Loader>) -> std::result::Result<usize, SerializationError> {
        let mut lookup = FxHashMap::default();

        let archives = self.archives.values_mut().collect::<Vec<_>>();

        let mut total: usize = 0;
        for archive in archives.iter() {
            total = total + archive.len().await? as usize;
        }
        loader.start_of_history(total).await;

        let mut cnt: usize = 0;
        for archive in archives
        {
            let mut lock = archive.lock_at(0).await?;

            let _version = match RedoHeader::read(&mut lock).await? {
                Some(a) => a,
                None => {
                    warn!("log-read-error: log file is empty");
                    continue;
                }
            };

            loop {
                match LogFileLocalFs::read_once_internal(&mut lock).await {
                    Ok(Some(head)) => {
                        #[cfg(feature = "super_verbose")]
                        debug!("log-read: {:?}", head);

                        lookup.insert(head.header.event_hash, head.lookup);

                        loader.feed_load_data(head).await;
                        cnt = cnt + 1;
                    },
                    Ok(None) => break,
                    Err(err) => {
                        debug!("log-load-error: {}", err.to_string());
                        continue;
                    }
                }
            }
        }

        for (v, k) in lookup.into_iter() {
            self.lookup.insert(v, k);
        }

        loader.end_of_history().await;

        Ok(cnt)
    }

    async fn read_once_internal(guard: &mut LogArchiveGuard<'_>) -> std::result::Result<Option<LoadData>, SerializationError>
    {
        let offset = guard.offset();
        
        #[cfg(feature = "super_verbose")]
        info!("log-read-event: offset={}", offset);

        // Read the log event
        let evt = match EventVersion::read(guard).await? {
            Some(e) => e,
            None => {
                return Ok(None);
            }
        };
        
        // Deserialize the meta bytes into a metadata object
        let meta = evt.header.format.meta.deserialize(&evt.meta[..])?;
        let data_hash = match &evt.data {
            Some(a) => Some(AteHash::from_bytes(&a[..])),
            None => None,
        };
        let data_size = match &evt.data {
            Some(a) => a.len(),
            None => 0,
        };
        let data = match evt.data {
            Some(a) => Some(Bytes::from(a)),
            None => None,
        };

        // Record the lookup map
        let header = EventHeaderRaw::new(
            AteHash::from_bytes(&evt.meta[..]),
            Bytes::from(evt.meta),
            data_hash,
            data_size,
            evt.header.format,
        );

        Ok(
            Some(
                LoadData {
                    header,
                    data: EventData {
                        meta: meta,
                        data_bytes: data,
                        format: evt.header.format,
                    },
                    lookup: LogLookup {
                        index: guard.index(),
                        offset,
                    },
                }
            )
        )
    }
}

#[async_trait]
impl LogFile
for LogFileLocalFs
{
    #[cfg(feature = "rotate")]
    async fn rotate(&mut self, header_bytes: Vec<u8>) -> Result<()>
    {
        // If this a temporary file then fail
        if self.temp {
            return Err(tokio::io::Error::new(ErrorKind::PermissionDenied, "Can not rotate a temporary redo log - only persistent logs support this behaviour."));
        }

        // Flush and close and increment the log index
        self.appender.sync().await?;
        let next_index = self.appender.index  + 1;
        
        // Create a new appender
        let (new_appender, new_archive) = LogAppender::new(
            self.path.clone(),
            false,
            next_index,
            &header_bytes[..]
        ).await?;
    
        // Set the new appender
        self.archives.insert(next_index , new_archive);
        self.appender = new_appender;

        // Success
        Ok(())
    }

    async fn copy(&mut self) -> Result<Box<dyn LogFile>>
    {
        // Copy all the archives
        let mut log_archives = FxHashMap::default();
        for (k, v) in self.archives.iter() {
            log_archives.insert(k.clone(), v.clone().await?);
        }

        #[cfg(feature = "caching")]
        let cache = {
            let cache = self.cache.lock();
            MutexSync::new(LogFileCache {
                flush: cache.flush.clone(),
                read: cached::TimedSizedCache::with_size_and_lifespan(cache.read.cache_capacity().unwrap(), cache.read.cache_lifespan().unwrap()),
                write: cached::TimedSizedCache::with_size_and_lifespan(cache.write.cache_capacity().unwrap(), cache.write.cache_lifespan().unwrap()),
            })
        };

        Ok(
            Box::new(LogFileLocalFs {
                path: self.path.clone(),
                temp: self.temp,
                lookup: self.lookup.clone(),
                appender: self.appender.clone().await?,
                #[cfg(feature = "caching")]
                cache,
                archives: log_archives,
            })
        )
    }

    async fn write(&mut self, evt: &EventData) -> std::result::Result<LogLookup, SerializationError>
    {
        // Write the appender
        let header = evt.as_header_raw()?;
        #[cfg(feature = "local_fs")]
        let lookup = self.appender.write(evt, &header).await?;
        
        // Record the lookup map
        self.lookup.insert(header.event_hash, lookup);

        #[cfg(feature = "verbose")]
        debug!("log-write: {} - {:?}", header.event_hash, lookup);
        #[cfg(feature = "super_verbose")]
        debug!("log-write: {:?} - {:?}", header, evt);

        // If we are in debug mode then do some extra checks
        #[cfg(feature = "extra_checks")]
        {
            self.appender.flush().await?;
            self.load(header.event_hash).await.expect("Failed to load!");
        }

        // Cache the data
        #[cfg(feature = "caching")]
        {
            let mut cache = self.cache.lock();
            cache.flush.insert(header.event_hash, LoadData {
                lookup,
                header,
                data: evt.clone(),
            });
        }

        // Return the result
        Ok(lookup)
    }

    async fn copy_event(&mut self, from_log: &Box<dyn LogFile>, hash: AteHash) -> std::result::Result<LogLookup, LoadError>
    {
        // Load the data from the log file
        let result = from_log.load(hash).await?;

        // Write it to the local log
        let lookup = self.appender.write(&result.data, &result.header).await?;

        // Record the lookup map
        self.lookup.insert(hash.clone(), lookup);

        // Cache the data
        #[cfg(feature = "caching")]
        {
            let mut cache = self.cache.lock();
            cache.flush.insert(hash.clone(), LoadData {
                header: result.header,
                lookup,
                data: result.data,
            });
        }

        Ok(lookup)
    }

    async fn load(&self, hash: AteHash) -> std::result::Result<LoadData, LoadError>
    {
        // Check the caches
        #[cfg(feature = "caching")]
        {
            let mut cache = self.cache.lock();
            if let Some(result) = cache.flush.get(&hash) {
                return Ok(result.clone());
            }
            if let Some(result) = cache.read.cache_get(&hash) {
                return Ok(result.clone());
            }
            if let Some(result) = cache.write.cache_remove(&hash) {
                return Ok(result);
            }
        }

        // Lookup the record in the redo log
        let lookup = match self.lookup.get(&hash) {
            Some(a) => a.clone(),
            None => {
                return Err(LoadError::NotFoundByHash(hash));
            }
        };
        let _offset = lookup.offset;

        // Load the archive
        let archive = match self.archives.get(&lookup.index) {
            Some(a) => a,
            None => {
                return Err(LoadError::NotFoundByHash(hash));
            }
        };

        // First read all the data into a buffer
        let result = {
            let mut loader = archive.lock_at(_offset).await?;
            match EventVersion::read(&mut loader).await? {
                Some(a) => a,
                None => { return Err(LoadError::NotFoundByHash(hash)); }
            }
        };
        
        // Hash body
        let data_hash = match &result.data {
            Some(data) => Some(AteHash::from_bytes(&data[..])),
            None => None,
        };
        let data_size = match &result.data {
            Some(data) => data.len(),
            None => 0
        };
        let data = match result.data {
            Some(data) => Some(Bytes::from(data)),
            None => None,
        };

        // Convert the result into a deserialized result
        let meta = result.header.format.meta.deserialize(&result.meta[..])?;
        let ret = LoadData {
            header: EventHeaderRaw::new(
                AteHash::from_bytes(&result.meta[..]),
                Bytes::from(result.meta),
                data_hash,
                data_size,
                result.header.format,
            ),
            data: EventData {
                meta,
                data_bytes: data,
                format: result.header.format,
            },
            lookup,
        };
        assert_eq!(hash.to_string(), ret.header.event_hash.to_string());

        // Store it in the read cache
        #[cfg(feature = "caching")]
        {
            let mut cache = self.cache.lock();
            cache.read.cache_set(ret.header.event_hash, ret.clone());
        }

        Ok(
            ret
        )
    }

    fn move_log_file(&mut self, new_path: &String) -> Result<()>
    {
        if self.temp == false
        {
            // First rename the orginal logs as a backup
            let mut n = 0;
            loop {
                let path_from = format!("{}.{}", new_path, n);
                let path_to = format!("{}.backup.{}", new_path, n);

                if std::path::Path::new(path_from.as_str()).exists() == false {
                    break;
                }

                std::fs::rename(path_from, path_to)?;
                n = n + 1;
            }

            // Move the flipped logs over to replace the originals
            let mut n = 0;
            loop {
                let path_from = format!("{}.{}", self.path.clone(), n);
                let path_to = format!("{}.{}", new_path, n);

                if std::path::Path::new(path_from.as_str()).exists() == false {
                    break;
                }

                std::fs::rename(path_from, path_to)?;
                n = n + 1;
            }

            // Now delete all the backups
            let mut n = 0;
            loop {
                let path_old = format!("{}.backup.{}", new_path, n);
                if std::path::Path::new(path_old.as_str()).exists() == true {
                    std::fs::remove_file(path_old)?;
                } else {
                    break;
                }
                n = n + 1;
            }
        }
        self.path = new_path.clone();
        Ok(())
    }

    async fn flush(&mut self) -> Result<()>
    {
        // Make a note of all the cache lines we need to move
        #[cfg(feature = "caching")]
        let mut keys = Vec::new();

        #[cfg(feature = "caching")]
        {
            let cache = self.cache.lock();
            for k in cache.flush.keys() {
                keys.push(k.clone());
            }
        }

        // Flush the data to disk
        self.appender.flush().await?;

        // Move the cache lines into the write cache from the flush cache which
        // will cause them to be released after the TTL is reached
        #[cfg(feature = "caching")]
        {
            let mut cache = self.cache.lock();
            for k in keys.into_iter() {
                if let Some(v) =  cache.flush.remove(&k) {
                    cache.write.cache_set(k, v);
                }
            }
        }

        Ok(())
    }

    fn count(&self) -> usize {
        self.lookup.values().len()
    }

    fn size(&self) -> u64 {
        self.appender.offset() - self.appender.header().len() as u64
    }

    fn index(&self) -> u32 {
        self.appender.index
    }

    fn offset(&self) -> u64 {
        self.appender.offset() as u64
    }

    fn header(&self, index: u32) -> Vec<u8> {
        if index == u32::MAX || index == self.appender.index {
            return Vec::from(self.appender.header());
        }

        if let Some(a) = self.archives.get(&index) {
            Vec::from(a.header())
        } else {
            Vec::new()
        }
    }

    fn destroy(&mut self) -> Result<()>
    {
        // Now delete all the log files
        let mut n = 0;
        loop {
            let path_old = format!("{}.{}", self.path, n);
            if std::path::Path::new(path_old.as_str()).exists() == true {
                std::fs::remove_file(path_old)?;
            } else {
                break;
            }
            n = n + 1;
        }
        Ok(())
    }

    async fn begin_flip(&self, header_bytes: Vec<u8>) -> Result<Box<dyn LogFile>> {
        let ret = {
            let path_flip = format!("{}.flip", self.path);

            #[cfg(feature = "caching")]
            let (cache_size, cache_ttl) = {
                let cache = self.cache.lock();
                let cache_size = cache.read.cache_capacity().unwrap();
                let cache_ttl = cache.read.cache_lifespan().unwrap();
                (cache_size, cache_ttl)
            };
            #[cfg(not(feature = "caching"))]
            let (cache_size, cache_ttl) = {
                (0, u64::MAX)
            };

            LogFileLocalFs::new(
                self.temp, 
                path_flip, 
                true, 
                cache_size, 
                cache_ttl,
                header_bytes,
            )
        };

        Ok(ret.await?)
    }
}