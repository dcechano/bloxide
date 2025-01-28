// Copyright 2025 Bloxide, all rights reserved

use crate::blox::counter::{components::*, ext_state::*, messaging::*, states::*};
use bloxide::core::state_machine::*;
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Idle;

impl State<CounterComponents> for Idle {
    fn parent(&self) -> CounterStateEnum {
        CounterStateEnum::Uninit(Uninit)
    }

    fn handle_message(
        &self,
        message: CounterMessageSet,
        _data: &mut CounterExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<CounterStateEnum>>,
        Option<CounterMessageSet>,
    ) {
        trace!("State: {:?} handle_message: {:?}", self, message);
        (None, None)
    }
}
