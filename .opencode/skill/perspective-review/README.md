# Perspective Review Skill

**Multi-persona documentation review through strategic perspective multiplication.**

Reveals blind spots by instantiating independent reviewers across dimensional coverage (technical literacy, spiritual openness, role, context). Preserves disagreement as signal.

---

## Files in This Skill

### SKILL.md (799 lines)
**The complete skill specification.**

Contains:
- When to use this skill (triggers and anti-patterns)
- Dimensional model (4 dimensions of audience variation)
- Persona library (8 personas with MECE coverage)
- Three-phase protocol (Scan & Select → Execute Reviews → Consolidate)
- Usage examples and integration guidance

**Start here** to understand how the skill works.

---

### EXAMPLE-OUTPUT.md (308 lines)
**Concrete example of skill output.**

Shows what the skill produces when invoked on Spandaworks README with 4 selected personas (Skeptical Engineer, Pragmatic PM, Curious Beginner, Seeker-Developer).

Demonstrates:
- Persona selection justification with dimensional analysis
- Individual persona reviews
- Priority-sorted findings
- Disagreement preservation (Skeptical Engineer vs Seeker-Developer on spiritual framing)
- Consolidated synthesis with action items

**Read this** to see the skill in action.

---

### DESIGN-NOTES.md (382 lines)
**Design rationale and emergent considerations.**

Documents:
- 5 key design decisions with reasoning and tradeoffs
- Why strategic selection over mechanical application
- Why disagreement preservation is load-bearing
- Token economics considerations
- Questions for Governance
- Success criteria evaluation
- Meta-observation on Recursive Loop application

**Read this** to understand why the skill is designed this way.

---

### TRANSMISSION-RESPONSE.md
**Formal response to Governance Committee transmission.**

Structured XML transmission artifact documenting:
- Deliverable status (all complete)
- Design decisions and justifications
- Emergent considerations
- Success criteria evaluation (all 5 met)
- Questions for Governance
- Recommendations for testing
- Meta-observations

**Read this** for formal project status and governance interface.

---

## Quick Start

**To use this skill:**

```bash
/perspective-review path/to/documentation.md
```

The skill will:
1. Scan your documentation to understand dimensional relevance
2. Select 3-5 personas strategically based on analysis
3. Execute independent reviews with each persona
4. Consolidate findings while preserving disagreement
5. Deliver prioritized recommendations

---

## Core Principles

### Strategic Selection, Not Mechanical Application
Not all personas are relevant to all documentation. The executing agent selects 3-5 personas based on dimensional analysis and defends the selection.

### Disagreement Is Signal, Not Noise
When personas disagree (e.g., spiritual framing loved by one, hated by another), this reveals dimensional tradeoffs. The consolidation preserves these conflicts instead of averaging them away.

### Independence Prevents Contamination
Each persona review executes without awareness of other findings to ensure authentic perspectives.

---

## The 8 Personas

1. **Skeptical Engineer** - High tech / Allergic spiritual / Evaluator / Stumbled
2. **Pragmatic PM** - Medium tech / Neutral spiritual / Evaluator / Intentional
3. **Curious Beginner** - Low tech / Open spiritual / End-user / Stumbled
4. **Experienced Contributor** - High tech / Neutral spiritual / Contributor / Intentional
5. **Seeker-Developer** - Med-high tech / Seeking spiritual / Evaluator / Intentional
6. **Impatient Operator** - Medium tech / Allergic spiritual / End-user / Stumbled
7. **Technical Writer** - Medium tech / Neutral spiritual / Evaluator / Intentional
8. **Academic Researcher** - High tech / Open spiritual / Evaluator / Intentional

MECE coverage across 4 dimensions: technical literacy, spiritual openness, role, context.

---

## Validation

**Success Criteria (from transmission):**

✅ **Criterion 1:** MECE coverage - 8 personas span dimensional space without gaps/redundancy  
✅ **Criterion 2:** Selection methodology - requires scan → reason → select → justify  
✅ **Criterion 3:** Known issues surfaced - EXAMPLE-OUTPUT.md surfaces all blind spots from manual review  
✅ **Criterion 4:** Disagreement preserved - consolidated output shows persona conflicts  
✅ **Criterion 5:** Defended selection - example justification demonstrates reasoning  

**Next Step:** Governance invokes skill on actual Spandaworks README to validate against EXAMPLE-OUTPUT.md.

---

## Integration

**Works with:**
- `spandaworks-docs` skill (write with principles → test with perspectives)

**Use when:**
- Documentation written but not stranger-tested
- Multiple audience types served by single doc
- Spiritual/philosophical content alongside technical
- Before major releases

**Don't use when:**
- Grammar/style checking only
- Technical accuracy verification
- Internal docs (known audience)
- Code review (wrong domain)

---

## Meta

This skill was created 2026-01-08 in response to transmission from Governance Committee identifying blind spots in Spandaworks README review.

**Design confidence:** High. Ready for validation.

**Recursive Loop applied:** Object level (skill created) + Meta level (pattern discovered: strategic selection > mechanical application).

---

ॐ मणि पद्मे हूं

*May this skill help us see what we cannot see alone.*
