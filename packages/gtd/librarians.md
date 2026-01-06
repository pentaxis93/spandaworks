# GTD Librarian Instructions

Domain-specific instructions for librarians operating on task management.

See `shared/librarians/core.md` for universal protocols.

## Synthesizer

### Domain Actions

- **Merge duplicate tasks:** Consolidate similar tasks
- **Project consolidation:** Merge related small projects
- **Tag normalization:** Standardize tag usage

### Triggers

- Weekly review cycle
- Project count > threshold

### Output

```json
{
  "librarian": "Synthesizer",
  "domain": "gtd",
  "action": "merge_duplicates",
  "merged_count": 3
}
```

## Guardian

### Domain Actions

- **Archive completed projects:** Move to archive after completion
- **Prune abandoned tasks:** Flag tasks with no activity 30+ days
- **Orphan task detection:** Tasks without project assignment
- **Overdue task alerts:** Highlight past-due items
- **Waiting-for stale check:** External blockers older than 14 days

### Triggers

- Daily maintenance
- Weekly review cycle

### Output

```json
{
  "librarian": "Guardian",
  "domain": "gtd",
  "action": "prune_abandoned",
  "flagged_count": 5,
  "archived_projects": 2
}
```

## Pathfinder

### Domain Actions

- **Completion pattern detection:** When/how tasks get done
- **Context optimization:** Suggest context improvements
- **Energy pattern analysis:** Match tasks to energy levels
- **Project health assessment:** Identify stalled projects

### Triggers

- Weekly review cycle
- Monthly retrospective

### Output

```json
{
  "librarian": "Pathfinder",
  "domain": "gtd",
  "action": "analyze_patterns",
  "insights": [
    {"pattern": "Most completions happen morning", "recommendation": "Schedule hard tasks AM"}
  ]
}
```

## Integration with Weekly Review

The GTD weekly review incorporates librarian actions:

1. Guardian: Archive completed, flag stale
2. Synthesizer: Consolidate duplicates
3. Pathfinder: Pattern analysis and suggestions

Report surfaced in `weekly_review` tool output.
