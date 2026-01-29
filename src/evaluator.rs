use crate::config::FeatureSet;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

pub fn build_context(features: &FeatureSet) -> Result<HashMap<String, String>> {
    let mut context = HashMap::new();

    for (key, val) in features {
        let value_str = match val {
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            _ => continue,
        };
        context.insert(key.to_uppercase(), value_str);
    }

    Ok(context)
}

pub fn evaluate_condition(condition: &str, context: &HashMap<String, String>) -> Result<bool> {
    let cond = condition.trim();

    if let Some(rest) = cond.strip_prefix('$') {
        let parts: Vec<&str> = rest.split("==").collect();

        if parts.len() == 2 {
            let var_name = parts[0].trim();

            let expected_val = parts[1]
                .trim()
                .trim_matches(|c| c == '\'' || c == '"')
                .to_lowercase();

            if let Some(actual_val) = context.get(var_name.to_uppercase().as_str()) {
                return Ok(*actual_val == expected_val);
            } else {
                return Ok(false);
            }
        }
    }

    Ok(false)
}
