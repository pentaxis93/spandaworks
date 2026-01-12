# Orchestration Skill

*Agent coordination and workflow patterns for aiandi*

**Version:** 1.0
**Domain:** Multi-agent orchestration

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
