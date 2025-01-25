// Copyright 2025 Bloxide, all rights reserved

mod actors;

use crate::actors::counter::*;
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

    //ROOT ACTOR SETUP////////////////////////////////////////
    // Make handles
    //TODO: Integrate these steps into one new() function
    let (counter_standard_sender, counter_standard_receiver) =
        mpsc::channel::<Message<StandardPayload>>(DEFAULT_CHANNEL_SIZE);
    let (counter_counter_sender, counter_receiver) =
        mpsc::channel::<Message<CounterPayload>>(DEFAULT_CHANNEL_SIZE);
    let counter_standard_handle = StandardMessageHandle::new(1, counter_standard_sender);
    let counter_handle = CounterHandle::new(1, counter_counter_sender);

    // Create actor config
    let counter_config = CounterActorConfig {
        standard_receiver: counter_standard_receiver,
        counter_receiver: counter_receiver,
        counter_handle: counter_handle.clone(),
    };

    // Create the actor
    let counter_actor = Actor::<CounterComponents>::new(
        counter_standard_handle.clone(),
        CounterExtendedState::new(),
        counter_config,
    );

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

    let root_spawn_fn = Box::new(|| -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            // Move the actor in so it’s owned:
            Box::new(counter_actor).run_actor().await;
        })
    });

    let supervisor_extended_state =
        SupervisorExtendedState::special_new(counter_standard_handle, root_spawn_fn, supervisor_supervisor_handle);
    let supervisor_actor = Actor::<SupervisorComponents>::new(
        supervisor_standard_handle.clone(),
        supervisor_extended_state,
        supervisor_config,
    );


    tokio::spawn(async move {
        let _ = Box::new(supervisor_actor).run_actor().await;
    });

    let _ = supervisor_standard_handle.try_send(Message::new(0, StandardPayload::Initialize));

    sleep(Duration::from_millis(100)).await;

    // Send some counter messages
    let _ = counter_handle.try_send(Message::new(0, CounterPayload::SetMax(Box::new(5))));

    let _ = counter_handle.try_send(Message::new(0, CounterPayload::SetMin(Box::new(0))));

    let _ = counter_handle.try_send(Message::new(
        0,
        CounterPayload::CountEvent(Box::new(CountEvent::StartCounting)),
    ));
    let _ = counter_handle.try_send(Message::new(0, CounterPayload::Increment(Box::new(3))));

    let _ = counter_handle.try_send(Message::new(
        0,
        CounterPayload::CountEvent(Box::new(CountEvent::GetCount)),
    ));

    let _ = counter_handle.try_send(Message::new(0, CounterPayload::Decrement(Box::new(1))));

    let _ = counter_handle.try_send(Message::new(
        0,
        CounterPayload::CountEvent(Box::new(CountEvent::GetCount)),
    ));

    let _ = counter_handle.try_send(Message::new(0, CounterPayload::Increment(Box::new(4))));

    //TODO: Use new ctrl-c macro
    sleep(Duration::from_secs(2)).await;
    info!("Done!");
}
