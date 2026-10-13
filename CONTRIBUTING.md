# Contributing

## Branches

- `main` holds released code only. Every commit on it is a version.
- `dev` is where work comes together. Branch from it, and open pull requests into it.
- Name branches after the change: `feat/storage-list`, `fix/error-code`.

Pull requests into `dev` are squash-merged, so the pull request title becomes the commit.

## Commits

[Conventional Commits](https://www.conventionalcommits.org), one line, no body.

```text
feat(wit): add storage-list
fix(sdk): keep the error code on the way back
ci: cache the cargo registry
```

- Types: `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `build`, `ci`, `chore`, `style`, `revert`.
- Scope: `wit`, `sdk`, or nothing.
- Subject: imperative, starts lowercase, no period, the whole line at most 72 characters.
- A breaking change gets a `!`: `feat(wit)!: rename run`.

`git config core.hooksPath .githooks` turns on the local check.

## Interface

- The WIT is versioned on its own. A change that breaks extensions bumps the package version, and the core keeps supporting the older one.
- The SDK follows the WIT: every type and function there has one plain Rust counterpart.

## Code

- One main type per file, named after it. A concept with several files gets its own folder.
- Private by default. `pub` only for what the crate offers to others.
- Errors with `thiserror`. No `unwrap()` outside tests.
- Full words in names: `connection`, not `conn`.

## Shape of code

- No comments and no doc comments. Names, types and small functions carry the meaning.
- A blank line between logical steps; intermediate results get names.
- Guards first, the main path flat below.
- Imports grouped `std`, external crates, `crate::`.

## Before you push

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --example hello --target wasm32-wasip2
```

CI runs the same commands on macOS, Windows and Linux, plus `cargo deny check` for dependency licenses and a check that the example builds into a valid component.
