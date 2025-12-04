//! Metrics module.
//!
//! Provides counter, gauge, and histogram primitives.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

use parking_lot::RwLock;

use fusabi_host::engine::ExecutionContext;
use fusabi_host::Value;

/// Global metrics registry.
static METRICS: once_cell::sync::Lazy<MetricsRegistry> =
    once_cell::sync::Lazy::new(MetricsRegistry::new);

/// Increment a counter.
pub fn counter_inc(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let name = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("metrics.counter_inc: missing name"))?;

    let value = args
        .get(1)
        .and_then(|v| v.as_int())
        .unwrap_or(1);

    METRICS.counter_inc(name, value as u64);
    Ok(Value::Null)
}

/// Set a gauge value.
pub fn gauge_set(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let name = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("metrics.gauge_set: missing name"))?;

    let value = args
        .get(1)
        .and_then(|v| v.as_float().or_else(|| v.as_int().map(|i| i as f64)))
        .ok_or_else(|| fusabi_host::Error::host_function("metrics.gauge_set: missing value"))?;

    METRICS.gauge_set(name, value);
    Ok(Value::Null)
}

/// Observe a histogram value.
pub fn histogram_observe(
    args: &[Value],
    _ctx: &ExecutionContext,
) -> fusabi_host::Result<Value> {
    let name = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| fusabi_host::Error::host_function("metrics.histogram_observe: missing name"))?;

    let value = args
        .get(1)
        .and_then(|v| v.as_float().or_else(|| v.as_int().map(|i| i as f64)))
        .ok_or_else(|| fusabi_host::Error::host_function("metrics.histogram_observe: missing value"))?;

    METRICS.histogram_observe(name, value);
    Ok(Value::Null)
}

/// A simple metrics registry.
pub struct MetricsRegistry {
    counters: RwLock<HashMap<String, AtomicU64>>,
    gauges: RwLock<HashMap<String, AtomicI64>>,
    histograms: RwLock<HashMap<String, Histogram>>,
}

impl MetricsRegistry {
    /// Create a new metrics registry.
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
        }
    }

    /// Increment a counter.
    pub fn counter_inc(&self, name: &str, value: u64) {
        let counters = self.counters.read();
        if let Some(counter) = counters.get(name) {
            counter.fetch_add(value, Ordering::Relaxed);
        } else {
            drop(counters);
            let mut counters = self.counters.write();
            counters
                .entry(name.to_string())
                .or_insert_with(|| AtomicU64::new(0))
                .fetch_add(value, Ordering::Relaxed);
        }
    }

    /// Get a counter value.
    pub fn counter_get(&self, name: &str) -> u64 {
        self.counters
            .read()
            .get(name)
            .map(|c| c.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    /// Set a gauge value.
    pub fn gauge_set(&self, name: &str, value: f64) {
        let bits = value.to_bits() as i64;
        let gauges = self.gauges.read();
        if let Some(gauge) = gauges.get(name) {
            gauge.store(bits, Ordering::Relaxed);
        } else {
            drop(gauges);
            let mut gauges = self.gauges.write();
            gauges
                .entry(name.to_string())
                .or_insert_with(|| AtomicI64::new(0))
                .store(bits, Ordering::Relaxed);
        }
    }

    /// Get a gauge value.
    pub fn gauge_get(&self, name: &str) -> f64 {
        self.gauges
            .read()
            .get(name)
            .map(|g| f64::from_bits(g.load(Ordering::Relaxed) as u64))
            .unwrap_or(0.0)
    }

    /// Observe a histogram value.
    pub fn histogram_observe(&self, name: &str, value: f64) {
        let histograms = self.histograms.read();
        if let Some(histogram) = histograms.get(name) {
            histogram.observe(value);
        } else {
            drop(histograms);
            let mut histograms = self.histograms.write();
            let histogram = histograms
                .entry(name.to_string())
                .or_insert_with(Histogram::new);
            histogram.observe(value);
        }
    }

    /// Get histogram statistics.
    pub fn histogram_stats(&self, name: &str) -> Option<HistogramStats> {
        self.histograms.read().get(name).map(|h| h.stats())
    }

    /// Get all metric names.
    pub fn names(&self) -> Vec<String> {
        let mut names = Vec::new();
        names.extend(self.counters.read().keys().cloned());
        names.extend(self.gauges.read().keys().cloned());
        names.extend(self.histograms.read().keys().cloned());
        names
    }

    /// Clear all metrics.
    pub fn clear(&self) {
        self.counters.write().clear();
        self.gauges.write().clear();
        self.histograms.write().clear();
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple histogram.
pub struct Histogram {
    values: RwLock<Vec<f64>>,
}

impl Histogram {
    /// Create a new histogram.
    pub fn new() -> Self {
        Self {
            values: RwLock::new(Vec::new()),
        }
    }

    /// Observe a value.
    pub fn observe(&self, value: f64) {
        self.values.write().push(value);
    }

    /// Get histogram statistics.
    pub fn stats(&self) -> HistogramStats {
        let values = self.values.read();

        if values.is_empty() {
            return HistogramStats::default();
        }

        let count = values.len() as u64;
        let sum: f64 = values.iter().sum();
        let mean = sum / count as f64;

        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Calculate percentiles (simple approach)
        let mut sorted: Vec<f64> = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let p50 = percentile(&sorted, 0.50);
        let p90 = percentile(&sorted, 0.90);
        let p99 = percentile(&sorted, 0.99);

        HistogramStats {
            count,
            sum,
            mean,
            min,
            max,
            p50,
            p90,
            p99,
        }
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }

    let index = (p * (sorted.len() - 1) as f64).round() as usize;
    sorted[index.min(sorted.len() - 1)]
}

/// Histogram statistics.
#[derive(Debug, Clone, Default)]
pub struct HistogramStats {
    /// Number of observations.
    pub count: u64,
    /// Sum of all observations.
    pub sum: f64,
    /// Mean value.
    pub mean: f64,
    /// Minimum value.
    pub min: f64,
    /// Maximum value.
    pub max: f64,
    /// 50th percentile (median).
    pub p50: f64,
    /// 90th percentile.
    pub p90: f64,
    /// 99th percentile.
    pub p99: f64,
}

// Lazy static for METRICS registry
mod once_cell {
    pub mod sync {
        pub struct Lazy<T> {
            cell: std::sync::OnceLock<T>,
            init: fn() -> T,
        }

        impl<T> Lazy<T> {
            pub const fn new(init: fn() -> T) -> Self {
                Self {
                    cell: std::sync::OnceLock::new(),
                    init,
                }
            }
        }

        impl<T> std::ops::Deref for Lazy<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.cell.get_or_init(self.init)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_host::Capabilities;
    use fusabi_host::sandbox::{Sandbox, SandboxConfig};
    use fusabi_host::Limits;

    fn create_test_ctx() -> ExecutionContext {
        let sandbox = Sandbox::new(SandboxConfig::default()).unwrap();
        ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox)
    }

    #[test]
    fn test_counter_inc() {
        let ctx = create_test_ctx();

        counter_inc(&[Value::String("test_counter".into())], &ctx).unwrap();
        counter_inc(&[Value::String("test_counter".into()), Value::Int(5)], &ctx).unwrap();

        let value = METRICS.counter_get("test_counter");
        assert!(value >= 6); // At least 1 + 5
    }

    #[test]
    fn test_gauge_set() {
        let ctx = create_test_ctx();

        gauge_set(&[Value::String("test_gauge".into()), Value::Float(42.5)], &ctx).unwrap();

        let value = METRICS.gauge_get("test_gauge");
        assert!((value - 42.5).abs() < 0.001);
    }

    #[test]
    fn test_histogram() {
        let ctx = create_test_ctx();

        for i in 1..=10 {
            histogram_observe(
                &[Value::String("test_histogram".into()), Value::Float(i as f64)],
                &ctx,
            ).unwrap();
        }

        let stats = METRICS.histogram_stats("test_histogram").unwrap();
        assert_eq!(stats.count, 10);
        assert!((stats.sum - 55.0).abs() < 0.001);
        assert!((stats.mean - 5.5).abs() < 0.001);
    }

    #[test]
    fn test_metrics_registry() {
        let registry = MetricsRegistry::new();

        registry.counter_inc("counter1", 1);
        registry.counter_inc("counter1", 2);
        assert_eq!(registry.counter_get("counter1"), 3);

        registry.gauge_set("gauge1", 100.0);
        assert!((registry.gauge_get("gauge1") - 100.0).abs() < 0.001);

        registry.histogram_observe("hist1", 1.0);
        registry.histogram_observe("hist1", 2.0);
        registry.histogram_observe("hist1", 3.0);

        let stats = registry.histogram_stats("hist1").unwrap();
        assert_eq!(stats.count, 3);
    }
}
