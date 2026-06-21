---
# bx-itqk
title: Modernize testing with assert_cmd
status: completed
type: task
priority: normal
created_at: 2026-01-22T10:46:14Z
updated_at: 2026-01-22T11:48:19Z
---

Add modern CLI testing infrastructure using assert_cmd and related crates.

## Why
- Current tests use custom macros that are harder to maintain
- assert_cmd is the industry standard for CLI testing
- Better assertions and error messages
- Easier to write new tests

## Checklist
- [x] Add dev-dependencies to Cargo.toml:
  - assert_cmd = "2.0"
  - predicates = "3.1"
  - assert_fs = "1.1"
- [x] Create tests/cli.rs with modern CLI tests
- [x] Add test for --version flag
- [x] Add test for --help flag
- [x] Add test for missing config file error
- [x] Add test for --init creates config
- [x] Add test for --cache creates cache file
- [x] Consider migrating existing tests to use assert_cmd patterns (existing tests work fine, no need to migrate)
- [x] Run full test suite to verify

## Example Tests
```rust
use assert_cmd::Command;
use predicates::prelude::*;
use assert_fs::prelude::*;

#[test]
fn test_version_flag() {
    Command::cargo_bin("bx")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_missing_config() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("bx")
        .unwrap()
        .current_dir(temp.path())
        .arg("build")
        .assert()
        .failure();
}
```