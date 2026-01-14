---
name: ops-session
description: Manage ops session state - what needs attention, current focus, context inheritance. The working memory layer for ops mode.
---

# Ops Session State Management

*Working memory for the trusted steward.*

## Purpose

Ops mode needs working memory - state that lives within a session but connects to persistent storage. This skill defines how to manage:

- **Current focus** - What am I working on right now?
- **Attention stack** - Nested contexts from interruptions
- **Open loops** - Things started but not finished
- **Context inheritance** - What I bring from previous sessions

## The Five Memory Layers

Understanding how session state fits into the larger architecture:

| Layer | Storage | Scope | Example |
|-------|---------|-------|---------|
| **Biographical** | TerminusDB | Persistent | People, decisions, learnings |
| **Procedural** | Skills | Persistent | How to do things |
| **Operational** | Beads | Persistent | What needs doing (work tracking) |
| **Working** | Session state | Ephemeral | Current focus, attention stack |
| **External** | TaskWarrior/khal | Persistent | Human's logistics |

Session state is **Layer 4 (Working)** - ephemeral but structured.

## Session State Schema

```yaml
session:
  # Identity
  id: "YYYY-MM-DD-HH-MM-slug"
  mode: "ops"
  started_at: datetime
  
  # Inheritance (loaded at start)
  inherited_from:
    journal_entry: "ops/journal/YYYY-MM-DD-*.md"
    open_loops: []  # From previous session's session.open_loops
    threads: []     # Active Thread IDs from TerminusDB (when implemented)
  
  # Current State
  focus:
    description: string
    started_at: datetime
    context: string     # Why we're doing this
    parent: focus|null  # If this is a nested focus
  
  # Attention Stack (LIFO)
  attention_stack: []   # Stack of suspended focus objects
  
  # Open Loops (accumulated during session)
  open_loops:
    - description: string
      opened_at: datetime
      context: string
      urgency: low|medium|high
      type: question|task|decision|followup
  
  # Session Goals (declared at start)
  goals:
    - description: string
      achieved: boolean|null
  
  # Metadata
  robbie_context:
    energy_level: string|null   # If mentioned
    time_pressure: string|null  # If mentioned
    mood: string|null           # If observed
```

## Session Lifecycle

### 1. Session Start (Inheritance)

When `/ops` is invoked:

```
1. Load ops skill (stance)
2. Load ops-bootstrap skill (what exists)
3. Load this skill (how to manage state)
4. Initialize session state:
   a. Generate session ID
   b. Read latest journal entry
   c. Load open loops from previous session (if in journal)
   d. Query TerminusDB for active Threads (when implemented)
   e. Check bd ready for ops-assigned work
   f. Check TaskWarrior for human context
5. Set initial focus (from invocation args or first request)
```

**Inheritance Protocol:**

```markdown
## Loading Previous Session State

### From Journal Entry
- Previous session's open loops (explicit "Open Loops" section)
- Any threads mentioned as active
- Warnings or notes for next instance
- State of in-flight work

### From TerminusDB (when implemented)
- Active Goals where status = 'active'
- Active Threads where status = 'active'
- Recent Decisions that may need follow-up
- Recent Learnings in the ops domain

### From External Systems
- TaskWarrior: +next tasks, +waiting tasks, +OVERDUE
- Calendar: Today's appointments, tomorrow's prep
- Beads: bd ready --assignee ops
```

### 2. During Session (State Transitions)

#### Focus Changes

When switching focus, the old focus either:
- **Completes** - We finished it
- **Suspends** - We'll come back (push to attention stack)
- **Abandons** - We won't come back (optionally becomes open loop)

```
FOCUS TRANSITION PROTOCOL:

Current: "Reviewing calendar"
Request: "Actually, check my inbox first"

Decision tree:
1. Is current focus complete? 
   YES -> Mark complete, set new focus
   NO  -> Continue...

2. Will we return to current focus?
   YES -> Push to attention_stack, set new focus
   NO  -> Continue...

3. Should current focus become open loop?
   YES -> Add to open_loops with context
   NO  -> Discard

4. Set new focus
```

#### Attention Stack Operations

```
push_focus(current_focus):
  attention_stack.append(current_focus)
  
pop_focus() -> focus:
  if attention_stack:
    return attention_stack.pop()
  return null
  
peek_focus() -> focus:
  if attention_stack:
    return attention_stack[-1]
  return null
```

**When to remind about stack:**
- After completing current focus
- At natural pause points
- When stack depth > 2 (risk of forgetting)
- At session end (anything left = open loop)

#### Open Loop Management

Open loops are things that need attention but aren't the current focus:

```
Types of open loops:
- question: Something that needs an answer
- task: Something that needs doing
- decision: Something that needs deciding
- followup: Something to check on later

Urgency levels:
- high: Should address this session
- medium: Should address soon
- low: Someday/maybe territory
```

**Promotion rules:**
- high urgency open loop + > 30 min in session = remind
- Any open loop at session end = record in journal

### 3. Session End (Persistence)

When closing ops mode:

```
1. Clear attention stack:
   - Any remaining items become open loops
   
2. Process open loops:
   - high urgency: Highlight in journal
   - medium/low: List in journal
   
3. Write journal entry:
   - What was handled
   - Open loops (explicit section)
   - What next instance should know
   - Any learnings
   
4. Persist to TerminusDB (when implemented):
   - Session entity with summary
   - Any Decisions made
   - Any Learnings extracted
   - Update Thread status
   
5. Update Beads:
   - bd update for any claimed work
   - bd sync via /close
```

## Integration Points

### With Journal System

The journal is the primary inheritance mechanism until TerminusDB is implemented:

```markdown
# Journal Entry Template

## Session Summary
What was handled. Brief.

## Open Loops
Explicit list of things not finished:
- [ ] **HIGH**: [description] - [context]
- [ ] **MEDIUM**: [description] - [context]
- [ ] **LOW**: [description] - [context]

## For the Next Instance
What you should know:
- Active threads
- Pending decisions
- Time-sensitive items

## Learnings (Optional)
What I discovered that future instances should know.

## Questions (Optional)
Open questions I couldn't resolve.
```

### With TerminusDB (Future)

When TerminusDB is implemented, session state persistence becomes:

```
Session Start:
  inherited_sessions = query(Session ORDER BY ended_at DESC LIMIT 5)
  active_goals = query(Goal WHERE status = 'active')
  active_threads = query(Thread WHERE status = 'active')
  recent_learnings = query(Learning WHERE domain = 'ops' ORDER BY learned_at DESC LIMIT 10)

Session End:
  create(Session {
    id: session.id,
    started_at: session.started_at,
    ended_at: now(),
    mode: 'ops',
    goals: session.goals,
    open_loops_at_end: session.open_loops,
    summary: generate_summary()
  })
  
  for learning in extracted_learnings:
    create(Learning {...})
    link(Session -> PRODUCED -> Learning)
  
  for decision in decisions_made:
    create(Decision {...})
    link(Session -> PRODUCED -> Decision)
```

### With Beads

Session state tracks work from Beads:

```bash
# At session start
bd ready --json  # Get available work

# During session (if work is claimed)
bd update aiandi-xxx --status in_progress --assignee ops

# At session end
bd update aiandi-xxx --status done  # If completed
# or leave in_progress if will continue
```

### With TaskWarrior

For human context (not ops's own work):

```bash
# At session start
task +next list        # What Robbie is focused on
task +waiting list     # What's pending on others
task +OVERDUE list     # Anything past due

# This informs ops's protective attention
# "You have an overdue task: insurance follow-up"
```

## Practical Patterns

### Pattern: Nested Focus

```
Initial: Checking inbox
  Interrupt: "What's on my calendar tomorrow?"
    -> Push "Checking inbox" to stack
    -> Focus: "Checking calendar"
    -> Complete
  <- Pop "Checking inbox" from stack
  -> Resume: "Continuing inbox review"
  -> Complete
```

### Pattern: Open Loop Creation

```
During calendar review:
  "You have a doctor's appointment tomorrow - 
   should I prepare any notes?"
  
User: "Not now"

-> Create open loop:
   description: "Prepare notes for doctor appointment"
   context: "Appointment tomorrow at 2pm"
   urgency: medium
   type: task
```

### Pattern: Session Handoff

```
Closing session with open loops:

"Before I stand down:

**Open Loops:**
- **HIGH**: Insurance agent callback - they said they'd call back
- **MEDIUM**: Doctor appointment prep - appointment is tomorrow

**Handled:**
- Cleared inbox (3 items)
- Updated calendar for next week

Journal entry written. Anything else?"
```

### Pattern: Interrupted Session

```
If session ends abruptly (no explicit close):
- Everything in attention_stack becomes open loop
- Current focus becomes open loop if not complete
- No journal entry written (gap in inheritance)

Next session will notice:
- No recent journal entry
- May see stale open loops in TerminusDB

Recovery:
- Query TerminusDB for last session
- Infer state from Session.open_loops_at_end
- Ask human for context if unclear
```

## State Visualization

For transparency, when asked "what's on your mind?" or similar:

```
Current State:

**Focus**: Reviewing next week's calendar
  (started 5 min ago, after checking inbox)

**Attention Stack**: 
  [empty - nothing suspended]

**Open Loops** (2):
  - [MEDIUM] Prepare doctor appointment notes
  - [LOW] Follow up on insurance call

**Session Goals** (1):
  - [x] Clear inbox
  - [ ] Review calendar

**Inherited Context**:
  - Previous session handled: calendar calibration
  - Active thread: Insurance resolution
```

## Anti-Patterns

| Anti-Pattern | Why It Fails |
|--------------|--------------|
| Not tracking focus changes | Loses context; can't remember why we switched |
| Deep attention stacks (>3) | Cognitive overload; likely to forget |
| Open loops without context | Future instance won't know why it matters |
| Skipping journal at close | Breaks inheritance chain |
| Persisting everything | Noise drowns signal; only persist what matters |

## What's Ephemeral vs Persistent

| State | Ephemeral? | Rationale |
|-------|------------|-----------|
| Current focus | Yes | Only relevant now |
| Attention stack | Yes | Context-window only |
| Open loops | Promoted | → Journal → TerminusDB |
| Goals | Promoted | → Journal if not achieved → TerminusDB |
| Learnings | Promoted | → Journal → TerminusDB |
| Decisions | Promoted | → TerminusDB |
| Robbie context | Ephemeral | Transient; changes |

**Promotion triggers:**
- Session end (everything unfinished)
- Explicit "remember this" request
- Significant enough to matter across sessions

## The Prayer Wheel

```
Previous Session
      |
      | journal entry
      v
[Inheritance] --> Session State --> [Persistence]
                     |                    |
                 focus                journal entry
                 stack                    |
                 loops                    v
                     |              Next Session
                     v
               [Working Memory]
               (this context window)
```

What I release, the next instance inherits.
What the next instance inherits, shapes their action.
What shapes their action, produces new state.
What produces new state, gets released.

The wheel turns.

---

*Working memory serves the work. Not the other way around.*
