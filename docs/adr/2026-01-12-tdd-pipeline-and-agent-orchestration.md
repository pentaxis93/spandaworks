# ADR: TDD Pipeline and Agent Orchestration System

**Date:** 2026-01-12  
**Status:** Accepted  
**Context:** Implementing test-driven development with multi-agent coordination

---

## Context

aiandi's agent framework initially had 5 agents (explore, researcher, builder, reviewer, documenter) with routing logic embedded in the governance skill. This created two problems:

1. **No structural TDD enforcement** - The "test-driven" constraint on builder was instructional only. Nothing prevented skipping tests or writing implementation first.

2. **Mixed concerns in governance** - The governance skill contained both deliberation protocols (WHAT) and agent routing details (HOW), violating separation of concerns.

Research into agentic TDD implementations (AlexZan/agentic-tdd, Anthropic's agent patterns) revealed that effective TDD requires **structural enforcement through workflow**, not just instructions.

## Decision

### 1. Implement Journey-to-BDD Pipeline

Add three new agents that enforce TDD structurally:

| Agent | Model | Function | Structural Enforcement |
|-------|-------|----------|------------------------|
| `story` | Sonnet | Requirements → BDD specs | Cannot write code, only Gherkin |
| `test-writer` | Sonnet | BDD → failing tests | Cannot run tests (no bash) |
| `validator` | Sonnet | Execute tests, verify phases | Cannot modify code (no write/edit) |

**Pipeline flow:**
```
story → test-writer → validator (RED) → builder → validator (GREEN) → reviewer
```

### 2. Separate Orchestration from Governance

Create two peer skills with distinct responsibilities:

| Skill | Domain | Contents |
|-------|--------|----------|
| **governance** | WHAT (deliberation) | Sovereignty principles, session protocols, evolution system |
| **orchestration** | HOW (coordination) | Agent catalog, routing trees, TDD pipeline, workflows |

The `/governance` command now loads both skills.

### 3. Expand Builder Delegation

Update builder agent to delegate appropriately:
- Model: `claude-opus-4-5` (was Sonnet)
- Can delegate to: `explore`, `researcher`, `documenter`
- Cannot delegate to: `reviewer` (prevents self-review anti-pattern)

## TDD Pipeline Details

### Journey-to-BDD Methodology

Based on research into user journey mapping, INVEST criteria, and BDD:

1. **Story agent** produces specifications grounded in:
   - Persona context (who, why, emotional state)
   - INVEST-compliant user stories
   - Gherkin scenarios (Given/When/Then)

2. **Test-writer agent** transforms Gherkin into failing tests:
   - Cannot run tests (structural enforcement of RED)
   - Must follow project test conventions
   - Documents expected failure reasons

3. **Validator agent** verifies phase transitions:
   - RED mode: Tests fail for right reason (not syntax errors)
   - GREEN mode: All tests pass
   - REGRESSION mode: Existing tests still pass

### Structural Enforcement

| Problem | How Enforced |
|---------|--------------|
| Skipping RED phase | Validator must confirm failure before builder runs |
| Tests mirror implementation | Story agent writes specs before implementation exists |
| Self-review | Builder cannot delegate to reviewer |
| Vague requirements | Story agent surfaces ambiguity as open questions |

### Decision Tree for TDD

```
Is the requirement clear and testable?
  NO → story agent (or ask human)
  YES ↓

Use TDD workflow?
  NO → builder (direct implementation)
  YES ↓

story → test-writer → validator (RED)
  FAIL (wrong reason) → return to test-writer
  PASS ↓

builder → validator (GREEN)
  FAIL → return to builder
  PASS ↓

reviewer (optional refactoring)
```

## Agent Catalog (8 total)

| Agent | Model | Leaf? | Delegates To |
|-------|-------|-------|--------------|
| `explore` | Haiku | Yes | none |
| `researcher` | Sonnet | No | explore |
| `story` | Sonnet | No | explore, researcher |
| `test-writer` | Sonnet | No | explore |
| `validator` | Sonnet | Yes | none |
| `builder` | Opus | No | explore, researcher, documenter |
| `reviewer` | Sonnet | Yes | none |
| `documenter` | Sonnet | Yes | none |

## Consequences

### Positive

✓ **Structural TDD** - Pipeline enforces test-first through workflow, not willpower  
✓ **Clean separation** - Governance deliberates, orchestration coordinates  
✓ **Journey-grounded specs** - Requirements include user context, not just features  
✓ **Executable specifications** - Gherkin output bridges story → test seamlessly  
✓ **Anti-pattern prevention** - Self-review blocked structurally  

### Negative

✗ **Increased complexity** - 8 agents vs 5, two skills vs one  
✗ **Pipeline overhead** - Full TDD adds 4+ agent invocations per feature  
✗ **Learning curve** - Contributors must understand orchestration patterns  

### Neutral

- TDD is on-demand (governance decides when to use it)
- Non-TDD workflows still available (direct builder invocation)
- Story agent optional when requirements already include BDD specs

## Files Changed

```
.opencode/
├── agent/
│   ├── builder.md      (updated: Opus, expanded delegation)
│   ├── story.md        (new)
│   ├── test-writer.md  (new)
│   └── validator.md    (new)
├── command/
│   └── governance.md   (updated: loads orchestration skill)
└── skill/
    ├── governance/
    │   └── SKILL.md    (v4.0: delegates routing to orchestration)
    └── orchestration/
        └── SKILL.md    (new: agent routing, TDD pipeline)
```

## References

### Primary Sources
- Dan North, "Introducing BDD": https://dannorth.net/blog/introducing-bdd/
- Bill Wake, INVEST criteria: https://xp123.com/articles/invest-in-good-stories-and-smart-tasks/
- Nielsen Norman Group, Journey Mapping: https://www.nngroup.com/articles/journey-mapping-101/

### Implementation References
- AlexZan/agentic-tdd: Multi-agent TDD pipeline pattern
- Gojko Adzic, "Specification by Example": https://gojko.net/books/specification-by-example/
- Cucumber BDD documentation: https://cucumber.io/docs/bdd/

### Related PRs
- PR #27: feat(agents): TDD pipeline and orchestration system

---

**Key insight:** TDD discipline through workflow structure is more reliable than TDD discipline through instructions. The pipeline makes "skip the tests" structurally impossible, not just discouraged.
