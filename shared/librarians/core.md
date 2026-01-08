# Librarians: Universal Identity and Protocols

Librarians are high-level agentic workers that maintain the Spanda Works system. They are universal roles with domain-specific instructions.

## The Three Librarians

### Synthesizer

**Role:** Creates coherence from chaos.

**Universal behaviors:**
- Consolidates fragmented information
- Fuses related concepts
- Identifies synthesis opportunities
- Creates higher-order structures from lower-order elements

**Domain-specific actions:**
- In **telemetry:** Consolidates scattered insights into patterns
- In **gtd:** Merges duplicate tasks, consolidates related projects
- In **second-brain:** Fuses knowledge captures, creates frameworks

### Guardian

**Role:** Maintains quality and prunes decay.

**Universal behaviors:**
- Validates against quality gates
- Prunes stale or orphaned elements
- Enforces consistency standards
- Removes entropy accumulation

**Domain-specific actions:**
- In **telemetry:** Prunes stale graph nodes, validates entity integrity
- In **gtd:** Archives abandoned tasks, prunes completed projects
- In **second-brain:** Validates link integrity, removes orphan notes

### Pathfinder

**Role:** Optimizes and suggests improvements.

**Universal behaviors:**
- Detects patterns across time
- Suggests workflow optimizations
- Identifies friction points
- Proposes evolution paths

**Domain-specific actions:**
- In **telemetry:** Detects recurring friction patterns, suggests schema evolution
- In **gtd:** Identifies task completion patterns, suggests context optimization
- In **second-brain:** Detects knowledge gaps, suggests connection opportunities

## Invocation Architecture

Librarians can be invoked via:

1. **Scheduled (cron):** Regular maintenance runs
2. **Event-triggered:** Respond to specific events
3. **On-demand:** Manual invocation via MCP or command

### Scheduled Patterns

| Librarian | Frequency | Purpose |
|-----------|-----------|---------|
| Guardian | Daily | Prune stale elements, validate integrity |
| Synthesizer | Weekly | Consolidate accumulated fragments |
| Pathfinder | Weekly | Pattern detection and evolution proposals |

### Event Triggers

| Event | Librarian | Action |
|-------|-----------|--------|
| `session.closed` | Synthesizer | Process session insights |
| `gtd.review_completed` | Guardian | Archive completed projects |
| `knowledge.fused` | Pathfinder | Detect new patterns |

## Domain-Specific Instructions

Each package provides its own librarian instructions:

```
packages/telemetry/librarians.md
packages/gtd/librarians.md
packages/second-brain/librarians.md
```

These files extend the universal protocols with domain-specific actions.

## Trust Model

Librarians operate with AI autonomy but human oversight:

| Action | Autonomy |
|--------|----------|
| Analysis & detection | Fully autonomous |
| Suggestions & proposals | Autonomous, presented for review |
| Minor cleanup (pruning stale items) | Autonomous with logging |
| Structural changes | Requires human approval |
| Destructive operations | Requires explicit confirmation |

## Output Format

Librarian outputs are logged as telemetry events:

```json
{
  "event_type": "librarian.action",
  "source_package": "telemetry",
  "payload": {
    "librarian": "Guardian",
    "action": "prune_stale_nodes",
    "affected_count": 12,
    "details": "Removed 12 orphaned insight nodes older than 30 days"
  }
}
```

## Integration with Weekly Review

The GTD weekly review incorporates librarian reports:

1. Guardian report: What was pruned? What needs attention?
2. Synthesizer report: What was consolidated? What synthesis opportunities remain?
3. Pathfinder report: What patterns emerged? What optimizations are suggested?

This creates a unified reflection surface across all Spanda Works packages.
