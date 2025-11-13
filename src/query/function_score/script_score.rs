use serde::Serialize;
use serde_json::{Map, Value};

/// Script score configuration
#[derive(Debug, Clone, Serialize)]
pub struct ScriptScore {
    /// The script to use for scoring
    pub source: String,
    /// The parameters to use for scoring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Map<String, Value>>,
}

impl ScriptScore {
    /// Create a new ScriptScore
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            params: None,
        }
    }

    /// Set the parameters
    pub fn params(mut self, params: Map<String, Value>) -> Self {
        self.params = Some(params);
        self
    }
}
