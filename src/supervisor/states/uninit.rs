// Copyright 2025 Bloxide, all rights reserved

use super::*;
#[cfg(feature = "runtime-tokio")]
use crate::runtime::*;
use crate::{core::state_machine::*, std_exports::*};
use log::*;
#[derive(Clone, PartialEq, Debug)]
pub struct Uninit;

impl State<SupervisorComponents> for Uninit {
    fn parent(&self) -> SupervisorStateEnum {
        SupervisorStateEnum::Uninit(Uninit)
    }
    fn handle_message(
        &self,
        _message: SupervisorMessageSet,
        _data: &mut SupervisorExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<SupervisorStateEnum>>,
        Option<SupervisorMessageSet>,
    ) {
        trace!("Uninit handle message");
        //Uninit never handles messages
        (None, None)
    }
    fn on_entry(&self, _data: &mut SupervisorExtendedState, _self_id: &u16) {
        trace!("State on_entry: {:?}", self);
        info!("This is the Blox Shutdown");
    }
    fn on_exit(&self, data: &mut SupervisorExtendedState, _self_id: &u16) {
        trace!("State on_exit: {:?}", self);
        info!("This is the Blox Initialization");

        if let Some(spawn_fn) = data.root_spawn_fn.take() {
            trace!("Running root spawn function");
            let future = spawn_fn();
            self.spawn_root(future);
        } else {
            panic!("Root spawn function not found");
        }
    }
}

#[cfg(feature = "runtime-tokio")]
impl Uninit {
    fn spawn_root(&self, future: Pin<Box<dyn Future<Output = ()> + Send>>) {
        spawn(future);
    }
}

#[cfg(feature = "runtime-embassy")]
impl Uninit {
    fn spawn_root(&self, _future: Pin<Box<dyn Future<Output = ()> + Send>>) {
        todo!()
    }
}
