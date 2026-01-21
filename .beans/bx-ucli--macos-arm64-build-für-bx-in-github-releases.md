---
# bx-ucli
title: macOS ARM64 build für bx in GitHub Releases
status: in-progress
type: feature
priority: normal
created_at: 2026-01-21T10:49:01Z
updated_at: 2026-01-21T11:43:56Z
---

Add macOS ARM64 (Apple Silicon) builds to the GitHub Releases workflow.

## Context
Currently the CD workflow (`cd.yml`) builds for:
- Linux amd64
- macOS amd64 (via `macos-latest`)  
- Windows amd64
- Linux musl amd64

Missing: **macOS ARM64** for Apple Silicon Macs (M1/M2/M3/M4).

## Approach
Used native ARM64 runner (`macos-14`) instead of cross-compilation. This is simpler and more reliable.
Also changed `macos-latest` → `macos-13` to explicitly use Intel runner for amd64 builds.

## Checklist
- [x] Add matrix entry for ARM64 target (used `macos-14` native ARM runner instead of cross-compile)
- [x] Install ARM64 Rust target in workflow step (not needed - native runner already has ARM64 toolchain)
- [x] Cross-compile or use native ARM runner for the build (chose native ARM runner)
- [x] Upload `bx-macos-arm64` and `bonnie-macos-arm64` assets to release (configured in workflow)
- [ ] Test workflow with a tag push or manual trigger