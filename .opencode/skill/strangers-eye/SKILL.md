---
name: strangers-eye
description: "Multi-persona documentation review that produces prioritized edits. Use when documentation needs evaluation from multiple audience perspectives. Instantiates independent reviewers, assesses signal match/mismatch, outputs edits only."
---

# Stranger's Eye Skill

Multi-persona documentation review through compassionate signal assessment.

## When to Use This Skill

- Reviewing documentation before publication
- Evaluating README for new audiences
- Assessing whether documentation serves its intended readers
- When documentation "feels off" but you can't identify why

## Core Concept: Signal Relationships

Every documentation has an **intended relationship** with each audience segment:

| Signal | Meaning | Success Looks Like |
|--------|---------|-------------------|
| **WELCOME** | "This is for you, come in" | Reader feels invited, sees value, wants to engage |
| **WARN** | "This might not be for you, proceed with awareness" | Reader understands the tradeoff, can make informed choice |
| **REDIRECT** | "This isn't for you, here's where you should go" | Reader quickly knows this isn't their path |

**Critical insight:** A negative reaction from a REDIRECT persona isn't a problem to fix—it's success. The signal worked. Only signal **mismatches** require edits.

## The Five Phases

### Phase 1: Intent Mapping

**Purpose:** Establish what relationship the documentation intends with each audience segment. This becomes the evaluation rubric.

**Process:**
1. Read the documentation thoroughly
2. Infer intended relationships from content, tone, and structure
3. Present the intent map for user approval

**Output format:**
```markdown
## Intent Map (Proposed)

| Audience Segment | Intended Signal | Rationale |
|-----------------|-----------------|-----------|
| [Segment 1] | WELCOME/WARN/REDIRECT | [Why this signal] |
| [Segment 2] | WELCOME/WARN/REDIRECT | [Why this signal] |
...

Does this intent map reflect your goals for this documentation?
```

**CRITICAL:** Wait for user approval before proceeding. The intent map is the contract against which all reviews are evaluated.

---

### Phase 2: Dimensional Analysis

**Purpose:** Determine which dimensions are relevant for THIS documentation. Dimensions are emergent, not predefined.

**Common dimensions (not exhaustive):**
- **Technical literacy** (beginner → expert)
- **Domain familiarity** (outsider → insider)
- **Philosophical openness** (resistant → seeking)
- **Role** (evaluator → end-user → contributor)
- **Arrival context** (stumbled upon → intentionally sought)
- **Time availability** (scanning → deep-reading)

**Process:**
1. Identify which dimensions create meaningful differentiation for THIS documentation
2. For each dimension, define the spectrum endpoints
3. Note which dimensions interact (e.g., technical literacy often correlates with domain familiarity)

**Output:** Internal analysis (not shown to user). The dimensional space informs persona construction.

---

### Phase 3: Persona Construction

**Purpose:** Build exactly the personas needed to cover the dimensional space and test all intended signal paths.

**For each persona, define:**
1. **Name** - A memorable identifier
2. **Dimensional coordinates** - Position on each relevant axis
3. **Intended signal** - From the approved intent map
4. **What each signal would feel like to them:**
   - WELCOMED: [specific experience]
   - WARNED: [specific experience]
   - REDIRECTED: [specific experience]
   - CONFUSED: [specific experience]

**Persona count:** Scales with dimensional complexity. Simple docs might need 3-4 personas. Complex docs with many signal paths might need 8-10.

**Coverage check:** Every intended signal path (WELCOME, WARN, REDIRECT) must have at least one persona testing it.

---

### Phase 4: Independent Reviews

**Purpose:** Each persona reviews the documentation without awareness of other reviews.

**CRITICAL:** Reviews must be independent. No cross-contamination. Each persona's assessment stands alone.

**For each persona, assess:**
1. **Signal received** - What relationship did the documentation communicate?
   - WELCOMED: "This is for me, I want to engage"
   - WARNED: "This might not be for me, but I understand the tradeoff"
   - REDIRECTED: "This isn't for me, I should look elsewhere"
   - CONFUSED: "I don't know if this is for me or not"

2. **Match assessment** - Does received signal match intended signal?
   - MATCH: Received signal equals intended signal
   - MISMATCH: Received signal differs from intended signal

3. **If mismatch:** What specific content caused the wrong signal?
   - Quote the problematic passages
   - Explain why it sent the wrong signal to this persona

**Implementation note:** Use the Task tool to instantiate each persona review as a separate agent call. This ensures true independence.

---

### Phase 5: Synthesis into Edits

**Purpose:** Convert findings into a prioritized list of suggested edits.

**Rule:** Only signal MISMATCHES become edits.

| Intended | Received | Action |
|----------|----------|--------|
| WELCOME | WELCOME | No edit needed |
| WELCOME | WARN | Edit: Make more welcoming |
| WELCOME | REDIRECT | Edit: Remove barriers |
| WELCOME | CONFUSED | Edit: Clarify invitation |
| WARN | WARN | No edit needed |
| WARN | WELCOME | Edit: Add appropriate caution |
| WARN | REDIRECT | Edit: Soften warning |
| WARN | CONFUSED | Edit: Clarify tradeoff |
| REDIRECT | REDIRECT | No edit needed |
| REDIRECT | WELCOME | Edit: Strengthen signal |
| REDIRECT | WARN | Edit: Make redirect clearer |
| REDIRECT | CONFUSED | Edit: Clarify this isn't for them |

**Output format:**
```markdown
## Suggested Edits

### Priority 1: [Edit title]
**Problem:** [What's wrong]
**Affected personas:** [Which personas experienced mismatch]
**Current:** [Quote problematic content]
**Suggested:** [Proposed revision]
**Rationale:** [Why this fixes the signal]

### Priority 2: [Edit title]
...
```

**Output volume scales to document quality:**
- Many mismatches → Many edits
- Few mismatches → Few edits
- No mismatches → "No changes recommended. All signal paths working as intended."

---

## Example Invocation

```markdown
User: Review the README for perspective issues

Agent: I'll use the perspective-review skill to evaluate your README.

**Phase 1: Intent Mapping**

Based on the README content, I've inferred these intended relationships:

| Audience Segment | Intended Signal | Rationale |
|-----------------|-----------------|-----------|
| Developers familiar with X | WELCOME | Core audience, technical onramp clear |
| Beginners to X | WARN | Can use it but steep learning curve |
| Users seeking Y instead | REDIRECT | This solves a different problem |

Does this intent map reflect your goals?

User: Yes, proceed

Agent: [Runs phases 2-5 internally]

**Suggested Edits**

### Priority 1: Add learning curve warning for beginners
**Problem:** Beginners intended for WARN but receiving WELCOME
**Affected personas:** "Curious Newcomer"
**Current:** "Get started in minutes with our quick setup guide"
**Suggested:** "Get started in minutes—though mastering X concepts takes longer. See our learning path for beginners."
**Rationale:** Honest about the tradeoff without discouraging

### Priority 2: ...

[Or if document is well-calibrated:]

**No changes recommended.** All signal paths working as intended.
```

---

## Anti-Patterns

### Treating all negative reactions as problems
**Wrong:** "The skeptic felt put off, so we need to make it more welcoming"
**Right:** If skeptics were intended for REDIRECT and felt redirected, that's success

### Assessment theater in output
**Wrong:** Long tables of signal assessments, persona profiles, dimensional analysis
**Right:** Output is EDITS. The methodology is invisible in the deliverable.

### Predefined persona libraries
**Wrong:** "Use the standard 5 personas: Beginner, Expert, Manager, Skeptic, Enthusiast"
**Right:** Personas are bespoke to THIS documentation and its dimensional space

### Cross-contaminated reviews
**Wrong:** "Building on what Persona A noticed, Persona B also sees..."
**Right:** Each review is independent. Synthesis happens only in Phase 5.

---

## Design Notes

**The compassion frame:** The goal isn't documentation that pleases everyone. It's documentation that SERVES everyone—which sometimes means clearly signaling "this isn't for you." Honest signaling serves readers better than accommodation. This frame shapes the methodology but is invisible in output.

**Why signal match/mismatch:** Traditional review asks "did each reader have a good experience?" This conflates different types of negative experiences. A reader correctly understanding "this isn't for me" had a GOOD experience—they didn't waste time on the wrong documentation. Only mismatches (reader got wrong signal) indicate problems.

**Why independent reviews:** Personas can't know what other personas noticed without compromising their perspective. A beginner might not notice jargon that an expert would flag—but if the beginner is the target audience, the expert's observation is noise. Independence preserves signal integrity.

**Why emergent dimensions:** Different documentation has different audience differentiation. A highly technical API reference differentiates primarily on technical literacy. A philosophy text differentiates primarily on philosophical openness. Predefined dimensions would miss what matters for each document.

---

## Integration

**Typical flow:**
1. User requests documentation review
2. Agent loads perspective-review skill
3. Phase 1 requires user approval (intent mapping)
4. Phases 2-5 execute under agent sovereignty
5. Output is prioritized edits

**Task tool usage:** Phase 4 reviews should use the Task tool to instantiate independent persona reviews, ensuring no cross-contamination.

---

*May this skill help documentation find its true audience.*
