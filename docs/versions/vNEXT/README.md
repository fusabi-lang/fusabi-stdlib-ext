# fusabi-stdlib-ext - vNEXT Documentation

Extended standard library modules for the Fusabi scripting language ecosystem.

## Overview

`fusabi-stdlib-ext` provides optional, safety-first standard library modules for Fusabi host applications. All modules implement capability-based security with configurable allowlists and resource limits.

## Features

### Core Modules

- `process` - Process execution (spawn, exec)
- `fs` - Filesystem operations (read, write, list, mkdir, remove)
- `path` - Path manipulation (join, dirname, basename, normalize)
- `env` - Environment variable access
- `format` - String formatting and JSON encode/decode
- `net` - HTTP client (GET, POST)
- `time` - Time and duration utilities
- `metrics` - Counter, gauge, and histogram metrics

### Extended Modules (vNEXT)

- `terminal` - Terminal UI utilities: key events, selection, clipboard, palette helpers
- `gpu` - GPU metrics: NVML/DGX metrics shims for observability
- `fs_stream` - File streaming: tail files with backpressure for log processing
- `net_http` - Enhanced HTTP client: lightweight client with advanced timeout controls

### Domain Packs

- `terminal-ui` - Terminal UI utilities (ANSI, prompts)
- `observability` - Logging, tracing, metrics integration
- `k8s` - Kubernetes/cloud helpers
- `mcp` - MCP/AI tool integration

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fusabi-stdlib-ext = { version = "0.2", features = ["fs", "net", "time", "terminal"] }
```

## Capability Flags

All modules support capability-based security:

| Feature | Capability | Description | Use Cases |
|---------|-----------|-------------|-----------|
| `process` | Command execution | Spawn and control processes | Scarab task runners |
| `fs` | File system | Read/write files and directories | Configuration management |
| `path` | Path manipulation | Normalize and validate paths | Safe path handling |
| `env` | Environment vars | Read environment variables | Configuration |
| `format` | Formatting | String formatting, JSON | Data serialization |
| `net` | Network HTTP | HTTP client requests | API integration |
| `time` | Time utilities | Timestamps, durations | Scheduling |
| `metrics` | Observability | Prometheus-style metrics | Monitoring |
| `terminal` | Terminal I/O | Key events, clipboard, colors | Interactive UIs |
| `gpu` | GPU metrics | NVML/DGX monitoring | Hibana/Tolaria |
| `fs_stream` | File streaming | Tail files with backpressure | Log processing |
| `net_http` | HTTP advanced | Enhanced HTTP client | API gateways |

## Usage Examples

### Terminal Module (vNEXT)

```rust
use fusabi_stdlib_ext::terminal;

// Read key events
let key = terminal::read_key(&[], &ctx)?;

// Get terminal size
let (width, height) = terminal::size(&[], &ctx)?;

// Read clipboard
let text = terminal::clipboard_read(&[], &ctx)?;

// Write to clipboard
terminal::clipboard_write(&[Value::String("copied text".into())], &ctx)?;

// Apply color palette
let colored = terminal::colorize(&[
    Value::String("Hello".into()),
    Value::String("green".into()),
], &ctx)?;
```

### GPU Metrics Module (vNEXT)

```rust
use fusabi_stdlib_ext::gpu;

// Get GPU utilization
let utilization = gpu::utilization(&[Value::Int(0)], &ctx)?;  // GPU 0

// Get GPU memory info
let memory = gpu::memory_info(&[Value::Int(0)], &ctx)?;

// Get GPU temperature
let temp = gpu::temperature(&[Value::Int(0)], &ctx)?;

// List all GPUs
let gpus = gpu::list_devices(&[], &ctx)?;
```

### File Streaming Module (vNEXT)

```rust
use fusabi_stdlib_ext::fs_stream;

// Tail a file with backpressure
let stream = fs_stream::tail(&[
    Value::String("/var/log/app.log".into()),
    Value::Int(100),  // Buffer size
], &ctx)?;

// Read next line from stream
let line = fs_stream::read_line(&[stream_handle], &ctx)?;

// Close stream
fs_stream::close(&[stream_handle], &ctx)?;
```

### Enhanced HTTP Client (vNEXT)

```rust
use fusabi_stdlib_ext::net_http;

// HTTP request with advanced options
let response = net_http::request(&[
    Value::String("GET".into()),
    Value::String("https://api.example.com/data".into()),
    Value::Map(headers),
    Value::Map(options),  // timeout, retries, etc.
], &ctx)?;

// Streaming download
let stream = net_http::download_stream(&[
    Value::String("https://cdn.example.com/large-file.bin".into()),
], &ctx)?;
```

## Safety Model

All modules follow a default-deny security model. See main README and docs/STRUCTURE.md for details.

## Migration from v0.1

- No breaking changes in core modules
- New modules require explicit feature flags
- `net_http` extends but doesn't replace `net` module

## See Also

- [Installation Guide](./installation.md)
- [API Reference](./api/README.md)
- [Examples](./examples/README.md)
- [Safety Model](./safety.md)
