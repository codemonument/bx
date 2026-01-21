---
# bx-bmqu
title: Remove bonnie CLI, keep only bx binary
status: completed
type: task
priority: normal
created_at: 2026-01-21T22:31:46Z
updated_at: 2026-01-21T22:34:15Z
---

## Summary
Remove the legacy 'bonnie' binary target from the project. The fork should only produce the 'bx' binary going forward.

## Rationale
- This is a fork intended to use 'bx' as the command name
- Having both binaries causes cargo warnings about shared source files
- Simplifies the build and reduces confusion

## Checklist
- [x] Remove 'bonnie' binary target from Cargo.toml
- [x] Update any references to 'bonnie' binary in CI workflows
- [x] Update README.md to reflect bx-only usage
- [x] Verify build produces only 'bx' binary
- [x] Run full CI validation (cargo check && cargo fmt -- --check && cargo clippy && cargo test)

## Notes
The warning 'file found to be present in multiple build targets' should disappear after this change.