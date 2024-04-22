use std::fmt::Display;

use serde_json::Value;

pub fn unwrap_json_string(val: &Value, name: impl Display) -> anyhow::Result<&String> {
    // pulls String out of Value
    // name is just for debugging/tracing, not strictly necessary for functionality
    if let Value::String(s) = val {
        Ok(s)
    } else {
        Err(anyhow::anyhow!(
            "{name} is not a string, but instead: {val:?}"
        ))
    }
}

pub fn unwrap_json_number(val: &Value, name: impl Display) -> anyhow::Result<f64> {
    // pulls f64 out of Value
    // name is just for debugging/tracing, not strictly necessary for functionality
    let Value::Number(num) = val else {
        return Err(anyhow::anyhow!(
            "{name} is not a number but instead a {val:?}."
        ));
    };
    let Some(num) = num.as_f64() else {
        return Err(anyhow::anyhow!("{name} {num:?} is not a valid f64."));
    };
    Ok(num)
}

pub fn unwrap_json_array(val: &Value, name: impl Display) -> anyhow::Result<&Vec<Value>> {
    // pulls Vec out of Value
    // name is just for debugging/tracing, not strictly necessary for functionality
    if let Value::Array(s) = val {
        Ok(s)
    } else {
        Err(anyhow::anyhow!(
            "{name} is not an array, but instead: {val:?}"
        ))
    }
}
