use bloxide::ExtendedState;

#[derive(Debug, Default)]
pub struct RootExtState {
    count: usize
}

impl ExtendedState for RootExtState {
    fn new() -> Self {
        Self::default()
    }
}
