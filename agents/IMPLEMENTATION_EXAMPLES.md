# Implementation Examples: Modern Rust CLI Best Practices for bx

This document provides concrete code examples for implementing the recommended best practices.

---

## 1. ERROR HANDLING WITH ANYHOW

### Step 1: Update Cargo.toml
```toml
[dependencies]
anyhow = "1.0"
# ... other dependencies
```

### Step 2: Update src/bin/main.rs
**Before:**
```rust
fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code)
}

fn real_main() -> i32 {
    let res = core();
    match res {
        Ok(exit_code) => exit_code,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    }
}

fn core() -> Result<i32, String> {
    // ... implementation
}
```

**After:**
```rust
use anyhow::Result;

fn main() -> Result<()> {
    let exit_code = core()?;
    std::process::exit(exit_code)
}

fn core() -> Result<i32> {
    let cfg_path = resolve_cfg_path(
        std::env::var("BX_CONF").ok().as_deref(),
        std::env::var("BONNIE_CONF").ok().as_deref(),
        std::path::Path::new(DEFAULT_BX_CFG_PATH).exists(),
    );
    
    let cfg_str = get_cfg()
        .context("Failed to load configuration file")?;
    
    let cfg = Config::new(&cfg_str)
        .context("Failed to parse configuration")?
        .to_final(BONNIE_VERSION, &mut std::io::stdout())
        .context("Failed to finalize configuration")?;
    
    // ... rest of logic
    Ok(exit_code)
}
```

### Step 3: Update Error-Returning Functions
**Before:**
```rust
pub fn get_cfg() -> Result<String, String> {
    let path = resolve_cfg_path(
        env::var("BX_CONF").ok().as_deref(),
        env::var("BONNIE_CONF").ok().as_deref(),
        Path::new(DEFAULT_BX_CFG_PATH).exists(),
    );
    let cfg_string = fs::read_to_string(&path);
    match cfg_string {
        Ok(cfg_string) => Ok(cfg_string),
        Err(_) => Err(format!("Error reading configuration file at '{}', make sure the file is present in this directory and you have the permissions to read it.", path))
    }
}
```

**After:**
```rust
use anyhow::{Context, Result};

pub fn get_cfg() -> Result<String> {
    let path = resolve_cfg_path(
        env::var("BX_CONF").ok().as_deref(),
        env::var("BONNIE_CONF").ok().as_deref(),
        Path::new(DEFAULT_BX_CFG_PATH).exists(),
    );
    
    fs::read_to_string(&path)
        .with_context(|| format!(
            "Failed to read configuration file at '{}'. \
             Make sure the file exists and you have read permissions.",
            path
        ))
}
```

---

## 2. ARGUMENT PARSING WITH CLAP

### Step 1: Update Cargo.toml
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
# ... other dependencies
```

### Step 2: Create src/cli/mod.rs
```rust
pub mod args;
pub mod commands;

pub use args::Cli;
pub use commands::execute_command;
```

### Step 3: Create src/cli/args.rs
```rust
use clap::{Parser, Subcommand};

/// Simple, cross-platform command aliases with superpowers
#[derive(Parser, Debug)]
#[command(name = "bx")]
#[command(about, long_about = None)]
#[command(version)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Path to configuration file
    #[arg(short = 'c', long, global = true, env = "BX_CONF")]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new bx configuration
    Init {
        /// Template file to use
        #[arg(short, long)]
        template: Option<String>,
    },

    /// Cache the configuration for faster startup
    Cache,

    /// Show help for a command
    Help {
        /// Command name to show help for
        command: Option<String>,
    },

    /// Run a configured command
    #[command(external_subcommand)]
    Run(Vec<String>),
}
```

### Step 3: Create src/cli/commands.rs
```rust
use anyhow::Result;
use crate::Cli;

pub fn execute_command(cli: Cli) -> Result<i32> {
    match cli.command {
        Some(crate::cli::args::Commands::Init { template }) => {
            crate::init(template.as_deref())?;
            println!("A new bx configuration file has been initialized!");
            Ok(0)
        }
        Some(crate::cli::args::Commands::Cache) => {
            crate::cache(&cfg, &mut std::io::stdout(), None)?;
            Ok(0)
        }
        Some(crate::cli::args::Commands::Help { command }) => {
            let msg = cfg.document(command)?;
            println!("{}", msg);
            Ok(0)
        }
        Some(crate::cli::args::Commands::Run(args)) => {
            // Execute the command
            let (command_to_run, command_name, relevant_args) = 
                cfg.get_command_for_args(&args)?;
            let bone = command_to_run.prepare(&command_name, &relevant_args, &cfg.default_shell)?;
            bone.run(&command_name, cli.verbose, &mut std::io::stdout())
        }
        None => {
            // Show help if no command provided
            println!("{}", Cli::command().render_help());
            Ok(0)
        }
    }
}
```

### Step 4: Update src/bin/main.rs
**Before:**
```rust
fn core() -> Result<i32, String> {
    let mut prog_args: Vec<String> = env::args().collect();
    let _executable_name = prog_args.remove(0);
    
    if !prog_args.is_empty() {
        if prog_args[0] == "-v" || prog_args[0] == "--version" {
            // ... handle version
        } else if prog_args[0] == "-i" || prog_args[0] == "--init" {
            // ... handle init
        }
    }
    // ... rest of logic
}
```

**After:**
```rust
use lib::cli::Cli;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let exit_code = core(cli)?;
    std::process::exit(exit_code)
}

fn core(cli: Cli) -> Result<i32> {
    lib::cli::commands::execute_command(cli)
}
```

---

## 3. TESTING WITH ASSERT_CMD

### Step 1: Update Cargo.toml
```toml
[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.1"
assert_fs = "1.1"
```

### Step 2: Create tests/cli.rs
```rust
use assert_cmd::Command;
use predicates::prelude::*;
use assert_fs::TempDir;

#[test]
fn test_version_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bx")?;
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.4.1"));
    Ok(())
}

#[test]
fn test_help_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bx")?;
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Simple, cross-platform"));
    Ok(())
}

#[test]
fn test_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bx")?;
    cmd.arg("help")
        .assert()
        .success();
    Ok(())
}

#[test]
fn test_missing_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let mut cmd = Command::cargo_bin("bx")?;
    cmd.current_dir(temp.path())
        .arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read configuration file"));
    Ok(())
}

#[test]
fn test_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let mut cmd = Command::cargo_bin("bx")?;
    cmd.current_dir(temp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("initialized"));
    
    // Verify config file was created
    assert!(temp.child("bx.toml").path().exists());
    Ok(())
}

#[test]
fn test_cache_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let config = temp.child("bx.toml");
    config.write_str("version = \"0.3.2\"\n[scripts]\nbuild = \"echo building\"")?;
    
    let mut cmd = Command::cargo_bin("bx")?;
    cmd.current_dir(temp.path())
        .arg("--cache")
        .assert()
        .success();
    
    // Verify cache file was created
    assert!(temp.child(".bx.cache.json").path().exists());
    Ok(())
}

#[test]
fn test_verbose_flag() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    let config = temp.child("bx.toml");
    config.write_str("version = \"0.3.2\"\n[scripts]\nbuild = \"echo building\"")?;
    
    let mut cmd = Command::cargo_bin("bx")?;
    cmd.current_dir(temp.path())
        .arg("--verbose")
        .arg("build")
        .assert()
        .success();
    Ok(())
}
```

---

## 4. CODE ORGANIZATION

### Step 1: Create Module Structure
```bash
mkdir -p src/cli
mkdir -p src/config
mkdir -p src/execution
mkdir -p src/cache
```

### Step 2: Create src/lib.rs
```rust
pub mod cli;
pub mod config;
pub mod execution;
pub mod cache;
pub mod error;
pub mod version;

// Re-export commonly used items
pub use cli::Cli;
pub use error::Result;
pub use version::BONNIE_VERSION;

// Re-export public functions
pub use cache::{cache, cache_exists, load_from_cache};
pub use config::get_cfg;
pub use cli::help;
pub use config::init;
```

### Step 3: Create src/error.rs
```rust
use std::fmt;

/// Custom error type for bx
#[derive(Debug)]
pub enum Error {
    /// Configuration file not found
    ConfigNotFound(String),
    
    /// Invalid configuration syntax
    InvalidConfig(String),
    
    /// Command execution failed
    ExecutionFailed(String),
    
    /// IO error
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ConfigNotFound(path) => {
                write!(f, "Configuration file not found at '{}'", path)
            }
            Error::InvalidConfig(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            Error::ExecutionFailed(msg) => {
                write!(f, "Command execution failed: {}", msg)
            }
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

/// Result type for bx operations
pub type Result<T> = std::result::Result<T, anyhow::Error>;
```

### Step 4: Reorganize Existing Code
```rust
// src/config/mod.rs
pub mod parser;
pub mod schema;

pub use parser::get_cfg;
pub use schema::Config;

// src/execution/mod.rs
pub mod bones;
pub mod shell;

pub use bones::Bone;

// src/cache/mod.rs
pub mod serialization;

pub use serialization::{cache, cache_exists, load_from_cache};
```

---

## 5. RUST EDITION UPGRADE

### Step 1: Run cargo fix
```bash
cargo fix --edition
```

### Step 2: Update Cargo.toml
```toml
[package]
name = "bx-cli"
version = "0.4.1"
edition = "2024"  # Changed from "2018"
# ... rest of config
```

### Step 3: Test
```bash
cargo test
cargo clippy
cargo fmt
```

---

## Migration Checklist

### Phase 1: Error Handling
- [ ] Add `anyhow` to Cargo.toml
- [ ] Update `main.rs` return type to `Result<()>`
- [ ] Update `core()` to return `Result<i32>`
- [ ] Replace `Result<T, String>` with `anyhow::Result<T>`
- [ ] Add `.context()` to error sites
- [ ] Test error messages
- [ ] Commit changes

### Phase 2: Argument Parsing
- [ ] Add `clap` to Cargo.toml
- [ ] Create `src/cli/mod.rs`
- [ ] Create `src/cli/args.rs` with `Cli` struct
- [ ] Create `src/cli/commands.rs` with command logic
- [ ] Update `src/bin/main.rs` to use clap
- [ ] Remove manual argument parsing
- [ ] Test all commands
- [ ] Commit changes

### Phase 3: Testing
- [ ] Add dev-dependencies to Cargo.toml
- [ ] Create `tests/cli.rs`
- [ ] Write tests for all commands
- [ ] Write tests for error cases
- [ ] Write tests for config loading
- [ ] Run full test suite
- [ ] Commit changes

### Phase 4: Code Organization
- [ ] Create module directories
- [ ] Create `src/error.rs`
- [ ] Create `src/cli/mod.rs` and submodules
- [ ] Create `src/config/mod.rs` and submodules
- [ ] Create `src/execution/mod.rs` and submodules
- [ ] Create `src/cache/mod.rs` and submodules
- [ ] Update `src/lib.rs` exports
- [ ] Update module documentation
- [ ] Test compilation
- [ ] Commit changes

### Phase 5: Edition Upgrade
- [ ] Run `cargo fix --edition`
- [ ] Update `Cargo.toml` to edition 2024
- [ ] Run full test suite
- [ ] Address any warnings
- [ ] Commit changes

---

## Testing the Implementation

### Run All Tests
```bash
cargo test
```

### Run Specific Test
```bash
cargo test test_version_flag
```

### Run with Output
```bash
cargo test -- --nocapture
```

### Run Integration Tests Only
```bash
cargo test --test cli
```

### Check Code Quality
```bash
cargo fmt --check
cargo clippy
cargo check
```

---

## Rollback Plan

If issues arise during implementation:

1. **For each phase:** Create a separate branch
2. **Before merging:** Run full test suite
3. **If problems:** Revert the branch
4. **Document:** What went wrong and why

Example:
```bash
git checkout -b feature/error-handling
# ... make changes ...
cargo test
git commit -m "feat: migrate to anyhow error handling"
git push origin feature/error-handling
# Create PR for review
```

---

## Performance Considerations

### Error Handling
- `anyhow` has minimal overhead
- Error chains are lazy-evaluated
- No performance impact expected

### Argument Parsing
- `clap` with derive is compiled
- Parsing happens once at startup
- No performance impact expected

### Testing
- Dev-dependencies don't affect binary size
- Tests run separately from production code
- No performance impact expected

### Code Organization
- Module system is zero-cost abstraction
- No performance impact expected

---

## Conclusion

These examples provide concrete implementations for all recommended best practices. Follow the migration checklist to implement changes incrementally and safely.

