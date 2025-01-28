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
        message: SupervisorMessageSet,
        _data: &mut SupervisorExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<SupervisorStateEnum>>,
        Option<SupervisorMessageSet>,
    ) {
        trace!("[Error] handle_message: {:?}", message);
        (None, None)
    }
}
