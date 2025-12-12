//! Observability module for Fusabi.
//!
//! Provides logging, tracing, and metrics integration using OpenTelemetry.

use opentelemetry::{
    global,
    trace::{Tracer, TracerProvider},
    KeyValue,
};
use std::collections::HashMap;
use std::time::Duration;

use crate::error::{Error, Result};
use fusabi_host::Value;

/// Configuration for observability features.
#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    /// Service name for tracing.
    pub service_name: String,
    /// Service version.
    pub service_version: String,
    /// Additional resource attributes.
    pub resource_attributes: HashMap<String, String>,
    /// Whether to enable tracing.
    pub tracing_enabled: bool,
    /// Whether to enable metrics.
    pub metrics_enabled: bool,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            service_name: "fusabi-app".to_string(),
            service_version: "0.1.0".to_string(),
            resource_attributes: HashMap::new(),
            tracing_enabled: true,
            metrics_enabled: true,
        }
    }
}

impl ObservabilityConfig {
    /// Create a new configuration with the given service name.
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            ..Default::default()
        }
    }

    /// Set the service version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.service_version = version.into();
        self
    }

    /// Add a resource attribute.
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.resource_attributes.insert(key.into(), value.into());
        self
    }

    /// Enable or disable tracing.
    pub fn with_tracing(mut self, enabled: bool) -> Self {
        self.tracing_enabled = enabled;
        self
    }

    /// Enable or disable metrics.
    pub fn with_metrics(mut self, enabled: bool) -> Self {
        self.metrics_enabled = enabled;
        self
    }
}

/// Span context for distributed tracing.
#[derive(Debug, Clone)]
pub struct SpanContext {
    /// Trace ID.
    pub trace_id: String,
    /// Span ID.
    pub span_id: String,
    /// Span name.
    pub name: String,
    /// Start time in nanoseconds since epoch.
    pub start_time_ns: u64,
    /// Attributes attached to the span.
    pub attributes: HashMap<String, Value>,
}

impl SpanContext {
    /// Create a new span context.
    pub fn new(name: impl Into<String>) -> Self {
        use std::time::SystemTime;

        let start_time_ns = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        Self {
            trace_id: generate_id(16),
            span_id: generate_id(8),
            name: name.into(),
            start_time_ns,
            attributes: HashMap::new(),
        }
    }

    /// Add an attribute to the span.
    pub fn with_attribute(mut self, key: impl Into<String>, value: Value) -> Self {
        self.attributes.insert(key.into(), value);
        self
    }

    /// Calculate the duration since span start.
    pub fn elapsed(&self) -> Duration {
        use std::time::SystemTime;

        let now_ns = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        Duration::from_nanos(now_ns.saturating_sub(self.start_time_ns))
    }
}

/// Log level for structured logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Trace level.
    Trace,
    /// Debug level.
    Debug,
    /// Info level.
    Info,
    /// Warning level.
    Warn,
    /// Error level.
    Error,
}

impl LogLevel {
    /// Convert to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Structured log entry.
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Log level.
    pub level: LogLevel,
    /// Log message.
    pub message: String,
    /// Structured fields.
    pub fields: HashMap<String, Value>,
    /// Timestamp in nanoseconds since epoch.
    pub timestamp_ns: u64,
}

impl LogEntry {
    /// Create a new log entry.
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        use std::time::SystemTime;

        let timestamp_ns = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        Self {
            level,
            message: message.into(),
            fields: HashMap::new(),
            timestamp_ns,
        }
    }

    /// Add a field to the log entry.
    pub fn with_field(mut self, key: impl Into<String>, value: Value) -> Self {
        self.fields.insert(key.into(), value);
        self
    }
}

/// Generate a random hex ID of the specified byte length.
fn generate_id(bytes: usize) -> String {
    use std::time::SystemTime;

    // Simple pseudo-random ID generation (not cryptographically secure)
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    let mut result = String::with_capacity(bytes * 2);
    let mut state = seed;
    for _ in 0..bytes {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        result.push_str(&format!("{:02x}", (state >> 56) as u8));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ObservabilityConfig::new("test-service")
            .with_version("1.0.0")
            .with_attribute("env", "test")
            .with_tracing(true)
            .with_metrics(false);

        assert_eq!(config.service_name, "test-service");
        assert_eq!(config.service_version, "1.0.0");
        assert!(config.tracing_enabled);
        assert!(!config.metrics_enabled);
    }

    #[test]
    fn test_span_context() {
        let span =
            SpanContext::new("test-span").with_attribute("key", Value::String("value".into()));

        assert_eq!(span.name, "test-span");
        assert!(!span.trace_id.is_empty());
        assert!(!span.span_id.is_empty());
    }

    #[test]
    fn test_log_entry() {
        let entry =
            LogEntry::new(LogLevel::Info, "test message").with_field("count", Value::Int(42));

        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "test message");
    }

    #[test]
    fn test_generate_id() {
        let id = generate_id(8);
        assert_eq!(id.len(), 16); // 8 bytes = 16 hex chars
    }
}
