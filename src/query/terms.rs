use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::ToOpenSearchJson;

/// Terms Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsQuery {
    /// The field to search
    pub field: String,
    /// The values to search for
    pub values: Vec<Value>,
    /// The boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

impl TermsQuery {
    /// Create a new TermsQuery with a given field and values
    pub fn new<T: Into<Value>>(field: &str, values: Vec<T>) -> Self {
        Self {
            field: field.to_string(),
            values: values.into_iter().map(|v| v.into()).collect(),
            boost: None,
        }
    }

    /// Set the boost value
    pub fn boost(mut self, boost: f64) -> Self {
        self.boost = Some(boost);
        self
    }
}

impl ToOpenSearchJson for TermsQuery {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let mut terms_obj = Map::new();

        if self.boost.is_some() {
            // Complex form with boost
            let mut field_obj = Map::new();
            field_obj.insert("terms".to_string(), Value::Array(self.values.clone()));
            if let Some(boost) = self.boost {
                field_obj.insert("boost".to_string(), boost.into());
            }
            terms_obj.insert(self.field.clone(), Value::Object(field_obj));
        } else {
            // Simple form: field: [values]
            terms_obj.insert(self.field.clone(), Value::Array(self.values.clone()));
        }

        result.insert("terms".to_string(), Value::Object(terms_obj));
        Value::Object(result)
    }
}
