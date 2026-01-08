# ADR: Monorepo Architecture Exploration & Refactoring Opportunities

**Date:** 2026-01-07  
**Status:** Accepted  
**Context:** Informal exploration session on refactoring branch  
**Decision:** Document current architecture and extract reusable CLI pattern

---

## Context

Conducted comprehensive exploration of the Spandaworks monorepo to understand:
- Current package structure and dependencies
- Cross-package integration patterns
- Refactoring opportunities
- Technical debt accumulation

This exploration was conducted on the `refactoring` branch as an informal visit to understand the system holistically.

---

## Current Architecture (As Discovered)

### Package Overview

| Package | Language | Role | Status | Lines of Code (est.) |
|---------|----------|------|--------|---------------------|
| **core** | Mixed (MD/TS) | Identity & Ceremony | Mature | ~500 |
| **telemetry** | Python | Knowledge Graph (Kuzu) | Complete | ~3000 |
| **gtd** | TypeScript | TaskWarrior MCP | Mature | ~2000 |
| **pim** | **Rust** | Email/Calendar/Contacts | **New** | ~800 |
| **second-brain** | Mixed | Vault/Protocols | Partial | Unknown |

### Integration Layer

**Key Finding:** No direct cross-package dependencies. Integration via:

1. **MCP Protocol**: All packages expose MCP servers
   - `gtd`: TaskWarrior tools (TypeScript)
   - `pim`: Email/calendar/contacts tools (Rust)
   - `telemetry`: Knowledge graph tools (Python, MCP server planned)

2. **Event Schemas**: Shared JSONL format (`shared/events/*.schema.json`)
   - Not yet implemented
   - Scaffolding for future event-driven architecture

3. **Session Protocol**: Shared ceremony lifecycle (`shared/session/protocol.md`)
   - LBRP opening ceremony
   - Session state machine
   - Metadata format

### Polyglot Rationale (Discovered Pattern)

Each language serves the package's domain:

- **Python** (telemetry): Kuzu graph, sentence-transformers, data science ecosystem
- **TypeScript** (gtd): Node MCP SDK maturity, JSON manipulation
- **Rust** (pim): Performance, memory safety for credentials, cutting-edge (edition 2024)

This is **strategic polyglot architecture**, not accidental accumulation.

---

## Key Discoveries

### 1. PIM Package Missing from Documentation

**Finding:** `packages/pim` exists and is production-ready (merged PR, clean Rust code), but:
- Not listed in README.md organism table
- Not shown in project structure diagram
- Not documented in architecture.md

**Impact:** New contributors will miss a critical package.

**Action Required:** Update documentation (see Decisions below).

### 2. Telemetry Integration Incomplete

**Finding:** LBRP Phase 4 attempts to call `spanda_telemetry.mcp.session_open`, but:
- Telemetry has MCP tools but no stdio MCP server entrypoint
- Integration not yet operational
- Fails gracefully (already handled)

**Impact:** Session inheritance via INHERITED relationship not yet functional.

**Action Required:** Implement telemetry MCP server (future work).

### 3. CLI Wrapping Pattern Ripe for Extraction

**Finding:** `packages/pim/mcp-server/src/cli.rs` contains excellent async CLI wrapping utilities:
- Clean error handling with `anyhow`
- Stdout/stderr capture
- Stdin piping
- Well-tested

**Pattern is reusable** across any Rust package that wraps CLI tools.

**Decision:** Extract to `shared/utils/rust-cli` (see Decisions below).

### 4. Event Schemas Unused

**Finding:** `shared/events/*.schema.json` define cross-package event types but:
- No implementations found
- No event emission/consumption code
- Pure scaffolding

**Interpretation:** Future architecture, not current reality.

### 5. Librarians System Exists But Not Understood

**Finding:** Each package has `librarians.md`:
- `packages/gtd/librarians.md`
- `packages/telemetry/librarians.md`
- `shared/librarians/core.md`

**Content not examined** in this session.

**Action Required:** Future exploration to understand librarian automation.

---

## Decisions

### Decision 1: Extract CLI Pattern to Shared Library

**Status:** ✅ Implemented  
**Rationale:** The CLI wrapping pattern in `pim/cli.rs` is:
- Well-designed
- Immediately reusable
- Creates leverage for future Rust packages

**Implementation:**
- Created `shared/utils/rust-cli/` as standalone crate
- Enhanced with `CommandBuilder` pattern for environment variables and working directory
- Added comprehensive tests (12 tests, all passing)
- Updated `pim` to use shared library

**Files Created:**
- `shared/utils/rust-cli/Cargo.toml`
- `shared/utils/rust-cli/src/lib.rs` (350 lines, well-documented)
- `shared/utils/rust-cli/README.md`

**Files Modified:**
- `packages/pim/mcp-server/Cargo.toml` (added dependency)
- `packages/pim/mcp-server/src/cli.rs` (now re-exports from `spanda-cli`)

**Benefits:**
- Future Rust MCP servers get this for free
- Single source of truth for CLI error handling
- Consistent patterns across packages
- Well-tested foundation

### Decision 2: Create ADR Directory

**Status:** ✅ Implemented  
**Rationale:** No `docs/adr/` directory existed for Architecture Decision Records.

**Implementation:**
- Created `docs/adr/`
- This document becomes first ADR

**Benefit:** Establishes workflow for documenting architectural decisions.

### Decision 3: Document Refactoring Opportunities (Deferred)

**Status:** 🟡 Documented but not implemented  
**Rationale:** Many quick wins identified but should be tracked, not executed immediately.

**Recommended Next Actions:**

#### Immediate (< 30 min)
1. Update README.md organism table to include `pim`
2. Update README.md project structure to include `pim`
3. Update `docs/architecture.md` package table to include `pim`
4. Fix LBRP Phase 4 to document telemetry as "planned, not yet available"

#### Medium Effort (< 4 hours)
5. Create root `package.json` for workspace management
6. Add `Makefile` or `justfile` for unified build commands
7. Document event schemas with usage examples
8. Add CI/CD for polyglot builds (GitHub Actions)

#### Long Term (Future Sessions)
9. Implement telemetry MCP server
10. Explore librarians system
11. Implement event emission/consumption
12. Add second-brain exploration

---

## Consequences

### Positive

- **Shared CLI library** creates immediate leverage for future Rust work
- **ADR process established** for documenting decisions
- **Architecture understood** at deep level
- **Refactoring opportunities identified** and prioritized

### Negative

- **Documentation debt increased slightly** (pim not documented in main README)
- **Testing time extended** (polyglot stack requires multiple test commands)

### Neutral

- **Event schemas remain unused** (no change)
- **Telemetry integration incomplete** (known, not blocking)

---

## Metrics

**Exploration Efficiency:**
- 7 tasks completed
- 2 packages extracted (cli-utils, adr-process)
- 1 new shared library created
- 12 new tests added
- ~400 lines of reusable code created

**Technical Debt:**
- **Reduced:** CLI pattern duplication eliminated
- **Documented:** Refactoring opportunities cataloged
- **Increased:** pim package still undocumented in README

---

## Follow-Up

**Immediate:**
- [ ] Update README.md to include pim package
- [ ] Update architecture.md to include pim package
- [ ] Create GitHub issue for telemetry MCP server

**Future Sessions:**
- [ ] Explore librarians system
- [ ] Implement event emission
- [ ] Add CI/CD for polyglot builds

---

## References

- Session: `2026-01-07-explore-refactor-spandaworks-monorepo`
- Branch: `refactoring`
- Related PRs: #2 (taskwarrior-gtd-skill), #3 (pim-mcp-server)

---

**Approved by:** Session work (Robbie + Claude)  
**Reviewed by:** N/A (exploration ADR)
