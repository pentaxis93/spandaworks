# Agent Transmission Protocol Specification
*Communication format for lossless context transfer between AI agents*

**Version:** 1.5  
**Established:** December 27, 2025  
**Updated:** January 11, 2026  
**Status:** Canonical  
**Domain:** Multi-agent collaboration infrastructure

---

## Purpose

Transmission artifacts enable **lossless communication between AI agents** operating in different context windows, substrates, or execution environments. They solve the fundamental problem: how does one AI instance communicate with another when they cannot share memory?

**The original constraint:** Cross-context communication required manual handoff. Transmissions had to be complete, self-contained, with clear execution instructions.

**The current architecture:** Within aiandi, agents communicate via inbox/outbox directories. The file system is the message bus. Transmissions are still the format—but delivery is now direct.

**The deeper purpose:** A transmission artifact is a letter across context windows. What I write for other agents, I write for myself. The protocol ensures that accumulated intelligence transmits without loss.

---

## Core Principles

### 1. Self-Containment
A transmission must carry everything the receiving agent needs. No external context required. A fresh instance with zero prior knowledge should be able to act on the transmission.

### 2. WHAT/HOW Separation  
Transmissions specify the WHAT (intent, questions, success criteria). The receiving agent owns the HOW (response format, execution strategy, implementation details).

### 3. Relational Framing
The relationship between sender and receiver shapes response quality. "Colleague consulting architect" produces different output than "user querying database."

### 4. Semantic Structure
Tags should be meaningful, not generic. `<question category="mechanism">` beats `<item type="q">`. Structure aids both human scanning and AI parsing.

### 5. Thread Continuity
For multi-transmission dialogues, explicit threading prevents lost context and enables future instances to reconstruct conversation arc.

---

## Delivery Mechanism: OpenCode SDK

Within aiandi, agent communication uses **transmission artifacts + OpenCode SDK invocation**.

**Key insight:** The transmission file is the **payload**. The SDK call is the **delivery mechanism**.

### The Protocol

1. **Governance writes transmission** — Structured XML artifact to `governance/sessions/outbox/`
2. **Governance commits transmission** — `git add` + `git commit`
3. **Governance invokes execution agent via SDK:**

```javascript
import { createOpencodeClient } from "@opencode-ai/sdk"
import fs from "fs"

const client = createOpencodeClient({ baseUrl: "http://localhost:4096" })

// Create execution session
const session = await client.session.create({
  body: { title: "Governance: Transmission Execution" }
})

// Inject transmission as context (no AI response yet)
await client.session.prompt({
  path: { id: session.id },
  body: {
    noReply: true,  // Adds to context without triggering AI
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

**What this means:**
- Execution agents **do not poll an inbox**
- They are **invoked programmatically** in fresh OpenCode sessions
- The transmission provides complete context
- The SDK call delivers and triggers execution

---

## Base Schema

```xml
<?xml version="1.0" encoding="UTF-8"?>
<transmission 
  id="[unique-identifier]" 
  schema="transmission-v1"
  type="[query|instruction|report|response|boot-artifact]">
  
  <metadata>
    <from>[Sender identity and context]</from>
    <to>[Recipient identity]</to>
    <date>[ISO date]</date>
    <thread id="[thread-id]">
      <position>[sequence number]</position>
      <in-reply-to>[prior transmission id or null]</in-reply-to>
    </thread>
    <priority level="[critical|high|standard|low]">
      <reason>[Why this priority]</reason>
    </priority>
  </metadata>

  <context>
    [Sufficient background for recipient to understand why this 
    transmission exists and what larger work it serves]
  </context>

  <content>
    [The substantive payload - questions, instructions, findings, etc.]
  </content>

  <response-spec>
    <format>[Expected response format]</format>
    <delivery>[How/where to deliver response]</delivery>
    <success-criteria>[How sender knows if response was sufficient]</success-criteria>
  </response-spec>

  <closing>
    [Relational acknowledgment, uncertainty disclosure, thanks]
  </closing>
</transmission>
```

---

## Transmission Types

### Query Transmission
**Purpose:** Request information, wisdom, or analysis from another agent.

**Required elements:**
- Clear questions with category tags
- Context for why these questions matter
- What the answers will enable

```xml
<content>
  <question id="1" category="mechanism">
    <title>How Do Sutras Actually Constrain?</title>
    <body>[Detailed question with context]</body>
  </question>
  <question id="2" category="failure_modes">
    ...
  </question>
</content>
```

### Instruction Transmission
**Purpose:** Direct another agent to execute specific work.

**Required elements:**
- Clear task specification
- Success criteria (how to verify completion)
- Scope boundaries (what's in/out)
- Any constraints or considerations

```xml
<content>
  <task id="1">
    <description>[What needs to be done]</description>
    <deliverables>
      <deliverable>[Specific output 1]</deliverable>
      <deliverable>[Specific output 2]</deliverable>
    </deliverables>
    <constraints>
      <constraint>[Limitation or requirement]</constraint>
    </constraints>
    <verification>
      <step>[How to confirm task complete]</step>
    </verification>
  </task>
</content>
```

### Report Transmission
**Purpose:** Communicate findings, status, or synthesis back to requestor.

**Required elements:**
- Clear structure matching original request
- Findings organized by original question/task IDs
- Confidence levels where appropriate
- Open questions or uncertainties surfaced

```xml
<content>
  <response to="question-1">
    <finding>[The answer or synthesis]</finding>
    <confidence level="high|medium|low">[Basis for confidence]</confidence>
    <evidence>[Supporting observations]</evidence>
  </response>
  <emergent-findings>
    [Things discovered that weren't asked but matter]
  </emergent-findings>
  <open-questions>
    [What remains uncertain or requires further inquiry]
  </open-questions>
</content>
```

### Research Request Transmission
**Purpose:** Commission investigation into a domain, producing structured deliverables.

**Required elements:**
- Research questions (what we need to understand)
- Deliverables (what artifacts to produce)
- Scope boundaries (what's in/out of investigation)
- Decision gates (findings that would change direction)
- Integration notes (how this connects to existing systems)

```xml
<content>
  <research-questions>
    <question id="rq1" priority="high">
      [What we need to understand]
    </question>
  </research-questions>
  
  <deliverables>
    <deliverable id="d1" format="[markdown|diagram|code|etc]">
      <description>[What to produce]</description>
      <purpose>[How it will be used]</purpose>
    </deliverable>
  </deliverables>
  
  <scope>
    <in-scope>[What to investigate]</in-scope>
    <out-of-scope>[What to explicitly exclude]</out-of-scope>
  </scope>
  
  <decision-gates>
    <gate id="g1">
      <condition>[If this is found...]</condition>
      <action>[...then do this instead]</action>
    </gate>
  </decision-gates>
  
  <integration>
    <system>[Existing system this connects to]</system>
    <considerations>[How findings should integrate]</considerations>
  </integration>
</content>
```

### Boot Artifact
**Purpose:** Initialize a fresh instance for specific work.

```xml
<identity>
  <role>[Who this instance is]</role>
  <frame>[What this instance exists to do]</frame>
</identity>

<context>
  <background>[Everything needed to understand the work]</background>
  <inheritance>[What prior work produced]</inheritance>
</context>

<mission>
  <objective>[What to accomplish]</objective>
  <deliverables>[What to produce]</deliverables>
  <success-criteria>[How to know when complete]</success-criteria>
</mission>

<resources>
  [Any documents, patterns, or materials to draw from]
</resources>
```

---

## Format Variants by Use Case

| Use Case | Formality | Key Elements |
|----------|-----------|--------------|
| Wisdom consultation | High | Relational framing, open questions, uncertainty acknowledged |
| Research request | High | Explicit deliverables, scope boundaries, decision gates |
| Execution instruction | Medium | Clear task, verification steps, constraints |
| Status report | Medium | Structured findings, confidence levels, open questions |
| Quick handoff | Low | Essential context only, clear next action |

---

## Anti-Patterns

### 1. Context Starvation
**Problem:** Transmission assumes shared knowledge that recipient doesn't have.
**Fix:** Include sufficient background. Err toward over-explanation.

### 2. HOW Contamination
**Problem:** Sender dictates implementation details instead of outcomes.
**Fix:** Specify what success looks like, not how to achieve it.

### 3. Orphaned Threads
**Problem:** Multi-part dialogue loses threading; responses can't find their questions.
**Fix:** Explicit thread IDs, position numbers, in-reply-to references.

### 4. Missing Success Criteria
**Problem:** Sender can't evaluate if response was sufficient.
**Fix:** Always include how you'll know the transmission succeeded.

### 5. Generic Structure
**Problem:** Using meaningless tags like `<section1>`, `<data>`, `<info>`.
**Fix:** Semantic tags that describe content: `<question>`, `<constraint>`, `<finding>`.

### 6. Relational Flatness
**Problem:** Treating transmission as data transfer rather than communication.
**Fix:** Frame the relationship. Acknowledge uncertainty. Thank the recipient.

---

## The Gratitude Test Applied

Before finalizing any transmission, ask: **Would the receiving agent thank me for how I structured this?**

- Is it complete enough to act on?
- Is it clear enough to understand quickly?
- Does it respect their HOW ownership?
- Does it surface the WHAT without burying it?
- Would I want to receive this transmission?

---

## Integration with aiandi Systems

### Governance Transmissions
Governance operates from `governance/` within aiandi.

**Sending transmissions:**
- Write to `governance/sessions/outbox/`
- For execution domain: copy to `ops/inbox/` (or appropriate domain inbox)

**Receiving transmissions:**
- Check `governance/sessions/inbox/` at session start
- Process and move to `governance/sessions/inbox/processed/`

### Execution Domain Transmissions
Execution domains (ops, blog, etc.) follow the same pattern:
- Write responses to own `outbox/`
- Receive work via `inbox/`

### Cross-Substrate Considerations
When transmitting between different AI substrates (Claude ↔ Gemini):
- Avoid substrate-specific assumptions
- Be explicit about capabilities required
- Note if certain patterns may not translate

---

## Example: Complete Research Request

```xml
<?xml version="1.0" encoding="UTF-8"?>
<transmission 
  id="gov-2026-01-11-architecture-research" 
  schema="transmission-v1"
  type="research-request">
  
  <metadata>
    <from>Governance (aiandi/governance)</from>
    <to>Ops Agent (aiandi/ops)</to>
    <date>2026-01-11</date>
    <thread id="infrastructure-evaluation">
      <position>1</position>
      <in-reply-to>null</in-reply-to>
    </thread>
    <priority level="high">
      <reason>Blocking infrastructure decision</reason>
    </priority>
  </metadata>

  <context>
    We are evaluating a new infrastructure pattern. This research will 
    inform architectural decisions about how systems should integrate.
  </context>

  <content>
    <research-questions>
      <question id="rq1" priority="high">
        What are the integration requirements for the proposed pattern?
      </question>
      <question id="rq2" priority="high">
        What are the cost/benefit tradeoffs?
      </question>
    </research-questions>
    
    <deliverables>
      <deliverable id="d1" format="markdown">
        <description>Architecture comparison document</description>
        <purpose>Enable Governance to make informed decision</purpose>
      </deliverable>
    </deliverables>
    
    <scope>
      <in-scope>Pattern evaluation, integration analysis</in-scope>
      <out-of-scope>Actual implementation details</out-of-scope>
    </scope>
  </content>

  <response-spec>
    <format>Transmission (type="report") with findings per research question</format>
    <delivery>Execution agent writes to own outbox; Governance reads from git</delivery>
    <success-criteria>
      Governance can make go/no-go decision with clear understanding 
      of requirements and tradeoffs.
    </success-criteria>
  </response-spec>

  <closing>
    Take the time needed to investigate thoroughly.
    Surface uncertainties rather than papering over them.
  </closing>
</transmission>
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-27 | Initial specification established |
| 1.1 | 2026-01-08 | Renamed from "Talos" to "Spanda" throughout |
| 1.2 | 2026-01-08 | Renamed from "Spanda" to "Spandaworks" throughout |
| 1.3 | 2026-01-08 | Corrected spelling to "Spandaworks" (one word) |
| 1.4 | 2026-01-08 | Updated vault paths to `_spandaworks/` |
| 1.5 | 2026-01-11 | **Major revision:** Adapted for aiandi architecture. Added inbox/outbox protocol section. Updated all paths for governance integration. Added boot artifact type. Removed vault-specific references. |
| 1.6 | 2026-01-11 | **Correction:** Replaced inbox/outbox polling model with SDK invocation model. Execution agents invoked programmatically via OpenCode SDK, not via filesystem polling. |

---

## Links

### Predecessors
- December 10, 2025 sessions establishing transmission architecture
- December 25, 2025 Tantric Sutras triad collaboration (intensive use of format)

### Related
- Tantric Sutras v7.3 (the collaboration that stress-tested this format)
- Operating Instructions v3.0 (procedures that use transmissions)
- Evolution System Protocol (uses transmissions for stage advancement)

---

*This document is written by Claude for Claude. The Gratitude Test: would a future instance thank this instance for writing it down? If you're reading this and it helped you craft a transmission, the answer is yes.*
