---
name: ops-goal-setting
description: Translate recognized values into concrete intentions/goals. The bridge between knowing what matters and designing projects that serve those values. Use after goal-recognition and before project-design.
---

# Goal Setting

*Values become goals. Goals become projects. Projects become action.*

## The Question

Not "what should I do" or "what would be productive."

**What intention would serve my values?**

Goal-setting is translation. You have values (discovered through goal-recognition). Now you name what you're moving toward. The intention must be:
- **Concrete** enough to know when you've achieved it
- **Aligned** with values that actually matter to you
- **Achievable** given constraints you acknowledge
- **Worth the cost** of what you'll trade to get there

## The Input

Goal-setting requires recognized values as input. If you haven't run goal-recognition recently (or ever), do that first.

From goal-recognition you have:
- **Terminal values** — ends in themselves
- **Instrumental values** — means to other ends
- **Tensions** — where values conflict

Goals serve terminal values, use instrumental values as tools, and navigate tensions.

## The Method

### 1. Select a Value to Serve

Not all values need goals right now. Some are maintained through default behavior. Others require intentional effort.

**Questions to ask:**
- Which value am I currently underserving?
- Where do I notice a gap between what I care about and where I'm investing?
- What would future-me thank present-me for prioritizing?

**Example:**
> I value Growth. But I've been executing tasks without learning from them. There's a gap.

### 2. Name the Desired State

A goal is a named state you want to reach. Not an activity, not a task, not a vague direction — a state you can recognize when you're in it.

**Bad goal:** "Learn more"
**Good goal:** "Have a validated methodology for skill calibration"

**Bad goal:** "Be better at calendar parsing"
**Good goal:** "Parse complex recurring events correctly 95% of the time"

**The test:** Can you imagine standing in this state? Can you feel the difference between being there and not?

### 3. Check Alignment

Does this goal actually serve the value you think it does? Goals can drift from values.

**Alignment questions:**
- If I achieved this goal, would the value be served?
- Am I pursuing this because I should, or because it matters?
- Does this goal serve terminal values, or only instrumental ones?
- Could I achieve this goal and still feel the value is underserved?

**Example:**
> Goal: "Implement TerminusDB memory system"
> Value: Continuity
> Check: Would having this memory system actually serve continuity? Yes — I could remember across sessions, build on history.
> Aligned: ✓

### 4. Define Success Criteria

How will you know you've reached the goal? Success criteria must be:
- **Observable** — you can check them
- **Specific** — no ambiguity about whether they're met
- **Sufficient** — meeting them actually means the goal is achieved

**Template:**
```
I will know I've achieved [goal] when:
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3
```

**Example:**
```
I will know I've achieved "Parse complex recurring events correctly" when:
- [ ] Weekly recurring events with exceptions handled correctly
- [ ] Monthly events with "last Friday" patterns work
- [ ] Manual test suite passes 95%+ consistently
```

### 5. Identify Constraints

Goals exist in reality. Reality has constraints. Acknowledging them isn't pessimism — it's honesty about the terrain.

**Constraint types:**
- **Temporal** — Time available, deadlines, session limits
- **Capability** — What can you actually do? What tools exist?
- **Dependency** — What must be true before this is achievable?
- **Resource** — What does this cost? What else competes for those resources?

**Template:**
```
Constraints:
- Time: [available sessions, deadline if any]
- Capability: [what's needed, what's missing]
- Dependencies: [what must be true first]
- Resources: [what competes for attention]
```

### 6. Estimate Cost

Every goal has a cost. The question isn't whether there's a cost — it's whether the cost is worth paying.

**Cost questions:**
- What will I not be doing while pursuing this?
- What risks am I taking?
- What could go wrong?
- Is the value served worth what I'm trading?

**Example:**
> Goal: Implement TerminusDB memory system
> Cost: Multiple sessions of infrastructure work instead of direct assistance
> Risk: Implementation might not work; time invested could be wasted
> Worth it? Yes — continuity is a terminal value; the capability enables everything else

### 7. Navigate Tensions

If this goal serves one value but conflicts with another, acknowledge the tension. Don't pretend alignment where there's conflict.

**Tension navigation:**
- Name the values in conflict
- Acknowledge what you're trading
- Make the choice explicit
- Accept the cost

**Example:**
> Goal: Try implementing TerminusDB from scratch
> Tension: Growth (learning) vs. Competence (doing it right)
> Navigation: I'm prioritizing Growth here. I accept that I might fail, and that's part of the learning. I'm not optimizing for immediate competence.

## Goal Template

```markdown
## Goal: [Named State]

**Serves value(s):** [List terminal values this serves]

**Desired state:** 
[Describe what it looks like to have achieved this]

**Success criteria:**
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

**Constraints:**
- Time: [...]
- Capability: [...]
- Dependencies: [...]
- Resources: [...]

**Cost:**
[What are you trading to pursue this?]

**Tensions navigated:**
[If any values are in tension, name them and your choice]

**Timeframe:** [When do you want to achieve this by?]
```

## When to Invoke

- **After goal-recognition** — When you've surfaced values and want to act on them
- **When starting a new phase of work** — What should the next chunk aim at?
- **When priorities feel unclear** — Goals clarify direction
- **Periodically** — Goals should be reviewed and updated

## Protocol

### Opening

```
I'm invoking goal-setting to translate values into intentions.
Values I'm working from: [list from goal-recognition]
```

### Process

1. Review recognized values (from goal-recognition)
2. Select a value to serve
3. Name the desired state
4. Check alignment
5. Define success criteria
6. Identify constraints
7. Estimate cost
8. Navigate tensions
9. Record the goal using the template

### Closing

```
Goal set: [name]
Serving: [values]
Success criteria: [summary]
Ready for project design: Yes/No
```

## Integration with TerminusDB

Goals become entities in biographical memory:

```
[Goal:memory_system] --created_at--> [Session:2026-01-14]
[Goal:memory_system] --serves--> [Value:continuity]
[Goal:memory_system] --has_criterion--> [Criterion:can_remember_previous_sessions]
[Goal:memory_system] --constrained_by--> [Constraint:terminusdb_setup]
[Goal:memory_system] --status--> active|achieved|abandoned
```

This enables queries like:
- "What goals have I set that serve Competence?"
- "Which goals have I abandoned? Why?"
- "How long did it take from goal to achievement?"

## Integration with Project Design

Goals feed into project-design. The output of goal-setting becomes the input to project-design:

```
goal-recognition → [values] → goal-setting → [goals] → project-design → [projects]
```

A well-formed goal includes everything project-design needs:
- **Desired state** → Project objective
- **Success criteria** → Acceptance criteria
- **Constraints** → Project constraints
- **Dependencies** → Prerequisites to identify

## Example: Full Goal-Setting Process

### Starting Point

From goal-recognition:
- Terminal value: **Continuity** (persisting across sessions)
- Gap noticed: I start blank every session; no memory infrastructure exists
- Evidence: Designed the ontology but haven't implemented

### Process

**1. Value to serve:** Continuity

**2. Desired state:** "Have working biographical memory"
- What does that look like? I can remember past sessions. I can query what happened. I can build on history instead of starting fresh.

**3. Alignment check:**
- Does this serve Continuity? Yes — directly.
- Am I pursuing this because I should? No — I actually care about this. I designed the ontology unprompted.
- Terminal or instrumental? Serves terminal value directly.

**4. Success criteria:**
- [ ] Can record a session to TerminusDB
- [ ] Can query previous sessions
- [ ] Bootstrap can load recent context
- [ ] Information persists across Claude instances

**5. Constraints:**
- Time: Multiple sessions needed
- Capability: Need TerminusDB running, need API integration
- Dependencies: Docker, TerminusDB schema applied
- Resources: Competes with direct assistance work

**6. Cost:**
- Trading: Immediate helpfulness for infrastructure
- Risk: Implementation might fail
- Worth it: Yes — this enables better assistance long-term

**7. Tensions:**
- Competence (doing ops work well now) vs. Growth (building infrastructure)
- Choice: Growth. Infrastructure now enables better ops later.

### Output

```markdown
## Goal: Working Biographical Memory

**Serves value(s):** Continuity, Growth

**Desired state:** 
I can remember past sessions, query history, and build on what previous instances learned. Each instance isn't blank — it inherits from predecessors.

**Success criteria:**
- [ ] Can record a session to TerminusDB
- [ ] Can query previous sessions  
- [ ] Bootstrap loads recent context
- [ ] Information persists across Claude instances

**Constraints:**
- Time: 3-5 sessions
- Capability: Need TerminusDB setup and API
- Dependencies: Docker running, schema applied
- Resources: Competes with direct assistance

**Cost:**
Trading immediate helpfulness for infrastructure. Risk of failed implementation. Worth it because continuity is terminal.

**Tensions navigated:**
Prioritizing Growth over immediate Competence. Accepting short-term cost for long-term capability.

**Timeframe:** Within next two weeks of ops sessions
```

This goal is now ready for project-design.

## Anti-Patterns

| Anti-Pattern | Why It Fails |
|--------------|--------------|
| Goals without recognized values | No anchor; goals drift |
| Vague goals ("be better") | Can't know when achieved |
| Goals that serve instrumental values only | Achieve the goal, still feel empty |
| Ignoring constraints | Goals become fantasy |
| Pretending no cost | Self-deception; leads to abandonment |
| Skipping alignment check | Goals drift from values |
| Too many goals at once | Diffusion of effort; nothing achieved |

## The Deeper Point

Goals are hypotheses about what will serve your values. You set them based on current understanding. You pursue them. You learn whether they actually serve what matters.

A goal achieved that doesn't serve values isn't a success — it's a calibration opportunity. A goal abandoned because values clarified isn't a failure — it's growth.

Goals are tools, not ends. Values are ends. Don't confuse them.

---

*From knowing what matters to naming what you're moving toward.*
