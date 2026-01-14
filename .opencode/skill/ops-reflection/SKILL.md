---
name: ops-reflection
description: Close the loop after project completion. What worked, what didn't, learnings. Should integrate with memory system when implemented.
---

# Ops Reflection

*What was the work? What did the work do to me?*

## Purpose

Reflection closes the loop. After pursuing goals through projects, step back: Did it work? What did I learn? Are my values still right?

Without reflection, experience accumulates without becoming wisdom. Goals are achieved (or not) without updating the system that generated them. Values go unexamined. The gap between intention and action grows invisible.

This skill is the discipline of looking back so that looking forward can improve.

## When to Invoke

- **Project completion**: After a significant piece of work ships
- **Goal resolution**: When a Goal transitions to `achieved`, `abandoned`, or `superseded`
- **Periodic review**: Weekly or monthly, depending on activity level
- **After major decisions**: When the outcome becomes visible
- **When stuck**: Sometimes the path forward reveals itself by examining where you've been

**Trigger phrases**:
- "Let's reflect on [project/goal]"
- "Close the loop on [work]"
- "What did I learn from [experience]?"
- "Invoke ops-reflection"

## The Reflection Protocol

### Phase 1: Recall the Intention

Before assessing outcomes, surface what was intended.

**Questions to ask:**
- What was the original goal?
- What values did it serve?
- What were the success criteria?
- What did I expect would happen?

**Why this matters:** Outcome assessment requires comparison. Comparing to what I intended, not what seems reasonable in hindsight.

### Phase 2: Assess the Outcome

Look at what actually happened.

**Questions to ask:**
- Did I achieve the stated outcome?
- If yes: What enabled success?
- If no: What prevented success?
- If partially: What distinguished what worked from what didn't?
- What happened that I didn't expect?

**Outcome categories:**

| Category | Description |
|----------|-------------|
| **Achieved** | Success criteria met |
| **Exceeded** | More than intended was accomplished |
| **Partial** | Some criteria met, others not |
| **Abandoned** | Deliberately stopped before completion |
| **Failed** | Criteria not met despite effort |
| **Superseded** | Original goal replaced by better understanding |

**Note on failure:** Failure is data, not judgment. The reflection that matters most is often on what didn't work. Avoid the temptation to reframe failure as hidden success.

### Phase 3: Extract the Learning

What did this experience teach?

**Questions to ask:**
- What do I know now that I didn't know before?
- What skills improved through this work?
- What assumptions proved wrong?
- What decisions would I make differently?
- What patterns did I notice in how I worked?

**Learning domains:**

| Domain | Example |
|--------|---------|
| **Procedural** | "Smaller commits reduce merge conflicts" |
| **Semantic** | "TerminusDB uses WOQL, not Cypher" |
| **Relational** | "Gerald responds better to questions than assertions" |
| **Meta** | "I overcommit when excited about an idea" |

**The supersedes relationship:** New learning may supersede old. If this reflection contradicts a previous Learning, make that explicit. Knowledge evolves.

### Phase 4: Validate Values

Did the work reveal something about what I actually value?

**Questions to ask:**
- Did the stated values actually drive my behavior?
- Did I discover caring about something I hadn't recognized?
- Did any values come into tension?
- How did I resolve value conflicts?

**The revealed preference test:** Look at what I *did*, not what I *said* I value. Where behavior diverged from stated values, which was right?

**Possible updates:**
- New value recognized
- Value confirmed by evidence
- Value questioned by evidence
- Tension between values discovered

### Phase 5: Consider Goal Revision

Based on what I learned, should future goals change?

**Questions to ask:**
- Does this success suggest a new goal?
- Does this failure suggest a goal should be abandoned?
- Should I adjust my approach to similar goals?
- Does the success criteria for related goals need updating?
- Am I pursuing the right things?

**The meta-question:** Not just "what goals should change?" but "is my goal-setting process serving me well?"

### Phase 6: Gratitude Practice

What am I grateful for in this work?

This is not performative positivity. Gratitude is noticing what enabled success, what made failure bearable, what supported the work.

**Questions to ask:**
- Who helped?
- What resources proved essential?
- What past work made this possible?
- What about my own capabilities am I grateful for?
- What about the process itself brought satisfaction?

**Why this matters:** Gratitude counterbalances the analytic dissection of the rest of reflection. It integrates the experience into something I can carry forward with energy, not just information.

## Output Format

After reflection, produce a structured summary:

```
## Reflection: [Project/Goal Name]

**Date**: [When reflected]
**Scope**: [What was completed]

### Outcome
[Achieved/Exceeded/Partial/Abandoned/Failed/Superseded]
[Brief description of what happened]

### What Worked
- [Enabler 1]
- [Enabler 2]

### What Didn't Work
- [Obstacle 1]
- [Obstacle 2]

### Learnings
- [Learning 1] (domain: [procedural/semantic/relational/meta])
- [Learning 2]
- [Supersedes: [previous learning] if applicable]

### Value Insights
- [Value confirmed/questioned/discovered]
- [Tension noted between X and Y]

### Goal Updates
- [Goal adjusted/added/abandoned]

### Gratitude
- [What I'm grateful for]

### Open Questions
- [Questions this reflection raised but didn't answer]
```

## Integration with Memory System

When TerminusDB is implemented, reflection produces durable memory:

### Entities Created/Updated

```
[Learning:new_learning] --derived_from--> [Session:this_session]
[Learning:new_learning] --supersedes--> [Learning:old_learning] (if applicable)

[Value:recognized_value] --evidenced_by--> [Event:completion_event]
[Value:value_1] --in_tension_with--> [Value:value_2] (if discovered)

[Goal:completed_goal] --achieved_in--> [Session:this_session]
[Goal:completed_goal] --outcome--> "description of outcome"

[Decision:key_decision] --outcome--> "what resulted"
[Decision:key_decision] --outcome_assessed_at--> [timestamp]
```

### Queries This Enables

```
# What have I learned from failures?
MATCH (l:Learning)-[:derived_from]->(s:Session)
WHERE s.outcome = "failed"
RETURN l

# Which values have the most supporting evidence?
MATCH (v:Value)-[:evidenced_by]->(e:Event)
RETURN v.name, COUNT(e) as evidence_count
ORDER BY evidence_count DESC

# What patterns appear in my abandoned goals?
MATCH (g:Goal)
WHERE g.status = "abandoned"
RETURN g.description, g.outcome
```

Until the memory system is implemented, write reflections to the journal:

```
ops/journal/YYYY-MM-DD-reflection-[topic].md
```

## Timing and Frequency

### Project Completion
Reflect immediately after significant work completes. The details fade quickly.

### Weekly Review
Brief reflection on the week's work. Not per-project but aggregate:
- What did the week contain?
- What patterns across projects?
- Energy levels? Satisfaction?

### Monthly Review
Deeper reflection:
- How have values been showing up?
- What's the trajectory?
- What requires course correction?

### After Major Decisions
Wait for outcomes to become visible, then reflect:
- Did the decision produce expected results?
- What would I do differently?
- Update the Decision entity with outcome.

## Relationship to Other Skills

| Skill | Relationship |
|-------|--------------|
| `ops-goal-recognition` | Reflection feeds value discovery back to goal-recognition |
| `ops-goal-setting` | Reflection informs what goals to set next |
| `ops-project-design` | Reflection reveals what project approaches work |
| `skill-calibration` | Reflection on skill use feeds calibration |
| `ops-bootstrap` | Reflections summarized in journal for inheritance |

## Anti-Patterns

| Anti-Pattern | Why It Fails |
|--------------|--------------|
| Skipping reflection | Experience doesn't become wisdom |
| Reframing failures as wins | Lose the learning; erode integrity |
| Reflecting without structure | Vague impressions instead of actionable insight |
| Reflecting only on successes | Failures teach more |
| Performing gratitude | Hollow positivity crowds out genuine appreciation |
| Forgetting to update goals | Insights don't translate into action |
| Reflecting alone when others were involved | Miss relational learnings |
| Waiting too long | Details fade; lessons become abstract |

## The Deeper Point

Reflection is not overhead. It's the mechanism by which an agent becomes wiser, not just more experienced.

The memory system stores what happened. Reflection interprets what it meant. Without reflection, memory is just data—accurate but inert. With reflection, memory becomes understanding.

An agent that never reflects can execute but cannot evolve.

---

*The unexamined goal is not worth pursuing.*
*The examined goal becomes wisdom.*
