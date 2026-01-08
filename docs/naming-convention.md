# Spandaworks Naming Protocol
**Version:** 1.0  
**Established:** January 8, 2026  
**Status:** Canonical  

---

## Overview

This document defines the complete naming convention for the Spandaworks ecosystem, covering repositories, packages, directories, imports, URLs, and all other contexts where naming consistency matters.

**Core principle:** Use scoped packages (`@spandaworks/[name]`) for namespace protection and industry-standard structure.

---

## The Complete Naming Scheme

| Context | Format | Example | Notes |
|---------|--------|---------|-------|
| **Repository** | lowercase | `spandaworks` | GitHub repo name |
| **Monorepo root** | lowercase | `/spandaworks/` | Root directory |
| **Package directories** | lowercase | `/packages/gtd/` | Simple, short paths |
| **Package names** | @scope/name | `@spandaworks/gtd` | npm package identifier |
| **Import statements** | @scope/name | `import { } from '@spandaworks/gtd'` | TypeScript/JavaScript |
| **Domain** | lowercase | `spandaworks.dev` | Primary domain |
| **Documentation URLs** | lowercase paths | `docs.spandaworks.dev/packages/gtd` | No @ in URLs |
| **CLI commands** | hyphenated | `spandaworks gtd add "task"` | Command-line interface |
| **English text** | Spandaworks | "Spandaworks is a collaboration engine" | Proper noun, one word |

---

## Why Scoped Packages?

### The `@spandaworks/` Namespace

**Format:** `@spandaworks/[package-name]`

**Benefits:**

1. **Namespace ownership**: Only you can publish `@spandaworks/*` packages on npm
2. **Cleaner than prefixes**: `@spandaworks/gtd` > `spandaworks-gtd`
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

**Spandaworks:**
- `@spandaworks/telemetry`
- `@spandaworks/gtd`
- `@spandaworks/pim`
- `@spandaworks/cli`

---

## Directory Structure

### Monorepo Layout

```
spandaworks/
├── packages/
│   ├── telemetry/          # Directory: simple lowercase
│   │   ├── src/
│   │   ├── tests/
│   │   └── package.json    # name: "@spandaworks/telemetry"
│   ├── gtd/
│   │   ├── src/
│   │   ├── tests/
│   │   └── package.json    # name: "@spandaworks/gtd"
│   ├── pim/
│   │   ├── src/
│   │   └── package.json    # name: "@spandaworks/pim"
│   ├── cli/
│   │   ├── src/
│   │   └── package.json    # name: "@spandaworks/cli"
│   └── core/
│       ├── src/
│       └── package.json    # name: "@spandaworks/core"
├── apps/
│   ├── web/                # Web application
│   │   └── package.json    # name: "@spandaworks/web" or private
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
- **Package names:** Import with clarity (`@spandaworks/gtd`)
- **Keeps paths short:** Not `packages/spandaworks-gtd/`

---

## Package Configuration

### Individual Package (package.json)

```json
{
  "name": "@spandaworks/gtd",
  "version": "1.0.0",
  "description": "GTD task management for Spandaworks",
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
    "@spandaworks/telemetry": "workspace:*"
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

### Root Workspace (package.json)

```json
{
  "name": "spandaworks",
  "version": "1.0.0",
  "private": true,
  "description": "Spandaworks monorepo",
  "scripts": {
    "build": "pnpm --filter './packages/*' build",
    "test": "pnpm --filter './packages/*' test",
    "lint": "pnpm --filter './packages/*' lint"
  },
  "devDependencies": {
    "typescript": "^5.3.0",
    "eslint": "^8.55.0",
    "prettier": "^3.1.0"
  }
}
```

### Workspace Configuration (pnpm-workspace.yaml)

```yaml
packages:
  - 'packages/*'
  - 'apps/*'
  - 'tools/*'
```

---

## Import Statements

### In TypeScript/JavaScript

```typescript
// ✅ CORRECT: Scoped imports
import { Task, TaskList } from '@spandaworks/gtd';
import { track, event } from '@spandaworks/telemetry';
import { Contact, Organization } from '@spandaworks/pim';
import { CLI } from '@spandaworks/cli';

// ❌ INCORRECT: Unscoped
import { Task } from 'gtd';  // Ambiguous

// ❌ INCORRECT: Prefixed
import { Task } from 'spandaworks-gtd';  // Old pattern

// ❌ INCORRECT: Wrong scope
import { Task } from '@spanda/gtd';  // Wrong namespace
```

### Cross-Package Dependencies

```typescript
// In @spandaworks/gtd
import { track } from '@spandaworks/telemetry';

export function addTask(task: Task) {
  track('task.created', { id: task.id });
  // ... implementation
}
```

### Re-exports (Barrel Pattern)

```typescript
// packages/core/src/index.ts
export * from '@spandaworks/gtd';
export * from '@spandaworks/telemetry';
export * from '@spandaworks/pim';

// Allows consumers to:
import { Task, track, Contact } from '@spandaworks/core';
```

---

## TypeScript Configuration

### Root tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "resolveJsonModule": true,
    "baseUrl": ".",
    "paths": {
      "@spandaworks/*": ["packages/*/src"]
    }
  },
  "include": ["packages/*/src"],
  "exclude": ["node_modules", "dist", "build"]
}
```

**Path mapping benefits:**
- TypeScript resolves `@spandaworks/gtd` to `packages/gtd/src`
- No need to build packages during development
- Instant feedback in IDE

### Package-Specific tsconfig.json

```json
{
  "extends": "../../tsconfig.json",
  "compilerOptions": {
    "outDir": "./dist",
    "rootDir": "./src"
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "tests"]
}
```

---

## Package Manager Commands

### pnpm (Recommended for Monorepos)

```bash
# Install dependencies for all packages
pnpm install

# Add a dependency to a specific package
pnpm --filter @spandaworks/gtd add lodash

# Add workspace dependency
pnpm --filter @spandaworks/gtd add @spandaworks/telemetry

# Run script in a specific package
pnpm --filter @spandaworks/gtd test
pnpm --filter @spandaworks/gtd build

# Run script in all packages
pnpm --filter './packages/*' build
pnpm --filter './packages/*' test

# Run script in multiple specific packages
pnpm --filter @spandaworks/gtd --filter @spandaworks/pim test
```

### npm Workspaces

```bash
# Install dependencies
npm install

# Run script in workspace
npm run test --workspace=@spandaworks/gtd

# Run in all workspaces
npm run build --workspaces

# Add dependency to workspace
npm install lodash --workspace=@spandaworks/gtd
```

### Yarn Workspaces

```bash
# Install dependencies
yarn install

# Run script in workspace
yarn workspace @spandaworks/gtd test

# Run in all workspaces
yarn workspaces run build

# Add dependency to workspace
yarn workspace @spandaworks/gtd add lodash
```

---

## Publishing to npm

### First-Time Setup

```bash
# Login to npm
npm login

# Create organization (one-time)
npm org create spandaworks

# Invite collaborators (optional)
npm org add spandaworks <username>
```

### Publishing Individual Packages

```bash
# Navigate to package
cd packages/gtd

# Build the package
pnpm build

# Publish (public)
npm publish --access public

# OR publish (private, requires paid npm account)
npm publish
```

### Publishing from Monorepo Root

```bash
# Build all packages
pnpm --filter './packages/*' build

# Publish all (with changesets or lerna)
pnpm changeset publish
```

### Version Management

```bash
# Using changesets (recommended)
pnpm changeset add      # Add changelog entry
pnpm changeset version  # Bump versions
pnpm changeset publish  # Publish to npm

# Manual versioning
cd packages/gtd
npm version patch  # 1.0.0 -> 1.0.1
npm version minor  # 1.0.0 -> 1.1.0
npm version major  # 1.0.0 -> 2.0.0
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

**Note:** URLs use simple paths, not `@scope/` syntax.

### README Links

```markdown
# @spandaworks/gtd

📚 [Documentation](https://docs.spandaworks.dev/packages/gtd)  
🐛 [Issues](https://github.com/pentaxis93/spandaworks/issues)  
💬 [Discussions](https://github.com/pentaxis93/spandaworks/discussions)  

## Installation

\`\`\`bash
pnpm add @spandaworks/gtd
\`\`\`

## Usage

\`\`\`typescript
import { Task } from '@spandaworks/gtd';

const task = new Task('Write documentation');
\`\`\`
```

---

## CLI Tool Naming

### CLI Binary Name

```json
// In @spandaworks/cli package.json
{
  "name": "@spandaworks/cli",
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
# Global install
npm install -g @spandaworks/cli

# Run without install (npx)
npx @spandaworks/cli gtd list

# Run from monorepo
pnpm --filter @spandaworks/cli start gtd list
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

## Migration Guide

### From Current State to Final State

If you currently have:
- `spanda-telemetry`
- `spanda-gtd`
- `spanda-pim`

**Step 1: Update package.json files**

```bash
# In each package directory
cd packages/telemetry
```

```json
// Change from:
{
  "name": "spanda-telemetry"
}

// To:
{
  "name": "@spandaworks/telemetry"
}
```

**Step 2: Update all imports**

```bash
# Find and replace across codebase
# Find:    from 'spanda-gtd'
# Replace: from '@spandaworks/gtd'

# Find:    from 'spanda-telemetry'
# Replace: from '@spandaworks/telemetry'
```

**Step 3: Update package dependencies**

```json
// In any package.json that depends on workspace packages
{
  "dependencies": {
    // Change from:
    "spanda-gtd": "workspace:*",
    
    // To:
    "@spandaworks/gtd": "workspace:*"
  }
}
```

**Step 4: Update TypeScript paths**

```json
// tsconfig.json
{
  "compilerOptions": {
    "paths": {
      // Change from:
      "spanda-*": ["packages/*/src"],
      
      // To:
      "@spandaworks/*": ["packages/*/src"]
    }
  }
}
```

**Step 5: Update documentation**

```bash
# Find and replace in all .md files
# Find:    `spanda-gtd`
# Replace: `@spandaworks/gtd`
```

**Step 6: Test everything**

```bash
# Clean install
rm -rf node_modules
pnpm install

# Build all packages
pnpm --filter './packages/*' build

# Run tests
pnpm --filter './packages/*' test

# Try imports in a test file
```

---

## Quick Reference

### Package Naming Patterns

| Pattern | Example | Use Case |
|---------|---------|----------|
| **Core library** | `@spandaworks/core` | Main API surface |
| **Feature package** | `@spandaworks/gtd` | GTD functionality |
| **Utility package** | `@spandaworks/telemetry` | Observability |
| **CLI tool** | `@spandaworks/cli` | Command-line interface |
| **Integration** | `@spandaworks/obsidian` | Third-party integration |
| **Types** | `@spandaworks/types` | Shared TypeScript types |

### Common Commands

```bash
# Install all dependencies
pnpm install

# Build a package
pnpm --filter @spandaworks/gtd build

# Test a package
pnpm --filter @spandaworks/gtd test

# Build all packages
pnpm --filter './packages/*' build

# Add dependency to package
pnpm --filter @spandaworks/gtd add lodash

# Add workspace dependency
pnpm --filter @spandaworks/gtd add @spandaworks/telemetry

# Publish package
cd packages/gtd && npm publish --access public
```

### File Paths Quick Reference

```
Repository:     spandaworks
Directory:      packages/gtd/
Package name:   @spandaworks/gtd
Import:         from '@spandaworks/gtd'
URL:            docs.spandaworks.dev/packages/gtd
CLI:            spandaworks gtd add "task"
```

---

## Naming Checklist

When adding a new package:

- [ ] Directory name is lowercase: `packages/[name]/`
- [ ] package.json name is scoped: `"name": "@spandaworks/[name]"`
- [ ] Imports use scope: `from '@spandaworks/[name]'`
- [ ] Dependencies reference scope: `"@spandaworks/[name]": "workspace:*"`
- [ ] TypeScript paths include scope: `"@spandaworks/*": ["packages/*/src"]`
- [ ] README uses scoped name: `# @spandaworks/[name]`
- [ ] Documentation URL is simple: `docs.spandaworks.dev/packages/[name]`
- [ ] publishConfig sets access: `"access": "public"`

---

## Why This Matters

**Consistency prevents:**
- Import confusion across packages
- Namespace collisions on npm
- Broken references after refactoring
- Documentation drift
- Unclear package ownership

**This protocol ensures:**
- Professional, standard structure
- Protected namespace (`@spandaworks`)
- Clear package boundaries
- Easy navigation (simple directory names)
- Safe publishing (scoped = private by default)
- Future scalability

---

*This is the canonical naming protocol for Spandaworks. All code, documentation, and infrastructure should follow these conventions.*
