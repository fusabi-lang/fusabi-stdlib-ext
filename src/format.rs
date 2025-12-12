//! Format module.
//!
//! Provides string formatting and templating functions.

use fusabi_host::ExecutionContext;
use fusabi_host::Value;

/// Sprintf-style string formatting.
pub fn sprintf(args: &[Value], _ctx: &ExecutionContext) -> fusabi_host::Result<Value> {
    let format_str = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
        fusabi_host::Error::host_function("format.sprintf: missing format string")
    })?;

    let format_args = &args[1..];
    let result = format_string(format_str, format_args)?;

    Ok(Value::String(result))
}

/// Simple template string substitution.
pub fn template(args: &[Value], _ctx: &ExecutionContext) -> fusabi_host::Result<Value> {
    let template_str = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
        fusabi_host::Error::host_function("format.template: missing template string")
    })?;

    let values = args
        .get(1)
        .and_then(|v| v.as_map())
        .ok_or_else(|| fusabi_host::Error::host_function("format.template: missing values map"))?;

    let mut result = template_str.to_string();

    for (key, value) in values {
        let placeholder = format!("{{{{{}}}}}", key); // {{key}}
        let replacement = value_to_string(value);
        result = result.replace(&placeholder, &replacement);
    }

    Ok(Value::String(result))
}

/// Encode a value to JSON string.
pub fn json_encode(args: &[Value], _ctx: &ExecutionContext) -> fusabi_host::Result<Value> {
    let value = args
        .first()
        .ok_or_else(|| fusabi_host::Error::host_function("format.json_encode: missing value"))?;

    #[cfg(feature = "serde-support")]
    {
        let json = value.to_json_string();
        Ok(Value::String(json))
    }

    #[cfg(not(feature = "serde-support"))]
    {
        // Simple serialization without serde
        let json = value_to_json_simple(value);
        Ok(Value::String(json))
    }
}

/// Decode a JSON string to a value.
pub fn json_decode(args: &[Value], _ctx: &ExecutionContext) -> fusabi_host::Result<Value> {
    let json_str = args.first().and_then(|v| v.as_str()).ok_or_else(|| {
        fusabi_host::Error::host_function("format.json_decode: missing JSON string")
    })?;

    #[cfg(feature = "serde-support")]
    {
        Value::from_json_str(json_str)
            .map_err(|e| fusabi_host::Error::host_function(format!("format.json_decode: {}", e)))
    }

    #[cfg(not(feature = "serde-support"))]
    {
        // Simple parsing without serde (very limited)
        Err(fusabi_host::Error::host_function(
            "json_decode requires serde-support feature",
        ))
    }
}

// Helper functions

fn format_string(format_str: &str, args: &[Value]) -> fusabi_host::Result<String> {
    let mut result = String::new();
    let mut chars = format_str.chars().peekable();
    let mut arg_index = 0;

    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(&next) = chars.peek() {
                match next {
                    '%' => {
                        result.push('%');
                        chars.next();
                    }
                    's' => {
                        chars.next();
                        let arg = args.get(arg_index).ok_or_else(|| {
                            fusabi_host::Error::host_function(
                                "format.sprintf: not enough arguments",
                            )
                        })?;
                        result.push_str(&value_to_string(arg));
                        arg_index += 1;
                    }
                    'd' | 'i' => {
                        chars.next();
                        let arg = args.get(arg_index).ok_or_else(|| {
                            fusabi_host::Error::host_function(
                                "format.sprintf: not enough arguments",
                            )
                        })?;
                        if let Some(n) = arg.as_int() {
                            result.push_str(&n.to_string());
                        } else {
                            result.push_str(&value_to_string(arg));
                        }
                        arg_index += 1;
                    }
                    'f' => {
                        chars.next();
                        let arg = args.get(arg_index).ok_or_else(|| {
                            fusabi_host::Error::host_function(
                                "format.sprintf: not enough arguments",
                            )
                        })?;
                        if let Some(f) = arg.as_float() {
                            result.push_str(&f.to_string());
                        } else {
                            result.push_str(&value_to_string(arg));
                        }
                        arg_index += 1;
                    }
                    _ => {
                        result.push(c);
                    }
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::List(l) => {
            let items: Vec<String> = l.iter().map(value_to_string).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Map(m) => {
            let items: Vec<String> = m
                .iter()
                .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
        _ => format!("{}", value),
    }
}

fn value_to_json_simple(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Value::List(l) => {
            let items: Vec<String> = l.iter().map(value_to_json_simple).collect();
            format!("[{}]", items.join(","))
        }
        Value::Map(m) => {
            let items: Vec<String> = m
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", k, value_to_json_simple(v)))
                .collect();
            format!("{{{}}}", items.join(","))
        }
        _ => "null".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusabi_host::Capabilities;
    use fusabi_host::Limits;
    use fusabi_host::{Sandbox, SandboxConfig};

    fn create_test_ctx() -> ExecutionContext {
        let sandbox = Sandbox::new(SandboxConfig::default()).unwrap();
        ExecutionContext::new(1, Capabilities::none(), Limits::default(), sandbox)
    }

    #[test]
    fn test_sprintf() {
        let ctx = create_test_ctx();

        let result = sprintf(
            &[
                Value::String("Hello, %s! You have %d messages.".into()),
                Value::String("Alice".into()),
                Value::Int(5),
            ],
            &ctx,
        )
        .unwrap();

        assert_eq!(
            result.as_str().unwrap(),
            "Hello, Alice! You have 5 messages."
        );
    }

    #[test]
    fn test_sprintf_float() {
        let ctx = create_test_ctx();

        let result = sprintf(
            &[
                Value::String("Pi is approximately %f".into()),
                Value::Float(3.14159),
            ],
            &ctx,
        )
        .unwrap();

        assert!(result.as_str().unwrap().contains("3.14"));
    }

    #[test]
    fn test_template() {
        let ctx = create_test_ctx();

        let mut values = std::collections::HashMap::new();
        values.insert("name".to_string(), Value::String("Bob".into()));
        values.insert("count".to_string(), Value::Int(3));

        let result = template(
            &[
                Value::String("Hello, {{name}}! You have {{count}} items.".into()),
                Value::Map(values),
            ],
            &ctx,
        )
        .unwrap();

        assert_eq!(result.as_str().unwrap(), "Hello, Bob! You have 3 items.");
    }

    #[test]
    fn test_json_encode() {
        let ctx = create_test_ctx();

        let result = json_encode(&[Value::Int(42)], &ctx).unwrap();
        assert_eq!(result.as_str().unwrap(), "42");

        let result = json_encode(&[Value::String("hello".into())], &ctx).unwrap();
        assert!(result.as_str().unwrap().contains("hello"));
    }
}
