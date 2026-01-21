---
# bx-ucli
title: macOS ARM64 build für bx in GitHub Releases
status: completed
type: feature
priority: normal
created_at: 2026-01-21T10:49:01Z
updated_at: 2026-01-21T11:52:25Z
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
Both macOS builds use `macos-14` (ARM runner) with explicit target specification:
- ARM64: `--target aarch64-apple-darwin` (native build on ARM runner)
- Intel: `--target x86_64-apple-darwin` (cross-compile from ARM runner)

Note: `macos-13` (Intel runner) was retired by GitHub, so cross-compilation is required for Intel builds.

## Checklist
- [x] Add matrix entry for ARM64 target (used `macos-14` native ARM runner)
- [x] Install ARM64 Rust target in workflow step (conditional on matrix.target)
- [x] Cross-compile or use native ARM runner for the build (ARM native + Intel cross-compile)
- [x] Upload `bx-macos-arm64` and `bonnie-macos-arm64` assets to release (configured in workflow)
- [x] Test workflow with a tag push or manual trigger (tested via workflow_dispatch - all assets uploaded)