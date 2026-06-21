---
# bx-4dgd
title: Create auto-update logic for config files with breaking changes
status: draft
type: feature
created_at: 2026-01-22T11:00:38Z
updated_at: 2026-01-22T11:00:38Z
---

## Goal
When a user's config file has an older version that's still in SUPPORTED_CONFIG_VERSIONS but not the latest, provide an upgrade path.

## Context
The version system now supports multiple config versions via SUPPORTED_CONFIG_VERSIONS. When breaking changes are introduced:
1. Add new version to SUPPORTED_CONFIG_VERSIONS
2. Users with old configs should be offered migration

## Requirements
- Detect when config version differs from LATEST_CONFIG_VERSION
- Provide a `bx --upgrade` or `bx -u` command to migrate config files
- Migration should transform old syntax to new syntax where possible
- For manual changes, provide clear instructions

## Migration System Design
- Each config version should have an upgrade function to the next version
- Chain upgrades: 0.3.2 -> 0.4.x -> 0.5.x etc.
- Store migration functions in a module (e.g., src/migrations.rs)
- Warn (don't error) when running older config versions that are still supported

## Checklist
- [ ] Add warning when config version != LATEST_CONFIG_VERSION (but still supported)
- [ ] Create src/migrations.rs module
- [ ] Add `--upgrade` CLI flag
- [ ] Implement upgrade logic that backs up original and writes upgraded config
- [ ] Add tests for migration paths