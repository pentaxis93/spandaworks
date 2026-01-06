# Routing Rules

GTD-aligned routing for inbox processing.

## Auto-Routing (No Confirmation Required)

During gtd-process, apply these rules automatically:

### Bare Links

Route to `01-projects/Recursive-Loop.md` as someday task:
```markdown
- [ ] Review: [title or description] - <url> #someday
```

### Fragments

Delete immediately:
- Single words, incomplete captures, obvious noise

### Media Recommendations

Route to `03-resources/Media-Queue.md`:
- Movies, TV shows, books, articles
- Record recommender as `[[Person]]` link

## Standard Routing

| Item Type | Destination |
|-----------|-------------|
| Actionable + deadline | Project task with date |
| Actionable + ongoing | Area task |
| Actionable + person | People note task |
| Knowledge/insight | Permanent note |
| Reference only | `03-resources/reference-notes/` |
| Someday/maybe | Task with `#someday` tag |

## Priority Assignment

| Indicator | Criteria |
|-----------|----------|
| `high:` prefix | Urgent, deadline <7 days, blocks work, top goals |
| (none) | Normal priority |
| `#someday` | Eventually, when time permits |

## GTD Questions

Apply to each inbox item:

1. **What is it?** (actionable vs reference vs trash)
2. **Is it actionable?**
   - No → Reference, Someday/Maybe, or Trash
   - Yes → Continue
3. **What's the next action?**
4. **Is it a project?** (>1 action required)
5. **Where does it belong?** (project, area, or person)
6. **When?** (deadline, scheduled, or someday)
