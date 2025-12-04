//! Example: Process execution with safety controls and timeouts.

use std::sync::Arc;
use std::time::Duration;

use fusabi_host::engine::ExecutionContext;
use fusabi_host::sandbox::{Sandbox, SandboxConfig};
use fusabi_host::{Capabilities, Limits, Value};
use fusabi_stdlib_ext::safety::{CommandAllowlist, SafetyConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure safety with allowed commands
    let safety = Arc::new(
        SafetyConfig::new()
            .with_commands(
                CommandAllowlist::none()
                    .allow("echo")
                    .allow("ls")
                    .allow("cat")
                    .allow("date"),
            )
            .with_default_timeout(Duration::from_secs(10)),
    );

    // Create execution context
    let sandbox = Sandbox::new(SandboxConfig::default())?;
    let ctx = ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox);

    println!("Process Execution Example");
    println!("=========================\n");

    // Execute echo command
    println!("Running: echo 'Hello from Fusabi!'");
    let result = fusabi_stdlib_ext::process::exec(
        &safety,
        Some(Duration::from_secs(5)),
        &[
            Value::String("echo".into()),
            Value::List(vec![Value::String("Hello from Fusabi!".into())]),
        ],
        &ctx,
    )?;
    println!("Result: {:?}\n", result);

    // Execute ls command
    println!("Running: ls -la /tmp");
    let result = fusabi_stdlib_ext::process::exec(
        &safety,
        Some(Duration::from_secs(5)),
        &[
            Value::String("ls".into()),
            Value::List(vec![
                Value::String("-la".into()),
                Value::String("/tmp".into()),
            ]),
        ],
        &ctx,
    )?;
    println!("Result: {:?}\n", result);

    // Execute date command
    println!("Running: date");
    let result = fusabi_stdlib_ext::process::exec(
        &safety,
        None,
        &[Value::String("date".into())],
        &ctx,
    )?;
    println!("Result: {:?}\n", result);

    // Try to execute a command outside the allowlist
    println!("Attempting to run 'rm -rf /' (should fail)...");
    let result = fusabi_stdlib_ext::process::exec(
        &safety,
        None,
        &[
            Value::String("rm".into()),
            Value::List(vec![
                Value::String("-rf".into()),
                Value::String("/".into()),
            ]),
        ],
        &ctx,
    );
    match result {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Access denied (expected): {}\n", e),
    }

    // Spawn a background process
    println!("Spawning background process: echo 'background task'");
    let result = fusabi_stdlib_ext::process::spawn(
        &safety,
        &[
            Value::String("echo".into()),
            Value::List(vec![Value::String("background task".into())]),
        ],
        &ctx,
    )?;
    println!("Spawn result: {:?}", result);

    Ok(())
}
