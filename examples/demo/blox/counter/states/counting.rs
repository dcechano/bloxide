// Copyright 2025 Bloxide, all rights reserved

use crate::blox::counter::{components::*, ext_state::*, messaging::*, states::*};
use bloxide::core::messaging::*;
use bloxide::core::state_machine::*;
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Counting;

impl State<CounterComponents> for Counting {
    fn parent(&self) -> CounterStateEnum {
        CounterStateEnum::Uninit(Uninit)
    }

    fn handle_message(
        &self,
        message: CounterMessageSet,
        data: &mut CounterExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<CounterStateEnum>>,
        Option<CounterMessageSet>,
    ) {
        trace!("[Counting] handle_message: {:?}", message);

        match message {
            CounterMessageSet::CounterMessage(msg) => match &msg.payload {
                CounterPayload::Increment(amount) => {
                    self.do_increment(**amount, data);
                    if data.count >= data.max {
                        for subscriber in data.subscribers.iter() {
                            let _ = subscriber.try_send(Message::new(
                                *self_id,
                                CounterPayload::CountEvent(Box::new(CountEvent::MaxReached)),
                            ));
                        }
                        (
                            Some(Transition::To(CounterStateEnum::Finished(Finished))),
                            None,
                        )
                    } else {
                        (None, None)
                    }
                }
                CounterPayload::Decrement(amount) => {
                    self.do_decrement(**amount, data);
                    if data.count <= data.min {
                        for subscriber in data.subscribers.iter() {
                            let _ = subscriber.try_send(Message::new(
                                *self_id,
                                CounterPayload::CountEvent(Box::new(CountEvent::MinReached)),
                            ));
                        }
                        (
                            Some(Transition::To(CounterStateEnum::Finished(Finished))),
                            None,
                        )
                    } else {
                        (None, None)
                    }
                }
                CounterPayload::CountEvent(event) => match **event {
                    CountEvent::GetCount => {
                        debug!("[Counting] Current count: {} Max: {}", data.count, data.max);
                        for subscriber in data.subscribers.iter() {
                            let _ = subscriber.try_send(Message::new(
                                *self_id,
                                CounterPayload::SetCount(Box::new(data.count)),
                            ));
                        }
                        (None, None)
                    }
                    CountEvent::Reset => (
                        Some(Transition::To(CounterStateEnum::NotStarted(NotStarted))),
                        None,
                    ),
                    _ => (None, None),
                },

                _ => (None, None),
            },
            _ => (None, None),
        }
    }
}

impl Counting {
    fn do_increment(&self, amount: usize, data: &mut CounterExtendedState) {
        data.count += amount;
        debug!("[Counting] Incremented by {} to {}", amount, data.count);
    }

    fn do_decrement(&self, amount: usize, data: &mut CounterExtendedState) {
        data.count -= amount;
        debug!("[Counting] Decremented by {} to {}", amount, data.count);
    }
}
