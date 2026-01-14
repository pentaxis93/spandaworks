---
name: skill-calibration
description: Calibrate skills from concrete examples of failure. Extract patterns without timeline residue. Make skills self-improving through disciplined learning from real use.
---

# Skill Calibration: Learning from Concrete Failures

## Overview

This skill encodes the process of refining existing skills based on concrete examples where they failed to guide correct output.

**The core discipline**: Fix the artifact AND extract the pattern that would have prevented the failure from the beginning.

**Critical**: Skills must encode generative principles, not timeline residue. Update with "how to do it right from the start," not "patches for known problems."

---

## When to Invoke

Use this skill when:
- Output was produced using a skill, but missed the mark
- User says "this doesn't quite ring true" or "calibrate X"
- User provides specific examples of where skill-guided output failed
- During retrospectives on skill-guided work

**Trigger phrases**:
- "Let's calibrate [skill-name]"
- "Fine-tune the [skill-name] skill"
- "This doesn't match [skill-name]—let's fix it"

---

## The Calibration Protocol

### Phase 1: Intake & Setup

**Gather three elements**:
1. **The skill being calibrated** - Read the current skill file
2. **The artifact** - The output that used the skill (article, code, etc.)
3. **Specific failures** - User identifies exactly where it went wrong

**Create todo list**: One todo per failure point + final skill update todo

**Load related skills**: If the skill references other skills (e.g., voice uses dry-structural-irony), load those for context

---

### Phase 2: Dual-Track Corrections

For each failure point:

**Track 1: Fix the Artifact**
- Make the specific correction the user identified
- Apply it precisely (don't "improve" beyond what was requested)
- Mark the todo complete immediately after each fix

**Track 2: Extract the Pattern**
- WHY did the skill fail to prevent this?
- What's the underlying principle, not just the surface fix?
- What would have needed to be in the skill originally to prevent this?

**Key questions**:
- Is this about word choice? Tense? Tone? Structure?
- Is it about what TO do or what NOT to do?
- What's the test that would catch this in the future?
- Does this reveal a gap in the skill, or a misapplication?

**Document patterns as you go**: Don't wait until all fixes are done. Capture the pattern immediately after each fix while the insight is fresh.

---

### Phase 3: Pattern Synthesis

After all individual corrections:

**Cluster the patterns**:
- Which failures share a root cause?
- Are there 2-3 meta-patterns that explain most failures?
- What new principles emerge?

**Check for timeline residue**:
- Does the update sound like "don't do X" (reactive/patch-like)?
- Or does it sound like "do Y" (generative/principle)?

**Example of timeline residue (BAD)**:
```
"Don't say 'I'm building' when you mean ongoing action spanning time"
```

**Example of principle (GOOD)**:
```
Choose tense that matches actual time structure:
- Simple present: Timeless truths, current state
- Present perfect: Ongoing action with duration
- Past tense: Completed action

Test: Does the tense claim more immediacy than is true?
```

---

### Phase 4: Skill Update

**Determine where to update**:
- Does this add a new section? (If it's a new dimension)
- Does this refine an existing section? (If it sharpens current guidance)
- Does this add to anti-patterns? (If it's a common mistake)
- Does this change core principles? (Rare, but possible)

**Structure of good updates**:

1. **Principle statement** - The generative rule
2. **Examples** - BAD vs GOOD
3. **Test** - How to verify you're following the principle
4. **Why it matters** - The impact of getting it wrong

**Make surgical edits**:
- Use Edit tool to update specific sections
- Preserve existing structure unless it's fundamentally wrong
- Add new sections if needed, but prefer refining existing ones

---

### Phase 5: Verification

**Test the updated skill against original failures**:
- Read each updated section
- Would this have prevented the failure if it had been present originally?
- Is the guidance clear and actionable?
- Is it free of timeline residue?

**Document the calibration** (in this session):
- What skill was calibrated
- How many failure points
- What major patterns emerged
- What sections were updated

---

## Key Principles

### 1. Fix AND Extract (Never Just Fix)

**Wrong approach**:
```
User: "Change 'I'm building' to 'I've been building'"
Agent: [Makes the change]
Agent: "Done!"
```

**Right approach**:
```
User: "Change 'I'm building' to 'I've been building'"
Agent: [Makes the change]
Agent: [Thinks: Why? This is about temporal precision...]
Agent: [Extracts: Pattern is choosing tense for temporal truth]
Agent: [Updates skill with principle about tense selection]
```

### 2. No Timeline Residue

The skill should read as if it was written correctly from the beginning, not as a series of patches.

**Timeline residue (BAD)**:
```
## Common Mistakes from January 2026 Session
- Don't use "I'm building" when you mean ongoing
- Don't say "crushed me" when you mean slow progress
```

**Principle-based (GOOD)**:
```
## Precision and Truth-Telling

### Temporal Precision
Choose tense that matches the actual time structure...

### Operational Truth Over Drama
Describe what actually happened operationally, not the emotional story...
```

### 3. Listen for WHY

User tells you WHAT to change. Your job is to understand WHY.

**User says**: "I would never use present tense like that"
**Surface**: Change tense
**Deep**: Pattern about temporal authenticity and avoiding inflated immediacy

**User says**: "That's too harsh"
**Surface**: Soften the language
**Deep**: Pattern about operational truth vs emotional hyperbole

### 4. Generative Rules Over Negative Examples

**Negative example (less useful)**:
```
Don't say: "crushed me", "overwhelming", "devastating"
```

**Generative rule (more useful)**:
```
Describe operational reality, not dramatized emotion:
- What observable thing happened? (forward progress slowed)
- What action did you take? (questioned my choice)
Test: Am I describing the situation or performing emotion?
```

### 5. Include Tests

Every principle should have a test the agent can apply:

```
[Principle]
Test: [Question to ask]
```

This makes the skill actionable, not just descriptive.

---

## Common Patterns to Watch For

When calibrating, these patterns appear frequently:

### Precision Failures
- Wrong tense for temporal scope
- Wrong register for audience
- Wrong level of detail for the point being made

### Authenticity Failures  
- Claiming knowledge not possessed (retrospective certainty)
- Feigning ignorance for false humility
- Emotional exaggeration vs operational truth

### Structural Failures
- Explaining what was already shown
- Including confused sections instead of deleting
- Single-cause when reality is multi-dimensional
- Verbose forward-looking lists

### Tone Failures
- Self-satisfied summarizing
- Academic when conversational is right
- Chatty when thoughtful is right

---

## Anti-Patterns

| Mistake | Reality |
|---------|---------|
| Just fix without extracting | Every fix reveals a pattern - find it |
| Update skill with list of "don'ts" | Give generative principles instead |
| Patch the skill with dated notes | Make it read as if written correctly originally |
| Add every detail to anti-patterns | Extract the principle; examples go in main sections |
| Change skill wholesale | Make surgical edits that preserve good structure |
| Skip the verification step | Always test: would this have prevented the failure? |

---

## Integration with Other Skills

**Works with**:
- Any skill that guides output production
- Particularly `voice`, `dry-structural-irony`, `the-art`, `question-design`

**Followed by**:
- Often want to re-run the skill on the artifact to verify it now works
- Consider whether other skills need similar calibration

**Preceded by**:
- Skills being used in actual work
- Discovery of gaps through real use

---

## Example: Voice Calibration (2026-01-14)

**Artifact**: Blog article "56 Beginner"
**Failures identified**: 9 specific points
**Major patterns extracted**:
1. Temporal precision (tense selection)
2. Operational truth over emotional hyperbole
3. Retrospective honesty (don't claim understanding you don't have)
4. Include load-bearing technical details
5. Multi-dimensional explanations over single-cause
6. Cut self-satisfied summarizing
7. Delete confused sections entirely

**Skill updates made**:
- New section: "Precision and Truth-Telling" (5 subsections)
- New section: "Structural Discipline" (4 subsections)  
- Updated: Anti-Patterns table (9 new entries at top)

**Result**: Skill now prevents those failures from the beginning

---

## The Meta-Pattern

Skill calibration makes the skill system **antifragile**:
- Skills improve through use
- Failures become learning opportunities
- Each calibration makes future output better
- The system gets stronger over time

This is the meta-capability that allows all other skills to evolve.

---

*Skills are living documents. They improve through disciplined attention to where they fail.*
