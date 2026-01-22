---
# bx-2ocd
title: Update Bonnie branding to bx throughout codebase
status: todo
type: task
priority: normal
created_at: 2026-01-22T08:56:14Z
updated_at: 2026-01-22T08:56:14Z
---

Many files still reference 'Bonnie' instead of 'bx'. This is a rebranding cleanup task.

## Code Constants to Rename
- [ ] src/version.rs: BONNIE_VERSION → BX_VERSION
- [ ] src/cache.rs: DEFAULT_BONNIE_CACHE_PATH → DEFAULT_BX_CACHE_PATH
- [ ] src/cache.rs: BONNIE_CACHE env var → BX_CACHE (keep BONNIE_CACHE as fallback)
- [ ] src/get_cfg.rs: DEFAULT_BONNIE_CFG_PATH (keep for compatibility)
- [ ] src/template.rs: BONNIE_TEMPLATE → BX_TEMPLATE (keep BONNIE_TEMPLATE as fallback)
- [ ] src/template.rs: ~/.bonnie/ → ~/.bx/ (keep ~/.bonnie/ as fallback)

## Error Messages to Update
- [ ] src/cache.rs: Update all 'Bonnie' references in error messages
- [ ] src/init.rs: Update 'Bonnie configuration file' messages
- [ ] src/raw_schema.rs: Update version error message
- [ ] src/bin/main.rs: Update help and error messages

## Help Text
- [ ] src/help.rs: Update entire help page to reference bx instead of Bonnie

## Documentation
- [ ] spec.md: Update title and content
- [ ] CONTRIBUTING.md: Update Bonnie references
- [ ] .github/ISSUE_TEMPLATE/bug_report.md: Update template
- [ ] .github/ISSUE_TEMPLATE/feature_request.md: Update template