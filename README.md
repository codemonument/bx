<h1 align="center">bx</h1>

<p align="center">
  <a href="https://crates.io/crates/bx-cli"><img src="https://img.shields.io/crates/v/bx-cli.svg" alt="Crates.io"></a>
  <a href="https://github.com/codemonument/bx/actions/workflows/ci.yml"><img src="https://github.com/codemonument/bx/actions/workflows/ci.yml/badge.svg" alt="Test"></a>
  <a href="https://github.com/codemonument/bx/actions/workflows/cd.yml"><img src="https://github.com/codemonument/bx/actions/workflows/cd.yml/badge.svg" alt="Build and Release"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
</p>

> **Simple, cross-platform, and fast command aliases with superpowers.**

A fork of [Bonnie](https://github.com/arctic-hen7/bonnie) with a shorter, snappier command name.

[Releases][releases] | [Bonnie Documentation][docs]

---

## Why "bx"?

- **Short** - Only 2 characters to type (vs 6 for "bonnie")
- **Unique** - Fairly unused command name in the Linux/macOS world
- **Meaningful** - Can stand for "**b**onnie e**x**ecute" or "**B**enny e**x**ecute"

---

## Features

bx is a command aliasing tool that lets you define short aliases for long commands you run repeatedly. Here's what it supports:

- Simple **key-value aliasing**
- Inserting **custom arguments** into commands
- **Interpolating environment variables**
- Adding **all arguments in a single place**
- Using **different commands on different operating systems**
- Specifying **custom shells** for individual commands
- Specifying **default shells** for different OSes on a per-file basis
- **Infinitely nestable subcommands**
- Subcommands executed in a certain **order based on exit codes**
- **Caching** large config files after parsing for performance
- Initializing new config files from **templates**
- **Global templates** in `~/.bx/template.toml`
- **Debug mode**

If you have commands you routinely run in a project, bx is for you.

---

## Quick Start

### 1. Create a config file

Create `bx.toml` (or `bonnie.toml`) in your project root:

```toml
version = "0.3.2"

[scripts]
build = "cargo build"
test = "cargo test"
dev = "cargo run -- %%"
greet = "echo 'Hello, %name!'"
```

### 2. Run your scripts

```bash
bx build              # Runs: cargo build
bx test               # Runs: cargo test
bx dev --release      # Runs: cargo run -- --release
bx greet Perseus      # Runs: echo 'Hello, Perseus!'
```

---

## Installation

### From Cargo (Recommended)

```bash
cargo install bx-cli
```

### From Releases

Download pre-built binaries from the [releases page][releases] for:
- **Linux** (amd64)
- **macOS** (amd64, arm64)
- **Windows** (amd64)
- **musl** (Alpine Linux)

After downloading:
```bash
chmod +x bx
sudo mv bx /usr/local/bin/
```

### In Docker

```dockerfile
RUN curl -L https://github.com/codemonument/bx/releases/download/v0.4.0/bx-linux-amd64 \
    -o /usr/local/bin/bx && chmod +x /usr/local/bin/bx
```

For Alpine Linux, use `bx-musl-amd64` instead.

---

## Configuration

bx looks for configuration files in this order:

1. `BX_CONF` environment variable
2. `bx.toml` in current directory
3. `BONNIE_CONF` environment variable  
4. `bonnie.toml` in current directory

---

## Syntax Guide

### Basic Commands

```toml
version = "0.3.2"

[scripts]
build = "cargo build"
test = "cargo test"
clean = "cargo clean"
```

### Arguments

Use `%%` to pass all remaining arguments, or `%name` for named arguments:

```toml
[scripts]
# Pass all arguments
run = "cargo run -- %%"

# Named argument (required)
greet = "echo 'Hello, %name!'"

# Named argument with default
greet.cmd = "echo 'Hello, %name!'"
greet.args = ["name=World"]
```

```bash
bx run --release       # cargo run -- --release
bx greet Alice         # echo 'Hello, Alice!'
bx greet               # echo 'Hello, World!'  (uses default)
```

### Environment Variables

Interpolate environment variables with `%varname`:

```toml
[scripts]
deploy = "rsync -avz ./dist/ %USER@%HOST:/var/www/"

[scripts.deploy.env]
HOST = "example.com"
```

You can also load from `.env` files:

```toml
[env]
files = [".env", ".env.local"]
```

### Multistage Commands

Run multiple commands in sequence:

```toml
[scripts]
ci = [
    "cargo fmt -- --check",
    "cargo clippy",
    "cargo test"
]
```

### Subcommands

Nest commands for better organization:

```toml
[scripts.db]
db.subcommands.migrate = "diesel migration run"
db.subcommands.reset = "diesel database reset"
db.subcommands.seed = "cargo run --bin seed"
```

```bash
bx db migrate
bx db reset
bx db seed
```

### OS-Specific Commands

Run different commands on different operating systems:

```toml
[scripts.open]
cmd.generic = "xdg-open %%"
cmd.targets.windows = "start %%"
cmd.targets.macos = "open %%"
```

### Custom Shells

Override the default shell for specific commands:

```toml
[scripts]
# Use PowerShell on Windows
build.cmd = "Write-Host 'Building...'; cargo build"
build.shell = ["powershell", "-Command", "{}"]

# Use Node.js
calc.cmd = "console.log(2 + 2)"
calc.shell = ["node", "-e", "{}"]
```

### Changing the Default Shell

Set default shells globally or per-OS:

```toml
# Global default
default_shell = ["bash", "-c", "{}"]

# Per-OS defaults
[default_shell.targets]
windows = ["powershell", "-Command", "{}"]
macos = ["zsh", "-c", "{}"]
linux = ["bash", "-c", "{}"]
```

---

## Advanced Features

### Bones (Ordered Subcommands)

Bones is a mini-language for defining subcommands that execute based on exit codes:

```toml
[scripts]
test.cmd.generic = """
start {
    unit => cargo test --lib {
        pass => integration => cargo test --test '*'
    }
}
"""
```

This runs unit tests first, and only runs integration tests if unit tests pass.

See the [Bonnie documentation on Bones][docs-bones] for full syntax.

### Caching

For large config files, enable caching for faster startup:

```bash
bx -c    # Cache the current config
bx build # Subsequent runs use the cache
```

The cache is stored at `.bx.cache.json` (or `.bonnie.cache.json`).

### Templates

Initialize new projects from templates:

```bash
bx -t    # Initialize from default template
```

Templates are stored in `~/.bx/template.toml` (or `~/.bonnie/template.toml`).

### Debug Mode

See exactly what commands bx will execute:

```bash
bx -d build    # Shows the command without running it
```

---

## How is this different from...

### Bash Aliases?

Bash aliases can't be customized per-folder. If you want to interpolate arguments or environment variables, you need scripts. bx solves this with simple, intuitive syntax that just works, and it's cross-platform.

### Make?

GNU Make was designed to recompile files when dependencies change. Its syntax is clunky for simple command aliasing. bx uses TOML, which is human-readable and only requires configuration for features you actually use.

### package.json scripts?

npm scripts don't support comments, have poor environment variable handling, and require Node.js. bx is a single binary with no runtime dependencies.

---

## Migrating from Bonnie

bx is a drop-in replacement for Bonnie:

1. Install bx: `cargo install bx-cli`
2. Your existing `bonnie.toml` files work as-is
3. Optionally rename to `bx.toml`
4. Replace `bonnie` with `bx` in your commands

---

## License

MIT - See [LICENSE](./LICENSE)

## Credits

Based on [Bonnie](https://github.com/arctic-hen7/bonnie) by [arctic_hen7](https://github.com/arctic-hen7).

---

[docs]: https://github.com/arctic-hen7/bonnie/wiki
[docs-bones]: https://github.com/arctic-hen7/bonnie/wiki/Getting-Started-with-Bones
[releases]: https://github.com/codemonument/bx/releases
