// Copyright 2025 Bloxide, all rights reserved

mod counting;
mod error;
mod finished;
mod idle;
pub(super) mod starting;
mod uninit;

use super::components::RootComponents;
use bloxide::core::{components::*, state_machine::*};
use counting::Counting;
use error::Error;
use finished::Finished;
use idle::Idle;
use starting::Starting;
use uninit::Uninit;

#[derive(Clone, PartialEq, Debug)]
pub enum RootStates {
    Uninit(Uninit),
    Idle(Idle),
    Starting(Starting),
    Counting(Counting),
    Finished(Finished),
    Error(Error),
}

impl State<RootComponents> for RootStates {
    fn handle_message(
        &self,
        message: <RootComponents as Components>::MessageSet,
        data: &mut <RootComponents as Components>::ExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::States>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        match self {
            RootStates::Uninit(uninit) => uninit.handle_message(message, data, self_id),
            RootStates::Counting(counting) => counting.handle_message(message, data, self_id),
            RootStates::Finished(finished) => finished.handle_message(message, data, self_id),
            RootStates::Error(error) => error.handle_message(message, data, self_id),
            RootStates::Idle(idle) => idle.handle_message(message, data, self_id),
            RootStates::Starting(starting) => starting.handle_message(message, data, self_id),
        }
    }

    fn on_entry(&self, data: &mut <RootComponents as Components>::ExtendedState, self_id: &u16) {
        match self {
            RootStates::Uninit(s) => s.on_entry(data, self_id),
            RootStates::Counting(s) => s.on_entry(data, self_id),
            RootStates::Finished(s) => s.on_entry(data, self_id),
            RootStates::Error(s) => s.on_entry(data, self_id),
            RootStates::Idle(s) => s.on_entry(data, self_id),
            RootStates::Starting(s) => s.on_entry(data, self_id),
        }
    }

    fn on_exit(&self, data: &mut <RootComponents as Components>::ExtendedState, self_id: &u16) {
        match self {
            RootStates::Uninit(uninit) => uninit.on_exit(data, self_id),
            RootStates::Counting(counting) => counting.on_exit(data, self_id),
            RootStates::Finished(finished) => finished.on_exit(data, self_id),
            RootStates::Error(error) => error.on_exit(data, self_id),
            RootStates::Idle(idle) => idle.on_exit(data, self_id),
            RootStates::Starting(starting) => starting.on_exit(data, self_id),
        }
    }

    fn parent(&self) -> RootStates {
        match self {
            RootStates::Uninit(s) => s.parent(),
            RootStates::Idle(s) => s.parent(),
            RootStates::Starting(s) => s.parent(),
            RootStates::Counting(s) => s.parent(),
            RootStates::Finished(s) => s.parent(),
            RootStates::Error(s) => s.parent(),
        }
    }
}

impl StateEnum for RootStates {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for RootStates {
    fn default() -> Self {
        RootStates::Uninit(Uninit)
    }
}
