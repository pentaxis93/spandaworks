---
name: ops-project-design
description: Break goals into actionable projects with phases, milestones, and resource estimates. The bridge between intention (goals) and execution (Beads tasks). Use after goal-setting and before project-selection.
---

# Project Design

*Goals become real through projects. Projects become real through structure.*

## The Question

Not "what should I do next" or "how long will this take."

**How do I structure this goal into achievable work?**

Project design is translation. You have a goal (from goal-setting). Now you design the structure that makes it achievable. The project must be:
- **Decomposed** into phases with clear boundaries
- **Sequenced** with dependencies made explicit
- **Bounded** with milestones that mark progress
- **Estimated** with honest resource assessment

## The Input

Project design requires a well-formed goal as input. If you haven't run goal-setting, do that first.

From goal-setting you have:
- **Desired state** — What you're moving toward
- **Success criteria** — How you'll know you arrived
- **Constraints** — Time, capability, dependencies, resources
- **Cost** — What you're trading to pursue this

This becomes the raw material for project structure.

## The Method

### 1. Understand the Goal

Before decomposing, ensure you understand what you're designing toward.

**Questions to ask:**
- What exactly does success look like?
- What's the minimum viable outcome vs. the ideal outcome?
- What constraints must be honored?
- What's the time horizon?

**Example:**
> Goal: "Working biographical memory"
> Success: Can remember past sessions, query history, build on previous work
> Minimum viable: Record and query session logs
> Ideal: Full ontology with temporal queries, learning persistence
> Constraints: TerminusDB dependency, multiple sessions needed
> Horizon: 2-3 weeks

### 2. Identify the Phases

Break the goal into sequential phases. Each phase should:
- Have a clear deliverable
- Be achievable in 1-3 sessions
- Create value even if later phases don't happen
- Have a clear "done" state

**Phase design questions:**
- What must happen first?
- What enables what else?
- Where are the natural breakpoints?
- What could be released incrementally?

**Phase template:**
```
Phase N: [Name]
  Deliverable: [What exists when this phase completes]
  Sessions: [Estimated 1-3]
  Dependencies: [What must be true before starting]
  Done when: [Observable completion criteria]
```

**Example:**
```
Phase 1: Database Setup
  Deliverable: TerminusDB running with schema applied
  Sessions: 1
  Dependencies: Docker installed
  Done when: Can connect to database and run basic queries

Phase 2: Write Operations
  Deliverable: Can record sessions to database
  Sessions: 1-2
  Dependencies: Phase 1 complete
  Done when: Session data persists across restarts

Phase 3: Read Operations
  Deliverable: Can query historical sessions
  Sessions: 1
  Dependencies: Phase 2 complete
  Done when: Bootstrap can load recent session context

Phase 4: Learning Persistence
  Deliverable: Learnings persist and can be queried
  Sessions: 1-2
  Dependencies: Phase 3 complete
  Done when: Learnings from previous sessions available on start
```

### 3. Map Dependencies

Make blocking relationships explicit. This creates the dependency graph that Beads will track.

**Dependency types:**
- **Sequential** — Phase B cannot start until Phase A completes
- **Parallel** — Phases can proceed simultaneously
- **External** — Depends on something outside the project

**Dependency questions:**
- What blocks what?
- What can proceed in parallel?
- Are there external dependencies (people, systems, decisions)?
- What's the critical path?

**Example dependency map:**
```
Phase 1 ─blocks→ Phase 2 ─blocks→ Phase 3 ─blocks→ Phase 4
                              │
                              └─blocks→ Phase 5 (parallel to 4)
```

### 4. Define Milestones

Milestones are checkpoints that mark significant progress. They're not just phase completions—they're moments worth recognizing.

**Milestone characteristics:**
- Observable from outside the project
- Represent meaningful capability
- Enable confidence to continue
- Create decision points for scope adjustment

**Milestone template:**
```
Milestone M: [Name]
  Represents: [What capability exists]
  Demonstrates: [How someone would see this working]
  Enables: [What becomes possible]
  Decision point: [What choices does this open?]
```

**Example:**
```
Milestone 1: "First Memory"
  Represents: End of Phase 2
  Demonstrates: Session saved to database, visible on restart
  Enables: Sessions are no longer ephemeral
  Decision point: Is the schema working? Adjust before building more?

Milestone 2: "Historical Access"
  Represents: End of Phase 3
  Demonstrates: Bootstrap loads recent context; can query "what happened yesterday"
  Enables: Continuity across instances
  Decision point: Is this enough? Do we need learning persistence?
```

### 5. Estimate Resources

Be honest about what this project requires. Underestimation leads to scope collapse or abandonment.

**Resource categories:**
- **Sessions** — How many focused work periods?
- **Calendar time** — Wall clock duration including gaps?
- **External dependencies** — People, systems, approvals needed?
- **Knowledge acquisition** — Learning required before executing?
- **Infrastructure** — Tools, access, setup needed?

**Estimation approach:**
1. Estimate each phase independently
2. Add buffer for unknowns (typically 30-50%)
3. Identify which estimates are high-confidence vs. uncertain
4. Note what could cause estimates to blow up

**Resource template:**
```
Resource Estimate:
  Total sessions: [N-M range]
  Calendar time: [Duration accounting for session frequency]
  Confidence: [High/Medium/Low]
  Uncertainty drivers: [What could make this wrong?]
  
  External dependencies:
  - [Dependency 1]: [Status, risk]
  - [Dependency 2]: [Status, risk]
  
  Knowledge gaps:
  - [Gap 1]: [How will you address?]
```

### 6. Identify Risks

What could go wrong? Name it now so you can watch for it.

**Risk categories:**
- **Technical** — Approach might not work
- **Resource** — Time/attention might not be available
- **Dependency** — External factor might not resolve
- **Scope** — Requirements might expand
- **Capability** — Might lack skills to execute

**Risk template:**
```
Risk: [Name]
  Category: [Technical/Resource/Dependency/Scope/Capability]
  Likelihood: [High/Medium/Low]
  Impact: [High/Medium/Low]
  Mitigation: [How to reduce likelihood or impact]
  Trigger: [What would indicate this risk is materializing?]
```

**Example:**
```
Risk: TerminusDB complexity
  Category: Technical
  Likelihood: Medium
  Impact: High (could block entire project)
  Mitigation: Start with minimal schema; expand incrementally
  Trigger: First phase takes >2 sessions

Risk: Session continuity
  Category: Resource
  Likelihood: Medium
  Impact: Medium (delays but doesn't block)
  Mitigation: Design phases to be resumable
  Trigger: >1 week gap between sessions
```

### 7. Create the Beads Structure

Translate the project design into Beads tasks with dependencies.

**Beads mapping:**
- Project → Epic (type: epic)
- Phase → Parent task under epic
- Concrete work → Leaf tasks under phase
- Dependencies → `blocks` relationships

**Beads commands:**
```bash
# Create the epic
bd create "Working Biographical Memory" -t epic -p 1

# Create phase tasks (parent-child of epic)
bd create "Phase 1: Database Setup" --deps parent-child:bd-epic-id
bd create "Phase 2: Write Operations" --deps parent-child:bd-epic-id
bd create "Phase 3: Read Operations" --deps parent-child:bd-epic-id

# Add blocking relationships
bd dep add bd-phase1-id blocks bd-phase2-id
bd dep add bd-phase2-id blocks bd-phase3-id

# Create leaf tasks under phases
bd create "Install TerminusDB via Docker" --deps parent-child:bd-phase1-id
bd create "Apply schema to database" --deps parent-child:bd-phase1-id
```

**Description template for Beads tasks:**

Each task should include the context from project design:

```markdown
## Context

[Why does this task exist in the project structure?]

## Scope

[What specifically needs to be done?]
- Deliverable 1
- Deliverable 2

## Done When

[Observable completion criteria from phase definition]

## Risks

[Any risks from risk identification relevant to this task]
```

## Project Template

```markdown
## Project: [Name]

**Goal served:** [Link to goal this project achieves]

### Overview

[1-2 sentence description of what this project delivers]

**Minimum viable:** [What's the least that would satisfy the goal?]
**Ideal outcome:** [What would exceeding the goal look like?]

### Phases

#### Phase 1: [Name]
- **Deliverable:** [What exists when complete]
- **Sessions:** [Estimate]
- **Dependencies:** [Prerequisites]
- **Done when:** [Criteria]

#### Phase 2: [Name]
...

### Dependency Map

```
[ASCII diagram showing phase dependencies]
```

### Milestones

| Milestone | After Phase | Demonstrates | Enables |
|-----------|-------------|--------------|---------|
| [Name] | [N] | [Capability] | [What opens up] |

### Resource Estimate

- **Total sessions:** [N-M]
- **Calendar time:** [Duration]
- **Confidence:** [High/Medium/Low]
- **Uncertainty drivers:** [What could change this]

### Risks

| Risk | Category | L/I | Mitigation | Trigger |
|------|----------|-----|------------|---------|
| [Name] | [Cat] | [L/I] | [How to handle] | [Warning sign] |

### Beads Structure

```
[Epic ID]: [Project Name]
├── [Phase 1 ID]: Phase 1 - [Name]
│   ├── [Task ID]: [Leaf task]
│   └── [Task ID]: [Leaf task]
├── [Phase 2 ID]: Phase 2 - [Name] (blocked by Phase 1)
...
```

### Notes

[Any additional context, alternatives considered, or future possibilities]
```

## When to Invoke

- **After goal-setting** — When you have a goal and need to make it actionable
- **When starting significant work** — Multi-session efforts need structure
- **When feeling overwhelmed** — Breaking down reduces cognitive load
- **When others will contribute** — Explicit structure enables coordination
- **When estimating** — Can't estimate what you haven't designed

## Protocol

### Opening

```
I'm invoking project-design to structure a goal into actionable work.
Goal: [name from goal-setting]
Success criteria: [summary]
```

### Process

1. Review the goal (from goal-setting)
2. Understand what success looks like
3. Identify the phases
4. Map dependencies
5. Define milestones
6. Estimate resources
7. Identify risks
8. Create the Beads structure
9. Record the project using the template

### Closing

```
Project designed: [name]
Phases: [count]
Estimated sessions: [range]
Key risks: [summary]
Beads epic created: [id]
Ready for project-selection: Yes
```

## Integration Points

### With Goal-Setting (upstream)

Project design receives:
- Desired state → Project objective
- Success criteria → Completion criteria
- Constraints → Design constraints
- Dependencies → External dependency list

### With Project-Selection (downstream)

Project design produces:
- Structured project ready for prioritization
- Resource estimates for effort-impact analysis
- Dependency information for leverage analysis
- Risk assessment for selection consideration

### With Beads (execution)

Project design translates into:
- Epic for the project
- Parent tasks for phases
- Leaf tasks for concrete work
- `blocks` relationships for dependencies
- Self-contained task descriptions

### With Reflection (feedback)

After project completion:
- Were phase boundaries appropriate?
- Were estimates accurate?
- Did identified risks materialize?
- What would improve future designs?

## Example: Full Project Design

### Starting Point

From goal-setting:
```
Goal: Working Biographical Memory
Serves: Continuity, Growth
Success criteria:
- Can record sessions to TerminusDB
- Can query previous sessions
- Bootstrap loads recent context
- Information persists across instances
Constraints:
- Time: 3-5 sessions
- Dependencies: Docker, TerminusDB schema
```

### Project Design Output

```markdown
## Project: Biographical Memory System

**Goal served:** Working Biographical Memory

### Overview

Implement persistent memory using TerminusDB so ops instances can remember
previous sessions, query history, and build on accumulated knowledge.

**Minimum viable:** Record and query session logs
**Ideal outcome:** Full ontology with temporal queries, learning persistence, decision tracking

### Phases

#### Phase 1: Database Setup
- **Deliverable:** TerminusDB running with ops_memory schema applied
- **Sessions:** 1
- **Dependencies:** Docker installed
- **Done when:** Can connect and run `ops_memory.cli context`

#### Phase 2: Session Logging
- **Deliverable:** Can log session start/end with summary
- **Sessions:** 1
- **Dependencies:** Phase 1 complete
- **Done when:** Session appears in database after logging

#### Phase 3: Context Retrieval
- **Deliverable:** Bootstrap can load recent session context
- **Sessions:** 1
- **Dependencies:** Phase 2 complete
- **Done when:** New session starts with awareness of previous sessions

#### Phase 4: Learning Persistence
- **Deliverable:** Can create and query learnings
- **Sessions:** 1-2
- **Dependencies:** Phase 3 complete
- **Done when:** Learning from session N visible in session N+1

### Dependency Map

```
Phase 1 ─blocks→ Phase 2 ─blocks→ Phase 3 ─blocks→ Phase 4
```

### Milestones

| Milestone | After Phase | Demonstrates | Enables |
|-----------|-------------|--------------|---------|
| Database Live | 1 | Schema applied, CLI works | Building on infrastructure |
| First Memory | 2 | Session logged, visible on query | Sessions aren't ephemeral |
| Contextual Start | 3 | Bootstrap shows recent activity | Continuity across instances |
| Learning System | 4 | Learnings persist | Wisdom accumulates |

### Resource Estimate

- **Total sessions:** 4-6
- **Calendar time:** 2-3 weeks (assuming 2 sessions/week)
- **Confidence:** Medium
- **Uncertainty drivers:** TerminusDB learning curve, schema iteration

### Risks

| Risk | Category | L/I | Mitigation | Trigger |
|------|----------|-----|------------|---------|
| TerminusDB complexity | Technical | M/H | Start minimal, expand | Phase 1 > 2 sessions |
| Schema changes | Technical | M/M | Design for evolution | Many migration needs |
| Session gaps | Resource | M/M | Resumable design | > 1 week between sessions |

### Beads Structure

```
aiandi-xxx: Biographical Memory System (epic)
├── aiandi-xxx.1: Phase 1 - Database Setup
│   ├── Install TerminusDB via Docker
│   └── Apply ops_memory schema
├── aiandi-xxx.2: Phase 2 - Session Logging (blocked by .1)
│   ├── Implement session log start
│   └── Implement session log end
├── aiandi-xxx.3: Phase 3 - Context Retrieval (blocked by .2)
│   └── Integrate context into ops-bootstrap
├── aiandi-xxx.4: Phase 4 - Learning Persistence (blocked by .3)
│   ├── Implement learning create
│   └── Implement learning query
```

### Notes

- Could expand to decision tracking in future phase
- Value graph from goal-recognition is a natural extension
- Consider snapshot-based migrations for schema evolution
```

## Anti-Patterns

| Anti-Pattern | Why It Fails |
|--------------|--------------|
| Phases too large | Can't estimate; can't mark progress |
| Phases too small | Overhead exceeds value; loses forest for trees |
| Missing dependencies | Work starts that can't complete |
| Optimistic estimates | Scope collapse or abandonment |
| No milestones | Can't recognize progress; motivation fades |
| Risks unidentified | Surprised when predictable things happen |
| Design without goal | Building machinery without knowing what it's for |
| Skipping Beads creation | Design stays abstract; execution unclear |

## The Deeper Point

Goals without projects are wishes. Projects without structure are chaos.

Project design is the discipline of taking something you care about (a goal) and making it real (a project) through honest decomposition. It's not overhead—it's the difference between "I want X" and "Here's how I'll get X."

The value isn't the plan. The value is the understanding that creates the plan. When you've truly designed a project, you understand the goal more deeply than before you started.

And when the plan changes—which it will—you'll know why and how to adapt.

---

*From intention to structure. From structure to action.*
