// Copyright 2025 Bloxide, all rights reserved

use crate::blox::demo_counter::{components::*, messaging::*, states::*};
use crate::{messaging::*, state_machine::*};

#[derive(Clone, PartialEq, Debug)]
pub struct Error;

impl<R: Runtime> State<CounterComponents<R>> for Error
where
    <R::MessageHandle<StandardPayload<R>> as MessageSender>::ReceiverType: Send + 'static,
    <R::MessageHandle<CounterPayload> as MessageSender>::ReceiverType: Send + 'static,
{
    fn parent(&self) -> CounterStateEnum {
        CounterStateEnum::Idle(Idle)
    }

    fn handle_message(
        &self,
        state_machine: &mut StateMachine<CounterComponents<R>>,
        message: CounterMessageSet<R>,
    ) -> Option<Transition<CounterStateEnum, CounterMessageSet<R>>> {
        match message {
            CounterMessageSet::CounterMessage(msg) => match &msg.payload {
                CounterPayload::CountEvent(event) => match **event {
                    CountEvent::Reset => {
                        state_machine.extended_state.count = 0;
                        Some(Transition::To(CounterStateEnum::NotStarted(NotStarted)))
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}
