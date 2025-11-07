use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{QueryType, ToOpenSearchJson};

/// Match Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchQuery {
    /// The field to search
    pub field: String,
    /// The query string
    pub query: String,
    /// The operator to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// The fuzziness value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<String>,
    /// The boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// The minimum should match value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<String>,
}

impl MatchQuery {
    /// Create a new MatchQuery with a given field and query string
    pub fn new(field: &str, query: &str) -> Self {
        Self {
            field: field.to_string(),
            query: query.to_string(),
            operator: None,
            fuzziness: None,
            boost: None,
            minimum_should_match: None,
        }
    }

    /// Set the operator to use
    pub fn operator(mut self, operator: &str) -> Self {
        self.operator = Some(operator.to_string());
        self
    }

    /// Set the fuzziness value
    pub fn fuzziness(mut self, fuzziness: &str) -> Self {
        self.fuzziness = Some(fuzziness.to_string());
        self
    }

    /// Set the boost value
    pub fn boost(mut self, boost: f64) -> Self {
        self.boost = Some(boost);
        self
    }

    /// Set the minimum should match value
    pub fn minimum_should_match(mut self, minimum_should_match: &str) -> Self {
        self.minimum_should_match = Some(minimum_should_match.to_string());
        self
    }
}

impl From<MatchQuery> for QueryType {
    fn from(match_query: MatchQuery) -> Self {
        QueryType::Match(match_query)
    }
}

impl ToOpenSearchJson for MatchQuery {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let mut match_obj = Map::new();

        // Check if we need the complex form
        let has_options = self.operator.is_some()
            || self.fuzziness.is_some()
            || self.boost.is_some()
            || self.minimum_should_match.is_some();

        if has_options {
            let mut field_obj = Map::new();
            field_obj.insert("query".to_string(), Value::String(self.query.clone()));

            if let Some(ref operator) = self.operator {
                field_obj.insert("operator".to_string(), Value::String(operator.clone()));
            }
            if let Some(ref fuzziness) = self.fuzziness {
                field_obj.insert("fuzziness".to_string(), Value::String(fuzziness.clone()));
            }
            if let Some(boost) = self.boost {
                field_obj.insert("boost".to_string(), boost.into());
            }

            if let Some(ref minimum_should_match) = self.minimum_should_match {
                field_obj.insert(
                    "minimum_should_match".to_string(),
                    Value::String(minimum_should_match.clone()),
                );
            }

            match_obj.insert(self.field.clone(), Value::Object(field_obj));
        } else {
            // Simple form: field: "query"
            match_obj.insert(self.field.clone(), Value::String(self.query.clone()));
        }

        result.insert("match".to_string(), Value::Object(match_obj));
        Value::Object(result)
    }
}
