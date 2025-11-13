use std::borrow::Cow;

use serde::Serialize;
use serde_json::{Map, Value};

use crate::{QueryType, ToOpenSearchJson};

/// Match Query
#[derive(Debug, Clone, Serialize)]
pub struct MatchQuery<'a> {
    /// The field to search
    #[serde(borrow)]
    pub field: Cow<'a, str>,
    /// The query string
    #[serde(borrow)]
    pub query: Cow<'a, str>,
    /// The operator to use
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub operator: Option<Cow<'a, str>>,
    /// The fuzziness value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub fuzziness: Option<Cow<'a, str>>,
    /// The boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// The minimum should match value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub minimum_should_match: Option<Cow<'a, str>>,
}

impl<'a> MatchQuery<'a> {
    /// Create a new MatchQuery with a given field and query string
    pub fn new(field: &'a str, query: &'a str) -> Self {
        Self {
            field: Cow::Borrowed(field),
            query: Cow::Borrowed(query),
            operator: None,
            fuzziness: None,
            boost: None,
            minimum_should_match: None,
        }
    }

    /// Set the operator to use
    pub fn operator(mut self, operator: &'a str) -> Self {
        self.operator = Some(Cow::Borrowed(operator));
        self
    }

    /// Set the fuzziness value
    pub fn fuzziness(mut self, fuzziness: &'a str) -> Self {
        self.fuzziness = Some(Cow::Borrowed(fuzziness));
        self
    }

    /// Set the boost value
    pub fn boost(mut self, boost: f64) -> Self {
        self.boost = Some(boost);
        self
    }

    /// Set the minimum should match value
    pub fn minimum_should_match(mut self, minimum_should_match: &'a str) -> Self {
        self.minimum_should_match = Some(Cow::Borrowed(minimum_should_match));
        self
    }
}

impl<'a> From<MatchQuery<'a>> for QueryType<'a> {
    fn from(match_query: MatchQuery<'a>) -> Self {
        QueryType::Match(match_query)
    }
}

impl<'a> ToOpenSearchJson for MatchQuery<'a> {
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
            field_obj.insert("query".to_string(), Value::String(self.query.to_string()));

            if let Some(ref operator) = self.operator {
                field_obj.insert("operator".to_string(), Value::String(operator.to_string()));
            }
            if let Some(ref fuzziness) = self.fuzziness {
                field_obj.insert(
                    "fuzziness".to_string(),
                    Value::String(fuzziness.to_string()),
                );
            }
            if let Some(boost) = self.boost {
                field_obj.insert("boost".to_string(), boost.into());
            }

            if let Some(ref minimum_should_match) = self.minimum_should_match {
                field_obj.insert(
                    "minimum_should_match".to_string(),
                    Value::String(minimum_should_match.to_string()),
                );
            }

            match_obj.insert(self.field.to_string(), Value::Object(field_obj));
        } else {
            // Simple form: field: "query"
            match_obj.insert(
                self.field.to_string(),
                Value::String(self.query.to_string()),
            );
        }

        result.insert("match".to_string(), Value::Object(match_obj));
        Value::Object(result)
    }
}
