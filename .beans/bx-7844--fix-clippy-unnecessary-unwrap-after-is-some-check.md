---
# bx-7844
title: 'Fix clippy: unnecessary unwrap after is_some check'
status: todo
type: task
priority: low
created_at: 2026-01-21T15:54:10Z
updated_at: 2026-01-21T15:54:10Z
---

## Summary
Clippy `unnecessary_unwrap` warning: calling `.unwrap()` on an Option after checking `.is_some()` in the same condition.

## Location
### src/init.rs:14-15
```rust
// Before
if template.is_some() && fs::metadata(template.as_ref().unwrap()).is_ok() {
    let template_path = template.unwrap();
    ...
}

// After - use if let or match
if let Some(template_path) = template {
    if fs::metadata(&template_path).is_ok() {
        ...
    }
}
```

## Checklist
- [ ] Refactor `src/init.rs` to use `if let` or `match` pattern
- [ ] Run `cargo test` to ensure nothing broke
- [ ] Run `cargo clippy` to verify warning is gone

## Notes
NOT auto-fixable - requires restructuring the conditional logic.