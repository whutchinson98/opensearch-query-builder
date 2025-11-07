use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::ToOpenSearchJson;

/// Highlight
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    /// Fields to highlight
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub fields: HashMap<String, HighlightField>,
    /// Require field match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_field_match: Option<bool>,
}

impl Highlight {
    /// Create a new Highlight
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a field to highlight
    pub fn field(mut self, field_name: &str, highlight_field: HighlightField) -> Self {
        self.fields.insert(field_name.to_string(), highlight_field);
        self
    }

    /// Set whether to require field match
    pub fn require_field_match(mut self, require_field_match: bool) -> Self {
        self.require_field_match = Some(require_field_match);
        self
    }
}

impl ToOpenSearchJson for Highlight {
    fn to_json(&self) -> Value {
        let mut result = Map::new();

        if !self.fields.is_empty() {
            let mut fields_obj = Map::new();
            for (name, field) in &self.fields {
                fields_obj.insert(name.clone(), field.to_json());
            }
            result.insert("fields".to_string(), Value::Object(fields_obj));
        }

        if let Some(require_field_match) = self.require_field_match {
            result.insert(
                "require_field_match".to_string(),
                Value::Bool(require_field_match),
            );
        }

        Value::Object(result)
    }
}

/// HighlightField
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightField {
    /// Highlight type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub highlight_type: Option<String>,
    /// Number of fragments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_fragments: Option<u32>,
    /// Pre-tags
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub pre_tags: Vec<String>,
    /// Post-tags
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub post_tags: Vec<String>,
}

impl Default for HighlightField {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightField {
    /// Create a new HighlightField
    pub fn new() -> Self {
        Self {
            highlight_type: None,
            number_of_fragments: None,
            pre_tags: Vec::new(),
            post_tags: Vec::new(),
        }
    }

    /// Set the highlight type
    pub fn highlight_type(mut self, highlight_type: &str) -> Self {
        self.highlight_type = Some(highlight_type.to_string());
        self
    }

    /// Set the number of fragments
    pub fn number_of_fragments(mut self, number_of_fragments: u32) -> Self {
        self.number_of_fragments = Some(number_of_fragments);
        self
    }

    /// Add a pre-tag
    pub fn pre_tags(mut self, pre_tags: Vec<String>) -> Self {
        self.pre_tags = pre_tags;
        self
    }

    /// Add a post-tag
    pub fn post_tags(mut self, post_tags: Vec<String>) -> Self {
        self.post_tags = post_tags;
        self
    }
}

impl ToOpenSearchJson for HighlightField {
    fn to_json(&self) -> Value {
        let mut result = Map::new();

        if let Some(ref highlight_type) = self.highlight_type {
            result.insert("type".to_string(), Value::String(highlight_type.clone()));
        }

        if let Some(number_of_fragments) = self.number_of_fragments {
            result.insert(
                "number_of_fragments".to_string(),
                Value::Number(number_of_fragments.into()),
            );
        }

        if !self.pre_tags.is_empty() {
            let pre_tags: Vec<Value> = self
                .pre_tags
                .iter()
                .map(|tag| Value::String(tag.clone()))
                .collect();
            result.insert("pre_tags".to_string(), Value::Array(pre_tags));
        }

        if !self.post_tags.is_empty() {
            let post_tags: Vec<Value> = self
                .post_tags
                .iter()
                .map(|tag| Value::String(tag.clone()))
                .collect();
            result.insert("post_tags".to_string(), Value::Array(post_tags));
        }

        Value::Object(result)
    }
}
