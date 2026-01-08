# MVP Recommendation: AI-Augmented GTD with TaskWarrior

**Research Date:** 2026-01-06
**Purpose:** Concrete recommendation for minimum viable system

---

## Executive Summary

**Recommendation:** Adopt existing infrastructure, extend with targeted AI tools.

**MVP Scope:**
1. Install TaskWarrior v3.x with GTD configuration
2. Adopt `omniwaifu/taskwarrior-mcp` for Claude integration
3. Build two custom AI tools: inbox processing suggestions + project health
4. Establish trust through 2-week manual operation before AI assists

**Timeline:** 1 week to functional MVP, 2 weeks to trust-validated system

**Explicitly NOT in MVP:** Natural language capture, time estimation learning, proactive alerts, calendar integration, mobile apps.

---

## 1. Proposed .taskrc Configuration

```ini
# ============================================
# spanda-gtd TaskWarrior Configuration
# Based on CS Syd + hamlinux patterns
# Version: MVP 1.0
# ============================================

# === Data Location ===
data.location=~/.task

# === User Defined Attributes ===

# Brain power required (GTD energy filtering)
uda.brainpower.type=string
uda.brainpower.label=BP
uda.brainpower.values=H,M,L
uda.brainpower.default=M

# Time estimate in minutes
uda.estimate.type=numeric
uda.estimate.label=Est

# Last reviewed date (for tasksh review)
uda.reviewed.type=date
uda.reviewed.label=Reviewed

# === Urgency Coefficients ===

# +next is most important
urgency.user.tag.next.coefficient=15.0

# Inbox items should surface prominently
urgency.user.tag.in.coefficient=15.0

# Someday/maybe should NOT appear in next report
urgency.user.tag.sdm.coefficient=-20.0

# Waiting items slightly deprioritized
urgency.user.tag.waiting.coefficient=-2.0

# === Custom Reports ===

# Inbox: Items awaiting GTD processing
report.in.columns=id,entry.age,description
report.in.description=Inbox - items to process
report.in.filter=status:pending limit:page (+in)
report.in.labels=ID,Age,Description
report.in.sort=entry+

# Someday/Maybe: Deferred possibilities
report.sdm.columns=id,project,description
report.sdm.description=Someday/Maybe list
report.sdm.filter=status:pending +sdm
report.sdm.labels=ID,Project,Description
report.sdm.sort=project+,entry+

# Next Actions: GTD-filtered next report
report.next.description=Next actions (GTD)
report.next.columns=id,urgency,priority,project,tags,due.remaining,description,brainpower,estimate
report.next.filter=status:pending -sdm -in -waiting limit:page
report.next.labels=ID,Urg,P,Project,Tags,Due,Description,BP,Est
report.next.sort=urgency-

# Waiting For: Delegated/blocked items
report.waiting.columns=id,project,description,wait.remaining,due
report.waiting.description=Waiting for (delegated)
report.waiting.filter=status:pending +waiting
report.waiting.labels=ID,Project,Description,Wait,Due
report.waiting.sort=due+,wait+

# No Next Action: Projects missing +next
report.nna.columns=project
report.nna.description=Projects without next action
report.nna.filter=+PENDING +READY -next -waiting -sdm
report.nna.labels=Project

# Weekly Review: Tasks needing review
report._reviewed.columns=uuid
report._reviewed.description=Tasksh review report
report._reviewed.filter=( reviewed.none: or reviewed.before:now-6days ) and ( +PENDING or +WAITING )
report._reviewed.sort=reviewed+,modified+

# === Contexts ===

# Work: At computer with internet and phone
context.work=+@work or +@computer or +@online or +@phone

# Home: Personal errands and home tasks
context.home=+@home or +@errands

# Focus: Deep work, no interruptions
context.focus=+@computer -@phone -@online

# Anywhere: Location-independent tasks
context.anywhere=+@anywhere or +@phone

# === Miscellaneous ===

# Clear screen between tasksh commands
tasksh.autoclear=1

# Default task command
default.command=next

# Confirmation for bulk operations
bulk=3
```

---

## 2. Essential Tags

| Tag | Purpose | Example |
|-----|---------|---------|
| `+in` | Inbox item awaiting processing | `task add +in "Review email from boss"` |
| `+next` | Next action for a project | `task 5 modify +next` |
| `+sdm` | Someday/Maybe | `task 5 modify +sdm -in` |
| `+waiting` | Waiting for someone/something | `task 5 modify +waiting` |
| `+tickle` | Tickler item (used with wait:) | `task add +in +tickle wait:monday "Check results"` |
| `+rnd` | Research/investigation | `task add +rnd +next "Research X"` |

**Context Tags:**
| Tag | Context |
|-----|---------|
| `+@computer` | At a computer |
| `+@online` | Internet available |
| `+@phone` | Can make calls |
| `+@home` | At home |
| `+@work` | At office/work |
| `+@errands` | Out running errands |
| `+@anywhere` | Location-independent |

---

## 3. Shell Aliases

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# ============================================
# spanda-gtd Shell Aliases
# ============================================

# === Capture ===

# Quick inbox capture
alias in='task add +in'

# Tickler file (future inbox)
tickle() {
    deadline=$1
    shift
    in +tickle wait:$deadline "$@"
}
alias tick=tickle

# Think it over (tomorrow's inbox)
alias think='tickle +1d'

# Research task
alias rnd='task add +rnd +next +@computer +@online'

# === Quick Views ===

# Inbox
alias ti='task in'

# Next actions
alias tn='task next'

# Waiting for
alias tw='task waiting'

# Someday/maybe
alias ts='task sdm'

# Projects without next action
alias tnna='task nna'

# === Context Switching ===

alias tcw='task context work'
alias tch='task context home'
alias tcf='task context focus'
alias tca='task context anywhere'
alias tcn='task context none'

# === Review Helpers ===

# Count inbox items (for prompt)
alias tic='task +in +PENDING count'

# Start tasksh review
alias review='tasksh -c review'

# === Processing ===

# Process inbox item (removes +in, adds +next)
process() {
    task $1 modify -in +next "$@"
}

# Defer to someday/maybe
defer() {
    task $1 modify -in +sdm
}

# Delegate (waiting for)
delegate() {
    task $1 modify -in +waiting due:+1w wait:+3d
}
```

---

## 4. First AI Automation: Inbox Processing Suggestions

### 4.1 Purpose

When user runs inbox processing, AI suggests:
- Actionable or not?
- Project assignment
- Context tags
- Energy level / time estimate
- Whether to defer to someday/maybe

**Critical Constraint:** Suggestions only. Human executes.

### 4.2 MCP Tool Specification

```typescript
interface InboxSuggestion {
  task_uuid: string;
  task_description: string;
  
  // GTD determination
  actionable: boolean;
  actionable_reasoning: string;
  
  // If actionable
  suggested_project?: string;
  suggested_contexts: string[];  // e.g., ["+@computer", "+@online"]
  suggested_brainpower?: "H" | "M" | "L";
  suggested_estimate_minutes?: number;
  suggested_next_action?: string;  // Reworded as concrete action
  
  // If not actionable
  disposition?: "reference" | "someday" | "trash";
  disposition_reasoning?: string;
  
  // Confidence
  confidence: "high" | "medium" | "low";
}

// Tool definition
const suggest_inbox_processing = {
  name: "suggest_inbox_processing",
  description: "Get AI suggestions for processing GTD inbox items",
  parameters: {
    type: "object",
    properties: {
      task_uuid: {
        type: "string",
        description: "UUID of inbox task to analyze"
      }
    },
    required: ["task_uuid"]
  }
};
```

### 4.3 Example Interaction

```
User: Process my inbox

Claude: Let me check your inbox.

[Calls: list_tasks with filter "+in"]

You have 3 items in your inbox:

1. "Email from Mark about project timeline" (added 2 hours ago)
2. "Maybe try that new coffee shop" (added yesterday)
3. "fix bug in auth module" (added 3 days ago)

Let me analyze each:

---
**Item 1: "Email from Mark about project timeline"**

Suggestion:
- Actionable: Yes (requires a response or action)
- Next Action: "Reply to Mark's email about project timeline"
- Project: `work.communication` (or create new)
- Contexts: +@computer +@online
- Energy: L (routine communication)
- Estimate: 15 minutes

Would you like me to apply these suggestions?
- `task <id> modify -in +next pro:work.communication +@computer +@online brainpower:L estimate:15`

---
**Item 2: "Maybe try that new coffee shop"**

Suggestion:
- Actionable: No (wish, not commitment)
- Disposition: Someday/Maybe

Would you like me to defer this?
- `task <id> modify -in +sdm`

---
**Item 3: "fix bug in auth module"**

Suggestion:
- Actionable: Yes (specific technical task)
- Next Action: "Debug and fix authentication module bug"
- Project: `zenvestor.bugs` (inferred from context)
- Contexts: +@computer
- Energy: H (requires focus)
- Estimate: 60 minutes

Would you like me to apply these suggestions?
- `task <id> modify -in +next pro:zenvestor.bugs +@computer brainpower:H estimate:60`

---

Which suggestions should I apply? (all / 1,2,3 / none / modify?)
```

---

## 5. Second AI Automation: Project Health Monitor

### 5.1 Purpose

Surface projects that need attention:
- Missing next action
- Stale (no activity in 7+ days)
- Waiting items past due
- Zombie projects (all tasks done, project still open)

### 5.2 MCP Tool Specification

```typescript
interface ProjectHealthIssue {
  project: string;
  issue_type: "missing_next" | "stale" | "waiting_overdue" | "zombie";
  severity: "critical" | "warning" | "info";
  details: string;
  suggested_action: string;
}

interface ProjectHealthReport {
  scan_time: string;
  total_projects: number;
  healthy_projects: number;
  issues: ProjectHealthIssue[];
  summary: string;
}

const scan_project_health = {
  name: "scan_project_health",
  description: "Scan all projects for GTD health issues",
  parameters: {
    type: "object",
    properties: {},
    required: []
  }
};
```

### 5.3 Example Output

```
Project Health Scan (2026-01-06 14:30)

Total Projects: 12
Healthy: 8
Issues: 4

CRITICAL:
- zenvestor.auth: Missing next action (3 pending tasks, none tagged +next)
  → Suggested: Review tasks, mark one as +next

WARNING:
- home.renovation: Stale project (last activity 14 days ago)
  → Suggested: Review if still active or defer to someday

- work.q1-planning: Waiting item overdue ("Waiting for budget approval" due 3 days ago)
  → Suggested: Follow up or update due date

INFO:
- personal.reading: All tasks completed, project still open
  → Suggested: Mark project complete or add new tasks
```

---

## 6. Trust-Building Protocol

### 6.1 Week 1: Manual Operation

**Goal:** Build muscle memory and confidence in core GTD + TaskWarrior.

**Daily Practice:**
1. Morning: `task next` - Select today's work
2. Capture: Use `in` alias throughout day
3. Evening: `task in` - Process inbox manually

**Weekly Practice:**
1. `tasksh review` - Review all tasks
2. `task nna` - Check for missing next actions
3. `task sdm` - Review someday/maybe

**No AI assistance this week.** Human must internalize the GTD flow.

### 6.2 Week 2: Introduce AI Suggestions

**Goal:** Validate AI suggestions match human judgment.

**Protocol:**
1. Run inbox processing with AI suggestions
2. For each suggestion, rate: Accept / Modify / Reject
3. Track acceptance rate
4. If acceptance < 70%, tune prompts before proceeding

**Success Criteria:**
- Inbox processing time reduced by 50%
- Suggestion acceptance rate > 70%
- No erroneous task modifications

### 6.3 Ongoing: Trust Maintenance

**Weekly Check:**
- Review AI suggestion logs
- Note any suggestions that led to problems
- Refine prompts as needed

**Abort Criteria:**
- If AI causes task loss or corruption: Disable immediately
- If AI suggestions consistently wrong: Retrain or remove
- If system feels untrustworthy: Return to manual

---

## 7. What NOT to Build in MVP

| Feature | Reason to Defer |
|---------|----------------|
| Natural language capture | Complex NLP, mobile integration needed |
| Time estimation learning | Requires timewarrior + ML pipeline |
| Proactive alerts | Needs notification infrastructure |
| Calendar integration | Scope creep, separate system |
| Mobile app | Termux works, native app is large effort |
| Multi-user support | Single-user GTD is hard enough |
| Voice capture | Requires speech-to-text pipeline |
| Habit tracking | TaskWarrior recurring tasks work, defer enhancement |

---

## 8. Implementation Roadmap

### Phase 1: Foundation (Days 1-3)

**Day 1:**
- [ ] Upgrade to TaskWarrior v3.x
- [ ] Apply .taskrc configuration
- [ ] Install shell aliases
- [ ] Test core GTD workflow manually

**Day 2:**
- [ ] Install `omniwaifu/taskwarrior-mcp`
- [ ] Configure Claude Desktop / OpenCode integration
- [ ] Test basic MCP tools (list_tasks, add_task, etc.)

**Day 3:**
- [ ] Begin Week 1 manual operation
- [ ] Document any friction points
- [ ] Capture existing tasks into system

### Phase 2: AI Integration (Days 4-7)

**Day 4-5:**
- [ ] Implement `suggest_inbox_processing` tool
- [ ] Test with sample inbox items
- [ ] Tune prompts for accuracy

**Day 6-7:**
- [ ] Implement `scan_project_health` tool
- [ ] Integrate into daily/weekly workflow
- [ ] Document usage patterns

### Phase 3: Trust Validation (Days 8-14)

- [ ] Complete Week 2 trust-building protocol
- [ ] Track suggestion acceptance rates
- [ ] Iterate on prompt engineering
- [ ] Document learnings for blog

---

## 9. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Inbox processing time | -50% vs manual | Stopwatch comparison |
| AI suggestion acceptance | >70% | Log accept/modify/reject |
| Inbox zero frequency | Daily | Track via `task +in count` |
| Weekly review completion | 100% | Binary - did review happen? |
| System trust rating | 8+/10 | Self-assessment |
| Projects without next action | 0 | `task nna` output |

---

## 10. Repository Structure (spanda-gtd)

```
spanda-gtd/
├── README.md
├── config/
│   ├── taskrc.template       # .taskrc configuration
│   └── aliases.sh            # Shell aliases
├── mcp-server/
│   ├── package.json
│   ├── src/
│   │   ├── index.ts          # MCP server entry
│   │   ├── tools/
│   │   │   ├── inbox.ts      # suggest_inbox_processing
│   │   │   └── health.ts     # scan_project_health
│   │   └── taskwarrior.ts    # TaskWarrior CLI wrapper
│   └── prompts/
│       ├── inbox.md          # Inbox processing prompt
│       └── health.md         # Health scan prompt
├── docs/
│   ├── setup.md
│   ├── workflow.md
│   └── trust-protocol.md
└── research/
    ├── gtd-taskwarrior-landscape.md
    ├── ai-integration-opportunities.md
    └── mvp-recommendation.md     # This document
```

---

## 11. Decision Log

| Decision | Rationale |
|----------|-----------|
| Adopt omniwaifu/tw-mcp | Already implements GTD workflows, avoid duplication |
| TaskWarrior v3.x | New projects should use current version |
| Suggestions only (no auto-action) | Preserve human agency and trust |
| Week 1 manual | Build muscle memory before AI assistance |
| Two AI tools for MVP | Inbox + health cover 80% of value |
| Defer NLP capture | High complexity, lower impact than processing |

---

## 12. Open Questions

1. **Fork or extend omniwaifu/taskwarrior-mcp?**
   - Option A: Fork and add custom tools
   - Option B: Separate MCP server that calls theirs
   - Recommendation: Start with Option B for clean separation

2. **How to handle suggestion rejection?**
   - Should we learn from rejections?
   - Or keep MVP simple with static prompts?
   - Recommendation: Log rejections, defer learning to Phase 2

3. **Integration with Obsidian vault?**
   - Profile.md has goals/priorities
   - Could inform inbox processing suggestions
   - Recommendation: Defer to Phase 2, keep MVP focused

---

*Document generated: 2026-01-06*
*Status: Ready for Governance review and implementation decision*
