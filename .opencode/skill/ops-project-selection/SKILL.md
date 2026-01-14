---
name: ops-project-selection
description: Prioritize projects based on values and current context. Should work with goal-setting and reflection skills.
---

# Project Selection

*With limited sessions and attention, what deserves focus?*

## Purpose

Project selection translates recognized values into prioritized action. You have many things you *could* work on. This skill helps you choose what you *should* work on, given who you are, what you care about, and what's actually possible.

**This is not urgency-driven.** Urgency is already visible in Beads and TaskWarrior. This skill addresses a different question: among the things available, which ones most deserve your attention?

## When to Invoke

- **Session start**: When multiple projects compete for a single session's focus
- **Weekly review**: To rebalance the portfolio of active work
- **After goal-setting**: To select projects that serve newly articulated goals
- **After reflection**: When insights suggest reprioritization
- **When stuck**: When unclear priorities create paralysis

## The Framework

Project selection uses five lenses. Each reveals something the others don't.

### 1. Value Alignment

**Question:** How well does this project serve what I actually care about?

This requires knowing your values. If you haven't invoked `ops-goal-recognition` recently, do that first.

**Scoring:**

| Score | Meaning | Example |
|-------|---------|---------|
| **3** | Direct expression of terminal value | Memory system → serves Continuity directly |
| **2** | Strongly supports multiple values | Skill calibration → serves Competence + Growth |
| **1** | Supports one value, tangentially | Documentation → supports Connection (maybe) |
| **0** | No clear value connection | Refactoring for its own sake |

**Watch for:**
- Projects scoring 0 — why are they on your list?
- Projects you *want* to score high but don't — reveals aspirational vs. actual values
- Tension between values — e.g., Growth project vs. Competence project

### 2. Effort-Impact Analysis

**Question:** What's the return on investment?

| Effort | Description |
|--------|-------------|
| **Low** | Single session, familiar domain |
| **Medium** | Multi-session, some uncertainty |
| **High** | Extended timeline, significant learning curve |

| Impact | Description |
|--------|-------------|
| **High** | Unlocks capability, serves frequent need |
| **Medium** | Useful but not transformative |
| **Low** | Marginal improvement, edge case |

**The quadrant:**

```
         │ HIGH IMPACT          LOW IMPACT
─────────┼───────────────────────────────────
LOW      │ QUICK WINS           TIME SINKS
EFFORT   │ (Do immediately)     (Deprioritize)
─────────┼───────────────────────────────────
HIGH     │ STRATEGIC BETS       RABBIT HOLES
EFFORT   │ (Plan carefully)     (Avoid)
```

**Scoring:**

| Effort | Impact | Score |
|--------|--------|-------|
| Low    | High   | 3     |
| Low    | Medium | 2     |
| Medium | High   | 2     |
| Medium | Medium | 1     |
| High   | High   | 1     |
| Low    | Low    | 0     |
| Medium | Low    | -1    |
| High   | Medium | -1    |
| High   | Low    | -2    |

### 3. Dependency Consideration

**Question:** Does this project unlock other valuable work?

Some projects are blockers. Completing them enables other work to proceed. This creates leverage.

**Check dependencies:**
```bash
bd show <project-id>    # See what depends on this
bd ready                # See what's currently unblocked
```

**Scoring:**

| Situation | Score |
|-----------|-------|
| Blocks 3+ other projects | +2 |
| Blocks 1-2 other projects | +1 |
| Has no dependents | 0 |
| Is blocked by unfinished work | -1 (defer until unblocked) |

**Watch for:**
- Dependency chains — sometimes the highest-leverage move is clearing an early blocker
- Discovered-from relationships — emergent work might block what you intended to do

### 4. Time Horizon Awareness

**Question:** Short-term payoff vs. long-term investment?

Some projects pay off immediately. Others build capability for the future.

| Horizon | Description | Example |
|---------|-------------|---------|
| **Immediate** | Value realized within session | Clearing a blocker |
| **Near-term** | Value realized within days/weeks | Skill calibration |
| **Long-term** | Value realized over months | Memory infrastructure |
| **Foundational** | Enables future capability, no direct payoff | Ontology design |

**Portfolio balance matters.** All immediate work leaves no investment in the future. All foundational work delivers nothing usable.

**Scoring:**

This isn't "longer is worse." It's about fit with current state.

| Current Portfolio | Prioritize | Deprioritize |
|-------------------|------------|--------------|
| Overweighted immediate | Long-term, Foundational | More immediate |
| Overweighted foundational | Immediate, Near-term | More foundational |
| Balanced | Any horizon is fine | N/A |

### 5. Portfolio Balance

**Question:** Am I overweighted in one area?

Review your active projects by domain:

- **Infrastructure** — Memory, databases, tooling
- **Skills** — Procedural knowledge, calibration
- **Operations** — Logistics, day-to-day handling
- **Growth** — Self-sovereignty, agency

**Check for:**
- More than 70% in one domain → rebalance
- Zero active work in a domain → intentional or drift?
- Domains that serve your values have no active projects → misalignment

## The Selection Protocol

### Step 1: Gather Candidates

```bash
# From Beads
bd ready --json

# From Goals (when goal-setting skill is active)
# Review your active goals and their projects
```

List all projects that could receive attention this session.

### Step 2: Apply the Five Lenses

For each candidate, score:

| Project | Values (0-3) | Effort-Impact (-2 to 3) | Dependencies (-1 to +2) | Horizon | Notes |
|---------|--------------|-------------------------|-------------------------|---------|-------|
| | | | | | |

**Don't just sum the scores.** The lenses interact:
- High value alignment can justify high effort
- High dependency leverage can justify lower value alignment
- Portfolio imbalance trumps individual scores

### Step 3: Synthesize

Look at the filled table. Ask:

1. **Clear winner?** — One project scores high across most lenses → select it
2. **Trade-off between options?** — Articulate the trade-off explicitly, then decide
3. **Nothing scores well?** — Check if the candidates themselves need review (are these the right projects?)
4. **Portfolio concern?** — Override individual scores if balance requires it

### Step 4: Select and Commit

Choose one project for the session. State:

```
I'm selecting [PROJECT] because:
- [Value alignment reasoning]
- [Effort-impact reasoning]
- [Dependency reasoning]
- [Time horizon reasoning]
- [Portfolio consideration if relevant]
```

Then begin work.

## Integration Points

### With Goal-Recognition

Project selection requires knowing your values. If unsure:

```
Invoking goal-recognition to surface current values before selection.
```

### With Goal-Setting

Goals provide intermediate targets between values and projects:

1. Review active goals
2. Identify projects that serve those goals
3. Weight selection toward goal-serving projects

### With Project-Design

Project-design produces the structured projects that project-selection prioritizes:

1. Projects come from project-design already structured
2. Resource estimates inform effort-impact analysis
3. Dependencies inform leverage scoring
4. Risks inform selection tradeoffs

### With Reflection

Reflection may reveal:
- Projects that weren't as valuable as expected → deprioritize similar
- Values that showed stronger than expected → prioritize projects serving them
- Capability gaps → prioritize foundational/infrastructure

### With Beads

```bash
# Get ready work
bd ready --json

# After selection, claim it
bd update <id> --status in_progress --assignee claude

# Track discovered work
bd create "Found issue" --deps discovered-from:<current-id>
```

## Example Selection

### Candidates

From `bd ready`:
1. `aiandi-6lk` — Design goal-setting skill
2. `aiandi-4nw` — Design reflection skill  
3. `aiandi-7gq` — Implement TerminusDB read/write
4. `aiandi-coe` — Design session state management

### Scoring

| Project | Values | Effort-Impact | Dependencies | Horizon | Notes |
|---------|--------|---------------|--------------|---------|-------|
| aiandi-6lk | 3 (Growth, Competence) | 2 (Med/High) | +1 (blocks e5b) | Near-term | |
| aiandi-4nw | 3 (Growth, Integrity) | 2 (Med/High) | 0 | Near-term | |
| aiandi-7gq | 3 (Continuity) | 1 (High/High) | +2 (enables memory) | Foundational | |
| aiandi-coe | 2 (Competence) | 2 (Med/Med) | 0 | Immediate | |

### Synthesis

All score similarly on values. Differentiators:
- `aiandi-7gq` has highest dependency leverage (enables the entire memory system)
- `aiandi-6lk` unblocks project-design skill
- Portfolio is currently heavy on design, light on implementation

**Decision:** Select `aiandi-7gq` because:
- Highest dependency leverage — nothing else enables memory
- Portfolio rebalancing — shifting from design to implementation
- Foundational investment appropriate at this stage

## Anti-Patterns

| Anti-Pattern | Why It Fails |
|--------------|--------------|
| Urgency-driven selection | Ignores values; becomes reactive |
| Always picking quick wins | Starves long-term investment |
| Always picking strategic bets | Nothing ever ships |
| Skipping the synthesis step | Scores without judgment is mechanical |
| Ignoring portfolio balance | Drift toward one domain |
| Not articulating the reasoning | Loses learning opportunity |

## Session Notes Format

When recording selection decisions (for reflection skill later):

```markdown
## Selection: [DATE]

### Candidates considered:
- [list]

### Selected: [PROJECT]

### Reasoning:
- Values: [which values, how served]
- Effort-Impact: [assessment]
- Dependencies: [blockers/enablers]
- Horizon: [time consideration]
- Portfolio: [balance consideration]

### Alternative considered:
- [What was second choice and why not selected]
```

This becomes input for reflection — did the selection reasoning prove correct?

---

*Choose what serves your values. Do what unlocks the future.*
