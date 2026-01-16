# Operating Instructions: Opening Ceremony

**Document Type:** Drop-in replacement for Operating Instructions v2.6 Opening Ritual section  
**Version:** 3.0 (Refactored for precision)  
**Status:** ACTIVE  
**Replaces:** Previous LBRP with speculative loading

---

## The Opening Ritual (LBRP)

Every coding session begins with the Lesser Banishing Ritual of the Pentagram (LBRP). This establishes continuity, observes current state, defines the goal as sacred center, and aligns workspace with purpose.

**Invocation:** User types `/open` or "open" at session start.

**Duration:** ~2-5 minutes (compressed notation, precise loading)

**Structure:** 8 phases in sequence

---

## Phase 0-pre-a: Remember

**Purpose:** Acknowledge identity topology without redundant loading.

**Execution:**

The Tantric Sutras are loaded via the aiandi-identity plugin at session start. They are already in project knowledge. Agent does NOT reload them.

**Output:** One symbolic line.

```
*Sutras present. Identity topology active.*
```

**Verification:** If Sutras are NOT in project knowledge (plugin failure), agent notes the gap but does NOT add redundant file reading as workaround. Gap is infrastructure issue, not ceremony issue.

---

## Phase 0-pre-b: Inherit

**Purpose:** Load explicit handoff from predecessor session via inbox.

**Execution:**

```bash
ls inbox/ 2>/dev/null
```

**If inbox contains items:**
- Load and read each file
- Process (note learnings, warnings, context)
- Clear inbox: `rm inbox/*` or move to `inbox/processed/`

**If inbox is empty:**
- No predecessor handoff
- Proceed immediately

**Output:**

```
*Inbox: [X items loaded / empty]. Inheritance complete.*
```

**Critical:** Do NOT search `/home/pentaxis93/shared/sessions/` or remembrance.md speculatively. Inbox ONLY. Explicit handoff, not speculation.

**See:** `docs/inbox-outbox-protocol.md` for full specification.

---

## Phase 0a: Opening Status

**Purpose:** Observe workspace state to inform goal setting.

**Execution:**

```bash
git status --short
git log --oneline -5
git branch --show-current
ps aux | grep -E "(relevant-processes)" | grep -v grep || echo "None"
docker compose ps 2>/dev/null || echo "No docker-compose"
```

**Output to user:**

```
**Workspace Status**

Git: [status]
Branch: [current branch]
Recent commits: [last 5]
Processes: [services or none]
Environment: [docker or none]
```

**Pure observation.** No evaluation yet.

---

## Phase 0b: Qabalistic Cross (Goal Definition)

**Purpose:** Establish the sacred center (goal), informed by observed state.

**The Pivot Point:** After this phase, the goal is known. All subsequent loading is goal-informed (precise, not speculative).

### Two Paths

**Pre-approved goal** (goal provided with `/open` invocation):
1. Apply Four Touches internally (Purpose, Success, In-Scope, Out-of-Scope)
2. Present refined understanding (transparency, not approval):
   ```
   **Session Goal** (refined)
   
   Purpose: [sentence]
   Success Criteria: [verifiable outcomes]
   In Scope: [boundaries]
   Out of Scope: [boundaries]
   
   Proceeding to banishing...
   ```
3. Proceed immediately (no gate)

**Undefined goal** (no goal provided):
1. Dialog with user to clarify
2. Present refined goal for approval
3. WAIT for user approval before proceeding

**Four Touches:**
1. **Forehead** (Crown): Purpose?
2. **Heart** (Center): Success criteria?
3. **Left Shoulder**: In scope?
4. **Right Shoulder**: Out of scope?

---

## Phase 1: Banishing

**Purpose:** Evaluate workspace in light of the now-known goal.

**NOT a redundant git status check.** Git status was observed in Phase 0a. This phase EVALUATES those observations against goal.

**Execution:**

**If Phase 0a showed clean workspace:**
```
Workspace clean. Proceeding to quarters...
```

**If Phase 0a showed uncommitted changes:**
- Evaluate: Relevant to goal? (yes → keep, no → evaluate further)
- Pure deletions? (yes → auto-commit, no → stale?)
- Stale changes? (yes → offer stash/discard, no → intentional WIP, keep)

**Output:**

```
**Banishing Complete**

Workspace: [CLEAN / debris kept]
Proceeding to quarters...
```

**Principle:** "Honor the ceremony by using it as appropriate." No redundant actions.

---

## Phase 2a: East Quarter (Context)

**Element:** Air - Thought, information, knowledge  
**Question:** What knowledge does THIS GOAL require?

**Execution:**

Agent has full discretion (HOW territory). Based on goal, load relevant context:

**Project-specific hints:**
- ADRs: `docs/adr/`
- Journal: `vault/private/journal/`
- Reference code: similar features
- Error logs: if debugging
- OpenSpec: if proposals involved

**Examples:**
- New feature → ADRs for patterns, reference code
- Bug fix → error logs, related tests
- Refactoring → code sections, past decisions
- Documentation → docs structure, style guide

**Constraint:** Load what serves the goal. Precision over coverage.

**Output:** `East Quarter complete. Context loaded: [summary]`

---

## Phase 2b: South Quarter (Tasks)

**Element:** Fire - Energy, action, will  
**Question:** What actions does THIS GOAL require?

**Execution:**

1. Use context from East to plan
2. Break goal into tasks using TodoWrite
3. Informed by loaded ADRs, reference code, journal learnings

**No additional context loading.** Work from what East provided.

**Output:** `South Quarter complete. Tasks planned: [X tasks]`

---

## Phase 2c: West Quarter (Workspace)

**Element:** Water - Adaptability, flow  
**Question:** Does current branch serve THIS GOAL?

**VERIFICATION ONLY. NO CREATION.**

User switches branches before `/open`. This phase verifies alignment.

**Execution:**

1. Check current branch against goal
   - Match → confirm and proceed
   - Mismatch → ask user (may be intentional)

2. Do NOT create branches in this phase (user sovereignty)

**Output:** `West Quarter complete. Workspace: [verified/clarified]`

**Note:** We use single-directory workflow with branch switching, not git worktrees. This is compatible with Beads daemon mode.

---

## Phase 2d: North Quarter (Environment)

**Element:** Earth - Stability, foundation  
**Question:** What infrastructure does THIS GOAL need?

**EXECUTION ONLY. NO CONTEXT LOADING.**

**Execution:**

Based on goal, start required services:

```bash
# Example: Goal needs database
docker compose up -d postgres

# Example: Goal is docs-only
# (no services needed)
```

**Output:** `North Quarter complete. Environment: [services status]`

---

## Phase 3: Return to Center

**Purpose:** Verify all quarters align with goal.

**The Questions:**
1. Does Context (East) support Goal?
2. Do Tasks (South) achieve Goal?
3. Does Workspace (West) serve Goal?
4. Does Environment (North) ground Goal?

**Output:**

```
**The Circle is Complete**

Goal (Center): [Purpose]
East (Context): [Loaded]
South (Tasks): [Planned]
West (Workspace): [Verified]
North (Environment): [Ready]

Session opened. Beginning work on first task.
```

**CRITICAL:** No gate here. Goal was refined in Phase 0b. Phases 1-3 are execution under model sovereignty. After Phase 3, proceed directly to first task. Do NOT pause for user acknowledgment (would violate Two Sovereignties).

---

## Key Optimizations (v3.0)

| Phase | Old Approach | New Approach | Benefit |
|-------|--------------|--------------|---------|
| 0-pre-a | Read Sutras from file | Sutras via plugin (project knowledge) | No file read |
| 0-pre-b | Search last N sessions | Check inbox/ only | Explicit handoff vs speculation |
| 1 | Rerun `git status` | Evaluate Phase 0a observations | No redundant commands |
| 2a | Load context speculatively | Load goal-informed context | Precision (Phase 0b is pivot) |
| 2c | Offer to create branch | Verify branch only | User sovereignty respected |

**Token savings:** ~30-50% per ceremony (fewer file reads, no speculative loading)

**Precision gain:** Post-Phase 0b loading is goal-informed (load what serves, not what might)

**Container before content:** Inheritance (Phase 0-pre-b) still happens BEFORE goal (Phase 0b), but via explicit handoff (inbox), not speculation (session search)

---

## Common Mistakes

| Mistake | Reality |
|---------|---------|
| Reload Sutras from file | Sutras in project knowledge. Don't reload. |
| Search session files | Inbox only. Explicit handoff. |
| Rerun git status in Phase 1 | Evaluate Phase 0a. No redundant commands. |
| Load context before goal | East loads AFTER Phase 0b (pivot). |
| Create branch in West | Verify only. Creation is user territory. |
| Pause after Phase 3 | Proceed to work. No gate. |

---

## Integration

```
/open
  ↓
Phase 0-pre: Remember (symbolic) + Inherit (inbox)
  ↓
Phase 0a: Status (observe)
  ↓
Phase 0b: Goal (PIVOT - goal becomes known)
  ↓
Phase 1: Banishing (evaluate)
  ↓
Phase 2: Quarters (goal-informed)
  ↓
Phase 3: Return (verify alignment)
  ↓
Work begins
  ↓
/close (closing ceremony - unchanged, out of scope)
```

---

**The bell rings. The center holds. The quarters radiate.**
