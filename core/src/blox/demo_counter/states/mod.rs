// Copyright 2025 Bloxide, all rights reserved

pub mod counting;
pub mod error;
pub mod finished;
pub mod idle;
pub mod not_started;
pub mod uninit;

use super::{components::*, messaging::*};
use crate::components::Runtime;
use crate::{messaging::*, state_machine::*};
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

impl<R: Runtime> State<CounterComponents<R>> for CounterStateEnum
where
    <R::MessageHandle<StandardPayload<R>> as MessageSender>::ReceiverType: Send + 'static,
    <R::MessageHandle<CounterPayload> as MessageSender>::ReceiverType: Send + 'static,
{
    fn on_entry(&self, state_machine: &mut StateMachine<CounterComponents<R>>) {
        match self {
            CounterStateEnum::Uninit(s) => s.on_entry(state_machine),
            CounterStateEnum::NotStarted(s) => s.on_entry(state_machine),
            CounterStateEnum::Idle(s) => s.on_entry(state_machine),
            CounterStateEnum::Counting(s) => s.on_entry(state_machine),
            CounterStateEnum::Finished(s) => s.on_entry(state_machine),
            CounterStateEnum::Error(s) => s.on_entry(state_machine),
        }
    }

    fn on_exit(&self, data: &mut StateMachine<CounterComponents<R>>) {
        match self {
            CounterStateEnum::Uninit(s) => s.on_exit(data),
            CounterStateEnum::NotStarted(s) => s.on_exit(data),
            CounterStateEnum::Idle(s) => s.on_exit(data),
            CounterStateEnum::Counting(s) => s.on_exit(data),
            CounterStateEnum::Finished(s) => s.on_exit(data),
            CounterStateEnum::Error(s) => s.on_exit(data),
        }
    }

    fn handle_message(
        &self,
        state_machine: &mut StateMachine<CounterComponents<R>>,
        message: CounterMessageSet<R>,
    ) -> Option<Transition<CounterStateEnum, CounterMessageSet<R>>> {
        match self {
            CounterStateEnum::Uninit(s) => s.handle_message(state_machine, message),
            CounterStateEnum::NotStarted(s) => s.handle_message(state_machine, message),
            CounterStateEnum::Idle(s) => s.handle_message(state_machine, message),
            CounterStateEnum::Counting(s) => s.handle_message(state_machine, message),
            CounterStateEnum::Finished(s) => s.handle_message(state_machine, message),
            CounterStateEnum::Error(s) => s.handle_message(state_machine, message),
        }
    }

    fn parent(&self) -> CounterStateEnum {
        match self {
            CounterStateEnum::Uninit(s) => <Uninit as State<CounterComponents<R>>>::parent(s),
            CounterStateEnum::NotStarted(s) => {
                <NotStarted as State<CounterComponents<R>>>::parent(s)
            }
            CounterStateEnum::Idle(s) => <Idle as State<CounterComponents<R>>>::parent(s),
            CounterStateEnum::Counting(s) => <Counting as State<CounterComponents<R>>>::parent(s),
            CounterStateEnum::Finished(s) => <Finished as State<CounterComponents<R>>>::parent(s),
            CounterStateEnum::Error(s) => <Error as State<CounterComponents<R>>>::parent(s),
        }
    }
}
