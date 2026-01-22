---
# bx-fktp
title: Migrate error handling to anyhow
status: todo
type: task
priority: high
created_at: 2026-01-22T09:25:47Z
updated_at: 2026-01-22T09:25:47Z
---

Replace `Result<T, String>` pattern with `anyhow::Result<T>` for better error handling.

## Why
- Current pattern loses error context and type information
- No error chain/backtrace support
- 40% less boilerplate with anyhow
- Industry standard for Rust CLI applications

## Checklist
- [ ] Add `anyhow = "1.0"` to Cargo.toml
- [ ] Update src/bin/main.rs: Change `fn core() -> Result<i32, String>` to `fn core() -> anyhow::Result<i32>`
- [ ] Update src/bin/main.rs: Change `fn real_main() -> i32` to return `anyhow::Result<()>`
- [ ] Update src/get_cfg.rs: `get_cfg()` to return `anyhow::Result<String>`
- [ ] Update src/cache.rs: All functions to use `anyhow::Result`
- [ ] Update src/init.rs: `init()` to use `anyhow::Result`
- [ ] Update src/raw_schema.rs: Config methods to use `anyhow::Result`
- [ ] Update src/schema.rs: All Result-returning functions
- [ ] Update src/bones.rs: All Result-returning functions
- [ ] Add `.context()` calls for meaningful error messages
- [ ] Run `cargo test` to verify all tests pass
- [ ] Run `cargo clippy` to check for issues

## Example Migration
```rust
// Before
pub fn get_cfg() -> Result<String, String> {
    match fs::read_to_string(&path) {
        Ok(s) => Ok(s),
        Err(_) => Err(format!("Error reading config at '{}'", path))
    }
}

// After
use anyhow::{Context, Result};

pub fn get_cfg() -> Result<String> {
    fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config at '{}'", path))
}
```