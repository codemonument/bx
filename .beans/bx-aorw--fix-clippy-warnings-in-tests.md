---
# bx-aorw
title: Fix clippy warnings in tests
status: todo
type: bug
priority: low
created_at: 2026-01-22T08:56:30Z
updated_at: 2026-01-22T08:56:30Z
---

Clippy reports unused code in test files.

## Checklist
- [ ] tests/known_cases.rs:9 - Remove unused import: BONNIE_VERSION
- [ ] tests/known_cases.rs:34 - Remove or use macro: expect_exit_code
- [ ] tests/known_cases.rs:56 - Remove or use macro: expect_error
- [ ] tests/known_cases.rs:76 - Remove or use macro: assert_contains_ordered
- [ ] tests/known_cases.rs:90 - Remove or use macro: assert_contains
- [ ] tests/known_cases.rs:15 - Remove or use function: run_e2e_test
- [ ] tests/caching.rs:76 - Use .is_err() instead of matches!(..., Err(_))
- [ ] Run cargo clippy to verify all warnings are resolved