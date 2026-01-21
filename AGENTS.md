# AGENTS

This repository is a Rust CLI forked from Bonnie with a `bx` binary. This file is for agentic coding tools.

## Quick Orientation
- Primary task runner is Bonnie; project scripts live in `bonnie.toml`.
- Source lives in `src/` with binaries under `src/bin/main.rs`.
- Tests are Rust unit + integration tests under `src/` and `tests/`.

## Build, Lint, Test

### Bonnie (preferred)
`bonnie.toml` defines convenience scripts. Use `bx <script>`:

- `bx build` -> `cargo build`
- `bx test` -> `cargo watch -x "test"`
- `bx check` -> `cargo check && cargo fmt -- --check && cargo clippy && cargo test`
- `bx dev -- <args>` -> `cd src && cargo run --bin bx -- <args>`

### Direct Cargo
- Build: `cargo build`
- Run binary: `cargo run --bin bx -- <args>`
- Run all tests: `cargo test`
- Format: `cargo fmt`
- Format check (CI style): `cargo fmt -- --check`
- Lint: `cargo clippy`
- Full validation (CI): `cargo check && cargo fmt -- --check && cargo clippy && cargo test`

### Single Test / Targeted Tests
Cargo test filters use substring match by default (exact match requires `--exact`).

- Single test by name: `cargo test test_name`
- Exact test path: `cargo test -- --exact module::test_name`
- Integration test file: `cargo test --test caching`
- Unit tests only: `cargo test --lib`
- Show output: `cargo test -- --nocapture`
- Single-threaded: `cargo test -- --test-threads=1`

Common integration tests:
- `cargo test --test caching`
- `cargo test --test known_cases`

### CI/CD Build Commands
From `.github/workflows/*.yml`:

- CI: `cargo check && cargo fmt -- --check && cargo clippy && cargo test`
- Release builds: `cargo build --release --locked`
- Musl release builds: `cargo build --release --target x86_64-unknown-linux-musl --locked`

## Code Style Guidelines

### Formatting
- `.editorconfig` enforces: 4-space indent, LF endings, UTF-8, trim trailing whitespace, final newline.
- No `rustfmt.toml`; use default `rustfmt` rules.
- Prefer `cargo fmt` before commits; CI enforces `cargo fmt -- --check`.

### Imports
Pattern in `src/*.rs`:

- Group imports by origin (external crates, std, internal crate).
- Use `use crate::...` for internal modules.
- Combine related imports with braces (`use serde::{Deserialize, Serialize};`).
- Alias with `as` for clarity (`use std::process::Command as OsCommand;`).

### Naming
Follow Rust conventions (consistent with existing code):

- Modules, functions, variables: `snake_case`.
- Types, traits, enums: `PascalCase`.
- Constants/statics: `SCREAMING_SNAKE_CASE`.
- Type aliases: `PascalCase`.

### Error Handling
Established pattern is `Result<T, String>` and user-facing error messages.

- Prefer returning `Result<_, String>` from fallible functions.
- Use `?` to propagate when possible, `match` for custom messages.
- Error strings should be descriptive and include guidance when possible.
- Only use `unwrap()` when the logic guarantees safety and the code documents why.

### API/Type Patterns
- Favor `impl std::io::Write` to pass output sinks for testability.
- Use `Option<T>` for optional config, `Result<T, String>` for fallible paths.
- Use `serde` derives (`Deserialize`, `Serialize`) where needed.

### Comments and Documentation
- CONTRIBUTING requires comments and updates to help/README when behavior changes.
- Use `//` comments; keep them focused on intent and non-obvious logic.

## Testing Conventions
- Integration tests live in `tests/*.rs`.
- Unit tests exist in source files (e.g., `src/version.rs`).
- Tests often use helpers/macros to assert exit codes and output.
- Prefer `std::io::Write` buffers in tests to avoid side effects.

## Commit/Release Notes (if needed)
- Conventional Commits are used for releases (see `.versionrc.json`).
- CI expects clean formatting + clippy (same as `bx check`).

## Cursor/Copilot Rules
- No `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` found.

## References
- `bonnie.toml`: canonical dev commands for this repo.
- `.github/workflows/ci.yml`: CI validation pipeline.
- `.editorconfig`: formatting rules.
