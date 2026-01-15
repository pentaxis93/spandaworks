---
name: orchestration
description: "Agent coordination and workflow patterns for aiandi. Decision trees for agent routing, TDD pipeline, bead-driven orchestration loop, delegation rules and topologies. For agent routing and workflows, see orchestration skill."
version: "1.1"
---

# Orchestration Skill

*Agent coordination and workflow patterns for aiandi*

---

## Purpose

This skill defines **how agents coordinate** to accomplish work. It provides:
- Decision trees for agent routing
- Workflow patterns (including TDD pipeline)
- Delegation rules and topologies

**Relationship to Governance:** Governance decides WHAT to build. Orchestration decides HOW to coordinate agents to build it.

---

## Agent Catalog

### Available Agents

| Agent | Model | Function | Leaf? |
|-------|-------|----------|-------|
| `explore` | Haiku | Fast codebase search, pattern discovery | Yes |
| `researcher` | Sonnet | Web research, external docs, API investigation | No |
| `story` | Sonnet | Requirements → journey-grounded BDD specs | No |
| `test-writer` | Sonnet | BDD specs → failing test code (RED) | No |
| `validator` | Sonnet | Execute tests, validate TDD phases | Yes |
| `builder` | Opus | Code implementation (GREEN) | No |
| `reviewer` | Sonnet | Code review, security audit, quality analysis | Yes |
| `documenter` | Sonnet | Documentation writing, README updates | Yes |

### Delegation Topology

```
Orchestrator (Governance or primary session)
    │
    ├── explore (leaf)
    │
    ├── researcher
    │       └── explore
    │
    ├── story
    │       ├── explore
    │       └── researcher
    │
    ├── test-writer
    │       └── explore
    │
    ├── validator (leaf)
    │
    ├── builder
    │       ├── explore
    │       ├── researcher
    │       └── documenter
    │
    ├── reviewer (leaf)
    │
    └── documenter (leaf)
```

**Leaf agents** cannot delegate further.
**Non-leaf agents** may delegate to their allowed children only.

---

## Routing Decision Tree

Use this tree to determine which agent(s) to invoke:

```
START: What does the task require?
│
├─► Requirements unclear or complex?
│   │
│   ├─► YES: Human available for clarification?
│   │       │
│   │       ├─► YES → Ask human directly
│   │       └─► NO → story agent (produce specification)
│   │
│   └─► NO → Continue to task type
│
├─► Task type?
│   │
│   ├─► Code implementation
│   │   │
│   │   └─► Use TDD workflow?
│   │       │
│   │       ├─► YES → TDD Pipeline (see below)
│   │       └─► NO → builder (direct implementation)
│   │
│   ├─► Code review/analysis
│   │   └─► reviewer
│   │
│   ├─► Documentation only
│   │   └─► documenter
│   │
│   ├─► External research needed
│   │   └─► researcher
│   │
│   ├─► Codebase exploration
│   │   └─► explore
│   │
│   └─► Other/unclear
│       └─► Do directly (no delegation)
```

### When to Use TDD Workflow

**Use TDD when:**
- Implementing new features with clear acceptance criteria
- Fixing bugs (write failing test first)
- Requirements benefit from BDD specification
- Code quality and test coverage are priorities

**Skip TDD when:**
- Trivial changes (typos, config updates)
- Exploratory/spike work
- Documentation-only tasks
- Time-critical hotfixes (but add tests after)

---

## TDD Pipeline

The full test-driven development workflow using the journey-to-BDD pattern.

### Pipeline Stages

```
┌─────────────────────────────────────────────────────────────┐
│ Stage 0: SPECIFICATION (optional)                           │
│ Agent: story                                                │
│ Input: Requirements (possibly vague)                        │
│ Output: BDD specification with Gherkin scenarios            │
│ Skip if: Requirements already include testable criteria     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Stage 1: RED                                                │
│ Agent: test-writer                                          │
│ Input: BDD specification                                    │
│ Output: Test files that should fail                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Stage 2: RED VALIDATION                                     │
│ Agent: validator (mode: RED)                                │
│ Input: Test files                                           │
│ Output: Validation report                                   │
│ Gate: Tests must fail for RIGHT reason                      │
│       - PASS → proceed to Stage 3                           │
│       - FAIL (wrong reason) → return to Stage 1             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Stage 3: GREEN                                              │
│ Agent: builder                                              │
│ Input: Failing tests + specification                        │
│ Output: Implementation that passes tests                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Stage 4: GREEN VALIDATION                                   │
│ Agent: validator (mode: GREEN)                              │
│ Input: Implementation                                       │
│ Output: Validation report                                   │
│ Gate: All tests must pass                                   │
│       - PASS → proceed to Stage 5                           │
│       - FAIL → return to Stage 3                            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Stage 5: REFACTOR (optional)                                │
│ Agent: reviewer                                             │
│ Input: Passing implementation                               │
│ Output: Review with refactoring suggestions                 │
│ If refactoring needed → builder implements → revalidate     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                         COMPLETE
```

### Pipeline Invocation Pattern

```
# Stage 0: Specification (if needed)
mcp_task(
  description: "Create BDD specification",
  prompt: "Create a specification for: [requirements]",
  subagent_type: "story"
)

# Stage 1: Write failing tests
mcp_task(
  description: "Write failing tests",
  prompt: "Write tests for this specification: [spec from story]",
  subagent_type: "test-writer"
)

# Stage 2: Validate RED
mcp_task(
  description: "Validate RED phase",
  prompt: "Validate tests fail correctly. Mode: RED",
  subagent_type: "validator"
)

# Stage 3: Implement
mcp_task(
  description: "Implement to pass tests",
  prompt: "Implement minimal code to pass: [tests]",
  subagent_type: "builder"
)

# Stage 4: Validate GREEN
mcp_task(
  description: "Validate GREEN phase", 
  prompt: "Validate all tests pass. Mode: GREEN",
  subagent_type: "validator"
)

# Stage 5: Review
mcp_task(
  description: "Review implementation",
  prompt: "Review code quality and suggest refactoring: [implementation]",
  subagent_type: "reviewer"
)
```

### Error Handling

| Situation | Action |
|-----------|--------|
| RED validation fails (syntax errors) | Return to test-writer with error details |
| RED validation fails (tests pass unexpectedly) | Return to test-writer—tests are wrong |
| GREEN validation fails | Return to builder with failure details |
| GREEN validation fails after 3 attempts | Escalate to human |
| Regression detected | Return to builder, preserve original test expectations |

---

## Non-TDD Workflows

### Direct Implementation

For tasks that don't warrant full TDD:

```
Requirements → builder → reviewer (optional)
```

### Research-First Pattern

When external knowledge is needed:

```
researcher → [findings] → builder/documenter
```

### Exploration Pattern

Understanding codebase before action:

```
explore → [findings] → story/builder/reviewer
```

---

## Bead-Driven Orchestration Loop

For autonomous execution of a backlog, the orchestrator continuously processes ready work from Beads.

### The Loop

```
┌─────────────────────────────────────────────────────────────┐
│                    ORCHESTRATOR LOOP                        │
│                                                             │
│  while bd ready returns work:                               │
│    1. SELECT:    Pick highest-priority ready bead           │
│    2. ANALYZE:   Determine agent type and workflow          │
│    3. DELEGATE:  Spawn agent with bead context              │
│    4. MONITOR:   Track execution, handle discoveries        │
│    5. VALIDATE:  Verify deliverable meets acceptance        │
│    6. INTEGRATE: Merge work, close bead                     │
│    7. REASSESS:  Loop—closing may unblock new work          │
│                                                             │
│  Termination: No ready work OR human interrupt              │
└─────────────────────────────────────────────────────────────┘
```

### Invocation

```
/open
```

When invoked without a specific goal, the session enters bead-driven mode:

1. LBRP South Quarter queries `bd ready --json`
2. Orchestrator presents ready work for confirmation (optional gate)
3. Upon confirmation, loop begins

Or explicitly request bead-driven execution:

```
/open Process the backlog
```

### Phase Details

#### 1. SELECT

```bash
# Get ready work as JSON
bd ready --json

# Select based on:
# - Priority (P1 > P2 > P3)
# - Dependency position (unblocks others = higher value)
# - Age (older items, all else equal)
```

**Selection heuristics:**
- Prefer beads that unblock many dependents
- Prefer beads with complete specifications (rich description, clear acceptance criteria)
- Defer beads with sparse descriptions until enriched

#### 2. ANALYZE

Determine execution strategy using the Routing Decision Tree:

```
Bead type/content → Agent selection
│
├─► Has BDD specification? → TDD Pipeline
├─► Implementation task? → builder (or TDD if criteria clear)
├─► Research needed? → researcher first
├─► Documentation task? → documenter
├─► Review request? → reviewer
└─► Unclear? → story agent to clarify, then re-route
```

#### 3. DELEGATE

Spawn agent with full bead context:

```python
mcp_task(
  description=f"Execute {bead.id}",
  prompt=f"""
## Task
{bead.title}

## Bead ID
{bead.id}

## Description
{bead.description}

## Acceptance Criteria
{bead.acceptance_criteria}

## Context
- Priority: {bead.priority}
- Dependencies: {bead.dependencies}
- Parent: {bead.parent or 'none'}

## Instructions
1. Execute the task as specified
2. If you discover new work, create beads with `discovered-from:{bead.id}`
3. Report completion status and any blockers found

## Output Format
Return:
- STATUS: complete | blocked | needs-clarification
- DELIVERABLES: [list of artifacts created/modified]
- DISCOVERED: [any new beads created]
- NOTES: [relevant observations]
""",
  subagent_type=selected_agent
)
```

#### 4. MONITOR

While agent executes:
- Track any `bd create` calls (discovered work enters queue automatically)
- Watch for blocking discoveries (`bd dep add X blocks current`)
- Note if agent requests clarification

**Discovered work handling:**
New beads created with `discovered-from` automatically enter the `bd ready` pool when unblocked. No special handling needed—the loop naturally picks them up.

#### 5. VALIDATE

Verify deliverable against acceptance criteria:

```python
def validate_deliverable(bead, agent_output):
    # Parse acceptance criteria from bead
    criteria = parse_acceptance_criteria(bead.description)
    
    # Check each criterion
    results = []
    for criterion in criteria:
        passed = verify_criterion(criterion, agent_output)
        results.append((criterion, passed))
    
    if all(r[1] for r in results):
        return ValidationResult.PASS
    else:
        return ValidationResult.RETRY, failed_criteria
```

**On validation failure:**
1. Send deliverable back to same agent with specific feedback
2. Include: what failed, why, what's needed
3. Retry up to 3 times
4. After 3 failures: mark bead `blocked`, add note, continue loop

```python
if validation_result == RETRY:
    retry_count += 1
    if retry_count > 3:
        bd update bead.id --status blocked
        bd update bead.id --description "BLOCKED: Validation failed 3x. {failure_details}"
        continue  # Move to next bead
    else:
        # Retry with feedback
        mcp_task(
          prompt=f"Previous attempt failed validation. Fix: {failure_details}",
          subagent_type=same_agent
        )
```

#### 6. INTEGRATE

On successful validation:

```bash
# Close the bead
bd update <bead-id> --status closed

# Commit if code changes
git add -A && git commit -m "feat(<bead-id>): <bead-title>"
```

**Integration responsibilities:**
- Ensure code compiles/passes tests
- Resolve any merge conflicts
- Update documentation if needed
- Commit with bead reference for traceability

#### 7. REASSESS

```bash
# Check what's now unblocked
bd ready --json
```

Closing a bead may unblock its dependents. The loop continues with the new ready set.

### Parallel Bead Execution

When multiple ready beads have no dependencies on each other:

```python
ready_beads = bd_ready_json()
independent_beads = find_independent_set(ready_beads)

if len(independent_beads) > 1:
    # Parallel execution
    tasks = [
        mcp_task(description=f"Execute {b.id}", ...)
        for b in independent_beads[:MAX_PARALLEL]
    ]
    # All tasks run concurrently
```

**Constraints:**
- Max parallel agents: 3 (resource limit)
- Must be truly independent (no shared files, no dep relationship)
- Each agent works on isolated bead

### Stopping Conditions

The loop terminates when:

1. **No ready work:** `bd ready` returns empty
2. **Human interrupt:** User sends message during execution
3. **Session time limit:** Optional time-box reached
4. **Consecutive failures:** N beads blocked in a row (system issue)

On termination, report:
```markdown
## Orchestration Complete

**Beads processed:** N
**Beads completed:** X
**Beads blocked:** Y
**Discovered work:** Z new beads created

**Ready queue:** [remaining ready beads, if any]
```

### Example Session

```
/open Process the backlog

◈ STATUS
Git: clean | Branch: main | Last: abc123

◈ GOAL (bead-driven)
Processing ready work from Beads backlog.

◈ S🜂 (Tasks)
Ready: 3 tasks
  [P1] aiandi-a1b2: Implement user authentication
  [P2] aiandi-c3d4: Add error handling to API
  [P2] aiandi-e5f6: Update README with setup instructions

Starting orchestration loop...

─────────────────────────────────────────
BEAD: aiandi-a1b2 (P1)
Agent: builder (TDD pipeline)
Status: ✓ Complete
Discovered: aiandi-g7h8 (session management)
─────────────────────────────────────────
BEAD: aiandi-c3d4 (P2)  
Agent: builder
Status: ✓ Complete
─────────────────────────────────────────
BEAD: aiandi-e5f6 (P2)
Agent: documenter
Status: ✓ Complete
─────────────────────────────────────────
BEAD: aiandi-g7h8 (P2) [discovered]
Agent: builder
Status: ✓ Complete
─────────────────────────────────────────

◈ ORCHESTRATION COMPLETE
Processed: 4 | Completed: 4 | Blocked: 0
Ready queue: empty
```

---

## Parallel Execution

Multiple independent agents can run in parallel:

```
# Good: Independent tasks
mcp_task(subagent_type: "explore", prompt: "Find all API endpoints...")
mcp_task(subagent_type: "researcher", prompt: "Research OAuth 2.0...")

# Bad: Dependent tasks (must be sequential)
# test-writer depends on story output
# builder depends on test-writer output
```

**Rule:** Parallelize only when tasks have no data dependencies.

---

## Transmission Format

When delegating to agents, include:

```markdown
## Task
[Clear description of what to accomplish]

## Context
[Relevant background, prior work, constraints]

## Input
[Artifacts from previous stages, if any]

## Success Criteria
[How to know the task is complete]

## Output Format
[Expected structure of response]
```

---

## Anti-Patterns

### 1. Skipping Validation Stages
**Problem:** Running builder without RED validation
**Why it fails:** Tests might pass trivially or fail for wrong reasons
**Fix:** Always validate between stages

### 2. Over-Delegation
**Problem:** Delegating trivial tasks
**Why it fails:** Overhead exceeds benefit
**Fix:** Do simple tasks directly

### 3. Under-Specification
**Problem:** Invoking builder with vague requirements
**Why it fails:** Builder guesses, often wrong
**Fix:** Use story agent or clarify with human first

### 4. Parallel Dependent Tasks
**Problem:** Running test-writer and builder simultaneously
**Why it fails:** Builder has no tests to implement against
**Fix:** Respect stage dependencies

### 5. Self-Review
**Problem:** Builder delegating to reviewer for its own code
**Why it fails:** Confirmation bias, no fresh perspective
**Fix:** Orchestrator invokes reviewer after builder completes

### 6. Undocumented Emergent Work
**Problem:** Agent discovers work during execution but creates ad-hoc task without linking
**Why it fails:** No audit trail, governance can't see where work originates
**Fix:** Always use `discovered-from` dependency when creating tasks during execution

---

## Emergent Work Protocol

When agents discover new work during execution (bugs, missing features, edge cases), they must:

```bash
# Create task with discovered-from dependency
bd create "Found: [description]" --deps discovered-from:<current-task-id> -p 2

# If it blocks current work
bd dep add <new-task-id> blocks <current-task-id>
```

**Why this matters:**
- Creates audit trail of where work originates
- Governance can see the discovery chain
- Helps estimate scope creep in retrospective
- Prevents orphaned tasks with no context

**Builder agent responsibility:**
When builder encounters issues during GREEN phase that require new work:
1. Create task with `discovered-from` pointing to the task being implemented
2. Decide: Does this block current work or can it be deferred?
3. If blocking: add `blocks` dependency; current task moves to waiting
4. If not blocking: continue current work; new task goes to backlog

---

## Integration with Governance

**Loading:** `/governance` should load both `governance` and `orchestration` skills.

**Separation of concerns:**
- Governance: WHAT to build, WHY it matters, canon/evolution
- Orchestration: HOW to coordinate agents, workflow patterns

**Handoff:** Governance decides "we need to implement feature X with TDD" → Orchestration skill provides the pipeline execution pattern.

---

## References

- TDD methodology: https://dannorth.net/blog/introducing-bdd/
- INVEST criteria: https://xp123.com/articles/invest-in-good-stories-and-smart-tasks/
- Gherkin syntax: https://cucumber.io/docs/gherkin/reference/
- Journey mapping: https://www.nngroup.com/articles/journey-mapping-101/
- Specification by Example: https://gojko.net/books/specification-by-example/

---

*Orchestration enables the coordination. Governance provides the direction. Together they serve the work.*
