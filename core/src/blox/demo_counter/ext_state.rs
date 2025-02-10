// Copyright 2025 Bloxide, all rights reserved

use super::messaging::CounterPayload;
use crate::components::Runtime;
use crate::{state_machine::*, SupervisorPayload};

#[derive(Default)]
pub struct CounterExtendedState<R: Runtime> {
    pub count: usize,
    pub max: usize,
    pub min: usize,
    pub subscribers: Vec<R::MessageHandle<CounterPayload>>,
    pub supervisor_handle: R::MessageHandle<SupervisorPayload>,
}

pub struct CounterInitArgs<R: Runtime> {
    pub supervisor_handle: R::MessageHandle<SupervisorPayload>,
}

impl<R: Runtime> ExtendedState for CounterExtendedState<R> {
    type InitArgs = CounterInitArgs<R>;
    fn new(args: Self::InitArgs) -> Self {
        Self {
            count: 0,
            max: 10,
            min: 0,
            subscribers: Vec::new(),
            supervisor_handle: args.supervisor_handle,
        }
    }
}
