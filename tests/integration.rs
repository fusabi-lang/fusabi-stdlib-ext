//! Integration tests for fusabi-stdlib-ext.

use std::sync::Arc;
use std::time::Duration;

use fusabi_host::engine::ExecutionContext;
use fusabi_host::sandbox::{Sandbox, SandboxConfig};
use fusabi_host::{Capabilities, Limits, Value};
use fusabi_stdlib_ext::safety::{
    CommandAllowlist, HostAllowlist, PathAllowlist, SafetyConfig,
};

fn create_test_ctx() -> ExecutionContext {
    let sandbox = Sandbox::new(SandboxConfig::default()).unwrap();
    ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox)
}

mod safety_tests {
    use super::*;

    #[test]
    fn test_path_allowlist_deny_by_default() {
        let paths = PathAllowlist::none();
        assert!(paths.check("/etc/passwd").is_err());
        assert!(paths.check("/home/user").is_err());
    }

    #[test]
    fn test_path_allowlist_allow_specific() {
        let paths = PathAllowlist::none().allow("/tmp").allow("/data");

        assert!(paths.check("/tmp/file.txt").is_ok());
        assert!(paths.check("/data/config.json").is_ok());
        assert!(paths.check("/etc/passwd").is_err());
    }

    #[test]
    fn test_path_allowlist_all() {
        let paths = PathAllowlist::all();
        assert!(paths.check("/etc/passwd").is_ok());
        assert!(paths.check("/root/.ssh/id_rsa").is_ok());
    }

    #[test]
    fn test_host_allowlist_deny_by_default() {
        let hosts = HostAllowlist::none();
        assert!(hosts.check("evil.com").is_err());
        assert!(hosts.check("api.example.com").is_err());
    }

    #[test]
    fn test_host_allowlist_allow_specific() {
        let hosts = HostAllowlist::none()
            .allow("api.example.com")
            .allow("cdn.example.com");

        assert!(hosts.check("api.example.com").is_ok());
        assert!(hosts.check("cdn.example.com").is_ok());
        assert!(hosts.check("evil.com").is_err());
    }

    #[test]
    fn test_command_allowlist() {
        let commands = CommandAllowlist::none()
            .allow("ls")
            .allow("cat");

        assert!(commands.check("ls").is_ok());
        assert!(commands.check("cat").is_ok());
        assert!(commands.check("rm").is_err());
    }

    #[test]
    fn test_safety_config_timeout_clamping() {
        let safety = SafetyConfig::new()
            .with_max_timeout(Duration::from_secs(60));

        let clamped = safety.clamp_timeout(Duration::from_secs(120));
        assert_eq!(clamped, Duration::from_secs(60));

        let not_clamped = safety.clamp_timeout(Duration::from_secs(30));
        assert_eq!(not_clamped, Duration::from_secs(30));
    }
}

#[cfg(feature = "fs")]
mod fs_tests {
    use super::*;

    #[test]
    fn test_fs_safety_check() {
        let safety = Arc::new(SafetyConfig::strict());
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::fs::read_file(
            &safety,
            &[Value::String("/etc/passwd".into())],
            &ctx,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fs_with_permission() {
        let safety = Arc::new(
            SafetyConfig::new()
                .with_paths(PathAllowlist::none().allow("/tmp")),
        );
        let ctx = create_test_ctx();

        // Write
        let result = fusabi_stdlib_ext::fs::write_file(
            &safety,
            &[
                Value::String("/tmp/fusabi-test.txt".into()),
                Value::String("test content".into()),
            ],
            &ctx,
        );
        assert!(result.is_ok());

        // Read
        let result = fusabi_stdlib_ext::fs::read_file(
            &safety,
            &[Value::String("/tmp/fusabi-test.txt".into())],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), Some("test content"));

        // Cleanup
        let _ = fusabi_stdlib_ext::fs::remove(
            &safety,
            &[Value::String("/tmp/fusabi-test.txt".into())],
            &ctx,
        );
    }

    #[test]
    fn test_fs_exists() {
        let safety = Arc::new(
            SafetyConfig::new()
                .with_paths(PathAllowlist::none().allow("/tmp")),
        );
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::fs::exists(
            &safety,
            &[Value::String("/tmp".into())],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_bool(), Some(true));
    }
}

#[cfg(feature = "path")]
mod path_tests {
    use super::*;

    #[test]
    fn test_path_join() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::path::join(
            &[
                Value::String("/home".into()),
                Value::String("user".into()),
                Value::String("documents".into()),
            ],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().as_str(),
            Some("/home/user/documents")
        );
    }

    #[test]
    fn test_path_dirname() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::path::dirname(
            &[Value::String("/home/user/file.txt".into())],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), Some("/home/user"));
    }

    #[test]
    fn test_path_basename() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::path::basename(
            &[Value::String("/home/user/file.txt".into())],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), Some("file.txt"));
    }

    #[test]
    fn test_path_extension() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::path::extension(
            &[Value::String("/home/user/file.txt".into())],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), Some("txt"));
    }
}

#[cfg(feature = "net")]
mod net_tests {
    use super::*;

    #[test]
    fn test_net_safety_check() {
        let safety = Arc::new(SafetyConfig::strict());
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::net::http_get(
            &safety,
            None,
            &[Value::String("https://evil.com".into())],
            &ctx,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_net_with_permission() {
        let safety = Arc::new(
            SafetyConfig::new()
                .with_hosts(HostAllowlist::none().allow("example.com")),
        );
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::net::http_get(
            &safety,
            None,
            &[Value::String("https://example.com/api".into())],
            &ctx,
        );
        assert!(result.is_ok());
    }
}

#[cfg(feature = "time")]
mod time_tests {
    use super::*;

    #[test]
    fn test_time_now() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::time::now(&[], &ctx);
        assert!(result.is_ok());

        let timestamp = result.unwrap().as_int().unwrap();
        assert!(timestamp > 1700000000); // After Nov 2023
    }

    #[test]
    fn test_time_now_millis() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::time::now_millis(&[], &ctx);
        assert!(result.is_ok());

        let timestamp = result.unwrap().as_int().unwrap();
        assert!(timestamp > 1700000000000); // After Nov 2023 in millis
    }

    #[test]
    fn test_time_format() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::time::format_time(
            &[Value::Int(1704067200)], // Jan 1, 2024
            &ctx,
        );
        assert!(result.is_ok());
        assert!(result.unwrap().as_str().unwrap().contains("2024"));
    }
}

#[cfg(feature = "format")]
mod format_tests {
    use super::*;

    #[test]
    fn test_sprintf() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::format::sprintf(
            &[
                Value::String("Hello, %s!".into()),
                Value::String("World".into()),
            ],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), Some("Hello, World!"));
    }

    #[test]
    fn test_sprintf_integer() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::format::sprintf(
            &[
                Value::String("Count: %d".into()),
                Value::Int(42),
            ],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), Some("Count: 42"));
    }

    #[test]
    fn test_template() {
        use std::collections::HashMap;

        let ctx = create_test_ctx();

        let mut values = HashMap::new();
        values.insert("name".into(), Value::String("Bob".into()));

        let result = fusabi_stdlib_ext::format::template(
            &[
                Value::String("Hello, {{name}}!".into()),
                Value::Map(values),
            ],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), Some("Hello, Bob!"));
    }

    #[test]
    fn test_json_encode() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::format::json_encode(
            &[Value::Int(42)],
            &ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), Some("42"));
    }
}

#[cfg(feature = "metrics")]
mod metrics_tests {
    use super::*;

    #[test]
    fn test_counter() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::metrics::counter_inc(
            &[Value::String("test_counter_integration".into())],
            &ctx,
        );
        assert!(result.is_ok());

        let result = fusabi_stdlib_ext::metrics::counter_inc(
            &[
                Value::String("test_counter_integration".into()),
                Value::Int(5),
            ],
            &ctx,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_gauge() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::metrics::gauge_set(
            &[
                Value::String("test_gauge_integration".into()),
                Value::Float(42.5),
            ],
            &ctx,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_histogram() {
        let ctx = create_test_ctx();

        for i in 1..=10 {
            let result = fusabi_stdlib_ext::metrics::histogram_observe(
                &[
                    Value::String("test_histogram_integration".into()),
                    Value::Float(i as f64 * 0.1),
                ],
                &ctx,
            );
            assert!(result.is_ok());
        }
    }
}

#[cfg(feature = "env")]
mod env_tests {
    use super::*;

    #[test]
    fn test_env_safety_check() {
        let safety = Arc::new(SafetyConfig::strict());
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::env::get(
            &safety,
            &[Value::String("PATH".into())],
            &ctx,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_env_with_permission() {
        let safety = Arc::new(
            SafetyConfig::new().with_env_vars(["PATH"]),
        );
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::env::get(
            &safety,
            &[Value::String("PATH".into())],
            &ctx,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_cwd() {
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::env::cwd(&[], &ctx);
        assert!(result.is_ok());
        assert!(result.unwrap().as_str().is_some());
    }
}

#[cfg(feature = "process")]
mod process_tests {
    use super::*;

    #[test]
    fn test_process_safety_check() {
        let safety = Arc::new(SafetyConfig::strict());
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::process::exec(
            &safety,
            None,
            &[Value::String("ls".into())],
            &ctx,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_process_with_permission() {
        let safety = Arc::new(
            SafetyConfig::new()
                .with_commands(CommandAllowlist::none().allow("echo")),
        );
        let ctx = create_test_ctx();

        let result = fusabi_stdlib_ext::process::exec(
            &safety,
            Some(Duration::from_secs(5)),
            &[
                Value::String("echo".into()),
                Value::List(vec![Value::String("test".into())]),
            ],
            &ctx,
        );
        assert!(result.is_ok());
    }
}
