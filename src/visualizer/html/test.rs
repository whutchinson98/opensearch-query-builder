use crate::{QueryType, SearchRequest, Visualizable};

use super::*;
#[test]
fn test_html_visualization() {
    let request = SearchRequest::new()
        .query(
            QueryType::bool_query()
                .must(QueryType::term("status", "published"))
                .should(QueryType::match_query("title", "rust"))
                .build(),
        )
        .size(10);

    // Test HTML generation
    let html_result = request.visualize().generate_html();
    assert!(html_result.is_ok());
    let html = html_result.unwrap();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Query Structure"));
    assert!(html.contains("BOOL"));
}
