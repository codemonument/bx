---
# bx-tyjv
title: Improve error handling (reduce unwrap/panic usage)
status: completed
type: task
priority: low
created_at: 2026-01-22T08:56:40Z
updated_at: 2026-01-22T12:23:56Z
---

The codebase has 8 unwrap() calls and several panic!() calls that could be improved.

## unwrap() locations to review
- [x] src/schema.rs - Multiple unwraps on cmd/subcommands - replaced with expect() with descriptive messages
- [x] src/bones.rs:197 - exit_code.unwrap() after is_ok() check - replaced with expect()
- [x] src/bones.rs - 3 Regex::new().unwrap() calls - kept as-is: compile-time constant patterns, documented as safe

## panic!() locations to review
- [x] src/version.rs:69 - Critical logic failure panic - kept: appropriate for impossible states
- [x] src/schema.rs:274 - Critical logic failure panic - kept: appropriate for impossible states
- [x] Test code panic!() calls are acceptable - unchanged

## Recommendations
- Consider using expect() with descriptive messages instead of unwrap()
- For regex, consider lazy_static! or once_cell::Lazy for compile-time safety
- Replace panics with Result returns where possible