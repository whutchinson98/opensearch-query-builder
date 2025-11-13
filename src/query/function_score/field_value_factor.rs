use serde::Serialize;

/// Field value factor configuration
#[derive(Debug, Clone, Serialize)]
pub struct FieldValueFactor {
    /// The field to use for factoring
    pub field: String,
    /// The factor to use for factoring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    /// The modifier to use for factoring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier: Option<String>,
    /// The missing value to use for factoring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,
}

impl FieldValueFactor {
    /// Create a new FieldValueFactor
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
            factor: None,
            modifier: None,
            missing: None,
        }
    }

    /// Set the factor
    pub fn factor(mut self, factor: f64) -> Self {
        self.factor = Some(factor);
        self
    }

    /// Set the modifier
    pub fn modifier(mut self, modifier: &str) -> Self {
        self.modifier = Some(modifier.to_string());
        self
    }

    /// Set the missing value
    pub fn missing(mut self, missing: f64) -> Self {
        self.missing = Some(missing);
        self
    }
}
