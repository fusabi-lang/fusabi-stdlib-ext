//! Configuration for stdlib modules.

use std::time::Duration;

use crate::safety::SafetyConfig;

/// Configuration for a specific module.
#[derive(Debug, Clone)]
pub struct ModuleConfig {
    /// Whether the module is enabled.
    pub enabled: bool,
    /// Default timeout for operations.
    pub timeout: Option<Duration>,
    /// Custom configuration options.
    pub options: std::collections::HashMap<String, String>,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Some(Duration::from_secs(30)),
            options: std::collections::HashMap::new(),
        }
    }
}

impl ModuleConfig {
    /// Create a new module configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Disable the module.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    /// Set enabled state.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Remove timeout limit.
    pub fn no_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    /// Set a custom option.
    pub fn with_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }
}

/// Configuration for the stdlib registry.
#[derive(Debug, Clone)]
pub struct StdlibConfig {
    /// Safety configuration.
    pub safety: SafetyConfig,

    /// Process module configuration.
    pub process: ModuleConfig,

    /// Filesystem module configuration.
    pub fs: ModuleConfig,

    /// Path module configuration.
    pub path: ModuleConfig,

    /// Environment module configuration.
    pub env: ModuleConfig,

    /// Format module configuration.
    pub format: ModuleConfig,

    /// Network module configuration.
    pub net: ModuleConfig,

    /// Time module configuration.
    pub time: ModuleConfig,

    /// Metrics module configuration.
    pub metrics: ModuleConfig,
}

impl Default for StdlibConfig {
    fn default() -> Self {
        Self {
            safety: SafetyConfig::default(),
            process: ModuleConfig::disabled(), // Disabled by default for security
            fs: ModuleConfig::default(),
            path: ModuleConfig::default(),
            env: ModuleConfig::default(),
            format: ModuleConfig::default(),
            net: ModuleConfig::disabled(), // Disabled by default for security
            time: ModuleConfig::default(),
            metrics: ModuleConfig::default(),
        }
    }
}

impl StdlibConfig {
    /// Create a new stdlib configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a permissive configuration (for trusted code only).
    pub fn permissive() -> Self {
        Self {
            safety: SafetyConfig::permissive(),
            process: ModuleConfig::default(),
            fs: ModuleConfig::default(),
            path: ModuleConfig::default(),
            env: ModuleConfig::default(),
            format: ModuleConfig::default(),
            net: ModuleConfig::default(),
            time: ModuleConfig::default(),
            metrics: ModuleConfig::default(),
        }
    }

    /// Create a strict configuration (minimal permissions).
    pub fn strict() -> Self {
        Self {
            safety: SafetyConfig::strict(),
            process: ModuleConfig::disabled(),
            fs: ModuleConfig::disabled(),
            path: ModuleConfig::default(),
            env: ModuleConfig::disabled(),
            format: ModuleConfig::default(),
            net: ModuleConfig::disabled(),
            time: ModuleConfig::default(),
            metrics: ModuleConfig::disabled(),
        }
    }

    /// Set safety configuration.
    pub fn with_safety(mut self, safety: SafetyConfig) -> Self {
        self.safety = safety;
        self
    }

    /// Configure the process module.
    pub fn with_process(mut self, config: ModuleConfig) -> Self {
        self.process = config;
        self
    }

    /// Configure the filesystem module.
    pub fn with_fs(mut self, config: ModuleConfig) -> Self {
        self.fs = config;
        self
    }

    /// Configure the path module.
    pub fn with_path(mut self, config: ModuleConfig) -> Self {
        self.path = config;
        self
    }

    /// Configure the environment module.
    pub fn with_env(mut self, config: ModuleConfig) -> Self {
        self.env = config;
        self
    }

    /// Configure the format module.
    pub fn with_format(mut self, config: ModuleConfig) -> Self {
        self.format = config;
        self
    }

    /// Configure the network module.
    pub fn with_net(mut self, config: ModuleConfig) -> Self {
        self.net = config;
        self
    }

    /// Configure the time module.
    pub fn with_time(mut self, config: ModuleConfig) -> Self {
        self.time = config;
        self
    }

    /// Configure the metrics module.
    pub fn with_metrics(mut self, config: ModuleConfig) -> Self {
        self.metrics = config;
        self
    }

    /// Enable all modules.
    pub fn enable_all(mut self) -> Self {
        self.process.enabled = true;
        self.fs.enabled = true;
        self.path.enabled = true;
        self.env.enabled = true;
        self.format.enabled = true;
        self.net.enabled = true;
        self.time.enabled = true;
        self.metrics.enabled = true;
        self
    }

    /// Disable all modules.
    pub fn disable_all(mut self) -> Self {
        self.process.enabled = false;
        self.fs.enabled = false;
        self.path.enabled = false;
        self.env.enabled = false;
        self.format.enabled = false;
        self.net.enabled = false;
        self.time.enabled = false;
        self.metrics.enabled = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_config() {
        let config = ModuleConfig::new()
            .with_timeout(Duration::from_secs(10))
            .with_option("key", "value");

        assert!(config.enabled);
        assert_eq!(config.timeout, Some(Duration::from_secs(10)));
        assert_eq!(config.options.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_stdlib_config_defaults() {
        let config = StdlibConfig::default();

        // Process and net disabled by default
        assert!(!config.process.enabled);
        assert!(!config.net.enabled);

        // Others enabled by default
        assert!(config.fs.enabled);
        assert!(config.time.enabled);
    }

    #[test]
    fn test_stdlib_config_permissive() {
        let config = StdlibConfig::permissive();

        assert!(config.process.enabled);
        assert!(config.net.enabled);
        assert!(config.fs.enabled);
    }

    #[test]
    fn test_stdlib_config_strict() {
        let config = StdlibConfig::strict();

        assert!(!config.process.enabled);
        assert!(!config.net.enabled);
        assert!(!config.fs.enabled);
        assert!(!config.env.enabled);
    }
}
