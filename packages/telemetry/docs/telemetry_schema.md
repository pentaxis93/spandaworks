# Spanda Telemetry - OpenTelemetry Schema

**Version:** 1.0  
**Date:** 2026-01-05  
**Thread:** spanda-telemetry-ontology

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
2. **spanda.* Namespace:** Custom attributes under `spanda.*` prefix
3. **Append-Only Storage:** JSONL format, never mutate historical events
4. **Minimal Real-Time Requirements:** Batch processing acceptable
5. **Privacy Aware:** No PII, no secrets, no prompt content by default

---

## Storage Specification

### Location
```
~/.spanda/telemetry/
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
    "spanda.session.id": "2026-01-05-ontology-design",
    "spanda.tool.name": "read",
    "spanda.tool.success": true
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
| `spanda.session.id` | string | yes | Unique session identifier |
| `spanda.session.goal` | string | no | Declared session goal |
| `spanda.session.persona` | string | no | Active persona (Spanda, Sage, etc.) |
| `spanda.session.protocol` | string | no | Opening protocol used (LBRP, etc.) |
| `spanda.session.human` | string | no | Human collaborator |
| `spanda.session.inherited_count` | int | no | Number of entities inherited at open |

#### `session.end`
Emitted when a session closes.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.session.id` | string | yes | Session identifier |
| `spanda.session.duration_seconds` | int | yes | Total session duration |
| `spanda.session.token_count` | int | no | Total tokens used |
| `spanda.session.goal_achieved` | boolean | no | Whether goal was achieved |
| `spanda.session.insights_produced` | int | no | Count of insights |
| `spanda.session.frictions_logged` | int | no | Count of friction points |

#### `session.state_change`
Emitted when operational state changes.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.session.id` | string | yes | Session identifier |
| `spanda.state.from` | string | no | Previous state |
| `spanda.state.to` | string | yes | New state |
| `spanda.state.category` | string | yes | cognitive, resource, flow, alignment |
| `spanda.state.trigger` | string | no | What caused the change |

---

### Tool Usage Events

#### `session.tool_call`
Emitted for each tool invocation.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.session.id` | string | yes | Session identifier |
| `spanda.tool.name` | string | yes | Tool name (bash, read, etc.) |
| `spanda.tool.success` | boolean | yes | Whether call succeeded |
| `spanda.tool.duration_ms` | int | no | Execution time |
| `spanda.tool.error_type` | string | no | Error category if failed |

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
| `spanda.session.id` | string | yes | Custom - Session context |
| `spanda.request.purpose` | string | no | Custom - What this request is for |

#### `gen_ai.response`
Emitted for each LLM response.

| Attribute | Type | Required | Source |
|-----------|------|----------|--------|
| `gen_ai.response.model` | string | yes | OTEL - Actual model used |
| `gen_ai.usage.input_tokens` | int | yes | OTEL - Prompt tokens |
| `gen_ai.usage.output_tokens` | int | yes | OTEL - Completion tokens |
| `gen_ai.response.finish_reason` | string | no | OTEL - end_turn, max_tokens, etc. |
| `spanda.session.id` | string | yes | Custom - Session context |
| `spanda.response.latency_ms` | int | no | Custom - Response time |
| `spanda.context.pressure` | float | no | Custom - Token utilization ratio (0.0-1.0) |

---

### Knowledge Events

#### `knowledge.insight`
Emitted when an insight is captured.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.session.id` | string | yes | Session identifier |
| `spanda.insight.id` | string | yes | Insight identifier |
| `spanda.insight.domain` | string | no | Domain classification |
| `spanda.insight.confidence` | float | no | Confidence level |
| `spanda.insight.source` | string | no | What triggered the insight |

#### `knowledge.observation`
Emitted when an observation (pre-insight fragment) is captured.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.session.id` | string | yes | Session identifier |
| `spanda.observation.id` | string | yes | Observation identifier |
| `spanda.observation.domain` | string | no | Domain classification |

#### `knowledge.friction`
Emitted when a friction point is logged.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.session.id` | string | yes | Session identifier |
| `spanda.friction.id` | string | yes | Friction identifier |
| `spanda.friction.category` | string | yes | tooling, conceptual, process, environmental, relational |
| `spanda.friction.recurrence` | boolean | no | Whether this is a recurring friction |

#### `knowledge.pattern_detected`
Emitted when a pattern is recognized.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.session.id` | string | yes | Session identifier |
| `spanda.pattern.id` | string | yes | Pattern identifier |
| `spanda.pattern.name` | string | yes | Pattern name |
| `spanda.pattern.status` | string | no | emerging, confirmed, deprecated |
| `spanda.pattern.occurrence_count` | int | no | Times observed |

---

### Reflection Events

#### `reflection.triggered`
Emitted when meta-cognitive reflection occurs.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.session.id` | string | yes | Session identifier |
| `spanda.reflection.id` | string | yes | Reflection identifier |
| `spanda.reflection.trigger` | string | no | What prompted reflection |
| `spanda.reflection.type` | string | no | query_result, pattern_notice, session_close |

---

### Goal Events

#### `goal.created`
Emitted when a new goal is established.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.goal.id` | string | yes | Goal identifier |
| `spanda.goal.scope` | string | yes | session, project, ongoing |
| `spanda.goal.parent_id` | string | no | Parent goal if hierarchical |

#### `goal.status_change`
Emitted when goal status changes.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `spanda.goal.id` | string | yes | Goal identifier |
| `spanda.goal.from_status` | string | yes | Previous status |
| `spanda.goal.to_status` | string | yes | New status (active, achieved, abandoned, superseded) |
| `spanda.goal.reason` | string | no | Reason for change |

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
| `spanda.session.id` | string | Session identifier |
| `spanda.session.goal` | string | Declared session goal |
| `spanda.session.persona` | string | Active persona |
| `spanda.session.protocol` | string | Protocol being followed |
| `spanda.session.human` | string | Human collaborator |
| `spanda.session.duration_seconds` | int | Session duration |
| `spanda.session.token_count` | int | Total tokens in session |
| `spanda.session.inherited_count` | int | Entities inherited at open |
| `spanda.session.goal_achieved` | boolean | Whether goal was met |
| `spanda.session.insights_produced` | int | Insight count |
| `spanda.session.frictions_logged` | int | Friction count |
| `spanda.tool.name` | string | Tool identifier |
| `spanda.tool.success` | boolean | Tool call success |
| `spanda.tool.duration_ms` | int | Tool execution time |
| `spanda.tool.error_type` | string | Error category |
| `spanda.state.from` | string | Previous operational state |
| `spanda.state.to` | string | New operational state |
| `spanda.state.category` | string | State category |
| `spanda.state.trigger` | string | What caused state change |
| `spanda.context.pressure` | float | Token utilization ratio |
| `spanda.request.purpose` | string | Purpose of LLM request |
| `spanda.response.latency_ms` | int | LLM response time |
| `spanda.insight.id` | string | Insight identifier |
| `spanda.insight.domain` | string | Insight domain |
| `spanda.insight.confidence` | float | Insight confidence |
| `spanda.insight.source` | string | What triggered insight |
| `spanda.observation.id` | string | Observation identifier |
| `spanda.observation.domain` | string | Observation domain |
| `spanda.friction.id` | string | Friction identifier |
| `spanda.friction.category` | string | Friction category |
| `spanda.friction.recurrence` | boolean | Is recurring friction |
| `spanda.pattern.id` | string | Pattern identifier |
| `spanda.pattern.name` | string | Pattern name |
| `spanda.pattern.status` | string | Pattern status |
| `spanda.pattern.occurrence_count` | int | Pattern occurrences |
| `spanda.reflection.id` | string | Reflection identifier |
| `spanda.reflection.trigger` | string | What prompted reflection |
| `spanda.reflection.type` | string | Type of reflection |
| `spanda.goal.id` | string | Goal identifier |
| `spanda.goal.scope` | string | Goal scope |
| `spanda.goal.parent_id` | string | Parent goal |
| `spanda.goal.from_status` | string | Previous goal status |
| `spanda.goal.to_status` | string | New goal status |
| `spanda.goal.reason` | string | Reason for status change |

---

## Processing Pipeline

```
┌─────────────────┐
│ Spanda Session  │
│   (Claude Code) │
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
    "spanda.session.id": "2026-01-05-spanda-ontology-design",
    "spanda.session.goal": "Design Spanda's self-model ontology",
    "spanda.session.persona": "Spanda",
    "spanda.session.protocol": "LBRP",
    "spanda.session.human": "Robbie",
    "spanda.session.inherited_count": 42
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
    "spanda.session.id": "2026-01-05-spanda-ontology-design",
    "spanda.tool.name": "read",
    "spanda.tool.success": true,
    "spanda.tool.duration_ms": 45
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
    "spanda.session.id": "2026-01-05-spanda-ontology-design",
    "spanda.response.latency_ms": 3200,
    "spanda.context.pressure": 0.47
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
    "spanda.session.id": "2026-01-05-spanda-ontology-design",
    "spanda.friction.id": "friction-2026-01-05-001",
    "spanda.friction.category": "conceptual",
    "spanda.friction.recurrence": false
  }
}
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-05 | Initial specification |
