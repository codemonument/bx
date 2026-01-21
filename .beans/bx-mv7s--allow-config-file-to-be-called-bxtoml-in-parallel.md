---
# bx-mv7s
title: Allow config file to be called bx.toml in parallel to bonnie.toml
status: completed
type: feature
priority: normal
created_at: 2026-01-21T10:55:13Z
updated_at: 2026-01-21T11:09:27Z
---

## Summary

Currently, the CLI only looks for `bonnie.toml` as the configuration file. Since this is a fork with the `bx` binary, users should also be able to use `bx.toml` as an alternative filename.

## Requirements

- Support `bx.toml` as a valid config filename alongside `bonnie.toml`
- When both files exist, decide on precedence (suggested: `bx.toml` takes priority, or error if both exist)
- Update help text and documentation to reflect the new option
- Ensure all code paths that load the config file support both names

## Implementation Notes

**Priority order**: BX_CONF > BONNIE_CONF > ./bx.toml (if exists) > ./bonnie.toml

Files changed:
- `src/get_cfg.rs`: Added `get_cfg_path()` function with priority logic + 4 unit tests
- `src/lib.rs`: Exported `get_cfg_path`
- `src/bin/main.rs`: Uses centralized `get_cfg_path()` instead of hardcoded fallback
- `src/help.rs`: Updated help text to mention both config files and env vars
- `Cargo.toml`: Added `tempfile` dev dependency for tests

## Checklist

- [x] Find where config file loading happens in the codebase
- [x] Update config loading logic to check for both `bx.toml` and `bonnie.toml`
- [x] Define and implement precedence rules when both files exist
- [x] Update CLI help text to mention both config file names
- [x] Add tests for the new config file name
- [x] Update README/documentation if applicable (not needed - README doesn't mention config filename)