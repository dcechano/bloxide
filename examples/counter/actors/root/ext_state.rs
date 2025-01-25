use crate::actors::counter::*;
use bloxide::ExtendedState;
#[derive(Debug, Default)]
pub struct RootExtState {
    pub self_counter_handle: Option<CounterHandle>, //not actually optional.  Need to implement Dylans Init stuff
    pub counter_handle: Option<CounterHandle>,
}

impl ExtendedState for RootExtState {
    fn new() -> Self {
        Self {
            self_counter_handle: None,
            counter_handle: None,
        }
    }
}

impl RootExtState {
    pub fn special_new(self_counter_handle: CounterHandle) -> Self {
        Self {
            self_counter_handle: Some(self_counter_handle),
            counter_handle: None,
        }
    }
}
