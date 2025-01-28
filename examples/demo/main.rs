// Copyright 2025 Bloxide, all rights reserved

pub mod blox;

use crate::blox::root::{components::*, ext_state::*};
use bloxide::{
    core::{components::*, messaging::*, state_machine::*},
    runtime::*,
    std_exports::*,
    supervisor::*,
};
use log::*;

#[::tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    // ROOT SETUP - This Blox turns into the program's entry point
    let (root_standard_handle, root_standard_receiver) =
        Handle::create_channel_with_size(1, DEFAULT_CHANNEL_SIZE);

    let (root_counter_handle, root_counter_receiver) =
        Handle::create_channel_with_size(1, DEFAULT_CHANNEL_SIZE);

    let root_receivers = RootReceivers {
        std_rx: root_standard_receiver,
        counter_rx: root_counter_receiver,
    };

    let root_init_args = RootInitArgs {
        self_counter_handle: root_counter_handle,
        counter_handle: None,
    };

    let root_extended_state = RootExtState::new(root_init_args);

    let root_blox = Blox::<RootComponents>::new(
        root_standard_handle.clone(),
        root_receivers,
        root_extended_state,
    );

    let root_spawn_fn = Box::new(|| -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            Box::new(root_blox).run().await;
        })
    });

    // SUPERVISOR SETUP
    let (supervisor_standard_handle, standard_receiver) =
        Handle::create_channel_with_size(0, DEFAULT_CHANNEL_SIZE);
    let (supervisor_supervisor_handle, supervisor_receiver) =
        Handle::create_channel_with_size(0, DEFAULT_CHANNEL_SIZE);
    init_supervisor_handle(supervisor_supervisor_handle);

    let supervisor_receivers = SupervisorReceivers {
        standard_receiver,
        supervisor_receiver,
    };

    let supervisor_init_args = SupervisorInitArgs {
        root_standard_handle,
        root_spawn_fn,
    };

    let supervisor_extended_state = SupervisorExtendedState::new(supervisor_init_args);
    let supervisor = Blox::<SupervisorComponents>::new(
        supervisor_standard_handle.clone(),
        supervisor_receivers,
        supervisor_extended_state,
    );

    // Spawn the supervisor.  It will automatically spawn the root blox.
    tokio::spawn(async move {
        let _ = Box::new(supervisor).run().await;
    });

    sleep(Duration::from_secs(2)).await;
    info!("Done!");
}
