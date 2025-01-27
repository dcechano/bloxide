//Copyright 2025 Bloxide, all rights reserved

#[cfg(feature = "runtime-tokio")]
pub mod runtime {
    use crate::core::messaging::*;
    use crate::std_exports::*;
    pub use tokio::pin;
    pub use tokio::runtime::Builder;
    pub use tokio::select;
    pub use tokio::sync::mpsc;
    pub use tokio::task::*;
    pub use tokio::time::{sleep, Duration};

    pub const DEFAULT_CHANNEL_SIZE: usize = 32;

    pub const STANDARD_MESSAGE_CHANNEL_SIZE: usize = DEFAULT_CHANNEL_SIZE;

    pub type TokioHandle<M> = Handle<TokioSender<M>>;
    pub type TokioReceiver<M> = mpsc::Receiver<Message<M>>;
    pub type TokioSender<M> = mpsc::Sender<Message<M>>;

    /// Creates a new message channel with the default channel size
    pub fn create_channel<M>(id: u16) -> (TokioHandle<M>, TokioReceiver<M>) {
        create_channel_with_size::<M>(id, DEFAULT_CHANNEL_SIZE)
    }

    /// Creates a new message channel with the specified channel size
    pub fn create_channel_with_size<M>(id: u16, size: usize) -> (TokioHandle<M>, TokioReceiver<M>) {
        let (sender, receiver) = mpsc::channel::<Message<M>>(size);
        (Handle::new(id, sender), receiver)
    }

    impl<M> MessageSender for TokioHandle<M> {
        type PayloadType = M;

        fn try_send(&self, msg: Message<M>) -> Result<(), TrySendError<Message<M>>> {
            self.sender.try_send(msg)
        }
    }

    pub type StandardMessageHandle = TokioHandle<StandardPayload>;
    pub type StandardSender = TokioSender<StandardPayload>;
    pub type StandardReceiver = TokioReceiver<StandardPayload>;
}
// Re-export everything from the internal module
#[cfg(feature = "runtime-tokio")]
pub use runtime::*;
