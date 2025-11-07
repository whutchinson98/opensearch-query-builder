use opensearch_query_builder::*;

fn main() {
    // Build the complex search query with function_score, sorting, aggregations, collapse, and highlighting
    let result = SearchRequest::new()
        .from(0)
        .size(10)
        .track_total_hits(true)
        .query(
            QueryType::function_score()
                .query(
                    QueryType::bool_query()
                        .should(QueryType::MatchPhrasePrefix(
                            MatchPhrasePrefixQuery::new("document_name", "haskell").boost(100.0),
                        ))
                        .should(QueryType::MatchPhrasePrefix(
                            MatchPhrasePrefixQuery::new("content", "haskell").boost(90.0),
                        ))
                        .should(QueryType::Match(
                            MatchQuery::new("document_name", "haskell")
                                .boost(0.1)
                                .minimum_should_match("80%"),
                        ))
                        .should(QueryType::Match(
                            MatchQuery::new("content", "haskell")
                                .boost(0.09)
                                .minimum_should_match("80%"),
                        ))
                        .minimum_should_match(1)
                        .build(),
                )
                .function(
                    ScoreFunction::gauss("updated_at_seconds", "21d")
                        .origin("now")
                        .offset("3d")
                        .decay(0.5)
                        .weight(1.3),
                )
                .score_mode(ScoreMode::Multiply)
                .boost_mode(BoostMode::Multiply)
                .build(),
        )
        .sort(SortType::ScoreWithOrder(ScoreSort::new(SortOrder::Desc)))
        .sort(SortType::Field(FieldSort::new(
            "document_id",
            SortOrder::Asc,
        )))
        .collapse(Collapse::new("document_id"))
        .agg(
            "total_uniques".to_string(),
            AggregationType::Cardinality(CardinalityAggregation::new("document_id")),
        )
        .highlight(
            Highlight::new()
                .require_field_match(true)
                .field(
                    "document_name",
                    HighlightField::new()
                        .highlight_type("unified")
                        .pre_tags(vec!["<macro_em>".to_string()])
                        .post_tags(vec!["</macro_em>".to_string()])
                        .number_of_fragments(1),
                )
                .field(
                    "content",
                    HighlightField::new()
                        .highlight_type("unified")
                        .pre_tags(vec!["<macro_em>".to_string()])
                        .post_tags(vec!["</macro_em>".to_string()])
                        .number_of_fragments(1),
                ),
        );

    let reference = serde_json::json!({
      "from": 0,
      "size": 10,
      "track_total_hits": true,
      "query": {
        "function_score": {
          "query": {
            "bool": {
              "should": [
                {
                  "match_phrase_prefix": {
                    "document_name": {
                      "query": "haskell",
                      "boost": 100.0
                    }
                  }
                },
                {
                  "match_phrase_prefix": {
                    "content": {
                      "query": "haskell",
                      "boost": 90.0
                    }
                  }
                },
                {
                  "match": {
                    "document_name": {
                      "query": "haskell",
                      "boost": 0.1,
                      "minimum_should_match": "80%"
                    }
                  }
                },
                {
                  "match": {
                    "content": {
                      "query": "haskell",
                      "boost": 0.09,
                      "minimum_should_match": "80%"
                    }
                  }
                }
              ],
              "minimum_should_match": 1
            }
          },
          "functions": [
            {
              "gauss": {
                "updated_at_seconds": {
                  "origin": "now",
                  "scale": "21d",
                  "offset": "3d",
                  "decay": 0.5
                }
              },
              "weight": 1.3
            }
          ],
          "score_mode": "multiply",
          "boost_mode": "multiply"
        }
      },
      "sort": [
        {
          "_score": "desc"
        },
        {
          "document_id": "asc"
        }
      ],
      "collapse": {
        "field": "document_id"
      },
      "aggs": {
        "total_uniques": {
          "cardinality": {
            "field": "document_id"
          }
        }
      },
      "highlight": {
        "require_field_match": true,
        "fields": {
          "document_name": {
            "type": "unified",
            "pre_tags": [
              "<macro_em>"
            ],
            "post_tags": [
              "</macro_em>"
            ],
            "number_of_fragments": 1
          },
          "content": {
            "type": "unified",
            "pre_tags": [
              "<macro_em>"
            ],
            "post_tags": [
              "</macro_em>"
            ],
            "number_of_fragments": 1
          }
        }
      }
    });

    assert_eq!(
        serde_json::to_string(&result.to_json()).unwrap(),
        serde_json::to_string(&reference).unwrap()
    );
}
