// Copyright 2025 Bloxide, all rights reserved

use super::{RootComponents, RootStates};
use crate::blox::counter::messaging::*;
use crate::blox::root::{messaging::*, states::*};
use bloxide::core::{messaging::*, state_machine::*};

use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Counting;

impl State<RootComponents> for Counting {
    fn parent(&self) -> RootStates {
        RootStates::Uninit(Uninit)
    }

    fn handle_message(
        &self,
        msg: <RootComponents as Components>::MessageSet,
        data: &mut <RootComponents as Components>::ExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<RootStates>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        match msg {
            RootMessage::CounterMessage(msg) => match msg.payload {
                CounterPayload::CountEvent(event) => match *event {
                    CountEvent::MaxReached => {
                        (Some(Transition::To(RootStates::Finished(Finished))), None)
                    }
                    CountEvent::MinReached => {
                        (Some(Transition::To(RootStates::Finished(Finished))), None)
                    }
                    _ => (None, None),
                },
                CounterPayload::SetCount(count) => {
                    info!("Current count: {}", count);
                    let _ = data.counter_handle.as_ref().unwrap().try_send(Message::new(
                        *self_id,
                        CounterPayload::Increment(Box::new(1)),
                    ));
                    let _ = data.counter_handle.as_ref().unwrap().try_send(Message::new(
                        *self_id,
                        CounterPayload::CountEvent(Box::new(CountEvent::GetCount)),
                    ));
                    (None, None)
                }
                _ => (None, None),
            },
            _ => (None, None),
        }
    }
}
