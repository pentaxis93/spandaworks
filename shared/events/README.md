# Event Schemas

Cross-package communication schemas.

## Event Categories

- `session.*` - Session lifecycle (core emits)
- `telemetry.*` - Observation events (telemetry emits)
- `gtd.*` - Task events (gtd emits)
- `knowledge.*` - Knowledge events (second-brain emits)

## Schema Format

```json
{
  "$schema": "https://talos.dev/schemas/event.json",
  "event_type": "session.opened",
  "timestamp": "2026-01-06T18:30:00Z",
  "source_package": "core",
  "payload": { ... }
}
```
