use std::borrow::Cow;

use serde::Serialize;

/// Field value factor configuration
#[derive(Debug, Clone, Serialize)]
pub struct FieldValueFactor<'a> {
    /// The field to use for factoring
    #[serde(borrow)]
    pub field: Cow<'a, str>,
    /// The factor to use for factoring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    /// The modifier to use for factoring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier: Option<Cow<'a, str>>,
    /// The missing value to use for factoring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,
}

impl<'a> FieldValueFactor<'a> {
    /// Create a new FieldValueFactor
    pub fn new(field: &'a str) -> Self {
        Self {
            field: Cow::Borrowed(field),
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
    pub fn modifier(mut self, modifier: &'a str) -> Self {
        self.modifier = Some(Cow::Borrowed(modifier));
        self
    }

    /// Set the missing value
    pub fn missing(mut self, missing: f64) -> Self {
        self.missing = Some(missing);
        self
    }
}
