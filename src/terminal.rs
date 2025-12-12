//! Terminal module.
//!
//! Provides terminal interaction utilities including key events, clipboard access,
//! and color palette helpers for interactive terminal UIs.
//!
//! ## Features
//!
//! - Read key events (blocking and non-blocking)
//! - Get terminal dimensions
//! - Clipboard read/write (platform-dependent)
//! - ANSI color utilities
//!
//! ## Example
//!
//! ```rust,ignore
//! use fusabi_stdlib_ext::terminal;
//!
//! // Read a single keypress
//! let key = terminal::read_key(&[], &ctx)?;
//!
//! // Get terminal size
//! let size = terminal::size(&[], &ctx)?;
//!
//! // Clipboard operations
//! terminal::clipboard_write(&[Value::String("text".into())], &ctx)?;
//! let text = terminal::clipboard_read(&[], &ctx)?;
//! ```

use fusabi_host::{Error, ExecutionContext, Result, Value};

/// Read a single key event (blocking).
///
/// Returns the key name as a string (e.g., "Enter", "Ctrl+C", "a", "ArrowUp").
///
/// # Arguments
///
/// * `args` - No arguments required
/// * `ctx` - Execution context
///
/// # Returns
///
/// String representing the key pressed
pub fn read_key(_args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    // TODO: Implement using crossterm
    // For now, return a placeholder
    tracing::warn!("terminal.read_key: not yet implemented");
    Err(Error::host_function(
        "terminal.read_key: not yet implemented",
    ))
}

/// Get terminal dimensions.
///
/// Returns a list `[width, height]` in columns and rows.
///
/// # Returns
///
/// List with two integers: [width_columns, height_rows]
pub fn size(_args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    // TODO: Implement using crossterm
    // For now, return default terminal size
    tracing::debug!("terminal.size: returning default 80x24");
    Ok(Value::List(vec![Value::Int(80), Value::Int(24)]))
}

/// Read text from system clipboard.
///
/// # Returns
///
/// String containing clipboard contents
pub fn clipboard_read(_args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    // TODO: Implement using clipboard crate
    tracing::warn!("terminal.clipboard_read: not yet implemented");
    Err(Error::host_function(
        "terminal.clipboard_read: not yet implemented",
    ))
}

/// Write text to system clipboard.
///
/// # Arguments
///
/// * `args[0]` - Text to write to clipboard
pub fn clipboard_write(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let text = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::host_function("terminal.clipboard_write: missing text argument"))?;

    // TODO: Implement using clipboard crate
    tracing::warn!(
        "terminal.clipboard_write: not yet implemented (text: {})",
        text
    );
    Err(Error::host_function(
        "terminal.clipboard_write: not yet implemented",
    ))
}

/// Apply ANSI color to text.
///
/// # Arguments
///
/// * `args[0]` - Text to colorize
/// * `args[1]` - Color name (red, green, blue, yellow, etc.)
///
/// # Returns
///
/// ANSI-formatted string
pub fn colorize(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let text = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::host_function("terminal.colorize: missing text argument"))?;

    let color = args
        .get(1)
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::host_function("terminal.colorize: missing color argument"))?;

    // Simple ANSI color codes
    let color_code = match color.to_lowercase().as_str() {
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        _ => {
            return Err(Error::host_function(format!(
                "terminal.colorize: unknown color '{}'",
                color
            )))
        }
    };

    let colored = format!("\x1b[{}m{}\x1b[0m", color_code, text);
    Ok(Value::String(colored))
}

/// Clear the terminal screen.
pub fn clear(_args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    // TODO: Implement using crossterm
    tracing::debug!("terminal.clear: not yet implemented");
    Err(Error::host_function("terminal.clear: not yet implemented"))
}

/// Set cursor position.
///
/// # Arguments
///
/// * `args[0]` - Column (x)
/// * `args[1]` - Row (y)
pub fn set_cursor(args: &[Value], _ctx: &ExecutionContext) -> Result<Value> {
    let _x = args
        .first()
        .and_then(|v| v.as_int())
        .ok_or_else(|| Error::host_function("terminal.set_cursor: missing x argument"))?;

    let _y = args
        .get(1)
        .and_then(|v| v.as_int())
        .ok_or_else(|| Error::host_function("terminal.set_cursor: missing y argument"))?;

    // TODO: Implement using crossterm
    tracing::debug!("terminal.set_cursor: not yet implemented");
    Err(Error::host_function(
        "terminal.set_cursor: not yet implemented",
    ))
}
