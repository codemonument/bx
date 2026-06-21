---
# bx-2ocd
title: Update Bonnie branding to bx throughout codebase
status: completed
type: task
priority: normal
created_at: 2026-01-22T08:56:14Z
updated_at: 2026-01-22T12:19:53Z
---

Many files still reference 'Bonnie' instead of 'bx'. This is a rebranding cleanup task.

## Code Constants to Rename
- [x] src/version.rs: BONNIE_VERSION → BX_VERSION (already deprecated, CLI_VERSION is used)
- [x] src/cache.rs: DEFAULT_BONNIE_CACHE_PATH → DEFAULT_BX_CACHE_PATH
- [x] src/cache.rs: BONNIE_CACHE env var → BX_CACHE (keep BONNIE_CACHE as fallback)
- [x] src/get_cfg.rs: DEFAULT_BONNIE_CFG_PATH (kept for compatibility)
- [x] src/template.rs: BONNIE_TEMPLATE → BX_TEMPLATE (keep BONNIE_TEMPLATE as fallback)
- [x] src/template.rs: ~/.bonnie/ → ~/.bx/ (keep ~/.bonnie/ as fallback)

## Error Messages to Update
- [x] src/cache.rs: Update all 'Bonnie' references in error messages
- [x] src/init.rs: Update 'Bonnie configuration file' messages
- [x] src/raw_schema.rs: Update version error message
- [x] src/bin/main.rs: Update help and error messages

## Help Text
- [x] src/help.rs: Update entire help page to reference bx instead of Bonnie

## Documentation
- [x] spec.md: Update title and content
- [x] CONTRIBUTING.md: Update Bonnie references
- [x] .github/ISSUE_TEMPLATE/bug_report.md: Update template
- [x] .github/ISSUE_TEMPLATE/feature_request.md: Update template