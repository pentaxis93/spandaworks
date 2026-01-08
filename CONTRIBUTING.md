# Contributing to Spandaworks

Thank you for your interest in contributing to Spandaworks!

## Repository Structure

Spandaworks is a monorepo with five packages:

- **[core](packages/core/)** - Identity and ceremony
- **[telemetry](packages/telemetry/)** - Knowledge graph and observation system
- **[gtd](packages/gtd/)** - TaskWarrior MCP server
- **[pim](packages/pim/)** - Email, calendar, contacts MCP server
- **[second-brain](packages/second-brain/)** - Obsidian vault infrastructure

Each package has its own README with package-specific development guidelines.

## General Guidelines

### Reporting Issues

1. Check existing issues first
2. Use the appropriate repository (monorepo or package-specific if standalone)
3. Include:
   - Clear description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, versions, etc.)

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature` or `fix/my-fix`
3. Make your changes
4. Test thoroughly (see package-specific testing)
5. Commit with clear conventional commit messages
6. Push and open a PR against `main`

### Branch Naming

- `feature/*` - New features
- `fix/*` - Bug fixes
- `docs/*` - Documentation improvements
- `refactor/*` - Code refactoring
- `chore/*` - Maintenance tasks

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add session inheritance to telemetry
fix: resolve MCP server stdio buffering issue
docs: update gtd installation instructions
refactor: extract CLI utilities to shared library
chore: update dependencies
```

### Code Style

Each package follows its language ecosystem conventions:

- **TypeScript** (gtd): ESLint with typescript-eslint
- **Rust** (pim): rustfmt + clippy
- **Python** (telemetry): ruff (formatting + linting)
- **Markdown** (core, second-brain): Consistent heading hierarchy

### Architecture Principles

When contributing, preserve these architectural constraints:

1. **No cross-package imports** - Packages communicate via MCP or events
2. **Language choice serves domain** - Don't rewrite packages for uniformity
3. **Preserve epistemic humility** - Track what's verifiable, avoid claims
4. **Recursive discipline** - Fix both object (the bug) and meta (why it happened)

## Package-Specific Guidelines

### Core
- Changes to identity documents (Tantric Sutras) require discussion
- Ceremony changes should preserve the four-quarter structure
- Skills and plugins must be symlink-compatible

### Telemetry
- Changes to ontology require schema migration
- All entity/relationship additions need embedding support
- Tests required for MCP tools and graph operations

### GTD
- Preserve the trust model (AI suggests, human approves)
- TaskWarrior 3.x compatibility required
- MCP tool additions need documentation and examples

### PIM
- Safety-first (especially email sending)
- CLI tools must remain wrappable (no direct library integrations)
- Credentials via `pass` only (never in code)

### Second Brain
- Vault structure changes need protocol updates
- Skills require Opus-class or specify model requirements
- Preserve PARA + GTD mapping

## Development Setup

See individual package READMEs for specific setup instructions.

### Quick Start

```bash
# Clone repository
git clone https://github.com/pentaxis93/spandaworks.git
cd spandaworks

# Install packages (choose what you need)
cd packages/telemetry && pip install -e ".[dev]"
cd packages/gtd/mcp-server && npm install
cd packages/pim/mcp-server && cargo build
```

## Testing

Each package has its own testing approach:

```bash
# Telemetry (Python)
cd packages/telemetry && pytest -v

# GTD (TypeScript)
cd packages/gtd/mcp-server && npm test && npm run typecheck

# PIM (Rust)
cd packages/pim/mcp-server && cargo test
```

## Questions?

Open an issue with the `question` label or reach out via the repository discussions.

---

**May your contributions benefit all beings everywhere, without exception.**

ॐ मणि पद्मे हूं
