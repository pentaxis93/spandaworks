# GTD Cheat Sheet

Quick reference for spanda-gtd. Print this page.

---

## The Five Stages

| Stage | Question | Action |
|-------|----------|--------|
| **Capture** | What's on your mind? | `in "task description"` |
| **Clarify** | Is it actionable? | Process with `ti` → decide |
| **Organize** | Where does it belong? | Assign project, context, tags |
| **Reflect** | Is the system current? | Daily review, weekly review |
| **Engage** | What do I do now? | `tn` → pick by context/energy |

---

## Tags

### GTD Status
| Tag | Meaning |
|-----|---------|
| `+in` | Inbox (unprocessed) |
| `+next` | Next action (ready) |
| `+sdm` | Someday/Maybe |
| `+waiting` | Waiting for external |
| `+tickle` | Future inbox item |

### Contexts
| Tag | When to use |
|-----|-------------|
| `+@computer` | Need a computer |
| `+@online` | Need internet |
| `+@phone` | Can make calls |
| `+@home` | At home |
| `+@work` | At office |
| `+@errands` | Out and about |
| `+@anywhere` | No location needed |
| `+@focus` | Deep work time |

---

## Shell Aliases

### Capture
```
in "text"     → Add to inbox
tick 3d "x"   → Tickler (appears in 3 days)
think "x"     → Tomorrow's inbox
rnd "x"       → Research task
```

### View
```
ti            → Inbox
tn            → Next actions
tw            → Waiting for
ts            → Someday/Maybe
tnna          → Projects without next action
```

### Context Switch
```
tcw           → Work context
tch           → Home context
tcf           → Focus context
tca           → Anywhere context
tcn           → Clear context
```

### Process
```
process 1 pro:project +@context   → Inbox → Next action
defer 1                            → Inbox → Someday
delegate 1                         → Inbox → Waiting
```

---

## Common TaskWarrior Commands

```bash
task add "description"        # Add task
task 1 modify pro:project     # Set project
task 1 modify +tag            # Add tag
task 1 modify -tag            # Remove tag
task 1 modify due:friday      # Set due date
task 1 modify wait:monday     # Hide until Monday
task 1 done                   # Complete task
task 1 delete                 # Delete task
```

---

## MCP Tools Quick Reference

| GTD Stage | Tool | Purpose |
|-----------|------|---------|
| Capture | `add_task` | Create new task |
| Clarify | `process_inbox` | Get inbox with prompts |
| Organize | `modify_task` | Update task attributes |
| Reflect | `weekly_review` | Full review summary |
| Engage | `get_next_actions` | Context-filtered actions |

### Other Key Tools
| Tool | Purpose |
|------|---------|
| `get_waiting_for` | External blockers |
| `get_blocked_tasks` | Internal dependencies |
| `get_project_status` | Project health |
| `get_someday_maybe` | Deferred items |
| `get_recurring_tasks` | Habits & streaks |

---

## Daily Workflow

**Morning (5 min)**
1. `ti` - Empty inbox or defer
2. `tn` - Review next actions
3. Pick 3 Most Important Tasks

**During Day**
1. `in "capture"` - Capture immediately
2. `tcX` - Switch context as needed
3. `tn` - What's next?

**Evening (2 min)**
1. `ti` - Clear inbox
2. Quick tomorrow preview

---

## Weekly Review Checklist

1. **Get Clear**
   - [ ] `ti` - Process inbox to zero
   - [ ] Collect loose papers/notes

2. **Get Current**
   - [ ] `tnna` - Add next actions to stalled projects
   - [ ] `tw` - Follow up on waiting items
   - [ ] `ts` - Review someday/maybe

3. **Get Creative**
   - [ ] Review goals and projects
   - [ ] Capture new ideas

---

## Clarify Decision Tree

```
Is it actionable?
├─ NO
│  ├─ Trash it → task X delete
│  ├─ Someday → defer X
│  └─ Reference → file elsewhere
│
└─ YES
   ├─ < 2 minutes? → Do it now
   ├─ Delegate? → delegate X
   ├─ Multiple steps? → Create project
   └─ Single action → process X pro:... +@...
```

---

*spanda-gtd: AI suggests, human approves*
