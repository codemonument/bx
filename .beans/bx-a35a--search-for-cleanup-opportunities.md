---
# bx-a35a
title: Search for cleanup opportunities
status: completed
type: task
priority: normal
created_at: 2026-01-21T23:26:32Z
updated_at: 2026-01-22T08:55:57Z
---

## Cleanup Opportunities Found

### Version Inconsistencies (CRITICAL)
- **Cargo.toml** says `version = "0.4.1"`
- **src/version.rs** has `pub const BONNIE_VERSION: &str = "0.3.2";`
- **bonnie.toml** (both root and src/) say `version = "0.3.2"`
- **README.md** references `v0.4.0` in Docker section but Cargo.toml says 0.4.1

**Action needed**: Update all version references to be consistent. The authoritative source should be Cargo.toml (0.4.1).

### CHANGELOG.md - Duplicate Entries
- CHANGELOG.md has **3 duplicate blocks** for version 0.3.2 (lines 14-39, 40-63, 64-87)
- Each block is identical content repeated

**Action needed**: Remove duplicate 0.3.2 entries, keep only one.

### Bonnie Branding References (Should Update to bx)
Many files still reference "Bonnie" instead of "bx":

**Code files:**
- `src/version.rs:4` - `pub const BONNIE_VERSION: &str` - should be `BX_VERSION`
- `src/cache.rs:7` - `DEFAULT_BONNIE_CACHE_PATH` - should use bx
- `src/cache.rs:13,16,17` - `BONNIE_CACHE` env var - should be `BX_CACHE`
- `src/cache.rs:45` - error message references "Bonnie configuration"
- `src/get_cfg.rs:8` - `DEFAULT_BONNIE_CFG_PATH` - should update
- `src/get_cfg.rs:13,25,26,31,64,65` - references to bonnie_conf
- `src/bin/main.rs:39,52,67,90` - error/help messages reference Bonnie
- `src/help.rs:6,9,13,17,19,23` - help text references Bonnie
- `src/init.rs:6` - "Creates a new Bonnie configuration file"
- `src/init.rs:10` - error message references "Bonnie configuration file"
- `src/template.rs:3,14,22,30` - `BONNIE_VERSION` constant and BONNIE_TEMPLATE env var
- `src/template.rs:30` - references `~/.bonnie/` directory

**Documentation:**
- `spec.md` - Title is "Bonnie Specification" and content references Bonnie
- `CONTRIBUTING.md:3` - "contribute to Bonnie"
- `CONTRIBUTING.md:11` - references Bonnie issues
- `.github/CODEOWNERS:1` - lists @arctic_hen7 (original Bonnie author)
- `.github/ISSUE_TEMPLATE/bug_report.md:14` - "The simplest `bonnie.toml`"
- `.github/ISSUE_TEMPLATE/bug_report.md:17` - "bonnie.toml"
- `.github/ISSUE_TEMPLATE/feature_request.md:14` - "make Bonnie better"
- `.github/ISSUE_TEMPLATE/feature_request.md:17` - "triage label removed"

**Error Messages:**
Many error messages say "Bonnie" instead of "bx" or "bx-cli":
- src/cache.rs:16 - "path to your Bonnie cache file"
- src/cache.rs:36 - "caching your parsed Bonnie configuration"
- src/cache.rs:40 - "cached Bonnie configuration"
- src/cache.rs:45 - "Your Bonnie configuration has been successfully cached"
- src/cache.rs:69 - "cached Bonnie configuration"
- src/cache.rs:75 - "cached Bonnie configuration"
- src/raw_schema.rs:26 - "Your Bonnie configuration file appears to be missing"
- src/cache.rs:36 - "caching your parsed Bonnie configuration"

**Environment Variables:**
Should standardize env var names:
- Keep `BX_CONF` and `BX_TEMPLATE` (already correct)
- `BONNIE_CONF` → `BX_CONF_COMPAT` or keep for compatibility
- `BONNIE_CACHE` → `BX_CACHE`
- `BONNIE_TEMPLATE` → `BX_TEMPLATE`

### File Paths/Constants (Should Use bx Names)
- `DEFAULT_BONNIE_CACHE_PATH` ("./.bonnie.cache.json") → use `./.bx.cache.json`
- `DEFAULT_BONNIE_CFG_PATH` ("./bonnie.toml") - keep for compatibility
- Template directory `~/.bonnie/` → `~/.bx/`

### Unused Code / Dead Code
From clippy warnings found:
- **tests/known_cases.rs:9** - Unused import: `BONNIE_VERSION`
- **tests/known_cases.rs:34** - Unused macro: `expect_exit_code`
- **tests/known_cases.rs:56** - Unused macro: `expect_error`
- **tests/known_cases.rs:76** - Unused macro: `assert_contains_ordered`
- **tests/known_cases.rs:90** - Unused macro: `assert_contains`
- **tests/known_cases.rs:15** - Unused function: `run_e2e_test`
- **tests/caching.rs:76** - Use `.is_err()` instead of `matches!(..., Err(_))`

### TODO/FIXME Comments (Should Be Addressed)
From grep found:
- `src/bones.rs:79` - "TODO document the above behaviour"
- `src/bin/main.rs:39` - "TODO add a checker for the executable..."
- `src/bin/main.rs:83` - "TODO 'doc' instead/as well?"
- `src/schema.rs:317` - "FIXME"
- `src/schema.rs:394` - "TODO handle '%%' as `[...]`"

### Panic / unwrap() Calls (Error Handling Improvements)
Found 8 unwrap() calls:
- `src/schema.rs` - Multiple unwraps on cmd/subcommands with comments "We know more than the compiler"
- `src/bones.rs:121` - `exit_code.unwrap()` on is_ok()
- `src/bones.rs` - 3 Regex::new().unwrap() calls (static regex compilation)
- `src/schema.rs` - Several unwraps that could be improved

### Stale / Outdated Files
1. **install_scripts/musl.sh** - References `arctic-hen7/bonnie` (old repo)
   - Should reference `codemonument/bx` instead
2. **wiki/** directory** - Empty directory (from git submodule)
   - The git submodule points to `bonnie.wiki` (old repo)
   - Should either populate or remove the submodule
3. **.github/CODEOWNERS** - Lists @arctic_hen7 who is not maintaining bx
   - Should update to @bjesuiter only or remove file
4. **bonnie.toml** files - Should optionally create bx.toml as default

### Dependency Updates
- **toml**: v0.5 (from 2020) → Consider upgrading to latest
- **regex**: v1.5.5 → Latest is v1.10.x
- **serde**: v1 (already latest feature version)
- **dotenv**: v0.15 → Latest is v0.15.x (current)

### Potential Code Improvements
- Large files: `schema.rs` (472 lines), `raw_schema.rs` (376 lines), `bones.rs` (366 lines), `version.rs` (312 lines)
- Consider breaking down large functions
- Code duplication in error message patterns
- Consider using a dedicated CLI argument parsing library (clap)

### .gitignore Additions
Consider adding:
- `.DS_Store` (macOS)
- `*.swp`, `*.swo` (vim)
- `*~` (backup files)
- `.beans/` (if tracking beans separately)

### GitHub Workflow Improvements
- cd-dev.yml doesn't have release/upload steps (incomplete)
- cd.yml uses hardcoded `main` branch for git push

---

## Follow-up Beans to Create
- [ ] Fix version inconsistencies (create separate bean)
- [ ] Update Bonnie branding to bx throughout codebase (create separate bean)
- [ ] Clean up CHANGELOG.md duplicate entries (create separate bean)
- [ ] Update/install remove stale files (install_scripts, wiki, CODEOWNERS)
- [ ] Fix clippy warnings (unused code in tests)
- [ ] Address TODO/FIXME comments
- [ ] Improve error handling (reduce unwrap/panic usage)