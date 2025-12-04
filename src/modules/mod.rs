//! Core stdlib modules.
//!
//! This module re-exports the individual stdlib modules for convenience.

#[cfg(feature = "process")]
pub mod process;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "path")]
pub mod path;

#[cfg(feature = "env")]
pub mod env;

#[cfg(feature = "format")]
pub mod format;

#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "metrics")]
pub mod metrics;
