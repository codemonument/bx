---
# bx-ucli
title: macOS ARM64 build für bx in GitHub Releases
status: todo
type: feature
priority: normal
created_at: 2026-01-21T10:49:01Z
updated_at: 2026-01-21T10:49:01Z
---

Add macOS ARM64 (Apple Silicon) builds to the GitHub Releases workflow.

## Context
Currently the CD workflow (`cd.yml`) builds for:
- Linux amd64
- macOS amd64 (via `macos-latest`)  
- Windows amd64
- Linux musl amd64

Missing: **macOS ARM64** for Apple Silicon Macs (M1/M2/M3/M4).

## Checklist
- [ ] Add matrix entry for `macos-latest` with ARM64 target (`aarch64-apple-darwin`)
- [ ] Install ARM64 Rust target in workflow step
- [ ] Cross-compile or use native ARM runner for the build
- [ ] Upload `bx-macos-arm64` and `bonnie-macos-arm64` assets to release
- [ ] Test workflow with a tag push or manual trigger