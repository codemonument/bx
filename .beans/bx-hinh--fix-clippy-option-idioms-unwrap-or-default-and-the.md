---
# bx-hinh
title: 'Fix clippy: Option idioms (unwrap_or_default, and_then)'
status: completed
type: task
priority: low
created_at: 2026-01-21T12:26:49Z
updated_at: 2026-01-21T13:23:52Z
---

## Summary
Two related clippy warnings about idiomatic Option handling.

## Warnings

### manual_unwrap_or_default
`src/raw_schema.rs:88` - Using match instead of `.unwrap_or_default()`
```rust
// Before
let env_files = match env_files {
    Some(env_files) => env_files,
    None => Vec::new(),
};

// After
let env_files = env_files.unwrap_or_default();
```

### map_flatten
`src/template.rs:31` - Using `.map(...).flatten()` instead of `.and_then(...)`
```rust
// Before
.map(|path| if path.exists() { Some(path) } else { None })
.flatten()

// After
.and_then(|path| if path.exists() { Some(path) } else { None })
```

## Checklist
- [ ] Fix `src/raw_schema.rs:88` - use `.unwrap_or_default()`
- [ ] Fix `src/template.rs:31` - use `.and_then()`
- [ ] Run `cargo test` to ensure nothing broke
- [ ] Run `cargo clippy` to verify warning is gone

## Notes
Auto-fixable with `cargo clippy --fix`