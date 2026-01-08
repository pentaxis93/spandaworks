# Session Protocol

Sessions are the container for practice work. They provide bounded context for deep work, distinct from GTD task execution.

## State Machine

```
UNOPENED ─► OPENING ─► OPEN ─► CLOSING ─► CLOSED
               │                   │
               └── (ceremony) ─────┘
```

### States

| State | Description |
|-------|-------------|
| `UNOPENED` | No active session. Default state between sessions. |
| `OPENING` | Ceremony in progress. LBRP executing. |
| `OPEN` | Session active. Work in progress. |
| `CLOSING` | Sealing in progress. Harvest and dedication. |
| `CLOSED` | Session complete. Ready for next UNOPENED. |

### Transitions

| From | To | Trigger | Action |
|------|-----|---------|--------|
| UNOPENED | OPENING | `/open` command | Begin LBRP ceremony |
| OPENING | OPEN | Ceremony complete | Session active |
| OPEN | CLOSING | `/close` command | Begin sealing |
| CLOSING | CLOSED | Sealing complete | Emit session.closed |
| CLOSED | UNOPENED | Automatic | Reset state |

## Session Metadata

```json
{
  "session_id": "2026-01-06-spanda-migration-001",
  "opened_at": "2026-01-06T13:30:00Z",
  "closed_at": null,
  "goal": "Migrate existing repos into Spanda Works monorepo",
  "persona": "Spanda",
  "protocol": "LBRP",
  "human": "Robbie",
  "inherited_count": 42,
  "state": "OPEN"
}
```

### Field Definitions

| Field | Required | Description |
|-------|----------|-------------|
| `session_id` | Yes | Unique identifier (YYYY-MM-DD-slug-NNN) |
| `opened_at` | Yes | ISO 8601 timestamp of session start |
| `closed_at` | No | ISO 8601 timestamp of session end |
| `goal` | Yes | Session intention or purpose |
| `persona` | Yes | Active persona (Spanda, Sage, etc.) |
| `protocol` | Yes | Ceremony used (typically "LBRP") |
| `human` | Yes | Human collaborator name |
| `inherited_count` | Yes | Knowledge state at session start |
| `state` | Yes | Current state in state machine |

## Session vs GTD

Sessions and GTD operate on parallel tracks:

| Aspect | Session | GTD |
|--------|---------|-----|
| Purpose | Practice, deepening, expanding | Getting things done |
| Trigger | `/open` ceremony | Any time |
| Container | Bounded by open/close | Always available |
| Focus | Deep work, reflection | Task completion |
| Tracking | Session metadata | Task status |

**At execution time:** Parallel tracks, no mixing.
**At reflection time:** They inform each other.

## Session ID Format

```
YYYY-MM-DD-slug-NNN
```

Examples:
- `2026-01-06-spanda-migration-001`
- `2026-01-06-deep-work-002`
- `2026-01-06-reflection-001`

The NNN suffix prevents collision when multiple sessions occur on the same day with similar slugs.

## Events

Sessions emit events for cross-package communication:

| Event | When | Payload |
|-------|------|---------|
| `session.opening` | LBRP starts | session_id, goal, persona |
| `session.opened` | Ceremony complete | Full metadata |
| `session.closing` | Sealing starts | session_id |
| `session.closed` | Session ends | Full metadata + harvest |

See `shared/events/session.schema.json` for full schema.
