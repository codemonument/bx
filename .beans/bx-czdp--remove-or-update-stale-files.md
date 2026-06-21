---
# bx-czdp
title: Remove or update stale files
status: completed
type: task
priority: low
created_at: 2026-01-22T08:56:26Z
updated_at: 2026-01-22T12:21:34Z
---

Several files are stale or reference the old Bonnie repository.

## Checklist
- [x] install_scripts/musl.sh: Update to reference codemonument/bx instead of arctic-hen7/bonnie
- [x] wiki/: Empty directory from git submodule pointing to bonnie.wiki - removed
- [x] .gitmodules: Removed the wiki submodule entry
- [x] .github/CODEOWNERS: Update to list only @bjesuiter (remove @arctic_hen7)