use crate::*;

use bloxide::{Components, State, StateEnum, Transition};

use super::{actor::RootComponents, messaging::RootMessage};

use bloxide::core::actor::*;
use bloxide::core::messaging::*;
use bloxide::runtime::*;
use bloxide::supervisor::*;

#[derive(Clone, PartialEq, Debug)]
pub enum RootStates {
    Uninit(Uninit),
    Idle(Idle),
    Starting(Starting),
    Counting(Counting),
    Finished(Finished),
    Error(Error),
}

impl State<RootComponents> for RootStates {
    fn handle_message(
        &self,
        message: <RootComponents as Components>::MessageSet,
        data: &mut <RootComponents as Components>::ExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::StateEnum>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        match self {
            RootStates::Uninit(uninit) => uninit.handle_message(message, data, self_id),
            RootStates::Counting(counting) => counting.handle_message(message, data, self_id),
            RootStates::Finished(finished) => finished.handle_message(message, data, self_id),
            RootStates::Error(error) => error.handle_message(message, data, self_id),
            RootStates::Idle(idle) => idle.handle_message(message, data, self_id),
            RootStates::Starting(starting) => starting.handle_message(message, data, self_id),
        }
    }

    fn on_entry(&self, data: &mut <RootComponents as Components>::ExtendedState, self_id: &u16) {
        match self {
            RootStates::Uninit(s) => s.on_entry(data, self_id),
            RootStates::Counting(s) => s.on_entry(data, self_id),
            RootStates::Finished(s) => s.on_entry(data, self_id),
            RootStates::Error(s) => s.on_entry(data, self_id),
            RootStates::Idle(s) => s.on_entry(data, self_id),
            RootStates::Starting(s) => s.on_entry(data, self_id),
        }
    }

    fn on_exit(&self, data: &mut <RootComponents as Components>::ExtendedState, self_id: &u16) {
        match self {
            RootStates::Uninit(uninit) => uninit.on_exit(data, self_id),
            RootStates::Counting(counting) => counting.on_exit(data, self_id),
            RootStates::Finished(finished) => finished.on_exit(data, self_id),
            RootStates::Error(error) => error.on_exit(data, self_id),
            RootStates::Idle(idle) => idle.on_exit(data, self_id),
            RootStates::Starting(starting) => starting.on_exit(data, self_id),
        }
    }

    fn parent(&self) -> RootStates {
        match self {
            RootStates::Uninit(s) => s.parent(),
            RootStates::Idle(s) => s.parent(),
            RootStates::Starting(s) => s.parent(),
            RootStates::Counting(s) => s.parent(),
            RootStates::Finished(s) => s.parent(),
            RootStates::Error(s) => s.parent(),
        }
    }
}

impl StateEnum for RootStates {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for RootStates {
    fn default() -> Self {
        RootStates::Uninit(Uninit)
    }
}

impl State<RootComponents> for Uninit {
    fn handle_message(
        &self,
        msg: <RootComponents as Components>::MessageSet,
        _data: &mut <RootComponents as Components>::ExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::StateEnum>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        match msg {
            RootMessage::StandardMessage(msg) => match msg.payload {
                StandardPayload::Initialize => {
                    (Some(Transition::To(RootStates::Starting(Starting))), None)
                }
                _ => (None, None),
            },

            _ => (None, None),
        }
    }

    fn parent(&self) -> RootStates {
        RootStates::Uninit(Uninit)
    }
}

impl State<RootComponents> for Counting {
    fn handle_message(
        &self,
        msg: <RootComponents as Components>::MessageSet,
        data: &mut <RootComponents as Components>::ExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<RootStates>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        match msg {
            RootMessage::CounterMessage(msg) => match msg.payload {
                CounterPayload::CountEvent(event) => match *event {
                    CountEvent::MaxReached => {
                        (Some(Transition::To(RootStates::Finished(Finished))), None)
                    }
                    CountEvent::MinReached => {
                        (Some(Transition::To(RootStates::Finished(Finished))), None)
                    }
                    _ => (None, None),
                },
                CounterPayload::SetCount(count) => {
                    info!("Current count: {}", count);
                    let _ = data.counter_handle.as_ref().unwrap().try_send(Message::new(
                        *self_id,
                        CounterPayload::Increment(Box::new(1)),
                    ));
                    let _ = data.counter_handle.as_ref().unwrap().try_send(Message::new(
                        *self_id,
                        CounterPayload::CountEvent(Box::new(CountEvent::GetCount)),
                    ));
                    (None, None)
                }
                _ => (None, None),
            },
            _ => (None, None),
        }
    }

    fn parent(&self) -> RootStates {
        RootStates::Uninit(Uninit)
    }
}

impl State<RootComponents> for Idle {
    fn handle_message(
        &self,
        _msg: <RootComponents as Components>::MessageSet,
        _data: &mut <RootComponents as Components>::ExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::StateEnum>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        (None, None)
    }

    fn parent(&self) -> RootStates {
        RootStates::Uninit(Uninit)
    }
}

impl State<RootComponents> for Starting {
    fn handle_message(
        &self,
        msg: <RootComponents as Components>::MessageSet,
        data: &mut <RootComponents as Components>::ExtendedState,
        self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::StateEnum>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        //Only message we care about is the handle message
        match msg {
            RootMessage::StandardMessage(msg) => match msg.payload {
                StandardPayload::StandardChannel(new_standard_handle, standard_receiver) => {
                    //Standard handle message (for the actor we just started)

                    let (counter_sender, counter_receiver) =
                        mpsc::channel::<Message<CounterPayload>>(DEFAULT_CHANNEL_SIZE);
                    let counter_handle =
                        CounterHandle::new(new_standard_handle.dest_id, counter_sender);
                    data.counter_handle = Some(counter_handle.clone());

                    let counter_config = CounterActorConfig {
                        standard_receiver: standard_receiver,
                        counter_receiver: counter_receiver,
                        self_counter_handle: counter_handle.clone(),
                    };

                    let mut counter_extended_state = CounterExtendedState::new();
                    counter_extended_state
                        .subscribers
                        .push(data.self_counter_handle.clone().unwrap());

                    let counter_actor = Actor::<CounterComponents>::new(
                        new_standard_handle.clone(),
                        counter_extended_state,
                        counter_config,
                    );

                    let spawn_request = Box::new(counter_actor).into_request();

                    let _ = get_supervisor_handle().try_send(Message::new(*self_id, spawn_request));

                    if let Err(e) = new_standard_handle
                        .try_send(Message::new(*self_id, StandardPayload::Initialize))
                    {
                        error!(
                            "Failed to send initialize message to new standard handle: {}",
                            e
                        );
                    }

                    //thread::sleep(Duration::from_millis(100)); // TODO: Bug - Counter gets SetMax before it is initialized

                    let _ = data
                        .counter_handle
                        .as_ref()
                        .unwrap()
                        .try_send(Message::new(*self_id, CounterPayload::SetMax(Box::new(4))));

                    (Some(Transition::To(RootStates::Counting(Counting))), None)
                }
                _ => (None, None),
            },
            _ => (None, None),
        }
    }

    fn parent(&self) -> RootStates {
        RootStates::Idle(Idle)
    }

    fn on_entry(&self, _data: &mut <RootComponents as Components>::ExtendedState, self_id: &u16) {
        //Send counter actor handle request
        let _ = get_supervisor_handle().try_send(Message::new(
            *self_id,
            SupervisorPayload::RequestNewActorHandle(DEFAULT_CHANNEL_SIZE),
        ));
    }

    fn on_exit(&self, data: &mut <RootComponents as Components>::ExtendedState, self_id: &u16) {
        //Start the counter actor ping pong
        let _ = data.counter_handle.as_ref().unwrap().try_send(Message::new(
            *self_id,
            CounterPayload::CountEvent(Box::new(CountEvent::StartCounting)),
        ));
    }
}

impl State<RootComponents> for Error {
    fn handle_message(
        &self,
        _msg: <RootComponents as Components>::MessageSet,
        _data: &mut <RootComponents as Components>::ExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::StateEnum>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        todo!()
    }

    fn parent(&self) -> RootStates {
        RootStates::Idle(Idle)
    }
}

impl State<RootComponents> for Finished {
    fn handle_message(
        &self,
        _msg: <RootComponents as Components>::MessageSet,
        _data: &mut <RootComponents as Components>::ExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<<RootComponents as Components>::StateEnum>>,
        Option<<RootComponents as Components>::MessageSet>,
    ) {
        //Program is finished, no more messages
        (None, None)
    }

    fn parent(&self) -> RootStates {
        RootStates::Idle(Idle)
    }

    fn on_entry(&self, _data: &mut <RootComponents as Components>::ExtendedState, _self_id: &u16) {
        info!("♫ I CAN ONLY COUNT TO 4 ♫");
    }
}

mod states {
    #[derive(Clone, PartialEq, Debug)]
    pub struct Uninit;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Idle;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Starting;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Counting;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Finished;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Error;
}

use states::*;
