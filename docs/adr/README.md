# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records for the Spandaworks monorepo.

## What is an ADR?

An ADR documents a significant architectural decision along with its context and consequences. It creates a permanent record of **why** something was done, not just **what** was done.

## Format

Each ADR follows this structure:

```markdown
# ADR: [Title]

**Date:** YYYY-MM-DD
**Status:** [Proposed | Accepted | Deprecated | Superseded]
**Context:** [Brief description of situation]
**Decision:** [What was decided]

## Context
[Full description of the problem/situation]

## Decision
[The decision and its rationale]

## Consequences
[Positive, negative, and neutral outcomes]

## References
[Related sessions, PRs, issues]
```

## Naming Convention

`YYYY-MM-DD-brief-topic.md`

Example: `2026-01-07-monorepo-architecture-exploration.md`

## When to Create an ADR

Create an ADR when:
- Significant architectural patterns are discovered
- Major refactoring decisions are made
- Polyglot choices are evaluated
- Integration patterns are established
- Technical debt is accepted or remediated

## ADR List

| Date | Title | Status |
|------|-------|--------|
| 2026-01-07 | [Monorepo Architecture Exploration](2026-01-07-monorepo-architecture-exploration.md) | Accepted |

---

**Principle:** Future instances should understand the reasoning, not just the result.
