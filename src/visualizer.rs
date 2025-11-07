use crate::{
    AggregationType, QueryType, SearchRequest, SortType, ToOpenSearchJson,
    visualizer::visualization_node::VisualizationNode,
};
use serde_json::Value;

mod html;
mod visualization_node;

pub use html::*;

/// Trait for converting a query component into a visualization node.
/// Once the node is built, it can be converted to various formats.
pub trait Visualizable {
    fn visualize(&self) -> VisualizationNode;
}

#[derive(Debug)]
pub enum VisualizationError {
    TemplateError(String),
    SerializationError(String),
}

impl std::fmt::Display for VisualizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualizationError::TemplateError(msg) => write!(f, "Template error: {msg}"),
            VisualizationError::SerializationError(msg) => {
                write!(f, "Serialization error: {msg}")
            }
        }
    }
}

impl std::error::Error for VisualizationError {}

impl Visualizable for SearchRequest {
    // Build the visualization node for the search request
    fn visualize(&self) -> VisualizationNode {
        let mut root = VisualizationNode::new("search").with_json(self.to_json());

        if let Some(ref query) = self.query {
            root = root.with_child(query.visualize());
        }

        if let Some(size) = self.size {
            root = root.with_detail("size", Value::Number(size.into()));
        }

        if let Some(from) = self.from {
            root = root.with_detail("from", Value::Number(from.into()));
        }

        if !self.sort.is_empty() {
            let mut sort_node = VisualizationNode::new("sort");
            for sort in &self.sort {
                sort_node = sort_node.with_child(sort.visualize());
            }
            root = root.with_child(sort_node);
        }

        if !self.aggs.is_empty() {
            let mut aggs_node = VisualizationNode::new("aggregations");
            for agg in self.aggs.values() {
                let agg_node = agg.visualize();
                aggs_node = aggs_node.with_child(agg_node);
            }
            root = root.with_child(aggs_node);
        }

        if !self._source.is_empty() {
            root = root.with_detail(
                "_source",
                Value::Array(
                    self._source
                        .iter()
                        .map(|s| Value::String(s.clone()))
                        .collect(),
                ),
            );
        }

        root
    }
}

impl Visualizable for QueryType {
    fn visualize(&self) -> VisualizationNode {
        match self {
            QueryType::Term(term) => VisualizationNode::new("term")
                .with_detail("field", Value::String(term.field.clone()))
                .with_detail("value", term.value.clone()),
            QueryType::Terms(terms) => VisualizationNode::new("terms")
                .with_detail("field", Value::String(terms.field.clone()))
                .with_detail("values", Value::Array(terms.values.clone())),
            QueryType::Match(match_q) => VisualizationNode::new("match")
                .with_detail("field", Value::String(match_q.field.clone()))
                .with_detail("query", Value::String(match_q.query.clone())),
            QueryType::MatchPhrase(match_phrase) => VisualizationNode::new("match_phrase")
                .with_detail("field", Value::String(match_phrase.field.clone()))
                .with_detail("query", Value::String(match_phrase.query.clone())),
            QueryType::MatchPhrasePrefix(match_phrase_prefix) => {
                VisualizationNode::new("match_phrase_prefix")
                    .with_detail("field", Value::String(match_phrase_prefix.field.clone()))
                    .with_detail("query", Value::String(match_phrase_prefix.query.clone()))
            }
            QueryType::Regexp(regexp_query) => {
                let mut node = VisualizationNode::new("regexp")
                    .with_detail("field", Value::String(regexp_query.field.clone()))
                    .with_detail("value", Value::String(regexp_query.value.clone()));

                if let Some(flags) = regexp_query.flags.as_ref()
                    && !flags.is_empty()
                {
                    node = node.with_detail(
                        "flags",
                        Value::String(
                            flags
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join("|"),
                        ),
                    );
                }

                node
            }
            QueryType::Bool(bool_query) => {
                let mut node = VisualizationNode::new("bool");

                for must in &bool_query.must {
                    node = node.with_child(must.visualize().with_clause_type("must"));
                }

                for must_not in &bool_query.must_not {
                    node = node.with_child(must_not.visualize().with_clause_type("must_not"));
                }

                for should in &bool_query.should {
                    node = node.with_child(should.visualize().with_clause_type("should"));
                }

                for filter in &bool_query.filter {
                    node = node.with_child(filter.visualize().with_clause_type("filter"));
                }

                if let Some(min_should_match) = bool_query.minimum_should_match {
                    node = node.with_detail(
                        "minimum_should_match",
                        Value::Number(min_should_match.into()),
                    );
                }

                if let Some(boost) = bool_query.boost {
                    node = node.with_detail("boost", Value::String(boost.to_string()));
                }

                node
            }
            QueryType::Range(range) => {
                let mut node = VisualizationNode::new("range")
                    .with_detail("field", Value::String(range.field.clone()));

                if let Some(ref gte) = range.gte {
                    node = node.with_detail("gte", gte.clone());
                }
                if let Some(ref gt) = range.gt {
                    node = node.with_detail("gt", gt.clone());
                }
                if let Some(ref lte) = range.lte {
                    node = node.with_detail("lte", lte.clone());
                }
                if let Some(ref lt) = range.lt {
                    node = node.with_detail("lt", lt.clone());
                }
                if let Some(boost) = range.boost {
                    node = node.with_detail("boost", Value::String(boost.to_string()));
                }

                node
            }
            QueryType::MatchAll => VisualizationNode::new("match_all"),
            QueryType::WildCard(wildcard) => VisualizationNode::new("wildcard")
                .with_detail("field", Value::String(wildcard.field.clone()))
                .with_detail("value", Value::String(wildcard.value.clone())),
            QueryType::FunctionScore(function_score) => {
                let mut node = VisualizationNode::new("function_score");

                if let Some(ref query) = function_score.query {
                    node = node.with_child(query.visualize().with_clause_type("query"));
                }

                if let Some(ref score_mode) = function_score.score_mode {
                    node =
                        node.with_detail("score_mode", Value::String(format!("{:?}", score_mode)));
                }

                if let Some(ref boost_mode) = function_score.boost_mode {
                    node =
                        node.with_detail("boost_mode", Value::String(format!("{:?}", boost_mode)));
                }

                if let Some(max_boost) = function_score.max_boost {
                    node = node.with_detail(
                        "max_boost",
                        Value::Number(serde_json::Number::from_f64(max_boost).unwrap()),
                    );
                }

                if let Some(min_score) = function_score.min_score {
                    node = node.with_detail(
                        "min_score",
                        Value::Number(serde_json::Number::from_f64(min_score).unwrap()),
                    );
                }

                if !function_score.functions.is_empty() {
                    node = node.with_detail(
                        "functions_count",
                        Value::Number((function_score.functions.len() as u64).into()),
                    );
                }

                node
            }
        }
    }
}

impl Visualizable for SortType {
    fn visualize(&self) -> VisualizationNode {
        match self {
            SortType::Field(field_sort) => VisualizationNode::new("field_sort")
                .with_detail("field", Value::String(field_sort.field.clone()))
                .with_detail("order", Value::String(format!("{:?}", field_sort.order))),
            SortType::Score => VisualizationNode::new("score_sort"),
            SortType::ScoreWithOrder(score_sort) => VisualizationNode::new("score_sort")
                .with_detail("order", Value::String(format!("{:?}", score_sort.order))),
        }
    }
}

impl Visualizable for AggregationType {
    fn visualize(&self) -> VisualizationNode {
        match self {
            AggregationType::Terms(terms_agg) => {
                let mut node = VisualizationNode::new("terms_agg")
                    .with_detail("field", Value::String(terms_agg.field.clone()));

                if let Some(size) = terms_agg.size {
                    node = node.with_detail("size", Value::Number(size.into()));
                }

                node
            }
            AggregationType::Cardinality(cardinality_agg) => {
                VisualizationNode::new("cardinality_agg")
                    .with_detail("field", Value::String(cardinality_agg.field.clone()))
            }
        }
    }
}

#[cfg(test)]
mod test;
