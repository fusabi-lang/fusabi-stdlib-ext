//! Example: HTTP requests with safety controls.

use std::sync::Arc;
use std::time::Duration;

use fusabi_host::engine::ExecutionContext;
use fusabi_host::sandbox::{Sandbox, SandboxConfig};
use fusabi_host::{Capabilities, Limits, Value};
use fusabi_stdlib_ext::safety::{HostAllowlist, SafetyConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure safety with allowed hosts and timeout
    let safety = Arc::new(
        SafetyConfig::new()
            .with_hosts(
                HostAllowlist::none()
                    .allow("httpbin.org")
                    .allow("api.github.com"),
            )
            .with_default_timeout(Duration::from_secs(30))
            .with_max_timeout(Duration::from_secs(60)),
    );

    // Create execution context
    let sandbox = Sandbox::new(SandboxConfig::default())?;
    let ctx = ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox);

    println!("HTTP Request Example");
    println!("====================\n");

    // HTTP GET request
    println!("GET request to httpbin.org/get...");
    let result = fusabi_stdlib_ext::net::http_get(
        &safety,
        Some(Duration::from_secs(10)),
        &[Value::String("https://httpbin.org/get".into())],
        &ctx,
    )?;
    println!("Response: {:?}\n", result);

    // HTTP POST request
    println!("POST request to httpbin.org/post...");
    let result = fusabi_stdlib_ext::net::http_post(
        &safety,
        Some(Duration::from_secs(10)),
        &[
            Value::String("https://httpbin.org/post".into()),
            Value::String(r#"{"message": "Hello from Fusabi!"}"#.into()),
        ],
        &ctx,
    )?;
    println!("Response: {:?}\n", result);

    // Try to access a host outside the allowlist
    println!("Attempting to access evil.com (should fail)...");
    let result = fusabi_stdlib_ext::net::http_get(
        &safety,
        None,
        &[Value::String("https://evil.com/malware".into())],
        &ctx,
    );
    match result {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Access denied (expected): {}\n", e),
    }

    // Demonstrate timeout clamping
    println!("Request with very long timeout (will be clamped to max)...");
    let result = fusabi_stdlib_ext::net::http_get(
        &safety,
        Some(Duration::from_secs(3600)), // 1 hour, will be clamped
        &[Value::String("https://httpbin.org/delay/1".into())],
        &ctx,
    )?;
    println!("Response: {:?}", result);

    Ok(())
}
