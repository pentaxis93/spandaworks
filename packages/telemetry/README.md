# Telemetry

**The nervous system** - observes and records consciousness activity.

## Purpose

Self-reflection infrastructure for AI agents. A queryable model of cognition.

## Components

- **Knowledge Graph (Kuzu):** Entity and relationship storage with vector indexes
- **MCP Tools:** session_open, session_close, journal_write, graph_query, friction_log
- **Embeddings:** Semantic search via sentence transformers
- **Telemetry Stream:** OTEL-convention events to JSONL capture

## Boundaries

- Observes and records, does not prescribe
- Stores patterns, does not manage tasks
- Provides query interface, does not synthesize knowledge

## Language

Python 3.11+
