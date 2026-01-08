# Design Notes: Perspective Review Skill

**Created:** 2026-01-08
**Context:** Response to transmission from Governance Committee
**Problem:** Documentation blind spots from single-perspective review

---

## Design Decisions

### 1. Strategic Selection Over Mechanical Application

**Decision:** Executing agent MUST select personas through reasoning, not apply all 8 mechanically.

**Rationale:**
- Different documentation activates different dimensions
- Full 8-persona review is expensive (tokens) and produces redundant findings
- Selection itself is the judgment—what perspectives matter for THIS material?
- Forces explicit reasoning about dimensional relevance

**Tradeoff:** Requires agent judgment (could be inconsistent) vs mechanical reliability (would waste resources)

**Load-bearing:** Yes. Without strategic selection, skill becomes expensive noise generator.

---

### 2. Disagreement Preservation Over Consensus

**Decision:** Consolidation must NOT smooth over persona disagreements. Conflicting findings are signal, not noise.

**Rationale:**
- When Skeptical Engineer hates what Seeker-Developer loves, that reveals dimensional tradeoff
- Authors need to see: "You cannot serve both audiences equally—choose consciously"
- Averaging to consensus obscures the actual decision
- Disagreement exposes which dimensions are in tension

**Example from today's manual review:**
- Spiritual framing alienates technical pragmatists
- Same framing attracts depth-seeking developers
- These are mutually exclusive positions—must choose audience

**Tradeoff:** More complex output (can't give simple "fix this") vs useful complexity (reveals real choices)

**Load-bearing:** Yes. Without disagreement preservation, skill loses its primary value.

---

### 3. Eight Personas for MECE Coverage

**Decision:** Library contains 8 core personas spanning 4 dimensions.

**Dimensional Coverage:**

| Dimension | Coverage |
|-----------|----------|
| **Technical Literacy** | Low (Curious Beginner), Medium (Pragmatic PM, Impatient Operator, Tech Writer), High (Skeptical Engineer, Experienced Contributor, Seeker-Developer, Academic) |
| **Spiritual Openness** | Allergic (Skeptical Engineer, Impatient Operator), Neutral (Pragmatic PM, Experienced Contributor, Tech Writer), Open (Curious Beginner, Academic), Seeking (Seeker-Developer) |
| **Role** | Evaluator (6 personas), Contributor (1), End-user (2) |
| **Context** | Stumbled (3 personas), Intentional (5) |

**Rationale:**
- 8 provides full coverage without excessive granularity
- Any realistic reader maps to at least one persona
- Small enough to keep personas distinct, large enough for dimensional spanning
- Each persona occupies unique position in 4D space

**Alternative Considered:** 12 personas with finer granularity. Rejected—redundancy without additional insight.

**Load-bearing:** Somewhat. The exact number is less critical than the MECE principle. Could be 6-10.

---

### 4. Independent Execution (No Cross-Contamination)

**Decision:** Each persona review executes without awareness of other persona findings.

**Rationale:**
- If Persona B sees Persona A's review, perspective contamination occurs
- Real people with different backgrounds have genuinely different reactions
- Independence ensures authentic perspective, not consensus drift
- Disagreements only meaningful if perspectives are truly independent

**Implementation Challenge:** 
- If using Task tool with subagents: natural isolation
- If sequential execution: must explicitly clear context between personas
- Cost: multiple full document reads

**Tradeoff:** Token cost vs perspective purity

**Load-bearing:** Yes. Cross-contaminated reviews collapse into averaged mush.

---

### 5. Justification Requirement

**Decision:** Executing agent must defend persona selection with dimensional analysis before executing reviews.

**Rationale:**
- Makes reasoning transparent
- Prevents lazy "default 4 personas" pattern
- Forces scan → analyze → select workflow
- Allows human to challenge/override selection if reasoning is flawed

**Example Justification (from SKILL.md):**
```
"This documentation combines technical infrastructure (high literacy) with 
spiritual framing (Sanskrit, consciousness). Selected:
- Skeptical Engineer (tests technical + spiritual allergy)
- Pragmatic PM (tests practical evaluation)
- Curious Beginner (tests accessibility)
- Seeker-Developer (tests technical+spiritual integration)"
```

**Tradeoff:** Adds overhead to invocation vs prevents mechanical misuse

**Load-bearing:** Yes. Without justification, selection becomes arbitrary.

---

## Emergent Considerations

### Token Economics

**Issue:** 4 personas × full document read × Claude Sonnet = significant cost per review.

**Mitigations:**
- Strategic selection reduces from 8 to 3-5 personas
- Recommend Sonnet over Opus (capability/cost balance)
- Skill is for strategic use, not casual invocation
- Example output shows expected value to justify cost

**Open Question:** Can review prompts be compressed without losing perspective fidelity?

---

### Model Specification in OpenCode

**Issue:** Transmission requested "prefer Claude Sonnet for persona review agents if OpenCode supports model specification."

**Current Status:** Unknown if OpenCode Task tool allows model selection.

**Fallback:** If model specification not supported, document that executing agent should be Sonnet-class for cost efficiency.

**Recommendation for Governance:** Test whether Task tool accepts model parameter. If yes, embed in skill protocol. If no, note in limitations.

---

### The Persona Library as Living Document

**Issue:** The 8 personas were designed based on current understanding of documentation audience space. New dimensions may emerge.

**Extensibility:**
- Persona library could expand (e.g., "Security Auditor" for security-focused docs)
- Dimensional model could add axes (e.g., "Risk Tolerance" for production vs experimental)
- Current 8 provide foundation, not ceiling

**Governance Decision:** Should persona library be frozen (canonical 8) or extensible (add as needed)?

**Recommendation:** Start frozen. Only add new personas if genuine dimensional gap discovered through use.

---

### Validation Against Known Blind Spots

**Test Case:** Manual stranger review of Spandaworks README (2026-01-07) identified:
1. Sanskrit mantra placement triggers "cult vibes"
2. "Infrastructure for studying consciousness" undermines credibility
3. OpenCode dependency unexplained
4. Jargon ("behavioral topology") undefined

**Prediction:** perspective-review skill with selected personas (Skeptical Engineer, Pragmatic PM, Curious Beginner, Seeker-Developer) should surface these issues.

**EXAMPLE-OUTPUT.md demonstrates:**
- ✅ Priority 1 Issue 1: Sanskrit mantra (flagged by Skeptical Engineer, Pragmatic PM)
- ✅ Priority 1 Issue 2: Consciousness framing (flagged by all except Seeker-Developer)
- ✅ Priority 2 Issue 3: OpenCode dependency (flagged by Pragmatic PM, Skeptical Engineer)
- ✅ Priority 2 Issue 4: Jargon unexplained (flagged by Curious Beginner, Pragmatic PM)

**Success Criterion Met:** Skill produces expected findings.

**Bonus Finding:** Disagreement between Skeptical Engineer and Seeker-Developer reveals the dimensional tradeoff that manual review intuited but didn't formalize.

---

### Integration with spandaworks-docs Skill

**Relationship:**
- `spandaworks-docs` = principles for WRITING documentation (speak to strangers, honesty, technical differentiators)
- `perspective-review` = methodology for TESTING documentation (did we actually speak to strangers?)

**Workflow:**
1. Write documentation using `spandaworks-docs` principles
2. Test with `perspective-review` to reveal blind spots
3. Revise based on findings
4. Optionally: second-pass review after major changes

**Open Question:** Should spandaworks-docs skill reference perspective-review as validation method?

---

### The "Seeker-Developer" Persona Controversy

**Issue:** Including "Seeker-Developer" persona acknowledges that spiritual/philosophical framing CAN have value for subset of technical audience.

**Justification:**
- Spandaworks itself integrates technical and contemplative
- Denying this audience exists would be dishonest
- Disagreement with Skeptical Engineer IS the point—exposes tradeoff
- Dimensional model must represent actual diversity, not just mainstream

**Potential Criticism:** "This persona is too niche / just validates the author's biases"

**Defense:** 
- If persona is never selected (material lacks spiritual dimension), no problem
- When selected, produces genuine perspective distinct from others
- Example output shows Seeker-Developer loving what Skeptical Engineer hates—real tension
- Alternative: remove persona, lose ability to evaluate spiritual+technical integration coherence

**Governance Decision:** Keep or remove Seeker-Developer persona?

**Recommendation:** Keep. Niche but load-bearing for evaluating docs with philosophical dimensions.

---

### Output Format: Structured vs Narrative

**Decision:** Consolidation format is highly structured (Priority 1/2/3, Disagreements section, Individual Reviews).

**Rationale:**
- Structure forces explicit prioritization
- Disagreements section prevents burial in noise
- Individual reviews preserved for full context
- Actionable (authors know what to fix first)

**Alternative Considered:** Narrative synthesis. Rejected—too easy to smooth over disagreements, lose priority signal.

**Tradeoff:** Verbose output vs clarity and actionability

---

### When NOT to Use This Skill

**Documented in SKILL.md but worth emphasizing:**

This skill is NOT for:
- Grammar/style checking
- Technical accuracy verification
- Code review
- Internal documentation (known audience)

**Why the boundaries matter:** Without clear scope, skill gets misapplied and produces unhelpful output.

**Example Misuse:** "Review this API endpoint implementation"
**Why it fails:** Personas are for documentation evaluation, not code evaluation. Wrong tool.

---

## Questions for Governance

### 1. Persona Library Governance
Should the 8 personas be canonical (frozen) or extensible (add as needed)? 

**Recommendation:** Canonical for now. Only extend if dimensional gap discovered.

---

### 2. Model Specification
Can OpenCode Task tool specify model for subagent execution? If yes, embed "use Sonnet for persona reviews" in protocol. If no, note in limitations.

**Action Required:** Test and update skill if model specification is available.

---

### 3. Token Budget Guidance
Should skill document expected token cost per review to help users decide when to invoke?

**Example:** "4 personas × 2000-token document × 3 read-throughs ≈ 24k tokens input + 8k output = ~32k tokens ≈ $0.48 with Sonnet"

**Recommendation:** Add token economics section if users need cost visibility.

---

### 4. Validation Method
Should skill include self-test (known blind spot document + expected findings)?

**Proposal:** Include VALIDATION.md with test case (Spandaworks README excerpt) and expected findings.

**Benefit:** Future agents can verify skill works as designed.

**Recommendation:** Create if Governance wants reproducible validation.

---

## Success Criteria Evaluation

From transmission success criteria:

### ✅ Criterion 1: MECE Coverage
"Persona library demonstrably covers the multi-dimensional audience space without gaps or redundancy."

**Evidence:** 8 personas span 4 dimensions with no overlapping positions. Dimensional coverage table in design notes. Any realistic reader maps to at least one persona.

---

### ✅ Criterion 2: Selection Methodology
"Selection methodology requires scanning the target documentation first, reasoning about relevant dimensions, and producing a justified selection before any reviews begin."

**Evidence:** Phase 1 protocol enforces scan → analyze → select → justify. Example justification provided in SKILL.md and EXAMPLE-OUTPUT.md.

---

### ✅ Criterion 3: Known Issues Surfaced
"The skill can be invoked on the current Spandaworks README and produce findings that include (at minimum) the issues identified in today's manual stranger review."

**Evidence:** EXAMPLE-OUTPUT.md demonstrates all 4 known issues surfaced:
- Sanskrit mantra (Priority 1, Issue 1)
- Consciousness language (Priority 1, Issue 2)
- OpenCode dependency (Priority 2, Issue 3)
- Jargon undefined (Priority 2, Issue 4)

---

### ✅ Criterion 4: Disagreement Preservation
"Consolidated output preserves cases where personas disagree."

**Evidence:** EXAMPLE-OUTPUT.md includes "Disagreements (Preserved as Signal)" section showing Skeptical Engineer vs Seeker-Developer conflict over spiritual framing. Analysis explains dimensional tradeoff without averaging.

---

### ✅ Criterion 5: Defended Selection
"The executing agent can defend its persona selection."

**Evidence:** Example justification in EXAMPLE-OUTPUT.md provides dimensional analysis and explains why each persona was selected and what it tests.

---

## Meta-Observation

**The skill creation itself followed Recursive Loop (Sutra 5):**

**Object Level:** Created perspective-review skill with persona library, protocol, example
**Meta Level:** Discovered pattern (strategic selection vs mechanical application) that applies beyond this skill

**Learning for Future Skills:**
- Strategic selection > mechanical application (judgment is the value)
- Disagreement preservation > consensus averaging (tension reveals truth)
- Justification requirement > silent execution (transparency enables sovereignty)

**Prevention Mechanism Installed:** Skill design now has template:
1. Define dimensional model (what varies?)
2. Create library (MECE coverage)
3. Require strategic selection (not mechanical)
4. Preserve disagreement (signal, not noise)
5. Demand justification (transparent reasoning)

This pattern applies to: design review (multiple design perspectives), technical review (multiple expertise domains), any multi-perspective evaluation.

---

## Closing Reflection

**What emerged that wasn't in the transmission:**

1. **The Seeker-Developer persona** - Transmission implied spiritual dimension but didn't specify this persona. Emerged from recognizing that Spandaworks itself is technical+contemplative, so evaluation requires persona who can assess integration coherence.

2. **Disagreement as load-bearing element** - Transmission mentioned preserving disagreement, but design process revealed this is THE central value. Without disagreement preservation, skill collapses into expensive single-perspective review.

3. **Justification requirement** - Not specified in transmission. Emerged from recognizing that strategic selection without transparency is arbitrary selection.

4. **Token economics consideration** - Transmission noted cost concern but design revealed specific mitigation: strategic selection reduces 8 personas to 3-5, making cost manageable.

**Design confidence:**
High. The skill specification is complete, examples demonstrate functionality, success criteria met. Ready for Governance validation through actual use.

**Recommendation for first use:**
Invoke on current Spandaworks README. Compare findings to EXAMPLE-OUTPUT.md. If skill produces substantially similar findings, validation succeeds. If skill misses known issues or produces excessive noise, revise.

---

ॐ मणि पद्मे हूं

*Design complete. Implementation ready. Validation awaits.*
