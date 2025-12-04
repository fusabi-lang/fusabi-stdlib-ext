//! Example: Filesystem operations with safety controls.

use std::sync::Arc;

use fusabi_host::engine::ExecutionContext;
use fusabi_host::sandbox::{Sandbox, SandboxConfig};
use fusabi_host::{Capabilities, Limits, Value};
use fusabi_stdlib_ext::safety::{PathAllowlist, SafetyConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure safety with allowed paths
    let safety = Arc::new(
        SafetyConfig::new()
            .with_paths(PathAllowlist::none().allow("/tmp").allow("/home")),
    );

    // Create execution context
    let sandbox = Sandbox::new(SandboxConfig::default())?;
    let ctx = ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox);

    println!("Filesystem Operations Example");
    println!("==============================\n");

    // Write a file
    println!("Writing to /tmp/test.txt...");
    let result = fusabi_stdlib_ext::fs::write_file(
        &safety,
        &[
            Value::String("/tmp/test.txt".into()),
            Value::String("Hello from Fusabi!".into()),
        ],
        &ctx,
    )?;
    println!("Write result: {:?}\n", result);

    // Read the file back
    println!("Reading /tmp/test.txt...");
    let result = fusabi_stdlib_ext::fs::read_file(
        &safety,
        &[Value::String("/tmp/test.txt".into())],
        &ctx,
    )?;
    println!("Content: {:?}\n", result);

    // Check if file exists
    println!("Checking if /tmp/test.txt exists...");
    let result = fusabi_stdlib_ext::fs::exists(
        &safety,
        &[Value::String("/tmp/test.txt".into())],
        &ctx,
    )?;
    println!("Exists: {:?}\n", result);

    // List directory
    println!("Listing /tmp...");
    let result = fusabi_stdlib_ext::fs::list_dir(
        &safety,
        &[Value::String("/tmp".into())],
        &ctx,
    )?;
    println!("Entries: {:?}\n", result);

    // Try to access a path outside the allowlist
    println!("Attempting to read /etc/passwd (should fail)...");
    let result = fusabi_stdlib_ext::fs::read_file(
        &safety,
        &[Value::String("/etc/passwd".into())],
        &ctx,
    );
    match result {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Access denied (expected): {}\n", e),
    }

    // Clean up
    println!("Removing /tmp/test.txt...");
    let result = fusabi_stdlib_ext::fs::remove(
        &safety,
        &[Value::String("/tmp/test.txt".into())],
        &ctx,
    )?;
    println!("Remove result: {:?}", result);

    Ok(())
}
