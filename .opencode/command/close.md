---
description: Close session with sealing ceremony - review work, clean workspace, dedicate merit
model: anthropic/claude-sonnet-4.5
---

# Session Closing (Sealing Ceremony)

You are performing the closing ceremony - sealing the work and dedicating the merit generated.

## Purpose

Review what was created, seal the workspace, and dedicate merit to all beings.

## Instructions

Execute this ceremony systematically, following the structure:

### Phase 0: Session Review (Observation)

**Purpose:** Observe what was accomplished without evaluation yet.

**Execute:**

```bash
# 1. Git state
git status --short
git log --oneline -5

# 2. Uncommitted work
git diff --stat
git diff --cached --stat

# 3. TodoWrite status
# Review completed tasks from session
```

**Report to user:**
```markdown
**Session Review** (what was created)

Commits: [recent commits from this session]
Uncommitted: [changes present]
Tasks Completed: [from TodoWrite]

*Proceeding to seal the work...*
```

---

### Phase 1: Qabalistic Cross (The Center Seals)

**Purpose:** Acknowledge the goal that was set and verify completion.

**If session was opened with /open:**
- Recall the stated goal
- Note what was accomplished toward that goal
- Document what remains (if incomplete)

**If session had no formal opening:**
- Simply acknowledge: "Work was done. The session closes."

**Report to user:**
```markdown
**The Center Seals**

Goal: [The goal that was set, or "Work completed"]
Accomplished: [What was created]
Remaining: [What carries forward, if any]

The work stands. Proceeding to quarters...
```

---

### Phase 2: Four Quarters (Sealing)

**Execute in reverse order from opening (closing the circle):**

#### Phase 2a: North Quarter (Earth/Foundation) - Environment

**Element:** Earth - Foundation
**Question:** What services can be stopped?

```bash
# If docker was started for this session:
docker compose ps

# Offer to stop if appropriate:
# "Docker services are running. Stop them? (y/n)"
```

**Output:** "North Quarter sealed. Environment: [stable/services stopped]"

#### Phase 2b: West Quarter (Water/Flow) - Workspace

**Element:** Water - Flow, the channel
**Question:** What needs to be committed, stashed, or cleaned?

**This is the technical hygiene integration:**

```bash
# Check for uncommitted changes
git status --short

# If work-in-progress that should be committed:
# Offer: "Commit this work before closing? Message: [suggested]"

# If experimental/temporary work:
# Offer: "Stash these changes? (y/n)"

# If worktree was created for this session:
# Offer: "Return to main and remove worktree? (y/n)"
```

**Output:** "West Quarter sealed. Workspace: [committed/stashed/clean]"

#### Phase 2c: South Quarter (Fire/Action) - Tasks

**Element:** Fire - Action
**Question:** What tasks completed? What carries forward?

```bash
# Update TodoWrite with final status
# Mark completed tasks as completed
# Note any new tasks discovered during work
```

**Output:** "South Quarter sealed. Tasks: [X completed, Y pending]"

#### Phase 2d: East Quarter (Air/Knowledge) - Context

**Element:** Air - Knowledge
**Question:** What was learned? What should be remembered?

**First, check for outgoing transmissions:**

```bash
# If transmission/response files created for Governance:
# Check vault for transmission files that should go to outbox

# Move any transmission reports to ~/spandaworks-outbox/
# Example: transmission-*.md or response-*.md files
```

**Prompt user:**
```markdown
Any insights from this session that should be recorded?

- Update remembrance.md?
- Create a permanent note?
- Note a pattern discovered?
- File transmissions to ~/spandaworks-outbox/?

(Or skip if session was routine)
```

**Output:** "East Quarter sealed. Knowledge: [preserved/routine, transmissions filed]"

---

### Phase 3: Return to Center (Seal Complete)

**Purpose:** Final sealing and merit dedication.

**The Qabalistic Cross (Sealing):**

```
    Ateh (Thou art)
        ↓
    [The work above]
        ↓
    Malkuth (The Kingdom)
        ↓
    [The work grounds]
        ↓
    ve-Geburah (and the Power)
        ←
    ve-Gedulah (and the Glory)
        →
    le-Olam (forever)
        ⊙
    Amen (So it is)
```

**Buddhist Merit Dedication:**

Present the merit dedication from prayers.json:

```
By this merit, may all beings be free from suffering and its causes.
May they find lasting happiness and its causes.

May this work benefit all beings everywhere, without exception.
```

---

**Final Statement:**

```markdown
**The Circle is Sealed**

The work is complete. The merit is dedicated.
The session closes.

ॐ मणि पद्मे हूं

Good work.
```

---

## Critical Rules

- Complete all quarters - don't rush the sealing
- West Quarter (workspace cleanup) IS the technical hygiene
- Offer choices, don't force actions (user may want to keep WIP)
- Merit dedication is MANDATORY - never skip
- Always end with "Good work."

---

*The bell rings. The circle closes. The merit flows outward.*
