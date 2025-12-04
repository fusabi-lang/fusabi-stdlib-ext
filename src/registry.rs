//! Stdlib module registry for registering modules with engines.

use std::sync::Arc;

use fusabi_host::engine::HostRegistry;

use crate::config::StdlibConfig;
use crate::error::{Error, Result};
use crate::safety::SafetyConfig;

/// Registry for stdlib modules.
pub struct StdlibRegistry {
    config: StdlibConfig,
    safety: Arc<SafetyConfig>,
}

impl StdlibRegistry {
    /// Create a new stdlib registry.
    pub fn new(config: StdlibConfig) -> Result<Self> {
        let safety = Arc::new(config.safety.clone());

        Ok(Self { config, safety })
    }

    /// Create with default configuration.
    pub fn default_config() -> Result<Self> {
        Self::new(StdlibConfig::default())
    }

    /// Get the configuration.
    pub fn config(&self) -> &StdlibConfig {
        &self.config
    }

    /// Get the safety configuration.
    pub fn safety(&self) -> &SafetyConfig {
        &self.safety
    }

    /// Register all enabled modules with a host registry.
    pub fn register_all(&self, registry: &mut HostRegistry) -> Result<()> {
        #[cfg(feature = "process")]
        if self.config.process.enabled {
            self.register_process(registry)?;
        }

        #[cfg(feature = "fs")]
        if self.config.fs.enabled {
            self.register_fs(registry)?;
        }

        #[cfg(feature = "path")]
        if self.config.path.enabled {
            self.register_path(registry)?;
        }

        #[cfg(feature = "env")]
        if self.config.env.enabled {
            self.register_env(registry)?;
        }

        #[cfg(feature = "format")]
        if self.config.format.enabled {
            self.register_format(registry)?;
        }

        #[cfg(feature = "net")]
        if self.config.net.enabled {
            self.register_net(registry)?;
        }

        #[cfg(feature = "time")]
        if self.config.time.enabled {
            self.register_time(registry)?;
        }

        #[cfg(feature = "metrics")]
        if self.config.metrics.enabled {
            self.register_metrics(registry)?;
        }

        Ok(())
    }

    /// Register the process module.
    #[cfg(feature = "process")]
    pub fn register_process(&self, registry: &mut HostRegistry) -> Result<()> {
        use crate::process;

        let safety = self.safety.clone();
        let timeout = self.config.process.timeout;

        registry.register_module("process", "exec", move |args, ctx| {
            process::exec(&safety, timeout, args, ctx)
        });

        registry.register_module("process", "spawn", move |args, ctx| {
            process::spawn(args, ctx)
        });

        Ok(())
    }

    /// Register the filesystem module.
    #[cfg(feature = "fs")]
    pub fn register_fs(&self, registry: &mut HostRegistry) -> Result<()> {
        use crate::fs;

        let safety = self.safety.clone();

        let s = safety.clone();
        registry.register_module("fs", "read", move |args, ctx| {
            fs::read_file(&s, args, ctx)
        });

        let s = safety.clone();
        registry.register_module("fs", "write", move |args, ctx| {
            fs::write_file(&s, args, ctx)
        });

        let s = safety.clone();
        registry.register_module("fs", "exists", move |args, ctx| {
            fs::exists(&s, args, ctx)
        });

        let s = safety.clone();
        registry.register_module("fs", "list", move |args, ctx| {
            fs::list_dir(&s, args, ctx)
        });

        let s = safety.clone();
        registry.register_module("fs", "mkdir", move |args, ctx| {
            fs::mkdir(&s, args, ctx)
        });

        let s = safety.clone();
        registry.register_module("fs", "remove", move |args, ctx| {
            fs::remove(&s, args, ctx)
        });

        Ok(())
    }

    /// Register the path module.
    #[cfg(feature = "path")]
    pub fn register_path(&self, registry: &mut HostRegistry) -> Result<()> {
        use crate::path;

        registry.register_module("path", "join", |args, ctx| {
            path::join(args, ctx)
        });

        registry.register_module("path", "dirname", |args, ctx| {
            path::dirname(args, ctx)
        });

        registry.register_module("path", "basename", |args, ctx| {
            path::basename(args, ctx)
        });

        registry.register_module("path", "extension", |args, ctx| {
            path::extension(args, ctx)
        });

        registry.register_module("path", "normalize", |args, ctx| {
            path::normalize(args, ctx)
        });

        registry.register_module("path", "is_absolute", |args, ctx| {
            path::is_absolute(args, ctx)
        });

        Ok(())
    }

    /// Register the environment module.
    #[cfg(feature = "env")]
    pub fn register_env(&self, registry: &mut HostRegistry) -> Result<()> {
        use crate::env;

        let safety = self.safety.clone();

        let s = safety.clone();
        registry.register_module("env", "get", move |args, ctx| {
            env::get(&s, args, ctx)
        });

        let s = safety.clone();
        registry.register_module("env", "set", move |args, ctx| {
            env::set(&s, args, ctx)
        });

        registry.register_module("env", "cwd", |args, ctx| {
            env::cwd(args, ctx)
        });

        Ok(())
    }

    /// Register the format module.
    #[cfg(feature = "format")]
    pub fn register_format(&self, registry: &mut HostRegistry) -> Result<()> {
        use crate::format;

        registry.register_module("format", "sprintf", |args, ctx| {
            format::sprintf(args, ctx)
        });

        registry.register_module("format", "template", |args, ctx| {
            format::template(args, ctx)
        });

        registry.register_module("format", "json_encode", |args, ctx| {
            format::json_encode(args, ctx)
        });

        registry.register_module("format", "json_decode", |args, ctx| {
            format::json_decode(args, ctx)
        });

        Ok(())
    }

    /// Register the network module.
    #[cfg(feature = "net")]
    pub fn register_net(&self, registry: &mut HostRegistry) -> Result<()> {
        use crate::net;

        let safety = self.safety.clone();
        let timeout = self.config.net.timeout;

        let s = safety.clone();
        registry.register_module("net", "get", move |args, ctx| {
            net::http_get(&s, timeout, args, ctx)
        });

        let s = safety.clone();
        registry.register_module("net", "post", move |args, ctx| {
            net::http_post(&s, timeout, args, ctx)
        });

        Ok(())
    }

    /// Register the time module.
    #[cfg(feature = "time")]
    pub fn register_time(&self, registry: &mut HostRegistry) -> Result<()> {
        use crate::time;

        registry.register_module("time", "now", |args, ctx| {
            time::now(args, ctx)
        });

        registry.register_module("time", "now_millis", |args, ctx| {
            time::now_millis(args, ctx)
        });

        registry.register_module("time", "sleep", |args, ctx| {
            time::sleep(args, ctx)
        });

        registry.register_module("time", "format", |args, ctx| {
            time::format_time(args, ctx)
        });

        registry.register_module("time", "parse", |args, ctx| {
            time::parse_time(args, ctx)
        });

        Ok(())
    }

    /// Register the metrics module.
    #[cfg(feature = "metrics")]
    pub fn register_metrics(&self, registry: &mut HostRegistry) -> Result<()> {
        use crate::metrics;

        registry.register_module("metrics", "counter_inc", |args, ctx| {
            metrics::counter_inc(args, ctx)
        });

        registry.register_module("metrics", "gauge_set", |args, ctx| {
            metrics::gauge_set(args, ctx)
        });

        registry.register_module("metrics", "histogram_observe", |args, ctx| {
            metrics::histogram_observe(args, ctx)
        });

        Ok(())
    }
}

impl std::fmt::Debug for StdlibRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StdlibRegistry")
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = StdlibRegistry::default_config().unwrap();
        assert!(registry.config().fs.enabled);
    }

    #[test]
    fn test_registry_permissive() {
        let config = StdlibConfig::permissive();
        let registry = StdlibRegistry::new(config).unwrap();

        assert!(registry.config().process.enabled);
        assert!(registry.config().net.enabled);
    }

    #[test]
    fn test_registry_strict() {
        let config = StdlibConfig::strict();
        let registry = StdlibRegistry::new(config).unwrap();

        assert!(!registry.config().process.enabled);
        assert!(!registry.config().fs.enabled);
    }
}
