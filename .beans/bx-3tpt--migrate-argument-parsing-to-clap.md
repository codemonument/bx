---
# bx-3tpt
title: Migrate argument parsing to clap
status: completed
type: task
priority: high
created_at: 2026-01-22T09:25:56Z
updated_at: 2026-01-22T11:46:30Z
---

Replace manual `env::args()` parsing with `clap` derive macros for type-safe argument handling.

## Why
- Current manual parsing is error-prone and hard to maintain
- No automatic help generation
- No argument validation
- 50% less code with clap
- Industry standard (95%+ of Rust CLI tools use clap)

## Checklist
- [x] Add `clap = { version = "4", features = ["derive"] }` to Cargo.toml
- [x] Create src/cli/mod.rs module
- [x] Create src/cli/args.rs with Cli struct using `#[derive(Parser)]`
- [x] Define Commands enum with subcommands (Init, Cache, Help, Run)
- [x] Update src/bin/main.rs to use `Cli::parse()`
- [x] Remove manual argument parsing code from main.rs
- [x] Update src/lib.rs to export cli module
- [x] Ensure backward compatibility with existing flags (-v, -i, -c, -d, -h)
- [x] Test all commands work as before
- [x] Verify auto-generated --help output is correct

## Cli Structure
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bx", version, about = "Simple, cross-platform command aliases")]
pub struct Cli {
    #[arg(short = 'd', long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "-v", alias = "--version")]
    Version,
    #[command(name = "-i", alias = "--init")]
    Init {
        #[arg(short, long)]
        template: Option<String>,
    },
    #[command(name = "-c", alias = "--cache")]
    Cache,
    #[command(name = "-h", alias = "--help")]  
    Help,
    #[command(external_subcommand)]
    Run(Vec<String>),
}
```

## Notes
- Keep backward compatibility with current flag-style arguments (-v, -i, etc.)
- Consider if we want to also support subcommand-style (bx init, bx cache) in addition