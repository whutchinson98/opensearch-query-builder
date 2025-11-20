use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{QueryType, ToOpenSearchJson};

/// Wildcard Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildcardQuery<'a> {
    /// The field to search
    #[serde(borrow)]
    field: Cow<'a, str>,
    /// The value to search for
    /// **NOTE**: You'll need to wrap the value in `*` yourself
    #[serde(borrow)]
    value: Cow<'a, str>,
    /// Whether to perform a case-insensitive search
    case_insensitive: bool,
}

impl<'a> WildcardQuery<'a> {
    /// Create a new WildcardQuery with a given field, value, and case_insensitive flag
    pub fn new(field: Cow<'a, str>, value: Cow<'a, str>, case_insensitive: bool) -> Self {
        Self {
            field,
            value,
            case_insensitive,
        }
    }
}

impl<'a> From<WildcardQuery<'a>> for QueryType<'a> {
    fn from(wildcard_query: WildcardQuery<'a>) -> Self {
        QueryType::WildCard(wildcard_query)
    }
}

impl<'a> ToOpenSearchJson for WildcardQuery<'a> {
    fn to_json(&self) -> Value {
        serde_json::json!({
            "wildcard": {
                self.field.as_ref(): {
                    "value": self.value.as_ref(),
                    "case_insensitive": self.case_insensitive
                }
            }
        })
    }
}
