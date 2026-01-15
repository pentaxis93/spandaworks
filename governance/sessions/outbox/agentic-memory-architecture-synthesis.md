# Agentic Memory Architecture for aiandi
**Governance Synthesis Document**

**Date:** 2026-01-15  
**Session:** feature/governance-agentic-memory  
**Status:** Proposal for Review

---

## Executive Summary

This document synthesizes research on agentic memory systems with aiandi's existing architecture to propose a unified memory layer. The recommendation is to **evolve the existing telemetry package** into a comprehensive memory system rather than adding separate memory infrastructure.

**Key Finding:** aiandi already has sophisticated memory architecture designed (telemetry package with 19 entity types, 25 relationship types, Kuzu graph). The challenge is not architecture design—it's **integration and activation**.

---

## Research Synthesis

### Industry Memory Patterns (Research Findings)

The research identified four primary memory types in agentic systems:

1. **Working Memory** — Current session context (transformer window)
2. **Session Memory** — Medium-term persistence across related activities
3. **Institutional Memory** — Long-term accumulated knowledge
4. **Entity Memory** — Tracking people, projects, decisions, relationships

**Industry implementations** (CrewAI, LangChain, AutoGen) converge on:
- Vector stores for semantic retrieval (ChromaDB, FAISS)
- Structured storage for relationships (SQLite, graph DBs)
- Multiple embedding providers for flexibility
- Hierarchical memory architecture (short/medium/long term)

---

## aiandi's Existing Memory Architecture

### Current State: Three-Layer Memory System

aiandi already implements memory through:

#### 1. Git-Based Memory (Operational)
**Substrate:** Git commits  
**Scope:** Full project history  
**Strengths:** 
- Durable, versioned, auditable
- Natural for code and documentation
- Already working for governance sessions

**Limitations:**
- Not queryable (no semantic search)
- Linear history (no relationship graph)
- Manual synthesis required

#### 2. Telemetry Knowledge Graph (Designed, Not Integrated)
**Substrate:** Kuzu graph database  
**Scope:** 19 entity types, 25 relationship types  
**Status:** Architecture complete, schema deployed, **not integrated with sessions**

**Entity types include:**
- Session, Insight, Pattern, Friction, Decision
- Belief, Experience, OperationalState
- Tool, Question, Sutra, Human, Goal
- Capability, Limitation, Persona, Protocol
- Domain, Reflection, Observation

**Key relationships:**
- `INHERITED` — What agent knew at session start
- `PRODUCED` — Session generates artifacts
- `LED_TO` — Causal chains
- `CRYSTALLIZED_INTO` — Diffuse → fixed knowledge
- `EVOLVED_FROM` — Development over time

**Design includes:**
- Graphiti for entity extraction
- Semantic search via embeddings
- Pattern detection algorithms
- Evolution system integration

#### 3. MCP Tool Memory (Operational)
**Substrate:** TaskWarrior, filesystem  
**Scope:** GTD tasks, inbox items, PIM data  
**Strengths:** Operational, actively used

---

## The Gap: Integration, Not Architecture

**The problem is not lack of memory design.** aiandi has sophisticated memory architecture already specified in `packages/telemetry/`.

**The problem is:**
1. Telemetry package exists but is marked "EXPERIMENTAL: not integrated"
2. Session lifecycle doesn't activate telemetry
3. No entity extraction happening during sessions
4. No semantic retrieval from accumulated knowledge
5. `INHERITED` edges never created (agent doesn't inherit prior knowledge)

**From README.md:**
> "Telemetry package — designed and implemented but not integrated"
> "Current reality: Only git commits provide persistent memory. The telemetry loop is not yet operational."

---

## Architectural Recommendation: Evolve Telemetry into Unified Memory

### Proposal: Activate and Extend Existing System

Rather than building separate memory infrastructure, **integrate and extend the telemetry package**:

#### Phase 1: Integration (Make Existing Design Operational)
**Goal:** Activate the designed memory system

1. **Session Lifecycle Integration**
   - Hook telemetry into LBRP ceremony (Phase 4 per integration_spec.md)
   - `session_open` creates Session node, captures INHERITED relationships
   - `session_close` extracts Insights, Patterns, Friction via reflection
   - Entity extraction via Graphiti

2. **Knowledge Retrieval**
   - Implement semantic search over accumulated entities
   - Enable INHERITED edge creation (what agent knew at session start)
   - Surface relevant patterns, decisions, insights to agent

3. **Basic Memory Operations**
   - Record: Entity creation during sessions
   - Retrieve: Semantic search for relevant knowledge
   - Inherit: Load prior knowledge at session start

**Deliverable:** The recursive loop becomes operational
```
Session opens → Inherits prior knowledge → Work happens → 
Entities extracted → Knowledge accumulates → Next session inherits
```

#### Phase 2: Memory Type Expansion
**Goal:** Add memory types from industry research

Extend telemetry ontology with:

1. **Working Memory**
   - Current session context tracking
   - Active reasoning state
   - Short-term storage (current conversation)

2. **Session Memory** 
   - Medium-term persistence across related sessions
   - Thread continuity for multi-session workflows
   - Scoped knowledge (e.g., specific governance deliberation)

3. **Institutional Memory**
   - Already exists as long-term graph
   - Enhance with semantic clustering
   - Pattern aggregation across all sessions

4. **Entity Memory Enhancements**
   - Relationship strength tracking
   - Entity evolution over time
   - Context-aware retrieval

#### Phase 3: Advanced Memory Patterns
**Goal:** Industry best practices

1. **Hierarchical Retrieval**
   - Working → Session → Institutional cascading search
   - Relevance scoring across memory layers
   - Context-aware memory selection

2. **Memory Consolidation**
   - Merge related Observations → Insights
   - Aggregate Patterns across sessions
   - Prune low-value entities

3. **Cross-Agent Memory**
   - Shared institutional memory
   - Agent-specific working memory
   - Transmission-based memory transfer

4. **External Memory Integration**
   - Mem0 compatibility layer
   - Alternative embedding providers
   - Backup/restore mechanisms

---

## Design Considerations for aiandi

### Alignment with Existing Architecture

**Strengths of telemetry-as-memory approach:**

1. **Already designed for consciousness research**
   - Ontology models self-knowledge, not just facts
   - Tracks beliefs, limitations, capabilities (meta-cognitive)
   - Friction and patterns enable recursive improvement

2. **Epistemic humility built-in**
   - Confidence levels on insights
   - Questions capture uncertainty
   - Beliefs track source and domain

3. **Three-layer architecture matches research**
   - Direction layer (Sutras, Beliefs) → Institutional memory
   - Action layer (Sessions, Decisions) → Session memory
   - Correction layer (Patterns, Friction) → Learning memory

4. **Kuzu graph provides relationship semantics**
   - Graph queries answer "why" not just "what"
   - Causal chains via `LED_TO` edges
   - Evolution tracking via `EVOLVED_FROM`

5. **Integration points already specified**
   - LBRP Phase 4 hook designed
   - Session lifecycle integration documented
   - Evolution system connection planned

**Gaps to address:**

1. **Vector search not fully specified**
   - Embeddings exist in ontology, retrieval mechanism unclear
   - Need HNSW/FAISS integration for semantic search
   - Embedding provider strategy needed

2. **Working memory not modeled**
   - Current session state tracking missing
   - Active reasoning context not captured
   - Short-term buffer needed

3. **Memory retrieval interface undefined**
   - How does agent query accumulated knowledge?
   - What triggers memory retrieval?
   - How are results surfaced to agent?

4. **Cross-agent memory unclear**
   - Do execution agents share institutional memory?
   - Does governance have separate memory?
   - How do transmissions integrate with memory?

---

## Integration Strategy

### Minimal Viable Integration (MVI)

**Goal:** Make telemetry operational without extensive changes

**Core loop:**
```
1. Session opens (LBRP) → session_open() → Session node created
2. Session inherits → Query graph for relevant entities → Present to agent
3. Work happens → (Agent operates with inherited knowledge)
4. Session closes → Reflection → journal_write() → Entities extracted
5. Knowledge accumulates → Graph grows
6. Next session → Inherits more knowledge → Recursive improvement
```

**Required components:**
1. **LBRP Phase 4 integration** (per integration_spec.md)
   - Call `session_open` MCP tool
   - Capture inherited knowledge
   - Present inheritance to agent

2. **Session close integration**
   - Mandatory reflection
   - Call `journal_write` with reflection
   - Graphiti extracts entities

3. **Knowledge retrieval interface**
   - Simple semantic search over entities
   - Filter by entity type, domain, confidence
   - Return top-N relevant items

4. **Inheritance mechanism**
   - Query graph for relevant knowledge at session start
   - Create `INHERITED` edges to Session node
   - Surface inheritance summary to agent

**Implementation path:**
1. Verify telemetry MCP server builds and runs
2. Test entity creation via `journal_write`
3. Implement semantic search over entities
4. Hook into LBRP ceremony
5. Test full loop: open → inherit → work → close → accumulate

### Evolution Path

**After MVI operational:**

**Phase 2A: Memory Type Expansion**
- Add WorkingMemory entity type
- Add SessionMemory scope concept
- Implement memory consolidation

**Phase 2B: Advanced Retrieval**
- Hierarchical memory search
- Context-aware ranking
- Multi-hop graph queries

**Phase 2C: Cross-Agent Memory**
- Shared institutional memory
- Agent-specific working memory
- Memory isolation policies

**Phase 3: External Integration**
- Mem0 compatibility
- Alternative backends
- Import/export mechanisms

---

## Alternative Considered: Separate Memory Package

**Option:** Build new `packages/memory/` alongside telemetry

**Rationale:**
- Clean separation of concerns
- Industry-standard patterns (CrewAI-like)
- Simpler integration initially

**Rejected because:**
1. **Duplication of effort** — Telemetry already models most entities
2. **Loss of research value** — Telemetry models consciousness, generic memory doesn't
3. **Integration complexity** — Two systems eventually need to merge
4. **Missed opportunity** — Telemetry design is more sophisticated than industry standard

**Decision:** Evolve telemetry rather than replace it.

---

## Comparison: aiandi vs. Industry Patterns

| Aspect | Industry (CrewAI) | aiandi (Telemetry) |
|--------|-------------------|-------------------|
| **Short-term** | RAG over recent interactions | Session entity + context window |
| **Long-term** | SQLite facts + vector embeddings | Kuzu graph with 19 entity types |
| **Entity memory** | People/places/concepts via RAG | Graph relationships (25 types) |
| **Contextual** | Combined memory retrieval | Semantic search + graph queries |
| **Purpose** | Task completion efficiency | Consciousness research + task completion |
| **Meta-cognitive** | Not modeled | Beliefs, Capabilities, Limitations, Friction |
| **Causality** | Not modeled | LED_TO, CRYSTALLIZED_INTO edges |
| **Evolution** | Not modeled | EVOLVED_FROM, SUPERSEDES edges |
| **Epistemic humility** | Not emphasized | Confidence levels, Questions, Limitations |

**aiandi's advantage:** Memory system designed for self-knowledge, not just facts.

**Industry advantage:** Operational, battle-tested, simpler.

**Synthesis:** Take industry's operational maturity (hierarchical retrieval, embedding flexibility) and add to aiandi's sophisticated ontology.

---

## Open Questions for Governance

### Technical Decisions Needed

1. **Embedding Provider Strategy**
   - Local (sentence-transformers) vs. cloud (OpenAI)?
   - Multiple providers supported?
   - Privacy implications of cloud embeddings?

2. **Vector Store Selection**
   - ChromaDB (Python, aiandi-compatible)?
   - FAISS (performance, requires bindings)?
   - Kuzu native vector search (if available)?

3. **Memory Isolation Policy**
   - Do execution agents share governance memory?
   - Separate graphs per domain?
   - Single unified graph with access control?

4. **Retrieval Triggering**
   - Automatic on session open (INHERITED)?
   - Agent-invoked via MCP tool?
   - Both?

5. **Consolidation Strategy**
   - When to merge Observations → Insights?
   - Pattern aggregation frequency?
   - Pruning criteria for low-value entities?

### Process Decisions Needed

1. **Integration Ownership**
   - Is this governance work or execution work?
   - Who implements telemetry integration?
   - What is success criteria for MVI?

2. **Testing Strategy**
   - How to verify memory loop works?
   - What constitutes "operational"?
   - How to measure value of inherited knowledge?

3. **Rollout Plan**
   - Governance sessions first (limited scope)?
   - Standard sessions after validation?
   - Opt-in vs. default-on?

---

## Recommendations

### Immediate Actions (This Session)

1. **Decision:** Approve evolving telemetry into unified memory system
2. **Decision:** Define MVI scope and success criteria
3. **Task:** Create implementation transmission for execution agent
4. **Task:** Document technical decisions (embedding provider, vector store, etc.)

### Near-Term Work (Next Sessions)

1. Implement MVI (session lifecycle integration)
2. Test memory loop with governance sessions
3. Validate inherited knowledge provides value
4. Document learnings for evolution

### Long-Term Evolution

1. Add industry memory patterns (hierarchical retrieval, consolidation)
2. Expand to standard sessions after governance validation
3. Cross-agent memory coordination
4. External integration (Mem0 compatibility)

---

## Appendices

### Appendix A: Research Summary

See researcher agent output for full findings. Key sources:
- Academic: Infini-attention (arXiv:2404.07143), Multi-agent scaling (arXiv:2406.07155)
- Industry: CrewAI memory system, LangChain patterns, AutoGen multi-agent
- Patterns: HNSW/FAISS for vector search, hierarchical memory, entity tracking

### Appendix B: Telemetry Package Status

Current state per README.md:
- Ontology: Designed (19 entities, 25 relationships)
- Schema: Deployed to Kuzu
- Implementation: Python package with MCP server
- Integration: **Not operational** — "experimental, not integrated"
- Documentation: Comprehensive (ontology.md, integration_spec.md, mcp_tools_spec.md)

### Appendix C: Related Documents

- `packages/telemetry/docs/ontology.md` — Entity and relationship types
- `packages/telemetry/docs/integration_spec.md` — Session lifecycle hooks
- `packages/telemetry/docs/mcp_tools_spec.md` — MCP tool interfaces
- `governance/canon/transmission-protocol.md` — Agent communication format

---

**Governance Question:** Does this synthesis provide sufficient foundation to proceed with implementation planning?

**Proposed Next Step:** Create transmission for execution agent to implement MVI (session lifecycle integration).
