// Copyright 2025 Bloxide, all rights reserved

pub mod error;
pub mod running;
pub mod uninit;

use super::{components::*, ext_state::*, messaging::*};
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
    fn on_entry(&self, data: &mut SupervisorExtendedState, self_id: &u16) {
        match self {
            SupervisorStateEnum::Uninit(s) => s.on_entry(data, self_id),
            SupervisorStateEnum::Running(s) => s.on_entry(data, self_id),
            SupervisorStateEnum::Error(s) => s.on_entry(data, self_id),
        }
    }

    fn on_exit(&self, data: &mut SupervisorExtendedState, self_id: &u16) {
        match self {
            SupervisorStateEnum::Uninit(s) => s.on_exit(data, self_id),
            SupervisorStateEnum::Running(s) => s.on_exit(data, self_id),
            SupervisorStateEnum::Error(s) => s.on_exit(data, self_id),
        }
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
        match self {
            SupervisorStateEnum::Uninit(s) => s.handle_message(message, data, self_id),
            SupervisorStateEnum::Running(s) => s.handle_message(message, data, self_id),
            SupervisorStateEnum::Error(s) => s.handle_message(message, data, self_id),
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
