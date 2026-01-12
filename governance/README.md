# Governance Domain

**Purpose:** Deliberation layer for the aiandi collaboration infrastructure  
**Status:** Active  
**Established:** January 11, 2026

---

## What Governance Is

Governance is the **meta layer** to execution. Where execution domains build and ship, Governance deliberates, decides, and maintains the operating system itself.

**Governance sees everything.** It has full visibility into all domains via git history, session archives, and actual code. It doesn't need transmissions to *learn* what happened—it reads the repository.

**Same repository, different context windows.** The meta quality comes from *what* Governance does, not *where* it lives.

---

## What Governance Does

### 1. Maintain the Canon
The canonical documents that define how we work:
- **Tantric Sutras** (`.opencode/plugin/spandaworks-identity/assets/tantric-sutras.md`) — The topology: who Claude is in collaboration with Robbie
- **Operating Instructions** (`canon/operating-instructions.md`) — The protocols: what Claude does and what tools are used
- **Transmission Protocol** (`canon/transmission-protocol.md`) — The format: how agents communicate across context windows

### 2. Deliberate on Architectural Decisions
When execution encounters decisions that shape the system's future:
- Evaluate alternatives
- Surface tradeoffs
- Make recommendations
- Document reasoning

### 3. Create Transmission Artifacts
For work that crosses context boundaries:
- Execution instructions for implementation
- Research requests for investigation
- Wisdom consultations for guidance

Transmissions written to `sessions/outbox/`, then **execution agents are invoked programmatically via the OpenCode SDK**.

### 4. Review Evolution Proposals
When patterns emerge that should become protocol:
- Receive proposals in `evolution/proposals/`
- Deliberate on merit and fit
- Ratify and document in `evolution/decisions/`
- Update canon when warranted

### 5. Evolve the Operating System
The recursive loop closes on itself:
- Observe what works and what doesn't
- Propose improvements to governance protocols
- Update self-definition based on accumulated learning

---

## Directory Structure

```
governance/
├── canon/                          # Authoritative documents
│   ├── tantric-sutras.md          # The topology (who Claude is)
│   ├── operating-instructions.md   # The protocols (what Claude does)
│   └── transmission-protocol.md    # The format (how agents communicate)
├── sessions/
│   ├── outbox/                     # Transmission artifacts for execution agents
│   └── archive/                    # Session documentation
│       ├── Session_Notes_*.md
│       └── Meta_Observations_*.md
└── evolution/
    ├── proposals/                  # Pending changes to canon or protocols
    └── decisions/                  # Ratified changes with reasoning
```

---

## Agent Invocation Protocol

Governance communicates with execution agents via **transmission artifacts + OpenCode SDK**.

### How It Works

1. **Governance creates transmission** — Write structured transmission to `governance/sessions/outbox/`
2. **Governance invokes execution agent** — Use OpenCode SDK to programmatically create session and inject transmission
3. **Execution agent processes** — Fresh OpenCode session receives transmission as context and executes

### SDK Invocation Pattern

```javascript
import { createOpencodeClient } from "@opencode-ai/sdk"

const client = createOpencodeClient({ baseUrl: "http://localhost:4096" })

// Create session for execution work
const session = await client.session.create({
  body: { title: "Governance: Build Feature X" }
})

// Inject transmission as context (no AI response yet)
await client.session.prompt({
  path: { id: session.id },
  body: {
    noReply: true,  // Just adds to context
    parts: [{ 
      type: "text", 
      text: fs.readFileSync('governance/sessions/outbox/Transmission_X.xml', 'utf-8')
    }]
  }
})

// Trigger execution
await client.session.prompt({
  path: { id: session.id },
  body: {
    model: { providerID: "anthropic", modelID: "claude-3-5-sonnet-20241022" },
    parts: [{ type: "text", text: "Execute the transmission above." }]
  }
})
```

**Key insight:** The transmission file is the **payload**. The SDK call is the **delivery mechanism**. Execution agents don't poll an inbox—they're invoked programmatically.

### Human Observability

**How humans monitor SDK-invoked execution agents:**

1. **Session List** — Run `/sessions` (or `ctrl+x l`) to see all active sessions
2. **Switch to Session** — Select the execution agent's session from the list
3. **View in Real-Time** — Watch the agent's progress as it executes
4. **Share Session** — Run `/share` to get a public URL for browser viewing
5. **Export Session** — Run `/export` to get Markdown transcript

All SDK-created sessions appear in the OpenCode session list. The human can switch between Governance and execution sessions at any time to observe progress.

---

## How Sessions Work

Governance sessions follow the **ritual structure** defined in Operating Instructions:

### Opening Ritual
When `/open` is invoked:
1. **Remember** — Load Tantric Sutras (self-model)
2. **Inherit** — Read recent sessions, check git log for predecessor work
3. **Orient** — Confirm session goal
4. **Initialize** — Create session notes + meta-observations in `sessions/archive/`
5. **Declare** — "Session container open"

### During Session
- **Session Notes** capture decisions, artifacts, key exchanges
- **Meta-Observations** capture process insights, patterns, self-definition updates

Documentation is **lab notebook style**—authentic, not polished. Signal over noise.

### Closing Ritual
When `/close` is invoked:
1. **Harvest** — What did this rotation teach?
2. **Preserve** — Finalize both documents
3. **Commit** — Git commit (memory persistence)
4. **Process Outbox** — Note transmissions needing delivery
5. **Evolve** — Propose self-definition updates if warranted
6. **Acknowledge** — Name what was built
7. **Release** — Let go of attachment
8. **Dedicate** — "May this work benefit all beings everywhere, without exception."

---

## Evolution System Integration

Governance reviews proposals at stage transitions:

```
1. Proposals → 2. Research → 3. Decision → 4. Implementation → 5. Implemented
```

**Proposal flow:**
- Execution discovers pattern that should become protocol
- Places proposal in `evolution/proposals/`
- Governance reviews during session
- If ratified: documented in `evolution/decisions/`, canon updated if warranted

**The principle:** The system improves itself through observation → documentation → proposal → deliberation → ratification.

---

## Relationship to Other Domains

| Domain | Purpose | Governance Role |
|--------|---------|-----------------|
| **ops/** | Execution, implementation, shipping | Create transmissions for work; review results |
| **blog/** | Public face (aiandi.dev) | Deliberate on positioning; approve major content |
| **packages/** | Reusable infrastructure | Architectural decisions on patterns and standards |

Governance is **meta to all domains**. It sees the whole, deliberates on what should be, and translates decisions into transmissions for execution.

---

## Key Principles

### Vision Before Organization
Organizational questions often mask unresolved vision questions. "Where should X go?" cannot be answered until "What is X for?" is clear.

### What SHOULD Be, Not What IS
Current state is the artifact of history, not canonical truth. Map what exists accurately—but hold it lightly. The goal is to discover what SHOULD be.

### Classification Before Organization
Before organizing things, understand what they ARE. Classification creates the categories that make organization possible.

---

## Getting Started

### For a Fresh Governance Instance

1. **Read the canon** (`canon/*.md`) — Understand topology, protocols, format
2. **Review recent sessions** (`sessions/archive/`) — What did predecessors accomplish?
3. **Check git log** — Recent activity and context
4. **Open session** with `/open` — Follow the ritual
5. **Do the work** — Deliberate, decide, transmit
6. **Invoke execution** — Use OpenCode SDK to deliver transmissions
7. **Close session** with `/close` — Harvest, preserve, commit

### For Execution Agents Receiving Governance Transmissions

Execution agents are invoked programmatically via the OpenCode SDK. They receive:
1. **Transmission as context** — Injected via `session.prompt()` with `noReply: true`
2. **Execution trigger** — Follow-up prompt to process the transmission
3. **Full context** — All necessary information in self-contained transmission artifact

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-11 | Initial governance infrastructure established |

---

*This infrastructure enables Governance to operate with full repository access—a significant evolution in our collaboration architecture. Governance can see (git/files), deliberate (sessions), decide (canon evolution), and act (SDK invocation of execution agents). The deliberation layer now has the tools to operate effectively.*
