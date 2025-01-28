// Copyright 2025 Bloxide, all rights reserved

#[cfg(feature = "runtime-tokio")]
use super::{components::*, messaging::*, states::*};
#[cfg(feature = "runtime-tokio")]
use crate::{core::components::*, runtime::*, std_exports::*};
#[cfg(feature = "runtime-tokio")]
use log::*;

#[cfg(feature = "runtime-tokio")]
use std::sync::OnceLock;

#[cfg(feature = "runtime-tokio")]
pub type SupervisorHandle = TokioHandle<SupervisorPayload>;
#[cfg(feature = "runtime-tokio")]
pub type SupervisorLocalHandle = TokioHandle<SupervisorLocalPayload>;

#[cfg(feature = "runtime-tokio")]
impl Runnable<SupervisorComponents> for Blox<SupervisorComponents> {
    fn run(mut self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        self.state_machine.init(
            &SupervisorStateEnum::Uninit(Uninit),
            &self.handle.dest_id(),
            &SupervisorStateEnum::Running(Running),
        );
        Box::pin(async move {
            trace!("Supervisor started. Listening for messages...");
            loop {
                select! {
                    Some(message) = self.receivers.standard_receiver.recv() => {
                        let current_state = self.state_machine.current_state.clone();
                        self.state_machine.dispatch(SupervisorMessageSet::StandardMessage(message), &current_state, &self.handle.dest_id());
                    },
                    Some(message) = self.receivers.supervisor_receiver.recv() => {
                        let current_state = self.state_machine.current_state.clone();
                        self.state_machine.dispatch(SupervisorMessageSet::SupervisorMessage(message), &current_state, &self.handle.dest_id());
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

#[cfg(feature = "runtime-tokio")]
pub static SUPERVISOR_HANDLE: OnceLock<SupervisorHandle> = OnceLock::new();

#[cfg(feature = "runtime-tokio")]
thread_local! {
    pub static SUPERVISORLOCAL_HANDLE: OnceCell<SupervisorLocalHandle> = const {OnceCell::new()};
}

#[cfg(feature = "runtime-tokio")]
pub fn init_supervisor_handle(handle: SupervisorHandle) {
    SUPERVISOR_HANDLE
        .set(handle)
        .expect("Supervisor handle can only be initialized once!");
}

#[cfg(feature = "runtime-tokio")]
pub fn get_supervisor_handle() -> &'static SupervisorHandle {
    SUPERVISOR_HANDLE
        .get()
        .expect("Supervisor handle not initialized!")
}

#[cfg(feature = "runtime-tokio")]
pub fn init_local_supervisor_handle(handle: SupervisorLocalHandle) {
    SUPERVISORLOCAL_HANDLE.with(|cell| {
        cell.set(handle)
            .expect("Supervisor handle already initialized in this thread!");
    });
}

#[cfg(feature = "runtime-tokio")]
pub fn get_local_supervisor_handle() -> SupervisorLocalHandle {
    SUPERVISORLOCAL_HANDLE.with(|cell| {
        cell.get()
            .expect("Supervisor handle not initialized in this thread!")
            .clone()
    })
}

#[cfg(feature = "runtime-embassy")]
use crate::runtime::*;
#[cfg(feature = "runtime-embassy")]
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
#[cfg(feature = "runtime-embassy")]
pub type SupervisorMutex = CriticalSectionRawMutex;
#[cfg(feature = "runtime-embassy")]
pub const SUPERVISOR_CHANNEL_SIZE: usize = 16;
#[cfg(feature = "runtime-embassy")]
use super::messaging::*;
#[cfg(feature = "runtime-embassy")]
pub type SupervisorHandle =
    EmbassyHandle<SupervisorPayload, SupervisorMutex, SUPERVISOR_CHANNEL_SIZE>;
#[cfg(feature = "runtime-embassy")]
pub type SupervisorLocalHandle =
    EmbassyHandle<SupervisorLocalPayload, SupervisorMutex, SUPERVISOR_CHANNEL_SIZE>;
