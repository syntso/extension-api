<div align="center">

<img src="docs/images/logo.png" alt="Synt" width="96" />

# Synt Extension API

**The interface and Rust SDK for writing Synt extensions.**

</div>

---

An extension adds actions, views and panels to [Synt](https://github.com/syntso/synt). It is a WebAssembly component that runs in a sandbox inside the core.

- **One interface.** [`wit/0.2/synt-extension.wit`](wit/0.2/synt-extension.wit) is the whole contract. The core still runs extensions built against [`wit/0.1`](wit/0.1/synt-extension.wit).
- **Nothing by default.** No files, no network, no environment. An extension gets only the capabilities its manifest asks for and the person approves at install.
- **You approve.** Every core action it calls, and every HTTP request it sends, goes through the same approvals and journal as yours.
- **Any language.** Rust has an SDK here. Anything that builds a WebAssembly component can implement the WIT.

## Write an extension in Rust

```bash
rustup target add wasm32-wasip2
cargo new --lib uptime && cd uptime
```

`Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
synt-extension-api = { git = "https://github.com/syntso/extension-api" }
```

`src/lib.rs`:

```rust
use synt_extension_api::view::Node;
use synt_extension_api::{ActionInfo, Error, Extension, Risk, call, register_extension};

struct Uptime;

impl Extension for Uptime {
    fn describe() -> Vec<ActionInfo> {
        vec![ActionInfo::new("uptime.check", "Check uptime", Risk::Read).takes_target()]
    }

    fn run(_action: &str, target: Option<&str>, _params: &str) -> Result<String, Error> {
        call("ssh.exec", target, r#"{"command":"uptime"}"#)
    }

    fn render(_view: &str, target: Option<&str>, _params: &str) -> Result<String, Error> {
        let output = call("ssh.exec", target, r#"{"command":"uptime"}"#)?;

        Ok(Node::column([Node::title("Uptime"), Node::text(output)]).to_json())
    }
}

register_extension!(Uptime);
```

`extension.toml` next to `Cargo.toml`:

```toml
id = "uptime"
name = "Uptime"
version = "0.2.0"
api = "0.2"
description = "Shows how long a host has been up."
repository = "https://github.com/you/extension-uptime"

[capabilities]
call = ["ssh.exec"]

[[contributes.resource.menu]]
title = "Check uptime"
action = "uptime.check"
kinds = ["host"]
```

Build it:

```bash
cargo build --release --target wasm32-wasip2
```

The component is `target/wasm32-wasip2/release/uptime.wasm`. Action ids start with the extension id; `params` and `params_schema` are JSON.

## What an extension can do

| In the manifest | In Rust | What the core does |
|---|---|---|
| `call = ["ssh.exec"]` | `call(action, target, params)` | Runs the core action through the broker and the journal |
| `storage = true` | `storage::get`, `storage::set` | Keeps up to 1 MiB of its own values |
| `http = ["api.example.com"]` | `http::send(Request::get(url))` | Sends the request itself, to listed hosts only, and journals it |
| `secrets = { "grafana/token" = ["api.example.com"] }` | `Request::secret_header("Authorization", "grafana/token", "Bearer ")` | Inserts the secret into the request; the extension never sees it |
| `documents = true` and `[[contributes.document]]` | `documents::read`, `write`, `listing`, `remove` | Keeps the extension's own kinds in project folders, with revisions, and calls `merge` on conflicts |
| `events = ["journal", "session", "resource", "approval"]` | `fn on_event` | Calls the extension when something happens |
| `timers = true` | `timers::set(name, seconds)`, `fn on_timer` | Wakes the extension while the core runs |
| `files = "read"` or `"read-write"` | `files::chosen`, `files::read`, `files::write` | Reads and writes only files the person picked for it |

## Contribution points

Under `[contributes]` in the manifest: `sidebar` sections, `document` types, `panel`s, `command`s, `resource.menu` entries, `status` items and a `settings` form. A sidebar section, a status item or a panel shows either a **native view**, a JSON view tree the extension returns from `render` and updates with `ui::patch_view`, or a **web panel**, an HTML and JS bundle the extension serves from `asset` and talks to through `panel_message` and `ui::post`. The `view` module builds the trees and patches; `Assets` serves a bundle built into the component.

See [`examples/`](examples): `hello` calls a core action, `watch` polls an endpoint with a secret on a timer and patches a status item, `notes` keeps documents edited in a web panel.

## Build

```bash
git clone https://github.com/syntso/extension-api.git && cd extension-api
git config core.hooksPath .githooks
cargo test
cargo build --examples --target wasm32-wasip2 --release
```

## License

[Apache 2.0](LICENSE).
