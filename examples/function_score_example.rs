use opensearch_query_builder::*;

fn main() {
    // Example 1: Simple function_score with gauss decay
    println!("=== Example 1: Function Score with Gauss Decay ===");
    let query1 = QueryType::function_score()
        .query(QueryType::match_query("content", "rust programming"))
        .function(
            ScoreFunction::gauss("updated_at_seconds", "21d")
                .origin("now")
                .offset("3d")
                .decay(0.5)
                .weight(1.3),
        )
        .score_mode(ScoreMode::Multiply)
        .boost_mode(BoostMode::Multiply)
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&query1.to_json()).unwrap()
    );
    println!("\n");

    // Example 2: Complete example matching the user's use case
    println!("=== Example 2: Complete Search with Function Score ===");
    let query2 = QueryType::function_score()
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
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&query2.to_json()).unwrap()
    );
    println!("\n");

    // Example 3: Multiple scoring functions
    println!("=== Example 3: Multiple Scoring Functions ===");
    let query3 = QueryType::function_score()
        .query(QueryType::match_all())
        .function(
            ScoreFunction::gauss("location", "10km")
                .origin("40.0,-74.0")
                .decay(0.33)
                .weight(2.0),
        )
        .function(ScoreFunction::field_value_factor("popularity").factor(1.5))
        .function(ScoreFunction::random_score().seed(12345))
        .score_mode(ScoreMode::Sum)
        .boost_mode(BoostMode::Multiply)
        .max_boost(20.0)
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&query3.to_json()).unwrap()
    );
    println!("\n");

    // Example 4: Function with filter
    println!("=== Example 4: Function Score with Filter ===");
    let query4 = QueryType::function_score()
        .query(QueryType::match_query("title", "search"))
        .function(ScoreFunction::weight_only(3.0).filter(QueryType::term("category", "premium")))
        .function(ScoreFunction::weight_only(1.5).filter(QueryType::term("category", "featured")))
        .score_mode(ScoreMode::Max)
        .boost_mode(BoostMode::Multiply)
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&query4.to_json()).unwrap()
    );
    println!("\n");

    // Example 5: Exponential and linear decay functions
    println!("=== Example 5: Exp and Linear Decay Functions ===");
    let query5 = QueryType::function_score()
        .query(QueryType::match_all())
        .function(
            ScoreFunction::exp("date", "30d")
                .origin("2024-01-01")
                .decay(0.5),
        )
        .function(ScoreFunction::linear("price", "100").origin(50).weight(0.8))
        .score_mode(ScoreMode::Avg)
        .boost_mode(BoostMode::Sum)
        .build();

    println!(
        "{}",
        serde_json::to_string_pretty(&query5.to_json()).unwrap()
    );
    println!("\n");

    // Example 6: Using in a SearchRequest
    println!("=== Example 6: Full SearchRequest with Function Score ===");
    let search_request = SearchRequest::new()
        .query(
            QueryType::function_score()
                .query(QueryType::match_query("content", "opensearch"))
                .function(
                    ScoreFunction::gauss("timestamp", "7d")
                        .origin("now")
                        .decay(0.5)
                        .weight(2.0),
                )
                .score_mode(ScoreMode::Multiply)
                .boost_mode(BoostMode::Multiply)
                .build(),
        )
        .size(20)
        .from(0);

    println!(
        "{}",
        serde_json::to_string_pretty(&search_request.to_json()).unwrap()
    );
}
