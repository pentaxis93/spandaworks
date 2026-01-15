# aiandi Agent Guidelines

This is **aiandi** (AI and I) - infrastructure for human-AI collaboration.

## Policies

### Sutra of the Third Force

**Friction is always curriculum in this project.**

When things don't work as expected:
- **Stop** trying variations and workarounds
- **Recognize** the pattern causing friction
- **Encode** the learning in skills, documentation, or code
- **Update** systems so the pattern is prevented structurally

Friction that recurs is a sign the lesson hasn't been learned. The third force (the work itself) teaches through resistance. Honor it by evolving the system.

Every correction calibrates. Every friction point is an invitation to improve the operating system itself.

### Zero Tech Debt

Orphaned code, documentation, and infrastructure is deleted immediately.
No "we might need this later." If it's not actively used, it's removed.

### Naming

This project is "aiandi".

### Skill Evolution (No Temporal Traces)

When updating skills, documentation, or canonical patterns based on learning:

**Timeless pattern encoding:**
- Describe the pattern without reference to when it was discovered
- Use present tense for principles ("First section gets no heading")
- Update examples to demonstrate the pattern
- No "added on DATE" or "learned in session X"

**Wrong (temporal residue):**
```markdown
## Update 2026-01-15
We discovered today that section headings should...
```

**Right (timeless pattern):**
```markdown
## Section Headings
First section gets no heading (article title serves).
Subsequent sections use ## headings with evocative names.
```

Skills self-improve through disciplined learning from real use. Each calibration removes temporal scaffolding and encodes only the pattern.

### Test-Driven Development (TDD)

1. **RED** — Write a failing test that defines expected behavior
2. **GREEN** — Write minimal code to make the test pass
3. **REFACTOR** — Improve code quality while keeping tests green

Run tests: `cargo test`

### Git Workflow

- Single-directory workflow with branch switching (no git worktrees)
- All changes via pull requests. Never commit directly to `main`.
- Branch prefixes: `feature/`, `fix/`, `docs/`, `refactor/`, `chore/`
- After PR merge: switch to `main`, delete local feature branch.
- **Why single-directory:** Beads daemon mode requires it

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

## Task Tracking

**Beads is PRIMARY** for all work tracking. Single source of truth.

```
Where does this task go?
│
├─ External contributor filing a bug? → GitHub Issue
├─ Cross-repo coordination needed?    → GitHub Issue  
├─ Need GitHub PR auto-close?         → GitHub Issue
│
└─ Everything else                    → Beads
```

**No dual-tracking.** A task lives in ONE place.

**Binary path**: `/home/pentaxis93/.npm-global/bin/bd`

For detailed workflows, dependency types, and patterns, invoke the `beads-workflow` skill.

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
