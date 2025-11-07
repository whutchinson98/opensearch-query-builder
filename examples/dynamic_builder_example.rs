use opensearch_query_builder::*;

fn main() {
    // Example 1: Building a search request dynamically based on user input
    println!("=== Example 1: Dynamic Search Request ===\n");

    // Simulate user input
    let search_text = Some("rust programming");
    let filter_category = Some("tutorial");
    let page_number = 2;
    let results_per_page = 20;
    let include_highlights = true;

    // Create a mutable builder
    let mut builder = SearchRequestBuilder::new();

    // Conditionally add query based on whether user provided search text
    if let Some(text) = search_text {
        builder.query(
            QueryType::bool_query()
                .must(QueryType::match_query("content", text))
                .build()
        );
    }

    // Add pagination
    builder.from(page_number * results_per_page);
    builder.size(results_per_page);

    // Conditionally add filter
    if let Some(category) = filter_category {
        // If we already have a query, we need to rebuild it with the filter
        // For simplicity, let's just add it as a must clause
        builder.query(
            QueryType::bool_query()
                .must(QueryType::match_query("content", "rust programming"))
                .filter(QueryType::term("category", category))
                .build()
        );
    }

    // Conditionally add highlighting
    if include_highlights {
        builder.highlight(
            Highlight::new()
                .field("content", HighlightField::new())
                .field("title", HighlightField::new())
        );
    }

    // Add source fields to return
    builder.add_source_field("id".to_string());
    builder.add_source_field("title".to_string());
    builder.add_source_field("content".to_string());
    builder.add_source_field("category".to_string());

    // Build the final request
    let request = builder.build();

    println!("Generated OpenSearch Query:");
    println!("{}", serde_json::to_string_pretty(&request.to_json()).unwrap());
    println!("\n");

    // Example 2: Iteratively building a request
    println!("=== Example 2: Iterative Builder ===\n");

    let mut builder2 = SearchRequestBuilder::new();

    // Start with basic query
    builder2.query(QueryType::match_all());
    builder2.size(10);

    // Add sorting
    builder2.add_sort(SortType::Field(FieldSort::new("created_at", SortOrder::Desc)));

    // Add first aggregation
    builder2.add_agg(
        "by_category".to_string(),
        AggregationType::Terms(TermsAggregation::new("category"))
    );

    // Later, add another aggregation
    builder2.add_agg(
        "by_author".to_string(),
        AggregationType::Terms(TermsAggregation::new("author"))
    );

    // Change your mind about one aggregation
    builder2.remove_agg("by_author");

    // Add a different aggregation instead
    builder2.add_agg(
        "unique_authors".to_string(),
        AggregationType::Cardinality(CardinalityAggregation::new("author"))
    );

    let request2 = builder2.build();

    println!("Generated OpenSearch Query:");
    println!("{}", serde_json::to_string_pretty(&request2.to_json()).unwrap());
    println!("\n");

    // Example 3: Using fluent style with the builder
    println!("=== Example 3: Fluent-Style Builder ===\n");

    let mut builder3 = SearchRequestBuilder::new();

    // The builder supports chaining since methods return &mut self
    builder3
        .query(QueryType::match_query("title", "opensearch"))
        .size(50)
        .from(0)
        .track_total_hits(true)
        .add_sort(SortType::Score)
        .add_source_field("id".to_string())
        .add_source_field("title".to_string())
        .add_source_field("score".to_string());

    let request3 = builder3.build();

    println!("Generated OpenSearch Query:");
    println!("{}", serde_json::to_string_pretty(&request3.to_json()).unwrap());
}
