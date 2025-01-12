//Copyright 2025 Bloxide, all rights reserved

#[cfg(feature = "runtime-tokio")]
pub mod runtime {
    use crate::core::{actor::*, messaging::*};
    use crate::std_exports::*;
    use tokio::sync::mpsc;
    pub const DEFAULT_TOKIO_CHANNEL_SIZE: usize = 32;
    pub const STANDARD_MESSAGE_CHANNEL_SIZE: usize = DEFAULT_TOKIO_CHANNEL_SIZE;

    #[derive(Debug, Clone)]
    pub struct TokioHandle<M: ActorMessage, const Q: usize> {
        sender: mpsc::Sender<M>,
    }

    impl<M: ActorMessage, const Q: usize> TokioHandle<M, Q> {
        pub fn new(sender: mpsc::Sender<M>) -> Self {
            Self { sender }
        }
    }

    impl<M: ActorMessage, const Q: usize> ActorHandle<M> for TokioHandle<M, Q> {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn box_clone(&self) -> Box<dyn ActorHandle<M>> {
            Box::new(TokioHandle::<M, Q> {
                sender: self.sender.clone(),
            })
        }

        fn send(&self, msg: M) {
            let _ = self.sender.try_send(msg);
        }
    }

    pub type StandardMessageHandle = TokioHandle<StandardMessage, STANDARD_MESSAGE_CHANNEL_SIZE>;

    #[derive(Debug)]
    pub struct TokioActorSpawner<const Q: usize, A: Actor> {
        phantom: std::marker::PhantomData<A>,
    }

    impl<const Q: usize, A: Actor> Default for TokioActorSpawner<Q, A> {
        fn default() -> Self {
            Self {
                phantom: std::marker::PhantomData,
            }
        }
    }

    impl<const Q: usize, A: Actor> TokioActorSpawner<Q, A> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<const Q: usize, A> ActorSpawner for TokioActorSpawner<Q, A>
    where
        A: Actor + Debug + 'static,
    {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn spawn(&self, id: u16) -> Result<StandardMessageHandle, ActorError> {
            let (tx, mut rx) = mpsc::channel::<StandardMessage>(STANDARD_MESSAGE_CHANNEL_SIZE);
            let mut actor = A::new(id, StandardMessageHandle::new(tx.clone()));

            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    actor.handle_message(&A::MessageSet::from_standard(msg));
                }
            });

            Ok(StandardMessageHandle::new(tx))
        }
    }
}

// Re-export everything from the internal module
#[cfg(feature = "runtime-tokio")]
pub use runtime::*;
