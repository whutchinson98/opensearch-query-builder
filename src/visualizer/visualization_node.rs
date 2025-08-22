use std::collections::HashMap;

/// Represents a visualized node from a query.
#[derive(Debug, Clone)]
pub struct VisualizationNode {
    pub node_type: String,
    pub label: String,
    pub details: HashMap<String, serde_json::Value>,
    pub children: Vec<VisualizationNode>,
    pub clause_type: Option<String>,
    pub json: Option<serde_json::Value>,
}

impl VisualizationNode {
    pub fn new(node_type: &str, label: &str) -> Self {
        Self {
            node_type: node_type.to_string(),
            label: label.to_string(),
            details: HashMap::new(),
            children: Vec::new(),
            clause_type: None,
            json: None,
        }
    }

    pub fn with_json(mut self, json: serde_json::Value) -> Self {
        self.json = Some(json);
        self
    }

    pub fn with_detail(mut self, key: &str, value: serde_json::Value) -> Self {
        self.details.insert(key.to_string(), value);
        self
    }

    pub fn with_child(mut self, child: VisualizationNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_clause_type(mut self, clause_type: &str) -> Self {
        self.clause_type = Some(clause_type.to_string());
        self
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn format_value(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => format!("\"{}\"", s),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.format_value(v)).collect();
                format!("[{}]", items.join(", "))
            }
            serde_json::Value::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.format_value(v)))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            serde_json::Value::Null => "null".to_string(),
        }
    }
}
