// Copyright 2025 Bloxide, all rights reserved

use super::{RootComponents, RootStates};
use crate::blox::root::states::*;
use bloxide::core::state_machine::*;
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Finished;

impl State<RootComponents> for Finished {
    fn parent(&self) -> RootStates {
        RootStates::Idle(Idle)
    }

    fn handle_message(
        &self,
        _msg: <RootComponents as Components>::MessageSet,
        _data: &mut <RootComponents as Components>::ExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::States>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        //Program is finished, no more messages
        (None, None)
    }

    fn on_entry(&self, _data: &mut <RootComponents as Components>::ExtendedState, _self_id: &u16) {
        trace!("State on_entry: {:?}", self);
        info!("♫ I CAN ONLY COUNT TO 4 ♫");
    }
}
