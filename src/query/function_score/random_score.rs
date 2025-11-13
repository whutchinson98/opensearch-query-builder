use serde::Serialize;
use serde_json::Value;

/// Random score configuration
#[derive(Debug, Clone, Serialize, Default)]
pub struct RandomScore {
    /// The seed to use for randomizing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<Value>,
    /// The field to use for randomizing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl RandomScore {
    /// Create a new empty RandomScore
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the seed
    pub fn seed<T: Into<Value>>(mut self, seed: T) -> Self {
        self.seed = Some(seed.into());
        self
    }

    /// Set the field
    pub fn field(mut self, field: &str) -> Self {
        self.field = Some(field.to_string());
        self
    }
}
