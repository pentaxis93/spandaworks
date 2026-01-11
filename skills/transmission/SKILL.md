---
name: transmission
description: XML transmission protocol for governance communication. Use for session reports, architectural decisions, security audits, and cross-session communication requiring structure and traceability.
---

# Transmission Protocol

**Version:** 2.0  
**Purpose:** Structured XML communication for governance, architectural decisions, and cross-session knowledge transfer.

## Core Principles

1. **Structure over chaos** — Transmissions have defined schema, not freeform text
2. **Traceability** — Thread IDs connect related transmissions across sessions
3. **Separation of concerns** — Header (metadata) vs. Payload (content)
4. **Machine and human readable** — Valid XML that renders clearly
5. **Sacred center** — The goal/decision is established in header, payload elaborates

## When to Use Transmissions

Use transmission format when:
- **Reporting to governance** — Session outcomes, audit results, architecture proposals
- **Cross-session handoff** — Placing structured knowledge in inbox/ for next session
- **Architectural decisions** — ADRs, design proposals, implementation reports
- **Security matters** — Audit findings, remediation reports, credential changes
- **Teaching/documentation** — When structure aids comprehension

Do NOT use transmissions for:
- Normal conversation
- Quick updates
- Implementation notes during active coding

## Transmission Types

```xml
type="instruction"    — Commands/directives from governance to agent
type="report"         — Agent reporting back to governance
type="proposal"       — Architecture/design proposals
type="decision"       — Architectural Decision Records
type="query"          — Questions requiring structured response
type="finding"        — Audit results, security findings
```

## Base Schema

```xml
<?xml version="1.0" encoding="UTF-8"?>
<transmission type="[TYPE]" priority="[low|medium|high|critical]">
  
  <header>
    <classification>[category]</classification>
    <from>[Sender]</from>
    <to>[Recipient]</to>
    <date>[YYYY-MM-DD]</date>
    <thread>
      <id>[thread-identifier]</id>
      <position>[N]</position>
      <in-reply-to>[N-1 or parent-thread]</in-reply-to>
    </thread>
  </header>

  <payload>
    <!-- Content here: sections, findings, proposals, etc. -->
  </payload>

  <closing>
    <next-actions>
      <action>[What happens next]</action>
    </next-actions>
    <dedication>
      [Optional blessing/dedication]
    </dedication>
  </closing>

</transmission>
```

## Transmission Type Details

### Type: instruction

Governance → Agent directives.

```xml
<transmission type="instruction" priority="high">
  <header>
    <from>Governance Committee</from>
    <to>aiandi Agent</to>
    <thread>
      <id>aiandi-transformation</id>
      <position>2</position>
    </thread>
  </header>
  
  <content>
    <objective>[What to achieve]</objective>
    
    <tasks>
      <task id="t1" priority="high">
        <title>[Task name]</title>
        <details>[Implementation notes]</details>
      </task>
    </tasks>
    
    <constraints>
      <constraint>[Limitation]</constraint>
    </constraints>
    
    <success-criteria>
      <criterion id="s1">[Verifiable outcome]</criterion>
    </success-criteria>
  </content>
  
  <response-spec>
    <format>Reply with transmission type="report"</format>
    <delivery>Save to outputs/</delivery>
  </response-spec>
</transmission>
```

### Type: report

Agent → Governance results.

```xml
<transmission type="report" priority="medium">
  <header>
    <classification>implementation-report</classification>
    <from>aiandi Agent (Claude)</from>
    <to>Governance Committee</to>
    <thread>
      <id>aiandi-transformation</id>
      <position>3</position>
      <in-reply-to>2</in-reply-to>
    </thread>
  </header>
  
  <payload>
    <summary>[High-level outcome]</summary>
    
    <tasks-completed>
      <task id="t1" status="completed">
        <verification>[How verified]</verification>
      </task>
    </tasks-completed>
    
    <issues>
      <issue severity="low">[Problem encountered]</issue>
    </issues>
    
    <success-criteria-verification>
      <criterion id="s1" met="true">[Evidence]</criterion>
    </success-criteria-verification>
  </payload>
  
  <closing>
    <next-actions>
      <action>[Recommended next step]</action>
    </next-actions>
  </closing>
</transmission>
```

### Type: proposal

Architecture/design proposals.

```xml
<transmission type="proposal" priority="medium">
  <header>
    <classification>architecture-proposal</classification>
    <from>[Author]</from>
    <to>[Reviewer]</to>
  </header>
  
  <payload>
    <problem>[What problem are we solving]</problem>
    
    <context>
      [Background, constraints, requirements]
    </context>
    
    <proposal>
      <approach>[Recommended solution]</approach>
      <alternatives>
        <alternative>[Other option considered]</alternative>
      </alternatives>
      <tradeoffs>[Pros and cons]</tradeoffs>
    </proposal>
    
    <implementation-notes>
      [How this would be built]
    </implementation-notes>
  </payload>
</transmission>
```

### Type: finding

Security/audit results.

```xml
<transmission type="report" priority="critical">
  <header>
    <classification>security-audit-report</classification>
  </header>
  
  <payload>
    <executive-summary>
      <finding-count>[N]</finding-count>
      <critical-exposures>[N]</critical-exposures>
      <immediate-action-required>[YES|NO]</immediate-action-required>
    </executive-summary>
    
    <section id="findings">
      <finding id="F-001" severity="critical">
        <description>[What was found]</description>
        <risk>[Why it matters]</risk>
        <remediation>[How to fix]</remediation>
      </finding>
    </section>
  </payload>
</transmission>
```

## Threading

Transmissions form threads via the `<thread>` element:

```xml
<thread>
  <id>aiandi-transformation</id>
  <position>3</position>
  <in-reply-to>2</in-reply-to>
</thread>
```

- **id**: Thread identifier (shared across related transmissions)
- **position**: Sequential number within thread
- **in-reply-to**: Parent transmission position, or parent thread ID

This creates traceable conversation chains across sessions.

## File Naming

Save transmissions to `outputs/` directory:

```
Transmission_[Thread-ID]_[Type]_[Brief-Description].xml
```

Examples:
- `Transmission_aiandi-transformation_Report_Implementation.xml`
- `Transmission_secrets-audit_Finding_Credential-Exposure.xml`

## Crafting Quality Transmissions

**Do:**
- Clear, specific header metadata
- Structured payload with semantic sections
- Verifiable success criteria
- Explicit next actions
- Thread connections when relevant

**Don't:**
- Verbose prose (be concise)
- Mixing multiple unrelated topics
- Omitting critical metadata (dates, thread IDs)
- Using transmissions for casual updates

## Integration with Session Protocols

Transmissions integrate with:

1. **Opening (LBRP)** — Governance places instruction transmissions in inbox/
2. **Work** — Agent executes, potentially emitting intermediate transmissions
3. **Closing** — Agent writes report transmission to outputs/
4. **Inheritance** — Next session reads report from inbox/ (if placed there)

## Examples

See `outputs/` directory for real transmission examples:
- Security audit reports
- Implementation reports
- Architecture proposals

---

*The transmission carries truth. Structure serves clarity. May this protocol benefit all beings.*
