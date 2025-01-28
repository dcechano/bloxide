// Copyright 2025 Bloxide, all rights reserved

use super::{components::*, messaging::*, states::*};
use bloxide::{core::components::*, runtime::*, std_exports::*};
use log::*;

pub type CounterHandle = TokioHandle<CounterPayload>;

impl Runnable<CounterComponents> for Blox<CounterComponents> {
    fn run(mut self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        self.state_machine.init(
            &CounterStateEnum::Uninit(Uninit),
            &self.handle.dest_id(),
            &CounterStateEnum::NotStarted(NotStarted),
        );
        Box::pin(async move {
            loop {
                select! {
                    Some(message) = self.receivers.standard_receiver.recv() => {
                        let current_state = self.state_machine.current_state.clone();
                        self.state_machine.dispatch(CounterMessageSet::StandardMessage(message), &current_state, &self.handle.dest_id());
                    },
                    Some(message) = self.receivers.counter_receiver.recv() => {
                        let current_state = self.state_machine.current_state.clone();
                        self.state_machine.dispatch(CounterMessageSet::CounterMessage(message), &current_state, &self.handle.dest_id());
                    },
                    else => {
                        // If all channels closed, break out
                        trace!("All channels closed. Stopping run loop.");
                        break;
                    }
                }
            }
        })
    }
}
