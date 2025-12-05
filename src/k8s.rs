//! Kubernetes API bindings for Fusabi.
//!
//! Provides access to Kubernetes resources and operations.

use k8s_openapi::api::core::v1::{ConfigMap, Namespace, Pod, Secret, Service};
use kube::{
    api::{Api, ListParams, PostParams},
    Client, Config,
};
use std::collections::{BTreeMap, HashMap};

use crate::error::{Error, Result};
use fusabi_host::Value;

/// Kubernetes client wrapper for Fusabi.
pub struct K8sClient {
    client: Client,
    namespace: String,
}

impl K8sClient {
    /// Create a new K8s client with in-cluster configuration.
    pub async fn new_in_cluster() -> Result<Self> {
        let config = Config::incluster()
            .map_err(|e| Error::K8s(format!("in-cluster config failed: {}", e)))?;
        let client = Client::try_from(config)
            .map_err(|e| Error::K8s(format!("client creation failed: {}", e)))?;

        Ok(Self {
            client,
            namespace: "default".to_string(),
        })
    }

    /// Create a new K8s client from kubeconfig.
    pub async fn from_kubeconfig() -> Result<Self> {
        let config = Config::infer()
            .await
            .map_err(|e| Error::K8s(format!("kubeconfig inference failed: {}", e)))?;
        let client = Client::try_from(config)
            .map_err(|e| Error::K8s(format!("client creation failed: {}", e)))?;

        Ok(Self {
            client,
            namespace: "default".to_string(),
        })
    }

    /// Set the default namespace for operations.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = namespace.into();
        self
    }

    /// Get the current namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// List pods in the current namespace.
    pub async fn list_pods(&self, label_selector: Option<&str>) -> Result<Vec<PodInfo>> {
        let api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
        let mut lp = ListParams::default();
        if let Some(selector) = label_selector {
            lp = lp.labels(selector);
        }

        let pods = api
            .list(&lp)
            .await
            .map_err(|e| Error::K8s(format!("list pods failed: {}", e)))?;

        Ok(pods
            .items
            .into_iter()
            .map(|pod| PodInfo::from_pod(&pod))
            .collect())
    }

    /// Get a config map.
    pub async fn get_configmap(&self, name: &str) -> Result<HashMap<String, String>> {
        let api: Api<ConfigMap> = Api::namespaced(self.client.clone(), &self.namespace);
        let cm = api
            .get(name)
            .await
            .map_err(|e| Error::K8s(format!("get configmap failed: {}", e)))?;

        let btree = cm.data.unwrap_or_default();
        Ok(btree.into_iter().collect())
    }

    /// Get a secret (decoded).
    pub async fn get_secret(&self, name: &str) -> Result<HashMap<String, String>> {
        let api: Api<Secret> = Api::namespaced(self.client.clone(), &self.namespace);
        let secret = api
            .get(name)
            .await
            .map_err(|e| Error::K8s(format!("get secret failed: {}", e)))?;

        let mut result = HashMap::new();
        if let Some(data) = secret.data {
            for (key, value) in data {
                if let Ok(decoded) = String::from_utf8(value.0) {
                    result.insert(key, decoded);
                }
            }
        }

        Ok(result)
    }

    /// List namespaces.
    pub async fn list_namespaces(&self) -> Result<Vec<String>> {
        let api: Api<Namespace> = Api::all(self.client.clone());
        let namespaces = api
            .list(&ListParams::default())
            .await
            .map_err(|e| Error::K8s(format!("list namespaces failed: {}", e)))?;

        Ok(namespaces
            .items
            .into_iter()
            .filter_map(|ns| ns.metadata.name)
            .collect())
    }
}

/// Simplified pod information.
#[derive(Debug, Clone)]
pub struct PodInfo {
    /// Pod name.
    pub name: String,
    /// Pod namespace.
    pub namespace: String,
    /// Pod phase (Pending, Running, Succeeded, Failed, Unknown).
    pub phase: String,
    /// Pod IP address.
    pub pod_ip: Option<String>,
    /// Node the pod is running on.
    pub node_name: Option<String>,
    /// Pod labels.
    pub labels: HashMap<String, String>,
}

impl PodInfo {
    /// Create pod info from a Pod resource.
    fn from_pod(pod: &Pod) -> Self {
        let metadata = &pod.metadata;
        let status = pod.status.as_ref();

        Self {
            name: metadata.name.clone().unwrap_or_default(),
            namespace: metadata.namespace.clone().unwrap_or_default(),
            phase: status
                .and_then(|s| s.phase.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
            pod_ip: status.and_then(|s| s.pod_ip.clone()),
            node_name: pod.spec.as_ref().and_then(|s| s.node_name.clone()),
            labels: metadata.labels.clone().unwrap_or_default().into_iter().collect(),
        }
    }

    /// Convert to Fusabi Value.
    pub fn to_value(&self) -> Value {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::String(self.name.clone()));
        map.insert("namespace".to_string(), Value::String(self.namespace.clone()));
        map.insert("phase".to_string(), Value::String(self.phase.clone()));

        if let Some(ref ip) = self.pod_ip {
            map.insert("pod_ip".to_string(), Value::String(ip.clone()));
        }

        if let Some(ref node) = self.node_name {
            map.insert("node_name".to_string(), Value::String(node.clone()));
        }

        let labels: HashMap<String, Value> = self
            .labels
            .iter()
            .map(|(k, v)| (k.clone(), Value::String(v.clone())))
            .collect();
        map.insert("labels".to_string(), Value::Map(labels));

        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_info_to_value() {
        let info = PodInfo {
            name: "test-pod".to_string(),
            namespace: "default".to_string(),
            phase: "Running".to_string(),
            pod_ip: Some("10.0.0.1".to_string()),
            node_name: Some("node-1".to_string()),
            labels: HashMap::from([("app".to_string(), "test".to_string())]),
        };

        let value = info.to_value();
        if let Value::Map(map) = value {
            assert_eq!(map.get("name"), Some(&Value::String("test-pod".to_string())));
            assert_eq!(map.get("phase"), Some(&Value::String("Running".to_string())));
        } else {
            panic!("Expected Map value");
        }
    }
}
