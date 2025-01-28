// Copyright 2025 Bloxide, all rights reserved

pub mod counting;
pub mod error;
pub mod finished;
pub mod idle;
pub mod not_started;
pub mod uninit;

use super::{components::*, ext_state::*, messaging::*};
use bloxide::core::state_machine::*;
pub use {
    counting::Counting, error::Error, finished::Finished, idle::Idle, not_started::NotStarted,
    uninit::Uninit,
};

#[derive(Clone, PartialEq, Debug)]
pub enum CounterStateEnum {
    Uninit(Uninit),
    Idle(Idle),
    NotStarted(NotStarted),
    Counting(Counting),
    Finished(Finished),
    Error(Error),
}
impl Default for CounterStateEnum {
    fn default() -> Self {
        CounterStateEnum::Uninit(Uninit)
    }
}

impl StateEnum for CounterStateEnum {}

impl State<CounterComponents> for CounterStateEnum {
    fn on_entry(&self, data: &mut CounterExtendedState, self_id: &u16) {
        match self {
            CounterStateEnum::Uninit(s) => s.on_entry(data, self_id),
            CounterStateEnum::NotStarted(s) => s.on_entry(data, self_id),
            CounterStateEnum::Idle(s) => s.on_entry(data, self_id),
            CounterStateEnum::Counting(s) => s.on_entry(data, self_id),
            CounterStateEnum::Finished(s) => s.on_entry(data, self_id),
            CounterStateEnum::Error(s) => s.on_entry(data, self_id),
        }
    }

    fn on_exit(&self, data: &mut CounterExtendedState, self_id: &u16) {
        match self {
            CounterStateEnum::Uninit(s) => s.on_exit(data, self_id),
            CounterStateEnum::NotStarted(s) => s.on_exit(data, self_id),
            CounterStateEnum::Idle(s) => s.on_exit(data, self_id),
            CounterStateEnum::Counting(s) => s.on_exit(data, self_id),
            CounterStateEnum::Finished(s) => s.on_exit(data, self_id),
            CounterStateEnum::Error(s) => s.on_exit(data, self_id),
        }
    }

    fn handle_message(
        &self,
        message: CounterMessageSet,
        data: &mut CounterExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<CounterStateEnum>>,
        Option<CounterMessageSet>,
    ) {
        match self {
            CounterStateEnum::Uninit(s) => s.handle_message(message, data, _self_id),
            CounterStateEnum::NotStarted(s) => s.handle_message(message, data, _self_id),
            CounterStateEnum::Idle(s) => s.handle_message(message, data, _self_id),
            CounterStateEnum::Counting(s) => s.handle_message(message, data, _self_id),
            CounterStateEnum::Finished(s) => s.handle_message(message, data, _self_id),
            CounterStateEnum::Error(s) => s.handle_message(message, data, _self_id),
        }
    }

    fn parent(&self) -> CounterStateEnum {
        match self {
            CounterStateEnum::Uninit(s) => s.parent(),
            CounterStateEnum::NotStarted(s) => s.parent(),
            CounterStateEnum::Idle(s) => s.parent(),
            CounterStateEnum::Counting(s) => s.parent(),
            CounterStateEnum::Finished(s) => s.parent(),
            CounterStateEnum::Error(s) => s.parent(),
        }
    }
}
