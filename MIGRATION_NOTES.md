# Migration Notes: spandaworks → aiandi

This document tracks the internal transformation from "spandaworks" to "aiandi".

## Status: Migration Complete (2026-01-16)

The complete transformation from spandaworks to aiandi is now finished.

## Phase 1: Foundation (2026-01-14)

- [x] Repository renamed: `~/src/spandaworks` → `~/src/aiandi`
- [x] Cargo workspace created at repo root
- [x] CLI crate created: `crates/aiandi/`
- [x] Skills directory created: `skills/` with transmission and gtd
- [x] Templates created: `templates/minimal.md` and `templates/comprehensive.md`
- [x] Packaging reorganized: `aur-packages/` → `packaging/aur/`
- [x] README.md updated to explain aiandi
- [x] Key documentation updated (pim/README.md, telemetry/README.md)

## Phase 2: Complete Renaming (2026-01-16)

### Package & Binary Names
- [x] `spandaworks-cli` → `aiandi-cli` (shared/utils/rust-cli/Cargo.toml)
- [x] `spandaworks-pim` → `aiandi-pim` (packages/pim/mcp-server/Cargo.toml)
- [x] `spandaworks-mode` → `aiandi-mode` (packages/core/mcp-server/Cargo.toml)

### Directory Renames
- [x] `.opencode/plugin/spandaworks-identity` → `.opencode/plugin/aiandi-identity`
- [x] `.opencode/skill/spandaworks-docs` → `.opencode/skill/aiandi-docs`
- [x] `vault-template/_spandaworks` → `vault-template/_aiandi`

### Code & Documentation
- [x] Import statements: `use spandaworks_cli::` → `use aiandi_cli::`
- [x] Dependency references in all Cargo.toml files
- [x] Documentation sweep: 20 files updated, ~165 references changed
  - docs/naming-protocol.md (110 changes)
  - docs/getting-started.md
  - packages/*/README.md
  - governance/canon/*.md
  - All ADRs and protocols

## Intentionally Preserved

These references keep "spandaworks" naming:

### External MCP Tool Names
- `spandaworks_gtd_*` functions in GTD skills documentation
- **Reason:** External GTD MCP server package, not part of aiandi repository

### Historical Records
- Git commit history (preserved as-is)
- Archived transmissions in governance/sessions/outbox/
- Historical session documentation
- "Formerly known as spandaworks" references in README

### Separate Repositories
- Blog posts (separate repo with independent naming)

---

**Migration complete.** All internal references to "spandaworks" have been replaced with "aiandi". The project is now consistently named throughout.
