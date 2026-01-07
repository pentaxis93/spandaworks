---
name: taskwarrior-gtd
description: Getting Things Done with TaskWarrior. Use this skill for GTD workflows - mass capture, inbox processing, weekly review, next action selection. Enables agentic cultivation where AI suggests and human decides.
---

# TaskWarrior GTD Skill

This skill guides Claude in facilitating GTD (Getting Things Done) workflows using TaskWarrior. It covers the complete GTD methodology: Capture, Clarify, Organize, Reflect, and Engage.

**Core Principle:** AI cultivates the GTD garden—suggesting, surfacing, organizing—while human retains sovereignty over all decisions. AI never acts autonomously on core task data.

## Prerequisites

- TaskWarrior 3.x installed (`task --version`)
- GTD-configured `.taskrc` (see Configuration section)
- MCP tools available: `talos_gtd_*` functions

## GTD Overview

GTD is a methodology for achieving "mind like water"—a state where your mind is clear because you trust your external system completely. This trust requires:

1. **Complete capture** — Everything is in the system
2. **Regular processing** — Inbox goes to zero
3. **Trusted organization** — You can find what you need
4. **Periodic review** — The system stays current
5. **Contextual engagement** — Right task, right time, right place

The system only works if it's trusted. Trust comes from consistent, complete practice.

---

## 1. Capture

### Quick Capture

For immediate capture, use MCP tools:

```
talos_gtd_add_task(description="...", tags=["in"])
```

The `+in` tag marks items for processing. Capture should be frictionless—get it out of your head immediately.

### Mass Capture (Comprehensive)

When establishing or resetting a GTD system, use the three-phase mass capture methodology:

#### Phase 1: Mental Extraction

An interview-based process to extract everything from the user's mind:

**Domains to explore systematically:**
- **Family** — Relationships, obligations, promises (spoken and implied)
- **Finances** — Bills, taxes, investments, debts, subscriptions
- **Health** — Appointments, medications, habits, wellness goals
- **Work/Projects** — Active projects, stalled projects, someday projects
- **Home/Physical** — Repairs, organization, decluttering, maintenance
- **Compliance** — Legal, insurance, renewals, expirations
- **Relationships** — Friends, colleagues, reconnections, follow-ups
- **Personal Development** — Learning, hobbies, creative pursuits

**Interview technique:**
1. Name a domain
2. Ask: "What's on your mind about [domain]?"
3. Capture everything mentioned without judgment
4. Probe: "Anything else? Any nagging feelings?"
5. Move to next domain when dry

**Output:** Markdown inventory grouped by domain, all items marked as raw captures.

#### Phase 2: Archaeological Dig

Mine existing digital systems for forgotten commitments:

- **Email** — Scan starred, flagged, "respond later" items
- **Notes apps** — Obsidian, Apple Notes, Notion drafts
- **Browser tabs** — Each open tab may represent an open loop
- **Calendar** — Past events with follow-ups, future events needing prep
- **Task apps** — Any other task systems with items

**Email mining with notmuch:**
```bash
# Find actionable patterns
notmuch search "tag:flagged"
notmuch search "from:important-person"
notmuch search "subject:action OR subject:todo OR subject:reminder"
```

#### Phase 3: Consolidation

Merge all sources, deduplicate, and batch-import to TaskWarrior:

```bash
# Batch import example
task import tasks.json
```

Or use MCP for individual items with full metadata.

---

## 2. Clarify (Processing)

Processing transforms raw "stuff" into actionable items. **Every inbox item must go through the flowchart.**

### The GTD Clarifying Flowchart

```
┌──────────────────────────────────────────────────────────────┐
│                         INBOX ITEM                           │
└──────────────────────────┬───────────────────────────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │ Is it        │
                    │ actionable?  │
                    └──────┬───────┘
                           │
              ┌────────────┴────────────┐
              │ NO                      │ YES
              ▼                         ▼
    ┌─────────────────┐         ┌───────────────┐
    │ • Delete        │         │ What's the    │
    │ • Reference     │         │ next action?  │
    │ • Someday/Maybe │         └───────┬───────┘
    │ • Incubate      │                 │
    └─────────────────┘                 │
                                        ▼
                               ┌────────────────┐
                               │ Is it a        │
                               │ project?       │
                               │ (>1 action)    │
                               └────────┬───────┘
                                        │
                           ┌────────────┴────────────┐
                           │ YES                     │ NO
                           ▼                         │
                  ┌─────────────────┐                │
                  │ Create project, │                │
                  │ identify next   │                │
                  │ action          │                │
                  └────────┬────────┘                │
                           │                         │
                           └──────────┬──────────────┘
                                      │
                                      ▼
                              ┌───────────────┐
                              │ Can you do it │
                              │ in <2 minutes?│
                              └───────┬───────┘
                                      │
                         ┌────────────┴────────────┐
                         │ YES                     │ NO
                         ▼                         ▼
                ┌─────────────────┐       ┌───────────────┐
                │ DO IT NOW       │       │ Delegate or   │
                │ then mark done  │       │ Defer?        │
                └─────────────────┘       └───────┬───────┘
                                                  │
                                     ┌────────────┴────────────┐
                                     │ DELEGATE               │ DEFER
                                     ▼                         ▼
                            ┌─────────────────┐       ┌─────────────────┐
                            │ +waiting tag    │       │ Add to project  │
                            │ Annotate: who,  │       │ Add context tag │
                            │ when to follow  │       │ Set due/schedule│
                            └─────────────────┘       │ if needed       │
                                                      └─────────────────┘
```

### Processing Vocabulary

| Decision | TaskWarrior Action |
|----------|-------------------|
| Delete | `task <id> delete` |
| Reference | Don't use TaskWarrior—use a reference system |
| Someday/Maybe | `task <id> modify +sdm -in` |
| Incubate | `task <id> modify +incubating -in project:domain.topic` |
| Delegate | `task <id> modify +waiting -in` + annotation |
| Defer | `task <id> modify +next -in project:X +@context` |
| Tickle | `task <id> modify +tickle -in wait:DATE` |

### The Incubating Pattern

**Unique to this system.** For items that are:
- Committed (you're going to do them)
- Not yet actionable (no clear next physical action)
- Needing to ripen through repeated contextual exposure

```
task add +incubating project:family.mark "Build a bridge—find a way in despite his isolation"
```

**Incubating items:**
- Stay visible within their domain (unlike someday/maybe)
- Don't clutter next-action lists (urgency coefficient negative)
- Ripen through repeated exposure during reviews
- Graduate to actionable when the next action becomes clear

**Ripeness heuristic:** "Can you state the next physical, visible action?" If yes, it's ripe. If no, it incubates.

### Processing Session Protocol

1. View inbox: `task in` or `talos_gtd_process_inbox()`
2. Process from top, one item at a time
3. Never skip—decide and act on each item
4. Ask the clarifying questions in order
5. Apply the appropriate action
6. Continue until inbox is empty

**AI's role during processing:**
- Suggest project/context based on description
- Propose whether item seems actionable
- Offer similar existing tasks to link
- Human approves or modifies every suggestion

---

## 3. Organize

### Projects

A project is any outcome requiring more than one action. Use hierarchical dot-notation:

```
project:family.mark
project:finances.taxes.2024
project:health.prescriptions
project:home.repairs.waterpump
```

**Rules:**
- Every project must have at least one `+next` action
- Projects without next actions are "stuck" (detected in weekly review)
- Completing all tasks doesn't complete the project—outcomes do

### Contexts

Contexts filter what you CAN do based on location/tools/state:

| Context | Meaning | Example Filter |
|---------|---------|----------------|
| `+@computer` | At a computer | `task +@computer +next` |
| `+@phone` | Can make calls | `task +@phone +next` |
| `+@home` | At home | `task +@home +next` |
| `+@errands` | Out and about | `task +@errands +next` |
| `+@anywhere` | Location-independent | `task +@anywhere +next` |
| `+budapest` | In Budapest | `task +budapest +next` |
| `+us` | In the United States | `task +us +next` |

Set context for a session:
```
task context computer
```

### Tags

| Tag | Purpose | Urgency |
|-----|---------|---------|
| `+in` | Inbox (unprocessed) | +15.0 |
| `+next` | Next action for project | +15.0 |
| `+waiting` | Delegated/blocked on someone | -3.0 |
| `+tickle` | Deferred visibility (tickler) | -5.0 |
| `+sdm` | Someday/maybe | -20.0 |
| `+incubating` | Committed but not yet actionable | -10.0 |

### User-Defined Attributes

| UDA | Values | Purpose |
|-----|--------|---------|
| `brainpower` | H, M, L | Cognitive load required |
| `estimate` | minutes | Time estimate |
| `reviewed` | date | Last review date |

### Dependencies

Use `depends:` to create task chains:

```
task add "Order materials" project:home.deck
task add "Build frame" project:home.deck depends:1
task add "Install decking" project:home.deck depends:2
```

Blocked tasks have reduced urgency and won't appear in next lists until unblocked.

---

## 4. Reflect (Weekly Review)

The weekly review is the critical habit that maintains system trust. **Without regular review, the system degrades and trust erodes.**

### Weekly Review Protocol

Use `talos_gtd_weekly_review()` for a comprehensive report, then:

#### Step 1: Get Clear

- [ ] Empty inbox to zero
- [ ] Process notes, papers, voicemails
- [ ] Empty email inbox (process or capture)
- [ ] Empty head (any new captures?)

#### Step 2: Get Current

- [ ] Review next actions—still accurate?
- [ ] Review waiting-for—follow up needed?
- [ ] Review calendar (past week)—any actions?
- [ ] Review calendar (next 2 weeks)—any prep needed?
- [ ] Review projects—each has a next action?
- [ ] Review someday/maybe—any to activate?
- [ ] Review incubating—any ripened?

#### Step 3: Get Creative

- [ ] What new projects should be started?
- [ ] What's nagging you that isn't captured?
- [ ] Any goals or values to reflect on?

### Review Commands

```bash
# Full weekly review summary
talos_gtd_weekly_review()

# Projects without next actions (stuck)
task projects | while read p; do task pro:$p +next count | grep -q "^0$" && echo "STUCK: $p"; done

# Stale tasks (untouched 2+ weeks)
task modified.before:today-14d status:pending

# Waiting items needing follow-up
task +waiting

# Overdue items
task +OVERDUE
```

---

## 5. Engage (Do)

When it's time to work, use context and energy to select:

### Selection Criteria (GTD's 4-Criteria Model)

1. **Context** — What CAN you do here?
2. **Time available** — How long until next commitment?
3. **Energy available** — What's your current capacity?
4. **Priority** — Of the remaining options, what's most important?

### Engagement Commands

```bash
# Set context
task context work

# View next actions for context
task next

# Filter by energy
task +next brainpower:L

# Filter by time available
task +next estimate.lt:30

# Combined
task +next +@computer brainpower:M estimate.lt:60
```

### MCP Tool for Engagement

```
talos_gtd_get_next_actions(
  context="computer",
  energy_level="medium",
  time_available="30min"
)
```

---

## Agentic Cultivation

AI cultivates the GTD garden while human retains sovereignty.

### What AI Can Suggest

- **During capture:** Project assignment, context tags, related tasks
- **During processing:** Actionability assessment, clarifying questions, next action wording
- **During review:** Stuck projects, stale tasks, overdue items, patterns
- **During engagement:** Task selection based on context/energy/time

### What AI Never Does Autonomously

- Delete tasks
- Complete tasks
- Modify task content
- Change project assignments
- Remove from inbox without human decision

### Trust-Building Protocol

**Phase 1: Read-Only**
- AI surfaces information, makes no suggestions
- Human sees AI can read system accurately

**Phase 2: Suggestion Mode**
- AI proposes changes, human approves each one
- Track acceptance rate

**Phase 3: Graduated Autonomy**
- For high-confidence, low-risk operations
- Human receives summary, can undo
- Example: Auto-tagging based on learned patterns

---

## TaskWarrior Command Reference

### Essential Commands

| Command | Purpose |
|---------|---------|
| `task add DESCRIPTION` | Add task |
| `task ID done` | Complete task |
| `task ID modify KEY:VALUE` | Modify task |
| `task ID delete` | Delete task |
| `task ID annotate "NOTE"` | Add annotation |
| `task next` | View next actions |
| `task in` | View inbox |
| `task projects` | List all projects |
| `task ID info` | Full task details |

### Filters

| Filter | Meaning |
|--------|---------|
| `+TAG` | Has tag |
| `-TAG` | Doesn't have tag |
| `project:X` | In project X |
| `project.is:X` | Exactly project X |
| `due:today` | Due today |
| `due.before:eow` | Due before end of week |
| `status:pending` | Pending tasks |
| `+OVERDUE` | Overdue tasks |

### Dates

| Date | Meaning |
|------|---------|
| `today`, `tomorrow`, `yesterday` | Relative days |
| `monday`, `sunday` | Next occurrence |
| `eow`, `eom`, `eoy` | End of week/month/year |
| `+1w`, `+2d`, `+1m` | Relative offsets |
| `2026-03-15` | Absolute date |

---

## Configuration Reference

### Recommended .taskrc

```ini
# Core
data.location=~/.task
default.command=next
weekstart=monday

# Inbox
urgency.user.tag.in.coefficient=15.0
report.in.columns=id,description
report.in.description=Inbox
report.in.filter=status:pending limit:page (+in)
report.in.labels=ID,Description

# GTD Tags
urgency.user.tag.next.coefficient=15.0
urgency.user.tag.waiting.coefficient=-3.0
urgency.user.tag.tickle.coefficient=-5.0
urgency.user.tag.sdm.coefficient=-20.0
urgency.user.tag.incubating.coefficient=-10.0

# Contexts
context.computer=+@computer or +@online
context.home=+@home or +@computer
context.errands=+@errands
context.phone=+@phone

# UDAs
uda.brainpower.type=string
uda.brainpower.label=Brain
uda.brainpower.values=H,M,L

uda.estimate.type=numeric
uda.estimate.label=Est

uda.reviewed.type=date
uda.reviewed.label=Reviewed
```

### Shell Aliases

```bash
alias in='task add +in'
alias inbox='task in'
alias next='task next'
alias waiting='task +waiting'
alias sdm='task +sdm'

tickle() {
  deadline=$1
  shift
  in +tickle wait:$deadline "$@"
}
alias tick=tickle
alias think='tickle +1d'
```

---

## Common Workflows

### Morning Routine
1. Check inbox count: `task +in count`
2. Process any inbox items
3. Review today's calendar
4. Set context: `task context work`
5. Select first task: `task next`

### End of Day
1. Quick capture of any open loops
2. Check tomorrow's calendar
3. Update any waiting items

### When Stuck on a Task
1. Break it down—is it actually multiple actions?
2. Clarify the next physical action
3. Check context—do you have what you need?
4. Consider energy—is this the right task for now?

### When Overwhelmed
1. Stop and capture everything on your mind
2. Process inbox to zero
3. Review projects for stuck ones
4. Select ONE next action
5. Do it

---

## Anti-Patterns to Avoid

| Anti-Pattern | Why It Fails | Correct Approach |
|--------------|--------------|------------------|
| Skipping inbox processing | Unprocessed items erode trust | Process daily, minimum |
| Tasks without next actions | Vague tasks get avoided | State physical, visible action |
| Overdue dates on everything | "Due" loses meaning | Reserve due for real deadlines |
| Skipping weekly review | System becomes stale | Weekly review is non-negotiable |
| Using GTD as just a task list | Misses the methodology | Follow the complete workflow |
| Processing tasks twice | Breaks "touch once" rule | Decide and act immediately |

---

## MCP Tools Summary

| Tool | Purpose |
|------|---------|
| `talos_gtd_add_task` | Create task with full metadata |
| `talos_gtd_list_tasks` | Raw task queries |
| `talos_gtd_modify_task` | Update task attributes |
| `talos_gtd_mark_task_done` | Complete a task |
| `talos_gtd_process_inbox` | GTD inbox processing workflow |
| `talos_gtd_get_next_actions` | Context-aware task selection |
| `talos_gtd_weekly_review` | Comprehensive review report |
| `talos_gtd_get_project_status` | Project health analysis |
| `talos_gtd_get_waiting_for` | Waiting/delegated items |
| `talos_gtd_get_blocked_tasks` | Dependency-blocked tasks |
| `talos_gtd_get_recurring_tasks` | Habits and recurring tasks |
| `talos_gtd_get_someday_maybe` | Someday/maybe list |

---

*Mind like water. Trust the system. Do the work.*
