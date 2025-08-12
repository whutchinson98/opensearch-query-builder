use crate::{AggregationType, QueryType, SearchRequest, SortType, ToOpenSearchJson};
use serde_json::Value;
use std::collections::HashMap;

use askama::Template;
use uuid::Uuid;

pub trait Visualizable {
    fn generate_html(&self) -> Result<String, VisualizationError>;
    fn generate_svg(&self) -> Result<String, VisualizationError>;
    fn generate_interactive_view(&self) -> Result<String, VisualizationError>;
}

#[derive(Debug)]
pub enum VisualizationError {
    TemplateError(String),
    SerializationError(String),
}

impl std::fmt::Display for VisualizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualizationError::TemplateError(msg) => write!(f, "Template error: {msg}"),
            VisualizationError::SerializationError(msg) => {
                write!(f, "Serialization error: {msg}")
            }
        }
    }
}

impl std::error::Error for VisualizationError {}

#[cfg(feature = "visualizer")]
#[derive(Template)]
#[template(
    source = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>OpenSearch Query Visualization</title>
    <style>
        body {
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            margin: 0;
            padding: 20px;
            background: #1e1e1e;
            color: #d4d4d4;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
        }
        .panel {
            background: #252526;
            border: 1px solid #3e3e42;
            border-radius: 8px;
            padding: 20px;
        }
        .title {
            font-size: 18px;
            font-weight: bold;
            margin-bottom: 15px;
            color: #569cd6;
            border-bottom: 1px solid #3e3e42;
            padding-bottom: 10px;
        }
        .query-tree {
            font-size: 14px;
            line-height: 1.5;
        }
        .node {
            margin: 5px 0;
            padding: 8px;
            border-left: 3px solid #4ec9b0;
            background: #2d2d30;
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.2s ease;
        }
        .node:hover {
            background: #333337;
            border-left-color: #dcdcaa;
        }
        .node.bool { border-left-color: #ce9178; }
        .node.term { border-left-color: #9cdcfe; }
        .node.match { border-left-color: #c586c0; }
        .node.range { border-left-color: #4fc1ff; }
        .node.wildcard { border-left-color: #f9e79f; }
        .children {
            margin-left: 20px;
            border-left: 1px dashed #3e3e42;
            padding-left: 10px;
        }
        .json-view {
            background: #1e1e1e;
            border: 1px solid #3e3e42;
            border-radius: 4px;
            padding: 15px;
            overflow: auto;
            max-height: 500px;
            font-size: 12px;
        }
        .key { color: #9cdcfe; }
        .string { color: #ce9178; }
        .number { color: #b5cea8; }
        .boolean { color: #569cd6; }
        .null { color: #808080; }
        .controls {
            margin-bottom: 15px;
        }
        .btn {
            background: #0e639c;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 4px;
            cursor: pointer;
            margin-right: 10px;
            font-size: 12px;
        }
        .btn:hover {
            background: #1177bb;
        }
        .expanded { display: block; }
        .collapsed { display: none; }
        .toggle {
            color: #569cd6;
            cursor: pointer;
            user-select: none;
            font-weight: bold;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="panel">
            <div class="title">Query Structure</div>
            <div class="controls">
                <button class="btn" onclick="expandAll()">Expand All</button>
                <button class="btn" onclick="collapseAll()">Collapse All</button>
            </div>
            <div class="query-tree">
                {{ tree_html|safe }}
            </div>
        </div>
        <div class="panel">
            <div class="title">Generated JSON</div>
            <div class="controls">
                <button class="btn" onclick="copyJson()">Copy JSON</button>
                <button class="btn" onclick="prettifyJson()">Prettify</button>
            </div>
            <pre class="json-view" id="json-output">{{ json_pretty|safe }}</pre>
        </div>
    </div>

    <script>
        function toggleNode(element) {
            const children = element.nextElementSibling;
            if (children && children.classList.contains('children')) {
                children.classList.toggle('collapsed');
                children.classList.toggle('expanded');
                const toggle = element.querySelector('.toggle');
                if (toggle) {
                    toggle.textContent = children.classList.contains('collapsed') ? '▶' : '▼';
                }
            }
        }

        function expandAll() {
            document.querySelectorAll('.children').forEach(el => {
                el.classList.remove('collapsed');
                el.classList.add('expanded');
            });
            document.querySelectorAll('.toggle').forEach(el => {
                el.textContent = '▼';
            });
        }

        function collapseAll() {
            document.querySelectorAll('.children').forEach(el => {
                el.classList.remove('expanded');
                el.classList.add('collapsed');
            });
            document.querySelectorAll('.toggle').forEach(el => {
                el.textContent = '▶';
            });
        }

        function copyJson() {
            const jsonText = document.getElementById('json-output').textContent;
            navigator.clipboard.writeText(jsonText).then(() => {
                alert('JSON copied to clipboard!');
            });
        }

        function prettifyJson() {
            const jsonElement = document.getElementById('json-output');
            try {
                const parsed = JSON.parse(jsonElement.textContent);
                jsonElement.innerHTML = syntaxHighlight(JSON.stringify(parsed, null, 2));
            } catch (e) {
                console.error('Invalid JSON:', e);
            }
        }

        function syntaxHighlight(json) {
            json = json.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
            return json.replace(/("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g, function (match) {
                var cls = 'number';
                if (/^"/.test(match)) {
                    if (/:$/.test(match)) {
                        cls = 'key';
                    } else {
                        cls = 'string';
                    }
                } else if (/true|false/.test(match)) {
                    cls = 'boolean';
                } else if (/null/.test(match)) {
                    cls = 'null';
                }
                return '<span class="' + cls + '">' + match + '</span>';
            });
        }

        // Initialize with syntax highlighting
        document.addEventListener('DOMContentLoaded', function() {
            prettifyJson();
        });
    </script>
</body>
</html>
"#,
    ext = "html"
)]
struct QueryVisualizationTemplate {
    tree_html: String,
    json_pretty: String,
}

#[derive(Debug, Clone)]
pub struct VisualizationNode {
    #[allow(dead_code)]
    pub id: String,
    pub node_type: String,
    pub label: String,
    pub details: HashMap<String, Value>,
    pub children: Vec<VisualizationNode>,
}

impl VisualizationNode {
    fn new(node_type: &str, label: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            node_type: node_type.to_string(),
            label: label.to_string(),
            details: HashMap::new(),
            children: Vec::new(),
        }
    }

    fn with_detail(mut self, key: &str, value: Value) -> Self {
        self.details.insert(key.to_string(), value);
        self
    }

    fn with_child(mut self, child: VisualizationNode) -> Self {
        self.children.push(child);
        self
    }

    fn to_html(&self) -> String {
        let mut html = format!(
            r#"<div class="node {}" onclick="toggleNode(this)">
                <span class="toggle">▼</span> 
                <strong>{}</strong> - {}
            </div>"#,
            self.node_type,
            self.node_type.to_uppercase(),
            self.label
        );

        if !self.children.is_empty() {
            html.push_str(r#"<div class="children expanded">"#);
            for child in &self.children {
                html.push_str(&child.to_html());
            }
            html.push_str("</div>");
        }

        html
    }
}

impl Visualizable for SearchRequest {
    fn generate_html(&self) -> Result<String, VisualizationError> {
        let tree = self.build_visualization_tree();
        let tree_html = tree.to_html();
        let json_value = self.to_json();
        let json_pretty = serde_json::to_string_pretty(&json_value)
            .map_err(|e| VisualizationError::SerializationError(e.to_string()))?;

        let template = QueryVisualizationTemplate {
            tree_html,
            json_pretty,
        };

        template
            .render()
            .map_err(|e| VisualizationError::TemplateError(e.to_string()))
    }

    fn generate_svg(&self) -> Result<String, VisualizationError> {
        let tree = self.build_visualization_tree();
        Ok(self.generate_svg_diagram(&tree))
    }

    fn generate_interactive_view(&self) -> Result<String, VisualizationError> {
        self.generate_html()
    }
}

impl SearchRequest {
    fn build_visualization_tree(&self) -> VisualizationNode {
        let mut root = VisualizationNode::new("search", "SearchRequest");

        if let Some(ref query) = self.query {
            root = root.with_child(query.build_visualization_node());
        }

        if let Some(size) = self.size {
            root = root.with_detail("size", Value::Number(size.into()));
        }

        if let Some(from) = self.from {
            root = root.with_detail("from", Value::Number(from.into()));
        }

        if !self.sort.is_empty() {
            let mut sort_node = VisualizationNode::new("sort", "Sort");
            for sort in &self.sort {
                sort_node = sort_node.with_child(sort.build_visualization_node());
            }
            root = root.with_child(sort_node);
        }

        if !self.aggs.is_empty() {
            let mut aggs_node = VisualizationNode::new("aggregations", "Aggregations");
            for (name, agg) in &self.aggs {
                let mut agg_node = agg.build_visualization_node();
                agg_node.label = format!("{}: {}", name, agg_node.label);
                aggs_node = aggs_node.with_child(agg_node);
            }
            root = root.with_child(aggs_node);
        }

        if !self._source.is_empty() {
            root = root.with_detail(
                "_source",
                Value::Array(
                    self._source
                        .iter()
                        .map(|s| Value::String(s.clone()))
                        .collect(),
                ),
            );
        }

        root
    }

    fn generate_svg_diagram(&self, tree: &VisualizationNode) -> String {
        let mut svg = String::new();
        svg.push_str(r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">"#);
        svg.push_str(
            r#"<defs>
            <style>
                .node-rect { fill: #2d3748; stroke: #4a5568; stroke-width: 1; rx: 5; }
                .node-text { fill: #e2e8f0; font-family: monospace; font-size: 12px; }
                .edge-line { stroke: #718096; stroke-width: 1.5; }
                .bool-node { fill: #c53030; }
                .term-node { fill: #3182ce; }
                .match-node { fill: #805ad5; }
                .range-node { fill: #319795; }
                .wildcard-node { fill: #d69e2e; }
            </style>
        </defs>"#,
        );

        self.render_svg_node(tree, &mut svg, 400, 50, 0);

        svg.push_str("</svg>");
        svg
    }

    #[allow(clippy::only_used_in_recursion)]
    fn render_svg_node(
        &self,
        node: &VisualizationNode,
        svg: &mut String,
        x: i32,
        y: i32,
        level: i32,
    ) {
        let class = match node.node_type.as_str() {
            "bool" => "bool-node",
            "term" => "term-node",
            "match" => "match-node",
            "range" => "range-node",
            "wildcard" => "wildcard-node",
            _ => "node-rect",
        };

        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="150" height="40" class="node-rect {}"/>"#,
            x - 75,
            y - 20,
            class
        ));

        svg.push_str(&format!(
            r#"<text x="{}" y="{}" class="node-text" text-anchor="middle">{}</text>"#,
            x,
            y + 5,
            node.label
        ));

        let child_y = y + 80;
        let child_count = node.children.len() as i32;
        if child_count > 0 {
            let child_spacing = 200;
            let start_x = x - (child_count - 1) * child_spacing / 2;

            for (i, child) in node.children.iter().enumerate() {
                let child_x = start_x + i as i32 * child_spacing;

                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" class="edge-line"/>"#,
                    x,
                    y + 20,
                    child_x,
                    child_y - 20
                ));

                self.render_svg_node(child, svg, child_x, child_y, level + 1);
            }
        }
    }
}

impl QueryType {
    fn build_visualization_node(&self) -> VisualizationNode {
        match self {
            QueryType::Term(term) => {
                VisualizationNode::new("term", &format!("Term: {}", term.field))
                    .with_detail("field", Value::String(term.field.clone()))
                    .with_detail("value", term.value.clone())
            }
            QueryType::Terms(terms) => {
                VisualizationNode::new("terms", &format!("Terms: {}", terms.field))
                    .with_detail("field", Value::String(terms.field.clone()))
                    .with_detail("values", Value::Array(terms.values.clone()))
            }
            QueryType::Match(match_q) => {
                VisualizationNode::new("match", &format!("Match: {}", match_q.field))
                    .with_detail("field", Value::String(match_q.field.clone()))
                    .with_detail("query", Value::String(match_q.query.clone()))
            }
            QueryType::MatchPhrase(match_phrase) => VisualizationNode::new(
                "match_phrase",
                &format!("MatchPhrase: {}", match_phrase.field),
            )
            .with_detail("field", Value::String(match_phrase.field.clone()))
            .with_detail("query", Value::String(match_phrase.query.clone())),
            QueryType::MatchPhrasePrefix(match_phrase_prefix) => VisualizationNode::new(
                "match_phrase_prefix",
                &format!("MatchPhrasePrefix: {}", match_phrase_prefix.field),
            )
            .with_detail("field", Value::String(match_phrase_prefix.field.clone()))
            .with_detail("query", Value::String(match_phrase_prefix.query.clone())),
            QueryType::Bool(bool_query) => {
                let mut node = VisualizationNode::new("bool", "Bool Query");

                for must in &bool_query.must {
                    node = node.with_child(
                        must.build_visualization_node()
                            .with_detail("clause_type", Value::String("must".to_string())),
                    );
                }

                for must_not in &bool_query.must_not {
                    node = node.with_child(
                        must_not
                            .build_visualization_node()
                            .with_detail("clause_type", Value::String("must_not".to_string())),
                    );
                }

                for should in &bool_query.should {
                    node = node.with_child(
                        should
                            .build_visualization_node()
                            .with_detail("clause_type", Value::String("should".to_string())),
                    );
                }

                for filter in &bool_query.filter {
                    node = node.with_child(
                        filter
                            .build_visualization_node()
                            .with_detail("clause_type", Value::String("filter".to_string())),
                    );
                }

                node
            }
            QueryType::Range(range) => {
                VisualizationNode::new("range", &format!("Range: {}", range.field))
                    .with_detail("field", Value::String(range.field.clone()))
            }
            QueryType::MatchAll => VisualizationNode::new("match_all", "Match All"),
            QueryType::WildCard(wildcard) => {
                VisualizationNode::new("wildcard", &format!("Wildcard: {}", wildcard.field))
                    .with_detail("field", Value::String(wildcard.field.clone()))
                    .with_detail("value", Value::String(wildcard.value.clone()))
            }
        }
    }
}

impl SortType {
    fn build_visualization_node(&self) -> VisualizationNode {
        match self {
            SortType::Field(field_sort) => VisualizationNode::new(
                "field_sort",
                &format!("Sort: {} {:?}", field_sort.field, field_sort.order),
            )
            .with_detail("field", Value::String(field_sort.field.clone()))
            .with_detail("order", Value::String(format!("{:?}", field_sort.order))),
            SortType::Score => VisualizationNode::new("score_sort", "Sort: _score"),
        }
    }
}

impl AggregationType {
    fn build_visualization_node(&self) -> VisualizationNode {
        match self {
            AggregationType::Terms(terms_agg) => {
                VisualizationNode::new("terms_agg", &format!("Terms Agg: {}", terms_agg.field))
                    .with_detail("field", Value::String(terms_agg.field.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "visualizer")]
    #[test]
    fn test_visualization() {
        let request = SearchRequest::new()
            .query(
                QueryType::bool_query()
                    .must(QueryType::term("status", "published"))
                    .should(QueryType::match_query("title", "rust"))
                    .build(),
            )
            .size(10);

        // Test HTML generation
        let html_result = request.generate_html();
        assert!(html_result.is_ok());
        let html = html_result.unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Query Structure"));
        assert!(html.contains("Bool Query"));

        // Test SVG generation
        let svg_result = request.generate_svg();
        assert!(svg_result.is_ok());
        let svg = svg_result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));

        // Test interactive view (should be same as HTML)
        let interactive_result = request.generate_interactive_view();
        assert!(interactive_result.is_ok());
    }
}
