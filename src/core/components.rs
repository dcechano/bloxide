// Copyright 2025 Bloxide, all rights reserved

use crate::{
    core::{messaging::*, state_machine::*},
    runtime::*,
    std_exports::*,
    supervisor::messaging::*,
};

/// A trait to encapsulate types needed for a blox
pub trait Components {
    /// The extended state of the blox.
    /// Different from the state enum in that
    /// ExtendedState is for holding data while StateEnum isn't.
    type ExtendedState: ExtendedState;
    /// The state enum of the blox.
    /// Represents the different states the blox can be in.
    type States: StateEnum + Default;
    /// The message set of the blox.
    type MessageSet: MessageSet;
    /// The receivers of the blox
    /// The blox should receive messages from the receivers in this type.
    type Receivers;
    /// The handles of the blox
    /// The blox should hand out handles of this type to other bloxes
    /// so they can send messages to the blox.
    type Handles;
}

///The main blox struct.  Bloxes are differentiated by their components
///Anything that all Bloxes should have is stored here
pub struct Blox<C: Components> {
    pub handle: StandardMessageHandle,
    pub state_machine: StateMachine<C>,
    pub receivers: C::Receivers,
}

impl<C> Blox<C>
where
    C: Components,
    C::States: State<C> + Clone + PartialEq + Default,
    C::ExtendedState: ExtendedState,
    StandardMessageHandle: MessageSender,
{
    pub fn new(
        standard_handle: StandardMessageHandle,
        receivers: C::Receivers,
        extended_state: C::ExtendedState,
        self_handles: C::Handles,
    ) -> Self {
        Self {
            handle: standard_handle.clone(),
            state_machine: StateMachine::<C>::new(extended_state, self_handles),
            receivers,
        }
    }
}

/// The core engine that runs the blox. The blox should receive messages from the receivers in the `run` method.
/// Implement Runnable or RunnableLocal depending on if the blox implements Send
pub trait Runnable<B: Components> {
    /// The main loop of the blox. Receive messages from the receivers and process them here.
    fn run(self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    /// Convert the blox into a request to be sent to the supervisor.
    fn into_request(self: Box<Self>) -> SupervisorPayload
    where
        Self: Send + 'static,
    {
        let closure = move || {
            Box::pin(async move { self.run().await })
                as Pin<Box<dyn Future<Output = ()> + Send + 'static>>
        };

        SupervisorPayload::Spawn(Box::new(closure))
    }
}

/// The non-send counterpart to Runnable.
pub trait RunnableLocal<B: Components> {
    /// The main loop of the blox. Receive messages from the receivers and process them here.
    fn run_local(self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + 'static>>;

    /// Convert the blox into a request to be sent to the supervisor.
    fn into_request(self: Box<Self>) -> SupervisorLocalPayload
    where
        Self: Send + 'static,
    {
        let closure = move || {
            Box::pin(async move { self.run_local().await })
                as Pin<Box<dyn Future<Output = ()> + 'static>>
        };

        SupervisorLocalPayload::SpawnLocal(Box::new(closure))
    }
}
