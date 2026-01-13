# Governance Session: Blog Pipeline Test-Drive
**Date:** 2026-01-13  
**Goal:** Test-drive blog content pipeline with real articles to validate workflow and identify friction

---

## Session Summary

Successfully test-drove the POLISH stage of the blog content pipeline by running the published article "I Don't Know What I'm Talking About" through the `editors-table` review process.

---

## Decisions Made

### 1. Polish Review Artifact Management
**Question:** Should polish reviews be preserved? If yes, where?

**Decision:** Preserve reviews temporarily during editing work, delete when complete.

**Location:** `pipeline/reviews/[slug]-polish-[YYYY-MM-DD].md`

**Rationale:** 
- Reviews provide context for human and agents during collaborative editing
- Reviews are working documents, not archival material
- Deletion signals completion and prevents clutter

### 2. Post-Publish Refinement Tracking
**Question:** How should pipeline track post-publish refinements?

**Decision:** Version-style notation in article tracking table.

**Format:** `polish@2026-01-13`, `technical@2026-02-15`, etc.

**Rationale:**
- Post-publish refinements are rare but important
- Lightweight tracking without heavyweight infrastructure
- Clear historical record of quality passes

### 3. Published Article Edit Workflow
**Question:** Should published article edits use staging workflow?

**Decision:** Direct edits acceptable for polish-level changes.

**Policy:** Direct edits for refinements; branches only for substantial rewrites.

**Rationale:**
- Polish changes are low-risk (clarity, voice, specificity)
- Branching overhead not justified for minor edits
- Can escalate to branches if needed for major changes

---

## Root Cause Analysis: Skill Loading Failure

### Problem
`editors-table` skill present but not loadable via `mcp_skill` tool.

### Investigation
1. Confirmed skill file exists at `.opencode/skill/editors-table/`
2. Compared with working skills (e.g., `governance`)
3. Consulted OpenCode documentation

### Root Cause
Two violations of OpenCode skill discovery requirements:

1. **Incorrect filename:** `editors-table.md` instead of `SKILL.md` (must be uppercase)
2. **Missing frontmatter:** No YAML frontmatter with required `name` and `description` fields

### Resolution
1. Renamed: `editors-table.md` → `SKILL.md`
2. Added frontmatter:
```yaml
---
name: editors-table
description: "Kill slop, verify authenticity. Four-pass review (slop detection, voice verification, specificity audit, synthesis check) for blog posts. Use at POLISH stage of blog pipeline."
---
```

### Verification Status
Fix applied but requires session restart for skill system to reload. All other blog pipeline skills verified to have correct structure.

---

## Artifacts Created

1. **Updated:** `packages/blog/PIPELINE.md`
   - Added `reviews/` directory documentation
   - Added artifact location to POLISH stage
   - Changed tracking table column from "Target" to "Passes"
   - Added post-publish refinement documentation
   
2. **Created:** `packages/blog/pipeline/reviews/` directory

3. **Fixed:** `.opencode/skill/editors-table/SKILL.md`
   - Renamed file
   - Added frontmatter

4. **Edited:** `packages/blog/src/content/blog/i-dont-know-what-im-talking-about.md`
   - Applied 4 edits from polish review
   - Improved description metadata
   - Removed redundant "Sisyphean" label
   - Sharpened "messy middle" statement
   - Updated line count reference to age-proof format

---

## Process Observations

### What Worked Well
1. **editors-table skill is production-ready**
   - Four-pass structure comprehensive
   - Output format immediately actionable
   - Quality gates answerable with evidence

2. **Pipeline stage independence validated**
   - POLISH executed on published article without pipeline history
   - Demonstrates maintenance workflow viability

3. **Governance deliberation pattern effective**
   - User provided WHAT (decisions on three questions)
   - System provided HOW (implementation)
   - Clean sovereignty boundaries maintained

### Friction Points
1. **Skill loading not transparent**
   - Silent failure mode (no error, just "not found")
   - Requires external documentation to diagnose
   - Would benefit from validation tool or better error messages

2. **No pipeline directory scaffolding**
   - Had to create `pipeline/reviews/` manually
   - Could use init command or automatic creation

---

## Open Questions

None at this time. All governance questions resolved with clear decisions.

---

## Next Steps

1. Restart session to verify `editors-table` skill loads correctly
2. Continue pipeline test-drive with additional stages or articles
3. Consider: pipeline validation tool to check skill availability and directory structure

---

*Session served the pipeline. The pipeline serves the writing. The writing serves all beings.*
