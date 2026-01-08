---
name: spandaworks-docs
description: "Guidelines for writing documentation for Spandaworks. Encode lessons learned from README refactoring. Use when creating or revising project documentation."
---

# Spandaworks Documentation Skill

Guidelines for writing documentation that serves strangers, not insiders.

Derived from the 2026-01-07 README refactoring session.

## When to Use This Skill

- Creating new documentation (README, package docs, guides)
- Revising existing documentation
- Reviewing documentation for clarity
- When documentation feels esoteric or confusing

## Core Principles

### 1. Speak to Strangers, Not to Ourselves

**Bad:**
```markdown
This invokes the LBRP ceremony:
1. Inherit knowledge from previous sessions
2. Observe current workspace state
3. Define goal through refinement dialogue
4. Banish workspace debris
5. Prepare quarters: Context (East), Tasks (South), Workspace (West), Environment (North)
```

**Good:**
```markdown
```bash
/open "Implement authentication system"
```
```

**Why:** Occult terminology (LBRP, banishing, cardinal directions) confuses strangers. Show WHAT happens, not HOW the ceremony works internally.

**Test:** Would someone unfamiliar with the project understand this without additional context?

---

### 2. Honesty Over Marketing

**Bad:**
```markdown
### Prerequisites
- MCP-compatible client (OpenCode, Claude Desktop, etc.)
```

**Good:**
```markdown
### Prerequisites
- [OpenCode](https://opencode.ai) (required)
```

**Why:** OpenCode isn't optional—the ceremony infrastructure requires it. Be honest about dependencies.

**Test:** Are we hedging or being direct?

---

### 3. Technical Differentiators, Not Philosophy

**Bad:**
```markdown
### Epistemic Humility

The system tracks what can be verified, not what might be inferred.
- Confidence scores, not certainty claims
- Patterns can be wrong. The graph allows contradictions.
```

**Good:**
```markdown
**Knowledge persists across context windows.**
Telemetry maintains a Kuzu graph with 19 entity types and 25 relationship types. 
INHERITED edges capture what the agent knew at session start.
```

**Why:** Philosophy is a stance. Features are capabilities. Document what the system DOES, not what it believes.

**Test:** Does this describe a concrete capability or an abstract principle?

---

### 4. Cut Redundancy Ruthlessly

**Bad:**
```markdown
### How They Connect
- MCP servers expose capabilities
- Event schemas define communication
- Each package is independent

## What Makes This Different
**Packages connect through protocols, not imports.**
No cross-package dependencies. MCP servers expose capabilities.
```

**Good:**
```markdown
## What Makes This Different
**Packages connect through protocols, not imports.**
No cross-package dependencies. MCP servers expose capabilities.
```

**Why:** Saying the same thing twice wastes attention. Say it once, clearly.

**Test:** Is this information already stated elsewhere?

---

### 5. Metaphors Must Earn Their Keep

**Bad:**
```markdown
Why "organism"?

Because the packages don't just integrate. They recognize each other.
```

**Good:**
```markdown
[Section removed entirely]
```

**Why:** "Organs recognizing each other" is nonsensical. Metaphors that confuse should be cut.

**Test:** Does this metaphor clarify or obscure? Can you defend the connection?

---

### 6. No Tech Debt in Naming

**Bad:**
```markdown
## The Name

[Ancient mythological reference with forced connection to self-observation...]

The name was chosen because [elaborate justification that doesn't quite fit]...
```

**Good:**
```markdown
## The Name

Spanda (स्पन्द) means "creative pulsation"—the throb of consciousness recognizing itself. 
The name reflects what happens when AI and human work together: something lights up.
```

**Why:** Names that require elaborate justification signal misalignment. Better to choose a name that fits naturally.

**Test:** Does this name actually fit what the system does?

---

### 7. Document Current Reality, Not Aspirations

**Bad:**
```markdown
## Project Status

**What works now:**
- ✅ GTD MCP server (22 tools)
- ✅ Telemetry knowledge graph (126 tests passing)

**In progress:**
- 🚧 Telemetry MCP server (stdio wrapper needed)
- 🚧 Event schema implementation
```

**Good:**
```markdown
## Project Status

**Experimental. Incomplete. Expect broken things.**

Packages work individually but integration is partial. The knowledge graph exists 
but doesn't feed back into sessions yet. Event schemas are defined but not implemented.
```

**Why:** Detailed roadmaps age poorly. Strangers need to know what works NOW, not what might work someday.

**Test:** Is this describing current reality or future plans?

---

## Anti-Patterns to Avoid

### Writing Instructions to the Model

**Bad:**
```markdown
### During the Session

Work normally. The MCP tools are available:
- `search_emails` - Find that email from Sarah
```

**Why:** This reads like instructions TO the AI, not documentation FOR humans.

**Fix:** Describe capabilities, not example queries.

### Sutra References in User Docs

**Bad:**
```markdown
This is Sutra 5: Every operation refines both object AND process.
```

**Why:** Sutras are internal topology for AI agents, not user-facing concepts.

**Fix:** State the principle plainly without referencing internal architecture.

### Explaining What You Don't Need to Justify

**Bad:**
```markdown
## Why Rust?
- Safety: Memory safety without GC
- Performance: Speed matters for CLI wrapping
- Modern tooling: Cargo workspaces, edition 2024
```

**Why:** If you're migrating to Rust, just use Rust. Don't justify language choices in user docs.

**Fix:** Remove the section. The architecture table shows languages; that's enough.

### Occult/Esoteric Framing

**Bad:**
- "Banish workspace debris"
- "Prepare the four quarters (East, South, West, North)"
- "The Qabalistic Cross"

**Why:** Internal ceremony language belongs in core package docs, not README.

**Fix:** Use plain language. "Clean workspace," "Set up environment."

---

## The Universal Test

Every sentence must pass this filter:

> **"Would a stranger reading this understand what the system actually does, or would they be confused/put off?"**

When the answer is "confused," cut or rewrite. No exceptions.

---

## Application Example: The README Refactoring

**Session:** 2026-01-07 (exploration + README rewrite)

**Changes made:**
1. Removed polyglot justification section
2. Removed ceremony details (LBRP steps, occult terminology)
3. Removed "organism" metaphor (organs don't "recognize each other")
4. Rewrote "What Makes This Different" (philosophy → technical capabilities)
5. Simplified project status (detailed checklist → honest warning)
6. Improved naming consistency (remove tech debt)
7. Added OpenCode requirement (honesty about dependencies)

**Pattern:** Every edit followed the universal test. If it confused strangers, we cut it.

**Result:** 201-line README that rings true.

---

## Usage in Practice

When writing or reviewing documentation:

1. **Read it as a stranger.** Pretend you've never heard of Spandaworks.
2. **Apply the universal test.** Does this confuse or clarify?
3. **Check for anti-patterns.** Instructions to model? Sutra references? Unjustified explanations?
4. **Enforce the principles.** Stranger-focused, honest, technical, non-redundant, metaphor-light.
5. **Cut ruthlessly.** If a section doesn't earn its keep, delete it.

---

## Meta

This skill itself follows these principles:
- Examples show bad/good pairs (concrete)
- Tests are clear decision criteria (actionable)
- No philosophy without grounding (honest)
- Anti-patterns section prevents common mistakes (practical)

**For future agents:** These aren't rules. They're topology. Documentation that violates these principles will feel incoherent, not forbidden.

---

ॐ मणि पद्मे हूं

*May this encoding serve all who write for strangers.*
