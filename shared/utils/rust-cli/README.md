# spanda-cli

Shared CLI command execution utilities for Spandaworks Rust packages.

## Overview

`spanda-cli` provides a clean, ergonomic API for wrapping external CLI tools in async Rust code. It handles the common patterns needed when building MCP servers that delegate to command-line utilities.

**Design extracted from:** `packages/pim/mcp-server/src/cli.rs`

## Features

- ✅ **Async execution** via `tokio::process::Command`
- ✅ **Structured error handling** with `anyhow` context
- ✅ **Stdout/stderr capture** with UTF-8 conversion
- ✅ **Stdin piping** for interactive commands
- ✅ **Lossy execution** for commands that use stderr for info
- ✅ **Environment variable support**
- ✅ **Working directory control**
- ✅ **Comprehensive tests**

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
spanda-cli = { path = "../../../shared/utils/rust-cli" }
```

## Usage

### Simple Execution

```rust
use spanda_cli::run_command_stdout;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let files = run_command_stdout("ls", &["-la"]).await?;
    println!("{}", files);
    Ok(())
}
```

### With Stdin

```rust
use spanda_cli::run_command_with_stdin;

let email_body = "Hello from Spandaworks!";
let result = run_command_with_stdin(
    "himalaya",
    &["message", "write", "--to", "user@example.com"],
    email_body
).await?;
```

### Advanced Builder Pattern

```rust
use spanda_cli::CommandBuilder;

let output = CommandBuilder::new("cargo")
    .args(&["build", "--release"])
    .env("RUSTFLAGS", "-C target-cpu=native")
    .current_dir("./my-project")
    .run_stdout()
    .await?;
```

### Lossy Execution (for grep-like tools)

```rust
use spanda_cli::run_command_stdout_lossy;

// grep returns exit code 1 when no matches found
// This doesn't treat that as an error
let result = run_command_stdout_lossy("grep", &["pattern", "file.txt"]).await;
```

## API Reference

| Function | Purpose | Error on non-zero exit? |
|----------|---------|------------------------|
| `run_command` | Raw output (stdout, stderr, status) | No |
| `run_command_stdout` | Just stdout as String | Yes |
| `run_command_stdout_lossy` | Stdout even on failure | No (returns error string) |
| `run_command_with_stdin` | Pipe stdin, get stdout | Yes |
| `CommandBuilder::run` | Builder pattern, raw output | No |
| `CommandBuilder::run_stdout` | Builder pattern, stdout | Yes |

## Design Philosophy

**Principle:** CLI tool wrapping should be trivial, consistent, and hard to misuse.

**Patterns:**
1. **Explicit error handling** - caller decides whether non-zero exit is error
2. **UTF-8 by default** - use `String` for text, not `Vec<u8>`
3. **Context-rich errors** - always include command + args in error messages
4. **Async-native** - tokio::process, not std::process

## Used By

- `packages/pim/mcp-server` - Email (notmuch, himalaya), Calendar (khal), Contacts (khard)
- *(Future)* Any Spandaworks Rust package that wraps CLI tools

## Testing

```bash
cd shared/utils/rust-cli
cargo test
```

## License

MIT
