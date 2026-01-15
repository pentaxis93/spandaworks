---
name: beads-workflow
description: Dependency-aware task tracking for agentic work. Use when managing multi-phase projects, tracking emergent work, or computing ready work queues.
---

# Beads Workflow Skill

This skill guides agents in using Beads (`bd`) for dependency-aware task tracking in aiandi projects.

## Overview

Beads is a git-native, offline-first issue tracker designed for AI agents. It provides:
- **Typed dependencies** with semantic meaning (blocks, related, parent-child, discovered-from)
- **Deterministic ready-work detection** via `bd ready`
- **Hash-based IDs** to prevent collisions across branches/agents
- **3-way merge** for multi-agent coordination via git

## When to Use Beads vs GitHub Issues

**Beads is PRIMARY** for all work tracking in aiandi.

**Use Beads for:**
- All project tasks and work items
- Dependency tracking (blocks, related, parent-child, discovered-from)
- Session-local task management
- Computing ready work queues (`bd ready`)
- Multi-phase projects with blocking relationships
- Agent-discovered work during execution

**Use GitHub Issues ONLY for:**
- External contributor interface (bug reports from outside)
- Cross-repo coordination (referencing other repositories)
- Items requiring GitHub PR auto-close workflow (`Closes #X`)

**No dual-tracking.** An item lives in ONE place. Single source of truth.

---

## Writing Good Tasks

**Tasks must be self-contained.** Anyone picking up a task should be able to execute without chasing external references.

### Task Description Template

```markdown
## Context

Why does this task exist? What problem does it solve?
Brief background for someone unfamiliar with the history.

## Scope

What specifically needs to be done?
- Concrete deliverable 1
- Concrete deliverable 2

## Out of Scope (optional)

What is explicitly NOT part of this task?
Prevents scope creep and clarifies boundaries.

## Acceptance Criteria

- [ ] Checkable criterion 1
- [ ] Checkable criterion 2
- [ ] How do we know this is done?
```

### Example: Good Task

```
bd create "Integrate Beads into LBRP South Quarter" -p 1 --description "## Context

The LBRP skill's South Quarter (Tasks) currently doesn't integrate with Beads.
Sessions should load ready work from bd ready instead of creating tasks ad-hoc.

## Scope

Update LBRP skill Phase 2b (South Quarter) to:
1. Query bd ready --json for available work
2. Present ready tasks to inform session goal selection
3. Allow claiming tasks via bd update --status in_progress

## Out of Scope

- Modifying other quarters (handled separately)
- Creating tasks during LBRP (that's during-session work)

## Acceptance Criteria

- [ ] South Quarter queries bd ready
- [ ] Ready tasks displayed during ceremony
- [ ] Agent can claim a task as session goal
- [ ] Works when no tasks exist (graceful empty state)"
```

### Example: Bad Task

```
bd create "Fix the login bug"
```

Why it's bad:
- No context (which login? what bug?)
- No acceptance criteria (how do we know it's fixed?)
- No scope (what's included/excluded?)
- Cannot be executed without asking questions

### Creating Tasks with Long Descriptions

For descriptions longer than a command line allows:

```bash
# Use --body-file to read from a file
bd create "Task title" -p 1 --body-file task-spec.md

# Or use stdin
cat << 'EOF' | bd create "Task title" -p 1 --body-file -
## Context
...
EOF
```

### Updating Sparse Tasks

If you encounter a sparse task, **enrich it before starting work**:

```bash
bd update aiandi-xxxx --description "## Context
..."
```

This is not overhead—it's essential for trust in the system.

---

## Essential Commands

### Query Ready Work

```bash
# Check what's unblocked (human-readable)
bd ready

# Machine-readable for agents
bd ready --json

# Filter by assignee
bd ready --assignee agent-name --json
```

**Pattern:** Start every session with `bd ready` to see what work is unblocked.

### Create Tasks

```bash
# Simple task
bd create "Task description"

# Task with priority
bd create "Critical bug" -p 1

# Child task (hierarchical)
bd create "Subtask" --deps parent-child:bd-a1b2

# Agent discovery (emergent work)
bd create "Found edge case" --deps discovered-from:bd-c3d4
```

**Pattern:** Use `discovered-from` when you find work while executing another task.

### Manage Dependencies

```bash
# Add blocking dependency (A must finish before B can start)
bd dep add bd-a111 blocks bd-b222

# Add related dependency (connected but not blocking)
bd dep add bd-c333 related bd-d444

# View all dependencies for a task
bd show bd-1234
```

**Pattern:** Use `blocks` to model sequential work; use `related` to track connections without blocking.

### Claim and Update Status

**Claiming a task (MANDATORY format):**
```bash
# Always use BOTH --status AND --assignee when claiming
bd update bd-1234 --status in_progress --assignee <your-identity>
```

**Identity for assignee:**
- Governance sessions: `governance`
- Standard sessions: `agent` or your git user.name
- Execution agents: agent type (e.g., `builder`, `reviewer`)

**Status transitions:**
```bash
# Complete a task
bd update bd-1234 --status closed

# Mark as blocked/waiting
bd update bd-1234 --status blocked
```

**Valid statuses:** `open`, `in_progress`, `blocked`, `closed`

**Pattern:** Always claim with both status AND assignee so multi-agent coordination knows who owns what.

### Sync with Git

```bash
# 3-way merge (local, base, remote)
bd sync
```

**Pattern:** `bd sync` is integrated into the `/close` command. Don't call manually unless needed.

---

## Dependency Types

Beads supports four typed dependencies with different semantics:

### 1. `blocks` - Sequential Dependency

**Semantics:** Task A must complete before B can start.

```bash
bd dep add bd-a111 blocks bd-b222
```

**Behavior:**
- Blocked tasks won't appear in `bd ready` until blocker is done
- Transitive: If A blocks B and B blocks C, then A blocks C
- Use for: Sequential workflows, prerequisite work

**Example:**
```bash
bd create "Design API schema" -p 1        # bd-a1b2
bd create "Implement endpoints" -p 1      # bd-c3d4
bd dep add bd-a1b2 blocks bd-c3d4         # Design must finish first
bd ready                                  # Only shows bd-a1b2
```

### 2. `related` - Associative Dependency

**Semantics:** Tasks are connected but not blocking.

```bash
bd dep add bd-c333 related bd-d444
```

**Behavior:**
- No impact on `bd ready` computation
- Documents connections for context
- Use for: Related features, cross-cutting concerns, FYI links

**Example:**
```bash
bd create "Add login UI" -p 1             # bd-e5f6
bd create "Add logout UI" -p 1            # bd-g7h8
bd dep add bd-e5f6 related bd-g7h8        # Related but independent
bd ready                                  # Shows both (neither blocked)
```

### 3. `parent-child` - Hierarchical Dependency

**Semantics:** Epic/subtask relationship.

```bash
bd create "Subtask" --deps parent-child:bd-e555
```

**Behavior:**
- Creates hierarchical IDs: `bd-e555.1`, `bd-e555.2`, etc.
- Child IDs use parent hash + sequence number
- Up to 3 levels of nesting supported
- Use for: Epics, work breakdown structures, feature organization

**Example:**
```bash
bd create "Auth System" -t epic -p 1      # bd-a3f8e9
bd create "Login UI" --deps parent-child:bd-a3f8e9      # bd-a3f8e9.1
bd create "Validation" --deps parent-child:bd-a3f8e9    # bd-a3f8e9.2
bd create "Tests" --deps parent-child:bd-a3f8e9         # bd-a3f8e9.3
```

### 4. `discovered-from` - Agent Discovery

**Semantics:** Agent found work during execution of parent task.

```bash
bd create "Fix bug" --deps discovered-from:bd-f666
```

**Behavior:**
- Tracks emergent work for review
- Documents discovery context
- Use for: Agent-discovered issues, unexpected findings, debt discovered while working

**Example:**
```bash
# Agent working on bd-1234 (implementing feature)
# Discovers missing validation
bd create "Add input validation" --deps discovered-from:bd-1234 -p 2

# Discovers technical debt
bd create "Refactor error handling" --deps discovered-from:bd-1234 -p 3
```

---

## Session Workflow Patterns

### Session Start

```bash
# Check what's ready to work
bd ready

# Or for agents (JSON output)
bd ready --json

# Filter by assignee
bd ready --assignee my-agent-name --json
```

**Pattern:** Always start by checking ready work to understand what's unblocked.

### During Work

**Claiming work:**
```bash
# Always use BOTH status AND assignee
bd update bd-1234 --status in_progress --assignee agent-name
```

**Discovering new work:**
```bash
# Found issue while working on bd-1234
bd create "Missing validation" --deps discovered-from:bd-1234 -p 2
```

**Tracking blockers:**
```bash
# Can't proceed until dependency is resolved
bd dep add bd-upstream blocks bd-current-task
bd ready  # Current task won't appear until upstream is done
```

### Session End

**Sync is handled by `/close` command.** Don't call `bd sync` manually unless you have a specific need.

The `/close` command's West Quarter handles:
1. `git pull --rebase`
2. `bd sync`
3. `git push`
4. Verification

---

## Hash-Based IDs

Beads uses hash IDs (e.g., `bd-a1b2`) to prevent collisions when multiple agents or branches create tasks concurrently.

### Why Hash IDs?

**Problem with sequential IDs:**
- Branch A creates `bd-10`
- Branch B also creates `bd-10` (different task!)
- Merge conflict when branches merge

**Hash ID solution:**
- Each task gets a unique UUID at creation
- ID is hash of UUID (first N chars)
- Different tasks automatically get different IDs
- Clean merges across branches/agents

### Progressive Length Scaling

Hash length scales automatically as database grows:

- **4 chars** (0-500 issues): `bd-a1b2`
- **5 chars** (500-1,500 issues): `bd-f14c3`
- **6 chars** (1,500+ issues): `bd-3e7a5b`

### Hierarchical IDs

Epics use parent hash + sequence:

```bash
bd create "Auth System" -t epic -p 1      # bd-a3f8e9
bd create "Login UI" -p 1                 # bd-a3f8e9.1
bd create "Validation" -p 1               # bd-a3f8e9.2
```

**Benefits:**
- Parent hash ensures unique namespace
- Sequential child IDs are human-friendly
- Clear visual grouping in listings

---

## Agent-Specific Patterns

### Query Work by Assignee

```bash
bd ready --assignee agent-name --json
```

Use this to filter ready work to a specific agent.

### Create Tasks from Discovery

```bash
# Agent found issue while working on bd-1234
bd create "Missing validation" --deps discovered-from:bd-1234 -p 2

# Track it as a blocker if needed
bd dep add bd-5678 blocks bd-1234
```

**Pattern:** Always use `discovered-from` to document what work led to the discovery.

### Track Blockers Explicitly

```bash
# Can't proceed until dependency is resolved
bd dep add bd-upstream blocks bd-current-task

# Verify it's blocking
bd ready  # Current task won't appear until upstream is done
```

**Pattern:** Make blocking relationships explicit in the graph rather than in task descriptions.

### Multi-Agent Coordination

```bash
# Agent 1 claims work
bd update bd-1234 --status in_progress --assignee agent-1

# Agent 2 discovers blocker
bd create "Fix dependency issue" --deps discovered-from:bd-1234 -p 1
bd dep add bd-new-id blocks bd-1234

# Both agents sync via git
bd sync
git push
```

**Pattern:** Use assignee + status + dependencies to coordinate work across agents.

---

## Offline Operation

Beads works completely offline:

- **All queries** run against local SQLite database
- **No network required** for any commands
- **Sync happens via git** when you push/pull
- **Auto-import** after `git pull` (first bd command detects newer JSONL)

**How auto-import works:**
- First `bd` command after `git pull` detects that `.beads/issues.jsonl` is newer than database
- Automatically imports changes
- No daemon required

---

## Project Isolation

Each aiandi project has its own `.beads/` directory with isolated database:

```
.beads/
├── beads.db          # SQLite database (auto-synced)
├── issues.jsonl      # Git-tracked source of truth
├── config.yaml       # Project-specific config
└── metadata.json     # Project metadata
```

**Rules:**
- Tasks cannot reference tasks in other projects
- Each database is completely isolated
- If cross-project tracking needed, initialize Beads in parent directory
- **Single worktree per project** - Beads daemon mode requires this
- Work via branch switching, not multiple worktrees

---

## Configuration

### Binary Path

The bd binary is installed at: `/home/pentaxis93/.npm-global/bin/bd`

For shell sessions, ensure the PATH includes npm global bin directory, or use the full path.

### Project Prefix

aiandi uses the `aiandi` prefix for Beads tasks. This was set during `bd init --prefix aiandi`.

---

## Common Patterns

### Pattern: Start Session

```bash
# 1. Check ready work
bd ready --json

# 2. Claim a task
bd update bd-1234 --status in_progress --assignee agent-name

# 3. Begin work
```

### Pattern: Discover Work

```bash
# While working on bd-1234, discover missing validation
bd create "Add input validation" --deps discovered-from:bd-1234 -p 2

# If it blocks current work
bd dep add bd-new-id blocks bd-1234

# Current work now blocked until validation added
bd ready  # bd-1234 won't appear; bd-new-id will
```

### Pattern: Epic Breakdown

```bash
# Create epic
bd create "User Authentication" -t epic -p 1
# Returns: bd-a1b2

# Create subtasks
bd create "Login UI" --deps parent-child:bd-a1b2       # bd-a1b2.1
bd create "Session mgmt" --deps parent-child:bd-a1b2   # bd-a1b2.2
bd create "Tests" --deps parent-child:bd-a1b2          # bd-a1b2.3

# Add blocking relationships if sequential
bd dep add bd-a1b2.1 blocks bd-a1b2.2  # Login before sessions
bd dep add bd-a1b2.2 blocks bd-a1b2.3  # Sessions before tests
```

### Pattern: Multi-Phase Work

```bash
# Phase 1: Design
bd create "Design API" -p 1           # bd-aaa

# Phase 2: Implementation (blocked by design)
bd create "Implement API" -p 1        # bd-bbb
bd dep add bd-aaa blocks bd-bbb

# Phase 3: Testing (blocked by implementation)
bd create "Test API" -p 1             # bd-ccc
bd dep add bd-bbb blocks bd-ccc

# Ready work shows only bd-aaa initially
bd ready
```

---

## Troubleshooting

### PATH Issues

If `bd` command not found:

```bash
# Use full path
/home/pentaxis93/.npm-global/bin/bd ready

# Or add to PATH
export PATH="$PATH:/home/pentaxis93/.npm-global/bin"
```

### Stale Data After git pull

```bash
# Just run any bd command - auto-import triggers
bd ready

# Or explicit sync
bd sync
```

### Merge Conflicts in JSONL

When two agents modify the same issue:

1. Git will show conflict in `.beads/issues.jsonl`
2. Resolve by keeping the newer `updated_at` timestamp
3. Run `bd import -i .beads/issues.jsonl` to update database

Hash-based IDs prevent ID collisions; conflicts only happen on same-issue updates.

---

## Further Reading

- `.beads/README.md` - Quick reference in repository
- [Beads FAQ](https://github.com/steveyegge/beads/blob/main/docs/FAQ.md) - Comprehensive documentation
- [Beads Repository](https://github.com/steveyegge/beads) - Source and issue tracking

---

*Use Beads to track what emerges. Let dependencies guide the flow.*
