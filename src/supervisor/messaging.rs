// Copyright 2025 Bloxide, all rights reserved

use crate::core::messaging::*;
use crate::std_exports::*;

#[derive(Debug)]
pub enum SupervisorMessageSet {
    StandardMessage(Message<StandardPayload>),
    SupervisorMessage(Message<SupervisorPayload>),
}
impl MessageSet for SupervisorMessageSet {}

/// Communication with Supervisor to spawn new Blox. Send and !Send options
pub enum SupervisorPayload {
    Spawn(Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>),
    RequestNewStandardHandle(usize),
    Error(Box<String>),
}

impl fmt::Debug for SupervisorPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupervisorPayload::Spawn(_) => write!(f, "Spawn"),
            SupervisorPayload::RequestNewStandardHandle(queue_size) => {
                write!(f, "RequestNewStandardHandle: {}", queue_size)
            }
            SupervisorPayload::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

pub enum SupervisorLocalPayload {
    SpawnLocal(Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + 'static>> + 'static>),
    RequestNewStandardHandle(usize),
    Error(Box<String>),
}

impl fmt::Debug for SupervisorLocalPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupervisorLocalPayload::SpawnLocal(_) => write!(f, "SpawnLocal"),
            SupervisorLocalPayload::RequestNewStandardHandle(queue_size) => {
                write!(f, "RequestNewStandardHandle: {}", queue_size)
            }
            SupervisorLocalPayload::Error(e) => write!(f, "Error: {}", e),
        }
    }
}
