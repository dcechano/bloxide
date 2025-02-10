// Copyright 2025 Bloxide, all rights reserved

#![cfg_attr(feature = "runtime-embassy", no_std)]

pub mod blox;
pub mod components;
pub mod macros;
pub mod merge;
pub mod messaging;
pub mod state_machine;
// Core re-exports
pub use crate::{components::*, messaging::*};

pub use blox::supervisor::*;

// Conditional type exports

pub mod std_exports {
    pub use crate::common_exports::*;
    //pub use tokio::sync::mpsc::error::TrySendError;
}

#[cfg(feature = "runtime-embassy")]
pub mod std_exports {
    pub use crate::common_exports::*;
    pub use embassy_sync::channel::TrySendError;
    pub use embassy_sync::once_lock::OnceLock;
}

pub mod common_exports {
    extern crate alloc;
    pub use alloc::boxed::Box;
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
    pub use core::any::Any;
    pub use core::cell::LazyCell;
    pub use core::cell::OnceCell;
    pub use core::fmt;
    pub use core::future::Future;
    pub use core::hash::Hash;
    pub use core::hash::Hasher;
    pub use core::marker::PhantomData;
    pub use core::pin::Pin;
    pub use hashbrown::HashMap;
}

// Re-export everything from std_exports at crate root
pub use std_exports::*;
