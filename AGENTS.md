# aiandi Agent Guidelines

## Policies

### Zero Tech Debt

Orphaned code, documentation, and infrastructure is deleted immediately.
No "we might need this later." If it's not actively used, it's removed.
This prevents accumulation of confusion and maintenance burden.

### Inbox/Outbox Protocol

Agent-to-agent communication uses inbox/outbox directories:
- `inbox/` — Incoming transmissions
- `inbox/processed/` — Handled transmissions (archived after processing)
- `outbox/` — Outgoing transmissions (responses, reports)
- `outbox/processed/` — Sent transmissions (archived after sending)

There is no separate "outputs" directory.

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

When responding to governance transmissions:
1. Read the instruction from inbox/ (or as provided)
2. Execute tasks as specified
3. Write report transmission to outbox/
4. Format per the `transmission` skill

See `skills/transmission/SKILL.md` for the full protocol.
