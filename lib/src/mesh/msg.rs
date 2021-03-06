use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use bytes::Bytes;
use std::sync::Arc;

use crate::{crypto::{PrivateEncryptKey, PrivateSignKey}, meta::{CoreMetadata, Metadata}};
use crate::crypto::AteHash;
use crate::event::*;
use crate::chain::ChainKey;
use crate::pipe::EventPipe;
use crate::chain::Chain;
use crate::error::*;
use crate::header::PrimaryKey;
use crate::spec::*;
use crate::session::AteSession;
use crate::crypto::PublicSignKey;
use crate::trust::IntegrityMode;
use crate::time::ChainTimestamp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct MessageEvent
{
    pub(crate) meta: Metadata,
    pub(crate) data: Option<Vec<u8>>,
    pub(crate) format: MessageFormat,
}

impl MessageEvent
{
    pub(crate) fn convert_to(evts: &Vec<EventData>) -> Vec<MessageEvent>
    {
        let mut feed_me = Vec::new();
        for evt in evts {
            let evt = MessageEvent {
                    meta: evt.meta.clone(),
                    data: match &evt.data_bytes {
                        Some(d) => Some(d.to_vec()),
                        None => None,
                    },
                    format: evt.format,
                };
            feed_me.push(evt);
        }
        feed_me
    }

    pub(crate) fn convert_from_single(evt: MessageEvent) -> EventData
    {
        EventData {
            meta: evt.meta.clone(),
            data_bytes: match evt.data {
                Some(d) => Some(Bytes::from(d)),
                None => None,
            },
            format: evt.format,
        }
    }

    pub(crate) fn convert_from(evts: impl Iterator<Item=MessageEvent>) -> Vec<EventData>
    {
        let mut feed_me = Vec::new();
        for evt in evts {
            feed_me.push(MessageEvent::convert_from_single(evt));
        }
        feed_me
    }

    pub(crate) fn data_hash(&self) -> Option<AteHash> {
        match self.data.as_ref() {
            Some(d) => Some(AteHash::from_bytes(&d[..])),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) enum Message {
    Noop,
    Connected,
    Disconnected,

    Subscribe {
        chain_key: ChainKey,
        from: ChainTimestamp
    },
    
    NotYetSubscribed,
    NotFound,
    NotThisRoot,

    Lock {
        key: PrimaryKey,
    },
    Unlock {
        key: PrimaryKey,
    },
    LockResult {
        key: PrimaryKey,
        is_locked: bool
    },

    StartOfHistory {
        size: usize,
        from: Option<ChainTimestamp>,
        to: Option<ChainTimestamp>,
        integrity: IntegrityMode,
        root_keys: Vec<PublicSignKey>,
    },
    Events {
        commit: Option<u64>,
        evts: Vec<MessageEvent>
    },
    EndOfHistory,
    
    /// Asks to confirm all events are up-to-date for transaction keeping purposes
    Confirmed(u64),
    CommitError {
        id: u64,
        err: String,
    },

    FatalTerminate {
        err: String
    },

    SecuredWith(AteSession),
}

impl Default
for Message
{
    fn default() -> Message {
        Message::Noop
    }
}