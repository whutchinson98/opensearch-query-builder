use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[cfg(feature = "visualizer")]
mod visualizer;

#[cfg(feature = "visualizer")]
pub use visualizer::{HtmlVisualization, Visualizable, VisualizationError};

/// Trait for converting a Rust struct to an OpenSearch JSON object.
pub trait ToOpenSearchJson {
    fn to_json(&self) -> serde_json::Value;
}

/// Enum representing the different types of queries that can be used in a search request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum QueryType {
    Term(TermQuery),
    Terms(TermsQuery),
    Match(MatchQuery),
    MatchPhrase(MatchPhraseQuery),
    MatchPhrasePrefix(MatchPhrasePrefixQuery),
    Bool(BoolQuery),
    Range(RangeQuery),
    MatchAll,
    WildCard(WildcardQuery),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum AggregationType {
    Terms(TermsAggregation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SortType {
    Field(FieldSort),
    Score,
}

/// Struct representing a search request.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<QueryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sort: Vec<SortType>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub aggs: HashMap<String, AggregationType>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub _source: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Highlight>,
}

impl SearchRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(mut self, query: QueryType) -> Self {
        self.query = Some(query);
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn from(mut self, from: u32) -> Self {
        self.from = Some(from);
        self
    }

    pub fn sort(mut self, sort: SortType) -> Self {
        self.sort.push(sort);
        self
    }

    pub fn agg(mut self, name: String, agg: AggregationType) -> Self {
        self.aggs.insert(name, agg);
        self
    }

    pub fn source_fields(mut self, fields: Vec<String>) -> Self {
        self._source = fields;
        self
    }

    pub fn highlight(mut self, highlight: Highlight) -> Self {
        self.highlight = Some(highlight);
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

        Value::Object(result)
    }
}

/// Convenience constructors for QueryType
impl QueryType {
    pub fn term<T: Into<Value>>(field: &str, value: T) -> Self {
        QueryType::Term(TermQuery::new(field, value))
    }

    pub fn terms<T: Into<Value>>(field: &str, values: Vec<T>) -> Self {
        QueryType::Terms(TermsQuery::new(field, values))
    }

    pub fn match_query(field: &str, query: &str) -> Self {
        QueryType::Match(MatchQuery::new(field, query))
    }

    pub fn match_phrase(field: &str, query: &str) -> Self {
        QueryType::MatchPhrase(MatchPhraseQuery::new(field, query))
    }

    pub fn match_phrase_prefix(field: &str, query: &str) -> Self {
        QueryType::MatchPhrasePrefix(MatchPhrasePrefixQuery::new(field, query))
    }

    pub fn bool_query() -> BoolQueryBuilder {
        BoolQueryBuilder::new()
    }

    pub fn range(field: &str) -> RangeQueryBuilder {
        RangeQueryBuilder::new(field)
    }

    pub fn match_all() -> Self {
        QueryType::MatchAll
    }
}

/// Builder pattern for BoolQuery
pub struct BoolQueryBuilder {
    inner: BoolQuery,
}

impl Default for BoolQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BoolQueryBuilder {
    pub fn new() -> Self {
        Self {
            inner: BoolQuery::new(),
        }
    }

    pub fn must(mut self, query: QueryType) -> Self {
        self.inner = self.inner.must(query);
        self
    }

    pub fn must_not(mut self, query: QueryType) -> Self {
        self.inner = self.inner.must_not(query);
        self
    }

    pub fn should(mut self, query: QueryType) -> Self {
        self.inner = self.inner.should(query);
        self
    }

    pub fn filter(mut self, query: QueryType) -> Self {
        self.inner = self.inner.filter(query);
        self
    }

    pub fn minimum_should_match(mut self, min: i32) -> Self {
        self.inner = self.inner.minimum_should_match(min);
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.inner = self.inner.boost(boost);
        self
    }

    pub fn build(self) -> QueryType {
        QueryType::Bool(self.inner)
    }
}

/// Builder pattern for RangeQuery
pub struct RangeQueryBuilder {
    inner: RangeQuery,
}

impl RangeQueryBuilder {
    pub fn new(field: &str) -> Self {
        Self {
            inner: RangeQuery::new(field),
        }
    }

    pub fn gte<T: Into<Value>>(mut self, value: T) -> Self {
        self.inner = self.inner.gte(value);
        self
    }

    pub fn gt<T: Into<Value>>(mut self, value: T) -> Self {
        self.inner = self.inner.gt(value);
        self
    }

    pub fn lte<T: Into<Value>>(mut self, value: T) -> Self {
        self.inner = self.inner.lte(value);
        self
    }

    pub fn lt<T: Into<Value>>(mut self, value: T) -> Self {
        self.inner = self.inner.lt(value);
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.inner = self.inner.boost(boost);
        self
    }

    pub fn build(self) -> QueryType {
        QueryType::Range(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_query_simple() {
        let query = QueryType::term("status", "published");
        let json = query.to_json();

        assert_eq!(
            json["term"]["status"],
            Value::String("published".to_string())
        );
    }

    #[test]
    fn test_term_query_with_boost() {
        let query = QueryType::Term(TermQuery::new("status", "published").boost(2.0));
        let json = query.to_json();

        assert_eq!(
            json["term"]["status"]["value"],
            Value::String("published".to_string())
        );
        assert_eq!(json["term"]["status"]["boost"], 2.0);
    }

    #[test]
    fn test_match_phrase_simple() {
        let query = QueryType::match_phrase("content", "testing");
        let json = query.to_json();

        assert_eq!(
            json["match_phrase"]["content"],
            Value::String("testing".to_string())
        );
    }

    #[test]
    fn test_match_phrase_with_options() {
        let query = QueryType::MatchPhrase(MatchPhraseQuery::new("content", "testing").slop(2));
        let json = query.to_json();

        assert_eq!(
            json["match_phrase"]["content"]["query"],
            Value::String("testing".to_string())
        );
        assert_eq!(
            json["match_phrase"]["content"]["slop"],
            Value::Number(2.into())
        );
    }

    #[test]
    fn test_terms_query_simple() {
        let query = QueryType::terms("file_type", vec!["pdf", "docx"]);
        let json = query.to_json();

        let expected_values = vec![
            Value::String("pdf".to_string()),
            Value::String("docx".to_string()),
        ];
        assert_eq!(json["terms"]["file_type"], Value::Array(expected_values));
    }

    #[test]
    fn test_bool_query() {
        let bool_query = QueryType::bool_query()
            .must(QueryType::term("status", "published"))
            .should(QueryType::match_query("title", "rust"))
            .minimum_should_match(1)
            .build();

        let json = bool_query.to_json();
        assert!(json["bool"]["must"].is_array());
        assert!(json["bool"]["should"].is_array());
        assert_eq!(
            json["bool"]["minimum_should_match"],
            Value::Number(1.into())
        );
    }

    #[test]
    fn test_search_request_serialization() {
        let request = SearchRequest::new()
            .query(QueryType::match_query("title", "elasticsearch"))
            .size(10)
            .from(0)
            .sort(SortType::Field(FieldSort::new(
                "created_at",
                SortOrder::Desc,
            )));

        let json = request.to_json();
        assert_eq!(
            json["query"]["match"]["title"],
            Value::String("elasticsearch".to_string())
        );
        assert_eq!(json["size"], Value::Number(10.into()));
        assert_eq!(json["from"], Value::Number(0.into()));
    }

    #[test]
    fn test_serde_roundtrip() {
        let request = SearchRequest::new()
            .query(QueryType::match_query("title", "test"))
            .size(5);

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: SearchRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.size, Some(5));
    }
}

impl ToOpenSearchJson for QueryType {
    fn to_json(&self) -> Value {
        match self {
            QueryType::Term(term) => term.to_json(),
            QueryType::Terms(terms) => terms.to_json(),
            QueryType::Match(match_query) => match_query.to_json(),
            QueryType::MatchPhrase(match_phrase) => match_phrase.to_json(),
            QueryType::MatchPhrasePrefix(match_phrase_prefix) => match_phrase_prefix.to_json(),
            QueryType::Bool(bool_query) => bool_query.to_json(),
            QueryType::Range(range) => range.to_json(),
            QueryType::MatchAll => serde_json::json!({"match_all": {}}),
            QueryType::WildCard(wildcard_query) => wildcard_query.to_json(),
        }
    }
}

impl ToOpenSearchJson for AggregationType {
    fn to_json(&self) -> Value {
        match self {
            AggregationType::Terms(terms) => terms.to_json(),
        }
    }
}

impl ToOpenSearchJson for SortType {
    fn to_json(&self) -> Value {
        match self {
            SortType::Field(field_sort) => field_sort.to_json(),
            SortType::Score => serde_json::json!("_score"),
        }
    }
}

// Term Query - FIXED: Support both simple and complex forms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermQuery {
    pub field: String,
    pub value: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f32>,
}

impl TermQuery {
    pub fn new<T: Into<Value>>(field: &str, value: T) -> Self {
        Self {
            field: field.to_string(),
            value: value.into(),
            boost: None,
        }
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.boost = Some(boost);
        self
    }
}

impl ToOpenSearchJson for TermQuery {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let mut term_obj = Map::new();

        // If we have additional parameters like boost, use the object form
        if self.boost.is_some() {
            let mut field_obj = Map::new();
            field_obj.insert("value".to_string(), self.value.clone());
            if let Some(boost) = self.boost {
                field_obj.insert("boost".to_string(), boost.into());
            }
            term_obj.insert(self.field.clone(), Value::Object(field_obj));
        } else {
            // Simple form: just field: value
            term_obj.insert(self.field.clone(), self.value.clone());
        }

        result.insert("term".to_string(), Value::Object(term_obj));
        Value::Object(result)
    }
}

// Terms Query - FIXED: Correct structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsQuery {
    pub field: String,
    pub values: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f32>,
}

impl TermsQuery {
    pub fn new<T: Into<Value>>(field: &str, values: Vec<T>) -> Self {
        Self {
            field: field.to_string(),
            values: values.into_iter().map(|v| v.into()).collect(),
            boost: None,
        }
    }

    pub fn boost(mut self, boost: f32) -> Self {
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

// Match Phrase Query - FIXED: Support both simple and complex forms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPhraseQuery {
    pub field: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slop: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f32>,
}

impl MatchPhraseQuery {
    pub fn new(field: &str, query: &str) -> Self {
        Self {
            field: field.to_string(),
            query: query.to_string(),
            analyzer: None,
            slop: None,
            boost: None,
        }
    }

    pub fn slop(mut self, slop: u32) -> Self {
        self.slop = Some(slop);
        self
    }

    pub fn analyzer(mut self, analyzer: &str) -> Self {
        self.analyzer = Some(analyzer.to_string());
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.boost = Some(boost);
        self
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

// Match Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchQuery {
    pub field: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f32>,
}

impl MatchQuery {
    pub fn new(field: &str, query: &str) -> Self {
        Self {
            field: field.to_string(),
            query: query.to_string(),
            operator: None,
            fuzziness: None,
            boost: None,
        }
    }

    pub fn operator(mut self, operator: &str) -> Self {
        self.operator = Some(operator.to_string());
        self
    }

    pub fn fuzziness(mut self, fuzziness: &str) -> Self {
        self.fuzziness = Some(fuzziness.to_string());
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.boost = Some(boost);
        self
    }
}

impl ToOpenSearchJson for MatchQuery {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let mut match_obj = Map::new();

        // Check if we need the complex form
        let has_options =
            self.operator.is_some() || self.fuzziness.is_some() || self.boost.is_some();

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

            match_obj.insert(self.field.clone(), Value::Object(field_obj));
        } else {
            // Simple form: field: "query"
            match_obj.insert(self.field.clone(), Value::String(self.query.clone()));
        }

        result.insert("match".to_string(), Value::Object(match_obj));
        Value::Object(result)
    }
}

// Match Phrase Prefix Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPhrasePrefixQuery {
    pub field: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expansions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slop: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f32>,
}

impl MatchPhrasePrefixQuery {
    pub fn new(field: &str, query: &str) -> Self {
        Self {
            field: field.to_string(),
            query: query.to_string(),
            max_expansions: None,
            slop: None,
            boost: None,
        }
    }

    pub fn max_expansions(mut self, max_expansions: u32) -> Self {
        self.max_expansions = Some(max_expansions);
        self
    }

    pub fn slop(mut self, slop: u32) -> Self {
        self.slop = Some(slop);
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.boost = Some(boost);
        self
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

// Bool Query
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BoolQuery {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub must: Vec<QueryType>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub must_not: Vec<QueryType>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub should: Vec<QueryType>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub filter: Vec<QueryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f32>,
}

impl BoolQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn must(mut self, query: QueryType) -> Self {
        self.must.push(query);
        self
    }

    pub fn must_not(mut self, query: QueryType) -> Self {
        self.must_not.push(query);
        self
    }

    pub fn should(mut self, query: QueryType) -> Self {
        self.should.push(query);
        self
    }

    pub fn filter(mut self, query: QueryType) -> Self {
        self.filter.push(query);
        self
    }

    pub fn minimum_should_match(mut self, min: i32) -> Self {
        self.minimum_should_match = Some(min);
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
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

// Range Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeQuery {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f32>,
}

impl RangeQuery {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
            gte: None,
            gt: None,
            lte: None,
            lt: None,
            boost: None,
        }
    }

    pub fn gte<T: Into<Value>>(mut self, value: T) -> Self {
        self.gte = Some(value.into());
        self
    }

    pub fn gt<T: Into<Value>>(mut self, value: T) -> Self {
        self.gt = Some(value.into());
        self
    }

    pub fn lte<T: Into<Value>>(mut self, value: T) -> Self {
        self.lte = Some(value.into());
        self
    }

    pub fn lt<T: Into<Value>>(mut self, value: T) -> Self {
        self.lt = Some(value.into());
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.boost = Some(boost);
        self
    }
}

impl ToOpenSearchJson for RangeQuery {
    fn to_json(&self) -> Value {
        let mut range_obj = Map::new();
        let mut field_obj = Map::new();

        if let Some(ref gte) = self.gte {
            field_obj.insert("gte".to_string(), gte.clone());
        }
        if let Some(ref gt) = self.gt {
            field_obj.insert("gt".to_string(), gt.clone());
        }
        if let Some(ref lte) = self.lte {
            field_obj.insert("lte".to_string(), lte.clone());
        }
        if let Some(ref lt) = self.lt {
            field_obj.insert("lt".to_string(), lt.clone());
        }
        if let Some(boost) = self.boost {
            field_obj.insert("boost".to_string(), boost.into());
        }

        range_obj.insert(self.field.clone(), Value::Object(field_obj));

        let mut result = Map::new();
        result.insert("range".to_string(), Value::Object(range_obj));
        Value::Object(result)
    }
}

// Field Sort
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSort {
    pub field: String,
    pub order: SortOrder,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl FieldSort {
    pub fn new(field: &str, order: SortOrder) -> Self {
        Self {
            field: field.to_string(),
            order,
            missing: None,
        }
    }

    pub fn missing(mut self, missing: &str) -> Self {
        self.missing = Some(missing.to_string());
        self
    }
}

impl ToOpenSearchJson for FieldSort {
    fn to_json(&self) -> Value {
        let mut field_obj = Map::new();
        field_obj.insert(
            "order".to_string(),
            Value::String(match self.order {
                SortOrder::Asc => "asc".to_string(),
                SortOrder::Desc => "desc".to_string(),
            }),
        );

        if let Some(ref missing) = self.missing {
            field_obj.insert("missing".to_string(), Value::String(missing.clone()));
        }

        let mut result = Map::new();
        result.insert(self.field.clone(), Value::Object(field_obj));
        Value::Object(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildcardQuery {
    field: String,
    value: String,
    case_insensitive: bool,
}

impl WildcardQuery {
    pub fn new(field: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            value: format!("*{}*", value.to_lowercase()),
            case_insensitive: true,
        }
    }

    pub fn case_insensitive(mut self, case_insensitive: bool) -> Self {
        self.case_insensitive = case_insensitive;
        self
    }
}

impl ToOpenSearchJson for WildcardQuery {
    fn to_json(&self) -> Value {
        serde_json::json!({
            "wildcard": {
                self.field.clone(): {
                    "value": self.value,
                    "case_insensitive": self.case_insensitive
                }
            }
        })
    }
}

// Terms Aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsAggregation {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub sub_aggs: HashMap<String, AggregationType>,
}

impl TermsAggregation {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
            size: None,
            sub_aggs: HashMap::new(),
        }
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

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

// Highlight support
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub fields: HashMap<String, HighlightField>,
}

impl Highlight {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field(mut self, field_name: &str, highlight_field: HighlightField) -> Self {
        self.fields.insert(field_name.to_string(), highlight_field);
        self
    }
}

impl ToOpenSearchJson for Highlight {
    fn to_json(&self) -> Value {
        let mut result = Map::new();

        if !self.fields.is_empty() {
            let mut fields_obj = Map::new();
            for (name, field) in &self.fields {
                fields_obj.insert(name.clone(), field.to_json());
            }
            result.insert("fields".to_string(), Value::Object(fields_obj));
        }

        Value::Object(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightField {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub highlight_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_fragments: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub pre_tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub post_tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_field_match: Option<bool>,
}

impl Default for HighlightField {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightField {
    pub fn new() -> Self {
        Self {
            highlight_type: None,
            number_of_fragments: None,
            pre_tags: Vec::new(),
            post_tags: Vec::new(),
            require_field_match: None,
        }
    }

    pub fn highlight_type(mut self, highlight_type: &str) -> Self {
        self.highlight_type = Some(highlight_type.to_string());
        self
    }

    pub fn number_of_fragments(mut self, number_of_fragments: u32) -> Self {
        self.number_of_fragments = Some(number_of_fragments);
        self
    }

    pub fn pre_tags(mut self, pre_tags: Vec<String>) -> Self {
        self.pre_tags = pre_tags;
        self
    }

    pub fn post_tags(mut self, post_tags: Vec<String>) -> Self {
        self.post_tags = post_tags;
        self
    }

    pub fn require_field_match(mut self, require_field_match: bool) -> Self {
        self.require_field_match = Some(require_field_match);
        self
    }
}

impl ToOpenSearchJson for HighlightField {
    fn to_json(&self) -> Value {
        let mut result = Map::new();

        if let Some(ref highlight_type) = self.highlight_type {
            result.insert("type".to_string(), Value::String(highlight_type.clone()));
        }

        if let Some(number_of_fragments) = self.number_of_fragments {
            result.insert(
                "number_of_fragments".to_string(),
                Value::Number(number_of_fragments.into()),
            );
        }

        if !self.pre_tags.is_empty() {
            let pre_tags: Vec<Value> = self
                .pre_tags
                .iter()
                .map(|tag| Value::String(tag.clone()))
                .collect();
            result.insert("pre_tags".to_string(), Value::Array(pre_tags));
        }

        if !self.post_tags.is_empty() {
            let post_tags: Vec<Value> = self
                .post_tags
                .iter()
                .map(|tag| Value::String(tag.clone()))
                .collect();
            result.insert("post_tags".to_string(), Value::Array(post_tags));
        }

        if let Some(require_field_match) = self.require_field_match {
            result.insert(
                "require_field_match".to_string(),
                Value::Bool(require_field_match),
            );
        }

        Value::Object(result)
    }
}
