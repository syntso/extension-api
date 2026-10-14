use synt_extension_api::http::{self, Request};
use synt_extension_api::serde_json::{self, Value};
use synt_extension_api::view::{Node, Patch};
use synt_extension_api::{
    ActionInfo, Error, Event, Extension, Risk, register_extension, settings, storage, timers, ui,
};

const HEALTH: &str = "https://status.example.com/health";
const LAST: &str = "last";
const DENIALS: &str = "denials";

struct Watch;

impl Extension for Watch {
    fn describe() -> Vec<ActionInfo> {
        vec![ActionInfo::new(
            "watch.check",
            "Check health now",
            Risk::Read,
        )]
    }

    fn run(action: &str, _target: Option<&str>, _params: &str) -> Result<String, Error> {
        match action {
            "watch.check" => check(),
            other => Err(Error::new("unknown_action", other)),
        }
    }

    fn start() -> Result<(), Error> {
        timers::set("poll", every())
    }

    fn render(view: &str, _target: Option<&str>, _params: &str) -> Result<String, Error> {
        match view {
            "health" => Ok(Node::text(last()).to_json()),
            other => Err(Error::new("unknown_view", other)),
        }
    }

    fn on_timer(_name: &str) -> Result<(), Error> {
        check().map(drop)
    }

    fn on_event(event: &Event) -> Result<(), Error> {
        if !event.payload().contains("\"denied\"") {
            return Ok(());
        }

        let denials = count(DENIALS) + 1;
        storage::set(DENIALS, denials.to_string().as_bytes())
    }
}

fn check() -> Result<String, Error> {
    let request = Request::get(HEALTH).secret_header("Authorization", "watch/token", "Bearer ");
    let response = http::send(request)?;

    let text = format!("{} {}", response.status(), response.text().trim());
    storage::set(LAST, text.as_bytes())?;
    ui::patch_view(
        "health",
        &Patch::new().replace("/text", text.clone()).to_json(),
    )?;

    Ok(serde_json::json!({ "health": text }).to_string())
}

fn every() -> u32 {
    let values: Value = serde_json::from_str(&settings()).unwrap_or(Value::Null);

    values
        .get("every")
        .and_then(Value::as_u64)
        .and_then(|seconds| u32::try_from(seconds).ok())
        .unwrap_or(60)
}

fn last() -> String {
    storage::get(LAST).map_or_else(
        || "not checked yet".to_owned(),
        |bytes| String::from_utf8_lossy(&bytes).into_owned(),
    )
}

fn count(key: &str) -> u64 {
    storage::get(key)
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .and_then(|text| text.parse().ok())
        .unwrap_or(0)
}

register_extension!(Watch);
