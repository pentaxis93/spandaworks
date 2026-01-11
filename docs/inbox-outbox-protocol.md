# Inbox/Outbox Protocol Specification

**Version 1.0 — Explicit Inheritance Pattern**  
*Replaces speculative session file searches with deliberate handoff*

---

## Purpose

The inbox/outbox pattern enables **explicit inheritance** between sessions, replacing speculative loading (searching last N sessions hoping to find relevant context).

**Core principle:** If something is relevant for the next session, predecessor places it in inbox. If nothing is in inbox, nothing is inherited. No guessing, no hoping.

---

## Architecture

### Directory Structure

```
[project-root]/
├── inbox/           # Items TO current session FROM predecessor
│   ├── processed/   # Archive of consumed items
│   └── *.md         # Active inheritance items
│
└── outbox/          # Items FROM current session TO consumers
    └── *.md         # Governance reports, etc.
```

**Locations:**
- **Project root:** Base of git repository
- **Worktree aware:** If working in worktree, inbox/outbox at worktree root OR shared at main repo root (implementation choice)

---

## Inbox Pattern

### Purpose

Receive explicit inheritance from predecessor sessions.

### Structure

```
inbox/
├── [timestamp]-[type].md
├── [timestamp]-[type].md
└── processed/
    └── [consumed items moved here]
```

### Item Types

| Type | Purpose | Example Content |
|------|---------|-----------------|
| `learning.md` | Key insight from session | "JWT refresh token rotation pattern discovered" |
| `caution.md` | Warning about discovered issue | "Refresh endpoint MUST be rate-limited" |
| `context.md` | Loaded context still relevant | "ADR-007 on security patterns applies to next auth work" |
| `thread.md` | Open work requiring continuation | "Database migration paused at step 3, resume with..." |

### File Format

```markdown
---
from_session: 2026-01-10-auth-implementation
type: learning
priority: high
created: 2026-01-10T18:30:00Z
---

JWT refresh token rotation (new token per refresh, invalidate old) superior 
to long-lived tokens.

Implementation requires:
- Token hash storage
- Family IDs for reuse detection  
- Rate limiting (5/min/IP)
- Family invalidation on logout
```

**Frontmatter fields:**
- `from_session` (required): Session ID that created this item
- `type` (required): One of [learning, caution, context, thread]
- `priority` (required): One of [high, medium, low]
- `created` (required): ISO 8601 timestamp

**Content:** Markdown after frontmatter. Concise (apply Gratitude Test).

---

### Processing Behavior

**During Phase 0-pre (Inherit):**

```bash
# Check inbox
if [ -d "inbox" ] && [ "$(ls -A inbox/*.md 2>/dev/null)" ]; then
  echo "◈ INHERIT"
  
  # Process each item
  for file in inbox/*.md; do
    # Extract key info
    type=$(grep "^type:" "$file" | cut -d: -f2 | xargs)
    from=$(grep "^from_session:" "$file" | cut -d: -f2 | xargs)
    
    # Show content
    echo "[$type from $from]"
    sed '1,/^---$/d; /^---$/d' "$file"  # Content only
    
    # Move to processed
    mv "$file" "inbox/processed/"
  done
else
  echo "◈ INHERIT"
  echo "No inheritance items"
fi
```

**After processing:**
- Items moved to `inbox/processed/` for audit trail
- NOT deleted (allows reviewing what was inherited)
- Processed directory can be cleaned manually/periodically

---

### Creation Responsibility

**Predecessor session** (during closing ceremony, if applicable):

```bash
# If session produced non-trivial learning
if [ -n "$LEARNING" ]; then
  cat > inbox/$(date +%s)-learning.md <<EOF
---
from_session: $SESSION_ID
type: learning
priority: high
created: $(date -Iseconds)
---

$LEARNING
EOF
fi

# If discovered caution
if [ -n "$CAUTION" ]; then
  cat > inbox/$(date +%s)-caution.md <<EOF
---
from_session: $SESSION_ID
type: caution
priority: high
created: $(date -Iseconds)
---

$CAUTION
EOF
fi
```

**When to create inbox items:**
- Learning that future sessions need: YES
- Caution about discovered issue: YES
- Context that remains relevant: MAYBE (only if continuation likely)
- Open thread requiring pickup: YES
- General session notes: NO (that's what session files are for)

**Apply Gratitude Test:** Would future session thank you for this item?

---

## Outbox Pattern

### Purpose

Send items FROM current session TO external consumers (governance, other processes).

### Structure

```
outbox/
└── [item-type]-[timestamp].md
```

### Current Usage

**Governance reports:** Already in use (per transmission background).

Example:
```
outbox/
└── governance-report-2026-01-10.md
```

### File Format

Determined by consumer. For governance reports:
```markdown
# Governance Report: [Topic]

**Session:** [session-id]
**Date:** [date]

[Report content]
```

### Processing

**External processes** consume from outbox:
- Governance review processes
- Cross-repository coordination
- Build/deployment triggers
- etc.

**Not processed by sessions** - outbox is write-only from session perspective.

---

## Integration with LBRP

### Phase 0-pre: Inherit

**Old behavior (v1.0-v2.0):**
```bash
# Search last 5 sessions
for i in {1..5}; do
  session_file="shared/sessions/session-$i.md"
  if [ -f "$session_file" ]; then
    # Load and hope something is relevant
    cat "$session_file"
  fi
done
```

**Token cost:** ~2,800 tokens (loading 5 session files speculatively)

---

**New behavior (v3.0):**
```bash
# Check inbox only
if [ -d "inbox" ] && [ "$(ls -A inbox/*.md 2>/dev/null)" ]; then
  for file in inbox/*.md; do
    # Load explicitly inherited items
    process_inbox_item "$file"
  done
else
  echo "No inheritance items"
fi
```

**Token cost:** ~200 tokens (only explicit items) OR ~50 tokens (empty inbox)

**Savings:** ~2,600 tokens per session (when inbox empty, which is common)

---

## Migration Path

### Phase 1: Create Infrastructure

```bash
# In project root
mkdir -p inbox/processed
mkdir -p outbox

# Add to .gitignore (items are ephemeral)
echo "inbox/*.md" >> .gitignore
echo "!inbox/.gitkeep" >> .gitignore
echo "outbox/*.md" >> .gitignore
echo "!outbox/.gitkeep" >> .gitignore

# Keep directories tracked
touch inbox/.gitkeep outbox/.gitkeep
```

### Phase 2: Update Closing Ceremony

Modify closing ceremony to optionally create inbox items:

```markdown
◈ HARVEST
Learning: [if significant, will create inbox/learning.md]
Caution: [if discovered, will create inbox/caution.md]
```

### Phase 3: Update Opening Ceremony

Replace session file search with inbox check (already done in LBRP v3.0).

### Phase 4: Validation

First few sessions:
- Manually verify inbox/outbox behavior
- Confirm inheritance works for actual continuity needs
- Adjust item creation threshold if needed

---

## Design Rationale

### Why Explicit Handoff?

**Problem with speculative loading:**
- Load 5 session files (each ~2,000 tokens)
- Hope one contains relevant context
- Often find nothing relevant
- Waste ~10,000 tokens per opening

**Benefit of explicit handoff:**
- Predecessor decides what's relevant
- Only that content loaded
- Empty inbox = nothing to inherit (clear signal)
- Savings compound across sessions

### Why Not Just Better Session Search?

**Option:** Improve session file search (semantic search, relevance scoring, etc.)

**Issue:** Still speculative. Search algorithm guesses what's relevant.

**Explicit handoff wins:** Human (or AI with full context) decides what matters for next session. No guessing needed.

### Why Inbox AND Outbox?

**Symmetry:** Sessions both receive (inbox) and send (outbox).

**Inbox:** Session-to-session continuity  
**Outbox:** Session-to-governance continuity

Both are explicit, both replace speculation.

---

## Future Enhancements

### Multi-Repository Coordination

If working across multiple repos:
```
shared-inbox/
├── from-repo-A/
└── from-repo-B/
```

Cross-repository inheritance for coordinated work.

### Scheduled Cleanup

```bash
# Clean processed items older than 30 days
find inbox/processed -name "*.md" -mtime +30 -delete
```

### Inbox Priority Queue

Process high-priority items first:
```bash
for file in inbox/*-high-*.md; do
  process_inbox_item "$file"
done
for file in inbox/*-medium-*.md; do
  process_inbox_item "$file"
done
```

---

## Examples

### Example 1: Learning Inheritance

**Session A (closing):**
```bash
# Discovered JWT refresh token rotation pattern
cat > inbox/1736528400-learning.md <<EOF
---
from_session: 2026-01-10-auth-implementation
type: learning
priority: high
created: 2026-01-10T18:30:00Z
---

JWT refresh token rotation (new token per refresh, invalidate old) superior 
to long-lived tokens. Requires hash storage, family IDs, rate limiting.
EOF
```

**Session B (opening, Phase 0-pre):**
```markdown
◈ INHERIT
[learning from 2026-01-10-auth-implementation]
JWT refresh token rotation pattern discovered. Key implementation details noted.
```

**Token cost:** ~150 tokens (just the learning content)

---

### Example 2: Empty Inbox

**Session opening (Phase 0-pre):**
```markdown
◈ INHERIT
No inheritance items
```

**Token cost:** ~50 tokens (minimal acknowledgment)

---

### Example 3: Multiple Items

**Session opening (Phase 0-pre):**
```markdown
◈ INHERIT
[learning from 2026-01-10-auth]: JWT rotation pattern
[caution from 2026-01-10-auth]: Rate limiting on refresh endpoint required
[thread from 2026-01-09-migration]: Database migration paused at step 3, resume needed
```

**Token cost:** ~300 tokens (three concise items)

**Still far less than:** ~2,800 tokens for searching 5 session files

---

## Success Criteria

### For Sessions

- [ ] Inbox directory exists in project root
- [ ] Opening ceremony checks inbox (not session files)
- [ ] Closing ceremony can create inbox items
- [ ] Processed items moved to `inbox/processed/`

### For Inheritance Quality

- [ ] Only non-trivial learnings create inbox items
- [ ] Gratitude Test applied to item creation
- [ ] Empty inbox is common (not every session leaves inheritance)
- [ ] When items present, they're actually relevant

### For Token Economy

- [ ] Average inheritance cost < 500 tokens (vs ~2,800 previously)
- [ ] Empty inbox sessions cost ~50 tokens (vs ~2,800 previously)
- [ ] Savings compound across sessions

---

*Explicit handoff replaces speculation. Precision over coverage.*
