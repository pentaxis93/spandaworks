# Vault Template

Standard PARA structure for Obsidian vault with Spanda Works operational directories.

## Structure

```
vault/
├── 00-inbox/           # Captures, dumps, alembic, integration candidates
│   ├── dumps/          # Raw thought captures
│   ├── alembic/        # Pre-fusion staging
│   └── integration/    # Fusion candidates
├── 01-projects/        # Active projects with end state
├── 02-areas/           # Ongoing responsibilities
│   └── people/         # People notes
├── 03-resources/       # Reference materials
│   └── reference-notes/
├── 05-knowledge/       # Knowledge assets
│   ├── consolidated/   # Frameworks from fusion
│   └── patterns/       # Recognized patterns
├── _spanda/            # Spanda Works operational memory
│   ├── reference/      # Quick reference system
│   ├── protocols/      # Operational protocols
│   ├── evolution/      # System evolution tracking
│   └── remembrance.md  # Self-definition
├── archives/           # Historical records
│   ├── daily-plans/
│   ├── intelligence/
│   ├── design/         # ADRs and specs
│   └── sources/        # Processed captures
├── permanent-notes/    # Atomic evergreen notes
└── templates/          # Note templates
```

## PARA Distinction

| Directory | Type | Lifecycle |
|-----------|------|-----------|
| 00-inbox | Capture | Transient (process → archive) |
| 01-projects | Project | Finite (complete → archive) |
| 02-areas | Area | Infinite (ongoing) |
| 03-resources | Resource | Reference (stable) |
| 05-knowledge | Knowledge | Evergreen (grows) |

## Projects vs Areas

- **Project:** Has end state or deadline (finite)
- **Area:** Ongoing responsibility (infinite)

## File Patterns

See `../protocols/file-patterns.md` for naming conventions.
