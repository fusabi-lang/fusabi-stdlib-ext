//! Filesystem module.
//!
//! Provides functions for filesystem operations with safety controls.

use std::path::Path;
use std::sync::Arc;

use fusabi_host::ExecutionContext;
use fusabi_host::Value;

use crate::safety::SafetyConfig;

/// Read a file's contents.
pub fn read_file(
    safety: &Arc<SafetyConfig>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("fs.read: missing path argument"))?;

    let path = Path::new(path_str);

    // Check safety
    safety
        .paths
        .check_read(path)
        .map_err(|e| fusabi_host::Error::host_function(e.to_string()))?;

    // Read file
    let content = std::fs::read_to_string(path)
        .map_err(|e| fusabi_host::Error::host_function(format!("fs.read: {}", e)))?;

    Ok(Value::String(content))
}

/// Write content to a file.
pub fn write_file(
    safety: &Arc<SafetyConfig>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("fs.write: missing path argument"))?;

    let content = args
        .get(1)
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("fs.write: missing content argument"))?;

    let path = Path::new(path_str);

    // Check safety
    safety
        .paths
        .check_write(path)
        .map_err(|e| fusabi_host::Error::host_function(e.to_string()))?;

    // Write file
    std::fs::write(path, content)
        .map_err(|e| fusabi_host::Error::host_function(format!("fs.write: {}", e)))?;

    Ok(Value::Null)
}

/// Check if a path exists.
pub fn exists(
    safety: &Arc<SafetyConfig>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("fs.exists: missing path argument"))?;

    let path = Path::new(path_str);

    // Check safety (need read permission to check existence)
    safety
        .paths
        .check_read(path)
        .map_err(|e| fusabi_host::Error::host_function(e.to_string()))?;

    Ok(Value::Bool(path.exists()))
}

/// List directory contents.
pub fn list_dir(
    safety: &Arc<SafetyConfig>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("fs.list: missing path argument"))?;

    let path = Path::new(path_str);

    // Check safety
    safety
        .paths
        .check_read(path)
        .map_err(|e| fusabi_host::Error::host_function(e.to_string()))?;

    // List directory
    let entries: Vec<Value> = std::fs::read_dir(path)
        .map_err(|e| fusabi_host::Error::host_function(format!("fs.list: {}", e)))?
        .filter_map(|entry| entry.ok())
        .map(|entry| Value::String(entry.file_name().to_string_lossy().into_owned()))
        .collect();

    Ok(Value::List(entries))
}

/// Create a directory.
pub fn mkdir(
    safety: &Arc<SafetyConfig>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("fs.mkdir: missing path argument"))?;

    let path = Path::new(path_str);

    // Check safety
    safety
        .paths
        .check_write(path)
        .map_err(|e| fusabi_host::Error::host_function(e.to_string()))?;

    // Create directory
    std::fs::create_dir_all(path)
        .map_err(|e| fusabi_host::Error::host_function(format!("fs.mkdir: {}", e)))?;

    Ok(Value::Null)
}

/// Remove a file or directory.
pub fn remove(
    safety: &Arc<SafetyConfig>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("fs.remove: missing path argument"))?;

    let path = Path::new(path_str);

    // Check safety
    safety
        .paths
        .check_write(path)
        .map_err(|e| fusabi_host::Error::host_function(e.to_string()))?;

    // Remove
    if path.is_dir() {
        std::fs::remove_dir_all(path)
            .map_err(|e| fusabi_host::Error::host_function(format!("fs.remove: {}", e)))?;
    } else {
        std::fs::remove_file(path)
            .map_err(|e| fusabi_host::Error::host_function(format!("fs.remove: {}", e)))?;
    }

    Ok(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::safety::PathAllowlist;
    use fusabi_host::Capabilities;
    use fusabi_host::Limits;
    use fusabi_host::{Sandbox, SandboxConfig};

    fn create_test_ctx() -> ExecutionContext {
        let sandbox = Sandbox::new(SandboxConfig::default()).unwrap();
        ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox)
    }

    #[test]
    fn test_read_safety_check() {
        let safety = Arc::new(SafetyConfig::strict());
        let ctx = create_test_ctx();

        let result = read_file(&safety, &[Value::String("/etc/passwd".into())], &ctx);
        assert!(result.is_err()); // Should fail - path not allowed
    }

    #[test]
    fn test_exists_with_permission() {
        let safety =
            Arc::new(SafetyConfig::new().with_paths(PathAllowlist::none().allow_read("/tmp")));
        let ctx = create_test_ctx();

        let result = exists(&safety, &[Value::String("/tmp".into())], &ctx);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Bool(true));
    }
}
