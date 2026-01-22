---
# bx-jv26
title: Reorganize code into hierarchical modules
status: todo
type: task
priority: low
created_at: 2026-01-22T10:46:20Z
updated_at: 2026-01-22T10:46:20Z
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
- [ ] Create src/cli/ directory and module
- [ ] Create src/config/ directory and module
- [ ] Create src/execution/ directory and module
- [ ] Move argument handling to cli/
- [ ] Move schema/raw_schema to config/
- [ ] Move bones to execution/
- [ ] Update src/lib.rs exports
- [ ] Update all internal imports
- [ ] Run cargo test to verify
- [ ] Run cargo clippy to check for issues

## Notes
- This is optional and can be deferred
- Best done after clap migration (bx-3tpt) since that creates cli/ module anyway
- Can be done incrementally, one module at a time