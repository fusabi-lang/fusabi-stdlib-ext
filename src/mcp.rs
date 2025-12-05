//! MCP (Model Context Protocol) helpers for Fusabi.
//!
//! Provides utilities for building MCP servers and clients.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::error::{Error, Result};
use fusabi_host::Value;

/// MCP protocol version.
pub const PROTOCOL_VERSION: &str = "2024-11-05";

/// MCP message types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum McpMessage {
    /// Initialize request.
    #[serde(rename = "initialize")]
    Initialize(InitializeParams),

    /// List tools request.
    #[serde(rename = "tools/list")]
    ListTools,

    /// Call tool request.
    #[serde(rename = "tools/call")]
    CallTool(CallToolParams),

    /// List resources request.
    #[serde(rename = "resources/list")]
    ListResources,

    /// Read resource request.
    #[serde(rename = "resources/read")]
    ReadResource(ReadResourceParams),

    /// List prompts request.
    #[serde(rename = "prompts/list")]
    ListPrompts,

    /// Get prompt request.
    #[serde(rename = "prompts/get")]
    GetPrompt(GetPromptParams),
}

/// Initialize request parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    /// Protocol version.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Capabilities supported by the client.
    pub capabilities: ClientCapabilities,
    /// Client info.
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
}

/// Client capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientCapabilities {
    /// Whether client supports sampling.
    #[serde(default)]
    pub sampling: Option<JsonValue>,
    /// Whether client supports roots.
    #[serde(default)]
    pub roots: Option<JsonValue>,
}

/// Client info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Client name.
    pub name: String,
    /// Client version.
    pub version: String,
}

/// Server capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Tool capabilities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolCapabilities>,
    /// Resource capabilities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceCapabilities>,
    /// Prompt capabilities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptCapabilities>,
}

/// Tool capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolCapabilities {
    /// Whether tools list can change.
    #[serde(default, rename = "listChanged")]
    pub list_changed: bool,
}

/// Resource capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceCapabilities {
    /// Whether resources can subscribe.
    #[serde(default)]
    pub subscribe: bool,
    /// Whether resource list can change.
    #[serde(default, rename = "listChanged")]
    pub list_changed: bool,
}

/// Prompt capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PromptCapabilities {
    /// Whether prompts list can change.
    #[serde(default, rename = "listChanged")]
    pub list_changed: bool,
}

/// Server info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Server name.
    pub name: String,
    /// Server version.
    pub version: String,
}

/// Call tool parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolParams {
    /// Tool name.
    pub name: String,
    /// Tool arguments.
    #[serde(default)]
    pub arguments: HashMap<String, JsonValue>,
}

/// Read resource parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceParams {
    /// Resource URI.
    pub uri: String,
}

/// Get prompt parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPromptParams {
    /// Prompt name.
    pub name: String,
    /// Prompt arguments.
    #[serde(default)]
    pub arguments: HashMap<String, String>,
}

/// MCP tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name.
    pub name: String,
    /// Tool description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema for input.
    #[serde(rename = "inputSchema")]
    pub input_schema: JsonValue,
}

/// MCP resource definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    /// Resource URI.
    pub uri: String,
    /// Resource name.
    pub name: String,
    /// Resource description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type.
    #[serde(default, rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// MCP prompt definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptDefinition {
    /// Prompt name.
    pub name: String,
    /// Prompt description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Prompt arguments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<PromptArgument>,
}

/// Prompt argument definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    /// Argument name.
    pub name: String,
    /// Argument description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the argument is required.
    #[serde(default)]
    pub required: bool,
}

/// Convert Fusabi Value to JSON Value.
pub fn fusabi_to_json(value: &Value) -> JsonValue {
    match value {
        Value::Null => JsonValue::Null,
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Int(n) => JsonValue::Number((*n).into()),
        Value::Float(f) => {
            if let Some(n) = serde_json::Number::from_f64(*f) {
                JsonValue::Number(n)
            } else {
                JsonValue::Null
            }
        }
        Value::String(s) => JsonValue::String(s.clone()),
        Value::List(items) => JsonValue::Array(items.iter().map(fusabi_to_json).collect()),
        Value::Map(map) => {
            let obj: serde_json::Map<String, JsonValue> = map
                .iter()
                .map(|(k, v)| (k.clone(), fusabi_to_json(v)))
                .collect();
            JsonValue::Object(obj)
        }
        Value::Bytes(b) => {
            // Encode bytes as hex string
            let hex: String = b.iter().map(|byte| format!("{:02x}", byte)).collect();
            JsonValue::String(hex)
        }
        Value::Function(_) => JsonValue::String("<function>".to_string()),
        Value::Error(e) => JsonValue::Object({
            let mut obj = serde_json::Map::new();
            obj.insert("error".to_string(), JsonValue::String(e.clone()));
            obj
        }),
    }
}

/// Convert JSON Value to Fusabi Value.
pub fn json_to_fusabi(value: &JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::Null
            }
        }
        JsonValue::String(s) => Value::String(s.clone()),
        JsonValue::Array(items) => Value::List(items.iter().map(json_to_fusabi).collect()),
        JsonValue::Object(map) => {
            let converted: HashMap<String, Value> =
                map.iter().map(|(k, v)| (k.clone(), json_to_fusabi(v))).collect();
            Value::Map(converted)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fusabi_to_json_roundtrip() {
        let original = Value::Map(HashMap::from([
            ("name".to_string(), Value::String("test".to_string())),
            ("count".to_string(), Value::Int(42)),
            (
                "items".to_string(),
                Value::List(vec![Value::Int(1), Value::Int(2)]),
            ),
        ]));

        let json = fusabi_to_json(&original);
        let back = json_to_fusabi(&json);

        // Compare maps by checking individual keys (order doesn't matter)
        if let Value::Map(orig_map) = &original {
            if let Value::Map(back_map) = &back {
                assert_eq!(orig_map.len(), back_map.len());
                for (k, v) in orig_map {
                    assert_eq!(back_map.get(k).map(|v| format!("{:?}", v)), Some(format!("{:?}", v)));
                }
            } else {
                panic!("Expected Map value");
            }
        }
    }

    #[test]
    fn test_tool_definition_serialize() {
        let tool = ToolDefinition {
            name: "test-tool".to_string(),
            description: Some("A test tool".to_string()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "input": { "type": "string" }
                }
            }),
        };

        let json = serde_json::to_string(&tool).unwrap();
        assert!(json.contains("test-tool"));
    }
}
