//! Environment module.
//!
//! Provides functions for environment variable access with safety controls.

use std::sync::Arc;

use fusabi_host::ExecutionContext;
use fusabi_host::Value;

use crate::safety::SafetyConfig;

/// Get an environment variable.
pub fn get(
    safety: &Arc<SafetyConfig>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let name = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("env.get: missing name argument"))?;

    // Check safety
    safety.check_env(name).map_err(|e| {
        fusabi_host::Error::host_function(e.to_string())
    })?;

    match std::env::var(name) {
        Ok(value) => Ok(Value::String(value)),
        Err(_) => Ok(Value::Null),
    }
}

/// Set an environment variable.
pub fn set(
    safety: &Arc<SafetyConfig>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let name = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("env.set: missing name argument"))?;

    let value = args
        .get(1)
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("env.set: missing value argument"))?;

    // Check safety
    safety.check_env(name).map_err(|e| {
        fusabi_host::Error::host_function(e.to_string())
    })?;

    std::env::set_var(name, value);
    Ok(Value::Null)
}

/// Get the current working directory.
pub fn cwd(
    _args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    match std::env::current_dir() {
        Ok(path) => Ok(Value::String(path.to_string_lossy().into_owned())),
        Err(e) => Err(fusabi_host::Error::host_function(format!("env.cwd: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_host::Capabilities;
    use fusabi_host::{Sandbox, SandboxConfig};
    use fusabi_host::Limits;

    fn create_test_ctx() -> ExecutionContext {
        let sandbox = Sandbox::new(SandboxConfig::default()).unwrap();
        ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox)
    }

    #[test]
    fn test_get_safety_check() {
        let safety = Arc::new(SafetyConfig::strict());
        let ctx = create_test_ctx();

        let result = get(&safety, &[Value::String("PATH".into())], &ctx);
        assert!(result.is_err()); // Should fail - env not allowed
    }

    #[test]
    fn test_get_with_permission() {
        let safety = Arc::new(SafetyConfig::new().with_env_vars(["PATH"]));
        let ctx = create_test_ctx();

        let result = get(&safety, &[Value::String("PATH".into())], &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cwd() {
        let ctx = create_test_ctx();
        let result = cwd(&[], &ctx);
        assert!(result.is_ok());
        assert!(result.unwrap().as_str().is_some());
    }
}
