// Copyright 2025 Bloxide, all rights reserved

use crate::{runtime::*, std_exports::*};
use serde::{Deserialize, Serialize};

/// Basic message type that wraps any payload and has an id
#[derive(Debug)]
pub struct Message<P> {
    pub source_id: u16,
    pub payload: P,
}

impl<P> Message<P> {
    pub fn new(source_id: u16, payload: P) -> Self {
        Self { source_id, payload }
    }

    pub fn source_id(&self) -> u16 {
        self.source_id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawPayload {
    pub to: u16,
    pub from: u16,
    pub payload: Vec<u8>,
}

/// Marker trait for message sets
/// Blox's message enum should implement this trait to specify the messages they can send and receive.
pub trait MessageSet {}

/// Handle type that corresponds to a specific message type
pub struct Handle<S> {
    pub dest_id: u16,
    pub sender: S,
}

impl<S> PartialEq for Handle<S> {
    fn eq(&self, other: &Self) -> bool {
        self.dest_id == other.dest_id
    }
}

impl<S> Eq for Handle<S> {}

impl<S> Hash for Handle<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dest_id.hash(state);
    }
}

impl<S> Handle<S> {
    pub fn new(dest_id: u16, sender: S) -> Self {
        Self { dest_id, sender }
    }

    pub fn dest_id(&self) -> u16 {
        self.dest_id
    }
}

/// Add manual Clone implementation that only requires S: Clone
/// This allows `Message<T>`to be passed by value if necessary
impl<S: Clone> Clone for Handle<S> {
    fn clone(&self) -> Self {
        Self {
            dest_id: self.dest_id,
            sender: self.sender.clone(),
        }
    }
}

/// Implement type erasure to send any Handle over channels
impl<S: Clone + 'static> Handle<S> {
    pub fn into_erased(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

/// Trait for handles to send messages
pub trait MessageSender {
    type PayloadType;
    type SenderType;
    type ReceiverType;
    fn try_send(
        &self,
        msg: Message<Self::PayloadType>,
    ) -> Result<(), TrySendError<Message<Self::PayloadType>>>;

    fn create_channel_with_size(
        id: u16,
        size: usize,
    ) -> (Handle<Self::SenderType>, Self::ReceiverType);
}

/// Enum for standard payloads
#[derive(Debug)]
pub enum StandardPayload {
    Shutdown,
    PollHandle,
    Handle(Box<dyn Any + Send>),
    PollState,
    State(Box<dyn Any + Send>),
    Error(Box<String>),
    StandardChannel(
        StandardMessageHandle,
        <StandardMessageHandle as MessageSender>::ReceiverType,
    ),
    RawInbound(u16, Vec<u8>),  // source id, payload
    RawOutbound(u16, Vec<u8>), // dest id, payload
}
