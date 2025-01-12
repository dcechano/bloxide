//Copyright 2025 Bloxide, all rights reserved

// Re-export runtime-specific types
#[cfg(feature = "runtime-embassy")]
pub mod embassy;
#[cfg(feature = "runtime-embassy")]
pub use embassy::*;

#[cfg(feature = "runtime-tokio")]
pub mod tokio;
#[cfg(feature = "runtime-tokio")]
pub use tokio::*;

// Shared types and constants that both runtimes use
pub use self::runtime_core::*;

// Internal module for shared definitions
#[cfg(any(feature = "runtime-embassy", feature = "runtime-tokio"))]
mod runtime_core {
    #[cfg(feature = "runtime-embassy")]
    pub const DEFAULT_CHANNEL_SIZE: usize = 8;
    #[cfg(feature = "runtime-tokio")]
    pub const DEFAULT_CHANNEL_SIZE: usize = 32;

    // Add other shared types here
}
