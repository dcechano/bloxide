// Copyright 2025 Bloxide, all rights reserved

use crate::core::actor::ActorSpawner;
use crate::std_exports::*;

/// Basic message type that wraps any payload and has an id. Usually used as a "source id"
#[derive(Debug)]
pub struct Message<T> {
    pub id: u16,
    pub payload: T,
}

impl<T> Message<T> {
    pub fn new(id: u16, payload: T) -> Self {
        Self { id, payload }
    }

    pub fn id(&self) -> u16 {
        self.id
    }
}

/// Handle type that corresponds to a specific message type
#[derive(Debug)]
pub struct Handle<M, S> {
    pub id: u16,
    pub sender: S,
    pub _phantom: PhantomData<M>,
}

impl<M, S> Handle<M, S> {
    pub fn new(id: u16, sender: S) -> Self {
        Self {
            id,
            sender,
            _phantom: PhantomData,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }
}

/// Add manual Clone implementation that only requires S: Clone
/// This allows `Message<T>`to be passed by value if necessary
impl<M, S: Clone + Send> Clone for Handle<M, S> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            sender: self.sender.clone(),
            _phantom: PhantomData,
        }
    }
}

/// Implement type erasure to send any Handle over channels
impl<M: Send + 'static, S: Clone + Send + 'static> Handle<M, S> {
    pub fn into_erased(self) -> Box<dyn Any + Send> {
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
pub enum StandardPayload {
    Initialize,
    Shutdown,
    RegisterSender(Box<dyn Any + Send>),
    SpawnRequest(Box<dyn ActorSpawner>),
    Error(String),
}

impl Debug for StandardPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            StandardPayload::Initialize => write!(f, "Initialize"),
            StandardPayload::Shutdown => write!(f, "Shutdown"),
            StandardPayload::RegisterSender(_) => write!(f, "RegisterSender(...)"),
            StandardPayload::SpawnRequest(_) => write!(f, "SpawnRequest(...)"),
            StandardPayload::Error(msg) => write!(f, "Error({})", msg),
        }
    }
}
