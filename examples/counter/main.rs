// Copyright 2025 Bloxide, all rights reserved

mod actors;

use crate::actors::counter::*;
use crate::actors::root::actor::*;
use crate::actors::root::ext_state::*;
use bloxide::core::actor::*;
use bloxide::core::messaging::*;
use bloxide::runtime::*;
use bloxide::supervisor::*;
use log::*;
use std::future::Future;
use std::pin::Pin;

#[::tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::builder().init();

    let (root_standard_sender, root_standard_receiver) =
        mpsc::channel::<Message<StandardPayload>>(DEFAULT_CHANNEL_SIZE);
    let root_standard_handle = StandardMessageHandle::new(1, root_standard_sender);

    let (root_counter_sender, root_counter_receiver) =
        mpsc::channel::<Message<CounterPayload>>(DEFAULT_CHANNEL_SIZE);
    let root_counter_handle = CounterHandle::new(1, root_counter_sender);

    let root_config = RootActorConfig {
        std_rx: root_standard_receiver,
        counter_rx: root_counter_receiver,
    };

    let root_extended_state = RootExtState::special_new(root_counter_handle);

    let root_actor = Actor::<RootComponents>::new(
        root_standard_handle.clone(),
        root_extended_state,
        root_config,
    );
    let root_spawn_fn = Box::new(|| -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            // Move the actor in so it’s owned:
            Box::new(root_actor).run_actor().await;
        })
    });

    // SUPERVISOR SETUP
    let (supervisor_standard_sender, supervisor_standard_receiver) =
        mpsc::channel::<Message<StandardPayload>>(DEFAULT_CHANNEL_SIZE);
    let supervisor_standard_handle = StandardMessageHandle::new(0, supervisor_standard_sender);
    let (supervisor_supervisor_sender, supervisor_supervisor_receiver) =
        mpsc::channel::<Message<SupervisorPayload>>(DEFAULT_CHANNEL_SIZE);
    let supervisor_supervisor_handle = SupervisorHandle::new(0, supervisor_supervisor_sender);

    let supervisor_config = SupervisorConfig {
        standard_receiver: supervisor_standard_receiver,
        supervisor_receiver: supervisor_supervisor_receiver,
    };

    let supervisor_extended_state = SupervisorExtendedState::special_new(
        root_standard_handle,
        root_spawn_fn,
        supervisor_supervisor_handle,
    );
    let supervisor_actor = Actor::<SupervisorComponents>::new(
        supervisor_standard_handle.clone(),
        supervisor_extended_state,
        supervisor_config,
    );

    tokio::spawn(async move {
        let _ = Box::new(supervisor_actor).run_actor().await;
    });

    let _ = supervisor_standard_handle.try_send(Message::new(0, StandardPayload::Initialize));

    //TODO: Use new ctrl-c macro
    sleep(Duration::from_secs(2)).await;
    info!("Done!");
}
