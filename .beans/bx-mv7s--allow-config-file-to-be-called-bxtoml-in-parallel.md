---
# bx-mv7s
title: Allow config file to be called bx.toml in parallel to bonnie.toml
status: todo
type: feature
created_at: 2026-01-21T10:55:13Z
updated_at: 2026-01-21T10:55:13Z
---

## Summary

Currently, the CLI only looks for `bonnie.toml` as the configuration file. Since this is a fork with the `bx` binary, users should also be able to use `bx.toml` as an alternative filename.

## Requirements

- Support `bx.toml` as a valid config filename alongside `bonnie.toml`
- When both files exist, decide on precedence (suggested: `bx.toml` takes priority, or error if both exist)
- Update help text and documentation to reflect the new option
- Ensure all code paths that load the config file support both names

## Checklist

- [ ] Find where config file loading happens in the codebase
- [ ] Update config loading logic to check for both `bx.toml` and `bonnie.toml`
- [ ] Define and implement precedence rules when both files exist
- [ ] Update CLI help text to mention both config file names
- [ ] Add tests for the new config file name
- [ ] Update README/documentation if applicable