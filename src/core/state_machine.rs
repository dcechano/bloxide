// Copyright 2025 Bloxide, all rights reserved

use crate::{core::components::*, std_exports::*};
use log::*;

pub trait StateEnum: Default + fmt::Debug {
    fn new() -> Self {
        Self::default()
    }
}

pub trait ExtendedState {
    type InitArgs;
    fn new(args: Self::InitArgs) -> Self;
}

/// Used as `Option<Transition<T>>`, `None` = No transition
/// Errors handled as transitions to a Error state
pub enum Transition<T> {
    To(T),
    Parent,
}

pub struct StateMachine<C: Components> {
    pub current_state: C::States,
    // ExtendedState stored here to be passed to each state
    extended_state: C::ExtendedState,
}

impl<C> StateMachine<C>
where
    C: Components,
    C::States: State<C> + Clone + PartialEq + Default,
{
    pub fn new(extended_state: C::ExtendedState) -> Self {
        Self {
            current_state: C::States::default(),
            extended_state,
        }
    }

    // Initializes the state machine, performs the Uninit state transition
    pub fn init(&mut self, uninit: &C::States, self_id: &u16, entry_point: &C::States) {
        uninit.on_exit(&mut self.extended_state, self_id);
        self.change_state(entry_point.clone(), self_id);
    }

    // This is how messages get handled.  Structured so it can be called recursively for Parent message handling
    pub fn dispatch(&mut self, message: C::MessageSet, state: &C::States, self_id: &u16)
    where
        C::States: State<C>,
    {
        let (transition, message_option) =
            state.handle_message(message, &mut self.extended_state, self_id);
        match (transition, message_option) {
            (Some(Transition::Parent), Some(message)) => {
                trace!("Transitioning to parent state");
                self.dispatch(message, &state.parent(), self_id);
            }
            (Some(Transition::Parent), None) => {
                panic!("Transition to parent state without a message");
            }
            (Some(Transition::To(new_state)), _) => {
                trace!("Transitioning to state: {:?}", new_state);
                self.change_state(new_state, self_id);
            }
            _ => {
                // Do nothing if transition is None regardless of message
            }
        }
    }

    fn build_state_path(&self, start_state: C::States) -> Vec<C::States> {
        let mut path = Vec::new();
        let mut current = Some(start_state);
        while let Some(s) = current {
            path.push(s.clone());
            current = match s.parent() {
                p if p == s => None, // Check if parent equals self (for Uninit state)
                p => Some(p),
            };
        }
        path.reverse();
        path
    }

    fn find_lca_index(&self, current_path: &[C::States], dest_path: &[C::States]) -> usize {
        let mut lca_index = 0;
        while lca_index < current_path.len().min(dest_path.len())
            && current_path[lca_index] == dest_path[lca_index]
        {
            lca_index += 1;
        }
        lca_index
    }

    // This is how states get changed
    // Traverses the on_exit and on_entry functions
    fn change_state(&mut self, new_state: C::States, self_id: &u16) {
        // Build current state path
        let current_path = self.build_state_path(self.current_state.clone());
        trace!("Current state path: {:?}", current_path);

        // Build destination state path
        let dest_path = self.build_state_path(new_state.clone());
        trace!("Destination state path: {:?}", dest_path);

        // Find LCA
        let lca_index = self.find_lca_index(&current_path, &dest_path);
        trace!("LCA index: {:?}", lca_index);

        // Exit from current state up to (but not including) LCA
        for state in current_path[lca_index..].iter().rev() {
            state.on_exit(&mut self.extended_state, self_id);
        }

        // Enter from LCA down to destination
        for state in dest_path[lca_index..].iter() {
            state.on_entry(&mut self.extended_state, self_id);
        }

        // Set new current state
        self.current_state = new_state;
    }
}

pub trait State<C: Components>: fmt::Debug + 'static {
    // Default on_entry and on_exit functions do nothing, only need to be overridden if needed
    fn on_entry(&self, _data: &mut C::ExtendedState, _self_id: &u16) {
        trace!("State on_entry: {:?}", self);
    }

    fn on_exit(&self, _data: &mut C::ExtendedState, _self_id: &u16) {
        trace!("State on_exit: {:?}", self);
    }

    // All states must have a parent state
    fn parent(&self) -> C::States {
        panic!("No parent for this state");
    }

    fn handle_message(
        &self,
        message: C::MessageSet,
        data: &mut C::ExtendedState,
        self_id: &u16,
    ) -> (Option<Transition<C::States>>, Option<C::MessageSet>);
}
