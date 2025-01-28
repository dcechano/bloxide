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
        message: SupervisorMessageSet,
        data: &mut SupervisorExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<SupervisorStateEnum>>,
        Option<SupervisorMessageSet>,
    ) {
        trace!("[Running] handle_message: {:?}", message);
        let (transition, message_option) = match message {
            SupervisorMessageSet::SupervisorMessage(message) => match message.payload {
                SupervisorPayload::Spawn(spawn_fn) => {
                    self.spawn_blox(spawn_fn());
                    (None, None)
                }
                SupervisorPayload::RequestNewStandardHandle(queue_size) => {
                    let (new_handle, rx) = data.request_new_standard_handle(queue_size);
                    // get handle for the id in the message to send the response
                    let handle = data.blox.get(&message.source_id()).unwrap();
                    let _ = handle.try_send(Message::new(
                        *self_id,
                        StandardPayload::StandardChannel(new_handle, rx),
                    ));
                    (None, None)
                }
                _ => (None, None),
            },
            _ => (None, None),
        };
        (transition, message_option)
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
