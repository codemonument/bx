---
# bx-lgee
title: Address TODO/FIXME comments
status: completed
type: task
priority: low
created_at: 2026-01-22T08:56:34Z
updated_at: 2026-01-22T12:22:30Z
---

Several TODO and FIXME comments exist in the codebase that should be addressed.

## Checklist
- [x] src/bones.rs:75 - 'TODO document the above behaviour' - Removed; behavior is documented in comment above it
- [x] src/bin/main.rs:39 - 'TODO add a checker for the executable...' - No longer exists after clap migration
- [x] src/bin/main.rs:83 - 'TODO doc instead/as well?' - No longer exists after clap migration
- [x] src/schema.rs:313 - 'FIXME' - Removed; code works correctly
- [ ] src/schema.rs:390 - 'TODO handle %% as [...]' - Deferred: feature enhancement to show `[...]` in help for variadic args