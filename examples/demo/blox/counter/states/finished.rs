// Copyright 2025 Bloxide, all rights reserved

use crate::blox::counter::{components::*, ext_state::*, messaging::*, states::*};
use bloxide::core::state_machine::*;
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Finished;

impl State<CounterComponents> for Finished {
    fn parent(&self) -> CounterStateEnum {
        CounterStateEnum::Idle(Idle)
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
        trace!("[Finished] handle_message: {:?}", message);
        match message {
            CounterMessageSet::CounterMessage(msg) => match &msg.payload {
                CounterPayload::CountEvent(event) => match **event {
                    CountEvent::Reset => {
                        data.count = 0;
                        (
                            Some(Transition::To(CounterStateEnum::NotStarted(NotStarted))),
                            None,
                        )
                    }
                    _ => (None, None),
                },
                _ => (None, None),
            },
            _ => (None, None),
        }
    }

    fn on_entry(&self, data: &mut CounterExtendedState, _self_id: &u16) {
        trace!("State on_entry: {:?}", self);
        info!("Finished!");
        info!("Count is {}", data.count);
    }

    fn on_exit(&self, _data: &mut CounterExtendedState, _self_id: &u16) {
        trace!("State on_exit: {:?}", self);
    }
}
