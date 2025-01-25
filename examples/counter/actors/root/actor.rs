use bloxide::core::actor::*;
use bloxide::core::messaging::*;
use bloxide::runtime::*;
use bloxide::std_exports::*;
use log::*;

use crate::CounterPayload;

use super::ext_state::RootExtState;
use super::messaging::RootMessage;
use super::state::RootStates;

pub struct RootActorConfig {
    pub std_rx: mpsc::Receiver<Message<StandardPayload>>,
    pub counter_rx: mpsc::Receiver<Message<CounterPayload>>,
}

pub struct RootComponents;

impl Components for RootComponents {
    type StateEnum = RootStates;
    type MessageSet = RootMessage;
    type ExtendedState = RootExtState;
    type ActorConfig = RootActorConfig;
}

impl Runnable<RootComponents> for Actor<RootComponents> {
    fn run_actor(mut self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(async move {
            loop {
                select! {
                    Some(msg) = self.config.std_rx.recv() => {
                        trace!("Root Actor received a Standard message: {msg:?}");
                        let current_state = self.state_machine.current_state.clone();
                        self.state_machine.dispatch(RootMessage::StandardMessage(msg), &current_state, &self.handle.dest_id());
                    }
                    Some(msg) = self.config.counter_rx.recv() => {
                        trace!("Root Actor received a Counter message: {msg:?}");
                        let current_state = self.state_machine.current_state.clone();
                        self.state_machine.dispatch(RootMessage::CounterMessage(msg), &current_state, &self.handle.dest_id());
                    }
                    else => {
                        break;
                    }
                }
            }
        })
    }
}
