# bx CLI Modernization Guide

Research notes for modernizing the bx CLI codebase to follow current Rust best practices.

## Summary

| Area | Current | Recommended | Priority |
|------|---------|-------------|----------|
| Error Handling | `Result<T, String>` | `anyhow` | HIGH |
| Argument Parsing | Manual `env::args()` | `clap` with derive | HIGH |
| Testing | Custom macros | `assert_cmd` | MEDIUM |
| Code Organization | Flat modules | Hierarchical | MEDIUM |
| Rust Edition | 2018 | 2021 or 2024 | LOW |
| Async | Synchronous | Keep sync | N/A |

**Key Decision: Keep synchronous execution.** bx executes commands sequentially - async adds complexity without benefit.

---

## HIGH PRIORITY

### 1. Error Handling: Migrate to `anyhow`

**Problem:** `Result<T, String>` loses error context and type information.

**Solution:**
```rust
// Cargo.toml
[dependencies]
anyhow = "1.0"

// Before
fn core() -> Result<i32, String> {
    match get_cfg() {
        Ok(cfg) => { /* ... */ }
        Err(_) => Err(format!("Error reading config"))
    }
}

// After
use anyhow::{Context, Result};

fn core() -> Result<i32> {
    let cfg_str = get_cfg()
        .context("Failed to load configuration file")?;
    Ok(exit_code)
}
```

**Benefits:** Error chains, context preservation, better debugging, 40% less boilerplate.

---

### 2. Argument Parsing: Migrate to `clap`

**Problem:** Manual parsing is error-prone, no auto-generated help, hard to maintain.

**Solution:**
```rust
// Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }

// src/cli/args.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bx", version, about = "Simple, cross-platform command aliases")]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[arg(short = 'c', long, global = true, env = "BX_CONF")]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new bx configuration
    Init {
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Cache the configuration for faster startup
    Cache,
    /// Show help for a command
    Help { command: Option<String> },
    /// Run a configured command
    #[command(external_subcommand)]
    Run(Vec<String>),
}

// src/bin/main.rs
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let exit_code = execute_command(cli)?;
    std::process::exit(exit_code)
}
```

**Benefits:** Auto help/version, type-safe, validation, 50% less code.

---

## MEDIUM PRIORITY

### 3. Testing: Add `assert_cmd`

```toml
# Cargo.toml
[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.1"
assert_fs = "1.1"
```

```rust
// tests/cli.rs
use assert_cmd::Command;
use predicates::prelude::*;
use assert_fs::prelude::*;

#[test]
fn test_version_flag() {
    Command::cargo_bin("bx")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_missing_config() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("bx")
        .unwrap()
        .current_dir(temp.path())
        .arg("build")
        .assert()
        .failure();
}

#[test]
fn test_init_creates_config() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("bx")
        .unwrap()
        .current_dir(temp.path())
        .args(["--init"])
        .assert()
        .success();
    temp.child("bonnie.toml").assert(predicate::path::exists());
}
```

---

### 4. Code Organization: Hierarchical Modules

**Current:**
```
src/
├── bones.rs, cache.rs, schema.rs, raw_schema.rs, ...
```

**Recommended:**
```
src/
├── lib.rs
├── bin/main.rs
├── cli/
│   ├── mod.rs
│   ├── args.rs
│   └── commands.rs
├── config/
│   ├── mod.rs
│   ├── schema.rs
│   └── parser.rs
├── execution/
│   ├── mod.rs
│   └── bones.rs
├── cache.rs
├── error.rs
└── version.rs
```

---

## LOW PRIORITY

### 5. Rust Edition Upgrade

```bash
# Upgrade from 2018 to 2021 (stable, well-tested)
cargo fix --edition
# Update Cargo.toml: edition = "2021"
cargo test
```

Consider 2024 edition later once it's more mature.

---

## Dependency Changes

### Add
```toml
[dependencies]
anyhow = "1.0"
clap = { version = "4", features = ["derive"] }

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.1"
assert_fs = "1.1"
```

### Keep (no changes needed)
```toml
toml = "0.5"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dotenv = "0.15"
regex = "1"
home = "0.5"
```

---

## Implementation Order

1. **Error handling** - Add anyhow, update return types, add context
2. **Argument parsing** - Add clap, create cli module, migrate parsing
3. **Testing** - Add dev-deps, create tests/cli.rs
4. **Code organization** - Refactor into modules (optional, can defer)
5. **Edition upgrade** - Run cargo fix (optional, can defer)

Each phase is independent and can be done incrementally.

---

## Resources

- [Rust CLI Book](https://rust-cli.github.io/book/)
- [anyhow docs](https://docs.rs/anyhow)
- [clap docs](https://docs.rs/clap)
- [assert_cmd docs](https://docs.rs/assert_cmd)
