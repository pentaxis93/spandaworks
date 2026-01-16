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
  description: string    # What this agent does (for routing/docs) [REQUIRED]
  
  # Mode layer
  mode: primary | subagent | all   # How agent can be used [default: all]
  hidden: boolean                   # Hide from @ autocomplete [default: false]
  
  # Capability layer
  tools:
    enabled: [tool_names]
    disabled: [tool_names]
  
  # Instruction layer  
  instructions: |
    Domain expertise, reasoning patterns, and behavioral guidance.
    NOT a persona or backstory.
  
  # Constraint layer (OpenCode options - not used in aiandi)
  # guardrails, permissions, temperature, maxSteps
    
  # Orchestration layer
  permission:
    task:                  # Controls which subagents this agent can invoke
      "*": allow | ask | deny
      agent_pattern: allow | ask | deny
  
  # Resource layer
  model: provider/model-id  # Model for this agent [REQUIRED for subagents]
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
    Every task needs status: +next, +inbox, +someday, +waiting, or +incubating.
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

The following 5 agents form the canonical subagent types for aiandi, replacing OpenCode's built-in types. Each agent specifies a fixed model—this is the **only mechanism** for controlling which model executes a delegated task.

| Agent | Function | Model | Tools | Can Delegate To |
|-------|----------|-------|-------|-----------------|
| `explore` | Fast codebase search | Haiku | read, glob, grep | none |
| `researcher` | Web research + synthesis | Sonnet | read, webfetch | explore |
| `builder` | Implementation | Opus | full | explore |
| `reviewer` | Code analysis | Sonnet | read, glob, grep | none |
| `documenter` | Documentation | Sonnet | read, write, edit | none |

**Common Configuration (all agents):**
- `mode: subagent` — Invoked via Task tool, not primary agents
- `hidden: true` — Not visible in `@` autocomplete; only Governance invokes them

**Model Selection Rationale:**
- **Haiku** (`explore`): Token-efficient for high-volume codebase scanning
- **Sonnet** (`researcher`, `reviewer`, `documenter`): Balanced capability for synthesis, evaluation, and writing
- **Opus** (`builder`): Maximum reasoning capability for code implementation

**Delegation Topology:**
```
Governance (primary)
    ├── explore      (leaf - no delegation)
    ├── researcher   (can delegate to explore)
    ├── builder      (can delegate to explore)
    ├── reviewer     (leaf - no delegation)
    └── documenter   (leaf - no delegation)
```

Flat orchestration: Governance is the sole orchestrator. `researcher` and `builder` may delegate codebase exploration to `explore` for efficiency (Haiku vs Sonnet/Opus for search). All other agents are leaf nodes.

**Routing Rules:**
- `explore` vs `researcher`: Codebase (explore) vs external/web (researcher)
- `builder` vs `documenter`: Code changes (builder) vs pure documentation (documenter)
- `builder` vs `reviewer`: Creating/modifying (builder) vs evaluating (reviewer)

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

### 6.4 Implementation: Agent Embedding

Agents are embedded in the `aiandi` binary and extracted via `aiandi init`, following the same pattern as skills.

#### Source Structure

```
agents/
  explore.md
  researcher.md
  builder.md
  reviewer.md
  documenter.md
```

Each agent file is OpenCode-compatible markdown with YAML frontmatter:

```markdown
---
description: Fast codebase search and pattern discovery
mode: subagent
hidden: true
model: anthropic/claude-3-5-haiku-20241022
tools:
  write: false
  edit: false
  bash: false
permission:
  task:
    "*": deny
---

[Agent instructions...]
```

#### Rust Module (`crates/aiandi/src/agents.rs`)

```rust
//! Bundled agents embedded at compile time.

pub const EXPLORE_AGENT: &str = include_str!("../../../agents/explore.md");
pub const RESEARCHER_AGENT: &str = include_str!("../../../agents/researcher.md");
pub const BUILDER_AGENT: &str = include_str!("../../../agents/builder.md");
pub const REVIEWER_AGENT: &str = include_str!("../../../agents/reviewer.md");
pub const DOCUMENTER_AGENT: &str = include_str!("../../../agents/documenter.md");

pub struct BundledAgent {
    pub name: &'static str,
    pub content: &'static str,
}

pub fn bundled_agents() -> Vec<BundledAgent> {
    vec![
        BundledAgent { name: "explore", content: EXPLORE_AGENT },
        BundledAgent { name: "researcher", content: RESEARCHER_AGENT },
        BundledAgent { name: "builder", content: BUILDER_AGENT },
        BundledAgent { name: "reviewer", content: REVIEWER_AGENT },
        BundledAgent { name: "documenter", content: DOCUMENTER_AGENT },
    ]
}
```

#### Init Command Extension

Extend `crates/aiandi/src/commands/init.rs`:

1. Create `.opencode/agent/` directory
2. Extract agents to `.opencode/agent/{name}.md` (flat, not nested)
3. Add `--no-agents` flag for selective installation
4. Add `--agents=a,b,c` for specific agent selection

**Note:** OpenCode expects agent files directly in `.opencode/agent/` as `{name}.md`, unlike skills which use `{name}/SKILL.md`.

#### Installation Result

After `aiandi init`:

```
.opencode/
  agent/
    explore.md
    researcher.md
    builder.md
    reviewer.md
    documenter.md
  skill/
    governance/
      SKILL.md
    gtd/
      SKILL.md
    transmission/
      SKILL.md
```

### 6.5 Governance Routing Skill

Since all 5 subagents are hidden (`hidden: true`), the primary session needs explicit routing rules. The existing `governance` skill (`.opencode/skill/governance/SKILL.md`) must be **updated** to include routing rules for the new agent catalog.

**Current state:** The skill references `subagent_type: "general"` for all agent invocations.

**Required update:** Replace generic agent invocation with specific routing to the 5 specialized agents.

#### Updated Agent Invocation Section (for `skills/governance/SKILL.md`)

```markdown
---
name: governance
description: Routing rules for Governance sessions. Defines when to delegate to subagents and which subagent for which task type.
---

# Governance Routing

## Available Subagents

| Agent | Model | Purpose |
|-------|-------|---------|
| `explore` | Haiku | Fast codebase search, pattern discovery |
| `researcher` | Sonnet | Web research, external documentation, API investigation |
| `builder` | Opus | Code implementation, feature development, bug fixes |
| `reviewer` | Sonnet | Code review, security audit, quality analysis |
| `documenter` | Sonnet | Documentation writing, README updates, inline docs |

## When to Delegate

**Delegate when:**
- Task is self-contained (clear input → output)
- Task benefits from model specialization (Haiku for search, Opus for implementation)
- Task is parallelizable with other work
- Task doesn't require iterative conversation with user

**Do NOT delegate when:**
- Task requires back-and-forth with user
- Task needs context accumulated in current session
- Task is trivial (faster to do directly than to formulate delegation)
- Task outcome is uncertain and may need pivoting

## Routing Decision Tree

```
1. Does task require web access (external docs, APIs)?
   YES → researcher
   
2. Does task modify or create code?
   YES → builder
   
3. Does task evaluate/review existing code?
   YES → reviewer
   
4. Does task write documentation only (no code)?
   YES → documenter
   
5. Does task search/explore the codebase?
   YES → explore
   
6. None of the above?
   → Do directly (no delegation)
```

## Invocation Pattern

Use the Task tool to invoke subagents:

```
mcp_task(
  description: "Brief task description",
  prompt: "Complete task specification with context and success criteria",
  subagent_type: "{agent_name}"
)
```

**Critical:** The `subagent_type` must exactly match one of: `explore`, `researcher`, `builder`, `reviewer`, `documenter`.

## Parallel Delegation

Multiple independent tasks can be delegated in parallel:

```
# In a single response, invoke multiple Task tools:
mcp_task(subagent_type: "explore", prompt: "Find all API endpoints...")
mcp_task(subagent_type: "researcher", prompt: "Research OAuth 2.0 best practices...")
```

Results return to Governance for synthesis and next steps.

## Delegation Topology

```
Governance (you)
    ├── explore      (can't delegate further)
    ├── researcher   → explore (can delegate codebase search)
    ├── builder      → explore (can delegate codebase search)
    ├── reviewer     (can't delegate further)
    └── documenter   (can't delegate further)
```

Only `researcher` and `builder` can sub-delegate to `explore`. All other agents are leaf nodes.
```

#### Implementation

1. **Update existing skill** at `.opencode/skill/governance/SKILL.md`
2. **Update source** at `skills/governance/SKILL.md` (for embedding)
3. **Add to `skills.rs`** if not already present:

```rust
pub const GOVERNANCE_SKILL: &str = include_str!("../../../skills/governance/SKILL.md");
```

4. **Add to `bundled_skills()`**:

```rust
BundledSkill { name: "governance", content: GOVERNANCE_SKILL },
```

The `/governance` command already loads this skill, so no command changes are needed—only the skill content must be updated with routing rules.

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

~~Should agents specify their own models, or should orchestration control model selection based on task requirements?~~

**RESOLVED:** Agent-defined. Each agent specifies a fixed model. This is the **only mechanism** within OpenCode for controlling which model executes a delegated task. Governance selects the appropriate agent based on task requirements, which implicitly selects the model.

See Section 6.2 for the canonical agent-to-model mapping.

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
