---
# bx-xdgh
title: Update .editorconfig to use tabs and reformat codebase
status: completed
type: task
priority: normal
created_at: 2026-01-21T10:57:17Z
updated_at: 2026-01-21T12:25:02Z
---

## Summary

Change the project's indentation style from 4 spaces to tabs in `.editorconfig` and reformat all code to match.

## Requirements

- Update `.editorconfig` to use `indent_style = tab` instead of spaces
- Configure `rustfmt` to use tabs (create/update `rustfmt.toml` with `hard_tabs = true`)
- Reformat entire codebase with `cargo fmt`
- Ensure CI still passes after reformatting

## Verification: Check .editorconfig vs rustfmt defaults

Before finalizing, verify that the new `.editorconfig` rules align with rustfmt/cargo fmt defaults:

1. Create a new branch for this work
2. Update `.editorconfig` to use tabs
3. Manually convert codefiles to match the new `.editorconfig` rules (or use an editor that respects .editorconfig)
4. Commit these changes
5. Run `cargo fmt` and check if anything changes
   - If `cargo fmt` makes no changes → `.editorconfig` matches rustfmt defaults
   - If `cargo fmt` makes changes → there's a mismatch; adjust `rustfmt.toml` accordingly

This ensures `.editorconfig` and `rustfmt` stay in sync.

## Checklist

- [x] Create a new branch for this work
- [x] Update `.editorconfig` to set `indent_style = tab`
- [x] Create or update `rustfmt.toml` with `hard_tabs = true`
- [x] Run `cargo fmt` to reformat codebase (no manual reformatting needed since rustfmt.toml was created first)
- [x] Verify formatting with `cargo fmt -- --check`
- [x] Run `cargo check && cargo clippy && cargo test` to ensure nothing broke
- [x] Commit the final formatting changes