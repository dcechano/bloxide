// Copyright 2025 Bloxide, all rights reserved

use crate::core::actor::ActorSpawner;
use crate::std_exports::*;

// Base trait for all messages
pub trait ActorMessage: Debug + Send + Clone + 'static {}

// Blanket implementation
impl<T> ActorMessage for T where T: Debug + Send + Clone + 'static {}

pub trait ActorHandle<M: ActorMessage>: Send + Debug + 'static {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn ActorHandle<M>>;
    fn send(&self, msg: M);
}

impl<M: ActorMessage> Clone for Box<dyn ActorHandle<M>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

// Type Erasure allows us to send handles as trait objects between actors,
// but use the concrete type for sending messages
pub trait ErasedActorHandle: Send + Debug {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> Box<dyn ErasedActorHandle>;
}

impl Clone for Box<dyn ErasedActorHandle> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

// Wrapper type for type erasure
#[derive(Debug)]
struct ErasedHandleWrapper<M: ActorMessage>(Box<dyn ActorHandle<M>>);

impl<M: ActorMessage> ErasedActorHandle for ErasedHandleWrapper<M> {
    fn as_any(&self) -> &dyn Any {
        self.0.as_any()
    }

    fn box_clone(&self) -> Box<dyn ErasedActorHandle> {
        Box::new(ErasedHandleWrapper(self.0.clone()))
    }
}

pub fn erase_handle<M: ActorMessage>(
    handle: Box<dyn ActorHandle<M>>,
) -> Box<dyn ErasedActorHandle> {
    Box::new(ErasedHandleWrapper(handle))
}

#[derive(Debug)]
pub struct OwnedSpawner(Box<dyn ActorSpawner>);

impl OwnedSpawner {
    pub fn new(spawner: Box<dyn ActorSpawner>) -> Self {
        Self(spawner)
    }

    pub fn as_spawner(&self) -> &dyn ActorSpawner {
        self.0.as_ref()
    }
}

// Update StandardMessage to use OwnedSpawner
#[derive(Debug)]
pub enum StandardMessage {
    Initialize,
    Shutdown,
    RegisterSender(Box<dyn ErasedActorHandle>),
    SpawnRequest(OwnedSpawner),
    Error(String),
}

// We can still derive Clone for StandardMessage since OwnedSpawner is only moved
impl Clone for StandardMessage {
    fn clone(&self) -> Self {
        match self {
            Self::Initialize => Self::Initialize,
            Self::Shutdown => Self::Shutdown,
            Self::RegisterSender(handle) => Self::RegisterSender(handle.clone()),
            Self::Error(err) => Self::Error(err.clone()),
            Self::SpawnRequest(_) => panic!("SpawnRequest cannot be cloned"),
        }
    }
}

pub trait MessageSet: Debug + Send + 'static {
    fn from_standard(msg: StandardMessage) -> Self;
}

impl MessageSet for StandardMessage {
    fn from_standard(msg: StandardMessage) -> Self {
        msg
    }
}
