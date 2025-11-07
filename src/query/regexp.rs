use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ToOpenSearchJson;

/// Enum representing the different flags that can be used with a RegexpQuery
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum RegexpQueryFlags {
    /// Enables all optional features (default behavior)
    All,
    /// Allows @ to match any string
    Anystring,
    /// Matches the complement of the language described by the regex
    Complement,
    /// Allows matching empty strings
    Empty,
    /// Enables intersection of multiple patterns
    Intersection,
    /// Enables interval arithmetic on character classes
    Interval,
    /// Disables all optional features (default behavior)
    #[default]
    None,
}

impl RegexpQueryFlags {
    /// Create a new RegexpQueryFlags with the All flag
    pub fn all() -> Vec<Self> {
        vec![Self::All]
    }
}

impl Display for RegexpQueryFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexpQueryFlags::All => write!(f, "ALL"),
            RegexpQueryFlags::Anystring => write!(f, "ANYSTRING"),
            RegexpQueryFlags::Complement => write!(f, "COMPLEMENT"),
            RegexpQueryFlags::Empty => write!(f, "EMPTY"),
            RegexpQueryFlags::Intersection => write!(f, "INTERSECTION"),
            RegexpQueryFlags::Interval => write!(f, "INTERVAL"),
            RegexpQueryFlags::None => write!(f, "NONE"),
        }
    }
}

/// Regexp Query
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegexpQuery {
    /// The field to search in
    pub field: String,
    /// The stringified regex pattern to match on
    pub value: String,
    /// The flags to use when matching the regular expression
    pub flags: Option<Vec<RegexpQueryFlags>>,
}

impl RegexpQuery {
    /// Create a new RegexpQuery with a given field and value
    pub fn new(field: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            value: value.to_string(),
            flags: None,
        }
    }

    /// Set the flags to use when matching the regular expression
    pub fn flags(mut self, flags: Vec<RegexpQueryFlags>) -> Self {
        self.flags = Some(flags);
        self
    }
}

impl ToOpenSearchJson for RegexpQuery {
    fn to_json(&self) -> Value {
        let mut json = serde_json::json!({
            "regexp": {
                self.field.clone(): {
                    "value": self.value,
                }
            }
        });

        if let Some(flags) = self.flags.as_ref()
            && !flags.is_empty()
        {
            // Join all flags with |
            json["regexp"][self.field.clone()]["flags"] = Value::String(
                flags
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("|"),
            );
        }

        json
    }
}
