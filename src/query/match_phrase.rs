use std::borrow::Cow;

use serde::Serialize;
use serde_json::{Map, Value};

use crate::{QueryType, ToOpenSearchJson};

/// Match Phrase Query
#[derive(Debug, Clone, Serialize)]
pub struct MatchPhraseQuery<'a> {
    /// The field to search
    #[serde(borrow)]
    pub field: Cow<'a, str>,
    /// The query string
    #[serde(borrow)]
    pub query: Cow<'a, str>,
    /// The slop value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slop: Option<u32>,
    /// The analyzer to use
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub analyzer: Option<Cow<'a, str>>,
    /// The boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

impl<'a> MatchPhraseQuery<'a> {
    /// Create a new MatchPhraseQuery with a given field and query string
    pub fn new(field: Cow<'a, str>, query: Cow<'a, str>) -> Self {
        Self {
            field,
            query,
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
    pub fn analyzer(mut self, analyzer: Cow<'a, str>) -> Self {
        self.analyzer = Some(analyzer);
        self
    }

    /// Set the boost value
    pub fn boost(mut self, boost: f64) -> Self {
        self.boost = Some(boost);
        self
    }
}

impl<'a> From<MatchPhraseQuery<'a>> for QueryType<'a> {
    fn from(match_phrase_query: MatchPhraseQuery<'a>) -> Self {
        QueryType::MatchPhrase(match_phrase_query)
    }
}

impl<'a> ToOpenSearchJson for MatchPhraseQuery<'a> {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let mut match_phrase_obj = Map::new();

        // Check if we need the complex form
        let has_options = self.slop.is_some() || self.analyzer.is_some() || self.boost.is_some();

        if has_options {
            // Complex form with options
            let mut field_obj = Map::new();
            field_obj.insert("query".to_string(), Value::String(self.query.to_string()));

            if let Some(analyzer) = self.analyzer.as_ref() {
                field_obj.insert("analyzer".to_string(), Value::String(analyzer.to_string()));
            }
            if let Some(slop) = self.slop {
                field_obj.insert("slop".to_string(), Value::Number(slop.into()));
            }
            if let Some(boost) = self.boost {
                field_obj.insert("boost".to_string(), boost.into());
            }

            match_phrase_obj.insert(self.field.to_string(), Value::Object(field_obj));
        } else {
            // Simple form: just field: "query"
            match_phrase_obj.insert(
                self.field.to_string(),
                Value::String(self.query.to_string()),
            );
        }

        result.insert("match_phrase".to_string(), Value::Object(match_phrase_obj));
        Value::Object(result)
    }
}
