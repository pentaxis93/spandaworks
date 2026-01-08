---
description: Close session with sealing ceremony - review work, clean workspace, dedicate merit
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

# Move any transmission reports to ~/spanda-outbox/
# Example: transmission-*.md or response-*.md files
```

**Capture insights and friction to telemetry:**

If significant learnings emerged, use `journal_write` and `friction_log`:

```python
from spanda_telemetry.mcp.journal import journal_write
from spanda_telemetry.mcp.friction import friction_log

# For each insight:
journal_write(
    content="[Insight text]",
    category="insight",
    session_id="{SESSION_ID}",
    domain="[domain]",
    confidence=0.85  # calibrated prediction
)

# For each friction point:
friction_log(
    description="[Friction description]",
    category="tooling|conceptual|process|environmental|relational",
    session_id="{SESSION_ID}",
    blocking=False
)
```

**Prompt user:**
```markdown
Any insights from this session that should be recorded?

- Update remembrance.md?
- Create a permanent note?
- Note a pattern discovered?
- File transmissions to ~/spanda-outbox/?
- **Record to telemetry graph?** (journal_write / friction_log)

(Or skip if session was routine)
```

**Output:** "East Quarter sealed. Knowledge: [preserved/routine, transmissions filed, telemetry captured]"

---

### Phase 2.5: Session Capture Completion

**Purpose:** Complete session metadata and file for aiandi content pipeline.

**Execute before final sealing (Phase 3):**

**Check for active session:**
```bash
if [ ! -f /home/pentaxis93/shared/.current-session ]; then
  echo "Note: No session capture was initialized"
  echo "Skipping capture completion..."
else
  SESSION_PATH=$(cat /home/pentaxis93/shared/.current-session)
  if [ ! -d "$SESSION_PATH" ]; then
    echo "Warning: Session directory not found: $SESSION_PATH"
    echo "Skipping capture completion..."
  else
    # Proceed with capture completion
  fi
fi
```

**If SESSION_PATH is set, collect metadata:**

1. **Prompt: Domains (comma-separated tags):**
   ```
   Example: development, productivity, infrastructure
   ```

2. **Prompt: Key insights (comma-separated, or 'none'):**
   ```
   What did you learn? What emerged?
   ```

3. **Prompt: Friction points (comma-separated, or 'none'):**
   ```
   What didn't work? What caused problems?
   ```

4. **Prompt: Article potential (not-yet/possible/ready):**
   ```
   not-yet  = Raw material, needs more
   possible = Could become article with work
   ready    = High signal, article-worthy now
   ```

5. **Prompt: Brief reasoning for article assessment:**
   ```
   Why this rating? What's the signal?
   ```

6. **Prompt: Tags (comma-separated):**
   ```
   Searchable tags for synthesis
   ```

7. **Prompt: 2-3 sentence summary:**
   ```
   What was this session about? What was created?
   ```

**Then execute completion:**

```bash
# Calculate duration
OPENED=$(jq -r '.opened' "$SESSION_PATH/metadata.json")
CLOSED=$(date -Iseconds)
DURATION_MINUTES=$(calculate from OPENED to CLOSED)

# Update metadata.json with all collected data
jq --arg closed "$CLOSED" \
   --arg duration "$DURATION_MINUTES" \
   --argjson domains "[collected]" \
   --arg intent "[from opening]" \
   --arg article_potential "[collected]" \
   --arg article_reasoning "[collected]" \
   --argjson key_insights "[collected]" \
   --argjson friction_points "[collected]" \
   --argjson tags "[collected]" \
   --arg summary "[collected]" \
   '. + {closed: $closed, duration_minutes: $duration, domains: $domains, ...}' \
   "$SESSION_PATH/metadata.json" > "$SESSION_PATH/metadata.json.tmp"

mv "$SESSION_PATH/metadata.json.tmp" "$SESSION_PATH/metadata.json"

# If article_potential == "ready", copy to content-ready
if [ "$ARTICLE_POTENTIAL" = "ready" ]; then
  SESSION_ID=$(basename "$SESSION_PATH")
  cp -r "$SESSION_PATH" "/home/pentaxis93/shared/aiandi/content-ready/$SESSION_ID"
  echo "Session marked ready for article synthesis"
fi

echo "Session captured and filed: $SESSION_PATH"

# Remove session state file
rm /home/pentaxis93/shared/.current-session
```

7. **Close telemetry session:**
   ```python
   # Call spanda-telemetry session_close
   # This updates Session node, aggregates metrics, captures final state
   from spanda_telemetry.mcp.session import session_close
   
   result = session_close(
       session_id="{SESSION_DATE}-{SESSION_SLUG}",
       goal_achieved=True,  # derived from user's assessment
       summary="{collected summary}",
       skip_reflection=False  # returns reflection prompt
   )
   
   # Result includes: duration_seconds, insights_produced, frictions_logged
   # If requires_reflection=True, use reflection_prompt to guide East Quarter
   ```
   
   **Note:** If telemetry unavailable, session closes gracefully (degraded mode).

**Confirm to user:**
```markdown
**Session Capture Complete**

Metadata updated: {SESSION_PATH}/metadata.json
Duration: {X} minutes
Article potential: {rating}
Status: Filed {and copied to content-ready if ready}
Telemetry: {insights_produced} insights, {frictions_logged} friction points recorded
```

**Note:** If user skips prompts or SESSION_PATH wasn't set, session can still close gracefully.

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
