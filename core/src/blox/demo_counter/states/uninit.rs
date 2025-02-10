// Copyright 2025 Bloxide, all rights reserved

use super::*;
use crate::blox::demo_counter::{components::*, messaging::*};
use crate::{messaging::*, state_machine::*};
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Uninit;

impl<R: Runtime> State<CounterComponents<R>> for Uninit
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
        trace!("Uninit handle message");
        None
    }

    fn on_entry(&self, _data: &mut StateMachine<CounterComponents<R>>) {
        trace!("State on_entry: {:?}", self);
        info!("This is the Blox Shutdown");
    }

    fn on_exit(&self, _data: &mut StateMachine<CounterComponents<R>>) {
        trace!("State on_exit: {:?}", self);
        info!("This is the Blox Initialization");
    }
}
