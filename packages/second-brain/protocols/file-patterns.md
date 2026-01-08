# File Patterns

Standard file naming conventions for vault operations.

## Core Format

```
{type}-{YYYY-MM-DD}[-{HHMM}]-{slug}.md
```

## Temporal Artifacts

Files that are timestamped and accumulate over time.

| Type | Pattern | Location |
|------|---------|----------|
| Capture | `capture-YYYY-MM-DD-HHmmss-slug.md` | `00-inbox/` |
| Dump | `dump-YYYY-MM-DD-HHMM-slug.md` | `00-inbox/dumps/` |
| Extract | `extract-NNN-slug.md` | `00-inbox/alembic/` |
| Candidate | `candidate-NNN-slug.md` | `00-inbox/integration/` |
| Plan | `plan-YYYY-MM-DD.md` | `archives/daily-plans/` |
| Packet | `packet-YYYY-MM-DD-HHMM-slug.md` | `archives/intelligence/sessions/` |
| Intel | `intel-YYYY-MM-DD.md` | `archives/intelligence/intel/` |
| Audit | `audit-YYYY-MM-DD.md` | `archives/intelligence/audit/` |
| Debrief | `debrief-YYYY-MM-DD.md` | `archives/intelligence/checkins/` |
| Spec | `spec-YYYY-MM-DD-slug.md` | `archives/design/` |
| ADR | `adr-YYYY-MM-DD-slug.md` | `archives/design/` |

## Structural Files

Files that serve as indexes or containers.

| Type | Pattern | Location |
|------|---------|----------|
| Manifest | `manifest.md` | `00-inbox/alembic/` |
| Ledger | `session-ledger.md` | `_spandaworks/` |
| Framework | `{name}-framework.md` | `05-knowledge/consolidated/` |
| Pattern | `pattern-{slug}.md` | `05-knowledge/patterns/` |

## Evergreen Entities

Files that represent ongoing concepts, not timestamped.

| Type | Pattern | Location |
|------|---------|----------|
| Project | `Title-Case-Name.md` | `01-projects/` |
| Area | `Title-Case-Name.md` | `02-areas/` |
| Person | `Firstname-Lastname.md` | `02-areas/people/` |
| MOC | `MOC - Topic Name.md` | Various |
| Permanent Note | `Title-Case-Name.md` | `permanent-notes/` |

## Task Format

Tasks embedded in project/area/people files:

```markdown
- [ ] Task description 📅 2025-12-15 #context/computer #time/30m
```

### Priority Markers

- `high:` prefix - Urgent, deadline <7 days, blocks work
- (no marker) - Normal priority
- `#someday` tag - Eventually, when time permits

## Quality Gates

### Permanent Notes

- Atomic (one idea)
- Connected (3+ links)
- Own words
- Use Cases section for transmissible notes

### Projects

- Has next action
- Updated <7 days

### Inbox

- Empty after processing
