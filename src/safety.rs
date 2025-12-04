//! Safety controls for stdlib operations.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::error::{Error, Result};

/// Allowlist for filesystem paths.
#[derive(Debug, Clone, Default)]
pub struct PathAllowlist {
    /// Allowed paths for reading.
    pub read: HashSet<PathBuf>,
    /// Allowed paths for writing.
    pub write: HashSet<PathBuf>,
    /// Denied paths (overrides allowlist).
    pub deny: HashSet<PathBuf>,
}

impl PathAllowlist {
    /// Create an empty allowlist (all paths denied).
    pub fn none() -> Self {
        Self::default()
    }

    /// Create an allowlist that allows all paths.
    pub fn all() -> Self {
        Self {
            read: [PathBuf::from("/")].into_iter().collect(),
            write: [PathBuf::from("/")].into_iter().collect(),
            deny: HashSet::new(),
        }
    }

    /// Add a path for reading.
    pub fn allow_read(mut self, path: impl Into<PathBuf>) -> Self {
        self.read.insert(path.into());
        self
    }

    /// Add a path for writing.
    pub fn allow_write(mut self, path: impl Into<PathBuf>) -> Self {
        self.write.insert(path.into());
        self
    }

    /// Add paths for reading and writing.
    pub fn allow_rw(self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        self.allow_read(path.clone()).allow_write(path)
    }

    /// Deny a path.
    pub fn deny(mut self, path: impl Into<PathBuf>) -> Self {
        self.deny.insert(path.into());
        self
    }

    /// Check if a path is allowed for reading.
    pub fn can_read(&self, path: &Path) -> bool {
        if self.is_denied(path) {
            return false;
        }
        self.read.iter().any(|allowed| path.starts_with(allowed))
    }

    /// Check if a path is allowed for writing.
    pub fn can_write(&self, path: &Path) -> bool {
        if self.is_denied(path) {
            return false;
        }
        self.write.iter().any(|allowed| path.starts_with(allowed))
    }

    /// Check if a path is denied.
    fn is_denied(&self, path: &Path) -> bool {
        self.deny.iter().any(|denied| path.starts_with(denied))
    }

    /// Check read permission, returning error if denied.
    pub fn check_read(&self, path: &Path) -> Result<()> {
        if self.can_read(path) {
            Ok(())
        } else {
            Err(Error::path_not_allowed(path.display().to_string()))
        }
    }

    /// Check write permission, returning error if denied.
    pub fn check_write(&self, path: &Path) -> Result<()> {
        if self.can_write(path) {
            Ok(())
        } else {
            Err(Error::path_not_allowed(path.display().to_string()))
        }
    }
}

/// Allowlist for network hosts.
#[derive(Debug, Clone, Default)]
pub struct HostAllowlist {
    /// Allowed hosts.
    pub allowed: HashSet<String>,
    /// Denied hosts.
    pub denied: HashSet<String>,
}

impl HostAllowlist {
    /// Create an empty allowlist (all hosts denied).
    pub fn none() -> Self {
        Self::default()
    }

    /// Create an allowlist that allows all hosts.
    pub fn all() -> Self {
        Self {
            allowed: ["*".to_string()].into_iter().collect(),
            denied: HashSet::new(),
        }
    }

    /// Add an allowed host.
    pub fn allow(mut self, host: impl Into<String>) -> Self {
        self.allowed.insert(host.into());
        self
    }

    /// Add a denied host.
    pub fn deny(mut self, host: impl Into<String>) -> Self {
        self.denied.insert(host.into());
        self
    }

    /// Check if a host is allowed.
    pub fn can_access(&self, host: &str) -> bool {
        let host = host.to_lowercase();

        // Check deny list first
        for denied in &self.denied {
            if Self::host_matches(&host, denied) {
                return false;
            }
        }

        // Check allow list
        for allowed in &self.allowed {
            if Self::host_matches(&host, allowed) {
                return true;
            }
        }

        false
    }

    fn host_matches(host: &str, pattern: &str) -> bool {
        let pattern = pattern.to_lowercase();

        if pattern == "*" {
            return true;
        }

        if pattern.starts_with("*.") {
            let suffix = &pattern[1..];
            host.ends_with(suffix) || host == &pattern[2..]
        } else {
            host == pattern
        }
    }

    /// Check host permission, returning error if denied.
    pub fn check(&self, host: &str) -> Result<()> {
        if self.can_access(host) {
            Ok(())
        } else {
            Err(Error::host_not_allowed(host))
        }
    }
}

/// Safety configuration for stdlib operations.
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    /// Path allowlist.
    pub paths: PathAllowlist,
    /// Host allowlist.
    pub hosts: HostAllowlist,
    /// Allowed environment variable names (None = all denied).
    pub env_vars: Option<HashSet<String>>,
    /// Whether process execution is allowed.
    pub allow_process: bool,
    /// Allowed process commands (None = all allowed if allow_process is true).
    pub allowed_commands: Option<HashSet<String>>,
    /// Default timeout for operations.
    pub default_timeout: Duration,
    /// Maximum timeout allowed.
    pub max_timeout: Duration,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            paths: PathAllowlist::none(),
            hosts: HostAllowlist::none(),
            env_vars: Some(HashSet::new()),
            allow_process: false,
            allowed_commands: None,
            default_timeout: Duration::from_secs(30),
            max_timeout: Duration::from_secs(300),
        }
    }
}

impl SafetyConfig {
    /// Create a new safety configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a permissive configuration (for trusted code only).
    pub fn permissive() -> Self {
        Self {
            paths: PathAllowlist::all(),
            hosts: HostAllowlist::all(),
            env_vars: None,
            allow_process: true,
            allowed_commands: None,
            default_timeout: Duration::from_secs(60),
            max_timeout: Duration::from_secs(3600),
        }
    }

    /// Create a strict configuration.
    pub fn strict() -> Self {
        Self {
            paths: PathAllowlist::none(),
            hosts: HostAllowlist::none(),
            env_vars: Some(HashSet::new()),
            allow_process: false,
            allowed_commands: Some(HashSet::new()),
            default_timeout: Duration::from_secs(10),
            max_timeout: Duration::from_secs(30),
        }
    }

    /// Set path allowlist.
    pub fn with_paths(mut self, paths: PathAllowlist) -> Self {
        self.paths = paths;
        self
    }

    /// Set host allowlist.
    pub fn with_hosts(mut self, hosts: HostAllowlist) -> Self {
        self.hosts = hosts;
        self
    }

    /// Allow specific environment variables.
    pub fn with_env_vars<I, S>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.env_vars = Some(vars.into_iter().map(Into::into).collect());
        self
    }

    /// Allow all environment variables.
    pub fn allow_all_env(mut self) -> Self {
        self.env_vars = None;
        self
    }

    /// Allow process execution.
    pub fn with_allow_process(mut self, allow: bool) -> Self {
        self.allow_process = allow;
        self
    }

    /// Set allowed commands.
    pub fn with_allowed_commands<I, S>(mut self, commands: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.allowed_commands = Some(commands.into_iter().map(Into::into).collect());
        self
    }

    /// Set default timeout.
    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Set maximum timeout.
    pub fn with_max_timeout(mut self, timeout: Duration) -> Self {
        self.max_timeout = timeout;
        self
    }

    /// Check if an environment variable is accessible.
    pub fn can_access_env(&self, name: &str) -> bool {
        match &self.env_vars {
            None => true,
            Some(allowed) => allowed.contains(name),
        }
    }

    /// Check environment variable access, returning error if denied.
    pub fn check_env(&self, name: &str) -> Result<()> {
        if self.can_access_env(name) {
            Ok(())
        } else {
            Err(Error::not_permitted(format!(
                "environment variable access denied: {}",
                name
            )))
        }
    }

    /// Check if a command is allowed.
    pub fn can_execute(&self, command: &str) -> bool {
        if !self.allow_process {
            return false;
        }

        match &self.allowed_commands {
            None => true,
            Some(allowed) => allowed.contains(command),
        }
    }

    /// Check command execution, returning error if denied.
    pub fn check_execute(&self, command: &str) -> Result<()> {
        if !self.allow_process {
            return Err(Error::not_permitted("process execution not allowed"));
        }

        if let Some(ref allowed) = self.allowed_commands {
            if !allowed.contains(command) {
                return Err(Error::not_permitted(format!(
                    "command not allowed: {}",
                    command
                )));
            }
        }

        Ok(())
    }

    /// Clamp a timeout to the maximum allowed.
    pub fn clamp_timeout(&self, timeout: Duration) -> Duration {
        timeout.min(self.max_timeout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_allowlist() {
        let paths = PathAllowlist::none()
            .allow_read("/tmp")
            .allow_rw("/home/user/data")
            .deny("/home/user/data/secret");

        assert!(paths.can_read(Path::new("/tmp/file.txt")));
        assert!(!paths.can_write(Path::new("/tmp/file.txt")));

        assert!(paths.can_read(Path::new("/home/user/data/file.txt")));
        assert!(paths.can_write(Path::new("/home/user/data/file.txt")));

        assert!(!paths.can_read(Path::new("/home/user/data/secret/key")));
        assert!(!paths.can_write(Path::new("/home/user/data/secret/key")));

        assert!(!paths.can_read(Path::new("/etc/passwd")));
    }

    #[test]
    fn test_host_allowlist() {
        let hosts = HostAllowlist::none()
            .allow("api.example.com")
            .allow("*.trusted.org")
            .deny("evil.trusted.org");

        assert!(hosts.can_access("api.example.com"));
        assert!(hosts.can_access("sub.trusted.org"));
        assert!(hosts.can_access("trusted.org"));
        assert!(!hosts.can_access("evil.trusted.org"));
        assert!(!hosts.can_access("other.com"));
    }

    #[test]
    fn test_safety_config() {
        let config = SafetyConfig::new()
            .with_env_vars(["PATH", "HOME"])
            .with_allow_process(true)
            .with_allowed_commands(["ls", "cat"]);

        assert!(config.can_access_env("PATH"));
        assert!(!config.can_access_env("SECRET"));

        assert!(config.can_execute("ls"));
        assert!(!config.can_execute("rm"));
    }

    #[test]
    fn test_timeout_clamping() {
        let config = SafetyConfig::new().with_max_timeout(Duration::from_secs(60));

        assert_eq!(
            config.clamp_timeout(Duration::from_secs(30)),
            Duration::from_secs(30)
        );
        assert_eq!(
            config.clamp_timeout(Duration::from_secs(120)),
            Duration::from_secs(60)
        );
    }
}
