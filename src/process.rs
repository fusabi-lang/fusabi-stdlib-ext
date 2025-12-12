//! Process execution module.
//!
//! Provides functions for executing system processes with safety controls.

use std::sync::Arc;
use std::time::Duration;

use fusabi_host::ExecutionContext;
use fusabi_host::Value;

use crate::error::{Error, Result};
use crate::safety::SafetyConfig;

/// Execute a command and wait for completion.
pub fn exec(
    safety: &Arc<SafetyConfig>,
    timeout: Option<Duration>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let command = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("exec: missing command argument"))?;

    // Check safety
    safety
        .check_execute(command)
        .map_err(|e| fusabi_host::Error::host_function(e.to_string()))?;

    // Get arguments
    let cmd_args: Vec<String> = args
        .iter()
        .skip(1)
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    // Apply timeout
    let timeout = timeout
        .map(|t| safety.clamp_timeout(t))
        .unwrap_or(safety.default_timeout);

    // Execute command (simulated)
    tracing::info!(
        "Executing: {} {:?} (timeout: {:?})",
        command,
        cmd_args,
        timeout
    );

    // In real implementation, would use tokio::process::Command
    let output = format!("Executed: {} {}", command, cmd_args.join(" "));

    Ok(Value::Map({
        let mut m = std::collections::HashMap::new();
        m.insert("stdout".into(), Value::String(output));
        m.insert("stderr".into(), Value::String(String::new()));
        m.insert("exit_code".into(), Value::Int(0));
        m
    }))
}

/// Spawn a command without waiting.
pub fn spawn(args: &[Value], _ctx: &ExecutionContext) -> fusabi_host::Result<Value> {
    let command = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("spawn: missing command argument"))?;

    // In real implementation, would spawn the process and return a handle
    tracing::info!("Spawning: {}", command);

    Ok(Value::Map({
        let mut m = std::collections::HashMap::new();
        m.insert("pid".into(), Value::Int(12345));
        m.insert("command".into(), Value::String(command.to_string()));
        m
    }))
}

/// Options for process execution.
#[derive(Debug, Clone)]
pub struct ExecOptions {
    /// Working directory.
    pub cwd: Option<String>,
    /// Environment variables.
    pub env: std::collections::HashMap<String, String>,
    /// Timeout.
    pub timeout: Option<Duration>,
    /// Capture stdout.
    pub capture_stdout: bool,
    /// Capture stderr.
    pub capture_stderr: bool,
}

impl Default for ExecOptions {
    fn default() -> Self {
        Self {
            cwd: None,
            env: std::collections::HashMap::new(),
            timeout: Some(Duration::from_secs(30)),
            capture_stdout: true,
            capture_stderr: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_host::Capabilities;
    use fusabi_host::Limits;
    use fusabi_host::{Sandbox, SandboxConfig};

    fn create_test_ctx() -> ExecutionContext {
        let sandbox = Sandbox::new(SandboxConfig::default()).unwrap();
        ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox)
    }

    #[test]
    fn test_exec_safety_check() {
        let safety = Arc::new(SafetyConfig::strict());
        let ctx = create_test_ctx();

        let result = exec(&safety, None, &[Value::String("ls".into())], &ctx);
        assert!(result.is_err()); // Should fail - process not allowed
    }

    #[test]
    fn test_exec_with_permission() {
        let safety = Arc::new(
            SafetyConfig::new()
                .with_allow_process(true)
                .with_allowed_commands(["ls"]),
        );
        let ctx = create_test_ctx();

        let result = exec(&safety, None, &[Value::String("ls".into())], &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_exec_command_not_allowed() {
        let safety = Arc::new(
            SafetyConfig::new()
                .with_allow_process(true)
                .with_allowed_commands(["ls"]),
        );
        let ctx = create_test_ctx();

        let result = exec(&safety, None, &[Value::String("rm".into())], &ctx);
        assert!(result.is_err()); // rm not in allowed list
    }
}
