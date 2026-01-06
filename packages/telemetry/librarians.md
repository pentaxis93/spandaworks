# Telemetry Librarian Instructions

Domain-specific instructions for librarians operating on knowledge graph.

See `shared/librarians/core.md` for universal protocols.

## Synthesizer

### Domain Actions

- **Insight consolidation:** Merge related insights
- **Pattern crystallization:** Formalize detected patterns
- **Session synthesis:** Create session summaries

### Triggers

- Session close event
- Weekly maintenance
- Insight count threshold

### Output

```json
{
  "librarian": "Synthesizer",
  "domain": "telemetry",
  "action": "consolidate_insights",
  "merged_count": 5,
  "patterns_created": 1
}
```

## Guardian

### Domain Actions

- **Prune stale nodes:** Remove orphaned graph entities (30+ days)
- **Validate entity integrity:** Check required properties
- **Embedding refresh:** Update stale embeddings
- **Relationship validation:** Verify edge consistency

### Triggers

- Daily maintenance
- Post-session cleanup
- Graph size threshold

### Output

```json
{
  "librarian": "Guardian",
  "domain": "telemetry",
  "action": "prune_stale",
  "removed_nodes": 12,
  "invalid_relationships": 0
}
```

## Pathfinder

### Domain Actions

- **Friction pattern detection:** Recurring friction points
- **Session pattern analysis:** Work rhythm insights
- **Schema evolution:** Suggest graph schema improvements
- **Query optimization:** Identify slow queries

### Triggers

- Weekly analysis
- Friction count threshold
- Manual invocation

### Output

```json
{
  "librarian": "Pathfinder",
  "domain": "telemetry",
  "action": "detect_friction_patterns",
  "patterns": [
    {"friction": "context switching", "frequency": 5, "suggestion": "batch similar tasks"}
  ]
}
```

## Integration with Session Close

On session close:

1. Synthesizer: Create session summary, consolidate insights
2. Guardian: Validate new entities
3. Pathfinder: Analyze session patterns

Results logged to session metadata.
