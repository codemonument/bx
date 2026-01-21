<h1 align="center">bx</h1>

[![Crates.io](https://img.shields.io/crates/v/bx-cli.svg)](https://crates.io/crates/bx-cli)
[![Test](https://github.com/codemonument/bx/actions/workflows/ci.yml/badge.svg)](https://github.com/codemonument/bx/actions/workflows/ci.yml)
[![Build and Release](https://github.com/codemonument/bx/actions/workflows/cd.yml/badge.svg)](https://github.com/codemonument/bx/actions/workflows/cd.yml)

> **Simple, cross-platform, and fast command aliases with superpowers.**

A fork of [Bonnie](https://github.com/arctic-hen7/bonnie) with a shorter command name.

[Documentation][docs] | [Releases][releases]

## Why "bx"?

- **Short**: Only 2 characters to type (vs 6 for "bonnie")
- **Unique**: Fairly unused command name in the Linux world
- **Meaningful**: Can stand for "bonnie execute" or "Benny execute"

## Features

- Simple **key-value aliasing**
- **Custom arguments** interpolation
- **Environment variable** interpolation
- Different commands on different **operating systems**
- **Custom shells** for individual commands
- **Infinitely nestable subcommands**
- Subcommands executed based on **exit codes**
- **Caching** for large config files
- Config file **templates**
- **Debug** mode

## Quick Start

The simplest `bx.toml` (or `bonnie.toml`) configuration:

```toml
version = "0.3.2"

[scripts]
build = "echo Building"
test = "cargo test"
```

Run with:
```bash
bx build
bx test
```

## Installation

### From Cargo
```bash
cargo install bx-cli
```

### From Releases
Download pre-built binaries from the [releases page][releases] for:
- Linux (amd64)
- macOS (amd64, arm64)
- Windows (amd64)
- musl (Alpine Linux)

After downloading, move the binary to your PATH (e.g., `/usr/local/bin` on Linux/macOS) and make it executable:
```bash
chmod +x bx
sudo mv bx /usr/local/bin/
```

### In Docker
```Dockerfile
RUN curl -L https://github.com/codemonument/bx/releases/download/[VERSION]/bx-[OS]-amd64 -o /usr/local/bin/bx && chmod +x /usr/local/bin/bx
```

Replace `[VERSION]` with the version (e.g., `v0.3.4`) and `[OS]` with `linux` or `musl`.

## Configuration

bx looks for configuration in this order:
1. `BX_CONF` environment variable
2. `bx.toml` in current directory
3. `BONNIE_CONF` environment variable
4. `bonnie.toml` in current directory

## Documentation

For full documentation on all features (subcommands, environment variables, shells, etc.), see the [Bonnie documentation][docs].

## License

MIT - See [LICENSE](./LICENSE)

## Credits

Based on [Bonnie](https://github.com/arctic-hen7/bonnie) by arctic_hen7.

[docs]: https://github.com/arctic-hen7/bonnie/wiki
[releases]: https://github.com/codemonument/bx/releases
