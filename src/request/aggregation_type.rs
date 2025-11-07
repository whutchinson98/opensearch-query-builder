use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::ToOpenSearchJson;

/// Cardinality Aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardinalityAggregation {
    /// The field to aggregate
    pub field: String,
}

impl CardinalityAggregation {
    /// Create a new CardinalityAggregation
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
        }
    }
}

impl ToOpenSearchJson for CardinalityAggregation {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let mut cardinality_obj = Map::new();
        cardinality_obj.insert("field".to_string(), Value::String(self.field.clone()));
        result.insert("cardinality".to_string(), Value::Object(cardinality_obj));
        Value::Object(result)
    }
}

/// Terms Aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsAggregation {
    /// The field to aggregate
    pub field: String,
    /// The maximum number of terms to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// Sub-aggregations
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub sub_aggs: HashMap<String, AggregationType>,
}

impl TermsAggregation {
    /// Create a new TermsAggregation
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
            size: None,
            sub_aggs: HashMap::new(),
        }
    }

    /// Set the maximum number of terms to return
    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    /// Add a sub-aggregation
    pub fn sub_agg(mut self, name: &str, agg: AggregationType) -> Self {
        self.sub_aggs.insert(name.to_string(), agg);
        self
    }
}

impl ToOpenSearchJson for TermsAggregation {
    fn to_json(&self) -> Value {
        let mut terms_obj = Map::new();
        terms_obj.insert("field".to_string(), Value::String(self.field.clone()));

        if let Some(size) = self.size {
            terms_obj.insert("size".to_string(), Value::Number(size.into()));
        }

        let mut result = Map::new();
        result.insert("terms".to_string(), Value::Object(terms_obj));

        if !self.sub_aggs.is_empty() {
            let mut aggs_obj = Map::new();
            for (name, agg) in &self.sub_aggs {
                aggs_obj.insert(name.clone(), agg.to_json());
            }
            result.insert("aggs".to_string(), Value::Object(aggs_obj));
        }

        Value::Object(result)
    }
}

/// Aggregation Type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum AggregationType {
    /// Terms aggregation
    Terms(TermsAggregation),
    /// Cardinality aggregation
    Cardinality(CardinalityAggregation),
}

impl ToOpenSearchJson for AggregationType {
    fn to_json(&self) -> Value {
        match self {
            AggregationType::Terms(terms) => terms.to_json(),
            AggregationType::Cardinality(cardinality) => cardinality.to_json(),
        }
    }
}
