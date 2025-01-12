//Copyright 2025 Bloxide, all rights reserved

#[cfg(feature = "runtime-embassy")]
pub mod runtime {
    use crate::core::messaging::*;
    use crate::std_exports::*;
    use core::cell::RefCell;
    use embassy_sync::blocking_mutex::raw::RawMutex;
    use embassy_sync::channel::Channel;
    use log::{error, info};

    pub const STANDARD_MESSAGE_CHANNEL_SIZE: usize = 32;
    pub type DefaultChannelMutex = embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    pub type StandardMessageMutex = DefaultChannelMutex;
    pub type StandardMessageHandle =
        EmbassyHandle<StandardMessageMutex, StandardMessage, STANDARD_MESSAGE_CHANNEL_SIZE>;
    pub type StandardMessagePool =
        HandlePool<StandardMessageMutex, StandardMessage, STANDARD_MESSAGE_CHANNEL_SIZE>;

    pub struct HandlePool<Mutex: RawMutex + 'static, Msg: ActorMessage, const Q: usize> {
        pool: RefCell<Vec<(EmbassyHandle<Mutex, Msg, Q>, bool)>>,
    }

    impl<Mutex, Msg, const Q: usize> HandlePool<Mutex, Msg, Q>
    where
        Mutex: RawMutex + 'static,
        Msg: ActorMessage,
    {
        pub fn new(capacity: usize) -> Self {
            let mut pool = Vec::with_capacity(capacity);
            for _ in 0..capacity {
                let channel: &'static _ = Box::leak(Box::new(Channel::new()));
                let handle = EmbassyHandle::new(channel);
                pool.push((handle, false));
            }
            HandlePool {
                pool: RefCell::new(pool),
            }
        }

        pub fn acquire(&self) -> Option<EmbassyHandle<Mutex, Msg, Q>> {
            let mut pool = self.pool.borrow_mut();
            for (handle, in_use) in pool.iter_mut() {
                if !*in_use {
                    *in_use = true;
                    return Some(handle.clone());
                }
            }
            None
        }

        pub fn release(&self, handle: &EmbassyHandle<Mutex, Msg, Q>) {
            let mut pool = self.pool.borrow_mut();
            for (h, in_use) in pool.iter_mut() {
                if core::ptr::eq(h.channel, handle.channel) {
                    *in_use = false;
                    return;
                }
            }
        }
    }

    pub struct EmbassyHandle<Mutex: RawMutex + 'static, Msg: ActorMessage, const Q: usize> {
        pub channel: &'static Channel<Mutex, Msg, Q>,
    }

    impl<Mutex, Msg, const N: usize> Debug for EmbassyHandle<Mutex, Msg, N>
    where
        Mutex: RawMutex + 'static,
        Msg: ActorMessage,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("EmbassyHandle")
                .field("sender", &"<embassy channel>")
                .finish()
        }
    }

    impl<Mutex, Msg, const N: usize> EmbassyHandle<Mutex, Msg, N>
    where
        Mutex: RawMutex + 'static,
        Msg: ActorMessage,
    {
        pub fn new(channel: &'static Channel<Mutex, Msg, N>) -> Self {
            Self { channel }
        }
    }

    impl<Mutex, Msg, const N: usize> ActorHandle<Msg> for EmbassyHandle<Mutex, Msg, N>
    where
        Mutex: RawMutex + 'static + Send + Sync,
        Msg: ActorMessage,
    {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn box_clone(&self) -> Box<dyn ActorHandle<Msg>> {
            Box::new(self.clone())
        }

        fn send(&self, msg: Msg) {
            info!("Sending message to embassy channel: {:?}", msg);
            if let Err(e) = self.channel.sender().try_send(msg) {
                error!("Failed to send message: {:?}", e);
            }
        }
    }

    impl<Mutex, Msg, const N: usize> Clone for EmbassyHandle<Mutex, Msg, N>
    where
        Mutex: RawMutex + 'static,
        Msg: ActorMessage,
    {
        fn clone(&self) -> Self {
            Self {
                channel: self.channel,
            }
        }
    }
}

#[cfg(feature = "runtime-embassy")]
pub use runtime::*;
