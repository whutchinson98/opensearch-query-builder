use serde::{Deserialize, Serialize};

mod bool;
mod function_score;
mod match_phrase;
mod match_phrase_prefix;
mod match_query;
mod range;
mod regexp;
mod term;
mod terms;
mod wildcard;

pub use bool::*;
pub use function_score::*;
pub use match_phrase::*;
pub use match_phrase_prefix::*;
pub use match_query::*;
pub use range::*;
pub use regexp::*;
use serde_json::Value;
pub use term::*;
pub use terms::*;
pub use wildcard::*;

use crate::ToOpenSearchJson;

/// Enum representing the different types of queries that can be used in a search request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum QueryType {
    /// Bool query
    Bool(BoolQuery),
    /// Function score query
    FunctionScore(FunctionScoreQuery),
    /// Match phrase query
    MatchPhrase(MatchPhraseQuery),
    /// Match phrase prefix query
    MatchPhrasePrefix(MatchPhrasePrefixQuery),
    /// Match query
    Match(MatchQuery),
    /// Range query
    Range(RangeQuery),
    /// Regexp query
    Regexp(RegexpQuery),
    /// Term query
    Term(TermQuery),
    /// Terms query
    Terms(TermsQuery),
    /// Wildcard query
    WildCard(WildcardQuery),
}

impl ToOpenSearchJson for QueryType {
    fn to_json(&self) -> Value {
        match self {
            QueryType::Bool(bool_query) => bool_query.to_json(),
            QueryType::FunctionScore(function_score) => function_score.to_json(),
            QueryType::MatchPhrase(match_phrase) => match_phrase.to_json(),
            QueryType::MatchPhrasePrefix(match_phrase_prefix) => match_phrase_prefix.to_json(),
            QueryType::Match(match_query) => match_query.to_json(),
            QueryType::Term(term) => term.to_json(),
            QueryType::Terms(terms) => terms.to_json(),
            QueryType::Range(range) => range.to_json(),
            QueryType::WildCard(wildcard_query) => wildcard_query.to_json(),
            QueryType::Regexp(regexp_query) => regexp_query.to_json(),
        }
    }
}

impl QueryType {
    /// Convenience method for creating a term query
    pub fn term<T: Into<Value>>(field: &str, value: T) -> Self {
        QueryType::Term(TermQuery::new(field, value))
    }

    /// Convenience method for creating a terms query
    pub fn terms<T: Into<Value>>(field: &str, values: Vec<T>) -> Self {
        QueryType::Terms(TermsQuery::new(field, values))
    }

    /// Convenience method for creating a wildcard query
    pub fn wildcard(field: &str, value: &str, case_insensitive: bool) -> Self {
        QueryType::WildCard(WildcardQuery::new(field, value, case_insensitive))
    }

    /// Convenience method for creating a regexp query
    pub fn regexp(field: &str, value: &str) -> Self {
        QueryType::Regexp(RegexpQuery::new(field, value))
    }

    /// Convenience method for creating a match query
    pub fn match_phrase(field: &str, query: &str) -> Self {
        QueryType::MatchPhrase(MatchPhraseQuery::new(field, query))
    }

    /// Convenience method for creating a match phrase prefix query
    pub fn match_phrase_prefix(field: &str, query: &str) -> Self {
        QueryType::MatchPhrasePrefix(MatchPhrasePrefixQuery::new(field, query))
    }

    /// Convenience method for starting a bool query
    pub fn bool_query() -> BoolQueryBuilder {
        BoolQueryBuilder::new()
    }

    /// Convenience method for starting a match query
    pub fn range(field: &str) -> RangeQueryBuilder {
        RangeQueryBuilder::new(field)
    }

    /// Convenience method for starting a function score query
    pub fn function_score() -> FunctionScoreQueryBuilder {
        FunctionScoreQueryBuilder::new()
    }
}
