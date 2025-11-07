use askama::Template;

use crate::{VisualizationError, visualizer::VisualizationNode};

/// Trait for convert a visualization node into html.
pub trait HtmlVisualization {
    fn generate_html(&self) -> Result<String, VisualizationError>;
}

impl VisualizationNode {
    fn to_html(&self) -> String {
        let clause_badge = if let Some(ref clause_type) = self.clause_type {
            format!(
                r#"<span class="clause-type {}">{}</span>"#,
                clause_type,
                clause_type.to_uppercase()
            )
        } else {
            String::new()
        };

        let toggle = if !self.children.is_empty() {
            r#"<span class="toggle">▼</span> "#
        } else {
            ""
        };

        let mut html = format!(
            r#"<div class="node {}" onclick="toggleNode(this)">
                {}<strong>{}</strong> - {}
            "#,
            self.node_type,
            toggle,
            self.node_type.to_uppercase(),
            // self.label,
            clause_badge
        );

        // Add value displays for key details
        for (key, value) in &self.details {
            match key.as_str() {
                "value" | "query" | "values" | "field" | "gte" | "gt" | "lte" | "lt" | "order"
                | "size" | "from" => {
                    html.push_str(&format!(
                        r#"<div class="value-display">
                            <span class="value-label">{}:</span>
                            <span class="value-content">{}</span>
                        </div>"#,
                        key,
                        self.format_value(value)
                    ));
                }
                _ => {}
            }
        }

        html.push_str("</div>");

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
/// Implement the HtmlVisualization trait for VisualizationNode
impl HtmlVisualization for VisualizationNode {
    fn generate_html(&self) -> Result<String, VisualizationError> {
        let html = self.to_html();

        let json = if let Some(json) = self.json.as_ref() {
            json
        } else {
            &serde_json::Value::Null
        };

        let template = HtmlQueryVisualizationTemplate {
            tree_html: html,
            json_pretty: serde_json::to_string_pretty(json)
                .map_err(|e| VisualizationError::SerializationError(e.to_string()))?,
        };

        template
            .render()
            .map_err(|e| VisualizationError::TemplateError(e.to_string()))
    }
}

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
struct HtmlQueryVisualizationTemplate {
    tree_html: String,
    json_pretty: String,
}

#[cfg(test)]
mod test;
