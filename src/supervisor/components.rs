// Copyright 2025 Bloxide, all rights reserved

use super::{ext_state::*, messaging::*, runtime::*, states::*};
use crate::{core::components::*, core::messaging::*, runtime::*, std_exports::*};

pub struct SupervisorComponents;

impl Components for SupervisorComponents {
    type States = SupervisorStateEnum;
    type MessageSet = SupervisorMessageSet;
    type ExtendedState = SupervisorExtendedState;
    type Receivers = SupervisorReceivers;
}

pub struct SupervisorReceivers {
    pub standard_receiver: <StandardMessageHandle as MessageSender>::ReceiverType,
    pub supervisor_receiver: <SupervisorHandle as MessageSender>::ReceiverType,
}

pub struct SupervisorInitArgs {
    pub root_standard_handle: StandardMessageHandle,
    pub root_spawn_fn: RootSpawnFn,
}

pub(super) type RootSpawnFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;
