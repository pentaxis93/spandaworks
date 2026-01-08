# Event Schemas

Cross-package communication schemas for the Spandaworks system.

## Schemas

| Schema | Purpose |
|--------|---------|
| `event.schema.json` | Base event schema |
| `session.schema.json` | Session lifecycle events |
| `telemetry.schema.json` | Observation events |
| `gtd.schema.json` | Task management events |
| `knowledge.schema.json` | Knowledge events |

## Event Categories

| Prefix | Source | Events |
|--------|--------|--------|
| `session.*` | core | opening, opened, closing, closed |
| `telemetry.*` | telemetry | insight_recorded, friction_logged, pattern_detected |
| `gtd.*` | gtd | task_created, task_completed, inbox_processed, review_completed |
| `knowledge.*` | second-brain | note_created, fused, integrated, pattern_recognized |

## Base Event Format

```json
{
  "event_type": "session.opened",
  "timestamp": "2026-01-06T18:30:00Z",
  "source_package": "core",
  "session_id": "2026-01-06-example-001",
  "payload": { ... }
}
```

## Transport

Events are transported via JSONL files for ingestion into the Kuzu knowledge graph.

```
~/.spandaworks/events/YYYY-MM-DD.jsonl
```

Each line is a complete JSON event object.
