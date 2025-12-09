//! Error types for stdlib-ext operations.

use thiserror::Error;

/// Result type alias using [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during stdlib operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Operation not permitted by safety config.
    #[error("operation not permitted: {0}")]
    NotPermitted(String),

    /// Path not in allowlist.
    #[error("path not allowed: {0}")]
    PathNotAllowed(String),

    /// Host not in allowlist.
    #[error("host not allowed: {0}")]
    HostNotAllowed(String),

    /// Operation timed out.
    #[error("operation timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// Process execution failed.
    #[error("process error: {0}")]
    Process(String),

    /// Process exit with non-zero code.
    #[error("process exited with code {code}: {message}")]
    ProcessExit {
        /// Exit code.
        code: i32,
        /// Error message.
        message: String,
    },

    /// Filesystem error.
    #[error("filesystem error: {0}")]
    Filesystem(String),

    /// Network error.
    #[error("network error: {0}")]
    Network(String),

    /// Format error.
    #[error("format error: {0}")]
    Format(String),

    /// Environment error.
    #[error("environment error: {0}")]
    Environment(String),

    /// Module not available.
    #[error("module not available: {0}")]
    ModuleNotAvailable(String),

    /// Invalid argument.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// IO error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Host error.
    #[error("host error: {0}")]
    Host(#[from] fusabi_host::Error),

    /// Internal error.
    #[error("internal error: {0}")]
    Internal(String),

    /// Terminal UI error.
    #[error("terminal UI error: {0}")]
    TerminalUI(String),

    /// Kubernetes error.
    #[error("kubernetes error: {0}")]
    K8s(String),

    /// Invalid value error.
    #[error("invalid value: {0}")]
    InvalidValue(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    Serialization(String),
}

impl Error {
    /// Create a not permitted error.
    pub fn not_permitted(msg: impl Into<String>) -> Self {
        Self::NotPermitted(msg.into())
    }

    /// Create a path not allowed error.
    pub fn path_not_allowed(path: impl Into<String>) -> Self {
        Self::PathNotAllowed(path.into())
    }

    /// Create a host not allowed error.
    pub fn host_not_allowed(host: impl Into<String>) -> Self {
        Self::HostNotAllowed(host.into())
    }

    /// Create a timeout error.
    pub fn timeout(duration: std::time::Duration) -> Self {
        Self::Timeout(duration)
    }

    /// Create a process error.
    pub fn process(msg: impl Into<String>) -> Self {
        Self::Process(msg.into())
    }

    /// Create a process exit error.
    pub fn process_exit(code: i32, message: impl Into<String>) -> Self {
        Self::ProcessExit {
            code,
            message: message.into(),
        }
    }

    /// Create a filesystem error.
    pub fn filesystem(msg: impl Into<String>) -> Self {
        Self::Filesystem(msg.into())
    }

    /// Create a network error.
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    /// Create a format error.
    pub fn format(msg: impl Into<String>) -> Self {
        Self::Format(msg.into())
    }

    /// Create an invalid argument error.
    pub fn invalid_argument(msg: impl Into<String>) -> Self {
        Self::InvalidArgument(msg.into())
    }

    /// Check if this is a safety-related error.
    pub fn is_safety_error(&self) -> bool {
        matches!(
            self,
            Self::NotPermitted(_) | Self::PathNotAllowed(_) | Self::HostNotAllowed(_)
        )
    }

    /// Check if this is a timeout error.
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::path_not_allowed("/etc/passwd");
        assert!(err.to_string().contains("/etc/passwd"));

        let err = Error::process_exit(1, "command failed");
        assert!(err.to_string().contains("code 1"));
    }

    #[test]
    fn test_error_classification() {
        assert!(Error::not_permitted("test").is_safety_error());
        assert!(Error::path_not_allowed("/tmp").is_safety_error());
        assert!(!Error::process("test").is_safety_error());

        assert!(Error::timeout(std::time::Duration::from_secs(1)).is_timeout());
        assert!(!Error::process("test").is_timeout());
    }
}
