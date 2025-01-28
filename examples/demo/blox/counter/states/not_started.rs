// Copyright 2025 Bloxide, all rights reserved

use super::*;
use crate::blox::counter::{components::*, ext_state::*, messaging::*};
use bloxide::core::{messaging::*, state_machine::*};
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct NotStarted;

impl State<CounterComponents> for NotStarted {
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
        trace!("[Idle] handle_message: {:?}", message);
        match message {
            CounterMessageSet::CounterMessage(msg) => self.handle_counter_msg(msg, data, _self_id),
            _ => (None, None),
        }
    }
}

impl NotStarted {
    fn handle_counter_msg(
        &self,
        msg: Message<CounterPayload>,
        data: &mut CounterExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<CounterStateEnum>>,
        Option<CounterMessageSet>,
    ) {
        match &msg.payload {
            CounterPayload::SetCount(new_value) => {
                data.count = **new_value;
                debug!("State: {:?} Set count to {}", self, data.count);
                (None, None)
            }
            CounterPayload::SetMax(new_max) => {
                data.max = **new_max;
                debug!("State: {:?} New max set to {}", self, data.max);
                (None, None)
            }
            CounterPayload::SetMin(new_min) => {
                data.min = **new_min;
                debug!("State: {:?} New min set to {}", self, data.min);
                (None, None)
            }
            CounterPayload::CountEvent(event) => {
                trace!("State: {:?} Received CountEvent: {:?}", self, event);
                match **event {
                    CountEvent::GetCount => {
                        debug!("State: {:?} Current count: {}", self, data.count);

                        (None, None)
                    }
                    CountEvent::StartCounting => {
                        data.subscribers.iter().for_each(|subscriber| {
                            let _ = subscriber.try_send(Message::new(
                                *self_id,
                                CounterPayload::SetCount(Box::new(data.count)),
                            ));
                        });
                        (
                            Some(Transition::To(CounterStateEnum::Counting(Counting))),
                            None,
                        )
                    }
                    _ => (None, None),
                }
            }
            _ => (None, None),
        }
    }
}
