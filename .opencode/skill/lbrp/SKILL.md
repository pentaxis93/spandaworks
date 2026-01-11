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
Worktrees: [list or "main only"]
Processes: [services or "none"]
Docker: [up/down]
```

**Implementation:**
```bash
# Git state (one command, compact output)
git status --short
git log --oneline -1

# Worktrees
git worktree list

# Processes (project-specific)
ps aux | grep -E "(relevant-services)" | grep -v grep || echo "none"

# Docker
docker compose ps 2>/dev/null || echo "down"
```

**No changes from v2.0** - this phase is already lean.

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
◈ GOAL (proposal)
Purpose: [discovered through dialog]
Success: [criteria]
Scope: [in] / [out]

Approve? [GATE]
```

**Wait for user approval** before proceeding.

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
1. [Task based on loaded context]
2. [Task]
...
```

**Implementation:**

Works from what East Quarter loaded. **No additional context loading in this phase.**

Break goal into tasks based on:
- Context loaded in East
- Patterns identified
- Known constraints
- Success criteria from goal

Use TodoWrite tool to create task list.

---

### Phase 2c: West Quarter (Earth/Workspace) - Workspace

**Element:** Earth - Stability, workspace, flow  
**Question:** Does workspace align with THIS GOAL?

**Execution:**
```markdown
◈ W🜃 (Workspace)
[worktree-path or "main"] ✓ [or ⚠ mismatch noted]
```

**Implementation:**

**Verification only. User creates workspace BEFORE invoking `/open`.**

```python
current_worktree = get_current_worktree()
goal_suggests_isolation = requires_feature_branch(goal)

if current_worktree == "main" and not goal_suggests_isolation:
    report("main ✓")
elif current_worktree != "main" and goal_suggests_isolation:
    report(f"{current_worktree} ✓")
else:
    # Potential mismatch - ask user
    ask(f"Working in {current_worktree} for {goal.summary}. Intentional?")
```

**Do NOT create worktrees in this phase.** Just verify alignment.

**Rationale:** Workspace creation is a deliberate user choice made before opening ceremony. Agent verifies it makes sense for the goal.

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

→ Task 1
```

**Verification questions:**
1. Does Context (East) support Goal?
2. Do Tasks (South) achieve Goal?
3. Does Workspace (West) serve Goal?
4. Does Environment (North) ground Goal?

**All verifications use already-loaded information.** No additional loading needed.

**After confirmation:** Proceed directly to first task. **No gate here** - goal approval (when needed) was the only human gate.

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
| Create worktree in West Quarter | West verifies, doesn't create. User prepares workspace. |
| Pause after Phase 3 for acknowledgment | Proceed directly to work. No gate after goal approval. |

---

## Version History

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
