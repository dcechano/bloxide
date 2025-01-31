// Copyright 2025 Bloxide, all rights reserved

use super::*;
#[cfg(feature = "runtime-tokio")]
use crate::runtime::*;
use crate::{
    core::{messaging::*, state_machine::*},
    std_exports::*,
};
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Running;

impl State<SupervisorComponents> for Running {
    fn parent(&self) -> SupervisorStateEnum {
        SupervisorStateEnum::Uninit(Uninit)
    }

    fn handle_message(
        &self,
        state_machine: &mut StateMachine<SupervisorComponents>,
        message: SupervisorMessageSet,
    ) -> Option<Transition<SupervisorStateEnum, SupervisorMessageSet>> {
        trace!("[Running] handle_message: {:?}", message);
        let transition = match message {
            SupervisorMessageSet::SupervisorMessage(message) => match message.payload {
                SupervisorPayload::Spawn(spawn_fn) => {
                    self.spawn_blox(spawn_fn());
                    None
                }
                SupervisorPayload::RequestNewStandardHandle(queue_size) => {
                    let (new_handle, rx) = state_machine
                        .extended_state
                        .request_new_standard_handle(queue_size);
                    // get handle for the id in the message to send the response
                    let handle = state_machine
                        .extended_state
                        .blox
                        .get(&message.source_id())
                        .unwrap();
                    let _ = handle.try_send(Message::new(
                        state_machine.self_handles.standard_handle.dest_id,
                        StandardPayload::StandardChannel(new_handle, rx),
                    ));
                    None
                }
                _ => None,
            },
            _ => None,
        };
        transition
    }
}

#[cfg(feature = "runtime-tokio")]
impl Running {
    fn spawn_blox(&self, future: Pin<Box<dyn Future<Output = ()> + Send>>) {
        spawn(future);
    }
}

#[cfg(feature = "runtime-embassy")]
impl Running {
    fn spawn_blox(&self, _future: Pin<Box<dyn Future<Output = ()> + Send>>) {
        todo!()
    }
}
