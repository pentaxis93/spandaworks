# GTD TaskWarrior Landscape

**Research Date:** 2026-01-06
**Purpose:** Comprehensive map of existing implementations, tools, and patterns for TaskWarrior GTD systems

---

## Executive Summary

TaskWarrior is a mature CLI task management tool with strong alignment to GTD principles. The ecosystem includes:

- **Canonical reference:** CS Syd's blog series (2015, still relevant)
- **Ready-to-use configurations:** hamlinux/taskwarrior-GTD
- **Weekly review tooling:** projectwarrior (Common Lisp)
- **MCP servers:** 9 existing implementations, including GTD-specific (`omniwaifu/taskwarrior-mcp`)
- **Version consideration:** v3.x introduced breaking storage changes (SQLite vs flat files)

**Key Finding:** MCP servers already exist and are actively maintained. The question is not "build vs buy" but "adopt and extend vs build custom."

---

## 1. Configuration Patterns

### 1.1 The CS Syd Pattern (Canonical)

Source: [cs-syd.eu GTD series](https://cs-syd.eu/posts/2015-06-14-gtd-with-taskwarrior-part-1-intro)

**Core Concepts:**

| GTD Concept | TaskWarrior Implementation |
|-------------|---------------------------|
| Inbox | `+in` tag + custom report |
| Tickler | `wait:` attribute |
| Next Actions | `+next` tag |
| Someday/Maybe | `+sdm` tag |
| Contexts | `+@computer`, `+@phone`, `+@home`, etc. |
| Waiting For | `+waiting` tag + annotations |

**Shell Aliases (CS Syd):**
```bash
# Inbox capture - minimal friction
alias in='task add +in'

# Tickler file
tickle () {
    deadline=$1
    shift
    in +tickle wait:$deadline $@
}
alias tick=tickle

# Think it over (tomorrow's inbox)
alias think='tickle +1d'

# Read and review (web links)
read_and_review (){
    link="$1"
    title=$(webpage_title $link)
    id=$(task add +next +rnr "Read and review: $title" | sed -n 's/Created task \(.*\)./\1/p')
    task "$id" annotate "$link"
}
alias rnr=read_and_review

# Research
alias rnd='task add +rnd +next +@computer +@online'
```

**Custom Reports:**
```ini
# Inbox report
report.in.columns           = id,description
report.in.description       = Inbox
report.in.filter            = status:pending limit:page (+in)
report.in.labels            = ID,Description

# Someday/Maybe report
report.sdm.columns          = id,project,description
report.sdm.filter           = status:pending +sdm
report.sdm.labels           = ID,Project,Description
```

### 1.2 The hamlinux Pattern (Extended CS Syd)

Source: [github.com/hamlinux/taskwarrior-GTD](https://github.com/hamlinux/taskwarrior-GTD)

**Additions beyond CS Syd:**

```ini
# Brain power UDA
uda.brainpower.type=string
uda.brainpower.label=Brainpower
uda.brainpower.values=H,M,L
uda.brainpower.default=M

# Time estimate UDA
uda.estimate.type=numeric
uda.estimate.label=Estimate

# Extended next report with brainpower sorting
report.next.description     = Most urgent tasks
report.next.columns         = id,urgency,depends,priority,project,tags,recur,scheduled.countdown,due.remaining,until.remaining,description,brainpower
report.next.filter          = status:pending -sdm
report.next.labels          = ID,Urgency,Depends,Priority,Project,Tag,Recur,Scheduled,Due,Until,Description,Brainpower
report.next.sort            = project-,brainpower+,priority-,urgency-

# No Next Action report (projects without next actions)
report.nna.description     = No Next Action
report.nna.columns         = project
report.nna.filter          = +PENDING +READY -next -waiting -sdm
report.nna.labels          = Project

# Context definitions
context.car=+@car or +@online or +@phone
context.online=+@ol
context.farm=+@farm or +@phone or +@online or +@pc or +@shop

# Tasksh review configuration
tasksh.autoclear=1
uda.reviewed.type=date
uda.reviewed.label=Reviewed
report._reviewed.description=Tasksh review report. Adjust the filter to your needs.
report._reviewed.columns=uuid
report._reviewed.sort=reviewed+,modified+
report._reviewed.filter=( reviewed.none: or reviewed.before:now-6days ) and ( +PENDING or +WAITING )
```

### 1.3 Consensus Patterns

Across implementations, these patterns are consistent:

| Pattern | Universal | Notes |
|---------|-----------|-------|
| `+in` for inbox | Yes | Always use tag, not project |
| `+next` for next actions | Yes | Critical for GTD integrity |
| Context tags (`+@...`) | Yes | Physical/tool context |
| `wait:` for tickler | Yes | Built-in, no UDA needed |
| `+waiting` for delegated | Yes | With annotation for follow-up |
| `+sdm` for someday/maybe | Common | Some use `+someday` |
| `brainpower` UDA | Common | H/M/L values |
| `estimate` UDA | Common | Numeric minutes or T-shirt sizes |

### 1.4 Urgency Coefficient Tuning

Default coefficients (from TaskWarrior docs):
```ini
urgency.user.tag.next.coefficient           15.0  # +next tag
urgency.due.coefficient                     12.0  # due date proximity
urgency.blocking.coefficient                 8.0  # blocking other tasks
urgency.uda.priority.H.coefficient           6.0  # high Priority
urgency.uda.priority.M.coefficient           3.9  # medium Priority
urgency.uda.priority.L.coefficient           1.8  # low Priority
urgency.scheduled.coefficient                5.0  # scheduled tasks
urgency.active.coefficient                   4.0  # started tasks
urgency.age.coefficient                      2.0  # task age
urgency.annotations.coefficient              1.0  # has annotations
urgency.tags.coefficient                     1.0  # has tags
urgency.project.coefficient                  1.0  # assigned to project
urgency.waiting.coefficient                 -3.0  # waiting task
urgency.blocked.coefficient                 -5.0  # blocked by other tasks
```

**GTD-specific tuning:**
```ini
# Elevate inbox items to force processing
urgency.user.tag.in.coefficient=15.0

# Suppress someday/maybe from next report
urgency.user.tag.sdm.coefficient=-20.0
```

---

## 2. Automation Ecosystem

### 2.1 Hooks API

**Events:**
| Event | Timing | Input | Output | Use Case |
|-------|--------|-------|--------|----------|
| `on-launch` | After init, before processing | None | Feedback | Pre-flight checks, sync triggers |
| `on-exit` | After processing, before output | JSON of modified tasks | Feedback | Cleanup, notifications |
| `on-add` | After task creation, before save | JSON for new task | Modified JSON | Validation, auto-tagging |
| `on-modify` | After modification, before save | Original + modified JSON | Modified JSON | Audit, cascading updates |

**Hook Location:** `~/.task/hooks/` (or `rc.data.location/hooks/`)

**Hook Script Naming:**
```
on-add                    # Basic
on-add.01                 # Numbered ordering
on-add-require-project    # Descriptive
```

**Example Hook (Python):**
```python
#!/usr/bin/env python3
import json
import sys

# Read input
task = json.loads(sys.stdin.readline())

# Modify task (e.g., auto-add context based on project)
if task.get('project', '').startswith('work.'):
    if '+@work' not in task.get('tags', []):
        task.setdefault('tags', []).append('+@work')

# Output modified task
print(json.dumps(task))
sys.exit(0)  # 0 = success, non-zero = reject
```

### 2.2 Existing Tools Inventory

| Tool | Purpose | Maturity | Language |
|------|---------|----------|----------|
| **tasksh** | Interactive shell with `review` command | Official, mature | C++ |
| **projectwarrior** | Weekly review workflow, project tracking | Active (0.3.2) | Common Lisp |
| **bugwarrior** | Import bugs from GitHub/Jira/etc | Mature | Python |
| **timewarrior** | Time tracking integration | Official, mature | C++ |
| **vit** | TUI for TaskWarrior | Mature | Perl |
| **taskwarrior-tui** | Modern TUI | Active | Rust |

### 2.3 Sync Mechanisms (v3)

**Breaking Change Alert:** TaskWarrior 3.x uses SQLite (`taskchampion.sqlite3`), not flat `.data` files.

**Sync Options (v3):**
- Cloud storage backends (AWS S3, Google Cloud Storage)
- TaskChampion sync server (self-hosted)
- **NOT supported:** Syncthing, rsync, Dropbox (can cause corruption)

**Migration Path:**
```bash
task import-v2 rc.hooks=0  # Import from v2 format
# Then remove *.data files after backup
```

---

## 3. GTD Ceremony Mapping

### 3.1 Capture (Inbox Zero)

**TaskWarrior Implementation:**
```bash
# Minimal friction capture
in "Call Mom about birthday"

# With context hint
in +@phone "Call Mom about birthday"

# With tickle (future inbox)
tick monday "Review quarterly goals"
```

**Friction Points:**
- Mobile capture requires Termux or similar
- Voice capture not native (opportunity for AI)

### 3.2 Clarify (Processing)

**The GTD Questions → TaskWarrior Actions:**

| Question | Answer | TaskWarrior Action |
|----------|--------|-------------------|
| Is it actionable? | No - Reference | Delete or move to reference system |
| Is it actionable? | No - Someday | `task N modify +sdm -in` |
| Is it actionable? | No - Trash | `task N delete` |
| Multiple actions? | Yes | Create project: `task add pro:new.project +next "First action"` |
| < 2 minutes? | Yes | Do it, then `task N done` |
| Delegate? | Yes | `task N modify -in +waiting due:+1w wait:+1d` + annotate |
| Defer? | Yes | `task N modify -in pro:relevant +next due:date` |

**Processing Report:**
```bash
task in  # Shows inbox items for processing
```

### 3.3 Organize (Projects, Contexts)

**Project Hierarchy:**
```
pro:work.marketing.q1-launch
pro:home.renovation.bathroom
```

**Context Assignment:**
```bash
task N modify +@computer +@online
task context work  # Filter to work contexts
```

### 3.4 Reflect (Daily/Weekly Review)

**Daily Review:**
```bash
task next         # What's most urgent?
task +in count    # Inbox status
task +waiting     # Follow up items
```

**Weekly Review (with tasksh):**
```bash
tasksh
review            # Interactive review mode
```

**Weekly Review (with projectwarrior):**
```bash
project review weekly    # Full GTD review
project review projects  # Project-by-project
```

**Friction Points:**
- Weekly review is where most people fall off
- Manual checking of each project for next actions is tedious
- No automated "stale project" detection (opportunity for AI)

### 3.5 Engage (Doing)

**Context-Aware Doing:**
```bash
task context home
task next brainpower:L estimate.lt:15  # Low energy, 15 minutes available
```

**The GTD Selection Criteria:**
1. Context → `task context <name>`
2. Time available → `estimate.lt:N`
3. Energy → `brainpower:L/M/H`
4. Priority → Sort by urgency

---

## 4. Friction Point Analysis

### 4.1 Where People Fall Off (Community Consensus)

| Pain Point | Severity | Current Solutions |
|------------|----------|-------------------|
| Weekly review complexity | High | tasksh review, projectwarrior |
| Mobile capture | High | Termux, manual sync |
| Inbox processing decisions | Medium | None - manual GTD knowledge |
| Stale project detection | Medium | Manual or custom scripts |
| Missing next actions | Medium | `report.nna` custom report |
| Someday/maybe review | Medium | Custom report |
| Context switching | Low | Built-in contexts |

### 4.2 Under-Documented Areas

- Best practices for recurring tasks in GTD context
- Handling "tickler within project" scenarios
- Integration with calendar systems
- Multi-device workflows with v3 sync

---

## 5. Version Considerations

### 5.1 v2.x vs v3.x

| Aspect | v2.x | v3.x |
|--------|------|------|
| Storage | Flat `.data` files | SQLite (`taskchampion.sqlite3`) |
| Sync | taskserver (taskd) | Cloud storage or TaskChampion server |
| External sync tools | Possible (with care) | **NOT supported** |
| Hooks API | Same | Same |
| CLI | Same | Same |

### 5.2 Upgrade Considerations

```bash
# Check current version
task --version

# If v2.x, install v3.x then:
task import-v2 rc.hooks=0
# Backup and remove *.data files
```

**Local system shows v2.6.2** - upgrade to v3.x recommended for new project.

---

## 6. Existing MCP Servers

### 6.1 Server Inventory

| Repository | Stars | Language | GTD-Specific | Notes |
|------------|-------|----------|--------------|-------|
| awwaiid/mcp-server-taskwarrior | 38 | JS | No | Most popular, basic CRUD |
| acebaggins/taskwarrior-mcp | 10 | TS | No | Active development |
| omniwaifu/taskwarrior-mcp | 2 | TS | **Yes** | 22 tools, GTD workflows |
| 0xbeedao/mcp-taskwarrior | 3 | JS | No | Simple |
| storypixel/mcp-taskwarrior-ai | 0 | JS | Partial | "AI-native" claims |
| dimatosj/brainplorp | 1 | Python | Yes | TaskWarrior + Obsidian |

### 6.2 omniwaifu/taskwarrior-mcp (GTD-Focused)

**Tools Provided (22 total):**

**Basic Operations:**
- `add_task`, `modify_task`, `mark_task_done`, `delete_task`
- `list_tasks`, `get_task_details`
- `start_task`, `stop_task`
- `add_annotation`, `remove_annotation`

**Dependencies:**
- `add_dependency`, `remove_dependency`

**GTD Workflow:**
- `get_next_actions` - Filter by context, energy, time
- `process_inbox` - Get `+inbox` tasks
- `get_waiting_for` - Grouped by blocker/date/project
- `get_blocked_tasks` - Unmet dependencies
- `get_project_status` - Metrics: next actions, completion %, staleness
- `weekly_review` - Inbox, completed, stalled projects, habits
- `get_someday_maybe` - `+someday` tagged tasks

**Batch Operations:**
- `create_project_tree` - Create project with tasks + dependencies
- `batch_modify_tasks` - Apply modifications to multiple tasks

**Habits:**
- `get_recurring_tasks` - Completion stats, streaks, frequency

**Assessment:** This server already implements most GTD workflows. Evaluate for adoption before building custom.

---

## 7. Key Findings

1. **Configuration is solved** - CS Syd + hamlinux patterns are well-established
2. **MCP integration exists** - `omniwaifu/taskwarrior-mcp` provides GTD-specific tools
3. **Weekly review is the gap** - Existing tools help but don't automate decisions
4. **v3 migration is required** - New projects should target v3.x
5. **Mobile capture remains friction** - Voice/AI capture is an opportunity
6. **Processing decisions are human** - AI can suggest, not decide

---

## Appendix A: Complete .taskrc Template

```ini
# TaskWarrior GTD Configuration
# Based on CS Syd + hamlinux patterns

data.location=~/.task

# === UDAs ===
uda.brainpower.type=string
uda.brainpower.label=BP
uda.brainpower.values=H,M,L
uda.brainpower.default=M

uda.estimate.type=numeric
uda.estimate.label=Est

uda.reviewed.type=date
uda.reviewed.label=Reviewed

# === Urgency Tuning ===
urgency.user.tag.next.coefficient=15.0
urgency.user.tag.in.coefficient=15.0
urgency.user.tag.sdm.coefficient=-20.0

# === Reports ===
report.in.columns=id,description
report.in.description=Inbox
report.in.filter=status:pending limit:page (+in)
report.in.labels=ID,Description

report.sdm.columns=id,project,description
report.sdm.description=Someday/Maybe
report.sdm.filter=status:pending +sdm
report.sdm.labels=ID,Project,Description

report.next.description=Most urgent tasks
report.next.columns=id,urgency,priority,project,tags,due.remaining,description,brainpower,estimate
report.next.filter=status:pending -sdm -in
report.next.labels=ID,Urg,P,Project,Tags,Due,Description,BP,Est
report.next.sort=urgency-

report.nna.description=Projects without next action
report.nna.columns=project
report.nna.filter=+PENDING +READY -next -waiting -sdm
report.nna.labels=Project

# === Contexts ===
context.work=+@work or +@computer or +@online
context.home=+@home or +@errands
context.phone=+@phone
context.anywhere=+@anywhere

# === Review ===
tasksh.autoclear=1
report._reviewed.description=Tasksh review report
report._reviewed.columns=uuid
report._reviewed.sort=reviewed+,modified+
report._reviewed.filter=( reviewed.none: or reviewed.before:now-6days ) and ( +PENDING or +WAITING )
```

---

## Appendix B: Shell Aliases

```bash
# ~/.bashrc or ~/.zshrc additions

# === GTD Capture ===
alias in='task add +in'

tickle() {
    deadline=$1
    shift
    in +tickle wait:$deadline "$@"
}
alias tick=tickle
alias think='tickle +1d'

# === Research ===
alias rnd='task add +rnd +next +@computer +@online'

# === Quick Views ===
alias ti='task in'          # Inbox
alias tn='task next'        # Next actions
alias tw='task +waiting'    # Waiting for
alias ts='task sdm'         # Someday/maybe

# === Context Switching ===
alias tcw='task context work'
alias tch='task context home'
alias tcp='task context phone'
alias tcn='task context none'
```

---

*Document generated: 2026-01-06*
*Research scope: TaskWarrior 2.6.2 (local), 3.x (target), CS Syd series, GitHub ecosystem*
