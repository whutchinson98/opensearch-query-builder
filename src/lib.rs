use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{collections::HashMap, fmt::Display};

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
    Regexp(RegexpQuery),
    FunctionScore(FunctionScoreQuery),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum AggregationType {
    Terms(TermsAggregation),
    Cardinality(CardinalityAggregation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SortType {
    Field(FieldSort),
    Score,
    ScoreWithOrder(ScoreSort),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_total_hits: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse: Option<Collapse>,
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

    pub fn track_total_hits(mut self, track: bool) -> Self {
        self.track_total_hits = Some(track);
        self
    }

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

    pub fn function_score() -> FunctionScoreQueryBuilder {
        FunctionScoreQueryBuilder::new()
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

// Function Score enums and structs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreMode {
    Multiply,
    Sum,
    Avg,
    First,
    Max,
    Min,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoostMode {
    Multiply,
    Replace,
    Sum,
    Avg,
    Max,
    Min,
}

/// Decay function configuration for location/time-based scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayFunction {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Value>,
    pub scale: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decay: Option<f64>,
}

impl DecayFunction {
    pub fn new(field: &str, scale: &str) -> Self {
        Self {
            field: field.to_string(),
            origin: None,
            scale: scale.to_string(),
            offset: None,
            decay: None,
        }
    }

    pub fn origin<T: Into<Value>>(mut self, origin: T) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn offset(mut self, offset: &str) -> Self {
        self.offset = Some(offset.to_string());
        self
    }

    pub fn decay(mut self, decay: f64) -> Self {
        self.decay = Some(decay);
        self
    }
}

/// Field value factor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValueFactor {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<f64>,
}

impl FieldValueFactor {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
            factor: None,
            modifier: None,
            missing: None,
        }
    }

    pub fn factor(mut self, factor: f64) -> Self {
        self.factor = Some(factor);
        self
    }

    pub fn modifier(mut self, modifier: &str) -> Self {
        self.modifier = Some(modifier.to_string());
        self
    }

    pub fn missing(mut self, missing: f64) -> Self {
        self.missing = Some(missing);
        self
    }
}

/// Random score configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomScore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl RandomScore {
    pub fn new() -> Self {
        Self {
            seed: None,
            field: None,
        }
    }

    pub fn seed<T: Into<Value>>(mut self, seed: T) -> Self {
        self.seed = Some(seed.into());
        self
    }

    pub fn field(mut self, field: &str) -> Self {
        self.field = Some(field.to_string());
        self
    }
}

impl Default for RandomScore {
    fn default() -> Self {
        Self::new()
    }
}

/// Script score configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScore {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Map<String, Value>>,
}

impl ScriptScore {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            params: None,
        }
    }

    pub fn params(mut self, params: Map<String, Value>) -> Self {
        self.params = Some(params);
        self
    }
}

/// Enum representing different scoring functions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScoreFunctionType {
    Gauss(DecayFunction),
    Exp(DecayFunction),
    Linear(DecayFunction),
    FieldValueFactor(FieldValueFactor),
    RandomScore(RandomScore),
    ScriptScore(ScriptScore),
    Weight(f64),
}

/// A single scoring function with optional filter and weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreFunction {
    #[serde(flatten)]
    pub function: ScoreFunctionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<QueryType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

impl ScoreFunction {
    pub fn gauss(field: &str, scale: &str) -> Self {
        Self {
            function: ScoreFunctionType::Gauss(DecayFunction::new(field, scale)),
            filter: None,
            weight: None,
        }
    }

    pub fn exp(field: &str, scale: &str) -> Self {
        Self {
            function: ScoreFunctionType::Exp(DecayFunction::new(field, scale)),
            filter: None,
            weight: None,
        }
    }

    pub fn linear(field: &str, scale: &str) -> Self {
        Self {
            function: ScoreFunctionType::Linear(DecayFunction::new(field, scale)),
            filter: None,
            weight: None,
        }
    }

    pub fn field_value_factor(field: &str) -> Self {
        Self {
            function: ScoreFunctionType::FieldValueFactor(FieldValueFactor::new(field)),
            filter: None,
            weight: None,
        }
    }

    pub fn random_score() -> Self {
        Self {
            function: ScoreFunctionType::RandomScore(RandomScore::new()),
            filter: None,
            weight: None,
        }
    }

    pub fn script_score(source: &str) -> Self {
        Self {
            function: ScoreFunctionType::ScriptScore(ScriptScore::new(source)),
            filter: None,
            weight: None,
        }
    }

    pub fn weight_only(weight: f64) -> Self {
        Self {
            function: ScoreFunctionType::Weight(weight),
            filter: None,
            weight: Some(weight),
        }
    }

    pub fn filter(mut self, filter: QueryType) -> Self {
        self.filter = Some(Box::new(filter));
        self
    }

    pub fn weight(mut self, weight: f64) -> Self {
        self.weight = Some(weight);
        self
    }

    // Builder methods for decay functions
    pub fn origin<T: Into<Value>>(mut self, origin: T) -> Self {
        match &mut self.function {
            ScoreFunctionType::Gauss(decay)
            | ScoreFunctionType::Exp(decay)
            | ScoreFunctionType::Linear(decay) => {
                decay.origin = Some(origin.into());
            }
            _ => {}
        }
        self
    }

    pub fn offset(mut self, offset: &str) -> Self {
        match &mut self.function {
            ScoreFunctionType::Gauss(decay)
            | ScoreFunctionType::Exp(decay)
            | ScoreFunctionType::Linear(decay) => {
                decay.offset = Some(offset.to_string());
            }
            _ => {}
        }
        self
    }

    pub fn decay(mut self, decay_val: f64) -> Self {
        match &mut self.function {
            ScoreFunctionType::Gauss(decay)
            | ScoreFunctionType::Exp(decay)
            | ScoreFunctionType::Linear(decay) => {
                decay.decay = Some(decay_val);
            }
            _ => {}
        }
        self
    }

    // Builder methods for field_value_factor
    pub fn factor(mut self, factor: f64) -> Self {
        if let ScoreFunctionType::FieldValueFactor(fvf) = &mut self.function {
            fvf.factor = Some(factor);
        }
        self
    }

    pub fn modifier(mut self, modifier: &str) -> Self {
        if let ScoreFunctionType::FieldValueFactor(fvf) = &mut self.function {
            fvf.modifier = Some(modifier.to_string());
        }
        self
    }

    pub fn missing(mut self, missing: f64) -> Self {
        if let ScoreFunctionType::FieldValueFactor(fvf) = &mut self.function {
            fvf.missing = Some(missing);
        }
        self
    }

    // Builder methods for random_score
    pub fn seed<T: Into<Value>>(mut self, seed: T) -> Self {
        if let ScoreFunctionType::RandomScore(rs) = &mut self.function {
            rs.seed = Some(seed.into());
        }
        self
    }

    pub fn field(mut self, field: &str) -> Self {
        if let ScoreFunctionType::RandomScore(rs) = &mut self.function {
            rs.field = Some(field.to_string());
        }
        self
    }

    // Builder methods for script_score
    pub fn params(mut self, params: Map<String, Value>) -> Self {
        if let ScoreFunctionType::ScriptScore(ss) = &mut self.function {
            ss.params = Some(params);
        }
        self
    }
}

impl ToOpenSearchJson for ScoreFunction {
    fn to_json(&self) -> Value {
        let mut result = Map::new();

        // Add the function type
        match &self.function {
            ScoreFunctionType::Gauss(decay) => {
                let mut decay_obj = Map::new();
                let mut field_obj = Map::new();

                if let Some(ref origin) = decay.origin {
                    field_obj.insert("origin".to_string(), origin.clone());
                }
                field_obj.insert("scale".to_string(), Value::String(decay.scale.clone()));
                if let Some(ref offset) = decay.offset {
                    field_obj.insert("offset".to_string(), Value::String(offset.clone()));
                }
                if let Some(decay_val) = decay.decay {
                    field_obj.insert("decay".to_string(), decay_val.into());
                }

                decay_obj.insert(decay.field.clone(), Value::Object(field_obj));
                result.insert("gauss".to_string(), Value::Object(decay_obj));
            }
            ScoreFunctionType::Exp(decay) => {
                let mut decay_obj = Map::new();
                let mut field_obj = Map::new();

                if let Some(ref origin) = decay.origin {
                    field_obj.insert("origin".to_string(), origin.clone());
                }
                field_obj.insert("scale".to_string(), Value::String(decay.scale.clone()));
                if let Some(ref offset) = decay.offset {
                    field_obj.insert("offset".to_string(), Value::String(offset.clone()));
                }
                if let Some(decay_val) = decay.decay {
                    field_obj.insert("decay".to_string(), decay_val.into());
                }

                decay_obj.insert(decay.field.clone(), Value::Object(field_obj));
                result.insert("exp".to_string(), Value::Object(decay_obj));
            }
            ScoreFunctionType::Linear(decay) => {
                let mut decay_obj = Map::new();
                let mut field_obj = Map::new();

                if let Some(ref origin) = decay.origin {
                    field_obj.insert("origin".to_string(), origin.clone());
                }
                field_obj.insert("scale".to_string(), Value::String(decay.scale.clone()));
                if let Some(ref offset) = decay.offset {
                    field_obj.insert("offset".to_string(), Value::String(offset.clone()));
                }
                if let Some(decay_val) = decay.decay {
                    field_obj.insert("decay".to_string(), decay_val.into());
                }

                decay_obj.insert(decay.field.clone(), Value::Object(field_obj));
                result.insert("linear".to_string(), Value::Object(decay_obj));
            }
            ScoreFunctionType::FieldValueFactor(fvf) => {
                let mut fvf_obj = Map::new();
                fvf_obj.insert("field".to_string(), Value::String(fvf.field.clone()));
                if let Some(factor) = fvf.factor {
                    fvf_obj.insert("factor".to_string(), factor.into());
                }
                if let Some(ref modifier) = fvf.modifier {
                    fvf_obj.insert("modifier".to_string(), Value::String(modifier.clone()));
                }
                if let Some(missing) = fvf.missing {
                    fvf_obj.insert("missing".to_string(), missing.into());
                }
                result.insert("field_value_factor".to_string(), Value::Object(fvf_obj));
            }
            ScoreFunctionType::RandomScore(rs) => {
                let mut rs_obj = Map::new();
                if let Some(ref seed) = rs.seed {
                    rs_obj.insert("seed".to_string(), seed.clone());
                }
                if let Some(ref field) = rs.field {
                    rs_obj.insert("field".to_string(), Value::String(field.clone()));
                }
                result.insert("random_score".to_string(), Value::Object(rs_obj));
            }
            ScoreFunctionType::ScriptScore(ss) => {
                let mut script_obj = Map::new();
                script_obj.insert("source".to_string(), Value::String(ss.source.clone()));
                if let Some(ref params) = ss.params {
                    script_obj.insert("params".to_string(), Value::Object(params.clone()));
                }
                let mut ss_obj = Map::new();
                ss_obj.insert("script".to_string(), Value::Object(script_obj));
                result.insert("script_score".to_string(), Value::Object(ss_obj));
            }
            ScoreFunctionType::Weight(_) => {
                // Weight-only functions don't add a function type field
            }
        }

        // Add filter if present
        if let Some(ref filter) = self.filter {
            result.insert("filter".to_string(), filter.to_json());
        }

        // Add weight if present
        if let Some(weight) = self.weight {
            result.insert("weight".to_string(), weight.into());
        }

        Value::Object(result)
    }
}

/// Function Score Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionScoreQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<QueryType>>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub functions: Vec<ScoreFunction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_mode: Option<ScoreMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_mode: Option<BoostMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_boost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
}

impl Default for FunctionScoreQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionScoreQuery {
    pub fn new() -> Self {
        Self {
            query: None,
            functions: Vec::new(),
            score_mode: None,
            boost_mode: None,
            max_boost: None,
            boost: None,
            min_score: None,
        }
    }

    pub fn query(mut self, query: QueryType) -> Self {
        self.query = Some(Box::new(query));
        self
    }

    pub fn function(mut self, function: ScoreFunction) -> Self {
        self.functions.push(function);
        self
    }

    pub fn score_mode(mut self, score_mode: ScoreMode) -> Self {
        self.score_mode = Some(score_mode);
        self
    }

    pub fn boost_mode(mut self, boost_mode: BoostMode) -> Self {
        self.boost_mode = Some(boost_mode);
        self
    }

    pub fn max_boost(mut self, max_boost: f64) -> Self {
        self.max_boost = Some(max_boost);
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.boost = Some(boost);
        self
    }

    pub fn min_score(mut self, min_score: f64) -> Self {
        self.min_score = Some(min_score);
        self
    }
}

impl ToOpenSearchJson for FunctionScoreQuery {
    fn to_json(&self) -> Value {
        let mut function_score_obj = Map::new();

        // Add query if present
        if let Some(ref query) = self.query {
            function_score_obj.insert("query".to_string(), query.to_json());
        }

        // Add functions array if not empty
        if !self.functions.is_empty() {
            let functions: Vec<Value> = self.functions.iter().map(|f| f.to_json()).collect();
            function_score_obj.insert("functions".to_string(), Value::Array(functions));
        }

        // Add score_mode if present
        if let Some(ref score_mode) = self.score_mode {
            function_score_obj.insert(
                "score_mode".to_string(),
                Value::String(match score_mode {
                    ScoreMode::Multiply => "multiply".to_string(),
                    ScoreMode::Sum => "sum".to_string(),
                    ScoreMode::Avg => "avg".to_string(),
                    ScoreMode::First => "first".to_string(),
                    ScoreMode::Max => "max".to_string(),
                    ScoreMode::Min => "min".to_string(),
                }),
            );
        }

        // Add boost_mode if present
        if let Some(ref boost_mode) = self.boost_mode {
            function_score_obj.insert(
                "boost_mode".to_string(),
                Value::String(match boost_mode {
                    BoostMode::Multiply => "multiply".to_string(),
                    BoostMode::Replace => "replace".to_string(),
                    BoostMode::Sum => "sum".to_string(),
                    BoostMode::Avg => "avg".to_string(),
                    BoostMode::Max => "max".to_string(),
                    BoostMode::Min => "min".to_string(),
                }),
            );
        }

        // Add max_boost if present
        if let Some(max_boost) = self.max_boost {
            function_score_obj.insert("max_boost".to_string(), max_boost.into());
        }

        // Add boost if present
        if let Some(boost) = self.boost {
            function_score_obj.insert("boost".to_string(), boost.into());
        }

        // Add min_score if present
        if let Some(min_score) = self.min_score {
            function_score_obj.insert("min_score".to_string(), min_score.into());
        }

        let mut result = Map::new();
        result.insert(
            "function_score".to_string(),
            Value::Object(function_score_obj),
        );
        Value::Object(result)
    }
}

/// Builder pattern for FunctionScoreQuery
pub struct FunctionScoreQueryBuilder {
    inner: FunctionScoreQuery,
}

impl Default for FunctionScoreQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionScoreQueryBuilder {
    pub fn new() -> Self {
        Self {
            inner: FunctionScoreQuery::new(),
        }
    }

    pub fn query(mut self, query: QueryType) -> Self {
        self.inner = self.inner.query(query);
        self
    }

    pub fn function(mut self, function: ScoreFunction) -> Self {
        self.inner = self.inner.function(function);
        self
    }

    pub fn score_mode(mut self, score_mode: ScoreMode) -> Self {
        self.inner = self.inner.score_mode(score_mode);
        self
    }

    pub fn boost_mode(mut self, boost_mode: BoostMode) -> Self {
        self.inner = self.inner.boost_mode(boost_mode);
        self
    }

    pub fn max_boost(mut self, max_boost: f64) -> Self {
        self.inner = self.inner.max_boost(max_boost);
        self
    }

    pub fn boost(mut self, boost: f32) -> Self {
        self.inner = self.inner.boost(boost);
        self
    }

    pub fn min_score(mut self, min_score: f64) -> Self {
        self.inner = self.inner.min_score(min_score);
        self
    }

    pub fn build(self) -> QueryType {
        QueryType::FunctionScore(self.inner)
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
            QueryType::Regexp(regexp_query) => regexp_query.to_json(),
            QueryType::FunctionScore(function_score) => function_score.to_json(),
        }
    }
}

impl ToOpenSearchJson for AggregationType {
    fn to_json(&self) -> Value {
        match self {
            AggregationType::Terms(terms) => terms.to_json(),
            AggregationType::Cardinality(cardinality) => cardinality.to_json(),
        }
    }
}

impl ToOpenSearchJson for SortType {
    fn to_json(&self) -> Value {
        match self {
            SortType::Field(field_sort) => field_sort.to_json(),
            SortType::Score => serde_json::json!("_score"),
            SortType::ScoreWithOrder(score_sort) => score_sort.to_json(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<String>,
}

impl MatchQuery {
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

    pub fn minimum_should_match(mut self, minimum_should_match: &str) -> Self {
        self.minimum_should_match = Some(minimum_should_match.to_string());
        self
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

// Sort Order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

// Field Sort
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSort {
    pub field: String,
    pub order: SortOrder,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,
}

// Score Sort
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreSort {
    pub order: SortOrder,
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

impl ScoreSort {
    pub fn new(order: SortOrder) -> Self {
        Self { order }
    }
}

impl ToOpenSearchJson for FieldSort {
    fn to_json(&self) -> Value {
        let mut result = Map::new();

        // Use simplified format when there are no additional parameters
        if self.missing.is_none() {
            result.insert(
                self.field.clone(),
                Value::String(match self.order {
                    SortOrder::Asc => "asc".to_string(),
                    SortOrder::Desc => "desc".to_string(),
                }),
            );
        } else {
            // Use object format when there are additional parameters
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

            result.insert(self.field.clone(), Value::Object(field_obj));
        }

        Value::Object(result)
    }
}

impl ToOpenSearchJson for ScoreSort {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        result.insert(
            "_score".to_string(),
            Value::String(match self.order {
                SortOrder::Asc => "asc".to_string(),
                SortOrder::Desc => "desc".to_string(),
            }),
        );
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum RegexpQueryFlags {
    /// Enables all optional features (default behavior)
    All,
    /// Allows @ to match any string
    Anystring,
    /// Matches the complement of the language described by the regex
    Complement,
    /// Allows matching empty strings
    Empty,
    /// Enables intersection of multiple patterns
    Intersection,
    /// Enables interval arithmetic on character classes
    Interval,
    /// Disables all optional features (default behavior)
    #[default]
    None,
}

impl RegexpQueryFlags {
    pub fn all() -> Vec<Self> {
        vec![Self::All]
    }
}

impl Display for RegexpQueryFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexpQueryFlags::All => write!(f, "ALL"),
            RegexpQueryFlags::Anystring => write!(f, "ANYSTRING"),
            RegexpQueryFlags::Complement => write!(f, "COMPLEMENT"),
            RegexpQueryFlags::Empty => write!(f, "EMPTY"),
            RegexpQueryFlags::Intersection => write!(f, "INTERSECTION"),
            RegexpQueryFlags::Interval => write!(f, "INTERVAL"),
            RegexpQueryFlags::None => write!(f, "NONE"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegexpQuery {
    /// The field to search in
    pub field: String,
    /// The stringified regex pattern to match on
    pub value: String,
    /// The flags to use when matching the regular expression
    pub flags: Option<Vec<RegexpQueryFlags>>,
}

impl RegexpQuery {
    pub fn new(field: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            value: value.to_string(),
            flags: None,
        }
    }

    pub fn flags(mut self, flags: Vec<RegexpQueryFlags>) -> Self {
        self.flags = Some(flags);
        self
    }
}

impl ToOpenSearchJson for RegexpQuery {
    fn to_json(&self) -> Value {
        let mut json = serde_json::json!({
            "regexp": {
                self.field.clone(): {
                    "value": self.value,
                }
            }
        });

        if let Some(flags) = self.flags.as_ref()
            && !flags.is_empty()
        {
            // Join all flags with |
            json["regexp"][self.field.clone()]["flags"] = Value::String(
                flags
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("|"),
            );
        }

        json
    }
}

// Cardinality Aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardinalityAggregation {
    pub field: String,
}

impl CardinalityAggregation {
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

// Collapse support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collapse {
    pub field: String,
}

impl Collapse {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
        }
    }
}

impl ToOpenSearchJson for Collapse {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        result.insert("field".to_string(), Value::String(self.field.clone()));
        Value::Object(result)
    }
}

// Highlight support
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub fields: HashMap<String, HighlightField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_field_match: Option<bool>,
}

impl Highlight {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field(mut self, field_name: &str, highlight_field: HighlightField) -> Self {
        self.fields.insert(field_name.to_string(), highlight_field);
        self
    }

    pub fn require_field_match(mut self, require_field_match: bool) -> Self {
        self.require_field_match = Some(require_field_match);
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

        if let Some(require_field_match) = self.require_field_match {
            result.insert(
                "require_field_match".to_string(),
                Value::Bool(require_field_match),
            );
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

        Value::Object(result)
    }
}

#[cfg(test)]
mod test;
