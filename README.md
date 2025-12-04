# fusabi-stdlib-ext

Extended standard library modules for the Fusabi scripting language ecosystem.

## Overview

`fusabi-stdlib-ext` provides optional, safety-first standard library modules for Fusabi host applications. All modules implement capability-based security with configurable allowlists and resource limits.

## Features

### Module Features

- `process` - Process execution (spawn, exec)
- `fs` - Filesystem operations (read, write, list, mkdir, remove)
- `path` - Path manipulation (join, dirname, basename, normalize)
- `env` - Environment variable access
- `format` - String formatting and JSON encode/decode
- `net` - HTTP client (GET, POST)
- `time` - Time and duration utilities
- `metrics` - Counter, gauge, and histogram metrics

### Pack Features

- `terminal-ui` - Terminal UI utilities (ANSI, prompts)
- `observability` - Logging, tracing, metrics integration
- `k8s` - Kubernetes/cloud helpers
- `mcp` - MCP/AI tool integration

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fusabi-stdlib-ext = { version = "0.1", features = ["fs", "net", "time"] }
```

## Usage

### Basic Setup

```rust
use std::sync::Arc;
use fusabi_stdlib_ext::{StdlibRegistry, StdlibConfig, SafetyConfig};
use fusabi_stdlib_ext::safety::{PathAllowlist, HostAllowlist};
use fusabi_host::Engine;

// Create safety configuration
let safety = Arc::new(
    SafetyConfig::new()
        .with_paths(PathAllowlist::none().allow("/tmp"))
        .with_hosts(HostAllowlist::none().allow("api.example.com"))
);

// Create stdlib registry
let registry = StdlibRegistry::new(StdlibConfig::default(), safety);

// Register with engine
let mut engine = Engine::new();
registry.register_all(&mut engine);
```

### Filesystem Operations

```rust
use fusabi_stdlib_ext::fs;
use fusabi_stdlib_ext::safety::{SafetyConfig, PathAllowlist};

// Configure allowed paths
let safety = Arc::new(
    SafetyConfig::new()
        .with_paths(PathAllowlist::none().allow("/data"))
);

// Read a file
let content = fs::read_file(&safety, &[Value::String("/data/config.json".into())], &ctx)?;

// Write a file
fs::write_file(&safety, &[
    Value::String("/data/output.txt".into()),
    Value::String("Hello, World!".into()),
], &ctx)?;

// List directory
let entries = fs::list_dir(&safety, &[Value::String("/data".into())], &ctx)?;
```

### HTTP Requests

```rust
use fusabi_stdlib_ext::net;
use fusabi_stdlib_ext::safety::{SafetyConfig, HostAllowlist};
use std::time::Duration;

// Configure allowed hosts
let safety = Arc::new(
    SafetyConfig::new()
        .with_hosts(HostAllowlist::none().allow("api.example.com"))
        .with_default_timeout(Duration::from_secs(30))
);

// GET request
let response = net::http_get(
    &safety,
    Some(Duration::from_secs(10)),
    &[Value::String("https://api.example.com/data".into())],
    &ctx,
)?;

// POST request
let response = net::http_post(
    &safety,
    None,
    &[
        Value::String("https://api.example.com/submit".into()),
        Value::String(r#"{"key": "value"}"#.into()),
    ],
    &ctx,
)?;
```

### Process Execution

```rust
use fusabi_stdlib_ext::process;
use fusabi_stdlib_ext::safety::{SafetyConfig, CommandAllowlist};

// Configure allowed commands
let safety = Arc::new(
    SafetyConfig::new()
        .with_commands(CommandAllowlist::none().allow("ls").allow("cat"))
);

// Execute command and wait for result
let result = process::exec(
    &safety,
    Some(Duration::from_secs(5)),
    &[
        Value::String("ls".into()),
        Value::List(vec![Value::String("-la".into())]),
    ],
    &ctx,
)?;

// Spawn background process
let handle = process::spawn(
    &safety,
    &[Value::String("long-running-task".into())],
    &ctx,
)?;
```

### Metrics

```rust
use fusabi_stdlib_ext::metrics;

// Increment a counter
metrics::counter_inc(&[Value::String("requests_total".into())], &ctx)?;
metrics::counter_inc(&[Value::String("requests_total".into()), Value::Int(5)], &ctx)?;

// Set a gauge
metrics::gauge_set(&[
    Value::String("active_connections".into()),
    Value::Float(42.0),
], &ctx)?;

// Observe histogram value
metrics::histogram_observe(&[
    Value::String("request_duration_seconds".into()),
    Value::Float(0.235),
], &ctx)?;
```

### Time Utilities

```rust
use fusabi_stdlib_ext::time;

// Get current timestamp
let now = time::now(&[], &ctx)?;  // Unix seconds
let now_ms = time::now_millis(&[], &ctx)?;  // Unix milliseconds

// Sleep
time::sleep(&[Value::Int(1000)], &ctx)?;  // Sleep 1 second

// Format timestamp
let formatted = time::format_time(&[
    Value::Int(1704067200),
    Value::String("%Y-%m-%d".into()),
], &ctx)?;
```

### String Formatting

```rust
use fusabi_stdlib_ext::format;

// Sprintf-style formatting
let result = format::sprintf(&[
    Value::String("Hello, %s! You have %d messages.".into()),
    Value::String("Alice".into()),
    Value::Int(5),
], &ctx)?;

// Template substitution
let mut values = HashMap::new();
values.insert("name".into(), Value::String("Bob".into()));
let result = format::template(&[
    Value::String("Hello, {{name}}!".into()),
    Value::Map(values),
], &ctx)?;

// JSON encode/decode
let json = format::json_encode(&[value], &ctx)?;
```

## Safety Model

All modules follow a default-deny security model:

### Path Allowlist

```rust
use fusabi_stdlib_ext::safety::PathAllowlist;

// Deny all paths (default)
let paths = PathAllowlist::none();

// Allow specific paths
let paths = PathAllowlist::none()
    .allow("/data")
    .allow("/tmp");

// Allow all paths (use with caution)
let paths = PathAllowlist::all();
```

### Host Allowlist

```rust
use fusabi_stdlib_ext::safety::HostAllowlist;

// Deny all hosts (default)
let hosts = HostAllowlist::none();

// Allow specific hosts
let hosts = HostAllowlist::none()
    .allow("api.example.com")
    .allow("cdn.example.com");

// Allow all hosts (use with caution)
let hosts = HostAllowlist::all();
```

### Timeouts

```rust
use fusabi_stdlib_ext::SafetyConfig;
use std::time::Duration;

let safety = SafetyConfig::new()
    .with_default_timeout(Duration::from_secs(30))
    .with_max_timeout(Duration::from_secs(300));
```

## License

MIT OR Apache-2.0
