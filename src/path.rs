//! Path manipulation module.
//!
//! Provides functions for path manipulation operations.

use std::path::{Path, PathBuf};

use fusabi_host::ExecutionContext;
use fusabi_host::Value;

/// Join path components.
pub fn join(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    if args.is_empty() {
        return Err(fusabi_host::Error::host_function("path.join: no arguments"));
    }

    let mut result = PathBuf::new();

    for arg in args {
        let part = arg
            .as_str()
            .ok_or_else(|| fusabi_host::Error::host_function("path.join: argument must be string"))?;
        result.push(part);
    }

    Ok(Value::String(result.to_string_lossy().into_owned()))
}

/// Get the directory name of a path.
pub fn dirname(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("path.dirname: missing path argument"))?;

    let path = Path::new(path_str);

    match path.parent() {
        Some(parent) => Ok(Value::String(parent.to_string_lossy().into_owned())),
        None => Ok(Value::Null),
    }
}

/// Get the base name of a path.
pub fn basename(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("path.basename: missing path argument"))?;

    let path = Path::new(path_str);

    match path.file_name() {
        Some(name) => Ok(Value::String(name.to_string_lossy().into_owned())),
        None => Ok(Value::Null),
    }
}

/// Get the file extension.
pub fn extension(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("path.extension: missing path argument"))?;

    let path = Path::new(path_str);

    match path.extension() {
        Some(ext) => Ok(Value::String(ext.to_string_lossy().into_owned())),
        None => Ok(Value::Null),
    }
}

/// Normalize a path.
pub fn normalize(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("path.normalize: missing path argument"))?;

    // Simple normalization - in real implementation would handle . and ..
    let path = Path::new(path_str);
    let normalized = path.components().collect::<PathBuf>();

    Ok(Value::String(normalized.to_string_lossy().into_owned()))
}

/// Check if a path is absolute.
pub fn is_absolute(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let path_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("path.is_absolute: missing path argument"))?;

    let path = Path::new(path_str);
    Ok(Value::Bool(path.is_absolute()))
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
    fn test_join() {
        let ctx = create_test_ctx();
        let result = join(&[
            Value::String("/home".into()),
            Value::String("user".into()),
            Value::String("file.txt".into()),
        ], &ctx).unwrap();

        let path = result.as_str().unwrap();
        assert!(path.contains("home"));
        assert!(path.contains("user"));
        assert!(path.contains("file.txt"));
    }

    #[test]
    fn test_dirname() {
        let ctx = create_test_ctx();
        let result = dirname(&[Value::String("/home/user/file.txt".into())], &ctx).unwrap();
        assert_eq!(result.as_str().unwrap(), "/home/user");
    }

    #[test]
    fn test_basename() {
        let ctx = create_test_ctx();
        let result = basename(&[Value::String("/home/user/file.txt".into())], &ctx).unwrap();
        assert_eq!(result.as_str().unwrap(), "file.txt");
    }

    #[test]
    fn test_extension() {
        let ctx = create_test_ctx();
        let result = extension(&[Value::String("/home/user/file.txt".into())], &ctx).unwrap();
        assert_eq!(result.as_str().unwrap(), "txt");
    }

    #[test]
    fn test_is_absolute() {
        let ctx = create_test_ctx();

        let result = is_absolute(&[Value::String("/absolute/path".into())], &ctx).unwrap();
        assert_eq!(result.as_bool().unwrap(), true);

        let result = is_absolute(&[Value::String("relative/path".into())], &ctx).unwrap();
        assert_eq!(result.as_bool().unwrap(), false);
    }
}
