# aiandi Naming Protocol
**Version:** 1.1  
**Last Updated:** January 8, 2026  
**Status:** Canonical  

---

## Overview

This document defines the complete naming convention for the aiandi ecosystem, covering repositories, packages, directories, imports, URLs, and all other contexts where naming consistency matters.

**Core principle:** Use scoped packages (`@aiandi/[name]`) for namespace protection and industry-standard structure.

---

## The Complete Naming Scheme

| Context | Format | Example | Notes |
|---------|--------|---------|-------|
| **Repository** | lowercase | `aiandi` | GitHub repo name |
| **Monorepo root** | lowercase | `/aiandi/` | Root directory |
| **Package directories** | lowercase | `/packages/gtd/` | Simple, short paths |
| **Package names** | @scope/name | `@aiandi/gtd` | npm package identifier |
| **Import statements** | @scope/name | `import { } from '@aiandi/gtd'` | TypeScript/JavaScript |
| **Domain** | lowercase | `aiandi.dev` | Primary domain |
| **Documentation URLs** | lowercase paths | `docs.aiandi.dev/packages/gtd` | No @ in URLs |
| **CLI commands** | hyphenated | `aiandi gtd add "task"` | Command-line interface |
| **English text** | aiandi | "aiandi is a collaboration engine" | Proper noun, one word |

---

## Why Scoped Packages?

### The `@aiandi/` Namespace

**Format:** `@aiandi/[package-name]`

**Benefits:**

1. **Namespace ownership**: Only you can publish `@aiandi/*` packages on npm
2. **Cleaner than prefixes**: `@aiandi/gtd` > `aiandi-gtd`
3. **Industry standard**: `@react/router`, `@babel/core`, `@anthropic/sdk`
4. **Private by default**: Scoped packages are private unless explicitly made public
5. **Grouped in listings**: All your packages appear together in npm/pnpm
6. **Avoids collisions**: Bare names like `telemetry` or `gtd` are likely taken

### How Modern Ecosystems Use Scopes

**React:**
- `@react/router`
- `@react/native`
- `@react/test-renderer`

**Babel:**
- `@babel/core`
- `@babel/cli`
- `@babel/preset-env`

**Vercel:**
- `@vercel/analytics`
- `@vercel/edge`
- `@vercel/postgres`

**aiandi:**
- `@aiandi/telemetry`
- `@aiandi/gtd`
- `@aiandi/pim`
- `@aiandi/cli`

---

## Multi-Language Package Naming

aiandi is a polyglot monorepo with packages in **Node.js/TypeScript**, **Python**, and **Rust**. Each language ecosystem has different conventions for package naming and namespacing.

### Language-Specific Patterns

| Language | Package Format | Example | Import Format | Registry |
|----------|---------------|---------|---------------|----------|
| **Node.js/TypeScript** | `@aiandi/[name]` | `@aiandi/gtd` | `from '@aiandi/gtd'` | npm |
| **Python** | `aiandi-[name]` | `aiandi-telemetry` | `import aiandi_telemetry` | PyPI |
| **Rust** | `aiandi-[name]` | `aiandi-pim` | `use aiandi_pim::` | crates.io |

### Current Package Assignments

```
packages/
├── gtd/              # Node.js/TypeScript → @aiandi/gtd
├── telemetry/        # Python → aiandi-telemetry  
├── pim/              # Rust → aiandi-pim
└── cli/              # Rust → aiandi-cli (provides 'aiandi' binary)
```

### Why Different Formats?

**Node.js (npm):**
- ✅ **Native scope support**: `@aiandi/gtd`
- ✅ **Best practice**: Scopes provide namespace protection
- ✅ **Private by default**: Scoped packages require `--access public` to publish
- ✅ **Grouped listings**: All `@aiandi/*` packages appear together in npm

**Python (PyPI):**
- ❌ **No scope support**: PyPI doesn't have npm-style organizational scopes
- ✅ **Hyphenated package**: `aiandi-telemetry` (hyphens in package name on PyPI)
- ✅ **Underscored module**: `import aiandi_telemetry` (underscores in Python code)
- ⚠️ **Namespace via convention**: Project prefix acts as namespace, but not enforced by PyPI

**Rust (crates.io):**
- ❌ **No scope support**: Similar to PyPI, flat global namespace
- ✅ **Hyphenated crate**: `aiandi-pim` in Cargo.toml
- ✅ **Underscored module**: `use aiandi_pim::` in Rust code
- ⚠️ **Namespace via convention**: Crate name prefix, but not enforced by crates.io

---

## Directory Structure

### Monorepo Layout

```
aiandi/
├── packages/
│   ├── telemetry/          # Directory: simple lowercase
│   │   ├── src/
│   │   ├── tests/
│   │   └── package.json    # name: "@aiandi/telemetry"
│   ├── gtd/
│   │   ├── src/
│   │   ├── tests/
│   │   └── package.json    # name: "@aiandi/gtd"
│   ├── pim/
│   │   ├── src/
│   │   └── package.json    # name: "@aiandi/pim"
│   ├── cli/
│   │   ├── src/
│   │   └── package.json    # name: "@aiandi/cli"
│   └── core/
│       ├── src/
│       └── package.json    # name: "@aiandi/core"
├── apps/
│   ├── web/                # Web application
│   │   └── package.json    # name: "@aiandi/web" or private
│   └── mobile/             # Mobile application
│       └── package.json
├── tools/
│   └── scripts/            # Build/deployment scripts
├── docs/
│   └── packages/           # Package documentation
├── pnpm-workspace.yaml     # pnpm workspace config
├── package.json            # Root package.json
├── tsconfig.json           # Root TypeScript config
└── README.md
```

**Key principle:** Directory names are simple (for humans), package names are scoped (for package managers).

**Why separate them?**
- **Directories:** Navigate filesystem easily (`packages/gtd/`)
- **Package names:** Import with clarity (`@aiandi/gtd`)
- **Keeps paths short:** Not `packages/aiandi-gtd/`

---

## Package Configuration

### Node.js Package (package.json)

```json
{
  "name": "@aiandi/gtd",
  "version": "1.0.0",
  "description": "GTD task management for aiandi",
  "main": "./dist/index.js",
  "module": "./dist/index.mjs",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.mjs",
      "require": "./dist/index.js"
    }
  },
  "scripts": {
    "build": "tsup src/index.ts --format cjs,esm --dts",
    "test": "vitest",
    "lint": "eslint src/"
  },
  "dependencies": {
    "@aiandi/telemetry": "workspace:*"
  },
  "devDependencies": {
    "typescript": "^5.3.0",
    "tsup": "^8.0.0",
    "vitest": "^1.0.0"
  },
  "publishConfig": {
    "access": "public"
  }
}
```

### Python Package (pyproject.toml)

```toml
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "aiandi-telemetry"
version = "1.0.0"
description = "Telemetry and observability for aiandi"
readme = "README.md"
requires-python = ">=3.10"
license = {text = "MIT"}
authors = [
    {name = "aiandi Contributors"}
]
keywords = ["telemetry", "observability", "aiandi"]
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
]
dependencies = [
    "pydantic>=2.0.0",
    "httpx>=0.25.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.4.0",
    "pytest-cov>=4.1.0",
    "mypy>=1.5.0",
    "ruff>=0.1.0",
]

[project.urls]
Homepage = "https://aiandi.dev"
Documentation = "https://docs.aiandi.dev/packages/telemetry"
Repository = "https://github.com/pentaxis93/aiandi"
Issues = "https://github.com/pentaxis93/aiandi/issues"

[tool.hatch.build.targets.wheel]
packages = ["src/aiandi_telemetry"]
```

### Rust Package (Cargo.toml)

```toml
[package]
name = "aiandi-pim"
version = "1.0.0"
edition = "2021"
rust-version = "1.75"
authors = ["aiandi Contributors"]
description = "Personal information management for aiandi"
readme = "README.md"
homepage = "https://aiandi.dev"
repository = "https://github.com/pentaxis93/aiandi"
license = "MIT"
keywords = ["pim", "contacts", "aiandi"]
categories = ["database"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
chrono = "0.4"

[dev-dependencies]
tokio = { version = "1.35", features = ["full"] }
criterion = "0.5"

[[bench]]
name = "benchmark"
harness = false

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

### Rust CLI Binary (Cargo.toml)

```toml
[package]
name = "aiandi-cli"
version = "1.0.0"
edition = "2021"
rust-version = "1.75"
authors = ["aiandi Contributors"]
description = "Command-line interface for aiandi"
readme = "README.md"
homepage = "https://aiandi.dev"
repository = "https://github.com/pentaxis93/aiandi"
license = "MIT"
keywords = ["cli", "aiandi"]
categories = ["command-line-utilities"]

[[bin]]
name = "aiandi"  # Binary name (what users run)
path = "src/main.rs"

[dependencies]
clap = { version = "4.4", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
anyhow = "1.0"
aiandi-pim = { path = "../pim" }  # Workspace dependency

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
```

**Key points for CLI:**
- `[[bin]]` section defines the binary
- `name = "aiandi"` creates the `aiandi` executable
- Users run: `aiandi gtd add "task"`
- Not: `aiandi-cli gtd add "task"` ❌

---

## Import Statements

### TypeScript/JavaScript

```typescript
// ✅ CORRECT: Scoped imports
import { Task, TaskList } from '@aiandi/gtd';
import { track, event } from '@aiandi/telemetry';
import { Contact, Organization } from '@aiandi/pim';
import { CLI } from '@aiandi/cli';

// ❌ INCORRECT: Unscoped
import { Task } from 'gtd';  // Ambiguous

// ❌ INCORRECT: Prefixed
import { Task } from 'aiandi-gtd';  // Old pattern

// ❌ INCORRECT: Wrong scope
import { Task } from '@spanda/gtd';  // Wrong namespace
```

### Python

```python
# ✅ CORRECT: Import with underscores
import aiandi_telemetry
from aiandi_telemetry import track, event
from aiandi_telemetry.tracker import Tracker

# ❌ INCORRECT: Hyphens don't work in Python imports
import aiandi-telemetry  # SyntaxError!

# Usage
from aiandi_telemetry import track

track("user.login", {"user_id": 123})
```

### Rust

```rust
// ✅ CORRECT: Import with underscores
use aiandi_pim::{Contact, Organization};
use aiandi_pim::error::PimError;

// In main.rs or lib.rs
use aiandi_pim::Contact;

fn main() {
    let contact = Contact::new("Jane Doe");
    println!("{:?}", contact);
}

// Workspace dependencies (in other Cargo.toml files)
[dependencies]
aiandi-pim = { path = "../pim" }  # Hyphenated in Cargo.toml
// Then in code: use aiandi_pim::  # Underscored in Rust
```

---

## Package Manager Commands

### pnpm (Node.js - Recommended for Monorepos)

```bash
# Install dependencies for all packages
pnpm install

# Add a dependency to a specific package
pnpm --filter @aiandi/gtd add lodash

# Add workspace dependency
pnpm --filter @aiandi/gtd add @aiandi/telemetry

# Run script in a specific package
pnpm --filter @aiandi/gtd test
pnpm --filter @aiandi/gtd build

# Run script in all packages
pnpm --filter './packages/*' build
pnpm --filter './packages/*' test

# Run script in multiple specific packages
pnpm --filter @aiandi/gtd --filter @aiandi/pim test
```

### pip/uv (Python)

```bash
# Install in development mode
pip install -e packages/telemetry

# Install from PyPI
pip install aiandi-telemetry

# Run tests
cd packages/telemetry
pytest

# Using uv (modern alternative)
uv sync
uv run pytest
```

### cargo (Rust)

```bash
# Build specific package
cargo build -p aiandi-pim

# Test specific package
cargo test -p aiandi-pim

# Run CLI binary
cargo run -p aiandi-cli -- --help

# Build all workspace packages
cargo build --workspace
```

---

## Publishing

### npm (Node.js)

```bash
# First-time setup
npm login

# Create organization (one-time)
npm org create spandaworks

# Navigate to package
cd packages/gtd

# Build the package
pnpm build

# Publish (public)
npm publish --access public
```

### PyPI (Python)

```bash
# Build the package
cd packages/telemetry
python -m build

# Publish to PyPI (requires account and API token)
python -m twine upload dist/*

# Or publish to Test PyPI first
python -m twine upload --repository testpypi dist/*
```

### crates.io (Rust)

```bash
# Login to crates.io (one-time)
cargo login

# Publish from package directory
cd packages/pim
cargo publish

# Or publish dry-run first
cargo publish --dry-run
```

---

## Repository & Git

### Repository Configuration

```
Repository name: spandaworks
URL: https://github.com/pentaxis93/spandaworks
Clone: git clone https://github.com/pentaxis93/spandaworks.git
```

**NOT** `@spandaworks/spandaworks` - the `@scope/` syntax is only for npm.

### Branch Naming

```bash
# Feature branches
git checkout -b feature/gtd-recurring-tasks
git checkout -b feature/telemetry-dashboard

# Fix branches
git checkout -b fix/gtd-timezone-bug
git checkout -b fix/pim-export-crash

# Package-specific work
git checkout -b gtd/add-priorities
git checkout -b telemetry/prometheus-exporter
```

### Commit Messages

```bash
# Conventional commits with scope
git commit -m "feat(gtd): add recurring task support"
git commit -m "fix(telemetry): correct event timestamp handling"
git commit -m "docs(pim): update contact API examples"
git commit -m "chore(build): update TypeScript to 5.3"

# Multi-package changes
git commit -m "feat(gtd,pim): integrate contact linking in tasks"
```

---

## Documentation & URLs

### Documentation Site Structure

```
docs.spandaworks.dev/
├── /                          # Home
├── /getting-started/          # Quick start
├── /packages/                 # Package listing
│   ├── /packages/gtd/         # GTD docs
│   ├── /packages/telemetry/   # Telemetry docs
│   ├── /packages/pim/         # PIM docs
│   └── /packages/cli/         # CLI docs
├── /api/                      # API reference
│   ├── /api/gtd/              # GTD API
│   └── /api/telemetry/        # Telemetry API
└── /guides/                   # Tutorials
```

**Note:** URLs use simple paths, not `@scope/` syntax. URLs are always lowercase and language-agnostic.

---

## CLI Tool Naming

### CLI Binary Name

```json
// In @spandaworks/cli or spandaworks-cli package
{
  "bin": {
    "spandaworks": "./dist/cli.js"
  }
}
```

### CLI Command Structure

```bash
# Main command
spandaworks

# Subcommands by package
spandaworks gtd add "Write naming protocol"
spandaworks gtd list --project work
spandaworks gtd complete 123

spandaworks telemetry dashboard
spandaworks telemetry export --format json

spandaworks pim contact add "Jane Doe"
spandaworks pim contact search "acme"

# Global options
spandaworks --version
spandaworks --help
spandaworks gtd --help
```

### Installation

```bash
# Global install (Node.js)
npm install -g @spandaworks/cli

# Global install (Rust)
cargo install spandaworks-cli

# Run without install (npx)
npx @spandaworks/cli gtd list

# Run from monorepo
pnpm --filter @spandaworks/cli start gtd list
cargo run -p spandaworks-cli -- gtd list
```

---

## Domain & Hosting

### Primary Domain

```
spandaworks.dev
```

### Subdomain Strategy

```
www.spandaworks.dev          # Marketing site
app.spandaworks.dev          # Web application
docs.spandaworks.dev         # Documentation
api.spandaworks.dev          # API endpoints
cdn.spandaworks.dev          # Static assets
```

### API Endpoints

```
https://api.spandaworks.dev/v1/gtd/tasks
https://api.spandaworks.dev/v1/telemetry/events
https://api.spandaworks.dev/v1/pim/contacts
```

---

## System Paths & Configuration

### Local Data Directories

All Spandaworks system data is stored under a single root:

```
~/.spandaworks/
├── telemetry/              # Telemetry database and logs
│   ├── kuzu/               # Knowledge graph database
│   ├── events.jsonl        # Event stream
│   └── embeddings/         # Cached embeddings
├── gtd-telemetry/          # GTD operation logs
├── cache/                  # Shared cache directory
│   └── embeddings/         # Model cache
└── config/                 # System configuration
```

**Pattern:** `~/.spandaworks/[subsystem]/`

### Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `SPANDAWORKS_DB_PATH` | Override database location | `~/.spandaworks/telemetry/kuzu` |
| `SPANDAWORKS_SESSION_ID` | Current session identifier | `2026-01-08-naming-001` |
| `SPANDAWORKS_TELEMETRY_PATH` | Event log location | `~/.spandaworks/telemetry/events.jsonl` |
| `SPANDAWORKS_EVOLUTION_DIR` | Evolution proposals directory | `~/vault/_spandaworks/evolution/proposals/` |

**Pattern:** `SPANDAWORKS_[COMPONENT]_[PURPOSE]`

### MCP Server Names

MCP server identifiers in OpenCode configuration:

```json
{
  "mcp": {
    "spandaworks_gtd": {
      "type": "local",
      "command": ["node", "/path/to/packages/gtd/mcp-server/dist/index.js"]
    },
    "spandaworks_pim": {
      "type": "local",
      "command": ["/path/to/packages/pim/mcp-server/target/release/spandaworks-pim"]
    },
    "spandaworks_telemetry": {
      "type": "local",
      "command": ["python", "-m", "spandaworks_telemetry.mcp"]
    }
  }
}
```

**Pattern:** `spandaworks_[package]` (underscored, not hyphenated)

**Rationale:** MCP server identifiers use underscores (not hyphens) to align with JSON key naming conventions and to avoid ambiguity in configuration contexts.

---

## Telemetry Attribute Namespaces

### OpenTelemetry Custom Attributes

Spandaworks uses the `spandaworks.*` namespace for all custom OpenTelemetry attributes:

```json
{
  "spandaworks.session.id": "2026-01-08-naming-001",
  "spandaworks.session.goal": "Complete naming migration",
  "spandaworks.session.persona": "assistant",
  "spandaworks.tool.name": "read",
  "spandaworks.tool.success": true,
  "spandaworks.state.from": "exploring",
  "spandaworks.state.to": "executing"
}
```

**Pattern:** `spandaworks.[domain].[attribute]`

### Attribute Domains

| Domain | Purpose | Examples |
|--------|---------|----------|
| `spandaworks.session.*` | Session metadata | `session.id`, `session.goal`, `session.persona` |
| `spandaworks.tool.*` | Tool call tracking | `tool.name`, `tool.success`, `tool.duration_ms` |
| `spandaworks.state.*` | State transitions | `state.from`, `state.to`, `state.trigger` |
| `spandaworks.context.*` | Contextual metrics | `context.pressure`, `context.token_count` |
| `spandaworks.insight.*` | Knowledge capture | `insight.id`, `insight.domain`, `insight.confidence` |
| `spandaworks.friction.*` | Friction logging | `friction.id`, `friction.category`, `friction.recurrence` |
| `spandaworks.pattern.*` | Pattern detection | `pattern.id`, `pattern.name`, `pattern.occurrence_count` |
| `spandaworks.goal.*` | Goal tracking | `goal.id`, `goal.scope`, `goal.status` |

**Why not shorten to `spanda.*`?**
- Consistency: All package identifiers use "spandaworks"
- Clarity: No ambiguity about what system generated the telemetry
- Future-proofing: If "spanda" becomes a generic term, "spandaworks" remains specific

---

## Event Schemas & JSON References

### Schema URLs

Event schemas are identified by canonical URLs:

```json
{
  "$schema": "https://spandaworks.dev/schemas/event.json",
  "$id": "https://spandaworks.dev/schemas/session-event.json"
}
```

**Available schemas:**
- `https://spandaworks.dev/schemas/event.json` - Base event schema
- `https://spandaworks.dev/schemas/session-event.json` - Session lifecycle events
- `https://spandaworks.dev/schemas/telemetry-event.json` - Telemetry events
- `https://spandaworks.dev/schemas/gtd-event.json` - GTD task events
- `https://spandaworks.dev/schemas/knowledge-event.json` - Knowledge capture events

**Pattern:** `https://spandaworks.dev/schemas/[domain]-event.json`

**Note:** These URLs are canonical identifiers. Actual schema files live in `shared/events/` in the monorepo.

### Event Storage

Events are stored in JSONL format:

```
~/.spandaworks/telemetry/events.jsonl
~/.spandaworks/gtd-telemetry/YYYY-MM-DD.jsonl
```

Each line is a complete JSON event document conforming to a schema.

---

## Quick Reference

### Package Naming Summary

| Language | Package Name | Module/Import | Registry |
|----------|-------------|---------------|----------|
| **Node.js/TS** | `@spandaworks/gtd` | `from '@spandaworks/gtd'` | npm |
| **Python** | `spandaworks-telemetry` | `import spandaworks_telemetry` | PyPI |
| **Rust** | `spandaworks-pim` | `use spandaworks_pim::` | crates.io |
| **Rust CLI** | `spandaworks-cli` (package) | `spandaworks` (binary) | crates.io |

### Common Commands

```bash
# Install all dependencies
pnpm install                                    # Node.js
pip install -e packages/telemetry               # Python
cargo build --workspace                         # Rust

# Build a package
pnpm --filter @spandaworks/gtd build            # Node.js
python -m build                                  # Python (in package dir)
cargo build -p spandaworks-pim                  # Rust

# Test a package
pnpm --filter @spandaworks/gtd test             # Node.js
pytest                                          # Python (in package dir)
cargo test -p spandaworks-pim                   # Rust

# Publish package
npm publish --access public                      # Node.js (in package dir)
python -m twine upload dist/*                    # Python (in package dir)
cargo publish                                    # Rust (in package dir)
```

### File Paths Quick Reference

```
Repository:     spandaworks
Directory:      packages/gtd/
Package name:   @spandaworks/gtd (Node.js) or spandaworks-gtd (Python/Rust)
Import:         from '@spandaworks/gtd' (Node.js)
URL:            docs.spandaworks.dev/packages/gtd
CLI:            spandaworks gtd add "task"
```

---

## Naming Checklist

When adding a new package:

### Node.js/TypeScript Package
- [ ] Directory name is lowercase: `packages/[name]/`
- [ ] package.json name is scoped: `"name": "@spandaworks/[name]"`
- [ ] Imports use scope: `from '@spandaworks/[name]'`
- [ ] Dependencies reference scope: `"@spandaworks/[name]": "workspace:*"`
- [ ] TypeScript paths include scope: `"@spandaworks/*": ["packages/*/src"]`
- [ ] README uses scoped name: `# @spandaworks/[name]`
- [ ] publishConfig sets access: `"access": "public"`

### Python Package
- [ ] Directory name is lowercase: `packages/[name]/`
- [ ] pyproject.toml name is hyphenated: `"spandaworks-[name]"`
- [ ] Module directory is underscored: `spandaworks_[name]/`
- [ ] Imports use underscores: `import spandaworks_[name]`
- [ ] `py.typed` marker file present for type hints
- [ ] `__init__.py` exports public API
- [ ] README uses hyphenated name: `# spandaworks-[name]`

### Rust Package
- [ ] Directory name is lowercase: `packages/[name]/`
- [ ] Cargo.toml name is hyphenated: `"spandaworks-[name]"`
- [ ] Imports use underscores: `use spandaworks_[name]::`
- [ ] For CLI: `[[bin]]` name is `"spandaworks"` (no suffix)
- [ ] `README.md` uses crates.io package name
- [ ] `lib.rs` or `main.rs` exists as entry point

### All Packages
- [ ] Documentation URL is simple: `docs.spandaworks.dev/packages/[name]`
- [ ] Repository field points to: `https://github.com/pentaxis93/spandaworks`
- [ ] MCP server name (if applicable): `spandaworks_[name]`
- [ ] System paths use: `~/.spandaworks/[name]/`
- [ ] Telemetry attributes use: `spandaworks.[name].*`
- [ ] Event schemas reference: `https://spandaworks.dev/schemas/[name]-event.json`
- [ ] Environment variables use: `SPANDAWORKS_[NAME]_[PURPOSE]`

---

## Why This Matters

**Consistency prevents:**
- Import confusion across packages
- Namespace collisions on package registries
- Broken references after refactoring
- Documentation drift
- Unclear package ownership

**This protocol ensures:**
- Professional, standard structure
- Protected namespace (where supported)
- Clear package boundaries
- Easy navigation (simple directory names)
- Language-appropriate conventions
- Future scalability

---

*This is the canonical naming protocol for Spandaworks. All code, documentation, and infrastructure must follow these conventions.*
