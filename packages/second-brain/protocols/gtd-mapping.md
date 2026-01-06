# GTD Mapping

How GTD stages map to vault operations.

## Stage Mapping

| GTD Stage | Trigger | Action |
|-----------|---------|--------|
| **Capture** | `/gtd:capture` or "capture this" | Create `00-inbox/capture-*.md` |
| **Clarify** | `/gtd:process-gtd` or "process inbox" | Route items using GTD questions |
| **Organize** | (with clarify) | File to project/area/resource |
| **Reflect** | (with clarify) | Review active projects |
| **Engage** | `/gtd:plan-gtd` or "plan", "orient" | Time-aware orientation |

## Two Operations

Talos implements GTD with two primary operations:

### PROCESS (`/gtd:process-gtd`)

Clarifies and routes inbox items:
1. Read each capture
2. Apply GTD questions
3. Route to appropriate destination
4. Archive or delete source

### PLAN (`/gtd:plan-gtd`)

Time-aware engagement orientation:
- **Morning (05:00-11:59):** Full day planning
- **Afternoon (12:00-16:59):** Remaining runway regroup
- **Evening (17:00-20:59):** Close + tomorrow seed
- **Night (21:00-04:59):** Tomorrow focus

PLAN subsumes CLOSE via temporal awareness.

## Idempotence

Both operations are fully idempotent:
- Run anytime, any day
- Skipped days aren't failures
- Each invocation orients to now

## Task Location

Tasks live ONLY in project/area/people files. Never duplicate.

Daily plans use Obsidian Tasks plugin queries to aggregate:
```markdown
\`\`\`tasks
not done
due before tomorrow
\`\`\`
```

Checking a task in a query updates the source file.

**NEVER write inline tasks in daily plans.** Use only query blocks.

## Inbox Lifecycle

```
capture → process → archive
```

Processed items move to `archives/sources/`. Master Disc Principle: original captures are preserved permanently.

## Session vs GTD

| Context | Purpose | Example |
|---------|---------|---------|
| **Session** | Fruit-bearing practice | /open ceremony, deep work, reflection |
| **GTD** | Getting things done | Daily review, task completion, inbox processing |

At **execution time:** parallel tracks, no mixing.
At **reflection time:** they inform each other.
