use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{QueryType, ToOpenSearchJson};

/// Match Phrase Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPhraseQuery {
    /// The field to search
    pub field: String,
    /// The query string
    pub query: String,
    /// The slop value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slop: Option<u32>,
    /// The analyzer to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    /// The boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

impl MatchPhraseQuery {
    /// Create a new MatchPhraseQuery with a given field and query string
    pub fn new(field: &str, query: &str) -> Self {
        Self {
            field: field.to_string(),
            query: query.to_string(),
            analyzer: None,
            slop: None,
            boost: None,
        }
    }

    /// Set the slop value
    pub fn slop(mut self, slop: u32) -> Self {
        self.slop = Some(slop);
        self
    }

    /// Set the analyzer to use
    pub fn analyzer(mut self, analyzer: &str) -> Self {
        self.analyzer = Some(analyzer.to_string());
        self
    }

    /// Set the boost value
    pub fn boost(mut self, boost: f64) -> Self {
        self.boost = Some(boost);
        self
    }
}

impl From<MatchPhraseQuery> for QueryType {
    fn from(match_phrase_query: MatchPhraseQuery) -> Self {
        QueryType::MatchPhrase(match_phrase_query)
    }
}

impl ToOpenSearchJson for MatchPhraseQuery {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let mut match_phrase_obj = Map::new();

        // Check if we need the complex form
        let has_options = self.slop.is_some() || self.analyzer.is_some() || self.boost.is_some();

        if has_options {
            // Complex form with options
            let mut field_obj = Map::new();
            field_obj.insert("query".to_string(), Value::String(self.query.clone()));

            if let Some(analyzer) = self.analyzer.as_ref() {
                field_obj.insert("analyzer".to_string(), Value::String(analyzer.clone()));
            }
            if let Some(slop) = self.slop {
                field_obj.insert("slop".to_string(), Value::Number(slop.into()));
            }
            if let Some(boost) = self.boost {
                field_obj.insert("boost".to_string(), boost.into());
            }

            match_phrase_obj.insert(self.field.clone(), Value::Object(field_obj));
        } else {
            // Simple form: just field: "query"
            match_phrase_obj.insert(self.field.clone(), Value::String(self.query.clone()));
        }

        result.insert("match_phrase".to_string(), Value::Object(match_phrase_obj));
        Value::Object(result)
    }
}
