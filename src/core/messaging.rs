// Copyright 2025 Bloxide, all rights reserved

use crate::runtime::*;
use crate::std_exports::*;

/// Basic message type that wraps any payload and has an id. Usually used as a "source id"
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

/// Handle type that corresponds to a specific message type
#[derive(Debug)]
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
    fn try_send(
        &self,
        msg: Message<Self::PayloadType>,
    ) -> Result<(), TrySendError<Message<Self::PayloadType>>>;
}

/// Enum for standard payloads
#[derive(Debug)]
pub enum StandardPayload {
    Initialize,
    Shutdown,
    PollHandle,
    Handle(Box<dyn Any + Send>),
    PollState,
    State(Box<dyn Any + Send>),
    Error(Box<String>),
    StandardChannel(StandardMessageHandle, StandardReceiver),
}

pub enum SupervisorPayload {
    Spawn(Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>),
    RequestNewActorHandle(usize),
    Error(Box<String>),
}

impl fmt::Debug for SupervisorPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupervisorPayload::Spawn(_) => write!(f, "Spawn"),
            SupervisorPayload::RequestNewActorHandle(queue_size) => {
                write!(f, "RequestNewActorHandle: {}", queue_size)
            }
            SupervisorPayload::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

pub enum SupervisorLocalPayload {
    SpawnLocal(Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + 'static>> + 'static>),
    RequestNewActorHandle(usize),
    Error(Box<String>),
}

impl fmt::Debug for SupervisorLocalPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupervisorLocalPayload::SpawnLocal(_) => write!(f, "SpawnLocal"),
            SupervisorLocalPayload::RequestNewActorHandle(queue_size) => {
                write!(f, "RequestNewActorHandle: {}", queue_size)
            }
            SupervisorLocalPayload::Error(e) => write!(f, "Error: {}", e),
        }
    }
}
