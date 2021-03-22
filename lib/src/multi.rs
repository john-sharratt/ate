use tokio::sync::RwLock;
use parking_lot::RwLock as StdRwLock;
#[allow(unused_imports)]
use std::sync::mpsc as smpsc;
#[allow(unused_imports)]
use std::sync::{Weak, Arc};

use crate::session::{Session};

use super::meta::*;
use super::error::*;
use super::chain::*;
use super::pipe::*;
use super::trust::*;
use super::header::*;
#[allow(unused_imports)]
use super::event::*;
use super::lint::*;
use super::spec::*;
use super::index::*;

use bytes::Bytes;

#[derive(Clone)]
pub struct ChainMultiUser
where Self: Send + Sync
{
    pub(super) inside_async: Arc<RwLock<ChainProtectedAsync>>,
    pub(super) inside_sync: Arc<StdRwLock<ChainProtectedSync>>,
    pub(super) pipe: Arc<dyn EventPipe>,
    pub(super) default_format: MessageFormat,
}

impl ChainMultiUser
{
    pub(crate) async fn new(accessor: &Chain) -> ChainMultiUser
    {
        ChainMultiUser {
            inside_async: Arc::clone(&accessor.inside_async),
            inside_sync: Arc::clone(&accessor.inside_sync),
            pipe: Arc::clone(&accessor.pipe),
            default_format: accessor.default_format,
        }
    }
 
    #[allow(dead_code)]
    pub async fn load(&self, leaf: EventLeaf) -> Result<LoadResult, LoadError> {
        self.inside_async.read().await.chain.load(leaf).await
    }

    #[allow(dead_code)]
    pub async fn load_many(&self, leafs: Vec<EventLeaf>) -> Result<Vec<LoadResult>, LoadError> {
        self.inside_async.read().await.chain.load_many(leafs).await
    }

    #[allow(dead_code)]
    pub async fn lookup_primary(&self, key: &PrimaryKey) -> Option<EventLeaf> {
        self.inside_async.read().await.chain.lookup_primary(key)
    }

    #[allow(dead_code)]
    pub async fn lookup_secondary(&self, key: &MetaCollection) -> Option<Vec<EventLeaf>> {
        self.inside_async.read().await.chain.lookup_secondary(key)
    }

    #[allow(dead_code)]
    pub(crate) fn metadata_lint_many<'a>(&self, lints: &Vec<LintData<'a>>, session: &Session) -> Result<Vec<CoreMetadata>, LintError> {
        let guard = self.inside_sync.read();
        let mut ret = Vec::new();
        for linter in guard.linters.iter() {
            ret.extend(linter.metadata_lint_many(lints, session)?);
        }
        for plugin in guard.plugins.iter() {
            ret.extend(plugin.metadata_lint_many(lints, session)?);
        }
        Ok(ret)
    }

    #[allow(dead_code)]
    pub(crate) fn metadata_lint_event(&self, meta: &mut Metadata, session: &Session) -> Result<Vec<CoreMetadata>, LintError> {
        let guard = self.inside_sync.read();
        let mut ret = Vec::new();
        for linter in guard.linters.iter() {
            ret.extend(linter.metadata_lint_event(meta, session)?);
        }
        for plugin in guard.plugins.iter() {
            ret.extend(plugin.metadata_lint_event(meta, session)?);
        }
        Ok(ret)
    }

    #[allow(dead_code)]
    pub(crate) fn data_as_overlay(&self, meta: &Metadata, data: Bytes, session: &Session) -> Result<Bytes, TransformError> {
        let guard = self.inside_sync.read();
        let mut ret = data;
        for plugin in guard.plugins.iter().rev() {
            ret = plugin.data_as_overlay(meta, ret, session)?;
        }
        for transformer in guard.transformers.iter().rev() {
            ret = transformer.data_as_overlay(meta, ret, session)?;
        }
        Ok(ret)
    }

    #[allow(dead_code)]
    pub(crate) fn data_as_underlay(&self, meta: &mut Metadata, data: Bytes, session: &Session) -> Result<Bytes, TransformError> {
        let guard = self.inside_sync.read();
        let mut ret = data;
        for transformer in guard.transformers.iter() {
            ret = transformer.data_as_underlay(meta, ret, session)?;
        }
        for plugin in guard.plugins.iter() {
            ret = plugin.data_as_underlay(meta, ret, session)?;
        }
        Ok(ret)
    }
    
    #[allow(dead_code)]
    pub async fn count(&self) -> usize {
        self.inside_async.read().await.chain.redo.count()
    }
}