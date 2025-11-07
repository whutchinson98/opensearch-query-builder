use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::ToOpenSearchJson;

/// Collapse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collapse {
    /// The field to collapse on
    pub field: String,
}

impl Collapse {
    /// Create a new Collapse
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
        }
    }
}

impl ToOpenSearchJson for Collapse {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        result.insert("field".to_string(), Value::String(self.field.clone()));
        Value::Object(result)
    }
}
