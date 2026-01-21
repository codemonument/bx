---
# bx-xz7w
title: 'Fix clippy: needless borrows for generic args'
status: completed
type: task
priority: low
created_at: 2026-01-21T12:26:41Z
updated_at: 2026-01-21T13:23:52Z
---

## Summary
Clippy `needless_borrows_for_generic_args` warning: the borrowed expression implements the required traits, so the `&` is unnecessary.

## Locations
- `src/bones.rs:297` - `OsCommand::new(&executable)` → `OsCommand::new(executable)`
- `src/raw_schema.rs:96` - `dotenv::from_filename(&env_file)` → `dotenv::from_filename(env_file)`

## Checklist
- [ ] Fix `src/bones.rs:297`
- [ ] Fix `src/raw_schema.rs:96`
- [ ] Run `cargo test` to ensure nothing broke
- [ ] Run `cargo clippy` to verify warning is gone

## Notes
Auto-fixable with `cargo clippy --fix`