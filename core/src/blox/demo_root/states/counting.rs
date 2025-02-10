// Copyright 2025 Bloxide, all rights reserved

use super::{RootComponents, RootStates};
use crate::blox::demo_counter::messaging::*;
use crate::blox::demo_root::{messaging::*, states::*};
use crate::components::Runtime;
use crate::{components::*, messaging::*, state_machine::*};

use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Counting;

impl<R: Runtime> State<RootComponents<R>> for Counting
where
    R::MessageHandle<StandardPayload<R>>: Clone + Send + 'static,
    <R::MessageHandle<StandardPayload<R>> as MessageSender>::ReceiverType: Send,
    R::MessageHandle<CounterPayload>: Clone + Send + 'static,
    <R::MessageHandle<CounterPayload> as MessageSender>::ReceiverType: Send,
{
    fn parent(&self) -> RootStates {
        RootStates::Uninit(Uninit)
    }

    fn handle_message(
        &self,
        state_machine: &mut StateMachine<RootComponents<R>>,
        msg: <RootComponents<R> as Components>::MessageSet,
    ) -> Option<Transition<RootStates, <RootComponents<R> as Components>::MessageSet>> {
        match msg {
            RootMessageSet::CounterMessage(msg) => match msg.payload {
                CounterPayload::CountEvent(event) => match *event {
                    CountEvent::MaxReached => Some(Transition::To(RootStates::Finished(Finished))),
                    CountEvent::MinReached => Some(Transition::To(RootStates::Finished(Finished))),
                    _ => None,
                },
                CounterPayload::SetCount(count) => {
                    info!("Current count: {}", count);
                    let _ = state_machine
                        .extended_state
                        .counter_handle
                        .as_ref()
                        .unwrap()
                        .try_send(Message::new(
                            state_machine.self_handles.standard_handle.id(),
                            CounterPayload::Increment(Box::new(1)),
                        ));
                    let _ = state_machine
                        .extended_state
                        .counter_handle
                        .as_ref()
                        .unwrap()
                        .try_send(Message::new(
                            state_machine.self_handles.standard_handle.id(),
                            CounterPayload::CountEvent(Box::new(CountEvent::GetCount)),
                        ));
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }
}
