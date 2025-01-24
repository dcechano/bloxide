mod actors;

use crate::actors::counter::*;
use bloxide::core::actor::*;
use bloxide::core::messaging::*;
use bloxide::runtime::*;
use log::*;

#[::tokio::main]
async fn main() {
    let _ = env_logger::builder().init();
    let (standard_sender, standard_receiver) =
        mpsc::channel::<Message<StandardPayload>>(DEFAULT_CHANNEL_SIZE);
    let (counter_sender, counter_receiver) =
        mpsc::channel::<Message<CounterPayload>>(DEFAULT_CHANNEL_SIZE);

    // Make a handle for standard messages, if needed
    let handle = StandardMessageHandle::new(1, standard_sender);
    let counter_handle = CounterHandle::new(2, counter_sender);

    // Create your actor config
    let config = CounterActorConfig {
        standard_receiver,
        counter_receiver,
        counter_handle: counter_handle.clone(),
    };

    // Create the actor
    let actor =
        Actor::<CounterComponents>::new(handle.clone(), CounterExtendedState::new(), config);

    // Spawn the actor's run loop
    if let StandardPayload::SpawnRequest(spawn_fn) = Box::new(actor).into_request() {
        spawn(spawn_fn());
        // You might want to send an Initialize message:
        let _ = handle.try_send(Message::new(999, StandardPayload::Initialize));
        sleep(Duration::from_millis(50)).await; //TODO: Bug - this delay needed to make sure the actor handles Init before other messages
    }

    // Send some counter messages:

    let _ = counter_handle.try_send(Message::new(0, CounterPayload::SetMax(Box::new(5))));
    let _ = counter_handle.try_send(Message::new(0, CounterPayload::SetMin(Box::new(0))));
    let _ = counter_handle.try_send(Message::new(
        0,
        CounterPayload::CountEvent(Box::new(CountEvent::StartCounting)),
    ));
    let _ = counter_handle.try_send(Message::new(0, CounterPayload::Increment(Box::new(3))));
    //get count
    let _ = counter_handle.try_send(Message::new(
        0,
        CounterPayload::CountEvent(Box::new(CountEvent::GetCount)),
    ));
    let _ = counter_handle.try_send(Message::new(0, CounterPayload::Decrement(Box::new(1))));
    //get count
    let _ = counter_handle.try_send(Message::new(
        0,
        CounterPayload::CountEvent(Box::new(CountEvent::GetCount)),
    ));
    let _ = counter_handle.try_send(Message::new(0, CounterPayload::Increment(Box::new(4))));

    // That might trigger a transition to Finished if the count has exceeded the new max.

    // Keep the main alive for a short while to see output
    sleep(Duration::from_secs(2)).await;
    info!("Done!");
}
