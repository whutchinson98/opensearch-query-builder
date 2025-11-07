use opensearch_query_builder::*;

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
                    .post_tags(vec!["</macro_em>".to_string()]),
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
                "updated_at": "desc",
            },
            {
                "document_id": "asc",
            },
            {
                "node_id": "asc",
            },
        ],
        "highlight": {
            "fields": {
                "content": {
                    "type": "unified", // The way the highlight is done
                    "number_of_fragments": 500, // Breaks up the "content" field into said
                    "pre_tags": ["<macro_em>"], // HTML tag before highlight
                    "post_tags": ["</macro_em>"], // HTML tag after highlight
                }
            }
        },
    });

    assert_eq!(result.to_json(), reference);

    // println!("{}", result.to_json());
}
