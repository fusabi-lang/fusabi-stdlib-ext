//! File streaming module.
//!
//! Provides file streaming capabilities with backpressure control, particularly
//! useful for tailing log files and processing large files without loading them
//! entirely into memory.
//!
//! ## Features
//!
//! - Tail files (like `tail -f`)
//! - Stream file contents with buffering
//! - Backpressure control
//! - Non-blocking reads
//!
//! ## Example
//!
//! ```rust,ignore
//! use fusabi_stdlib_ext::fs_stream;
//!
//! // Open a file for tailing
//! let handle = fs_stream::tail(&[
//!     Value::String("/var/log/app.log".into()),
//!     Value::Int(100),  // buffer size
//! ], &ctx)?;
//!
//! // Read lines as they become available
//! loop {
//!     if let Some(line) = fs_stream::read_line(&[handle.clone()], &ctx)? {
//!         println!("New log line: {}", line);
//!     }
//! }
//! ```

use fusabi_host::{Error, ExecutionContext, Result, Value};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

/// Global registry of open file streams.
/// In a real implementation, this would be managed by the SafetyConfig/Registry.
lazy_static::lazy_static! {
    static ref STREAMS: Arc<Mutex<HashMap<i64, FileStream>>> = Arc::new(Mutex::new(HashMap::new()));
}

static NEXT_HANDLE: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(1);

/// Represents an open file stream.
#[derive(Clone)]
struct FileStream {
    path: String,
    buffer_size: usize,
    position: usize,
}

/// Open a file for tailing (like `tail -f`).
///
/// Returns a handle that can be used with other stream functions.
///
/// # Arguments
///
/// * `args[0]` - File path to tail
/// * `args[1]` - Buffer size (number of lines to buffer)
///
/// # Returns
///
/// Handle (integer) for the stream
pub fn tail(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let path = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::host_function("fs_stream.tail: missing path argument"))?;

    let buffer_size = args.get(1).and_then(|v| v.as_int()).unwrap_or(100) as usize;

    // TODO: Actually open file and set up tailing
    // For now, create a mock stream
    let handle = NEXT_HANDLE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let stream = FileStream {
        path: path.to_string(),
        buffer_size,
        position: 0,
    };

    STREAMS.lock().insert(handle, stream);

    tracing::debug!(
        "fs_stream.tail: opened {} with buffer_size={}, handle={}",
        path,
        buffer_size,
        handle
    );

    Ok(Value::Int(handle))
}

/// Read the next line from a stream (non-blocking).
///
/// Returns `null` if no data is available.
///
/// # Arguments
///
/// * `args[0]` - Stream handle
///
/// # Returns
///
/// String containing the next line, or null if no data available
pub fn read_line(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let handle = args
        .first()
        .and_then(|v| v.as_int())
        .ok_or_else(|| Error::host_function("fs_stream.read_line: missing handle argument"))?;

    let mut streams = STREAMS.lock();
    let stream = streams
        .get_mut(&handle)
        .ok_or_else(|| Error::host_function("fs_stream.read_line: invalid handle"))?;

    // TODO: Actually read from file
    // For now, return mock data occasionally
    stream.position += 1;

    if stream.position % 3 == 0 {
        Ok(Value::String(format!(
            "Mock line {} from {}",
            stream.position, stream.path
        )))
    } else {
        Ok(Value::Null)
    }
}

/// Close a file stream and release resources.
///
/// # Arguments
///
/// * `args[0]` - Stream handle
pub fn close(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let handle = args
        .first()
        .and_then(|v| v.as_int())
        .ok_or_else(|| Error::host_function("fs_stream.close: missing handle argument"))?;

    let mut streams = STREAMS.lock();
    if streams.remove(&handle).is_some() {
        tracing::debug!("fs_stream.close: closed handle {}", handle);
        Ok(Value::Null)
    } else {
        Err(Error::host_function("fs_stream.close: invalid handle"))
    }
}

/// Read all available lines from a stream (non-blocking).
///
/// Returns a list of lines currently available in the buffer.
///
/// # Arguments
///
/// * `args[0]` - Stream handle
///
/// # Returns
///
/// List of strings (may be empty)
pub fn read_available(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let handle = args
        .first()
        .and_then(|v| v.as_int())
        .ok_or_else(|| Error::host_function("fs_stream.read_available: missing handle argument"))?;

    let streams = STREAMS.lock();
    let _stream = streams
        .get(&handle)
        .ok_or_else(|| Error::host_function("fs_stream.read_available: invalid handle"))?;

    // TODO: Actually read available lines
    // For now, return empty list
    Ok(Value::List(vec![]))
}

/// Open a file for streaming (read entire file in chunks).
///
/// Unlike `tail`, this reads from the beginning of the file.
///
/// # Arguments
///
/// * `args[0]` - File path
/// * `args[1]` - Chunk size in bytes (optional, default 4096)
///
/// # Returns
///
/// Handle (integer) for the stream
pub fn open(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let path = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::host_function("fs_stream.open: missing path argument"))?;

    let chunk_size = args.get(1).and_then(|v| v.as_int()).unwrap_or(4096) as usize;

    // TODO: Actually open file
    let handle = NEXT_HANDLE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let stream = FileStream {
        path: path.to_string(),
        buffer_size: chunk_size,
        position: 0,
    };

    STREAMS.lock().insert(handle, stream);

    tracing::debug!(
        "fs_stream.open: opened {} with chunk_size={}, handle={}",
        path,
        chunk_size,
        handle
    );

    Ok(Value::Int(handle))
}

/// Read next chunk from a stream.
///
/// Returns `null` when end of file is reached.
///
/// # Arguments
///
/// * `args[0]` - Stream handle
///
/// # Returns
///
/// String containing the next chunk, or null at EOF
pub fn read_chunk(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let handle = args
        .first()
        .and_then(|v| v.as_int())
        .ok_or_else(|| Error::host_function("fs_stream.read_chunk: missing handle argument"))?;

    let mut streams = STREAMS.lock();
    let stream = streams
        .get_mut(&handle)
        .ok_or_else(|| Error::host_function("fs_stream.read_chunk: invalid handle"))?;

    // TODO: Actually read chunk from file
    stream.position += stream.buffer_size;

    // Mock: return null after a few chunks
    if stream.position > stream.buffer_size * 5 {
        Ok(Value::Null)
    } else {
        Ok(Value::String(format!(
            "Mock chunk at position {}",
            stream.position
        )))
    }
}
