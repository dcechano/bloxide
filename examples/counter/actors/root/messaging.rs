use bloxide::{Message, MessageSet, StandardPayload};

use crate::CounterPayload;

#[derive(Debug)]
pub enum RootMessage {
    StandardMessage(Message<StandardPayload>),
    CounterMessage(Message<CounterPayload>),
}

impl MessageSet for RootMessage {}
