use super::*;

#[test]
fn test_value_formatting() {
    let node = VisualizationNode::new("test");

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
