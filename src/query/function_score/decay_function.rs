use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Decay function configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayFunction {
    /// The field to use for decaying
    pub field: String,
    /// The origin value to use for decaying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Value>,
    /// The scale to use for decaying
    pub scale: String,
    /// The offset to use for decaying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// The decay to use for decaying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decay: Option<f64>,
}

impl DecayFunction {
    /// Create a new DecayFunction
    pub fn new(field: &str, scale: &str) -> Self {
        Self {
            field: field.to_string(),
            origin: None,
            scale: scale.to_string(),
            offset: None,
            decay: None,
        }
    }

    /// Set the origin value
    pub fn origin<T: Into<Value>>(mut self, origin: T) -> Self {
        self.origin = Some(origin.into());
        self
    }

    /// Set the offset
    pub fn offset(mut self, offset: &str) -> Self {
        self.offset = Some(offset.to_string());
        self
    }

    /// Set the decay
    pub fn decay(mut self, decay: f64) -> Self {
        self.decay = Some(decay);
        self
    }
}
