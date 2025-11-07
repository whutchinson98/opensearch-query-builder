use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{QueryType, ToOpenSearchJson};

mod aggregation_type;
mod collapse;
mod highlight;
mod sort_type;

pub use aggregation_type::*;
pub use collapse::*;
pub use highlight::*;
pub use sort_type::*;

/// Struct representing a search request.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    /// Query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<QueryType>,
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// Offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    /// Sort criteria
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sort: Vec<SortType>,
    /// Aggregations
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub aggs: HashMap<String, AggregationType>,
    /// Source fields
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub _source: Vec<String>,
    /// Highlight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Highlight>,
    /// Track total hits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_total_hits: Option<bool>,
    /// Collapse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse: Option<Collapse>,
}

impl SearchRequest {
    /// Create a new SearchRequest
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the query
    pub fn query(mut self, query: QueryType) -> Self {
        self.query = Some(query);
        self
    }

    /// Set the maximum number of results to return
    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the offset
    pub fn from(mut self, from: u32) -> Self {
        self.from = Some(from);
        self
    }

    /// Add a sort criterion
    pub fn sort(mut self, sort: SortType) -> Self {
        self.sort.push(sort);
        self
    }

    /// Add an aggregation
    pub fn agg(mut self, name: String, agg: AggregationType) -> Self {
        self.aggs.insert(name, agg);
        self
    }

    /// Set source fields
    pub fn source_fields(mut self, fields: Vec<String>) -> Self {
        self._source = fields;
        self
    }

    /// Set the highlight configuration
    pub fn highlight(mut self, highlight: Highlight) -> Self {
        self.highlight = Some(highlight);
        self
    }

    /// Set whether to track total hits
    pub fn track_total_hits(mut self, track: bool) -> Self {
        self.track_total_hits = Some(track);
        self
    }

    /// Set the collapse configuration
    pub fn collapse(mut self, collapse: Collapse) -> Self {
        self.collapse = Some(collapse);
        self
    }
}

impl ToOpenSearchJson for SearchRequest {
    fn to_json(&self) -> Value {
        let mut result = Map::new();

        if let Some(ref query) = self.query {
            result.insert("query".to_string(), query.to_json());
        }

        if let Some(size) = self.size {
            result.insert("size".to_string(), Value::Number(size.into()));
        }

        if let Some(from) = self.from {
            result.insert("from".to_string(), Value::Number(from.into()));
        }

        if !self.sort.is_empty() {
            let sorts: Vec<Value> = self.sort.iter().map(|s| s.to_json()).collect();
            result.insert("sort".to_string(), Value::Array(sorts));
        }

        if !self.aggs.is_empty() {
            let mut aggs_obj = Map::new();
            for (name, agg) in &self.aggs {
                aggs_obj.insert(name.clone(), agg.to_json());
            }
            result.insert("aggs".to_string(), Value::Object(aggs_obj));
        }

        if !self._source.is_empty() {
            let sources: Vec<Value> = self
                ._source
                .iter()
                .map(|s| Value::String(s.clone()))
                .collect();
            result.insert("_source".to_string(), Value::Array(sources));
        }

        if let Some(ref highlight) = self.highlight {
            result.insert("highlight".to_string(), highlight.to_json());
        }

        if let Some(track_total_hits) = self.track_total_hits {
            result.insert(
                "track_total_hits".to_string(),
                Value::Bool(track_total_hits),
            );
        }

        if let Some(ref collapse) = self.collapse {
            result.insert("collapse".to_string(), collapse.to_json());
        }

        Value::Object(result)
    }
}

/// Builder pattern for SearchRequest that allows dynamic updates.
/// Unlike the fluent methods on SearchRequest, this builder uses mutable methods
/// so you can dynamically add fields over time before calling build().
#[derive(Default, Debug, Clone)]
pub struct SearchRequestBuilder {
    query: Option<QueryType>,
    size: Option<u32>,
    from: Option<u32>,
    sort: Vec<SortType>,
    aggs: HashMap<String, AggregationType>,
    _source: Vec<String>,
    highlight: Option<Highlight>,
    track_total_hits: Option<bool>,
    collapse: Option<Collapse>,
}

impl SearchRequestBuilder {
    /// Create a new empty SearchRequestBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the query for this search request
    pub fn query(&mut self, query: QueryType) -> &mut Self {
        self.query = Some(query);
        self
    }

    /// Set the maximum number of results to return
    pub fn size(&mut self, size: u32) -> &mut Self {
        self.size = Some(size);
        self
    }

    /// Set the offset for pagination
    pub fn from(&mut self, from: u32) -> &mut Self {
        self.from = Some(from);
        self
    }

    /// Add a sort criterion (can be called multiple times)
    pub fn add_sort(&mut self, sort: SortType) -> &mut Self {
        self.sort.push(sort);
        self
    }

    /// Set all sort criteria at once (replaces existing sorts)
    pub fn set_sorts(&mut self, sorts: Vec<SortType>) -> &mut Self {
        self.sort = sorts;
        self
    }

    /// Clear all sort criteria
    pub fn clear_sorts(&mut self) -> &mut Self {
        self.sort.clear();
        self
    }

    /// Add an aggregation
    pub fn add_agg(&mut self, name: String, agg: AggregationType) -> &mut Self {
        self.aggs.insert(name, agg);
        self
    }

    /// Remove an aggregation by name
    pub fn remove_agg(&mut self, name: &str) -> &mut Self {
        self.aggs.remove(name);
        self
    }

    /// Clear all aggregations
    pub fn clear_aggs(&mut self) -> &mut Self {
        self.aggs.clear();
        self
    }

    /// Add a source field to include in the response
    pub fn add_source_field(&mut self, field: String) -> &mut Self {
        self._source.push(field);
        self
    }

    /// Set source fields (replaces existing fields)
    pub fn set_source_fields(&mut self, fields: Vec<String>) -> &mut Self {
        self._source = fields;
        self
    }

    /// Clear all source fields
    pub fn clear_source_fields(&mut self) -> &mut Self {
        self._source.clear();
        self
    }

    /// Set the highlight configuration
    pub fn highlight(&mut self, highlight: Highlight) -> &mut Self {
        self.highlight = Some(highlight);
        self
    }

    /// Set whether to track total hits
    pub fn track_total_hits(&mut self, track: bool) -> &mut Self {
        self.track_total_hits = Some(track);
        self
    }

    /// Set the collapse configuration
    pub fn collapse(&mut self, collapse: Collapse) -> &mut Self {
        self.collapse = Some(collapse);
        self
    }

    /// Build the final SearchRequest
    pub fn build(self) -> SearchRequest {
        SearchRequest {
            query: self.query,
            size: self.size,
            from: self.from,
            sort: self.sort,
            aggs: self.aggs,
            _source: self._source,
            highlight: self.highlight,
            track_total_hits: self.track_total_hits,
            collapse: self.collapse,
        }
    }
}
