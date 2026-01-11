# aiandi Agent Guidelines

## Policies

### Zero Tech Debt

Orphaned code, documentation, and infrastructure is deleted immediately.
No "we might need this later." If it's not actively used, it's removed.
This prevents accumulation of confusion and maintenance burden.

### Agent Invocation via OpenCode SDK

With OpenCode as our agent harness, agents are invoked **programmatically via the SDK**, not via filesystem polling.

**How it works:**
1. **Governance writes transmission** → `governance/sessions/outbox/Transmission_X.xml`
2. **Governance invokes execution agent** → OpenCode SDK creates session, injects transmission as context
3. **Execution agent processes** → Works in fresh OpenCode session with full transmission context
4. **Human observes execution** → `/sessions` command lists all sessions; can switch to view progress

**Human observability:**
- All SDK-created sessions appear in session list (`/sessions` or `ctrl+x l`)
- Human can switch to any session to observe progress in real-time
- Sessions can be shared (`/share`) for browser viewing
- Sessions can be exported (`/export`) to Markdown

**No inbox directories needed.** The OpenCode SDK handles agent invocation and context injection programmatically.

### Naming: aiandi

This project is "aiandi" (AI and I). The name "spandaworks" is deprecated.
All new code, documentation, and references should use "aiandi".

### Test-Driven Development (TDD)

All development follows TDD discipline:

1. **RED** — Write a failing test that defines expected behavior
2. **GREEN** — Write minimal code to make the test pass
3. **REFACTOR** — Improve code quality while keeping tests green

No production code without a failing test first. Tests are not afterthoughts;
they are the specification. The test suite is the executable documentation
of what the system does.

Run tests frequently: `cargo test`

## Build & Test

```bash
# Build CLI
cargo build

# Build gtd MCP server
cd packages/gtd/mcp-server && npm run build

# Build pim MCP server
cd packages/pim/mcp-server && cargo build --release
```

## Git Workflow

- All changes via pull requests. Never commit directly to `main`.
- Branch prefixes: `feature/`, `fix/`, `docs/`, `refactor/`, `chore/`
- After PR merge: switch to `main`, delete local feature branch.

## Transmissions

When Governance invokes an execution agent via SDK:
1. Transmission is injected as context via SDK
2. Execute tasks as specified in transmission
3. Write report transmission to `governance/sessions/outbox/` (if needed)
4. Commit work to git

See `skills/transmission/SKILL.md` for the full protocol.

**Note:** Execution agents are invoked programmatically, not by polling directories.
