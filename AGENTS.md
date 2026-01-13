# aiandi Agent Guidelines

This is **aiandi** (AI and I) - infrastructure for human-AI collaboration.

## Policies

### Zero Tech Debt

Orphaned code, documentation, and infrastructure is deleted immediately.
No "we might need this later." If it's not actively used, it's removed.

### Naming

This project is "aiandi".

### Test-Driven Development (TDD)

1. **RED** — Write a failing test that defines expected behavior
2. **GREEN** — Write minimal code to make the test pass
3. **REFACTOR** — Improve code quality while keeping tests green

Run tests: `cargo test`

### Git Workflow

- All changes via pull requests. Never commit directly to `main`.
- Branch prefixes: `feature/`, `fix/`, `docs/`, `refactor/`, `chore/`
- After PR merge: switch to `main`, delete local feature branch.

---

## Session Types

### Standard Sessions

Use `/open` to begin with the LBRP ceremony. Use `/close` to seal.

Standard sessions handle:
- Feature development
- Bug fixes
- Documentation
- General coding work

### Governance Sessions

Use `/governance` to open a Governance session.

Governance is the **deliberation layer** - meta to execution:
- Architectural decisions
- System evolution
- Agent coordination via transmissions
- Canon maintenance

Governance resources live in `governance/`:
- `sessions/archive/` - Session documentation
- `sessions/inbox/` - Human→AI file transfer (governance receives)
- `sessions/outbox/` - AI→human artifacts (governance produces)
- `evolution/` - Proposals and decisions
- `canon/transmission-protocol.md` - Agent communication format

Project root also has:
- `inbox/` - Human→AI file transfer (standard sessions receive)

---

## Agent Coordination

### Execution Agents

Governance coordinates work by spawning execution agents via `mcp_task`:

```
mcp_task(
  description: "Brief description",
  prompt: "Full context and instructions",
  subagent_type: "general"
)
```

Execution agents:
- Inherit project root as working directory
- See this AGENTS.md (project guidelines)
- Use your anthropic subscription
- Have full tool access

### Transmissions

Formal agent-to-agent communication uses the transmission protocol.
See `governance/transmission-protocol.md` for the XML schema.

Transmissions are:
- Self-contained (no external context needed)
- WHAT specified, HOW belongs to recipient
- Written to `governance/sessions/outbox/`

---

## Build & Test

```bash
# Build CLI
cargo build

# Build gtd MCP server
cd packages/gtd/mcp-server && npm run build

# Build pim MCP server
cd packages/pim/mcp-server && cargo build --release
```

---

*May this work serve all beings everywhere, without exception.*
