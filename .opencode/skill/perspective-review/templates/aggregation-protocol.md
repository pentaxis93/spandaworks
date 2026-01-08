# Aggregation Protocol

This document guides the main agent in synthesizing multiple subagent evaluations into a prioritized action plan.

## Input

You have received N completed evaluation templates, each from a different persona. Each evaluation contains:
- Scores (trust, value clarity, structure, completeness, accessibility)
- Lists (red flags, what worked, confusion points, recommendations)
- Binary decisions (would use, would star, would contribute, would recommend)
- Quotes with analysis
- Free-form summary

## Aggregation Steps

### Step 1: Extract Quantitative Data

Create a scoring matrix:

| Persona | Trust | Value | Structure | Complete | Access | Would Use |
|---------|-------|-------|-----------|----------|--------|-----------|
| [Name]  | [1-10]| [1-10]| [1-10]    | [1-10]   | [1-10] | [Y/N/M]   |
| ...     | ...   | ...   | ...       | ...      | ...    | ...       |

Calculate:
- **Mean scores** per dimension
- **Standard deviation** per dimension (high SD = disagreement)
- **Adoption rate** = (Yes + Maybe×0.5) / N

### Step 2: Apply Audience Weighting

If a target audience was specified, apply weights:

```
weighted_score = sum(persona_score × persona_weight) / sum(weights)
```

Default weights (general developer audience):
- skeptical-senior: 1.5
- junior-dev: 1.0
- enterprise-architect: 1.0
- oss-maintainer: 1.0
- drive-by-visitor: 0.5
- non-technical-stakeholder: 0.5
- competitor: 0.3 (diagnostic, not prescriptive)
- accessibility-advocate: 1.0
- security-conscious: 1.0
- impatient-scanner: 0.8

### Step 3: Extract Issues by Frequency

Catalog every issue mentioned across evaluations:

| Issue | Personas Who Mentioned | Count | Weighted Count |
|-------|------------------------|-------|----------------|
| [Issue description] | [List of personas] | N | weighted |
| ... | ... | ... | ... |

**Consensus threshold:** Issues mentioned by 50%+ of personas
**Strong consensus:** Issues mentioned by 70%+ of personas

### Step 4: Preserve Outlier Concerns

Some concerns may come from only one persona but still be important:

- **Security concerns** — Even if only security-conscious flagged it
- **Accessibility concerns** — Even if only accessibility-advocate flagged it  
- **Enterprise concerns** — May not matter for individual devs but critical for team adoption

Mark outliers with their source persona and note: *"Consider if this audience matters to you."*

### Step 5: Synthesize What Worked

List positive elements that multiple personas noted:

| Strength | Personas Who Noted | Count |
|----------|-------------------|-------|
| [Description] | [List] | N |
| ... | ... | ... |

Preserve these in recommendations — don't remove what's working.

### Step 6: Collate Quote Evidence

For each major issue, gather supporting quotes:

**Issue: [Description]**
- skeptical-senior: "[quote]"
- junior-dev: "[quote]"
- ...

This evidence supports the recommendation and helps the author understand the problem.

### Step 7: Prioritize Recommendations

Rank recommendations by:

1. **Impact** = Frequency × Severity
   - Frequency: How many personas mentioned it?
   - Severity: Red flag? Confusion? Minor friction?

2. **Effort** (estimate)
   - Low: Copy change, reorganization
   - Medium: Section rewrite, new content
   - High: Structural change, new features

3. **Priority Score** = Impact / Effort

Sort recommendations by priority score, highest first.

### Step 8: Check for Conflicts

Sometimes personas disagree fundamentally:
- Junior dev wants more explanation
- Impatient scanner wants less text

Resolution approach:
1. Note the conflict explicitly
2. Consider if both can be satisfied (progressive disclosure, collapsible sections)
3. If truly conflicting, recommend based on target audience weighting

## Output Format

```markdown
## Perspective Review Summary

**Document:** [path]
**Personas:** [N] ([list])
**Date:** [ISO timestamp]

### Overall Scores

| Metric | Mean | StdDev | Interpretation |
|--------|------|--------|----------------|
| Trust | X.X | X.X | [High/Medium/Low agreement] |
| Value Clarity | X.X | X.X | ... |
| Structure | X.X | X.X | ... |
| Completeness | X.X | X.X | ... |
| Accessibility | X.X | X.X | ... |

**Adoption likelihood:** X% (based on "would use" responses)

---

### Consensus Issues (flagged by 50%+ of personas)

#### 1. [Issue Title]
**Frequency:** X/N personas
**Severity:** [Critical/Moderate/Minor]
**Evidence:**
> "[Quote from persona A]"
> "[Quote from persona B]"

**Recommendation:** [Specific action]

#### 2. [Next issue]
...

---

### Audience-Specific Issues

**For junior developers:**
- [Issue only they caught]

**For enterprise adoption:**
- [Issue only architect caught]

**For security posture:**
- [Issue only security-conscious caught]

---

### What Worked (preserve these)

1. [Strength] — noted by [N] personas
2. ...

---

### Minority Concerns (outlier insights)

| Concern | Raised By | Consider If... |
|---------|-----------|----------------|
| [Concern] | [Persona] | [Audience condition] |
| ... | ... | ... |

---

### Prioritized Recommendations

| Priority | Recommendation | Impact | Effort | Addresses |
|----------|----------------|--------|--------|-----------|
| 1 | [Action] | High | Low | Issues 1, 3 |
| 2 | [Action] | High | Medium | Issue 2 |
| 3 | [Action] | Medium | Low | Issue 4 |
| ... | ... | ... | ... | ... |

---

### Conflicting Feedback

| Conflict | Persona A Says | Persona B Says | Resolution |
|----------|----------------|----------------|------------|
| [Topic] | [View] | [Opposing view] | [Recommendation] |

---

### Raw Data

<details>
<summary>Individual Persona Scores</summary>

[Full scoring matrix]

</details>

<details>
<summary>All Identified Issues</summary>

[Complete issue list with frequencies]

</details>
```

## Aggregation Principles

1. **Preserve signal, not just consensus** — A single security concern from security-conscious is not noise
2. **Evidence over summary** — Include quotes so the author can see what triggered concerns
3. **Actionable over analytical** — Every issue should map to a recommendation
4. **Honest about conflicts** — Don't paper over disagreements
5. **Protect what works** — Recommendations should not remove strengths

---

*Transform N perspectives into one prioritized action plan.*
