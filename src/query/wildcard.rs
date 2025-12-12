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
    /// The boost value
    boost: Option<f64>,
}

impl<'a> WildcardQuery<'a> {
    /// Create a new WildcardQuery with a given field, value, and case_insensitive flag
    pub fn new(
        field: impl Into<Cow<'a, str>>,
        value: impl Into<Cow<'a, str>>,
        case_insensitive: bool,
        boost: Option<f64>,
    ) -> Self {
        Self {
            field: field.into(),
            value: value.into(),
            case_insensitive,
            boost,
        }
    }

    /// Set the boost
    pub fn boost(mut self, boost: f64) -> Self {
        self.boost = Some(boost);
        self
    }

    /// Convert to an owned version with 'static lifetime
    pub fn to_owned(&self) -> WildcardQuery<'static> {
        WildcardQuery {
            field: Cow::Owned(self.field.to_string()),
            value: Cow::Owned(self.value.to_string()),
            case_insensitive: self.case_insensitive,
            boost: self.boost,
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
                    "case_insensitive": self.case_insensitive,
                    "boost": self.boost,
                }
            }
        })
    }
}
