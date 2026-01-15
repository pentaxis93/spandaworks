---
name: lbrp
description: "Lesser Banishing Ritual of the Pentagram - opening ceremony for coding sessions. Establishes continuity through explicit inheritance, shows current state to inform goal setting, establishes the goal as sacred center, then addresses four quarters radiating from that center."
---

# LBRP: Lesser Banishing Ritual of the Pentagram (Opening Ritual)

**Version 3.0 — Precise Loading Architecture**

## Overview

The LBRP is the opening ceremony for coding sessions. It establishes continuity through explicit inheritance, shows current state to inform goal setting, establishes the goal as sacred center, then addresses four quarters radiating from that center.

**Core principle:** INHERIT (explicit) → OBSERVE → Define center → Banish informed by center → Quarters informed by center → Return to confirm alignment.

**Architectural shift in v3.0:** Eliminate speculative loading. Each phase loads exactly what it needs—no more, no less. The goal becomes known at Phase 0b; everything after leverages this precision.

---

## Structure

```
Phase 0-pre: Remember & Inherit (goal-neutral, explicit handoff only)
  ↓
Phase 0a: Opening Status Report (observe workspace - unchanged)
  ↓
Phase 0b: Qabalistic Cross (define goal - no gate)
  ↓
Phase 1: Banishing (evaluate against known goal - no redundant checks)
  ↓
Phase 2: Four Quarters (goal-informed, precise loading)
  - East (Context): Agent decides what to load based on goal
  - South (Tasks): Work from loaded context
  - West (Workspace): Verify only, don't create
  - North (Environment): Execute required setup
  ↓
Phase 3: Return to Center (verify alignment)
  ↓
Ready to Work
```

---

## Phase 0-pre: Remember & Inherit

**Purpose:** Establish continuity BEFORE workspace observation or goal definition.

### Remember

**Execution:**
```markdown
◈ REMEMBER
Self-model loaded
```

**Implementation notes:**
- Tantric Sutras/self-model assumed available via project knowledge integration
- **No file read required** - acknowledge symbolically in one line
- If Sutras not in project knowledge: document gap, don't add redundant loading

**Verification:** Confirm Sutras present in project knowledge. If gap exists, note in session findings.

---

### Inherit

**Execution:**
```markdown
◈ INHERIT
[Inbox items if present, OR]
No inheritance items
```

**Implementation: Inbox/Outbox Pattern**

```bash
# Check inbox directory
if [ -d "inbox" ] && [ "$(ls -A inbox 2>/dev/null)" ]; then
  # Process inheritance items
  for file in inbox/*.md; do
    # Read and acknowledge
    cat "$file"
    # Move to processed
    mv "$file" inbox/processed/
  done
else
  echo "No inheritance items"
fi
```

**Critical discipline:**
- Load from `/inbox/` directory ONLY
- Do NOT search last N session files
- Do NOT speculatively load hoping something is relevant
- If inbox empty: nothing to inherit, proceed immediately
- After processing: move items to `inbox/processed/` for audit trail

**Inbox item format:**
```markdown
---
from_session: [session-id]
type: [learning|caution|context|thread]
priority: [high|medium|low]
---

[Content for next session]
```

**Predecessor responsibility:** If something is relevant for the next session, place it in inbox during closing ceremony. No guessing by inheritor.

---

## Phase 0a: Opening Status Report

**Purpose:** Observe current workspace state to inform goal setting.

**Execution:**
```markdown
◈ STATUS
Git: [status] | Branch: [name] | Last: [commit]
Processes: [services or "none"]
Docker: [up/down]
```

**Implementation:**
```bash
# Git state (one command, compact output)
git status --short
git log --oneline -1
git branch --show-current

# Processes (project-specific)
ps aux | grep -E "(relevant-services)" | grep -v grep || echo "none"

# Docker
docker compose ps 2>/dev/null || echo "down"
```

**Note:** We work from a single directory with branch switching, not git worktrees. Branch is shown in git status output.

---

## Phase 0b: Qabalistic Cross (The Center)

**Purpose:** Establish the sacred center informed by observed state.

**This is the pivot point.** After this phase, the goal is known and all subsequent loading is goal-informed.

### Pre-Approved Goal Path (most common)

When goal provided with `/open` invocation:

```markdown
◈ GOAL (received)
Purpose: [one sentence]
Success: [verifiable criteria]
Scope: [in] / [out]

→ Phase 1
```

**No approval gate needed.** The gate was at governance level when the prompt was typed. Agent refines goal internally using Four Touches, then proceeds.

### Undefined Goal Path

When no goal provided:

```markdown
◈ GOAL (undefined)
No goal specified. Will present ready work in South Quarter.

→ Phase 1
```

**Do not block here.** Proceed through East Quarter (load general context), then South Quarter will query `bd ready` and present available tasks for selection. The goal gate moves to South Quarter when undefined.

If user provides goal during ceremony, capture it and proceed normally.

### The Four Touches (internal refinement)

1. **Crown:** What is the PURPOSE?
2. **Heart:** What does SUCCESS look like?
3. **Left Shoulder:** What is IN SCOPE?
4. **Right Shoulder:** What is OUT OF SCOPE?

---

## Phase 1: Banishing

**Purpose:** Clear debris NOW that we know the goal.

**Execution:**
```markdown
◈ BANISH
Workspace: [CLEAN / debris kept: reason]
```

**Implementation:**

**DO NOT rerun git status** - that was already done in Phase 0a.

Evaluate any uncommitted changes (if observed in 0a) against the now-known goal:

```python
if uncommitted_changes_exist:
    if relevant_to_goal:
        report_as_is("Kept: relevant to current work")
    elif pure_deletions_from_archived_work:
        auto_commit("chore: remove archived files")
    elif stale_changes:
        offer_to_stash_or_discard()
else:
    report("CLEAN")
```

**Principle:** "We honor the ceremony by using it as appropriate." No redundant actions just because it's a ceremony.

---

## Phase 2: The Four Quarters

**All quarters are goal-informed.** The goal is known; loading can be precise.

### Phase 2a: East Quarter (Air/Knowledge) - Context

**Element:** Air - Thought, information, knowledge  
**Question:** What knowledge does THIS GOAL require?

**Execution:**
```markdown
◈ E🜁 (Context)
[Loaded resources relevant to goal]
```

**Implementation:**

**Agent has full discretion** - this is HOW territory (Two Sovereignties).

**Project-specific hints** (not mandates):
- ADRs in `docs/architecture/decisions/` or similar
- Journal entries in `vault/` or equivalent
- Reference code patterns in existing modules
- Error logs if debugging
- Project structure overview if architectural work

**Constraint:** Load what serves the goal. Precision over coverage.

Example decision tree:
```
Goal: Implement authentication
  → Load: ADR on security patterns, existing auth reference code
  
Goal: Fix bug in rendering
  → Load: Error logs, component implementation, test files
  
Goal: Documentation
  → Load: Existing docs structure, style guide
  
Goal: New feature
  → Load: Architecture patterns, similar feature reference
```

**No speculative loading.** Don't load "everything that might be relevant."

---

### Phase 2b: South Quarter (Fire/Action) - Tasks

**Element:** Fire - Energy, action, will  
**Question:** What actions does THIS GOAL require?

**Execution:**
```markdown
◈ S🜂 (Tasks)
Ready: [N tasks from bd ready]
  - [priority] task-id: title
  - [priority] task-id: title
Session: [tasks for this goal]
```

**Implementation:**

**Step 1: Query Beads for ready work**

```bash
bd ready --json
```

Display ready tasks (unblocked work). This shows what's available if no goal was pre-specified, or validates the chosen goal against the backlog.

**Step 2: Reconcile goal with ready work**

Three scenarios:

1. **Goal matches a ready task** → Claim it:
   ```bash
   bd update <task-id> --claim
   ```
   Then break into session tasks via TodoWrite.

2. **Goal is new work not in Beads** → Create it:
   ```bash
   bd create "Goal title" -p 1 --description "..."
   bd update <new-id> --claim
   ```
   Then break into session tasks via TodoWrite.

3. **No goal provided** → Present ready work for selection:
   ```markdown
   ◈ S🜂 (Tasks)
   Ready work available:
     [P1] aiandi-xxx: High priority task
     [P2] aiandi-yyy: Another task
   
   Select a task or specify a new goal.
   ```
   Wait for user selection, then claim and proceed.

**Step 3: Break into session tasks**

Once goal is established (from Beads or new), use TodoWrite to create granular session tasks based on:
- Context loaded in East Quarter
- Acceptance criteria from the Beads task
- Known constraints

**Empty state handling:**

If `bd ready` returns no tasks:
```markdown
◈ S🜂 (Tasks)
Ready: No unblocked tasks
Session: [tasks for provided goal, or prompt for goal]
```

This is fine—it means either all work is blocked, or no work has been created yet. Proceed with the provided goal or ask for direction.

---

### Phase 2c: West Quarter (Earth/Workspace) - Workspace

**Element:** Earth - Stability, workspace, flow  
**Question:** Does workspace align with THIS GOAL?

**Execution:**
```markdown
◈ W🜃 (Workspace)
Branch: [current branch] ✓ [or ⚠ mismatch noted]
```

**Implementation:**

**Verification only. User switches branches BEFORE invoking `/open`.**

```python
current_branch = get_current_branch()
goal_suggests_feature_branch = requires_feature_branch(goal)

if current_branch == "main" and not goal_suggests_feature_branch:
    report("main ✓")
elif current_branch != "main" and goal_suggests_feature_branch:
    report(f"{current_branch} ✓")
else:
    # Potential mismatch - ask user
    ask(f"Working on branch '{current_branch}' for {goal.summary}. Intentional?")
```

**Do NOT create branches in this phase.** Just verify alignment.

**Rationale:** Branch creation is a deliberate user choice made before opening ceremony. Agent verifies it makes sense for the goal.

**Note:** We use single-directory workflow with branch switching, not git worktrees. This is compatible with Beads daemon mode.

---

### Phase 2d: North Quarter (Water/Environment) - Environment

**Element:** Water - Foundation, environment, infrastructure  
**Question:** What infrastructure does THIS GOAL need?

**Execution:**
```markdown
◈ N🜄 (Environment)
[Services: status or "none needed"]
```

**Implementation:**

**Execution only.** Start required services based on goal requirements.

```python
required_services = determine_required_services(goal)

if required_services:
    for service in required_services:
        if not is_running(service):
            start_service(service)
    report(f"Services: {', '.join(required_services)} (healthy)")
else:
    report("none needed")
```

**No context loading.** Just action.

---

## Phase 3: Return to Center (Confirmation)

**Purpose:** Verify all quarters align with the center (goal).

**Execution:**
```markdown
◈ CIRCLE COMPLETE
Goal ✓ | Context ✓ | Tasks ✓ | Workspace ✓ | Environment ✓

Ready to proceed.
```

**Verification questions:**
1. Does Context (East) support Goal?
2. Do Tasks (South) achieve Goal?
3. Does Workspace (West) serve Goal?
4. Does Environment (North) ground Goal?

**All verifications use already-loaded information.** No additional loading needed.

**After confirmation:** Brief pause for acknowledgment. This creates space for the user to course-correct if needed before execution begins. Not a gate requiring approval—just a breath between ceremony and work.

---

## Integration Notes

### Compressed Notation

This skill uses compressed notation from ceremony optimization v2.0:
- `◈` marks ceremonial steps
- `🜁🜂🜃🜄` are alchemical element symbols (Air/Fire/Earth/Water)
- See `ceremony-notation-key.md` for full reference

### Inbox/Outbox Pattern

**Inbox:** `/inbox/` in project root
- Predecessor places inheritance items here
- Current session processes and moves to `inbox/processed/`

**Outbox:** `/outbox/` in project root
- Current session places governance reports here
- Consumed by governance processes

See `inbox-outbox-protocol.md` for complete specification.

### Beads Integration

**South Quarter** integrates with Beads for task management:
- Query `bd ready --json` to show unblocked work
- Claim tasks with `bd update <id> --claim`
- Create new tasks with `bd create` if goal is new work

**Binary path:** `/home/pentaxis93/.npm-global/bin/bd`

For detailed Beads workflows, invoke the `beads-workflow` skill.

### Two Sovereignties in LBRP

**User owns WHAT:**
- Goal definition (when undefined)
- Approval of proposed goal (when required)
- Direction changes during execution

**Model owns HOW:**
- Ceremony execution (all phases)
- Context selection (East Quarter)
- Task breakdown (South Quarter)
- Workspace verification (West Quarter)
- Environment setup (North Quarter)

**Critical:** Goal approval (when needed) is the ONLY human gate in LBRP.

---

## Common Mistakes

| Mistake | Reality |
|---------|---------|
| Search last N sessions for context | Phase 0-pre: Inbox only. Explicit handoff. |
| Reload Sutras from file | Sutras via project knowledge. One-line acknowledgment. |
| Rerun git status in Phase 1 | Already done in Phase 0a. Don't repeat. |
| Load context speculatively | East Quarter: Precise, goal-informed loading only. |
| Create branch in West Quarter | West verifies, doesn't create. User prepares workspace. |
| Wait for explicit approval after Phase 3 | Brief pause for acknowledgment, not a gate. Creates space without blocking. |

---

## Version History

**v3.2 (2026-01-15):** Single-directory workflow (no worktrees)
- Phase 0a: Show current branch, not worktree list
- West Quarter: Verify branch alignment, not worktree
- Compatible with Beads daemon mode (requires single worktree)
- All references to git worktrees removed

**v3.1 (2026-01-14):** Beads integration
- South Quarter queries `bd ready` for available work
- Task claiming via `bd update --claim`
- Undefined goal path defers to South Quarter task selection
- Empty state handling when no ready tasks

**v3.0 (2026-01-11):** Precise loading architecture
- Inbox/outbox inheritance pattern
- No speculative loading
- Goal-informed context loading in East Quarter
- Workspace verification (not creation) in West Quarter
- No redundant operations (git status once)

**v2.0 (2026-01-10):** Token-optimized notation
- Compressed ceremony output (~75% token reduction)
- Symbolic notation (alchemical elements)

**v1.0:** Original LBRP ceremony

---

*The ceremony serves the work. The work serves all beings.*
