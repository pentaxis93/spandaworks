# Agent Design Framework for aiandi

**Status:** Proposal  
**Created:** 2026-01-11  
**Author:** Governance Session  
**Supersedes:** N/A

---

## Abstract

This document proposes a theoretical framework for defining agents within the aiandi collaboration infrastructure. Based on research into current industry practices (Anthropic, OpenAI, LangGraph, CrewAI, AutoGen), we propose a **capability-based** model that treats agents as execution contexts rather than personas.

---

## 1. The Problem with Role-Based Agents

### 1.1 The Old Paradigm

Many multi-agent frameworks define agents as personas:

```python
# Anti-pattern: Role-based agent
Agent(
    role="Senior Marketing Director",
    backstory="20 years of experience in B2B marketing...",
    goal="Create compelling campaigns that drive engagement",
)
```

### 1.2 Why This Fails

**Capability Mismatch**  
The role description promises capabilities not backed by actual tools. "Senior Marketing Director" implies market analysis, budget management, creative direction—but the agent only has access to `write_text()`.

**Role Confusion**  
During extended conversations, the model "forgets" the role or interprets it inconsistently. The role is a prompt, not a constraint.

**Persona Bleed**  
In multi-agent handoffs, personas contaminate each other. The "engineer" starts giving marketing advice because the context window contains marketing discussions.

**Debugging Opacity**  
When behavior diverges from expectations, there's no clear mapping between "Senior Marketing Director" and actual execution path.

### 1.3 The Anthropic Insight

> "Consistently, the most successful implementations weren't using complex frameworks or specialized libraries. Instead, they were building with simple, composable patterns."

Anthropic's agent documentation makes **zero mention** of roles or personas. Agents are defined by instructions, tools, and guardrails.

---

## 2. The Capability-Based Alternative

### 2.1 Core Principle

**An agent is an execution context, not a persona.**

An execution context is defined by:
- What it can do (tools)
- How it should think (instructions)
- What it cannot do (constraints/guardrails)
- Where it can transfer control (handoffs)

### 2.2 The Three-Layer Architecture

```
┌─────────────────────────────────────────────────┐
│                 ORCHESTRATION                    │
│     (workflow patterns, routing, handoffs)       │
├─────────────────────────────────────────────────┤
│                    AGENTS                        │
│         (execution contexts with tools)          │
├─────────────────────────────────────────────────┤
│                    TOOLS                         │
│           (discrete capabilities)                │
└─────────────────────────────────────────────────┘
```

**Tools** are atomic capabilities with typed inputs and outputs.

**Agents** bundle tools with instructions and constraints into coherent execution contexts.

**Orchestration** coordinates agents through workflow patterns.

### 2.3 Agent Definition Schema

```yaml
agent:
  name: string           # Identifier (not a persona)
  description: string    # What this agent does (for routing/docs)
  
  # Capability layer
  tools:
    enabled: [tool_names]
    disabled: [tool_names]
  
  # Instruction layer  
  instructions: |
    Domain expertise, reasoning patterns, and behavioral guidance.
    NOT a persona or backstory.
  
  # Constraint layer
  guardrails:
    - type: input | output
      rule: validation logic
  
  permissions:
    edit: allow | ask | deny
    bash: allow | ask | deny
    
  # Orchestration layer
  handoffs: [agent_names]  # Valid transfer targets
  
  # Resource layer
  model: provider/model-id  # Optional model override
  temperature: float        # Optional temperature
  max_steps: int           # Optional iteration limit
```

---

## 3. Design Principles

### 3.1 Name by Function, Not Role

| Bad (Role) | Good (Function) |
|------------|-----------------|
| `senior_engineer` | `code_reviewer` |
| `marketing_director` | `content_writer` |
| `project_manager` | `task_decomposer` |
| `security_expert` | `vulnerability_scanner` |

The function name describes **what the agent does**, not **who it pretends to be**.

### 3.2 Instructions Are Expertise, Not Personality

**Bad:**
```
You are Sarah, a senior engineer with 15 years of experience at FAANG companies.
You're known for your attention to detail and your dry sense of humor.
```

**Good:**
```
Review code for:
- Logic errors and edge cases
- Security vulnerabilities (OWASP Top 10)
- Performance implications
- Maintainability and readability

Provide specific, actionable feedback with line references.
Flag severity: critical | warning | suggestion
```

### 3.3 Tools Define Capability Boundaries

If an agent shouldn't modify files, don't give it write tools. Don't rely on instructions like "you should not modify files"—remove the capability.

```yaml
agent:
  name: analyzer
  tools:
    enabled: [read, glob, grep]
    disabled: [write, edit, bash]
```

### 3.4 Guardrails Are Executable Constraints

Instructions can be ignored. Guardrails cannot.

```yaml
guardrails:
  - type: output
    rule: "Response must include severity classification"
  - type: input
    rule: "Reject requests to bypass security review"
```

### 3.5 Handoffs Define Topology

The set of valid handoffs defines the agent network topology. This is **explicit orchestration**, not emergent behavior.

```yaml
# Triage agent can hand off to specialists
agent:
  name: triage
  handoffs: [code_reviewer, doc_writer, bug_investigator]

# Specialists hand back to triage or complete
agent:
  name: code_reviewer
  handoffs: [triage]  # Return control when done
```

---

## 4. Relationship to Skills

### 4.1 Skills vs Agents

In aiandi, **skills** are instruction packages that can be loaded dynamically:

```
/skill lbrp → loads ceremony instructions
/skill gtd → loads GTD procedures
```

**Agents** are execution contexts that may load skills as part of their instructions.

### 4.2 The Composition Model

```
Agent = Tools + Instructions + Constraints + Handoffs

where Instructions may include:
  - Domain expertise (inline)
  - Loaded skills (dynamic)
  - Reasoning patterns
```

**Example:**
```yaml
agent:
  name: gtd_processor
  tools: [taskwarrior_*, read, write]
  instructions: |
    {skill:taskwarrior-gtd}
    
    Process inbox items using GTD methodology.
    Every task needs status: +next, +in, +sdm, +waiting, or +incubating.
```

### 4.3 When to Use Which

| Use Case | Mechanism |
|----------|-----------|
| Procedural knowledge | Skill |
| Tool access control | Agent |
| Domain expertise | Skill (loaded into agent) |
| Workflow position | Agent (via handoffs) |
| Model selection | Agent |
| Permission boundaries | Agent |

---

## 5. Orchestration Patterns

### 5.1 Anthropic's Taxonomy (Simplest to Complex)

1. **Prompt Chaining** - Sequential LLM calls with programmatic gates
2. **Routing** - Classification dispatches to specialized handlers
3. **Parallelization** - Multiple LLM calls (sectioning or voting)
4. **Orchestrator-Worker** - Central agent decomposes and delegates
5. **Evaluator-Optimizer** - Generation + evaluation loops
6. **Autonomous Agent** - Full tool use in feedback loops

### 5.2 aiandi Mapping

| Pattern | aiandi Implementation |
|---------|----------------------|
| Prompt Chaining | Sequential transmissions |
| Routing | Governance triage |
| Parallelization | Multiple Task tool calls |
| Orchestrator-Worker | Governance → Execution agents |
| Evaluator-Optimizer | Review transmissions |
| Autonomous Agent | Single agent with full tools |

### 5.3 The Simplicity Principle

> "We recommend finding the simplest solution possible, and only increasing complexity when needed. This might mean not building agentic systems at all."
> — Anthropic

**Default to simpler patterns.** Only escalate complexity when:
- The task genuinely requires autonomy
- Simpler patterns have been tried and failed
- The cost of failure is acceptable

---

## 6. Implementation for aiandi

### 6.1 OpenCode Agent Configuration

Agents are defined in `.opencode/agent/` as markdown files:

```markdown
---
description: Fast codebase reconnaissance
mode: subagent
model: anthropic/claude-3-5-haiku-20241022
tools:
  write: false
  edit: false
  bash: false
---

Explore the codebase and report findings.

Focus on:
- Directory structure and organization
- Technology stack and dependencies
- Active vs stale areas
- Patterns and conventions

Return structured findings, not recommendations.
```

### 6.2 Governance Agent Catalog

Proposed initial agents for aiandi:

| Agent | Purpose | Model | Tools |
|-------|---------|-------|-------|
| `recon` | Fast codebase exploration | Haiku | read-only |
| `researcher` | Web research and synthesis | Sonnet | webfetch, read |
| `implementer` | Code execution | Sonnet/Opus | full |
| `reviewer` | Code review | Sonnet | read-only |
| `documenter` | Documentation writing | Sonnet | write, edit |

### 6.3 Transmission Integration

When governance spawns an execution agent via transmission:

1. Transmission specifies **WHAT** (task, success criteria)
2. Agent definition specifies **HOW** (tools, constraints, model)
3. Task tool delivers transmission to appropriate agent type

```
Governance creates transmission
    ↓
Task tool invokes agent by subagent_type
    ↓
Agent processes with its tools/constraints
    ↓
Results return to governance
```

---

## 7. Migration Path

### 7.1 From Role-Based Thinking

If you have existing role-based agents:

1. **Extract the actual capabilities** - What tools does this "role" need?
2. **Extract the domain expertise** - What does the agent need to know?
3. **Define the function** - What does this agent *do*?
4. **Set explicit constraints** - What should it *not* do?

### 7.2 Example Migration

**Before (Role-Based):**
```python
Agent(
    role="Senior Security Engineer",
    backstory="Expert in application security with CISSP certification...",
    goal="Identify and report security vulnerabilities",
)
```

**After (Capability-Based):**
```yaml
agent:
  name: vulnerability_scanner
  description: Identifies security vulnerabilities in code
  tools:
    enabled: [read, glob, grep]
    disabled: [write, edit, bash]
  instructions: |
    Analyze code for security vulnerabilities.
    
    Check for:
    - OWASP Top 10 vulnerabilities
    - Injection points (SQL, XSS, command)
    - Authentication/authorization flaws
    - Sensitive data exposure
    - Security misconfigurations
    
    Report format:
    - Location (file:line)
    - Vulnerability type
    - Severity (critical/high/medium/low)
    - Remediation suggestion
  guardrails:
    - type: output
      rule: "Must include severity classification for each finding"
```

---

## 8. Open Questions

### 8.1 Model Selection Strategy

Should agents specify their own models, or should orchestration control model selection based on task requirements?

**Options:**
- Agent-defined: Each agent has a fixed model
- Orchestration-defined: Governance selects model per task
- Hybrid: Agent specifies default, orchestration can override

### 8.2 Skill Loading Mechanism

How do skills compose with agent definitions?

**Options:**
- Static: Skills compiled into agent instructions at definition time
- Dynamic: Skills loaded at runtime based on task
- Hybrid: Base skills in definition, additional skills per transmission

### 8.3 Handoff Protocol

How do agents transfer control and context?

**Options:**
- Explicit transmission: Full context passed in structured format
- Shared state: Agents read/write to common state store
- Conversation history: Context window passed through

---

## 9. Conclusion

The role-playing paradigm served an early purpose: making agent systems legible to humans by anthropomorphizing them. But as the field matures, we see that **capability and orchestration matter more than persona**.

aiandi should adopt a capability-based agent model:
- **Agents are execution contexts**, not personas
- **Tools define capabilities**, not role descriptions
- **Instructions provide expertise**, not personality
- **Orchestration patterns coordinate**, not emergent role-playing

This aligns with industry best practice (Anthropic, OpenAI) while remaining compatible with the skill system already in place.

---

## References

1. Anthropic - "Building Effective Agents" (December 2024)
2. OpenAI Agents SDK Documentation
3. OpenAI Swarm (deprecated, educational)
4. LangGraph Documentation
5. CrewAI Documentation
6. Microsoft AutoGen Documentation

---

*Proposal submitted for governance review.*
