use std::borrow::Cow;

use serde::Serialize;
use serde_json::Value;

/// Decay function configuration
#[derive(Debug, Clone, Serialize)]
pub struct DecayFunction<'a> {
    /// The field to use for decaying
    #[serde(borrow)]
    pub field: Cow<'a, str>,
    /// The origin value to use for decaying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Value>,
    /// The scale to use for decaying
    #[serde(borrow)]
    pub scale: Cow<'a, str>,
    /// The offset to use for decaying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Cow<'a, str>>,
    /// The decay to use for decaying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decay: Option<f64>,
}

impl<'a> DecayFunction<'a> {
    /// Create a new DecayFunction
    pub fn new(field: &'a str, scale: &'a str) -> Self {
        Self {
            field: Cow::Borrowed(field),
            origin: None,
            scale: Cow::Borrowed(scale),
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
    pub fn offset(mut self, offset: &'a str) -> Self {
        self.offset = Some(Cow::Borrowed(offset));
        self
    }

    /// Set the decay
    pub fn decay(mut self, decay: f64) -> Self {
        self.decay = Some(decay);
        self
    }
}
