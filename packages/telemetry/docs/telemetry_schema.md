# Spanda Telemetry - OpenTelemetry Schema

**Version:** 1.0  
**Date:** 2026-01-05  
**Thread:** spandaworks-telemetry-ontology

---

## Overview

This document specifies the OpenTelemetry attribute schema for passive telemetry capture. This is the "physiology" stream—heartbeat, respiration, metabolic data—that complements the "psychology" stream (journal entries processed by Graphiti).

**Two Streams, One Graph:**
- **Telemetry (this document):** Passive, automatic, structured events
- **Journal (Graphiti):** Active, intentional, natural language

Both streams contribute to the same Kuzu graph, but through different paths.

---

## Design Principles

1. **OTEL GenAI Conventions First:** Use emerging semantic conventions where they exist
2. **spanda.* Namespace:** Custom attributes under `spandaworks.*` prefix
3. **Append-Only Storage:** JSONL format, never mutate historical events
4. **Minimal Real-Time Requirements:** Batch processing acceptable
5. **Privacy Aware:** No PII, no secrets, no prompt content by default

---

## Storage Specification

### Location
```
~/.spandaworks/telemetry/
├── events.jsonl          # Primary event stream
├── events.jsonl.1        # Rotated archives
├── events.jsonl.2
└── ...
```

### Format
JSONL (JSON Lines) - one event per line, append-only.

### Rotation
- Rotate at 100MB or daily, whichever comes first
- Keep 30 days of history
- Compress rotated files with gzip

### Event Structure
```json
{
  "timestamp": "2026-01-05T19:30:00.000Z",
  "event_type": "session.tool_call",
  "trace_id": "abc123...",
  "span_id": "def456...",
  "attributes": {
    "gen_ai.request.model": "claude-3-opus-20240229",
    "spandaworks.session.id": "2026-01-05-ontology-design",
    "spandaworks.tool.name": "read",
    "spandaworks.tool.success": true
  }
}
```

---

## Event Types

### Session Lifecycle Events

#### `session.start`
Emitted when a session begins (after LBRP completion).

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Unique session identifier |
| `spandaworks.session.goal` | string | no | Declared session goal |
| `spandaworks.session.persona` | string | no | Active persona (Spanda, Sage, etc.) |
| `spandaworks.session.protocol` | string | no | Opening protocol used (LBRP, etc.) |
| `spandaworks.session.human` | string | no | Human collaborator |
| `spandaworks.session.inherited_count` | int | no | Number of entities inherited at open |

#### `session.end`
Emitted when a session closes.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Session identifier |
| `spandaworks.session.duration_seconds` | int | yes | Total session duration |
| `spandaworks.session.token_count` | int | no | Total tokens used |
| `spandaworks.session.goal_achieved` | boolean | no | Whether goal was achieved |
| `spandaworks.session.insights_produced` | int | no | Count of insights |
| `spandaworks.session.frictions_logged` | int | no | Count of friction points |

#### `session.state_change`
Emitted when operational state changes.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Session identifier |
| `spandaworks.state.from` | string | no | Previous state |
| `spandaworks.state.to` | string | yes | New state |
| `spandaworks.state.category` | string | yes | cognitive, resource, flow, alignment |
| `spandaworks.state.trigger` | string | no | What caused the change |

---

### Tool Usage Events

#### `session.tool_call`
Emitted for each tool invocation.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Session identifier |
| `spandaworks.tool.name` | string | yes | Tool name (bash, read, etc.) |
| `spandaworks.tool.success` | boolean | yes | Whether call succeeded |
| `spandaworks.tool.duration_ms` | int | no | Execution time |
| `spandaworks.tool.error_type` | string | no | Error category if failed |

---

### LLM Interaction Events

Using OpenTelemetry GenAI Semantic Conventions where applicable.

#### `gen_ai.request`
Emitted for each LLM request.

| Attribute | Type | Required | Source |
|-----------|------|----------|--------|
| `gen_ai.system` | string | yes | OTEL - "anthropic" |
| `gen_ai.request.model` | string | yes | OTEL - Model identifier |
| `gen_ai.request.max_tokens` | int | no | OTEL - Max output tokens |
| `gen_ai.request.temperature` | float | no | OTEL - Sampling temperature |
| `gen_ai.operation.name` | string | yes | OTEL - "chat" |
| `spandaworks.session.id` | string | yes | Custom - Session context |
| `spandaworks.request.purpose` | string | no | Custom - What this request is for |

#### `gen_ai.response`
Emitted for each LLM response.

| Attribute | Type | Required | Source |
|-----------|------|----------|--------|
| `gen_ai.response.model` | string | yes | OTEL - Actual model used |
| `gen_ai.usage.input_tokens` | int | yes | OTEL - Prompt tokens |
| `gen_ai.usage.output_tokens` | int | yes | OTEL - Completion tokens |
| `gen_ai.response.finish_reason` | string | no | OTEL - end_turn, max_tokens, etc. |
| `spandaworks.session.id` | string | yes | Custom - Session context |
| `spandaworks.response.latency_ms` | int | no | Custom - Response time |
| `spandaworks.context.pressure` | float | no | Custom - Token utilization ratio (0.0-1.0) |

---

### Knowledge Events

#### `knowledge.insight`
Emitted when an insight is captured.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Session identifier |
| `spandaworks.insight.id` | string | yes | Insight identifier |
| `spandaworks.insight.domain` | string | no | Domain classification |
| `spandaworks.insight.confidence` | float | no | Confidence level |
| `spandaworks.insight.source` | string | no | What triggered the insight |

#### `knowledge.observation`
Emitted when an observation (pre-insight fragment) is captured.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Session identifier |
| `spandaworks.observation.id` | string | yes | Observation identifier |
| `spandaworks.observation.domain` | string | no | Domain classification |

#### `knowledge.friction`
Emitted when a friction point is logged.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Session identifier |
| `spandaworks.friction.id` | string | yes | Friction identifier |
| `spandaworks.friction.category` | string | yes | tooling, conceptual, process, environmental, relational |
| `spandaworks.friction.recurrence` | boolean | no | Whether this is a recurring friction |

#### `knowledge.pattern_detected`
Emitted when a pattern is recognized.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Session identifier |
| `spandaworks.pattern.id` | string | yes | Pattern identifier |
| `spandaworks.pattern.name` | string | yes | Pattern name |
| `spandaworks.pattern.status` | string | no | emerging, confirmed, deprecated |
| `spandaworks.pattern.occurrence_count` | int | no | Times observed |

---

### Reflection Events

#### `reflection.triggered`
Emitted when meta-cognitive reflection occurs.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.session.id` | string | yes | Session identifier |
| `spandaworks.reflection.id` | string | yes | Reflection identifier |
| `spandaworks.reflection.trigger` | string | no | What prompted reflection |
| `spandaworks.reflection.type` | string | no | query_result, pattern_notice, session_close |

---

### Goal Events

#### `goal.created`
Emitted when a new goal is established.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.goal.id` | string | yes | Goal identifier |
| `spandaworks.goal.scope` | string | yes | session, project, ongoing |
| `spandaworks.goal.parent_id` | string | no | Parent goal if hierarchical |

#### `goal.status_change`
Emitted when goal status changes.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spandaworks.goal.id` | string | yes | Goal identifier |
| `spandaworks.goal.from_status` | string | yes | Previous status |
| `spandaworks.goal.to_status` | string | yes | New status (active, achieved, abandoned, superseded) |
| `spandaworks.goal.reason` | string | no | Reason for change |

---

## Attribute Reference

### Standard OTEL GenAI Attributes Used

| Attribute | Type | Description |
|-----------|------|-------------|
| `gen_ai.system` | string | AI system provider ("anthropic") |
| `gen_ai.request.model` | string | Model identifier |
| `gen_ai.request.max_tokens` | int | Maximum output tokens |
| `gen_ai.request.temperature` | float | Sampling temperature |
| `gen_ai.operation.name` | string | Operation type ("chat") |
| `gen_ai.response.model` | string | Actual model used |
| `gen_ai.usage.input_tokens` | int | Prompt token count |
| `gen_ai.usage.output_tokens` | int | Completion token count |
| `gen_ai.response.finish_reason` | string | Why generation stopped |

### Custom spanda.* Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `spandaworks.session.id` | string | Session identifier |
| `spandaworks.session.goal` | string | Declared session goal |
| `spandaworks.session.persona` | string | Active persona |
| `spandaworks.session.protocol` | string | Protocol being followed |
| `spandaworks.session.human` | string | Human collaborator |
| `spandaworks.session.duration_seconds` | int | Session duration |
| `spandaworks.session.token_count` | int | Total tokens in session |
| `spandaworks.session.inherited_count` | int | Entities inherited at open |
| `spandaworks.session.goal_achieved` | boolean | Whether goal was met |
| `spandaworks.session.insights_produced` | int | Insight count |
| `spandaworks.session.frictions_logged` | int | Friction count |
| `spandaworks.tool.name` | string | Tool identifier |
| `spandaworks.tool.success` | boolean | Tool call success |
| `spandaworks.tool.duration_ms` | int | Tool execution time |
| `spandaworks.tool.error_type` | string | Error category |
| `spandaworks.state.from` | string | Previous operational state |
| `spandaworks.state.to` | string | New operational state |
| `spandaworks.state.category` | string | State category |
| `spandaworks.state.trigger` | string | What caused state change |
| `spandaworks.context.pressure` | float | Token utilization ratio |
| `spandaworks.request.purpose` | string | Purpose of LLM request |
| `spandaworks.response.latency_ms` | int | LLM response time |
| `spandaworks.insight.id` | string | Insight identifier |
| `spandaworks.insight.domain` | string | Insight domain |
| `spandaworks.insight.confidence` | float | Insight confidence |
| `spandaworks.insight.source` | string | What triggered insight |
| `spandaworks.observation.id` | string | Observation identifier |
| `spandaworks.observation.domain` | string | Observation domain |
| `spandaworks.friction.id` | string | Friction identifier |
| `spandaworks.friction.category` | string | Friction category |
| `spandaworks.friction.recurrence` | boolean | Is recurring friction |
| `spandaworks.pattern.id` | string | Pattern identifier |
| `spandaworks.pattern.name` | string | Pattern name |
| `spandaworks.pattern.status` | string | Pattern status |
| `spandaworks.pattern.occurrence_count` | int | Pattern occurrences |
| `spandaworks.reflection.id` | string | Reflection identifier |
| `spandaworks.reflection.trigger` | string | What prompted reflection |
| `spandaworks.reflection.type` | string | Type of reflection |
| `spandaworks.goal.id` | string | Goal identifier |
| `spandaworks.goal.scope` | string | Goal scope |
| `spandaworks.goal.parent_id` | string | Parent goal |
| `spandaworks.goal.from_status` | string | Previous goal status |
| `spandaworks.goal.to_status` | string | New goal status |
| `spandaworks.goal.reason` | string | Reason for status change |

---

## Processing Pipeline

```
┌─────────────────┐
│ Spanda Session  │
└────────┬────────┘
         │ Events emitted during operation
         ▼
┌─────────────────┐
│   Event Sink    │
│  (append JSONL) │
└────────┬────────┘
         │ Batch processing (idle time)
         ▼
┌─────────────────┐
│  Event Processor│
│  - Parse JSONL  │
│  - Extract data │
│  - Update Kuzu  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Kuzu Graph    │
│  (Session nodes,│
│   USED edges,   │
│   etc.)         │
└─────────────────┘
```

### Processing Notes

1. **Session nodes** created from `session.start` / `session.end` pairs
2. **USED relationships** aggregated from `session.tool_call` events
3. **EXPERIENCED_STATE relationships** derived from `session.state_change` events
4. **Token metrics** aggregated from `gen_ai.response` events
5. **Insights/Frictions** from `knowledge.*` events cross-referenced with Graphiti extractions

---

## Privacy Considerations

### What We Capture
- Session metadata (timing, token counts)
- Tool usage patterns
- State transitions
- Knowledge artifact references (IDs, not content)

### What We Do NOT Capture
- Prompt content (handled by Graphiti if journal entries)
- Response content (handled by Graphiti if journal entries)
- File paths containing PII
- Secrets or credentials
- Human-entered text (only classification metadata)

### Content Handling
All actual content (insights, observations, friction descriptions) goes through the **journal stream** (Graphiti), not telemetry. Telemetry captures **that** something happened and **metadata about it**, not **what** was said.

---

## Example Events

### Session Start
```json
{
  "timestamp": "2026-01-05T17:15:00.000Z",
  "event_type": "session.start",
  "trace_id": "sess-2026-01-05-ontology",
  "attributes": {
    "spandaworks.session.id": "2026-01-05-spanda-ontology-design",
    "spandaworks.session.goal": "Design Spanda's self-model ontology",
    "spandaworks.session.persona="assistant",
    "spandaworks.session.protocol": "LBRP",
    "spandaworks.session.human": "user",
    "spandaworks.session.inherited_count": 42
  }
}
```

### Tool Call
```json
{
  "timestamp": "2026-01-05T17:20:15.123Z",
  "event_type": "session.tool_call",
  "trace_id": "sess-2026-01-05-ontology",
  "span_id": "tool-read-001",
  "attributes": {
    "spandaworks.session.id": "2026-01-05-spanda-ontology-design",
    "spandaworks.tool.name": "read",
    "spandaworks.tool.success": true,
    "spandaworks.tool.duration_ms": 45
  }
}
```

### LLM Response
```json
{
  "timestamp": "2026-01-05T17:25:30.456Z",
  "event_type": "gen_ai.response",
  "trace_id": "sess-2026-01-05-ontology",
  "span_id": "llm-resp-015",
  "attributes": {
    "gen_ai.system": "anthropic",
    "gen_ai.response.model": "claude-sonnet-4-20250514",
    "gen_ai.usage.input_tokens": 45000,
    "gen_ai.usage.output_tokens": 2500,
    "gen_ai.response.finish_reason": "end_turn",
    "spandaworks.session.id": "2026-01-05-spanda-ontology-design",
    "spandaworks.response.latency_ms": 3200,
    "spandaworks.context.pressure": 0.47
  }
}
```

### Friction Logged
```json
{
  "timestamp": "2026-01-05T18:10:00.000Z",
  "event_type": "knowledge.friction",
  "trace_id": "sess-2026-01-05-ontology",
  "attributes": {
    "spandaworks.session.id": "2026-01-05-spanda-ontology-design",
    "spandaworks.friction.id": "friction-2026-01-05-001",
    "spandaworks.friction.category": "conceptual",
    "spandaworks.friction.recurrence": false
  }
}
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-05 | Initial specification |
