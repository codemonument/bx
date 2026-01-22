---
# bx-czdp
title: Remove or update stale files
status: todo
type: task
priority: low
created_at: 2026-01-22T08:56:26Z
updated_at: 2026-01-22T08:56:26Z
---

Several files are stale or reference the old Bonnie repository.

## Checklist
- [ ] install_scripts/musl.sh: Update to reference codemonument/bx instead of arctic-hen7/bonnie
- [ ] wiki/: Empty directory from git submodule pointing to bonnie.wiki - decide whether to populate, remove submodule, or delete
- [ ] .gitmodules: Update or remove the wiki submodule entry
- [ ] .github/CODEOWNERS: Update to list only @bjesuiter (remove @arctic_hen7)