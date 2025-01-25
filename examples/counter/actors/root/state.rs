use crate::CounterPayload;
use bloxide::{Components, State, StateEnum, Transition};

use super::{actor::RootComponents, messaging::RootMessage};

#[derive(Clone, PartialEq, Debug)]
pub enum RootState {
    Uninit(Uninit),
    Counting(Counting),
    Finished(Finished),
    Error(Error),
}

impl State<RootComponents> for RootState {
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
            RootState::Uninit(uninit) => uninit.handle_message(message, data, self_id),
            RootState::Counting(counting) => counting.handle_message(message, data, self_id),
            RootState::Finished(finished) => finished.handle_message(message, data, self_id),
            RootState::Error(error) => error.handle_message(message, data, self_id),
        }
    }
}

impl StateEnum for RootState {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for RootState {
    fn default() -> Self {
        RootState::Uninit(Uninit)
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
        let RootMessage::CounterMessage(msg) = msg else {
            return (None, None);
        };

        match msg.payload {
            CounterPayload::SetCount(_) => todo!(),
            CounterPayload::Increment(_) => todo!(),
            CounterPayload::Decrement(_) => todo!(),
            CounterPayload::SetMax(_) => todo!(),
            CounterPayload::SetMin(_) => todo!(),
            CounterPayload::CountEvent(_) => todo!(),
        }
    }
}

impl State<RootComponents> for Counting {
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
        todo!()
    }
}

mod types {
    #[derive(Clone, PartialEq, Debug)]
    pub struct Uninit;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Counting;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Finished;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Error;
}

use types::*;
