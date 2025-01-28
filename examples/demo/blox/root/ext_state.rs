// Copyright 2025 Bloxide, all rights reserved

use crate::blox::counter::runtime::*;
use bloxide::core::state_machine::*;

#[derive(Debug)]
pub struct RootExtState {
    pub self_counter_handle: CounterHandle,
    pub counter_handle: Option<CounterHandle>,
}

pub struct RootInitArgs {
    pub self_counter_handle: CounterHandle,
    pub counter_handle: Option<CounterHandle>,
}

impl ExtendedState for RootExtState {
    type InitArgs = RootInitArgs;
    fn new(args: Self::InitArgs) -> Self {
        Self {
            self_counter_handle: args.self_counter_handle,
            counter_handle: args.counter_handle,
        }
    }
}
