// Copyright 2025 Bloxide, all rights reserved

use super::*;
use crate::blox::counter::{components::*, ext_state::*, messaging::*};
use bloxide::core::state_machine::*;
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Uninit;

impl State<CounterComponents> for Uninit {
    fn parent(&self) -> CounterStateEnum {
        CounterStateEnum::Uninit(Uninit)
    }

    fn handle_message(
        &self,
        _message: CounterMessageSet,
        _data: &mut CounterExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<CounterStateEnum>>,
        Option<CounterMessageSet>,
    ) {
        trace!("Uninit handle message");
        (None, None)
    }

    fn on_entry(&self, _data: &mut CounterExtendedState, _self_id: &u16) {
        trace!("State on_entry: {:?}", self);
        info!("This is the Blox Shutdown");
    }

    fn on_exit(&self, _data: &mut CounterExtendedState, _self_id: &u16) {
        trace!("State on_exit: {:?}", self);
        info!("This is the Blox Initialization");
    }
}
