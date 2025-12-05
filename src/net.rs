//! Network module.
//!
//! Provides HTTP request functions with safety controls.

use std::sync::Arc;
use std::time::Duration;

use fusabi_host::ExecutionContext;
use fusabi_host::Value;

use crate::safety::SafetyConfig;

/// Perform an HTTP GET request.
pub fn http_get(
    safety: &Arc<SafetyConfig>,
    timeout: Option<Duration>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let url = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("net.get: missing URL argument"))?;

    // Extract host from URL
    let host = extract_host(url)?;

    // Check safety
    safety.hosts.check(&host).map_err(|e| {
        fusabi_host::Error::host_function(e.to_string())
    })?;

    // Apply timeout
    let timeout = timeout
        .map(|t| safety.clamp_timeout(t))
        .unwrap_or(safety.default_timeout);

    // Perform request (simulated)
    tracing::info!("HTTP GET {} (timeout: {:?})", url, timeout);

    // In real implementation, would use reqwest
    Ok(Value::Map({
        let mut m = std::collections::HashMap::new();
        m.insert("status".into(), Value::Int(200));
        m.insert("body".into(), Value::String(format!("Response from {}", url)));
        m.insert("headers".into(), Value::Map(std::collections::HashMap::new()));
        m
    }))
}

/// Perform an HTTP POST request.
pub fn http_post(
    safety: &Arc<SafetyConfig>,
    timeout: Option<Duration>,
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let url = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("net.post: missing URL argument"))?;

    let body = args
        .get(1)
        .map(|v| v.to_string())
        .unwrap_or_default();

    // Extract host from URL
    let host = extract_host(url)?;

    // Check safety
    safety.hosts.check(&host).map_err(|e| {
        fusabi_host::Error::host_function(e.to_string())
    })?;

    // Apply timeout
    let timeout = timeout
        .map(|t| safety.clamp_timeout(t))
        .unwrap_or(safety.default_timeout);

    // Perform request (simulated)
    tracing::info!("HTTP POST {} (body: {} bytes, timeout: {:?})", url, body.len(), timeout);

    // In real implementation, would use reqwest
    Ok(Value::Map({
        let mut m = std::collections::HashMap::new();
        m.insert("status".into(), Value::Int(200));
        m.insert("body".into(), Value::String("OK".into()));
        m.insert("headers".into(), Value::Map(std::collections::HashMap::new()));
        m
    }))
}

/// HTTP request options.
#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    /// Request headers.
    pub headers: std::collections::HashMap<String, String>,
    /// Request timeout.
    pub timeout: Option<Duration>,
    /// Follow redirects.
    pub follow_redirects: bool,
    /// Maximum redirects to follow.
    pub max_redirects: usize,
}

impl RequestOptions {
    /// Create new request options.
    pub fn new() -> Self {
        Self {
            headers: std::collections::HashMap::new(),
            timeout: Some(Duration::from_secs(30)),
            follow_redirects: true,
            max_redirects: 10,
        }
    }

    /// Add a header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set follow redirects.
    pub fn with_follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }
}

/// HTTP response.
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP status code.
    pub status: u16,
    /// Response headers.
    pub headers: std::collections::HashMap<String, String>,
    /// Response body.
    pub body: String,
}

impl Response {
    /// Check if the response was successful (2xx status).
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Convert to a Value.
    pub fn to_value(&self) -> Value {
        let mut m = std::collections::HashMap::new();
        m.insert("status".into(), Value::Int(self.status as i64));
        m.insert("body".into(), Value::String(self.body.clone()));

        let headers: std::collections::HashMap<String, Value> = self
            .headers
            .iter()
            .map(|(k, v)| (k.clone(), Value::String(v.clone())))
            .collect();
        m.insert("headers".into(), Value::Map(headers));

        Value::Map(m)
    }
}

// Helper function to extract host from URL
fn extract_host(url: &str) -> fusabi_host::Result<String> {
    // Simple URL parsing
    let url = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    let host = url
        .split('/')
        .next()
        .unwrap_or(url)
        .split(':')
        .next()
        .unwrap_or(url);

    if host.is_empty() {
        Err(fusabi_host::Error::host_function("invalid URL: no host"))
    } else {
        Ok(host.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_host::Capabilities;
    use fusabi_host::{Sandbox, SandboxConfig};
    use fusabi_host::Limits;
    use crate::safety::HostAllowlist;

    fn create_test_ctx() -> ExecutionContext {
        let sandbox = Sandbox::new(SandboxConfig::default()).unwrap();
        ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox)
    }

    #[test]
    fn test_extract_host() {
        assert_eq!(extract_host("https://example.com/path").unwrap(), "example.com");
        assert_eq!(extract_host("http://api.test.com:8080/").unwrap(), "api.test.com");
        assert_eq!(extract_host("example.com").unwrap(), "example.com");
    }

    #[test]
    fn test_get_safety_check() {
        let safety = Arc::new(SafetyConfig::strict());
        let ctx = create_test_ctx();

        let result = http_get(
            &safety,
            None,
            &[Value::String("https://example.com".into())],
            &ctx,
        );
        assert!(result.is_err()); // Should fail - host not allowed
    }

    #[test]
    fn test_get_with_permission() {
        let safety = Arc::new(
            SafetyConfig::new()
                .with_hosts(HostAllowlist::none().allow("example.com"))
        );
        let ctx = create_test_ctx();

        let result = http_get(
            &safety,
            None,
            &[Value::String("https://example.com/api".into())],
            &ctx,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_request_options() {
        let opts = RequestOptions::new()
            .with_header("Content-Type", "application/json")
            .with_timeout(Duration::from_secs(10))
            .with_follow_redirects(false);

        assert_eq!(opts.headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(opts.timeout, Some(Duration::from_secs(10)));
        assert!(!opts.follow_redirects);
    }

    #[test]
    fn test_response() {
        let response = Response {
            status: 200,
            headers: std::collections::HashMap::new(),
            body: "OK".into(),
        };

        assert!(response.is_success());

        let value = response.to_value();
        let map = value.as_map().unwrap();
        assert_eq!(map.get("status"), Some(&Value::Int(200)));
    }
}
