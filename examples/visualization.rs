use opensearch_query_builder::Visualizable;
use opensearch_query_builder::*;
use std::fs;
use std::path::Path;

pub fn main() {
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
                        .should(QueryType::Regexp(
                            RegexpQuery::new("content", "abc.*").flags(vec![
                                RegexpQueryFlags::Intersection,
                                RegexpQueryFlags::Empty,
                            ]),
                        ))
                        .minimum_should_match(1)
                        .build(),
                )
                .must(QueryType::terms("file_type", file_types.clone()))
                .should(QueryType::term("owner_id", user_id))
                .should(QueryType::terms("RANDOM", Vec::<String>::new()))
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

    // Create output directory
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Failed to create output directory");
        println!("Created output directory: {}", output_dir.display());
    }

    // Generate and save HTML visualization
    match result.visualize().generate_html() {
        Ok(html_content) => {
            let html_path = output_dir.join("query_visualization.html");
            fs::write(&html_path, html_content).expect("Failed to write HTML file");
            println!("Generated HTML visualization: {}", html_path.display());
        }
        Err(e) => {
            eprintln!("Failed to generate HTML visualization: {e}");
        }
    }

    println!(
        "Open output/query_visualization.html in your browser to view the interactive visualization"
    );
}
