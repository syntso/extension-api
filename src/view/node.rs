use serde_json::{Map, Value, json};

#[derive(Debug, Clone, PartialEq)]
pub struct Node(Value);

impl Node {
    pub fn text(text: impl Into<String>) -> Self {
        Self(json!({ "type": "text", "text": text.into() }))
    }

    pub fn title(text: impl Into<String>) -> Self {
        Self(json!({ "type": "text", "text": text.into(), "style": "title" }))
    }

    pub fn styled(text: impl Into<String>, style: &str) -> Self {
        Self(json!({ "type": "text", "text": text.into(), "style": style }))
    }

    pub fn column(children: impl IntoIterator<Item = Self>) -> Self {
        Self::stack("column", children)
    }

    pub fn row(children: impl IntoIterator<Item = Self>) -> Self {
        Self::stack("row", children)
    }

    pub fn key_values<K: Into<String>, V: Into<Value>>(
        items: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let items: Vec<Value> = items
            .into_iter()
            .map(|(key, value)| json!({ "key": key.into(), "value": value.into() }))
            .collect();

        Self(json!({ "type": "keyvalue", "items": items }))
    }

    pub fn log<L: Into<String>>(lines: impl IntoIterator<Item = L>) -> Self {
        let lines: Vec<String> = lines.into_iter().map(Into::into).collect();

        Self(json!({ "type": "log", "lines": lines }))
    }

    pub fn table<C: Into<String>>(
        columns: impl IntoIterator<Item = C>,
        rows: impl IntoIterator<Item = Vec<Value>>,
    ) -> Self {
        let columns: Vec<String> = columns.into_iter().map(Into::into).collect();
        let rows: Vec<Vec<Value>> = rows.into_iter().collect();

        Self(json!({ "type": "table", "columns": columns, "rows": rows }))
    }

    pub fn line_chart<N: Into<String>>(
        series: impl IntoIterator<Item = (N, Vec<(f64, f64)>)>,
    ) -> Self {
        let series: Vec<Value> = series
            .into_iter()
            .map(|(name, points)| {
                let points: Vec<[f64; 2]> = points.into_iter().map(|(x, y)| [x, y]).collect();
                json!({ "name": name.into(), "points": points })
            })
            .collect();

        Self(json!({ "type": "chart", "kind": "line", "series": series }))
    }

    pub fn button(title: impl Into<String>, action: impl Into<String>) -> Self {
        Self(json!({ "type": "button", "title": title.into(), "action": action.into() }))
    }

    pub fn field(name: impl Into<String>, label: impl Into<String>, input: &str) -> Self {
        Self(json!({ "type": "field", "name": name.into(), "label": label.into(), "input": input }))
    }

    pub fn form(
        fields: impl IntoIterator<Item = Self>,
        submit: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        let children: Vec<Value> = fields.into_iter().map(Self::into_value).collect();

        Self(json!({
            "type": "form",
            "children": children,
            "submit": { "title": submit.into(), "action": action.into() },
        }))
    }

    #[must_use]
    pub fn target(self, target: impl Into<String>) -> Self {
        self.with("target", Value::String(target.into()))
    }

    #[must_use]
    pub fn params(self, params: Value) -> Self {
        self.with("params", params)
    }

    pub fn into_value(self) -> Value {
        self.0
    }

    pub fn to_json(&self) -> String {
        self.0.to_string()
    }

    fn stack(direction: &str, children: impl IntoIterator<Item = Self>) -> Self {
        let children: Vec<Value> = children.into_iter().map(Self::into_value).collect();

        Self(json!({ "type": "stack", "direction": direction, "children": children }))
    }

    fn with(mut self, field: &str, value: Value) -> Self {
        if let Value::Object(fields) = &mut self.0 {
            fields.insert(field.to_owned(), value);
        } else {
            let mut fields = Map::new();
            fields.insert(field.to_owned(), value);
            self.0 = Value::Object(fields);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Node;

    #[test]
    fn builds_the_view_tree_the_core_checks() {
        let view = Node::column([
            Node::title("web-1"),
            Node::key_values([("up", json!("3 days")), ("load", json!(0.42))]),
            Node::line_chart([("load", vec![(0.0, 0.4), (1.0, 0.5)])]),
            Node::button("Check", "uptime.check").target("web-1"),
        ]);

        assert_eq!(
            view.into_value(),
            json!({
                "type": "stack",
                "direction": "column",
                "children": [
                    { "type": "text", "text": "web-1", "style": "title" },
                    { "type": "keyvalue", "items": [
                        { "key": "up", "value": "3 days" },
                        { "key": "load", "value": 0.42 }
                    ]},
                    { "type": "chart", "kind": "line", "series": [
                        { "name": "load", "points": [[0.0, 0.4], [1.0, 0.5]] }
                    ]},
                    { "type": "button", "title": "Check", "action": "uptime.check", "target": "web-1" }
                ]
            })
        );
    }

    #[test]
    fn builds_forms_tables_and_logs() {
        let form = Node::form(
            [Node::field("every", "Every", "number")],
            "Save",
            "uptime.save",
        );
        let table = Node::table(["pod", "ready"], [vec![json!("api"), json!(true)]]);

        assert_eq!(form.into_value()["submit"]["action"], "uptime.save");
        assert_eq!(table.into_value()["rows"][0][1], true);
        assert_eq!(
            Node::log(["a", "b"]).to_json(),
            r#"{"lines":["a","b"],"type":"log"}"#
        );
    }
}
