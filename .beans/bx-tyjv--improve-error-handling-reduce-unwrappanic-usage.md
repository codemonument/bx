---
# bx-tyjv
title: Improve error handling (reduce unwrap/panic usage)
status: todo
type: task
priority: low
created_at: 2026-01-22T08:56:40Z
updated_at: 2026-01-22T08:56:40Z
---

The codebase has 8 unwrap() calls and several panic!() calls that could be improved.

## unwrap() locations to review
- [ ] src/schema.rs - Multiple unwraps on cmd/subcommands with 'We know more than the compiler' comments
- [ ] src/bones.rs:121 - exit_code.unwrap() after is_ok() check
- [ ] src/bones.rs - 3 Regex::new().unwrap() calls (consider using lazy_static or once_cell)

## panic!() locations to review
- [ ] src/version.rs:61 - Critical logic failure panic
- [ ] src/schema.rs - Critical logic failure panic
- [ ] Test code panic!() calls are acceptable

## Recommendations
- Consider using expect() with descriptive messages instead of unwrap()
- For regex, consider lazy_static! or once_cell::Lazy for compile-time safety
- Replace panics with Result returns where possible