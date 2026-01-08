# GTD Workflow with @spandaworks/gtd

A practical guide to implementing Getting Things Done with @spandaworks/gtd.

## The GTD Model

GTD (Getting Things Done) is David Allen's productivity methodology built on five stages:

1. **Capture** - Get everything out of your head
2. **Clarify** - Process what each item means
3. **Organize** - Put items where they belong
4. **Reflect** - Review the system regularly
5. **Engage** - Take action with confidence

@spandaworks/gtd implements this with TaskWarrior as the backend and MCP tools for AI assistance.

---

## Stage 1: Capture

**Goal:** Get everything out of your head into a trusted system.

### Quick Capture (Shell)

```bash
# Basic inbox capture
in "Call dentist about appointment"

# With context hint
in +@phone "Call dentist about appointment"

# Tickler file - appears in inbox on specific date
tick friday "Follow up on proposal"
tick +3d "Check if package arrived"

# Tomorrow's inbox
think "Consider new project idea"

# Research task (auto-tagged for computer/online)
rnd "Research React Server Components"
```

### AI-Assisted Capture

When talking to your AI assistant:
- "Add 'review quarterly goals' to my inbox"
- "Capture: need to call mom about birthday"

The AI uses `add_task` with `tags: ["in"]`.

### Capture Principles

- **Capture everything** - Don't pre-filter
- **Capture quickly** - Minimize friction
- **Capture completely** - Enough to understand later
- **Trust the system** - It will be processed

---

## Stage 2: Clarify

**Goal:** Decide what each inbox item means and what to do with it.

### Processing Workflow

```bash
# View inbox
ti

# For each item, ask:
# 1. What is it?
# 2. Is it actionable?
# 3. What's the next physical action?
```

### Decision Tree

```
Is it actionable?
│
├─ NO
│  ├─ Trash → task X delete
│  ├─ Someday/Maybe → defer X
│  └─ Reference → file elsewhere (not TaskWarrior)
│
└─ YES
   ├─ Less than 2 minutes? → Do it now
   ├─ Should someone else do it? → delegate X
   ├─ Multiple steps required? → Create project
   └─ Single action → process X pro:... +@...
```

### Processing Commands

```bash
# Move from inbox to next action
process 1 pro:work.website +@computer

# Defer to someday/maybe
defer 2

# Delegate (waiting for)
delegate 3

# Delete (not actionable, not needed)
task 4 delete
```

### AI-Assisted Processing

Ask your AI assistant:
- "Process my inbox" → Uses `process_inbox` tool
- AI provides structured clarification prompts
- You make the decisions, AI executes

---

## Stage 3: Organize

**Goal:** Put items in the right place with the right metadata.

### Projects

A project is any outcome requiring more than one action.

```bash
# Add to project
task 1 modify pro:home.renovation

# Project naming convention
# category.specific
# - work.website
# - home.renovation
# - personal.health
```

### Contexts

Contexts indicate where/how you can do the task.

| Context | When to Use |
|---------|-------------|
| `+@computer` | Need a computer |
| `+@online` | Need internet |
| `+@phone` | Can make calls |
| `+@home` | At home |
| `+@work` | At work/office |
| `+@errands` | Out running errands |
| `+@anywhere` | No constraints |
| `+@focus` | Deep work, no interruptions |

```bash
task 1 modify +@computer +@online
```

### Energy Levels

Tag tasks by mental energy required:

```bash
task 1 modify brainpower:H  # High - creative, complex
task 2 modify brainpower:M  # Medium - normal work
task 3 modify brainpower:L  # Low - routine, tired-friendly
```

### Time Estimates

```bash
task 1 modify estimate:30   # 30 minutes
task 2 modify estimate:120  # 2 hours
```

### Dates

```bash
# Due date (hard deadline)
task 1 modify due:friday

# Scheduled (when to start)
task 1 modify scheduled:monday

# Wait (hide until)
task 1 modify wait:+3d

# Until (expires after)
task 1 modify until:2026-01-31
```

---

## Stage 4: Reflect

**Goal:** Keep the system current and trustworthy.

### Daily Review (5 minutes)

**Morning:**
```bash
ti              # Check inbox - process or defer
tn              # Review next actions
# Pick 3 Most Important Tasks for the day
```

**Evening:**
```bash
ti              # Clear inbox before bed
# Quick preview of tomorrow
```

### Weekly Review (30-60 minutes)

The weekly review is the critical habit that makes GTD work.

**Get Clear:**
```bash
ti              # Process inbox to zero
# Collect loose papers, notes, ideas
```

**Get Current:**
```bash
tnna            # Fix projects without next actions
tw              # Follow up on waiting items
ts              # Review someday/maybe - reactivate or remove
```

**Get Creative:**
- Review goals and vision
- Capture new projects/ideas
- Look ahead at calendar

### AI-Assisted Review

```
"Run my weekly review"
```

Uses `weekly_review` tool which returns:
- Inbox count
- Completed this week
- Stalled projects
- Projects without next actions
- Waiting items
- Overdue tasks
- Habit statistics

---

## Stage 5: Engage

**Goal:** Take action with confidence.

### Context Switching

```bash
tcw             # Switch to work context
tch             # Switch to home context
tcf             # Switch to focus context
tca             # Switch to anywhere context
tcn             # Clear context
```

### Selecting Tasks

```bash
tn              # Next actions (respects active context)
```

**Selection criteria (in order):**
1. **Context** - What can I do here?
2. **Time available** - How long do I have?
3. **Energy** - What's my mental state?
4. **Priority** - What's most important?

### AI-Assisted Engagement

```
"What should I work on? I have 30 minutes and I'm at my computer"
```

Uses `get_next_actions` with context and time filters.

---

## Common Workflows

### Morning Routine

```bash
ti              # Process overnight inbox
tn              # Review next actions
# Identify top 3 priorities
# Block calendar if needed
```

### Processing a Busy Inbox

```bash
ti              # View inbox
# For each item:
#   - 2 min rule: do it now
#   - Route: process, defer, delegate, or delete
#   - Don't organize yet, just clarify
```

### Starting a New Project

```bash
# Option 1: Manual
task add "Project: Renovate bathroom" pro:home.renovation
task add "Research contractors" pro:home.renovation +next +@computer
task add "Get quotes" pro:home.renovation +@phone

# Option 2: AI-assisted (uses create_project_tree)
"Create a project for bathroom renovation with tasks for research, quotes, and scheduling"
```

### Handling Waiting Items

```bash
delegate 1      # Mark as waiting, set follow-up
tw              # Check waiting list
task 5 annotate "Emailed Bob, awaiting response"
```

### Using Someday/Maybe

```bash
defer 1         # Move to someday
ts              # Review someday list

# Reactivate
task 10 modify -sdm +next +@computer
```

---

## Tips for Success

### Trust the System
- Capture everything immediately
- Process inbox at least daily
- Do weekly reviews religiously

### Keep Lists Clean
- Only actionable items on next actions
- Regular pruning of someday/maybe
- Delete ruthlessly

### Use AI Wisely
- AI suggests, you decide
- Use `process_inbox` for clarification structure
- Use `weekly_review` for comprehensive overview
- Don't over-automate decisions

### Start Simple
- Begin with just inbox and next actions
- Add contexts as needed
- Add energy/estimates when helpful
- Expand gradually

---

## Further Reading

- [Getting Things Done](https://gettingthingsdone.com/) by David Allen
- [CS Syd's GTD with TaskWarrior series](https://cs-syd.eu/posts/2015-06-14-gtd-with-taskwarrior-part-1-intro)
- [gtd-cheatsheet.md](gtd-cheatsheet.md) - Quick reference
- [mcp-tools.md](mcp-tools.md) - Tool documentation
