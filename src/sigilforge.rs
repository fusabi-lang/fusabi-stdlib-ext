//! Sigilforge authentication module for Fusabi.
//!
//! This module provides host functions for accessing credentials through
//! the Sigilforge authentication daemon.
//!
//! # Functions
//!
//! - `sigilforge.get_token(service, account)` - Get an OAuth access token
//! - `sigilforge.resolve(auth_uri)` - Resolve an auth:// URI to its secret value
//! - `sigilforge.is_available()` - Check if the Sigilforge daemon is available

use fusabi_host::{ExecutionContext, Result, Value};
use sigilforge_client::{SigilforgeClient, TokenProvider};
use std::sync::OnceLock;

// Global client instance - created lazily on first use
static CLIENT: OnceLock<SigilforgeClient> = OnceLock::new();

fn get_client() -> &'static SigilforgeClient {
    CLIENT.get_or_init(SigilforgeClient::new)
}

/// Get an OAuth access token for a service/account.
///
/// # Arguments
/// - `args[0]`: Service name (string, e.g., "spotify")
/// - `args[1]`: Account name (string, e.g., "personal")
///
/// # Returns
/// The access token as a string.
///
/// # Example (Fusabi script)
/// ```fsharp
/// let! token = Sigilforge.getToken "spotify" "personal"
/// ```
pub fn get_token(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let service = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
        fusabi_host::Error::host_function("sigilforge.get_token: service must be a string")
    })?;

    let account = args.get(1).and_then(|v| v.as_str()).ok_or_else(|| {
        fusabi_host::Error::host_function("sigilforge.get_token: account must be a string")
    })?;

    // Get the tokio runtime handle
    let rt = tokio::runtime::Handle::try_current()
        .map_err(|_| fusabi_host::Error::runtime("no tokio runtime available"))?;

    let result = rt.block_on(async { get_client().get_token(service, account).await });

    match result {
        Ok(token) => Ok(Value::String(token.token)),
        Err(e) => Err(fusabi_host::Error::runtime(e.to_string())),
    }
}

/// Ensure a valid token, refreshing if needed.
///
/// # Arguments
/// - `args[0]`: Service name (string)
/// - `args[1]`: Account name (string)
///
/// # Returns
/// A fresh access token as a string.
pub fn ensure_token(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let service = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
        fusabi_host::Error::host_function("sigilforge.ensure_token: service must be a string")
    })?;

    let account = args.get(1).and_then(|v| v.as_str()).ok_or_else(|| {
        fusabi_host::Error::host_function("sigilforge.ensure_token: account must be a string")
    })?;

    let rt = tokio::runtime::Handle::try_current()
        .map_err(|_| fusabi_host::Error::runtime("no tokio runtime available"))?;

    let result = rt.block_on(async { get_client().ensure_token(service, account).await });

    match result {
        Ok(token) => Ok(Value::String(token.token)),
        Err(e) => Err(fusabi_host::Error::runtime(e.to_string())),
    }
}

/// Resolve an auth:// URI to its secret value.
///
/// # Arguments
/// - `args[0]`: auth:// URI (string, e.g., "auth://openai/default/api_key")
///
/// # Returns
/// The resolved secret value as a string.
///
/// # Example (Fusabi script)
/// ```fsharp
/// let! apiKey = Sigilforge.resolve "auth://openai/default/api_key"
/// ```
pub fn resolve(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let reference = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
        fusabi_host::Error::host_function("sigilforge.resolve: reference must be a string")
    })?;

    let rt = tokio::runtime::Handle::try_current()
        .map_err(|_| fusabi_host::Error::runtime("no tokio runtime available"))?;

    let result = rt.block_on(async { get_client().resolve(reference).await });

    match result {
        Ok(secret) => Ok(Value::String(secret.value)),
        Err(e) => Err(fusabi_host::Error::runtime(e.to_string())),
    }
}

/// Check if the Sigilforge daemon is available.
///
/// # Returns
/// Boolean indicating if the daemon is reachable.
///
/// # Example (Fusabi script)
/// ```fsharp
/// let! available = Sigilforge.isAvailable ()
/// if available then
///     printfn "Daemon is running"
/// ```
pub fn is_available(_args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let rt = tokio::runtime::Handle::try_current()
        .map_err(|_| fusabi_host::Error::runtime("no tokio runtime available"))?;

    let available = rt.block_on(async { get_client().is_daemon_available().await });

    Ok(Value::Bool(available))
}
