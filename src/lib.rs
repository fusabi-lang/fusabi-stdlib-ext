//! # fusabi-stdlib-ext
//!
//! Extended standard library modules and domain packs for Fusabi.
//!
//! This crate provides optional stdlib modules that extend Fusabi's capabilities:
//!
//! ## Core Modules
//!
//! - **Process** - Execute system processes with timeout and environment control
//! - **Fs** - Filesystem operations (read, write, list, glob)
//! - **Path** - Path manipulation (join, normalize, resolve)
//! - **Env** - Environment variable access
//! - **Format** - String formatting (sprintf, templating)
//! - **Net** - Network operations (HTTP requests)
//! - **Time** - Time and duration utilities
//! - **Metrics** - Counter, gauge, histogram primitives
//!
//! ## Extended Modules
//!
//! - **Terminal** - Terminal I/O, key events, clipboard, colors
//! - **GPU** - GPU metrics via NVML (utilization, memory, temperature)
//! - **FsStream** - File streaming with backpressure (tail, chunked reads)
//! - **NetHttp** - Enhanced HTTP client (retries, streaming, custom options)
//!
//! ## Domain Packs
//!
//! - **terminal-ui** - Ratatui/TUI widgets and helpers
//! - **observability** - Logging, tracing, metrics integration
//! - **k8s** - Kubernetes API bindings
//! - **mcp** - MCP (Model Context Protocol) helpers
//!
//! ## Safety
//!
//! All modules follow a default-deny security model:
//! - Filesystem access requires explicit path allowlists
//! - Network access requires explicit host allowlists
//! - Process execution requires explicit permission
//! - All operations respect configured timeouts
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use fusabi_stdlib_ext::{StdlibRegistry, StdlibConfig};
//!
//! // Create registry with default modules
//! let config = StdlibConfig::default();
//! let registry = StdlibRegistry::new(config)?;
//!
//! // Register modules with an engine
//! registry.register_all(&mut engine)?;
//! ```

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod config;
mod error;
mod registry;
mod safety;

// Core modules
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

// Extended modules (vNEXT)
#[cfg(feature = "terminal")]
pub mod terminal;

#[cfg(feature = "gpu")]
pub mod gpu;

#[cfg(feature = "fs_stream")]
pub mod fs_stream;

#[cfg(feature = "net_http")]
pub mod net_http;

// Domain packs
#[cfg(feature = "terminal-ui")]
pub mod terminal_ui;

#[cfg(feature = "observability")]
pub mod observability;

#[cfg(feature = "k8s")]
pub mod k8s;

#[cfg(feature = "mcp")]
pub mod mcp;

pub use config::{StdlibConfig, ModuleConfig};
pub use error::{Error, Result};
pub use registry::StdlibRegistry;
pub use safety::{SafetyConfig, PathAllowlist, HostAllowlist};

/// Crate version for compatibility checks.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
