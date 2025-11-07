use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ToOpenSearchJson;

/// Wildcard Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildcardQuery {
    /// The field to search
    field: String,
    /// The value to search for
    /// **NOTE**: You'll need to wrap the value in `*` yourself
    value: String,
    /// Whether to perform a case-insensitive search
    case_insensitive: bool,
}

impl WildcardQuery {
    /// Create a new WildcardQuery with a given field, value, and case_insensitive flag
    pub fn new(field: &str, value: &str, case_insensitive: bool) -> Self {
        Self {
            field: field.to_string(),
            value: value.to_string(),
            case_insensitive,
        }
    }
}

impl ToOpenSearchJson for WildcardQuery {
    fn to_json(&self) -> Value {
        serde_json::json!({
            "wildcard": {
                self.field.clone(): {
                    "value": self.value,
                    "case_insensitive": self.case_insensitive
                }
            }
        })
    }
}
