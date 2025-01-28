// Copyright 2025 Bloxide, all rights reserved

use super::components::*;
use crate::{
    core::{messaging::*, state_machine::*},
    runtime::*,
    std_exports::*,
};

#[derive(Default)]
pub struct SupervisorExtendedState {
    pub blox: HashMap<u16, StandardMessageHandle>,
    pub root_spawn_fn: Option<RootSpawnFn>,
    pub next_id: u16,
}

impl SupervisorExtendedState {
    pub fn request_new_standard_handle(
        &mut self,
        queue_size: usize,
    ) -> (
        StandardMessageHandle,
        <StandardMessageHandle as MessageSender>::ReceiverType,
    ) {
        let (new_handle, rx) = Handle::create_channel_with_size(self.next_id, queue_size);
        self.blox.insert(new_handle.dest_id(), new_handle.clone());
        self.next_id += 1;
        (new_handle, rx)
    }
}

impl ExtendedState for SupervisorExtendedState {
    type InitArgs = SupervisorInitArgs;
    fn new(args: Self::InitArgs) -> Self {
        let blox = HashMap::from([(
            args.root_standard_handle.dest_id(),
            args.root_standard_handle.clone(),
        )]);

        SupervisorExtendedState {
            blox,
            root_spawn_fn: Some(args.root_spawn_fn),
            next_id: 2, //0 is reserved for the Supervisor, 1 is reserved for the Root
        }
    }
}

impl fmt::Debug for SupervisorExtendedState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SupervisorExtendedState")
    }
}
