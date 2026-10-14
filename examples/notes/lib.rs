use synt_extension_api::serde_json::{self, Value, json};
use synt_extension_api::view::Node;
use synt_extension_api::{
    ActionInfo, Assets, Error, Extension, Risk, documents, register_extension,
};

const BUNDLE: Assets = Assets::new(&[
    ("index.html", include_bytes!("panel/index.html")),
    ("editor.js", include_bytes!("panel/editor.js")),
]);

struct Notes;

impl Extension for Notes {
    fn describe() -> Vec<ActionInfo> {
        vec![
            ActionInfo::new("notes.list", "List the notes of a project", Risk::Read)
                .params_schema(r#"{"type":"object","properties":{"project":{"type":"string"}}}"#),
        ]
    }

    fn run(action: &str, _target: Option<&str>, params: &str) -> Result<String, Error> {
        match action {
            "notes.list" => Ok(paths(&project(params)?)?.to_string()),
            other => Err(Error::new("unknown_action", other)),
        }
    }

    fn render(view: &str, _target: Option<&str>, params: &str) -> Result<String, Error> {
        if view != "notes" {
            return Err(Error::new("unknown_view", view));
        }

        let listed = paths(&project(params)?)?;
        let names = listed.as_array().cloned().unwrap_or_default();

        Ok(Node::log(names.iter().filter_map(Value::as_str)).to_json())
    }

    fn panel_message(_panel: &str, message: &str) -> Result<String, Error> {
        let message: Value = parse(message)?;
        let project = text(&message, "project")?;
        let path = text(&message, "path")?;

        let saved = documents::write(
            project,
            path,
            text(&message, "text")?.as_bytes(),
            message.get("base").and_then(Value::as_str),
        )?;

        Ok(json!({ "revision": saved.info.revision, "merged": saved.merged }).to_string())
    }

    fn asset(path: &str) -> Option<Vec<u8>> {
        BUNDLE.get(path)
    }

    fn merge(
        _kind: &str,
        _base: Option<&[u8]>,
        ours: &[u8],
        theirs: &[u8],
    ) -> Result<Vec<u8>, Error> {
        let ours = String::from_utf8_lossy(ours);
        let theirs = String::from_utf8_lossy(theirs);

        let mut lines: Vec<&str> = ours.lines().collect();
        lines.extend(
            theirs
                .lines()
                .filter(|line| !ours.lines().any(|kept| kept == *line)),
        );

        Ok(lines.join("\n").into_bytes())
    }
}

fn paths(project: &str) -> Result<Value, Error> {
    let found = documents::listing(project)?;

    Ok(found
        .into_iter()
        .map(|info| Value::String(info.path))
        .collect())
}

fn project(params: &str) -> Result<String, Error> {
    let params = parse(params)?;

    text(&params, "project").map(str::to_owned)
}

fn parse(text: &str) -> Result<Value, Error> {
    serde_json::from_str(text).map_err(|error| Error::new("invalid_params", error.to_string()))
}

fn text<'v>(value: &'v Value, field: &str) -> Result<&'v str, Error> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| Error::new("invalid_params", format!("{field} is missing")))
}

register_extension!(Notes);
