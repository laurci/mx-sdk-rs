pub use serde_json::Value as JsonValue;
use std::str::FromStr;

pub struct HumanReadableValue {
    value: JsonValue,
}

impl FromStr for HumanReadableValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = serde_json::from_str(s).map_err(|e| e.to_string())?;
        Ok(HumanReadableValue { value })
    }
}

impl HumanReadableValue {
    pub fn get_value(&self) -> &JsonValue {
        &self.value
    }

    pub fn child(&self, key: &str) -> Option<Self> {
        self.value.get(key).map(|value| HumanReadableValue {
            value: value.clone(),
        })
    }
}
