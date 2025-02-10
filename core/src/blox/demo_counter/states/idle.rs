// Copyright 2025 Bloxide, all rights reserved

use crate::blox::demo_counter::{components::*, messaging::*, states::*};
use crate::{messaging::*, state_machine::*};

#[derive(Clone, PartialEq, Debug)]
pub struct Idle;

impl<R: Runtime> State<CounterComponents<R>> for Idle
where
    <R::MessageHandle<StandardPayload<R>> as MessageSender>::ReceiverType: Send + 'static,
    <R::MessageHandle<CounterPayload> as MessageSender>::ReceiverType: Send + 'static,
{
    fn parent(&self) -> CounterStateEnum {
        CounterStateEnum::Uninit(Uninit)
    }

    fn handle_message(
        &self,
        _state_machine: &mut StateMachine<CounterComponents<R>>,
        _message: CounterMessageSet<R>,
    ) -> Option<Transition<CounterStateEnum, CounterMessageSet<R>>> {
        None
    }
}
