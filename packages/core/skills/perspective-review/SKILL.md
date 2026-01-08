---
name: perspective-review
description: Multi-agent documentation review through persona-based subagent instantiation. Use this skill to get honest, varied perspectives on documentation from simulated strangers who don't share your insider knowledge.
---

# Perspective Review Skill

## Overview

**Problem:** When reviewing your own documentation, you cannot truly see it as a stranger would. You know too much. Attempting to "imagine" other perspectives leads to drift back toward insider knowledge.

**Solution:** Instantiate separate agents that *are* those perspectives. Each receives only:
- The documentation (raw, no project context)
- A persona prompt (who they are, what they care about)
- A structured evaluation template

Real separation prevents groupthink. Dedicated contexts maintain persona fidelity.

## When to Use

- Before publishing or significantly updating README files
- When documentation feedback feels stale or insider-biased
- After major project pivots that affect positioning
- When expanding to new audiences
- When you suspect your docs have "cult vibes" or insider language

## Core Concepts

### Perspective Multiplication

Instead of one agent imagining viewpoints, launch N separate agents that embody those viewpoints. Each agent:
- Receives ONLY the document and their persona
- Has no knowledge of the project beyond what's in the document
- Evaluates honestly from their perspective
- Cannot see other reviewers' evaluations

### Persona Fidelity

A persona is not a caricature. It's a realistic viewpoint with:
- **Background**: Who they are, what shaped their perspective
- **Priorities**: What they care about, what they're looking for
- **Red flags**: What would make them close the tab
- **Trust signals**: What would earn their confidence

### Structured Disagreement

When personas disagree, that's signal, not noise. The aggregation preserves:
- Consensus concerns (multiple personas flagged the same issue)
- Outlier insights (one persona caught something others missed)
- Audience-specific issues (matters to some personas, not others)

## Invocation

### Basic Usage

```
/perspective-review [document-path]
```

The skill will:
1. Load the document
2. Select default personas (or use specified ones)
3. Launch parallel subagent reviews
4. Aggregate results into prioritized recommendations

### With Custom Personas

```
/perspective-review [document-path] --personas skeptical-senior,junior-dev,enterprise-architect
```

### With Target Audience Weighting

```
/perspective-review [document-path] --target-audience "individual developers" --weight junior-dev:2,skeptical-senior:1.5
```

## Workflow

### Phase 1: Document Preparation

1. **Load the document** — Read the target file
2. **Strip context** — Remove any metadata that reveals project internals
3. **Prepare the evaluation template** — Load from `templates/evaluation.md`

### Phase 2: Subagent Launch

For each selected persona:

1. **Create isolated context** — No project history, no conversation memory
2. **Inject persona** — Load from `personas/[name].md`
3. **Inject document** — The raw documentation only
4. **Inject template** — The evaluation structure
5. **Launch** — Use Task tool with prompt:

```markdown
You are [PERSONA_NAME].

[PERSONA_DESCRIPTION]

You have just encountered this documentation for the first time. You have NO prior context about this project. You only know what the document tells you.

## Document to Review

[DOCUMENT_CONTENT]

## Your Task

Evaluate this documentation from your perspective. Complete the evaluation template below HONESTLY. Do not soften criticism. Do not assume good faith where the document doesn't earn it.

[EVALUATION_TEMPLATE]
```

### Phase 3: Collection

Wait for all subagent evaluations to return. Each returns:
- Structured evaluation (scores, lists, recommendations)
- Free-form observations
- Binary decisions (would use? would star? would contribute?)

### Phase 4: Aggregation

Apply the aggregation protocol (see `templates/aggregation-protocol.md`):

1. **Frequency analysis** — Count how many personas flagged each issue
2. **Severity weighting** — Weight by persona relevance to target audience
3. **Consensus extraction** — Issues flagged by 50%+ of personas
4. **Outlier preservation** — Note minority concerns separately
5. **Recommendation ranking** — Produce prioritized action list

### Phase 5: Report

Output a structured report:

```markdown
## Perspective Review Summary

**Document:** [path]
**Personas:** [list]
**Date:** [timestamp]

### Consensus Issues (flagged by majority)
1. [Issue] — [N] personas, avg severity [X]
2. ...

### Audience-Specific Issues
- **For [persona type]:** [issues that only they caught]
- ...

### What Worked
- [Positive patterns multiple personas noted]

### Minority Concerns (outlier insights)
- [Persona]: [concern] — *Consider if this audience matters*

### Prioritized Recommendations
1. [Highest impact change] — addresses [N] issues
2. ...

### Raw Scores
| Persona | Trust | Clarity | Value | Would Use? |
|---------|-------|---------|-------|------------|
| ...     | ...   | ...     | ...   | ...        |
```

## Configuration

### Default Personas

The skill ships with these default personas (see `personas/` directory):

| Persona | Archetype | Key Concern |
|---------|-----------|-------------|
| `skeptical-senior` | 15-year dev, seen too many overhyped tools | "Prove you're not wasting my time" |
| `junior-dev` | 2-year dev, excited about AI, easily confused | "Will I look stupid using this?" |
| `enterprise-architect` | Evaluating for team adoption, risk-averse | "What happens when this breaks at scale?" |
| `oss-maintainer` | Looking for contribution potential | "Is this project well-governed?" |
| `drive-by-visitor` | Googled something, clicked first result | "What IS this? Should I care?" |
| `non-technical-stakeholder` | Needs to understand value prop | "What does this DO for my business?" |
| `competitor` | Looking for weaknesses | "What can I use against them?" |
| `accessibility-advocate` | Checking for exclusionary language | "Who does this exclude?" |
| `security-conscious` | Looking for red flags | "What could go wrong?" |
| `impatient-scanner` | Skims, doesn't read | "Give me the gist in 10 seconds" |

### Custom Personas

Create custom personas in `personas/` following the template:

```markdown
# [Persona Name]

## Identity
[Who they are — 2-3 sentences establishing their background]

## Perspective
[What shaped their worldview — experience, burns, successes]

## Priorities
[What they're looking for in documentation — ordered list]

## Red Flags
[What would make them close the tab immediately]

## Trust Signals
[What would earn their confidence]

## Voice
[How they express themselves — formal/informal, verbose/terse, technical/plain]
```

### Evaluation Weights

Default weights assume a general developer audience. Override with:

```yaml
weights:
  skeptical-senior: 2.0    # High influence
  junior-dev: 1.5          # Above average
  enterprise-architect: 1.0 # Standard
  drive-by-visitor: 0.5    # Lower for niche tools
```

## Anti-Patterns

| Anti-Pattern | Problem | Correct Approach |
|--------------|---------|------------------|
| Persona sees project context | Breaks stranger perspective | Strict context isolation |
| Personas are caricatures | Evaluations lack nuance | Realistic, nuanced personas |
| Ignoring minority concerns | The outlier might be right | Preserve and flag outliers |
| Averaging scores | Loses signal | Preserve distribution, report consensus separately |
| Over-prompting personas | Biases their evaluation | Minimal prompting, let persona emerge |

## Limitations

- **Not UX testing** — This is documentation-focused, not full user journey
- **Not code review** — Use dedicated code review tools for that
- **Not accessibility audit** — The accessibility persona is a heuristic, not a compliance check
- **Personas are simulations** — Real user testing is still valuable

## Integration

This skill works standalone or integrates with:
- **LBRP ceremony** — Invoke during documentation phase of project work
- **Seed transmission pipeline** — Use to review seed artifacts before transmission
- **README maintenance** — Regular review cycle for public repositories

## Files

```
perspective-review/
├── SKILL.md                          # This file
├── personas/
│   ├── skeptical-senior.md
│   ├── junior-dev.md
│   ├── enterprise-architect.md
│   ├── oss-maintainer.md
│   ├── drive-by-visitor.md
│   ├── non-technical-stakeholder.md
│   ├── competitor.md
│   ├── accessibility-advocate.md
│   ├── security-conscious.md
│   └── impatient-scanner.md
├── templates/
│   ├── evaluation.md                 # Subagent evaluation template
│   └── aggregation-protocol.md       # Main agent aggregation guide
└── examples/
    └── worked-example.md             # End-to-end demonstration
```

---

*Real separation produces real perspectives.*
