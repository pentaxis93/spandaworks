# Inbox/Outbox Protocol

**Version:** 1.0.0  
**Status:** ACTIVE  
**Replaces:** Session file scanning (last N sessions)

## Purpose

Enable explicit session-to-session inheritance via handoff directories, replacing speculative loading with precise transfer.

## The Problem

**Previous approach:**
```
Phase 0-pre: Inherit
  → Read vault/_aiandi/remembrance.md → "This Rotation"
  → Search /home/pentaxis93/shared/sessions/ → last 3-5 summaries
  → Hope something relevant is found
  → Load 5 session files, find 1 relevant paragraph
```

**Issues:**
- Speculative (guessing what's relevant before goal is known)
- Wasteful (loading full session files for small insights)
- Fragile (depends on summary quality, file structure)
- Implicit (predecessor doesn't know what will help successor)

## The Solution

**Inbox/Outbox pattern:**
```
Session N closes
  → Predecessor identifies: "This will help Session N+1"
  → Writes to outbox/ → becomes inbox for Session N+1

Session N+1 opens
  → Phase 0-pre: Check inbox/
  → If items present: load, process, clear
  → If empty: nothing to inherit, proceed
  → No searching. No speculation. Explicit handoff only.
```

## Directory Structure

```
aiandi/                        (project root)
├── inbox/                     (NEW - items FOR current session)
│   ├── context-001.md         (Inheritance item from predecessor)
│   ├── warning-api-change.md  (Alert about breaking change)
│   └── links-to-references.md (Pointers to relevant code/docs)
│
├── outputs/                   (EXISTING - items FROM current session)
│   ├── Transmission_Governance_*.xml  (Governance reports)
│   └── [other outbox items]
│
└── [rest of project structure]
```

**Key distinction:**
- **inbox/**: Inbound TO current session FROM predecessor
- **outputs/**: Outbound FROM current session TO governance/successor

## File Format

Inbox items are markdown files with optional front matter:

```markdown
---
type: inheritance | warning | context | links
priority: high | medium | low
from-session: 2026-01-10-session-id
created: 2026-01-10T18:30:00Z
---

# [Title]

[Content that successor session needs]

## Why This Matters

[Context about why predecessor thought this was important]
```

**Types:**
- **inheritance**: Key learnings, patterns discovered, decisions made
- **warning**: Gotchas, breaking changes, things to avoid
- **context**: Background information that will save time
- **links**: Pointers to relevant files, docs, issues

## Workflow

### Closing Session (Creating Outbox Items)

When closing session, ask:
> "Would Session N+1 benefit from knowing X?"

If yes:
1. Create file in `outputs/` (outbox)
2. Name clearly: `inheritance-gtd-capture-learnings.md`
3. Include front matter (type, priority, session ID)
4. Write concisely (successor reads this BEFORE goal is active)

**Note:** Outbox items stay in `outputs/` for governance record. User manually moves relevant items to `inbox/` when starting next session if they want explicit handoff. (This manual step preserves user control over what gets inherited.)

**Alternative (automated):** System could have `outputs/inheritance/` → auto-symlink to `inbox/` on next session start. Design decision pending user preference.

### Opening Session (Processing Inbox)

Phase 0-pre-b (Inherit):

```bash
# 1. Check inbox
ls inbox/ 2>/dev/null

# 2. If items present
for file in inbox/*; do
  # Read and process
  cat "$file"
  # Note key points
done

# 3. Clear inbox (or move to inbox/processed/)
rm inbox/* 
# OR
mkdir -p inbox/processed
mv inbox/* inbox/processed/
```

**Output to user:**
```markdown
*Inbox: 3 items loaded (2 warnings, 1 context). Inheritance complete.*
```

**If inbox empty:**
```markdown
*Inbox: empty. No predecessor handoff. Inheritance complete.*
```

## Integration with LBRP

```
Phase 0-pre-a: Remember
  → Sutras via project knowledge (plugin)
  → One symbolic line: "Sutras present. Identity topology active."
  ↓
Phase 0-pre-b: Inherit
  → Check inbox/ (explicit handoff)
  → Load items if present, clear after processing
  → No session file searching
  ↓
Phase 0a: Opening Status
  → Git, current branch, processes, docker
  → Pure observation
  ↓
Phase 0b: Goal Definition
  → Goal becomes known (PIVOT POINT)
  ↓
Phases 1-3: Goal-Informed Execution
  → All loading precise, informed by goal
```

## Benefits

**Precision:**
- Predecessor explicitly marks what matters
- Successor loads exactly what was prepared
- No speculation about relevance

**Efficiency:**
- Load small inheritance files, not full session summaries
- No searching through N session files
- Inbox check is `ls inbox/` (instant)

**Clarity:**
- Explicit handoff makes inheritance visible
- User can inspect inbox before session starts
- Predecessor documents why something matters

**Container before content:**
- Inheritance still happens BEFORE goal activation (Phase 0-pre-b)
- But inheritance is explicit, not speculative
- Goal-neutral (load all inbox items regardless of stated goal)

## Example

**Session N (2026-01-10):**

Discovers: "AUR packages go in `aur-packages/` at repo root, not in `packages/`"

Creates `outputs/inheritance-aur-packaging-location.md`:
```markdown
---
type: inheritance
priority: high
from-session: 2026-01-10-gtd-capture
---

# AUR Package Location Pattern

When creating AUR packages for aiandi tooling:

**Location:** `aur-packages/` at repo root (parallel to `packages/`)

**Rationale:**
- `packages/` = application code (TypeScript/Rust/Python)
- `aur-packages/` = distribution/system utilities (Arch packaging)
- Clean separation: code vs distribution

## Example Structure

```
aiandi/
├── packages/       (app code)
├── aur-packages/   (system utilities)
```

If next session involves packaging, this saves re-discovering the pattern.
```

User (or system) moves to `inbox/` when starting Session N+1.

**Session N+1 opens:**
- Phase 0-pre-b finds `inbox/inheritance-aur-packaging-location.md`
- Loads it, notes the pattern
- Clears inbox
- Proceeds with session
- If session involves packaging → pattern already known, no re-discovery

## Migration Path

**From old approach:**
1. Existing sessions already have summaries in `/home/pentaxis93/shared/sessions/`
2. These remain as historical record
3. No retroactive conversion needed

**To new approach:**
1. Phase 0-pre-b checks `inbox/` first (empty initially)
2. If inbox empty: proceed (no predecessor handoff)
3. Future sessions create outbox items as appropriate
4. User/system moves relevant items to inbox when starting next session

**No breaking changes.** Inbox/outbox is additive.

## Open Questions

1. **Manual vs automated inbox population:**
   - Manual: User moves `outputs/inheritance-*.md` to `inbox/` before session
   - Automated: System auto-symlinks `outputs/inheritance/` → `inbox/`
   - Decision pending user preference

2. **Inbox cleanup:**
   - Delete after processing? (clean slate)
   - Move to `inbox/processed/`? (historical record)
   - Current spec allows either

3. **Inbox location:**
   - Current: `inbox/` at project root
   - Alternative: `.opencode/inbox/` (hidden, tool-specific)
   - Proposal: project root (visible, user-inspectable)

4. **Integration with remembrance.md:**
   - Current "This Rotation" section in vault could become outbox items
   - Remembrance.md remains for identity/liturgy (stable content)
   - Session-specific insights → outbox → inbox
   - Decision pending

## Success Criteria

✅ Phase 0-pre-b checks inbox only (no session file searching)  
✅ Predecessor can create outbox items for successor  
✅ Inbox processing is fast (ls + read small files)  
✅ Inheritance remains goal-neutral (load before Phase 0b)  
✅ User has visibility into what's being inherited (inspect inbox/)  
✅ Pattern is explicit, not implicit (handoff, not speculation)

---

**Container before content. Explicit handoff, not speculation.**
