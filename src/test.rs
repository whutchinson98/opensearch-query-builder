use super::*;

#[test]
fn test_boost_floating_point() {
    let query = QueryType::Match(MatchQuery::new("field", "value").boost(0.1));

    let json = query.to_json();

    assert_eq!(json["match"]["field"]["boost"], 0.1);
}

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
fn test_regexp_query() {
    let query = QueryType::bool_query()
        .must(QueryType::Regexp(RegexpQuery::new(
            "content",
            "test\\&test",
        )))
        .build();
    let json = query.to_json();

    let regexp = serde_json::json!({
        "regexp": {
            "content": {
                "value": "test\\&test"
            }
        }
    });

    assert_eq!(json["bool"]["must"][0], regexp);
}

#[test]
fn test_regexp_query_with_flags() {
    let query = QueryType::bool_query()
        .must(QueryType::Regexp(
            RegexpQuery::new("content", "test\\&test").flags(vec![
                RegexpQueryFlags::Intersection,
                RegexpQueryFlags::Empty,
            ]),
        ))
        .build();
    let json = query.to_json();

    let regexp = serde_json::json!({
        "regexp": {
            "content": {
                "value": "test\\&test",
                "flags": "INTERSECTION|EMPTY",
            }
        }
    });

    assert_eq!(json["bool"]["must"][0], regexp);
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

#[test]
fn test_function_score_with_gauss_decay() {
    let query = QueryType::function_score()
        .query(QueryType::match_query("content", "test"))
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

    let json = query.to_json();

    assert!(json["function_score"]["query"].is_object());
    assert!(json["function_score"]["functions"].is_array());
    assert_eq!(
        json["function_score"]["functions"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(json["function_score"]["score_mode"], "multiply");
    assert_eq!(json["function_score"]["boost_mode"], "multiply");

    let function = &json["function_score"]["functions"][0];
    assert!(function["gauss"]["updated_at_seconds"].is_object());
    assert_eq!(function["gauss"]["updated_at_seconds"]["origin"], "now");
    assert_eq!(function["gauss"]["updated_at_seconds"]["scale"], "21d");
    assert_eq!(function["gauss"]["updated_at_seconds"]["offset"], "3d");
    assert_eq!(function["gauss"]["updated_at_seconds"]["decay"], 0.5);
    assert_eq!(function["weight"], 1.3);
}

#[test]
fn test_function_score_complete_example() {
    // This test matches the exact example provided by the user
    let query = QueryType::function_score()
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

    let json = query.to_json();

    // Verify the structure matches the expected format
    assert!(json["function_score"]["query"]["bool"].is_object());
    assert_eq!(
        json["function_score"]["query"]["bool"]["should"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
    assert_eq!(
        json["function_score"]["query"]["bool"]["minimum_should_match"],
        1
    );

    assert_eq!(
        json["function_score"]["functions"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(json["function_score"]["score_mode"], "multiply");
    assert_eq!(json["function_score"]["boost_mode"], "multiply");

    let function = &json["function_score"]["functions"][0];
    assert_eq!(function["gauss"]["updated_at_seconds"]["origin"], "now");
    assert_eq!(function["gauss"]["updated_at_seconds"]["scale"], "21d");
    assert_eq!(function["gauss"]["updated_at_seconds"]["offset"], "3d");
    assert_eq!(function["gauss"]["updated_at_seconds"]["decay"], 0.5);
    assert_eq!(function["weight"], 1.3);
}

#[test]
fn test_function_score_with_multiple_functions() {
    let query = QueryType::function_score()
        .query(QueryType::match_all())
        .function(
            ScoreFunction::gauss("location", "10km")
                .origin("40.0,-74.0")
                .decay(0.33),
        )
        .function(ScoreFunction::field_value_factor("popularity").factor(1.2))
        .function(ScoreFunction::random_score().seed(12345))
        .score_mode(ScoreMode::Sum)
        .boost_mode(BoostMode::Replace)
        .build();

    let json = query.to_json();

    assert_eq!(
        json["function_score"]["functions"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(json["function_score"]["score_mode"], "sum");
    assert_eq!(json["function_score"]["boost_mode"], "replace");

    // Check gauss function
    let func1 = &json["function_score"]["functions"][0];
    assert!(func1["gauss"]["location"].is_object());

    // Check field_value_factor
    let func2 = &json["function_score"]["functions"][1];
    assert_eq!(func2["field_value_factor"]["field"], "popularity");
    assert_eq!(func2["field_value_factor"]["factor"], 1.2);

    // Check random_score
    let func3 = &json["function_score"]["functions"][2];
    assert_eq!(func3["random_score"]["seed"], 12345);
}

#[test]
fn test_function_score_with_filter() {
    let query = QueryType::function_score()
        .query(QueryType::match_all())
        .function(ScoreFunction::weight_only(2.0).filter(QueryType::term("category", "premium")))
        .build();

    let json = query.to_json();

    let function = &json["function_score"]["functions"][0];
    assert_eq!(function["weight"], 2.0);
    assert!(function["filter"]["term"]["category"].is_string());
}

#[test]
fn test_function_score_exp_decay() {
    let query = QueryType::function_score()
        .function(ScoreFunction::exp("date", "30d").origin("2024-01-01"))
        .build();

    let json = query.to_json();

    let function = &json["function_score"]["functions"][0];
    assert!(function["exp"]["date"].is_object());
    assert_eq!(function["exp"]["date"]["origin"], "2024-01-01");
    assert_eq!(function["exp"]["date"]["scale"], "30d");
}

#[test]
fn test_function_score_linear_decay() {
    let query = QueryType::function_score()
        .function(ScoreFunction::linear("price", "100").origin(50))
        .build();

    let json = query.to_json();

    let function = &json["function_score"]["functions"][0];
    assert!(function["linear"]["price"].is_object());
    assert_eq!(function["linear"]["price"]["origin"], 50);
    assert_eq!(function["linear"]["price"]["scale"], "100");
}

#[test]
fn test_function_score_with_max_boost() {
    let query = QueryType::function_score()
        .query(QueryType::match_all())
        .function(ScoreFunction::weight_only(2.0))
        .max_boost(10.0)
        .build();

    let json = query.to_json();

    assert_eq!(json["function_score"]["max_boost"], 10.0);
}

#[test]
fn test_function_score_with_min_score() {
    let query = QueryType::function_score()
        .query(QueryType::match_all())
        .function(ScoreFunction::weight_only(1.5))
        .min_score(0.5)
        .build();

    let json = query.to_json();

    assert_eq!(json["function_score"]["min_score"], 0.5);
}

#[test]
fn test_function_score_script_score() {
    let query = QueryType::function_score()
        .function(ScoreFunction::script_score("_score * doc['likes'].value"))
        .build();

    let json = query.to_json();

    let function = &json["function_score"]["functions"][0];
    assert_eq!(
        function["script_score"]["script"]["source"],
        "_score * doc['likes'].value"
    );
}

#[test]
fn test_function_score_all_modes() {
    // Test all score modes
    let modes = vec![
        (ScoreMode::Multiply, "multiply"),
        (ScoreMode::Sum, "sum"),
        (ScoreMode::Avg, "avg"),
        (ScoreMode::First, "first"),
        (ScoreMode::Max, "max"),
        (ScoreMode::Min, "min"),
    ];

    for (mode, expected) in modes {
        let query = QueryType::function_score()
            .function(ScoreFunction::weight_only(1.0))
            .score_mode(mode)
            .build();

        let json = query.to_json();
        assert_eq!(json["function_score"]["score_mode"], expected);
    }

    // Test all boost modes
    let boost_modes = vec![
        (BoostMode::Multiply, "multiply"),
        (BoostMode::Replace, "replace"),
        (BoostMode::Sum, "sum"),
        (BoostMode::Avg, "avg"),
        (BoostMode::Max, "max"),
        (BoostMode::Min, "min"),
    ];

    for (mode, expected) in boost_modes {
        let query = QueryType::function_score()
            .function(ScoreFunction::weight_only(1.0))
            .boost_mode(mode)
            .build();

        let json = query.to_json();
        assert_eq!(json["function_score"]["boost_mode"], expected);
    }
}

#[test]
fn test_search_request_builder_dynamic() {
    // Create a builder
    let mut builder = SearchRequestBuilder::new();

    // Dynamically add fields based on conditions
    let include_query = true;
    let page = 2;
    let page_size = 20;

    if include_query {
        builder.query(QueryType::match_query("title", "rust programming"));
    }

    // Add pagination
    builder.from(page * page_size);
    builder.size(page_size);

    // Conditionally add sorting
    let sort_by_date = true;
    if sort_by_date {
        builder.add_sort(SortType::Field(FieldSort::new("created_at", SortOrder::Desc)));
    }

    // Add multiple source fields
    builder.add_source_field("title".to_string());
    builder.add_source_field("content".to_string());
    builder.add_source_field("created_at".to_string());

    // Add aggregations dynamically
    let categories = vec!["tech", "science"];
    for category in categories {
        builder.add_agg(
            format!("{}_count", category),
            AggregationType::Terms(TermsAggregation::new(category)),
        );
    }

    // Build the final request
    let request = builder.build();

    // Verify the request
    let json = request.to_json();
    assert!(json["query"].is_object());
    assert_eq!(json["size"], Value::Number(20.into()));
    assert_eq!(json["from"], Value::Number(40.into()));
    assert_eq!(json["_source"].as_array().unwrap().len(), 3);
    assert!(json["sort"].is_array());
    assert!(json["aggs"]["tech_count"].is_object());
    assert!(json["aggs"]["science_count"].is_object());
}

#[test]
fn test_search_request_builder_mutable_updates() {
    // Create a builder
    let mut builder = SearchRequestBuilder::new();

    // Add initial query
    builder.query(QueryType::match_query("title", "rust"));

    // Add some sorts
    builder.add_sort(SortType::Field(FieldSort::new("score", SortOrder::Desc)));
    builder.add_sort(SortType::Field(FieldSort::new("date", SortOrder::Desc)));

    // Change mind - clear sorts and add a different one
    builder.clear_sorts();
    builder.add_sort(SortType::Field(FieldSort::new("relevance", SortOrder::Asc)));

    // Add aggregations
    builder.add_agg(
        "categories".to_string(),
        AggregationType::Terms(TermsAggregation::new("category")),
    );
    builder.add_agg(
        "authors".to_string(),
        AggregationType::Terms(TermsAggregation::new("author")),
    );

    // Remove one aggregation
    builder.remove_agg("authors");

    // Build and verify
    let request = builder.build();
    let json = request.to_json();

    assert_eq!(json["sort"].as_array().unwrap().len(), 1);
    assert!(json["aggs"]["categories"].is_object());
    assert!(json["aggs"]["authors"].is_null());
}

#[test]
fn test_search_request_builder_fluent_style() {
    // The builder also supports fluent-style chaining since methods return &mut self
    let mut builder = SearchRequestBuilder::new();

    builder
        .query(QueryType::match_query("content", "opensearch"))
        .size(50)
        .from(0)
        .track_total_hits(true)
        .add_source_field("id".to_string())
        .add_source_field("title".to_string());

    let request = builder.build();
    let json = request.to_json();

    assert_eq!(json["size"], Value::Number(50.into()));
    assert_eq!(json["track_total_hits"], Value::Bool(true));
    assert_eq!(json["_source"].as_array().unwrap().len(), 2);
}
