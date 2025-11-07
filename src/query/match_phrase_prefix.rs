use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{QueryType, ToOpenSearchJson};

/// Match Phrase Prefix Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPhrasePrefixQuery {
    /// The field to search
    pub field: String,
    /// The query string
    pub query: String,
    /// The maximum number of terms that can be expanded upon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expansions: Option<u32>,
    /// The slop value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slop: Option<u32>,
    /// The boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

impl MatchPhrasePrefixQuery {
    /// Create a new MatchPhrasePrefixQuery with a given field and query string
    pub fn new(field: &str, query: &str) -> Self {
        Self {
            field: field.to_string(),
            query: query.to_string(),
            max_expansions: None,
            slop: None,
            boost: None,
        }
    }

    /// Set the maximum number of terms that can be expanded upon
    pub fn max_expansions(mut self, max_expansions: u32) -> Self {
        self.max_expansions = Some(max_expansions);
        self
    }

    /// Set the slop value
    pub fn slop(mut self, slop: u32) -> Self {
        self.slop = Some(slop);
        self
    }

    /// Set the boost value
    pub fn boost(mut self, boost: f64) -> Self {
        self.boost = Some(boost);
        self
    }
}

impl From<MatchPhrasePrefixQuery> for QueryType {
    fn from(match_phrase_prefix_query: MatchPhrasePrefixQuery) -> Self {
        QueryType::MatchPhrasePrefix(match_phrase_prefix_query)
    }
}

impl ToOpenSearchJson for MatchPhrasePrefixQuery {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let mut match_phrase_prefix_obj = Map::new();
        let mut field_obj = Map::new();

        // Match phrase prefix always uses the complex form with "query" field
        field_obj.insert("query".to_string(), Value::String(self.query.clone()));

        if let Some(max_expansions) = self.max_expansions {
            field_obj.insert(
                "max_expansions".to_string(),
                Value::Number(max_expansions.into()),
            );
        }
        if let Some(slop) = self.slop {
            field_obj.insert("slop".to_string(), Value::Number(slop.into()));
        }
        if let Some(boost) = self.boost {
            field_obj.insert("boost".to_string(), boost.into());
        }

        match_phrase_prefix_obj.insert(self.field.clone(), Value::Object(field_obj));

        result.insert(
            "match_phrase_prefix".to_string(),
            Value::Object(match_phrase_prefix_obj),
        );
        Value::Object(result)
    }
}
