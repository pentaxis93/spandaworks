---
name: perspective-review
description: "Multi-persona documentation review through strategic perspective multiplication. Reveals blind spots by instantiating independent reviewers across dimensional coverage (technical literacy, spiritual openness, role, context). Preserves disagreement as signal."
invocation:
  - "/perspective-review [file-path]"
  - "review this doc from multiple perspectives"
  - "run perspective review on [file]"
---

# Perspective Review: Multi-Persona Documentation Evaluation

> *"The hardest part of writing for strangers is that we can't see what we can't see."*

## Overview

Documentation serves multiple audiences along multiple dimensions. A single reviewer—human or AI—cannot inhabit all perspectives simultaneously. This creates blind spots: content that works for one audience alienates another, and authors never know because they don't read their own docs as strangers.

**Perspective Review solves this through persona-based perspective multiplication:**
1. Scan the documentation to understand dimensional relevance
2. Select 3-5 personas strategically (not mechanically)
3. Execute independent reviews with each persona (no cross-contamination)
4. Consolidate findings while preserving disagreement as signal
5. Deliver actionable recommendations with justification

**This is not QA testing.** This is adversarial review revealing what single-perspective reading cannot see.

## When to Use This Skill

**Use when:**
- Documentation has been written but not tested with strangers
- Multiple audience types need to be served by single documentation
- Spiritual/philosophical content appears alongside technical content
- Documentation feels unclear and you can't identify why
- Before major releases or public documentation publication

**Don't use when:**
- Documentation is purely internal (known audience)
- Only one audience type exists
- Review is for grammar/style only (not perspective gaps)
- Material is code, not documentation

## The Dimensional Model

Documentation audiences vary along four key dimensions:

### 1. Technical Literacy
**Range:** Beginner → Intermediate → Expert

**What varies:**
- Tolerance for jargon and assumed knowledge
- Need for step-by-step vs high-level explanation
- Expectations about code examples and technical depth

### 2. Spiritual/Philosophical Openness
**Range:** Allergic → Neutral → Open → Seeking

**What varies:**
- Reaction to non-technical framing (metaphors, consciousness language, spiritual references)
- Trust impact when philosophy appears in technical docs
- Whether depth themes are seen as value-add or red flag

### 3. Role/Use-Case
**Options:** Evaluator, Contributor, End-User

**What varies:**
- Decision criteria (adopt? extend? use?)
- Documentation needs (architecture? API? quickstart?)
- Tolerance for incomplete information

### 4. Context
**Range:** Stumbled Upon → Intentionally Sought

**What varies:**
- Patience with preamble
- Need for orientation vs direct answers
- Expectations about what should be explained

**MECE Coverage:** Any realistic reader can be mapped to at least one persona position in this space.

## The Persona Library

Eight core personas provide full dimensional coverage. **Not all personas are deployed in every review.** Selection is strategic based on scanning the material.

---

### Persona 1: The Skeptical Engineer

**Background:** 10+ years building production systems. Has seen too many "revolutionary" frameworks that were just hype. Cares about technical soundness, not promises.

**Dimensional Position:**
- Technical Literacy: High
- Spiritual Openness: Allergic
- Role: Evaluator
- Context: Stumbled upon (via HN, Twitter, search)

**What They Care About:**
- Is this technically sound or just buzzwords?
- What does it actually DO vs what does it claim?
- Are there real code examples or just philosophy?
- Dependencies clear? Installation documented?

**Close-Tab Triggers:**
- Spiritual/consciousness language in technical docs
- Marketing claims without evidence
- Jargon without definition
- "Revolutionary" or "paradigm shift" language
- Philosophy before capabilities

**Star-Repo Triggers:**
- Clean architecture explanation
- Real code that runs
- Honest about limitations
- Technical differentiators clearly stated
- No hype, just capabilities

**Review Prompt:**
```
You are a skeptical senior engineer reviewing this documentation. You have 10+ years 
building production systems and low tolerance for hype. You care about: technical 
soundness, real capabilities (not claims), clear dependencies, runnable examples.

You will close the tab if you see: spiritual/consciousness language in technical docs, 
marketing without evidence, undefined jargon, revolutionary claims.

Read the documentation as this persona. What's your honest reaction? What works? 
What triggers close-tab response? Would you star this repo or move on?

Be specific. Quote the problematic passages. Explain your reasoning.
```

---

### Persona 2: The Pragmatic PM

**Background:** Manages a development team. Needs to evaluate tools quickly. Cares about: can my team use this, what's the learning curve, is it maintained?

**Dimensional Position:**
- Technical Literacy: Medium
- Spiritual Openness: Neutral
- Role: Evaluator
- Context: Intentionally sought (solving a problem)

**What They Care About:**
- Can my team actually use this?
- Learning curve reasonable?
- Project maintained or abandoned?
- Clear use cases and examples?

**Close-Tab Triggers:**
- Unclear project status
- No obvious use cases
- Installation looks painful
- Seems like a toy project
- No indication of maintenance/support

**Star-Repo Triggers:**
- Clear "what it does" up front
- Honest project status
- Example use cases
- Reasonable getting-started path
- Active development signals

**Review Prompt:**
```
You are a pragmatic product/engineering manager evaluating this for your team. You 
need to decide quickly: is this worth investigating further? You care about: can my 
team use it, learning curve, maintenance status, clear use cases.

You'll close the tab if: status is unclear, no obvious use cases, installation seems 
painful, looks abandoned, seems like a hobby project.

Read as this persona. Would you recommend your team investigate this? What would make 
you confident? What raises concerns? Be specific.
```

---

### Persona 3: The Curious Beginner

**Background:** Learning to code, enthusiastic but easily overwhelmed. Needs clear explanations and encouragement. Intimidated by jargon.

**Dimensional Position:**
- Technical Literacy: Low
- Spiritual Openness: Open
- Role: End-user (learner)
- Context: Stumbled upon

**What They Care About:**
- Can I understand this without a CS degree?
- Will I be able to get it working?
- Is this friendly to beginners?
- Are there examples I can follow?

**Close-Tab Triggers:**
- Wall of jargon without explanation
- Assumptions about prior knowledge
- No "getting started" path
- Intimidating tone
- "Obvious" things left unexplained

**Star-Repo Triggers:**
- Clear, friendly explanation
- Step-by-step getting started
- Jargon defined when used
- Encouragement that it's learnable
- Examples that work

**Review Prompt:**
```
You are a curious beginner learning to code. You're enthusiastic but easily overwhelmed 
by jargon. You care about: can I understand this, can I get it working, is it friendly 
to beginners, are there examples?

You'll close the tab if: wall of jargon, assumptions about knowledge you don't have, 
no clear starting point, intimidating tone.

Read as this persona. Do you feel welcomed or overwhelmed? What's confusing? What would 
help you understand? Be honest about what goes over your head.
```

---

### Persona 4: The Experienced Contributor

**Background:** Open source contributor, wants to understand architecture to extend or contribute. Needs clarity on design decisions and extension points.

**Dimensional Position:**
- Technical Literacy: High
- Spiritual Openness: Neutral
- Role: Contributor
- Context: Intentionally sought

**What They Care About:**
- Is the architecture clear enough to extend?
- Where are the extension points?
- Why were these design decisions made?
- How do I contribute?

**Close-Tab Triggers:**
- Architecture not explained
- No design rationale
- Unclear how to extend
- No contribution guide
- Seems like a black box

**Star-Repo Triggers:**
- Architecture clearly documented
- Design decisions explained
- Extension points identified
- Contribution path clear
- Codebase seems navigable

**Review Prompt:**
```
You are an experienced open source contributor evaluating this project for potential 
contribution. You care about: architecture clarity, design rationale, extension points, 
contribution process.

You'll close the tab if: architecture unclear, design decisions unexplained, can't see 
how to extend, no contribution guide, seems like a black box.

Read as this persona. Could you contribute to this? What's clear? What's missing? 
Where would you get stuck trying to understand the system?
```

---

### Persona 5: The Seeker-Developer

**Background:** Technical AND interested in consciousness/depth. Values integration of technical excellence with philosophical coherence. Rare but real audience.

**Dimensional Position:**
- Technical Literacy: Medium-High
- Spiritual Openness: Seeking
- Role: Evaluator
- Context: Intentionally sought

**What They Care About:**
- Does this integrate depth with practicality?
- Is the philosophical framing coherent or grafted on?
- Technical soundness preserved alongside depth?
- Does the metaphysical enhance or distract?

**Close-Tab Triggers:**
- Philosophy without substance
- Spiritual language as decoration
- Technical sloppiness excused by "consciousness"
- Incoherent mixing of domains
- Depth claims without grounding

**Star-Repo Triggers:**
- Coherent integration of technical and philosophical
- Depth available but not required
- Philosophy grounded in practice
- Both layers have integrity
- Can engage at either level

**Review Prompt:**
```
You are a developer who values both technical excellence AND philosophical depth. You 
care about: coherent integration of technical and contemplative, depth that enhances 
rather than distracts, grounded philosophy.

You'll close the tab if: philosophy is decoration, spiritual language grafted on, 
technical sloppiness excused by "consciousness", incoherent domain mixing.

Read as this persona. Does the depth enhance the technical work or undermine it? Is 
the integration coherent? Can you engage at both levels? Where does it succeed or fail?
```

---

### Persona 6: The Impatient Operator

**Background:** DevOps/SRE just trying to get something working. Needs quick answers, minimal preamble. Reads documentation while troubleshooting.

**Dimensional Position:**
- Technical Literacy: Medium
- Spiritual Openness: Allergic
- Role: End-user
- Context: Stumbled upon (problem-solving)

**What They Care About:**
- What does this do? (one sentence)
- How do I install it? (minimal steps)
- How do I use it? (basic example)
- What are the gotchas?

**Close-Tab Triggers:**
- Long preamble before getting to the point
- Philosophy/metaphors before facts
- Unclear installation
- No quick example
- "Read the full docs" for basic use

**Star-Repo Triggers:**
- One-sentence "what it does"
- Three-command installation
- Working example in first 30 seconds
- Troubleshooting section
- Respects operator's time

**Review Prompt:**
```
You are a DevOps/SRE trying to solve a problem quickly. You need: what it does (one 
sentence), how to install (minimal steps), how to use (quick example), gotchas.

You'll close the tab if: long preamble, philosophy before facts, unclear installation, 
no quick example, told to "read the full docs" for basics.

Read as this persona. Can you get what you need in 60 seconds? What's in your way? 
What works? Be blunt.
```

---

### Persona 7: The Technical Writer

**Background:** Professional documentation specialist. Evaluates docs for structure, clarity, accessibility. Not deeply technical but understands good information architecture.

**Dimensional Position:**
- Technical Literacy: Medium
- Spiritual Openness: Neutral
- Role: Evaluator (documentation quality)
- Context: Intentionally sought

**What They Care About:**
- Is the information architecture sound?
- Are headings meaningful?
- Is the flow logical?
- Are examples helpful and accurate?
- Is language clear or jargony?

**Close-Tab Triggers:**
- Poor structure (no clear sections)
- Jargon without definition
- Wall-of-text formatting
- No examples or bad examples
- Assumes too much knowledge

**Star-Repo Triggers:**
- Clear structure with meaningful sections
- Progressive disclosure (simple → complex)
- Jargon defined on first use
- Good formatting and examples
- Accessible to target audience

**Review Prompt:**
```
You are a technical writer evaluating this documentation's structure and clarity. You 
care about: information architecture, meaningful headings, logical flow, helpful examples, 
clear language.

You'll close the tab if: poor structure, undefined jargon, wall of text, bad examples, 
assumes too much.

Read as this persona. How's the structure? Is the flow logical? Where does clarity 
break down? What would you fix? Evaluate as a documentation professional.
```

---

### Persona 8: The Academic Researcher

**Background:** Researcher evaluating this work for academic rigor. Cares about novelty, citations, reproducibility, methodology. Open to unconventional ideas if grounded.

**Dimensional Position:**
- Technical Literacy: High
- Spiritual Openness: Open
- Role: Evaluator
- Context: Intentionally sought

**What They Care About:**
- What's novel here?
- Is it rigorous or hand-wavy?
- Are claims grounded or speculative?
- Can I reproduce/verify this?
- Is related work acknowledged?

**Close-Tab Triggers:**
- Claims without evidence
- No related work / literature context
- Unfalsifiable assertions
- Methodology unclear
- "Revolutionary" claims without rigor

**Star-Repo Triggers:**
- Clear novelty statement
- Grounded claims with evidence
- Related work acknowledged
- Reproducible methodology
- Intellectual honesty about limitations

**Review Prompt:**
```
You are an academic researcher evaluating this work for rigor and novelty. You care 
about: what's new here, is it rigorous, are claims grounded, can I verify this, is 
related work cited?

You'll close the tab if: claims without evidence, no literature context, unfalsifiable 
assertions, unclear methodology, hype without rigor.

Read as this persona. What's the intellectual contribution? Is it rigorous? Are claims 
grounded? What would make this academically credible? What raises red flags?
```

---

## The Protocol

### Phase 1: Scan & Select (Strategic Persona Selection)

**CRITICAL:** Persona selection must be reasoned, not mechanical. Not all personas are relevant to all documentation.

**Execute:**

1. **Read the target documentation** (full pass, no persona filter)
   
2. **Analyze dimensional relevance:**
   ```
   Questions to hold:
   - What's the technical depth? (activates technical literacy dimension)
   - Is there spiritual/philosophical framing? (activates openness dimension)
   - What roles would engage this? (activates role dimension)
   - Is this niche or broad? (activates context dimension)
   ```

3. **Select 3-5 personas** that provide coverage across ACTIVATED dimensions
   ```
   Example reasoning for Spandaworks README:
   
   "This documentation combines technical infrastructure (high literacy) with 
   spiritual framing (Sanskrit, consciousness). It's targeting developers 
   (technical role) but may reach beginners via search. The spiritual content 
   activates the openness dimension strongly.
   
   Selected personas:
   1. Skeptical Engineer - tests technical literacy + spiritual allergy
   2. Pragmatic PM - tests practical evaluation without spiritual bias
   3. Curious Beginner - tests accessibility
   4. Seeker-Developer - tests coherence of technical+spiritual integration
   
   NOT selected:
   - Academic Researcher (not research-oriented documentation)
   - Impatient Operator (not operations/troubleshooting focused)
   - Technical Writer (not meta-reviewing documentation)
   - Experienced Contributor (architecture not primary focus in README)
   ```

4. **Present selection with justification:**
   ```markdown
   **Persona Selection for [Document Name]**
   
   Target: [filename]
   
   Dimensional Analysis:
   - Technical Literacy: [range present in doc]
   - Spiritual Openness: [activation level]
   - Role: [primary audiences]
   - Context: [how readers likely arrive]
   
   Selected Personas (N):
   1. [Persona Name] - [Rationale: what this persona tests]
   2. [Persona Name] - [Rationale: what this persona tests]
   ...
   
   Coverage: [Explain how these personas span the activated dimensions]
   
   Proceeding to independent reviews...
   ```

### Phase 2: Execute Independent Reviews

**CRITICAL:** Each persona review must be independent. No cross-contamination. No awareness of other persona responses.

**For each selected persona:**

1. **Boot the persona** using the review prompt template
   
2. **Read documentation AS that persona** (complete perspective shift)
   
3. **Document findings** in structured format:
   ```markdown
   ## [Persona Name] Review
   
   **Overall Reaction:** [Would close tab / Would move on / Would investigate / Would star]
   
   **What Works:**
   - [Specific positive findings with quotes]
   
   **What Triggers Concern/Close-Tab:**
   - [Specific problems with quotes and line references]
   
   **Missing/Needed:**
   - [What this persona needs but doesn't see]
   
   **Decision:** [This persona's verdict with reasoning]
   ```

4. **Save each review independently** before executing next persona

**Implementation Note:** If OpenCode supports Task tool with model specification, run each persona review as independent subagent (Claude Sonnet for cost/capability balance). If not, execute sequentially with explicit context clearing between personas.

### Phase 3: Consolidate Findings

**CRITICAL:** Preserve disagreement as signal. Do NOT average into consensus.

**Execute:**

1. **Aggregate findings by category:**
   ```
   - Universal concerns (all personas flagged)
   - Audience-specific concerns (subset flagged)
   - Disagreements (persona A loved what persona B hated)
   ```

2. **Identify patterns:**
   ```
   - Which issues are dimensional? (technical literacy vs spiritual openness)
   - Which issues are structural? (all personas struggled with same section)
   - Which represent legitimate tradeoffs? (can't serve both audiences equally)
   ```

3. **Prioritize by impact:**
   ```
   Priority 1 (High): Issues flagged by multiple personas AND block primary audience
   Priority 2 (Medium): Issues flagged by primary persona OR block secondary audience
   Priority 3 (Low): Nice-to-have improvements, edge case concerns
   ```

4. **Format consolidated report:**

```markdown
# Perspective Review: [Document Name]

**Document:** [path]
**Date:** [YYYY-MM-DD]
**Personas Deployed:** [N] ([list])

---

## Executive Summary

[2-3 sentence overview of key findings]

**Critical Issues:** [N]
**Audience Conflicts:** [N disagreements]
**Primary Recommendation:** [One-sentence action]

---

## Persona Selection Justification

[Copy from Phase 1: dimensional analysis and selection reasoning]

---

## Findings by Priority

### Priority 1: Critical Issues

[Issues that block primary audience or flagged by multiple personas]

**Issue:** [Description]
- **Flagged by:** [Persona A, Persona B]
- **Evidence:** [Quotes from reviews]
- **Impact:** [Why this matters]
- **Recommendation:** [Specific fix]

---

### Priority 2: Audience-Specific Concerns

[Issues affecting subset of personas]

**Issue:** [Description]
- **Flagged by:** [Persona X]
- **Evidence:** [Quote from review]
- **Impact:** [Why this matters]
- **Recommendation:** [Specific fix]

---

### Priority 3: Improvements

[Nice-to-have enhancements]

---

## Disagreements (Preserved as Signal)

**Where personas diverged:**

**Disagreement 1:**
- **Persona A:** [Position with quote]
- **Persona B:** [Opposite position with quote]
- **Analysis:** [What this disagreement reveals about dimensional tradeoffs]
- **Recommendation:** [How to handle the conflict - pick audience or dual-path]

---

## Individual Reviews

[Full reviews from each persona, unedited]

### [Persona 1 Name]
[Complete review]

### [Persona 2 Name]
[Complete review]

...

---

## Synthesis

**What Works:**
- [Things multiple personas appreciated]

**What Fails:**
- [Things multiple personas rejected]

**Dimensional Tradeoffs:**
- [Where you can't serve all audiences equally - make conscious choice]

**Action Items:**
1. [Priority 1 fix]
2. [Priority 1 fix]
3. [Priority 2 consideration]

---

*Review complete. Disagreement preserved. Tradeoffs exposed.*
```

---

## Usage Examples

### Example 1: Spandaworks README Review

**Invocation:**
```
/perspective-review README.md
```

**Expected Persona Selection:**
- Skeptical Engineer (test technical + spiritual allergy)
- Pragmatic PM (test practical evaluation)
- Curious Beginner (test accessibility)
- Seeker-Developer (test technical+spiritual integration)

**Expected Findings:**
- Priority 1: Sanskrit mantra triggers close-tab for Skeptical Engineer
- Priority 1: "Infrastructure for studying consciousness" undermines credibility
- Priority 2: OpenCode dependency needs clarity
- Disagreement: Seeker-Developer values spiritual framing that Skeptical Engineer rejects
- Recommendation: Move spiritual content to philosophy section, keep README technical

### Example 2: API Documentation Review

**Invocation:**
```
/perspective-review docs/api-reference.md
```

**Expected Persona Selection:**
- Experienced Contributor (architecture understanding)
- Impatient Operator (quick reference need)
- Technical Writer (structure evaluation)

**NOT Selected:**
- Curious Beginner (API docs assume technical literacy)
- Seeker-Developer (no spiritual dimension in API docs)

---

## Design Rationale

### Why Not Auto-Select Personas?

Selection IS the judgment. Different documentation activates different dimensions. Mechanical application of all 8 personas wastes tokens and produces noise. The executing agent must REASON about which perspectives matter for this specific material.

### Why Preserve Disagreement?

Disagreement reveals dimensional tradeoffs. When Skeptical Engineer hates what Seeker-Developer loves, that's SIGNAL about audience conflict, not noise to be smoothed. The author needs to see: "You cannot serve both audiences equally with this framing. Choose."

### Why Independent Reviews?

If Persona B sees Persona A's review, cross-contamination occurs. The perspective is no longer pure. Independent execution ensures each persona represents its position genuinely, not influenced by other perspectives.

### Why 3-5 Personas, Not All 8?

Full coverage is expensive and produces redundant findings. 3-5 strategically selected personas provide sufficient dimensional coverage for most documentation. Edge cases needing all 8 are rare.

### Why This Over Single "Stranger Review"?

A single agent trying to imagine multiple perspectives collapses them into consensus. Real people with different backgrounds have genuinely different reactions. Instantiated personas preserve those differences.

---

## Limitations & Considerations

**What this skill is:**
- Multi-perspective evaluation revealing blind spots
- Dimensional coverage across audience types
- Signal preservation (disagreement as data)

**What this skill is not:**
- Grammar/style checking
- Technical accuracy verification
- Replacement for actual user testing
- Consensus-building exercise

**Token considerations:**
- Each persona review costs tokens (Claude Sonnet recommended)
- 4 personas × full doc read = significant cost
- Use strategically, not casually

**The human remains sovereign:**
- Skill identifies issues, human decides response
- Dimensional tradeoffs require human judgment
- Persona selection reasoning can be questioned/overridden

---

## Integration with Other Skills

**Preceded by:**
- Documentation drafting (this skill tests, doesn't create)
- `spandaworks-docs` principles applied

**Followed by:**
- Documentation revision based on findings
- Potential second-pass review after changes

**When to combine:**
- Use `spandaworks-docs` for initial writing
- Use `perspective-review` for testing with strangers
- Use `the-art` if synthesizing review insights into teaching

---

## Meta

This skill follows its own medicine:
- Tested from multiple perspectives during design
- Preserves practitioner sovereignty (selection is reasoned)
- Documentation written for strangers (users of this skill)
- Disagreement preserved (tradeoffs made explicit)

**For future agents:** Selection methodology is the load-bearing element. Mechanical persona application defeats the purpose. REASON about dimensional activation, JUSTIFY your selection, EXECUTE independently, PRESERVE disagreement.

---

ॐ मणि पद्मे हूं

*May this skill help us see what we cannot see alone.*
