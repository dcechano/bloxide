// Copyright 2025 Bloxide, all rights reserved

use crate::core::actor::*;
use crate::core::messaging::*;
use crate::runtime::*;
use crate::std_exports::*;
use log::*;

#[cfg(feature = "runtime-tokio")]
pub type SupervisorHandle = TokioHandle<SupervisorPayload>;

#[cfg(feature = "runtime-tokio")]
pub type SupervisorLocalHandle = TokioHandle<SupervisorLocalPayload>;

#[cfg(feature = "runtime-tokio")]
pub struct SupervisorConfig {
    pub standard_receiver: mpsc::Receiver<Message<StandardPayload>>, //TODO: Need a receiver type alias for each runtime
    pub supervisor_receiver: mpsc::Receiver<Message<SupervisorPayload>>,
}

#[cfg(feature = "runtime-tokio")]
impl Uninit {
    fn spawn_root_actor(&self, future: Pin<Box<dyn Future<Output = ()> + Send>>) {
       spawn(future);
    }
}

#[cfg(feature = "runtime-tokio")]
impl Running {
    fn spawn_actor(&self, future: Pin<Box<dyn Future<Output = ()> + Send>>) {
        spawn(future);
    }
}

#[cfg(feature = "runtime-tokio")]
impl Runnable<SupervisorComponents> for Actor<SupervisorComponents> {
    fn run_actor(self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(async move {
            let mut this = self;
            trace!("Supervisor actor started. Listening for messages...");

            loop {
                select! {
                    Some(message) = this.config.standard_receiver.recv() => {
                        trace!("Actor received STANDARD message: {:?}", message);
                        let current_state = this.state_machine.current_state.clone();
                        this.state_machine.dispatch(SupervisorMessageSet::StandardMessage(message), &current_state, &this.handle.dest_id());
                    },
                    Some(message) = this.config.supervisor_receiver.recv() => {
                        trace!("Actor received SUPERVISOR message: {:?}", message);
                        let current_state = this.state_machine.current_state.clone();
                        this.state_machine.dispatch(SupervisorMessageSet::SupervisorMessage(message), &current_state, &this.handle.dest_id());
                    },
                    else => {
                        // If all channels closed, break out
                        trace!("All channels closed. Stopping actor run loop.");
                        break;
                    }
                }
            }
        })
    }
}

pub static SUPERVISOR_HANDLE: OnceLock<SupervisorHandle> = OnceLock::new();

//pub static SUPERVISORLOCAL_HANDLE: OnceCell<SupervisorLocalHandle> = OnceCell::new();

thread_local! {
    pub static SUPERVISORLOCAL_HANDLE: OnceCell<SupervisorLocalHandle> = OnceCell::new();

   
}

pub fn init_supervisor_handle(handle: SupervisorHandle) {
    SUPERVISOR_HANDLE
        .set(handle)
        .expect("Supervisor handle can only be initialized once!");
}

pub fn get_supervisor_handle() -> &'static SupervisorHandle {
    SUPERVISOR_HANDLE
        .get()
        .expect("Supervisor handle not initialized!")
}

pub fn init_local_supervisor_handle(handle: SupervisorLocalHandle) {
    SUPERVISORLOCAL_HANDLE.with(|cell| {
        cell.set(handle)
            .expect("Supervisor handle already initialized in this thread!");
    });
}

pub fn get_local_supervisor_handle() -> SupervisorLocalHandle {
    SUPERVISORLOCAL_HANDLE.with(|cell| {
        cell.get()
            .expect("Supervisor handle not initialized in this thread!")
            .clone()
    })
}

pub struct SupervisorComponents;

impl Components for SupervisorComponents {
    type StateEnum = SupervisorStateEnum;
    type MessageSet = SupervisorMessageSet;
    type ExtendedState = SupervisorExtendedState;
    type ActorConfig = SupervisorConfig;
}

#[derive(Debug)]
pub enum SupervisorMessageSet {
    StandardMessage(Message<StandardPayload>),
    SupervisorMessage(Message<SupervisorPayload>),
}
impl MessageSet for SupervisorMessageSet {}

#[derive(Clone, PartialEq, Debug)]
pub enum SupervisorStateEnum {
    Uninit(Uninit),
    Running(Running),
    Error(Error),
}
impl Default for SupervisorStateEnum {
    fn default() -> Self {
        SupervisorStateEnum::Uninit(Uninit)
    }
}
impl StateEnum for SupervisorStateEnum {}

mod state {
    #[derive(Clone, PartialEq, Debug)]
    pub struct Uninit;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Running;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Error;
}

use state::*; // Import the state structs for internal use

type RootSpawnFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;
pub struct SupervisorExtendedState {
    pub actors: HashMap<u16, StandardMessageHandle>,
    pub root_spawn_fn: Option<RootSpawnFn>,
    pub next_id: u16,
}
impl ExtendedState for SupervisorExtendedState {
    fn new() -> Self {
        panic!("SupervisorExtendedState::new() should not be called");
        SupervisorExtendedState {
            actors: HashMap::new(),
            root_spawn_fn: None,
            next_id: 2,
        }
    }
}

impl fmt::Debug for SupervisorExtendedState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SupervisorExtendedState")
    }
}

impl SupervisorExtendedState {
    //TODO: We don't need a special new function with Dylans latest changes
    pub fn special_new(
        root_handle: StandardMessageHandle,
        root_spawn_fn: RootSpawnFn,
        supervisor_handle: SupervisorHandle,
    ) -> SupervisorExtendedState {

        if supervisor_handle.dest_id() != 0 {
            panic!("Supervisor handle must have id 0");
        }

        if root_handle.dest_id() != 1 {
            panic!("Root handle must have id 1");
        }

        init_supervisor_handle(supervisor_handle);

        Self {
            actors: HashMap::from([(1, root_handle)]),
            // Put that FnOnce in an Option
            root_spawn_fn: Some(Box::new(root_spawn_fn)),
            next_id: 2,
        }
    }

    pub fn special_new_local(
        root_handle: StandardMessageHandle,
        root_spawn_fn: RootSpawnFn,
        supervisor_handle: SupervisorLocalHandle,
    ) -> SupervisorExtendedState {

        if supervisor_handle.dest_id() != 0 {
            panic!("Supervisor handle must have id 0");
        }

        if root_handle.dest_id() != 1 {
            panic!("Root handle must have id 1");
        }

        init_local_supervisor_handle(supervisor_handle);

        Self {
            actors: HashMap::from([(1, root_handle)]),
            // Put that FnOnce in an Option
            root_spawn_fn: Some(Box::new(root_spawn_fn)),
            next_id: 2,
        }
    }
}

impl State<SupervisorComponents> for Uninit {
    fn handle_message(
        &self,
        message: SupervisorMessageSet,
        _data: &mut SupervisorExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<SupervisorStateEnum>>,
        Option<SupervisorMessageSet>,
    ) {
        trace!("Uninit handle message");
        let (transition, message_option) = match message {
            SupervisorMessageSet::StandardMessage(message) => {
                match message.payload {
                    StandardPayload::Initialize => {
                        self.on_exit(_data); //TODO: Bug- Shoud not have to run this manually on transition
                        (
                            Some(Transition::To(SupervisorStateEnum::Running(Running))),
                            None,
                        )
                    }
                    _ => (None, None),
                }
            }
            _ => (None, None),
        };
        (transition, message_option)
    }
    fn on_entry(&self, _data: &mut SupervisorExtendedState) {
        info!("Uninit on_entry");
        info!("This is the Actor Shutdown");
    }
    fn on_exit(&self, data: &mut SupervisorExtendedState) {
        info!("Uninit on_exit");
        info!("This is the Actor Initialization");

        if let Some(spawn_fn) = data.root_spawn_fn.take() {
            trace!("Running root spawn function");
            let future = spawn_fn();
            self.spawn_root_actor(future);
            if let Some(actor_handle) = data.actors.get(&1) {
                trace!("Sending Initialize message to root actor");
                let _ =actor_handle.try_send(Message::new(0, StandardPayload::Initialize));
            } else {
                panic!("Actor with id 1 not found");
            }
        }
        else {
            panic!("Root spawn function not found");
        }
    }
    fn parent(&self) -> SupervisorStateEnum {
        SupervisorStateEnum::Uninit(Uninit)
    }
}

impl State<SupervisorComponents> for Running {
    fn handle_message(
        &self,
        message: SupervisorMessageSet,
        _data: &mut SupervisorExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<SupervisorStateEnum>>,
        Option<SupervisorMessageSet>,
    ) {
        trace!("[Running] handle_message: {:?}", message);
        let (transition, message_option) = match message {
            SupervisorMessageSet::SupervisorMessage(message) => match message.payload {
                SupervisorPayload::Spawn(spawn_fn) => {
                    self.spawn_actor(spawn_fn());
                    (None, None)
                }
                _ => (None, None),
            },
            _ => (None, None),
        };
        (transition, message_option)
    }

    fn parent(&self) -> SupervisorStateEnum {
        SupervisorStateEnum::Uninit(Uninit)
    }
}

impl State<SupervisorComponents> for Error {
    fn handle_message(
        &self,
        message: SupervisorMessageSet,
        _data: &mut SupervisorExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<SupervisorStateEnum>>,
        Option<SupervisorMessageSet>,
    ) {
        trace!("[Error] handle_message: {:?}", message);
        (None, None)
    }

    fn parent(&self) -> SupervisorStateEnum {
        SupervisorStateEnum::Uninit(Uninit)
    }
}

//TODO: Generate this from a macro

impl State<SupervisorComponents> for SupervisorStateEnum {
    fn on_entry(&self, data: &mut SupervisorExtendedState) {
        match self {
            SupervisorStateEnum::Uninit(s) => s.on_entry(data),
            SupervisorStateEnum::Running(s) => s.on_entry(data),
            SupervisorStateEnum::Error(s) => s.on_entry(data),
        }
    }

    fn on_exit(&self, data: &mut SupervisorExtendedState) {
        match self {
            SupervisorStateEnum::Uninit(s) => s.on_exit(data),
            SupervisorStateEnum::Running(s) => s.on_exit(data),
            SupervisorStateEnum::Error(s) => s.on_exit(data),
        }
    }

    fn handle_message(
        &self,
        message: SupervisorMessageSet,
        data: &mut SupervisorExtendedState,
        _self_id: &u16,
    ) -> (
        Option<Transition<SupervisorStateEnum>>,
        Option<SupervisorMessageSet>,
    ) {
        match self {
            SupervisorStateEnum::Uninit(s) => s.handle_message(message, data, _self_id),
            SupervisorStateEnum::Running(s) => s.handle_message(message, data, _self_id),
            SupervisorStateEnum::Error(s) => s.handle_message(message, data, _self_id),
        }
    }

    fn parent(&self) -> SupervisorStateEnum {
        match self {
            SupervisorStateEnum::Uninit(s) => s.parent(),
            SupervisorStateEnum::Running(s) => s.parent(),
            SupervisorStateEnum::Error(s) => s.parent(),
        }
    }
}


/* impl RunnableLocal<CounterComponents> for Actor<CounterComponents> {
    fn run_actor(self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + 'static>> {
        Box::pin(async move {
            let mut this = self;
            trace!("Run actor started. Listening for messages...");

            loop {
                select! {
                    Some(message) = this.config.standard_receiver.recv() => {
                        trace!("Actor received STANDARD message: {:?}", message);
                        let current_state = this.state_machine.current_state.clone();
                        this.state_machine.dispatch(&CounterMessageSet::StandardMessage(message), &current_state, &this.handle.source_id());
                    },
                    Some(message) = this.config.counter_receiver.recv() => {
                        trace!("Actor received COUNTER message: {:?}", message);
                        let current_state = this.state_machine.current_state.clone();
                        this.state_machine.dispatch(&CounterMessageSet::CounterMessage(message), &current_state, &this.handle.source_id());
                    },
                    else => {
                        // If all channels closed, break out
                        trace!("All channels closed. Stopping actor run loop.");
                        break;
                    }
                }
            }
        })
    }
} */