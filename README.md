<div align="center">

<img src="docs/images/logo.png" alt="Synt" width="96" />

# Synt Extension API

**The interface and Rust SDK for writing Synt extensions.**

</div>

---

An extension adds actions to [Synt](https://github.com/syntso/synt). It is a WebAssembly component that runs in a sandbox inside the core.

- **One interface.** [`wit/synt-extension.wit`](wit/synt-extension.wit) is the whole contract: the extension describes its actions and runs them.
- **Nothing by default.** No files, no network, no environment. An extension can only call the core actions it asks for, keep a small store of its own, and log.
- **You approve.** Every action it calls goes through the same approvals and journal as yours.
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
use synt_extension_api::{ActionInfo, Error, Extension, Risk, register_extension};

struct Uptime;

impl Extension for Uptime {
    fn describe() -> Vec<ActionInfo> {
        vec![ActionInfo::new("uptime.check", "Check uptime", Risk::Read).takes_target()]
    }

    fn run(_action: &str, target: Option<&str>, _params: &str) -> Result<String, Error> {
        synt_extension_api::call("ssh.exec", target, r#"{"command":"uptime"}"#)
    }
}

register_extension!(Uptime);
```

`extension.toml` next to `Cargo.toml`:

```toml
id = "uptime"
name = "Uptime"
version = "0.1.0"
api = "0.1"
description = "Shows how long a host has been up."
repository = "https://github.com/you/extension-uptime"

[capabilities]
call = ["ssh.exec"]
storage = false
```

Build it:

```bash
cargo build --release --target wasm32-wasip2
```

The component is `target/wasm32-wasip2/release/uptime.wasm`. Action ids start with the extension id. `params` and `params_schema` are JSON. `call` reaches only the actions listed under `capabilities.call`; `storage::get` and `storage::set` need `storage = true`.

## Build

```bash
git clone https://github.com/syntso/extension-api.git && cd extension-api
git config core.hooksPath .githooks
cargo test
cargo build --example hello --target wasm32-wasip2
```

## License

[Apache 2.0](LICENSE).
