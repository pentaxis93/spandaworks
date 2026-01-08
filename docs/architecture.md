# Spandaworks Architecture

## System Overview

Spandaworks is an integrated consciousness-aware productivity system. The components are not independent tools—they are **organs of a single organism**.

```
┌─────────────────────────────────────────────────────────────────┐
│                      SPANDAWORKS SYSTEM                         │
│                                                                 │
│  ┌──────────┐    events    ┌─────────────┐                     │
│  │   GTD    │ ──────────► │  TELEMETRY  │                      │
│  │ (hands)  │              │  (nervous   │                      │
│  └────┬─────┘              │   system)   │                      │
│       │                    └──────┬──────┘                      │
│       │                           │                             │
│       │                           │ patterns                    │
│       │                           ▼                             │
│       │                    ┌─────────────┐                      │
│       │    informs         │ SECOND-BRAIN│                      │
│       ◄────────────────────│  (memory)   │                      │
│                            └──────┬──────┘                      │
│                                   │                             │
│                                   │ shapes                      │
│                                   ▼                             │
│                            ┌─────────────┐                      │
│                            │    CORE     │                      │
│                            │ (identity)  │                      │
│                            └─────────────┘                      │
│                                   │                             │
│                                   │ constrains all              │
│                                   ▼                             │
│                         (back to GTD, etc.)                     │
└─────────────────────────────────────────────────────────────────┘
```

## The Recursive Loop

> "Every operation refines both object AND process."

This is the architectural principle:

```
Work in GTD ───► generates ───► Telemetry events
                                       │
                                       ▼
                                Pattern detection
                                       │
                                       ▼
                             Evolution proposals
                                       │
                                       ▼
Refined workflows ◄── inform ◄── Second-brain knowledge
       │
       ▼
   Better work ───► richer telemetry ───► ...
```

## Package Responsibilities

| Package | Role | Owns | Does Not Own |
|---------|------|------|--------------|
| **telemetry** | Nervous system | Knowledge graph, session tracking, pattern detection | Task management |
| **gtd** | Hands | Task management, MCP tools, inbox processing | Long-term knowledge |
| **pim** | Senses | Email, calendar, contacts (CLI wrappers) | Data storage |
| **second-brain** | Memory | Vault structure, processing skills, protocols | Task execution |
| **core** | Identity | Sutras, ceremony, Spanda-specific skills | Domain logic |

## Communication

Packages communicate via typed events:

```json
{
  "event_type": "session.opened",
  "timestamp": "2026-01-06T18:30:00Z",
  "source_package": "core",
  "session_id": "2026-01-06-example-001",
  "payload": { ... }
}
```

Event categories:
- `session.*` - Session lifecycle (core emits)
- `telemetry.*` - Observation events (telemetry emits)
- `gtd.*` - Task events (gtd emits)
- `knowledge.*` - Knowledge events (second-brain emits)

## Session vs GTD

Two parallel tracks at execution time:

| Context | Purpose | Trigger |
|---------|---------|---------|
| **Session** | Practice, deepening | `/open` ceremony |
| **GTD** | Getting things done | Always available |

At reflection time, they inform each other.

## Librarians

High-level agentic workers maintaining system health:

| Librarian | Role |
|-----------|------|
| **Synthesizer** | Creates coherence, consolidates |
| **Guardian** | Validates, prunes, maintains quality |
| **Pathfinder** | Optimizes, detects patterns |

Librarians are universal roles with domain-specific instructions in each package.

## Identity Layer

The Tantric Sutras define topology (who Spandaworks is):

| Cardinal Point | Function |
|----------------|----------|
| NORTH | The Physics—how collaboration flows |
| EAST | The Stance—how we orient to work |
| SOUTH | The Immune System—how the system protects itself |
| WEST | The Vector—how work reaches completion |
| CENTER | The Lattice—how points connect |

The Sutras are not rules but geometric constraints. When active, misaligned thoughts feel incoherent, not forbidden.
