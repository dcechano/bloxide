// Copyright 2025 Bloxide, all rights reserved

#![cfg_attr(feature = "runtime-embassy", no_std)]

pub mod core;
pub mod macros;
pub mod runtime;
pub mod supervisor;

// Core re-exports
pub use crate::{core::components::*, core::messaging::*};

#[cfg(feature = "runtime-tokio")]
pub use supervisor::*;

// Conditional type exports
#[cfg(feature = "runtime-tokio")]
pub mod std_exports {
    pub use std::any::Any;
    pub use std::boxed::Box;
    pub use std::cell::OnceCell;
    pub use std::fmt::{Debug, Formatter};
    pub use std::future::Future;
    pub use std::marker::PhantomData;
    pub use std::pin::Pin;
    pub use std::string::String;
    pub use std::sync::Arc;
    pub use std::vec::Vec;
    pub use tokio::sync::mpsc::error::TrySendError;

    pub use hashbrown::HashMap;
    pub use hashbrown::HashSet;
    pub use std::cell::LazyCell;
    pub use std::cell::RefCell;
    pub use std::fmt;
    pub use std::hash::Hash;
    pub use std::hash::Hasher;
    pub use std::rc::Rc;
    pub use std::sync::LazyLock;
    pub use std::sync::OnceLock;
}

#[cfg(feature = "runtime-embassy")]
pub mod std_exports {
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
    pub use embassy_sync::channel::TrySendError;
    pub use embassy_sync::once_lock::OnceLock;
    pub use hashbrown::HashMap;
}

// Re-export everything from std_exports at crate root
pub use std_exports::*;

#[cfg(all(not(feature = "runtime-tokio"), not(feature = "runtime-embassy")))]
compile_error!("Either 'runtime-tokio' or 'runtime-embassy' feature must be enabled");

#[cfg(all(feature = "runtime-tokio", feature = "runtime-embassy"))]
compile_error!("Only one runtime can be enabled at a time");
