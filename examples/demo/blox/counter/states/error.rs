// Copyright 2025 Bloxide, all rights reserved

use crate::blox::counter::{components::*, ext_state::*, messaging::*, states::*};
use bloxide::core::state_machine::*;
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Error;

impl State<CounterComponents> for Error {
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
        trace!("[Error] handle_message: {:?}", message);
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
}
