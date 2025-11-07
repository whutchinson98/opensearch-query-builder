use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{QueryType, ToOpenSearchJson};

/// Bool Query
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BoolQuery {
    /// Must queries
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub must: Vec<QueryType>,
    /// Must not queries
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub must_not: Vec<QueryType>,
    /// Should queries
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub should: Vec<QueryType>,
    /// Filter queries
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub filter: Vec<QueryType>,
    /// Minimum should match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<i32>,
    /// Boost
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

impl BoolQuery {
    /// Create a new empty BoolQuery
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a must query
    pub fn must(mut self, query: QueryType) -> Self {
        self.must.push(query);
        self
    }

    /// Add a must not query
    pub fn must_not(mut self, query: QueryType) -> Self {
        self.must_not.push(query);
        self
    }

    /// Add a should query
    pub fn should(mut self, query: QueryType) -> Self {
        self.should.push(query);
        self
    }

    /// Add a filter query
    pub fn filter(mut self, query: QueryType) -> Self {
        self.filter.push(query);
        self
    }

    /// Set the minimum should match
    pub fn minimum_should_match(mut self, min: i32) -> Self {
        self.minimum_should_match = Some(min);
        self
    }

    /// Set the boost
    pub fn boost(mut self, boost: f64) -> Self {
        self.boost = Some(boost);
        self
    }
}

impl ToOpenSearchJson for BoolQuery {
    fn to_json(&self) -> Value {
        let mut bool_obj = Map::new();

        if !self.must.is_empty() {
            let must_queries: Vec<Value> = self.must.iter().map(|q| q.to_json()).collect();
            bool_obj.insert("must".to_string(), Value::Array(must_queries));
        }

        if !self.must_not.is_empty() {
            let must_not_queries: Vec<Value> = self.must_not.iter().map(|q| q.to_json()).collect();
            bool_obj.insert("must_not".to_string(), Value::Array(must_not_queries));
        }

        if !self.should.is_empty() {
            let should_queries: Vec<Value> = self.should.iter().map(|q| q.to_json()).collect();
            bool_obj.insert("should".to_string(), Value::Array(should_queries));
        }

        if !self.filter.is_empty() {
            let filter_queries: Vec<Value> = self.filter.iter().map(|f| f.to_json()).collect();
            bool_obj.insert("filter".to_string(), Value::Array(filter_queries));
        }

        if let Some(min) = self.minimum_should_match {
            bool_obj.insert(
                "minimum_should_match".to_string(),
                Value::Number(min.into()),
            );
        }

        if let Some(boost) = self.boost {
            bool_obj.insert("boost".to_string(), boost.into());
        }

        let mut result = Map::new();
        result.insert("bool".to_string(), Value::Object(bool_obj));
        Value::Object(result)
    }
}

/// Builder pattern for BoolQuery that allows dynamic updates.
#[derive(Default, Debug, Clone)]
pub struct BoolQueryBuilder {
    must: Vec<QueryType>,
    must_not: Vec<QueryType>,
    should: Vec<QueryType>,
    filter: Vec<QueryType>,
    minimum_should_match: Option<i32>,
    boost: Option<f64>,
}

impl BoolQueryBuilder {
    /// Create a new empty BoolQueryBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a must query
    pub fn must(&mut self, query: QueryType) -> &mut Self {
        self.must.push(query);
        self
    }

    /// Add a must not query
    pub fn must_not(&mut self, query: QueryType) -> &mut Self {
        self.must_not.push(query);
        self
    }

    /// Add a should query
    pub fn should(&mut self, query: QueryType) -> &mut Self {
        self.should.push(query);
        self
    }

    /// Add a filter query
    pub fn filter(&mut self, query: QueryType) -> &mut Self {
        self.filter.push(query);
        self
    }

    /// Set the minimum should match
    pub fn minimum_should_match(&mut self, min: i32) -> &mut Self {
        self.minimum_should_match = Some(min);
        self
    }

    /// Set the boost
    pub fn boost(&mut self, boost: f64) -> &mut Self {
        self.boost = Some(boost);
        self
    }

    /// Build the final BoolQuery
    pub fn build(self) -> BoolQuery {
        BoolQuery {
            must: self.must,
            must_not: self.must_not,
            should: self.should,
            filter: self.filter,
            minimum_should_match: self.minimum_should_match,
            boost: self.boost,
        }
    }
}

#[cfg(test)]
mod test;
