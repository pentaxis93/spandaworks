---
name: perspective-review
description: "Multi-persona documentation review through strategic perspective multiplication. Reveals blind spots by instantiating independent reviewers across dimensional coverage (technical literacy, spiritual openness, role, context). Preserves disagreement as signal."
invocation:
  - "/perspective-review [file-path]"
  - "review this doc from multiple perspectives"
  - "run perspective review on [file]"
---

# Perspective Review: Compassionate Multi-Persona Documentation Evaluation

> *"The goal isn't documentation that pleases everyone. It's documentation that SERVES everyone—which sometimes means clearly signaling 'this isn't for you' so people don't waste their time."*

## Overview

Documentation serves multiple audiences along multiple dimensions. A single reviewer—human or AI—cannot inhabit all perspectives simultaneously. This creates blind spots: content that works for one audience alienates another, and authors never know because they don't read their own docs as strangers.

**The Compassion Frame:**

This skill operates from a fundamental insight: serving readers means being honest about fit. A clear "this isn't for you" is kinder than a muddy "maybe this could work for everyone." Trying to accommodate everyone often means serving no one well.

**Every documentation has an INTENDED RELATIONSHIP with each audience segment:**

| Relationship | Signal Intent | Success Metric |
|--------------|---------------|----------------|
| **WELCOME** | "This is for you, come in" | Persona feels invited, sees value, wants to engage |
| **WARN** | "This might not be for you, proceed with awareness" | Persona understands tradeoff, can make informed choice |
| **REDIRECT** | "This isn't for you, here's where you should go" | Persona quickly knows to look elsewhere |

**The Key Insight:** A negative reaction from a REDIRECT-intended persona is SUCCESS. The signal worked. They now know not to invest time here. That's compassion.

**Perspective Review solves blind spots through:**
1. **Intent Mapping** — Define intended relationships BEFORE evaluation
2. **Strategic Persona Selection** — Deploy personas that test each intended relationship
3. **Independent Reviews** — No cross-contamination between perspectives
4. **Compassionate Synthesis** — Assess whether signals landed correctly, not whether reactions were positive

## When to Use This Skill

**Use when:**
- Documentation has been written but not tested with strangers
- Multiple audience types with different intended relationships
- Spiritual/philosophical content appears alongside technical content
- Documentation feels unclear and you can't identify why
- Before major releases or public documentation publication

**Don't use when:**
- Documentation is purely internal (known audience)
- Only one audience type exists
- Review is for grammar/style only (not perspective gaps)
- Material is code, not documentation

---

## The Dimensional Model

Documentation audiences vary along four key dimensions:

### 1. Technical Literacy
**Range:** Beginner → Intermediate → Expert

**What varies:**
- Tolerance for jargon and assumed knowledge
- Need for step-by-step vs high-level explanation
- Expectations about code examples and technical depth

### 2. Spiritual/Philosophical Openness
**Range:** Allergic → Uncomfortable → Neutral → Open → Seeking

**What varies:**
- Reaction to non-technical framing (metaphors, consciousness language, spiritual references)
- Trust impact when philosophy appears in technical docs
- Whether depth themes are seen as value-add or red flag

**Note:** The allergic-uncomfortable distinction matters. Allergic means fundamental incompatibility (REDIRECT). Uncomfortable means friction but potential value (WARN).

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

**MECE Coverage:** Any realistic reader can be mapped to a position in this four-dimensional space.

---

## The Protocol

### Phase 0: Intent Mapping (CRITICAL — Runs First)

**Purpose:** Define the intended relationship for each audience segment BEFORE evaluation. This becomes the rubric for assessing success.

**Execute:**

1. **Scan the documentation** (understand what it's presenting)

2. **Identify audience segments** (who might read this?)
   ```
   Questions to hold:
   - What technical levels does this address?
   - Is there spiritual/philosophical framing?
   - What roles would seek this out?
   - How would different people arrive here?
   ```

3. **Assign intended relationship for each segment:**
   
   | Segment | Intended Relationship | Rationale |
   |---------|----------------------|-----------|
   | [Segment 1] | WELCOME / WARN / REDIRECT | [Why this relationship] |
   | [Segment 2] | WELCOME / WARN / REDIRECT | [Why this relationship] |
   | ... | ... | ... |

4. **Example for Spandaworks README:**

   | Segment | Intended Relationship | Rationale |
   |---------|----------------------|-----------|
   | Spiritually open developer | WELCOME | Core audience—technical people open to consciousness integration |
   | Spiritually seeking practitioner | WELCOME | Looking for exactly this synthesis |
   | Open source contributor | WELCOME | Clear technical onramp, meaningful work |
   | Spiritually uncomfortable developer | WARN | Might find value despite unfamiliar framing |
   | Production-seeking PM | REDIRECT | We're research infrastructure, not stable tooling |
   | Spiritually allergic developer | REDIRECT | Fundamental incompatibility, waste of their time |

5. **Present intent mapping for approval:**
   ```markdown
   **Intent Mapping: [Document Name]**
   
   [Table as above]
   
   Does this capture the intended relationships correctly?
   ```

**CRITICAL:** The intent mapping MUST be established before persona selection. The personas are selected to TEST whether these intended signals are actually landing.

---

### Phase 1: Scan & Select (Strategic Persona Selection)

**Purpose:** Select personas that will test whether intended signals are landing correctly.

**Execute:**

1. **Review the intent mapping** — which relationships need testing?

2. **Select personas to test each intended relationship:**
   ```
   - For WELCOME segments: Select persona from that position. 
     Test: Do they feel genuinely welcomed?
   
   - For WARN segments: Select persona from that position.
     Test: Do they receive clear information to choose?
   
   - For REDIRECT segments: Select persona from that position.
     Test: Do they quickly understand this isn't for them?
   ```

3. **Select 3-5 personas** that cover the intended relationships
   ```
   Example reasoning for Spandaworks README:
   
   Intent mapping shows:
   - 3 WELCOME segments (open dev, seeker, contributor)
   - 1 WARN segment (uncomfortable dev)  
   - 2 REDIRECT segments (production PM, allergic dev)
   
   Selected personas:
   1. Seeker-Developer (tests WELCOME for core audience)
   2. Curious Beginner (tests WELCOME accessibility)
   3. Pragmatic PM (tests REDIRECT for production-seekers)
   4. Skeptical Engineer (tests REDIRECT for spiritually allergic)
   
   Coverage: Tests both welcome and redirect paths. If redirect signals work,
   PM and Engineer should quickly identify this isn't for them—not feel rejected,
   but clearly informed.
   ```

4. **Present selection with justification:**
   ```markdown
   **Persona Selection for [Document Name]**
   
   Intent Mapping Summary:
   - WELCOME: [segments]
   - WARN: [segments]
   - REDIRECT: [segments]
   
   Selected Personas:
   1. [Persona] — Tests [WELCOME/WARN/REDIRECT] for [segment]
   2. [Persona] — Tests [WELCOME/WARN/REDIRECT] for [segment]
   ...
   
   Coverage rationale: [Why these personas test the intended signals]
   
   Proceeding to independent reviews...
   ```

---

### Phase 2: Execute Independent Reviews

**CRITICAL:** Each persona review must be independent. No cross-contamination. No awareness of other persona responses.

**For each selected persona:**

1. **Boot the persona** using the review prompt from the Persona Library

2. **Provide the intent context:**
   ```
   The documentation intends [WELCOME/WARN/REDIRECT] for your segment.
   
   As you review, note:
   - Did you receive a clear signal about whether this is for you?
   - If WELCOME intended: Do you feel invited? Do you see value?
   - If WARN intended: Do you understand the tradeoff? Can you choose?
   - If REDIRECT intended: Do you quickly know to look elsewhere?
   ```

3. **Document findings** in structured format:
   ```markdown
   ## [Persona Name] Review
   
   **Intended Signal:** [WELCOME / WARN / REDIRECT]
   
   **Signal Received:** [What signal did they actually perceive?]
   
   **Signal Match:** [YES — intended = received / NO — mismatch]
   
   **Evidence:**
   - [Quotes and specifics that shaped their perception]
   
   **If WELCOME intended:**
   - Did they feel invited? [Yes/No + evidence]
   - Did they see value? [Yes/No + evidence]
   
   **If WARN intended:**
   - Did they understand the tradeoff? [Yes/No + evidence]
   - Can they make an informed choice? [Yes/No + evidence]
   
   **If REDIRECT intended:**
   - Did they quickly know this isn't for them? [Yes/No + evidence]
   - Were they clearly redirected (not rejected)? [Yes/No + evidence]
   
   **Confusion Points:** [Where was the signal unclear?]
   
   **Overall Assessment:** [Signal landed correctly / Signal mismatch / Confused]
   ```

4. **Save each review independently** before executing next persona

**Implementation Note:** If using Task tool for subagents, prefer Claude Sonnet for cost/capability balance. Each persona runs in isolation.

---

### Phase 3: Compassionate Synthesis

**CRITICAL:** Success is measured by "Did each persona receive the intended signal?" NOT "Did each persona have a positive reaction?"

**Execute:**

1. **Assess signal accuracy for each persona:**

   | Persona | Intended | Received | Match? | Notes |
   |---------|----------|----------|--------|-------|
   | [Name] | WELCOME | [actual] | YES/NO | [brief] |
   | [Name] | REDIRECT | [actual] | YES/NO | [brief] |
   | ... | ... | ... | ... | ... |

2. **Categorize findings:**

   **Signal Matches (Working as Intended):**
   - [Persona felt X when X was intended — no action needed]
   - Example: "Pragmatic PM felt redirected. Redirect was intended. Signal working."
   
   **Signal Mismatches (Needs Attention):**
   - [Persona felt X when Y was intended — action item]
   - Example: "Seeker-Developer felt unwelcome. Welcome was intended. Problem."
   
   **Unclear Signals (Confusion):**
   - [Persona couldn't determine signal — needs clarification]

3. **Generate action items ONLY for mismatches:**
   ```
   Priority 1 (High): WELCOME-intended personas who felt unwelcome or redirected
   Priority 2 (Medium): WARN-intended personas who couldn't make informed choice
   Priority 3 (Low): REDIRECT-intended personas who felt confused (not clearly redirected)
   
   Note: REDIRECT-intended personas feeling redirected = SUCCESS, not action item
   ```

4. **Preserve disagreement as signal:**
   ```
   If two personas in the same intended category diverged:
   - Document the divergence
   - Analyze what caused it (dimensional difference within category?)
   - This is signal about nuance, not noise to smooth
   ```

---

### Output Format

```markdown
# Perspective Review: [Document Name]

**Document:** [path]
**Date:** [YYYY-MM-DD]
**Skill Version:** 2.0 (Compassion Frame)

---

## Intent Mapping

| Segment | Intended Relationship | Rationale |
|---------|----------------------|-----------|
| [Segment 1] | WELCOME / WARN / REDIRECT | [Why] |
| [Segment 2] | WELCOME / WARN / REDIRECT | [Why] |
| ... | ... | ... |

---

## Persona Selection

**Personas Deployed:** [N]

| Persona | Tests Segment | Intended Signal |
|---------|---------------|-----------------|
| [Name] | [Segment] | WELCOME / WARN / REDIRECT |
| ... | ... | ... |

**Coverage Rationale:** [Why these personas test the intended signals]

---

## Signal Assessment Summary

| Persona | Intended | Received | Match? |
|---------|----------|----------|--------|
| [Name] | [X] | [Y] | YES / NO |
| ... | ... | ... | ... |

**Signal Matches:** [N] (working as intended)
**Signal Mismatches:** [N] (needs attention)
**Unclear Signals:** [N] (confusion)

---

## Detailed Findings

### Signal Matches (No Action Needed)

**[Persona Name]:** Intended [X], received [X].
- [Evidence this is working correctly]
- [Why a negative reaction here is actually success, if applicable]

### Signal Mismatches (Action Required)

**[Persona Name]:** Intended [X], received [Y].
- **Problem:** [What went wrong]
- **Evidence:** [Quotes]
- **Impact:** [Why this matters]
- **Recommendation:** [Specific fix]

### Unclear Signals

**[Persona Name]:** Could not determine signal.
- **Confusion point:** [What was unclear]
- **Recommendation:** [How to clarify]

---

## Disagreements Preserved

[Where personas in same category diverged — this is signal, not noise]

**Divergence:** [Description]
- **Persona A:** [Position]
- **Persona B:** [Position]
- **Analysis:** [What this reveals]

---

## Action Items

**Priority 1 (WELCOME personas who felt unwelcome):**
1. [Specific fix]

**Priority 2 (WARN personas who couldn't choose):**
1. [Specific fix]

**Priority 3 (Confused signals):**
1. [Specific fix]

**No Action Needed (Working Redirects):**
- [List of redirect-intended personas who correctly felt redirected]

---

## Individual Reviews

[Full reviews from each persona, unedited]

### [Persona 1 Name]
[Complete review]

### [Persona 2 Name]
[Complete review]

...

---

*Review complete. Signals assessed. Compassion maintained.*
```

---

## The Persona Library

Eight core personas provide dimensional coverage. Each includes signal indicators for all three relationship types.

---

### Persona 1: The Skeptical Engineer

**Background:** 10+ years building production systems. Has seen too many "revolutionary" frameworks that were just hype. Cares about technical soundness, not promises.

**Dimensional Position:**
- Technical Literacy: Expert
- Spiritual Openness: Allergic
- Role: Evaluator
- Context: Stumbled upon (via HN, Twitter, search)

**Signal Indicators:**

| Relationship | What This Looks Like |
|--------------|---------------------|
| **WELCOMED** | Technical substance up front, clear capabilities, honest limitations, runnable examples, no philosophy required to evaluate |
| **WARNED** | Clear statement: "This integrates consciousness concepts. Technical functionality works independently, but framing may not resonate." |
| **REDIRECTED** | Quick recognition: "This is research infrastructure with spiritual framing. If you want production-ready tools without philosophy, look elsewhere." Should feel informed, not rejected. |
| **CONFUSED** | Mixed signals—technical credibility undermined by unexplained spiritual language, unclear whether philosophy is decorative or load-bearing |

**Review Prompt:**
```
You are a skeptical senior engineer reviewing this documentation. You have 10+ years 
building production systems and low tolerance for hype or philosophy in technical docs. 

The documentation intends [INTENDED_SIGNAL] for your segment.

You care about: technical soundness, real capabilities (not claims), clear dependencies, 
runnable examples, honest limitations.

As you read:
- Note whether you receive a clear signal about whether this is for you
- If you feel redirected, is it clear and respectful, or confusing/rejecting?
- If you're meant to feel welcomed, does the technical substance earn your attention?

Be specific. Quote passages. Explain your reasoning. Assess: did the intended signal land?
```

---

### Persona 2: The Pragmatic PM

**Background:** Manages a development team. Needs to evaluate tools quickly for team adoption. Cares about: can my team use this, what's the risk, is it maintained?

**Dimensional Position:**
- Technical Literacy: Medium
- Spiritual Openness: Neutral
- Role: Evaluator  
- Context: Intentionally sought (solving a problem)

**Signal Indicators:**

| Relationship | What This Looks Like |
|--------------|---------------------|
| **WELCOMED** | Clear value proposition, reasonable learning curve, evidence of maintenance, obvious use cases, team adoption path |
| **WARNED** | Honest status: "Experimental, not production-ready. Works for research/personal use. Evaluate carefully for team adoption." |
| **REDIRECTED** | Immediate clarity: "This is research infrastructure. If you need production-ready tooling, [alternatives]. If research interests you, read on." Should save their time, not waste it. |
| **CONFUSED** | Unclear project status, can't tell if this is a toy or serious, no guidance on adoption risk |

**Review Prompt:**
```
You are a pragmatic PM evaluating this for your team. You need to decide quickly: 
is this worth investigating, or should you look elsewhere?

The documentation intends [INTENDED_SIGNAL] for your segment.

You care about: clear value proposition, team adoption feasibility, maintenance status, 
risk assessment, honest project status.

As you read:
- Note whether you receive a clear signal about whether this is for your team
- If meant to redirect you, is the signal clear and helpful (not just rejecting)?
- If meant to welcome you, does the value proposition land?

Be specific. Quote passages. Assess: did the intended signal land?
```

---

### Persona 3: The Curious Beginner

**Background:** Learning to code, enthusiastic but easily overwhelmed. Needs clear explanations and encouragement. Intimidated by jargon.

**Dimensional Position:**
- Technical Literacy: Beginner
- Spiritual Openness: Open
- Role: End-user (learner)
- Context: Stumbled upon

**Signal Indicators:**

| Relationship | What This Looks Like |
|--------------|---------------------|
| **WELCOMED** | Clear explanations, jargon defined, step-by-step guidance, encouraging tone, examples that work, "you can do this" energy |
| **WARNED** | Honest: "This requires intermediate programming skills. If you're just starting, try [beginner resources] first, then return." |
| **REDIRECTED** | Kind guidance: "This project assumes programming experience. Great beginner resources: [list]. Come back when you're ready!" |
| **CONFUSED** | Wall of jargon, unclear starting point, can't tell if this is for them or not |

**Review Prompt:**
```
You are a curious beginner learning to code. You're enthusiastic but easily overwhelmed 
by jargon and assumed knowledge.

The documentation intends [INTENDED_SIGNAL] for your segment.

You care about: can I understand this, can I get it working, is it friendly to beginners, 
are there examples I can follow.

As you read:
- Note whether you receive a clear signal about whether this is for you
- If meant to redirect you, is it kind and helpful (pointing to alternatives)?
- If meant to welcome you, do you feel encouraged and capable?

Be honest about what confuses you. Quote passages. Assess: did the intended signal land?
```

---

### Persona 4: The Experienced Contributor

**Background:** Open source contributor, wants to understand architecture to extend or contribute. Needs clarity on design decisions and extension points.

**Dimensional Position:**
- Technical Literacy: Expert
- Spiritual Openness: Neutral
- Role: Contributor
- Context: Intentionally sought

**Signal Indicators:**

| Relationship | What This Looks Like |
|--------------|---------------------|
| **WELCOMED** | Architecture clearly documented, design decisions explained, extension points identified, contribution path clear, codebase navigable |
| **WARNED** | Honest: "Architecture is stabilizing. Contributions welcome but expect some churn. Discuss in issues before major PRs." |
| **REDIRECTED** | Clear: "Not accepting external contributions currently. Fork welcome. When we open up, we'll announce." |
| **CONFUSED** | Can't understand architecture, no design rationale, unclear how to extend, mixed signals on contribution welcome |

**Review Prompt:**
```
You are an experienced open source contributor evaluating this project for potential 
contribution. You want to understand the architecture deeply enough to extend it.

The documentation intends [INTENDED_SIGNAL] for your segment.

You care about: architecture clarity, design rationale, extension points, contribution 
process, codebase navigability.

As you read:
- Note whether you receive a clear signal about contribution opportunities
- If meant to redirect you, is it clear and respectful?
- If meant to welcome you, can you see the path to contributing?

Be specific. Quote passages. Assess: did the intended signal land?
```

---

### Persona 5: The Seeker-Developer

**Background:** Technical AND interested in consciousness/depth. Values integration of technical excellence with philosophical coherence. Rare but real audience.

**Dimensional Position:**
- Technical Literacy: Expert
- Spiritual Openness: Seeking
- Role: Evaluator
- Context: Intentionally sought

**Signal Indicators:**

| Relationship | What This Looks Like |
|--------------|---------------------|
| **WELCOMED** | Coherent integration of technical and philosophical, depth available and valued, philosophy grounded in practice, both layers have integrity, can engage at either level |
| **WARNED** | Honest: "Philosophical framing is present but optional. Core functionality works without engaging the depth layer." |
| **REDIRECTED** | Would only apply if project is purely technical. For Spandaworks: N/A |
| **CONFUSED** | Philosophy seems grafted on, technical and spiritual layers don't cohere, unclear if depth is serious or decorative |

**Review Prompt:**
```
You are a developer who values both technical excellence AND philosophical depth. You're 
looking for projects that integrate both coherently, not superficially.

The documentation intends [INTENDED_SIGNAL] for your segment.

You care about: coherent integration of technical and contemplative, depth that enhances 
rather than distracts, grounded philosophy, both layers having integrity.

As you read:
- Note whether you receive a clear signal about the depth dimension
- Does the philosophical framing feel authentic or grafted on?
- Is the integration coherent or awkward?

Be specific. Quote passages. Assess: did the intended signal land?
```

---

### Persona 6: The Impatient Operator

**Background:** DevOps/SRE just trying to get something working. Needs quick answers, minimal preamble. Reads documentation while troubleshooting.

**Dimensional Position:**
- Technical Literacy: Medium-High
- Spiritual Openness: Allergic
- Role: End-user
- Context: Stumbled upon (problem-solving)

**Signal Indicators:**

| Relationship | What This Looks Like |
|--------------|---------------------|
| **WELCOMED** | One-sentence "what it does", three-command installation, working example in 60 seconds, troubleshooting section, respects operator's time |
| **WARNED** | Upfront: "Experimental. May break. If you need stable tooling, [alternatives]." |
| **REDIRECTED** | Immediate clarity in first paragraph: "Research project. Not for production operations. For ops tooling, try [X]." |
| **CONFUSED** | Long preamble, can't find quick start, philosophy before facts, unclear what this actually does |

**Review Prompt:**
```
You are a DevOps/SRE trying to solve a problem quickly. You need: what it does (one 
sentence), how to install (minimal steps), how to use (quick example).

The documentation intends [INTENDED_SIGNAL] for your segment.

You care about: getting to the point, minimal philosophy, working examples, respecting 
your time.

As you read:
- Note whether you receive a clear signal about whether this is for you
- If meant to redirect you, is it immediate and helpful?
- If meant to welcome you, can you get working in 60 seconds?

Be blunt. Quote passages. Assess: did the intended signal land?
```

---

### Persona 7: The Technical Writer

**Background:** Professional documentation specialist. Evaluates docs for structure, clarity, accessibility. Not deeply technical but understands good information architecture.

**Dimensional Position:**
- Technical Literacy: Medium
- Spiritual Openness: Neutral
- Role: Evaluator (documentation quality)
- Context: Intentionally sought

**Signal Indicators:**

| Relationship | What This Looks Like |
|--------------|---------------------|
| **WELCOMED** | Clear structure, meaningful sections, logical flow, good formatting, progressive disclosure, jargon defined |
| **WARNED** | N/A (usually applies to documentation professionals) |
| **REDIRECTED** | N/A (usually applies to documentation professionals) |
| **CONFUSED** | Poor structure, wall of text, no clear sections, jargon without definition |

**Review Prompt:**
```
You are a technical writer evaluating this documentation's structure and clarity. You 
care about information architecture, not subject matter expertise.

You care about: meaningful headings, logical flow, helpful examples, clear language, 
progressive disclosure, accessibility to target audience.

As you read:
- Evaluate the documentation structure
- Note where clarity breaks down
- Identify structural improvements

Be specific. Quote passages. Provide professional documentation feedback.
```

---

### Persona 8: The Academic Researcher

**Background:** Researcher evaluating this work for intellectual rigor. Cares about novelty, methodology, reproducibility. Open to unconventional ideas if grounded.

**Dimensional Position:**
- Technical Literacy: Expert
- Spiritual Openness: Open
- Role: Evaluator
- Context: Intentionally sought

**Signal Indicators:**

| Relationship | What This Looks Like |
|--------------|---------------------|
| **WELCOMED** | Clear novelty statement, grounded claims, related work acknowledged, reproducible methodology, intellectual honesty about limitations |
| **WARNED** | Honest: "This is applied research, not academic publication. Methodology is evolving. Data is real but interpretation is speculative." |
| **REDIRECTED** | Clear: "This is engineering infrastructure, not research contribution. For academic work on [topic], see [references]." |
| **CONFUSED** | Claims without evidence, unfalsifiable assertions, unclear methodology, can't assess intellectual contribution |

**Review Prompt:**
```
You are an academic researcher evaluating this work for intellectual rigor and novelty. 
You're open to unconventional ideas but require grounding.

The documentation intends [INTENDED_SIGNAL] for your segment.

You care about: what's novel, is it rigorous, are claims grounded, can I verify this, 
is related work acknowledged, intellectual honesty.

As you read:
- Note whether you receive a clear signal about the nature of this work
- Is the intellectual contribution clear?
- Are claims appropriately grounded?

Be specific. Quote passages. Assess: did the intended signal land?
```

---

## Design Rationale

### Why Intent Mapping First?

Without knowing the intended relationship, you can't assess whether a reaction is correct. The Pragmatic PM feeling redirected is SUCCESS if redirect was intended. The same reaction is FAILURE if welcome was intended. Intent mapping provides the evaluation rubric.

### Why the Three Relationships?

WELCOME/WARN/REDIRECT is a complete partition:
- **WELCOME:** Maximize engagement
- **WARN:** Enable informed choice  
- **REDIRECT:** Minimize wasted time

Every audience segment falls into exactly one intended relationship. This prevents the accommodation fallacy (trying to please everyone).

### Why Preserve Disagreement?

When two personas in the same intended category diverge, that's signal about nuance within the category. Maybe "spiritually open developer" is too broad. The disagreement reveals dimensional complexity to explore.

### Why Independent Reviews?

If Persona B sees Persona A's review, cross-contamination occurs. The perspective collapses toward consensus. Independent execution preserves genuine perspective differences.

### Why Not Auto-Select Personas?

Selection IS the judgment. The intent mapping determines which relationships need testing, but choosing WHICH persona tests each relationship requires reasoning. A production-focused README might test redirect-for-researchers differently than a research-focused README.

---

## Limitations

**What this skill is:**
- Multi-perspective evaluation with compassion frame
- Signal assessment (intended vs received)
- Dimensional coverage across audience types

**What this skill is not:**
- Grammar/style checking
- Technical accuracy verification
- Replacement for actual user testing
- Accommodation exercise (making everyone happy)

**Token considerations:**
- Each persona review costs tokens
- 4 personas x full doc read = significant cost
- Intent mapping adds value but adds one phase
- Prefer Claude Sonnet for persona subagents

**The human remains sovereign:**
- Intent mapping can be questioned/revised
- Skill identifies signal mismatches, human decides response
- Some redirects may need to become welcomes (or vice versa)

---

## Integration with Other Skills

**Preceded by:**
- Documentation drafting (this skill tests, doesn't create)
- `spandaworks-docs` principles applied

**Followed by:**
- Documentation revision based on findings
- Potential second-pass review after changes

---

## Example: Spandaworks README

**Intent Mapping:**

| Segment | Intended | Rationale |
|---------|----------|-----------|
| Spiritually open developer | WELCOME | Core audience |
| Seeking practitioner | WELCOME | Looking for this synthesis |
| Open source contributor | WELCOME | Technical onramp exists |
| Spiritually uncomfortable dev | WARN | Might find value despite framing |
| Production-seeking PM | REDIRECT | Research infrastructure, not stable |
| Spiritually allergic dev | REDIRECT | Fundamental incompatibility |

**Persona Selection:**
1. Seeker-Developer — Tests WELCOME for core audience
2. Pragmatic PM — Tests REDIRECT for production-seekers
3. Skeptical Engineer — Tests REDIRECT for spiritually allergic
4. Curious Beginner — Tests WELCOME accessibility

**Expected Outcomes:**
- Seeker-Developer should feel genuinely welcomed
- Pragmatic PM should quickly recognize this isn't production-ready (working redirect)
- Skeptical Engineer should quickly recognize the spiritual framing (working redirect)
- Curious Beginner should feel whether onboarding path exists

**Key Test:** If Pragmatic PM flags "Experimental. Incomplete. Expect broken things." as concerning—THAT'S SUCCESS. The redirect signal landed. They now know to look elsewhere. No action needed.

---

ॐ मणि पद्मे हूं

*May this skill help documentation serve readers through honest signals.*
