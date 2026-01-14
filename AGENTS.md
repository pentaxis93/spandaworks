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

## Task Tracking with Beads

aiandi uses **Beads** (`bd`) for dependency-aware task tracking. Beads is designed for AI agents: git-native, offline-first, with typed dependencies and deterministic ready-work detection.

### When to Use Beads vs GitHub Issues

**Use Beads for:**
- Agent-discovered work during execution
- Dependency tracking (blocks, related, parent-child, discovered-from)
- Session-local task management
- Computing ready work queues (`bd ready`)
- Multi-phase projects with blocking relationships

**Use GitHub Issues for:**
- Human-facing feature requests
- Public bug reports
- Cross-repo coordination
- Long-term roadmap items

**Integration:** Beads tasks can reference GitHub issues and vice versa. Use what fits the workflow.

### Essential Commands

```bash
# Check ready work (unblocked tasks)
bd ready                    # Human-readable list
bd ready --json            # Machine-readable (agents)

# Create tasks
bd create "Task description"                           # Simple task
bd create "Subtask" --deps parent-child:bd-a1b2       # Child task
bd create "Found issue" --deps discovered-from:bd-c3d4 # Agent discovery

# Manage dependencies
bd dep add bd-1234 blocks bd-5678      # Task 1234 blocks 5678
bd dep add bd-1234 related bd-9abc     # Tasks are related
bd show bd-1234                        # See all dependencies

# Update status
bd update bd-1234 --status in_progress
bd update bd-1234 --status done

# Sync with git
bd sync                     # 3-way merge with remote
```

### Dependency Types

Beads supports four typed dependencies with different semantics:

1. **blocks** - Task A must complete before B can start
   - `bd dep add bd-a111 blocks bd-b222`
   - Blocked tasks won't appear in `bd ready`

2. **related** - Tasks are connected but not blocking
   - `bd dep add bd-c333 related bd-d444`
   - No impact on ready work computation

3. **parent-child** - Hierarchical relationship (epics)
   - `bd create "Subtask" --deps parent-child:bd-e555`
   - Creates hierarchical IDs (bd-e555.1, bd-e555.2)

4. **discovered-from** - Agent found work during execution
   - `bd create "Fix bug" --deps discovered-from:bd-f666`
   - Tracks emergent work for review

### Session Workflow

**Session Start:**
```bash
bd ready              # Check what's unblocked
bd ready --json       # For programmatic consumption
```

**During Work:**
```bash
# Claim a task
bd update bd-1234 --status in_progress --assignee agent-name

# Discover new work
bd create "Found edge case" --deps discovered-from:bd-1234

# Track dependencies
bd dep add bd-5678 blocks bd-1234  # 5678 must finish first
```

**Session End:**
```bash
bd sync               # 3-way merge (local, base, remote)
git add .beads/
git commit -m "Update task tracking"
```

Note: `bd sync` is included in the "Landing the Plane" workflow (line 127).

### Hash-Based IDs

Beads uses hash IDs (e.g., `bd-a1b2`) to prevent collisions when multiple agents or branches create tasks concurrently. The hash length scales automatically:

- 4 chars (0-500 issues): `bd-a1b2`
- 5 chars (500-1,500 issues): `bd-f14c3`
- 6 chars (1,500+ issues): `bd-3e7a5b`

**Hierarchical IDs** for epics use parent hash + sequence:
```bash
bd create "Auth System" -t epic -p 1  # bd-a3f8e9
bd create "Login UI" -p 1             # bd-a3f8e9.1
bd create "Validation" -p 1           # bd-a3f8e9.2
```

### Agent-Specific Patterns

**Query ready work by assignee:**
```bash
bd ready --assignee agent-name --json
```

**Create tasks from agent discovery:**
```bash
# Agent found issue while working on bd-1234
bd create "Missing validation" --deps discovered-from:bd-1234 -p 2
```

**Track blockers explicitly:**
```bash
# Can't proceed until dependency is resolved
bd dep add bd-upstream blocks bd-current-task
bd ready  # Current task won't appear until upstream is done
```

### Offline Operation

Beads works completely offline:
- All queries run against local SQLite database
- No network required for any commands
- Sync happens via git when you push/pull
- Auto-import after `git pull` (first bd command detects newer JSONL)

### Project Isolation

Each aiandi project has its own `.beads/` directory with isolated database:
```
.beads/
├── beads.db          # SQLite database (auto-synced)
├── issues.jsonl      # Git-tracked source of truth
├── config.yaml       # Project-specific config
└── metadata.json     # Project metadata
```

Tasks cannot reference tasks in other projects. If cross-project tracking is needed, initialize Beads in a parent directory.

### Configuration

The bd binary is installed at: `/home/pentaxis93/.npm-global/bin/bd`

For shell sessions, ensure the PATH includes npm global bin directory, or use the full path.

### Further Reading

- `.beads/README.md` - Quick reference in repository
- [Beads FAQ](https://github.com/steveyegge/beads/blob/main/docs/FAQ.md) - Comprehensive documentation
- [Beads Repository](https://github.com/steveyegge/beads) - Source and issue tracking

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

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
