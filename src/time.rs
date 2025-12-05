//! Time module.
//!
//! Provides time and duration utilities.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fusabi_host::ExecutionContext;
use fusabi_host::Value;

/// Get current Unix timestamp in seconds.
pub fn now(
    _args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    Ok(Value::Int(timestamp as i64))
}

/// Get current Unix timestamp in milliseconds.
pub fn now_millis(
    _args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    Ok(Value::Int(timestamp as i64))
}

/// Sleep for a duration in milliseconds.
pub fn sleep(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let millis = args
        .first()
        .and_then(|v| v.as_int())
        .ok_or_else(|| fusabi_host::Error::host_function("time.sleep: missing milliseconds argument"))?;

    if millis < 0 {
        return Err(fusabi_host::Error::host_function("time.sleep: milliseconds must be non-negative"));
    }

    std::thread::sleep(Duration::from_millis(millis as u64));
    Ok(Value::Null)
}

/// Format a Unix timestamp.
pub fn format_time(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let timestamp = args
        .first()
        .and_then(|v| v.as_int())
        .ok_or_else(|| fusabi_host::Error::host_function("time.format: missing timestamp argument"))?;

    let format_str = args
        .get(1)
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d %H:%M:%S");

    // Simple formatting - in real implementation would use chrono
    let formatted = format_timestamp(timestamp, format_str);
    Ok(Value::String(formatted))
}

/// Parse a time string to Unix timestamp.
pub fn parse_time(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let time_str = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("time.parse: missing time string argument"))?;

    let _format_str = args
        .get(1)
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d %H:%M:%S");

    // Simple parsing - in real implementation would use chrono
    // For now, just return an error indicating format not supported
    Err(fusabi_host::Error::host_function(format!(
        "time.parse: parsing '{}' not yet implemented",
        time_str
    )))
}

// Helper function for simple timestamp formatting
fn format_timestamp(timestamp: i64, _format: &str) -> String {
    // Very simple formatting - real implementation would use chrono
    let secs = timestamp as u64;
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    // Calculate approximate date (very simplified, ignoring leap years)
    let years = 1970 + (days / 365);
    let remaining_days = days % 365;
    let month = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        years, month, day, hours, minutes, seconds
    )
}

/// Duration helper functions
pub mod duration {
    /// Convert seconds to milliseconds.
    pub fn seconds_to_millis(secs: i64) -> i64 {
        secs * 1000
    }

    /// Convert milliseconds to seconds.
    pub fn millis_to_seconds(millis: i64) -> i64 {
        millis / 1000
    }

    /// Convert minutes to seconds.
    pub fn minutes_to_seconds(mins: i64) -> i64 {
        mins * 60
    }

    /// Convert hours to seconds.
    pub fn hours_to_seconds(hours: i64) -> i64 {
        hours * 3600
    }

    /// Convert days to seconds.
    pub fn days_to_seconds(days: i64) -> i64 {
        days * 86400
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_host::Capabilities;
    use fusabi_host::{Sandbox, SandboxConfig};
    use fusabi_host::Limits;

    fn create_test_ctx() -> ExecutionContext {
        let sandbox = Sandbox::new(SandboxConfig::default()).unwrap();
        ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox)
    }

    #[test]
    fn test_now() {
        let ctx = create_test_ctx();
        let result = now(&[], &ctx).unwrap();

        let timestamp = result.as_int().unwrap();
        assert!(timestamp > 0);
        assert!(timestamp > 1700000000); // After Nov 2023
    }

    #[test]
    fn test_now_millis() {
        let ctx = create_test_ctx();
        let result = now_millis(&[], &ctx).unwrap();

        let timestamp = result.as_int().unwrap();
        assert!(timestamp > 0);
        assert!(timestamp > 1700000000000); // After Nov 2023 in millis
    }

    #[test]
    fn test_format_time() {
        let ctx = create_test_ctx();

        // Test with a known timestamp (Jan 1, 2024 00:00:00 UTC)
        let result = format_time(&[Value::Int(1704067200)], &ctx).unwrap();
        let formatted = result.as_str().unwrap();

        assert!(formatted.contains("2024"));
    }

    #[test]
    fn test_sleep_validation() {
        let ctx = create_test_ctx();

        // Negative sleep should fail
        let result = sleep(&[Value::Int(-100)], &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_duration_helpers() {
        assert_eq!(duration::seconds_to_millis(5), 5000);
        assert_eq!(duration::millis_to_seconds(5000), 5);
        assert_eq!(duration::minutes_to_seconds(2), 120);
        assert_eq!(duration::hours_to_seconds(1), 3600);
        assert_eq!(duration::days_to_seconds(1), 86400);
    }
}
