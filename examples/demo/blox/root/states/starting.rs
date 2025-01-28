// Copyright 2025 Bloxide, all rights reserved

use super::{RootComponents, RootStates};
use crate::blox::counter::{components::*, ext_state::*, messaging::*};
use crate::blox::root::{messaging::*, states::*};
use bloxide::{
    core::{messaging::*, state_machine::*},
    runtime::*,
    supervisor::*,
};
use log::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Starting;

impl State<RootComponents> for Starting {
    fn parent(&self) -> RootStates {
        RootStates::Idle(Idle)
    }

    fn handle_message(
        &self,
        msg: <RootComponents as Components>::MessageSet,
        data: &mut <RootComponents as Components>::ExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::States>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        match msg {
            RootMessage::StandardMessage(msg) => match msg.payload {
                StandardPayload::StandardChannel(new_standard_handle, standard_receiver) => {
                    let (counter_handle, counter_receiver) = Handle::create_channel_with_size(
                        new_standard_handle.dest_id,
                        DEFAULT_CHANNEL_SIZE,
                    );

                    data.counter_handle = Some(counter_handle.clone());

                    let counter_receivers = CounterReceivers {
                        standard_receiver,
                        counter_receiver,
                    };

                    let mut counter_extended_state = CounterExtendedState::new(());
                    counter_extended_state
                        .subscribers
                        .push(data.self_counter_handle.clone());

                    let counter_blox = Blox::<CounterComponents>::new(
                        new_standard_handle.clone(),
                        counter_receivers,
                        counter_extended_state,
                    );

                    let spawn_request = Box::new(counter_blox).into_request();

                    let _ = get_supervisor_handle().try_send(Message::new(*self_id, spawn_request));

                    let _ = data
                        .counter_handle
                        .as_ref()
                        .unwrap()
                        .try_send(Message::new(*self_id, CounterPayload::SetMax(Box::new(4))));

                    (Some(Transition::To(RootStates::Counting(Counting))), None)
                }
                _ => (None, None),
            },
            _ => (None, None),
        }
    }

    fn on_entry(&self, _data: &mut <RootComponents as Components>::ExtendedState, self_id: &u16) {
        trace!("State on_entry: {:?}", self);
        // Request a new standard handle to start the counter blox
        let _ = get_supervisor_handle().try_send(Message::new(
            *self_id,
            SupervisorPayload::RequestNewStandardHandle(DEFAULT_CHANNEL_SIZE),
        ));
    }

    fn on_exit(&self, data: &mut <RootComponents as Components>::ExtendedState, self_id: &u16) {
        trace!("State on_exit: {:?}", self);
        // Start the counter ping pong
        let _ = data.counter_handle.as_ref().unwrap().try_send(Message::new(
            *self_id,
            CounterPayload::CountEvent(Box::new(CountEvent::StartCounting)),
        ));
    }
}
