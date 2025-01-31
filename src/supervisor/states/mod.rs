// Copyright 2025 Bloxide, all rights reserved

pub mod error;
pub mod running;
pub mod uninit;

use super::{components::*, messaging::*};
use crate::core::state_machine::*;
pub use {error::*, running::*, uninit::*};

#[derive(Clone, PartialEq, Debug)]
pub enum SupervisorStateEnum {
    Uninit(Uninit),
    Running(Running),
    Error(Error),
}
impl Default for SupervisorStateEnum {
    fn default() -> Self {
        SupervisorStateEnum::Uninit(Uninit)
    }
}
impl StateEnum for SupervisorStateEnum {}

impl State<SupervisorComponents> for SupervisorStateEnum {
    fn on_entry(&self, state_machine: &mut StateMachine<SupervisorComponents>) {
        match self {
            SupervisorStateEnum::Uninit(s) => s.on_entry(state_machine),
            SupervisorStateEnum::Running(s) => s.on_entry(state_machine),
            SupervisorStateEnum::Error(s) => s.on_entry(state_machine),
        }
    }

    fn on_exit(&self, state_machine: &mut StateMachine<SupervisorComponents>) {
        match self {
            SupervisorStateEnum::Uninit(s) => s.on_exit(state_machine),
            SupervisorStateEnum::Running(s) => s.on_exit(state_machine),
            SupervisorStateEnum::Error(s) => s.on_exit(state_machine),
        }
    }

    fn handle_message(
        &self,
        state_machine: &mut StateMachine<SupervisorComponents>,
        message: SupervisorMessageSet,
    ) -> Option<Transition<SupervisorStateEnum, SupervisorMessageSet>> {
        match self {
            SupervisorStateEnum::Uninit(s) => s.handle_message(state_machine, message),
            SupervisorStateEnum::Running(s) => s.handle_message(state_machine, message),
            SupervisorStateEnum::Error(s) => s.handle_message(state_machine, message),
        }
    }

    fn parent(&self) -> SupervisorStateEnum {
        match self {
            SupervisorStateEnum::Uninit(s) => s.parent(),
            SupervisorStateEnum::Running(s) => s.parent(),
            SupervisorStateEnum::Error(s) => s.parent(),
        }
    }
}
