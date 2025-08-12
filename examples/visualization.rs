use opensearch_query_builder::Visualizable;
use opensearch_query_builder::*;
use std::fs;
use std::path::Path;

pub fn main() {
    let query_key = "match_phrase";
    let file_types = vec!["pdf".to_string(), "docx".to_string()];
    let user_id = "user";
    let page = 1;
    let page_size = 2;
    let from = page * page_size;
    let query = "testing";
    let result = SearchRequest::default()
        .query(
            QueryType::bool_query()
                .must(
                    QueryType::bool_query()
                        .should(QueryType::match_phrase("content", query))
                        .should(QueryType::match_phrase_prefix("document_name", query))
                        .minimum_should_match(1)
                        .build(),
                )
                .must(QueryType::terms("file_type", file_types.clone()))
                .should(QueryType::term("owner_id", user_id))
                .should(QueryType::terms("document_id", Vec::<String>::new()))
                .minimum_should_match(1)
                .build(),
        )
        .from(from)
        .size(page_size)
        .sort(SortType::Field(FieldSort::new(
            "updated_at",
            SortOrder::Desc,
        )))
        .sort(SortType::Field(FieldSort::new(
            "document_id",
            SortOrder::Asc,
        )))
        .sort(SortType::Field(FieldSort::new("node_id", SortOrder::Asc)))
        .highlight(
            Highlight::new().field(
                "content",
                HighlightField::new()
                    .highlight_type("unified")
                    .number_of_fragments(500)
                    .pre_tags(vec!["<macro_em>".to_string()])
                    .post_tags(vec!["</macro_em>".to_string()])
                    .require_field_match(true),
            ),
        );

    let reference = serde_json::json!({
        "query": {
            "bool": {
                "must": [
                    {
                        "bool": {
                            "should": [
                                {
                                    query_key: {
                                        "content": query
                                    }
                                },
                                {
                                    "match_phrase_prefix": {
                                        "document_name": {
                                            "query": query,
                                        }
                                    }
                                }
                            ],
                            "minimum_should_match": 1
                        }
                    },
                    {
                        "terms": {
                            "file_type": file_types
                        }
                    }
                ],
                    "should": [
                    {
                        "term": {
                            "owner_id": user_id
                        }
                    },
                    {
                        "terms": {
                            "document_id": Vec::<String>::new()
                        }
                    },
                ],
                "minimum_should_match": 1
            }
        },
        "from": from,
        "size": page_size,
        "sort":  [
            {
                "updated_at": {
                    "order": "desc"
                }
            },
            {
                "document_id": {
                    "order": "asc"
                }
            },
            {
                "node_id": {
                    "order": "asc"
                }
            },
        ],
        "highlight": {
            "fields": {
                "content": {
                    "type": "unified", // The way the highlight is done
                    "number_of_fragments": 500, // Breaks up the "content" field into said
                    "pre_tags": ["<macro_em>"], // HTML tag before highlight
                    "post_tags": ["</macro_em>"], // HTML tag after highlight
                    "require_field_match": true, // Default is true, but good to be explicit
                }
            }
        },
    });

    assert_eq!(result.to_json(), reference);

    // Create output directory
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Failed to create output directory");
        println!("Created output directory: {}", output_dir.display());
    }

    // Generate and save HTML visualization
    match result.generate_html() {
        Ok(html_content) => {
            let html_path = output_dir.join("query_visualization.html");
            fs::write(&html_path, html_content).expect("Failed to write HTML file");
            println!("Generated HTML visualization: {}", html_path.display());
        }
        Err(e) => {
            eprintln!("Failed to generate HTML visualization: {e}");
        }
    }

    // Generate and save SVG visualization
    match result.generate_svg() {
        Ok(svg_content) => {
            let svg_path = output_dir.join("query_diagram.svg");
            fs::write(&svg_path, svg_content).expect("Failed to write SVG file");
            println!("Generated SVG diagram: {}", svg_path.display());
        }
        Err(e) => {
            eprintln!("Failed to generate SVG visualization: {e}");
        }
    }

    println!("Query visualization files saved to output/ directory");
    println!(
        "Open output/query_visualization.html in your browser to view the interactive visualization"
    );
}
