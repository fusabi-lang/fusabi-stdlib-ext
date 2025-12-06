# API Reference - vNEXT

Complete API documentation for all fusabi-stdlib-ext modules.

## Core Modules

### Process Module

**Feature**: `process`

Functions for process execution and management.

#### `exec(command: String, args: List<String>) -> Map`

Execute a command and wait for completion.

**Parameters**:
- `command`: Command to execute
- `args`: List of command arguments

**Returns**: Map with `stdout`, `stderr`, and `exit_code`

**Requires**: Command allowlist configuration

---

### Filesystem Module

**Feature**: `fs`

Functions for filesystem operations.

#### `read_file(path: String) -> String`

Read file contents as string.

#### `write_file(path: String, content: String) -> Null`

Write string content to file.

#### `list_dir(path: String) -> List<String>`

List directory contents.

---

## Extended Modules (vNEXT)

### Terminal Module

**Feature**: `terminal`

Terminal interaction and UI utilities.

#### `read_key() -> String`

Read a single keypress. Returns key name (e.g., "Enter", "Ctrl+C", "a").

**Blocking**: Yes

**Example**:
```rust
let key = terminal::read_key(&[], &ctx)?;
```

#### `size() -> List<Int>`

Get terminal dimensions as `[width, height]`.

**Returns**: List with two integers: columns and rows

#### `clipboard_read() -> String`

Read text from system clipboard.

**Requires**: Clipboard capability (platform-dependent)

#### `clipboard_write(text: String) -> Null`

Write text to system clipboard.

**Parameters**:
- `text`: Text to copy

#### `colorize(text: String, color: String) -> String`

Apply ANSI color to text.

**Parameters**:
- `text`: Text to colorize
- `color`: Color name (e.g., "red", "green", "blue", "yellow")

**Returns**: ANSI-formatted string

---

### GPU Module

**Feature**: `gpu`

GPU monitoring and metrics (NVML-based).

#### `list_devices() -> List<Map>`

List all available GPU devices.

**Returns**: List of maps with `id`, `name`, `uuid` fields

#### `utilization(device_id: Int) -> Float`

Get GPU utilization percentage (0-100).

**Parameters**:
- `device_id`: GPU device index

#### `memory_info(device_id: Int) -> Map`

Get GPU memory information.

**Returns**: Map with `total`, `used`, `free` (bytes)

#### `temperature(device_id: Int) -> Float`

Get GPU temperature in Celsius.

**Requires**: NVML library (nvidia-smi on system)

---

### File Streaming Module

**Feature**: `fs_stream`

Stream file contents with backpressure control.

#### `tail(path: String, buffer_size: Int) -> Handle`

Open file for tailing (like `tail -f`).

**Parameters**:
- `path`: File path to tail
- `buffer_size`: Internal buffer size

**Returns**: Stream handle

#### `read_line(handle: Handle) -> String | Null`

Read next line from stream (non-blocking).

**Returns**: Next line or Null if no data available

#### `close(handle: Handle) -> Null`

Close stream and release resources.

---

### Enhanced HTTP Module

**Feature**: `net_http`

Advanced HTTP client with retries, timeouts, and streaming.

#### `request(method: String, url: String, headers: Map, options: Map) -> Map`

Make HTTP request with full control.

**Parameters**:
- `method`: HTTP method (GET, POST, PUT, DELETE, etc.)
- `url`: Request URL
- `headers`: Map of header key-value pairs
- `options`: Configuration map:
  - `timeout`: Request timeout in milliseconds (optional)
  - `retries`: Number of retry attempts (optional, default: 0)
  - `retry_delay`: Delay between retries in ms (optional, default: 1000)
  - `body`: Request body (optional)

**Returns**: Map with `status`, `headers`, `body`

#### `download_stream(url: String) -> Handle`

Stream download large file.

**Parameters**:
- `url`: URL to download

**Returns**: Stream handle for reading chunks

#### `upload_stream(url: String, stream: Handle) -> Map`

Upload from stream.

---

## Safety and Capabilities

All functions respect the configured safety policies:

- **Filesystem**: Path allowlist
- **Network**: Host allowlist
- **Process**: Command allowlist
- **Timeouts**: Configurable per-operation and default

See [Safety Model](./safety.md) for configuration details.
