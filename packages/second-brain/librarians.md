# Second-Brain Librarian Instructions

Domain-specific instructions for librarians operating on knowledge infrastructure.

See `shared/librarians/core.md` for universal protocols.

## Synthesizer

### Domain Actions

- **Fuse pending candidates:** Process `00-inbox/integration/` candidates
- **Consolidate frameworks:** Merge related frameworks in `05-knowledge/consolidated/`
- **Pattern synthesis:** Create pattern notes from recurring insights
- **Alembic processing:** Fuse `ready-for-fusion` alembic extracts

### Triggers

- Candidate count > 5
- Alembic extracts marked `ready-for-fusion`
- Weekly review cycle

### Output

```json
{
  "librarian": "Synthesizer",
  "domain": "second-brain",
  "action": "fuse_candidates",
  "processed_count": 3,
  "created": ["framework-x.md"],
  "merged": ["existing-note.md"]
}
```

## Guardian

### Domain Actions

- **Link validation:** Verify all `[[wikilinks]]` resolve
- **Orphan detection:** Find notes with 0 incoming links
- **Quality gate checks:** Permanent notes have 3+ links
- **Stale project detection:** Projects not updated in 7+ days
- **Inbox hygiene:** Flag items older than 7 days

### Triggers

- Daily maintenance run
- Post-fusion validation
- Weekly review cycle

### Output

```json
{
  "librarian": "Guardian",
  "domain": "second-brain",
  "action": "validate_links",
  "broken_links": 2,
  "orphan_notes": 5,
  "stale_projects": 1
}
```

## Pathfinder

### Domain Actions

- **Connection opportunities:** Suggest links between unconnected notes
- **Framework gaps:** Identify topics without frameworks
- **Pattern detection:** Find recurring themes across notes
- **Evolution suggestions:** Propose structural improvements

### Triggers

- Post-fusion analysis
- Weekly review cycle
- Manual invocation

### Output

```json
{
  "librarian": "Pathfinder",
  "domain": "second-brain",
  "action": "suggest_connections",
  "suggestions": [
    {"from": "note-a.md", "to": "note-b.md", "reason": "shared concept X"}
  ]
}
```

## Integration with Weekly Review

During debrief/weekly review:

1. Guardian runs first (validation)
2. Synthesizer processes pending items
3. Pathfinder suggests improvements
4. Report aggregated in debrief output
