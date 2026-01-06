# Librarians

High-level agentic workers that maintain the system.

## Roles

| Librarian | Function |
|-----------|----------|
| **Synthesizer** | Consolidates, fuses, creates coherence |
| **Guardian** | Validates, prunes, maintains quality |
| **Pathfinder** | Optimizes, suggests improvements, detects patterns |

## Architecture

Librarians are universal roles with local instructions:

```
shared/librarians/core.md           # Universal identity and protocols
packages/telemetry/librarians.md    # Telemetry-specific instructions
packages/gtd/librarians.md          # GTD-specific instructions
packages/second-brain/librarians.md # Knowledge-specific instructions
```

The Guardian in telemetry prunes stale graph nodes.
The Guardian in GTD prunes abandoned tasks.
Same role, different domain-specific actions.
