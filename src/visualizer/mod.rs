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
        let mut root = VisualizationNode::new("search", "SearchRequest").with_json(self.to_json());

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
            let mut sort_node = VisualizationNode::new("sort", "Sort");
            for sort in &self.sort {
                sort_node = sort_node.with_child(sort.visualize());
            }
            root = root.with_child(sort_node);
        }

        if !self.aggs.is_empty() {
            let mut aggs_node = VisualizationNode::new("aggregations", "Aggregations");
            for (name, agg) in &self.aggs {
                let mut agg_node = agg.visualize();
                agg_node.label = format!("{}: {}", name, agg_node.label);
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
            QueryType::Term(term) => {
                VisualizationNode::new("term", &format!("Field: {}", term.field))
                    .with_detail("field", Value::String(term.field.clone()))
                    .with_detail("value", term.value.clone())
            }
            QueryType::Terms(terms) => {
                VisualizationNode::new("terms", &format!("Field: {}", terms.field))
                    .with_detail("field", Value::String(terms.field.clone()))
                    .with_detail("values", Value::Array(terms.values.clone()))
            }
            QueryType::Match(match_q) => {
                VisualizationNode::new("match", &format!("Field: {}", match_q.field))
                    .with_detail("field", Value::String(match_q.field.clone()))
                    .with_detail("query", Value::String(match_q.query.clone()))
            }
            QueryType::MatchPhrase(match_phrase) => {
                VisualizationNode::new("match_phrase", &format!("Field: {}", match_phrase.field))
                    .with_detail("field", Value::String(match_phrase.field.clone()))
                    .with_detail("query", Value::String(match_phrase.query.clone()))
            }
            QueryType::MatchPhrasePrefix(match_phrase_prefix) => VisualizationNode::new(
                "match_phrase_prefix",
                &format!("Field: {}", match_phrase_prefix.field),
            )
            .with_detail("field", Value::String(match_phrase_prefix.field.clone()))
            .with_detail("query", Value::String(match_phrase_prefix.query.clone())),
            QueryType::Bool(bool_query) => {
                let mut node = VisualizationNode::new("bool", "Boolean Query");

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
                let mut node = VisualizationNode::new("range", &format!("Field: {}", range.field))
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
            QueryType::MatchAll => VisualizationNode::new("match_all", "Match All Documents"),
            QueryType::WildCard(wildcard) => {
                VisualizationNode::new("wildcard", &format!("Field: {}", wildcard.field))
                    .with_detail("field", Value::String(wildcard.field.clone()))
                    .with_detail("value", Value::String(wildcard.value.clone()))
            }
        }
    }
}

impl Visualizable for SortType {
    fn visualize(&self) -> VisualizationNode {
        match self {
            SortType::Field(field_sort) => {
                VisualizationNode::new("field_sort", &format!("Field: {}", field_sort.field))
                    .with_detail("field", Value::String(field_sort.field.clone()))
                    .with_detail("order", Value::String(format!("{:?}", field_sort.order)))
            }
            SortType::Score => VisualizationNode::new("score_sort", "Score"),
        }
    }
}

impl Visualizable for AggregationType {
    fn visualize(&self) -> VisualizationNode {
        match self {
            AggregationType::Terms(terms_agg) => {
                let mut node =
                    VisualizationNode::new("terms_agg", &format!("Field: {}", terms_agg.field))
                        .with_detail("field", Value::String(terms_agg.field.clone()));

                if let Some(size) = terms_agg.size {
                    node = node.with_detail("size", Value::Number(size.into()));
                }

                node
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_formatting() {
        let node = VisualizationNode::new("test", "Test Node");

        // Test string formatting
        assert_eq!(
            node.format_value(&Value::String("test".to_string())),
            "\"test\""
        );

        // Test number formatting
        assert_eq!(node.format_value(&Value::Number(42.into())), "42");

        // Test array formatting
        let arr = Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);
        assert_eq!(node.format_value(&arr), "[\"a\", \"b\"]");
    }
}
