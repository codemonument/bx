---
# bx-jv26
title: Reorganize code into hierarchical modules
status: completed
type: task
priority: low
created_at: 2026-01-22T10:46:20Z
updated_at: 2026-01-22T12:24:36Z
---

Restructure flat module layout into a clearer hierarchical organization.

## Why
- Current flat structure makes navigation harder
- Hierarchical modules clarify dependencies
- Better separation of concerns
- Easier to understand codebase

## Current Structure
```
src/
├── bones.rs, cache.rs, schema.rs, raw_schema.rs, ...
```

## Target Structure
```
src/
├── lib.rs
├── bin/main.rs
├── cli/
│   ├── mod.rs
│   ├── args.rs
│   └── commands.rs
├── config/
│   ├── mod.rs
│   ├── schema.rs
│   └── parser.rs
├── execution/
│   ├── mod.rs
│   └── bones.rs
├── cache.rs
├── error.rs
└── version.rs
```

## Checklist
- [x] Create src/cli/ directory and module (done during clap migration bx-3tpt)
- [ ] Create src/config/ directory and module (deferred - optional)
- [ ] Create src/execution/ directory and module (deferred - optional)
- [x] Move argument handling to cli/ (done during clap migration)
- [ ] Move schema/raw_schema to config/ (deferred - optional)
- [ ] Move bones to execution/ (deferred - optional)
- [x] Update src/lib.rs exports (done for cli module)
- [ ] Update all internal imports (partial - cli done)
- [x] Run cargo test to verify
- [x] Run cargo clippy to check for issues

## Notes
- This is optional and can be deferred
- Best done after clap migration (bx-3tpt) since that creates cli/ module anyway
- Can be done incrementally, one module at a time