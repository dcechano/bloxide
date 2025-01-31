// Copyright 2025 Bloxide, all rights reserved

use super::*;
use crate::core::state_machine::*;
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Error;

impl State<SupervisorComponents> for Error {
    fn parent(&self) -> SupervisorStateEnum {
        SupervisorStateEnum::Uninit(Uninit)
    }

    fn handle_message(
        &self,
        _state_machine: &mut StateMachine<SupervisorComponents>,
        message: SupervisorMessageSet,
    ) -> Option<Transition<SupervisorStateEnum, SupervisorMessageSet>> {
        trace!("[Error] handle_message: {:?}", message);
        None
    }
}
