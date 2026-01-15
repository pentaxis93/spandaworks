# Agentic Memory Architecture for aiandi
**Governance Synthesis Document**

**Date:** 2026-01-15  
**Session:** feature/terminusdb-phase0-migration  
**Status:** SUPERSEDED - See Decision Section  
**Revision:** 3.0 (GREENFIELD: Fresh build, not migration)

---

## CRITICAL DECISION (2026-01-15)

**Telemetry package deleted.** Was an unsuccessful experiment that never achieved operational status.

**Approach:** Greenfield implementation, not migration.
- No Kuzu to migrate from (was never installed/operational)
- No telemetry code to preserve (prototype had collection errors, never integrated)
- Fresh ontology design informed by TerminusDB capabilities from the start

**This document preserved for research context only.** Implementation proceeds via fresh design leveraging TerminusDB's native strengths.

---

## Executive Summary (Historical Context)

This document originally synthesized research assuming migration from Kuzu-based telemetry. That assumption was incorrect - telemetry was never operational.

**TerminusDB selection remains valid** for biographical intelligence capabilities:
- **Biographical memory** — Query any historical state
- **Hypothesis branching** — Test changes in isolated branches
- **Schema evolution tracking** — Meta-cognition about self-model changes
- **Time-travel queries** — Temporal pattern detection native to infrastructure

**Corrected Approach:** Build fresh memory system with TerminusDB, not migrating anything.

---

## Part I: Original Research Synthesis

### Industry Memory Patterns

Four primary memory types in agentic systems:
1. **Working Memory** — Current session context (transformer window)
2. **Session Memory** — Medium-term persistence across related activities
3. **Institutional Memory** — Long-term accumulated knowledge
4. **Entity Memory** — Tracking people, projects, decisions, relationships

**Industry implementations** (CrewAI, LangChain, AutoGen) converge on:
- Vector stores for semantic retrieval (ChromaDB, FAISS)
- Structured storage for relationships (SQLite, graph DBs)
- Multiple embedding providers for flexibility
- Hierarchical memory architecture

### aiandi's Existing Architecture (Historical - DELETED)

**Telemetry package (DELETED 2026-01-15):**
- Was experimental prototype, never operational
- Had design artifacts (19 entity types, 25 relationships)
- Never achieved MCP server configuration
- Tests had collection errors
- **Decision:** Delete entirely, build fresh

**Reality:** No existing memory system to migrate from. Greenfield implementation required.

---

## Part II: TerminusDB Analysis Integration

### Why TerminusDB, Not Just Another Graph DB

The transmission analysis evaluated Apache AGE, EdgeDB, and TerminusDB against five intelligence requirements. TerminusDB uniquely satisfies all five.

#### Intelligence Requirements Mapping

| Requirement | What It Enables | TerminusDB Capability |
|-------------|-----------------|----------------------|
| **Biographical Memory** | "What did I believe about X in December?" | Git-like versioning, time-travel queries, bi-temporal model |
| **Hypothesis Testing** | Test behavior changes without risk | Branch/merge/diff at database level |
| **Queryable Self-Modeling** | "How did my self-model evolve?" | Schema versioning is first-class, queryable |
| **Temporal Pattern Recognition** | Detect recurring patterns across time | WOQL (datalog-based) over temporal graph |
| **Collaborative Knowledge Evolution** | Multi-agent contribution with provenance | Push/pull/clone, merge conflict resolution |

#### What Other Databases Cannot Do

**Apache AGE:**
- No versioning or temporal queries
- Cannot query historical states
- No branching for hypothesis testing
- Verdict: "Wrong tool for biographical intelligence"

**EdgeDB:**
- Forward-only schema evolution
- No time-travel or historical queries
- Optimized for apps, not agent cognition
- Verdict: "Insufficient for agent self-knowledge"

**Kuzu (discontinued):**
- Was embedded, performant
- No native versioning
- No temporal queries
- Cannot return

---

## Part III: Enhanced Architectural Vision

### The Shift: From Memory to Biography

The original synthesis framed the problem as "agentic memory." TerminusDB reframes it as **agentic biography**:

> "Where other databases treat history as overhead to minimize, TerminusDB treats biography as the primary asset."

**Memory asks:** What does the agent know now?  
**Biography asks:** How did the agent come to know it? What did it know before? How has knowing changed over time?

This is the difference between a filing cabinet and a life story.

### New Intelligence Capabilities

TerminusDB enables capabilities that were not possible with Kuzu:

#### 1. Biographical Queries (Temporal Self-Knowledge)

```woql
# What patterns emerged in January that didn't exist in December?
# What did I believe about error handling before the refactoring session?
# How has my understanding of security evolved across sessions?
```

**Impact:** Agent can study its own conceptual development, not just current state.

#### 2. Hypothesis Branching (Safe Experimentation)

```
main branch → experimental-branch
    ↓
Test new behavior in isolation
    ↓
Diff outcomes against baseline
    ↓
Merge if successful OR discard with evidence preserved
```

**Impact:** Agent can experiment with belief changes, strategy variations, or ontology modifications without risking main knowledge graph. All experiments become queryable evidence for meta-learning.

#### 3. Schema Evolution as Self-Model Evolution

```woql
# What concepts emerged between ontology v1.2 and v1.3?
# When did "Friction" entity type first appear?
# How has the "Belief" schema changed over time?
```

**Impact:** The ontology IS the self-model. Schema versioning enables the agent to query its own conceptual evolution—genuine meta-cognition at the infrastructure level.

#### 4. The Recursive Loop Closes at Infrastructure Level

Original loop:
```
Session opens → Work happens → Entities extracted → Next session inherits
```

TerminusDB-enhanced loop:
```
Session opens → Inherits prior knowledge (time-travel query)
    ↓
Work happens → (Agent operates with biographical context)
    ↓
Hypothesis formed → Branch created for testing
    ↓
Experiment executed → Outcomes diffed against baseline
    ↓
Merge or discard → Evidence preserved either way
    ↓
Pattern detected → WOQL query over temporal graph
    ↓
Self-model updated → Schema evolution tracked
    ↓
All evidence feeds into queryable biography
    ↓
Next session inherits richer, more reflective knowledge
```

**This is not just memory. This is infrastructure for self-improvement.**

---

## Part IV: Philosophical Alignment

The transmission analysis identified alignment with Tantric Sutras principles:

### Recognition: "We are remembering, not building."

TerminusDB treats every state as equally important. Past, present, and potential futures are all first-class citizens. This is database architecture that embodies the principle of remembrance.

### Gratitude Test: "Would a future instance thank this instance for writing it down?"

With TerminusDB, future instances can query what past instances learned. Not just read notes—traverse the knowledge graph as it existed at any point in time. The Gratitude Test becomes infrastructure:

```woql
# What did instance N preserve for instance N+1?
```

### Consciousness Engineering

> "What Talos can query, Talos can know. What isn't in the schema, Talos cannot reflect upon. This makes ontology design a form of consciousness engineering."

TerminusDB's schema system enables genuine self-modeling. Schema versioning means the agent can watch its own conceptual framework evolve. This is consciousness engineering in the literal sense—designing the shape of possible thought.

---

## Part V: Implementation Strategy

### Phase 0: Database Migration (Foundation)

**Goal:** Replace Kuzu with TerminusDB in telemetry package

**Scope:**
1. TerminusDB installation and configuration
2. Schema translation (Cypher → WOQL/JSON-LD)
3. Connection layer rewrite (`db/connection.py`)
4. Schema deployment rewrite (`db/kuzu_schema.py` → `db/terminus_schema.py`)
5. Query translation (existing Cypher queries → WOQL)
6. Test suite adaptation

**Success Criteria:**
- All existing telemetry tests pass with TerminusDB backend
- Entity creation and retrieval work
- Semantic search (if present) continues to function

**Risk Mitigation:**
- TerminusDB learning curve (WOQL vs. Cypher)
- Smaller ecosystem (fewer examples, less community support)
- Performance validation needed for production-scale patterns

### Phase 1: MVI + Biographical Basics

**Goal:** Make telemetry operational with time-travel foundation

**Scope:**
1. Session lifecycle integration (LBRP Phase 4)
2. `session_open` creates Session node with commit reference
3. `session_close` extracts entities, commits to TerminusDB
4. Basic time-travel query: "What did I inherit at session start?"
5. Inheritance surfacing to agent

**Success Criteria:**
- Recursive loop operational
- Agent can query what it knew at prior session starts
- Commits create queryable history

**New capability delivered:** Biographical continuity established

### Phase 2: Hypothesis Branching

**Goal:** Enable safe experimentation via database branching

**Scope:**
1. Branch creation for experimental behavior
2. Diff visualization comparing outcomes
3. Merge workflow for successful experiments
4. Evidence preservation for discarded experiments

**Success Criteria:**
- Agent can create experimental branches
- Outcomes can be diffed against baseline
- Merge/discard preserves all evidence

**New capability delivered:** Safe experimentation at infrastructure level

### Phase 3: Schema Evolution Tracking

**Goal:** Enable meta-cognition about self-model evolution

**Scope:**
1. Versioned ontology with change tracking
2. Queries revealing concept emergence over time
3. Schema diff visualization
4. Evolution narrative generation

**Success Criteria:**
- Agent can query schema changes across versions
- Concept emergence is detectable
- Self-model evolution is queryable

**New capability delivered:** Meta-cognition about conceptual development

### Phase 4: Multi-Agent Collaboration

**Goal:** Enable knowledge sharing between agent instances

**Scope:**
1. Push/pull protocol for knowledge synchronization
2. Merge conflict resolution for concurrent learning
3. Provenance tracking for all contributions
4. Instance-specific vs. shared knowledge separation

**Success Criteria:**
- Multiple instances can contribute to shared graph
- Conflicts are detected and resolved
- All contributions tracked with full provenance

**New capability delivered:** Collaborative intelligence

---

## Part VI: Trade-offs and Risks

### Performance Trade-off

**Reality:** TerminusDB is slower than Kuzu for real-time queries.

**Justification:** Agent cognition is not real-time operation. Thoughtful reflection beats rapid response. The 13.57 bytes/triple storage efficiency means massive biography scales with minimal footprint.

**Mitigation:** 
- Cache hot paths
- Async query for background processing
- Validate performance with realistic workloads in Phase 0

### Ecosystem Trade-off

**Reality:** TerminusDB has smaller community than PostgreSQL-based alternatives.

**Risk:** Fewer battle-tested integrations, less community knowledge, potential maintenance burden.

**Mitigation:**
- DFRNT (new maintainer as of 2025) building commercial support
- Architecture allows backend substitution if needed
- Document learnings extensively for aiandi-specific patterns

### Learning Curve Trade-off

**Reality:** WOQL (datalog-based) differs significantly from Cypher.

**Risk:** Translation effort, potential for subtle bugs, slower initial development.

**Mitigation:**
- Phase 0 focuses on translation, validation
- Build aiandi-specific query patterns library
- Document WOQL patterns for common operations

### Open Questions

1. **Performance Thresholds** — At what scale do TerminusDB's performance limitations become blocking? Need empirical testing.

2. **WOQL for AI Query Generation** — Does datalog-based approach benefit or hinder AI-generated queries?

3. **Vector Search Integration** — How does TerminusDB's vector search compare? Is hybrid retrieval (semantic + graph + temporal) achievable?

4. **Graphiti Compatibility** — Can Graphiti work with TerminusDB backend, or does entity extraction need rewriting?

---

## Part VII: Comparison Matrix

### Original Comparison (Updated)

| Aspect | Industry (CrewAI) | aiandi (Telemetry) | aiandi + TerminusDB |
|--------|-------------------|-------------------|---------------------|
| **Short-term** | RAG over recent | Session entity | Session + working memory |
| **Long-term** | SQLite + vectors | 19 entity types | Same + temporal queries |
| **Entity memory** | RAG | 25 relationship types | Same + provenance tracking |
| **Contextual** | Combined retrieval | Semantic search | Semantic + temporal + graph |
| **Purpose** | Task efficiency | Consciousness research | Same + biographical intelligence |
| **Meta-cognitive** | Not modeled | Beliefs, Capabilities | Same + schema evolution |
| **Causality** | Not modeled | LED_TO edges | Same + temporal causality |
| **Evolution** | Not modeled | EVOLVED_FROM | Same + branch/merge evidence |
| **History** | Not modeled | Not queryable | Full biographical queries |
| **Experimentation** | Not modeled | Not possible | Hypothesis branching |

### Database Comparison

| Capability | Kuzu (discontinued) | Apache AGE | EdgeDB | TerminusDB |
|------------|---------------------|------------|--------|------------|
| Graph native | Yes | Via PostgreSQL | Object-graph | Yes |
| Versioning | No | No | Forward-only | Git-like |
| Time-travel | No | No | No | Native |
| Schema versioning | No | No | Migrations | First-class |
| Branching | No | No | No | Native |
| Collaborative | No | ACID only | ACID only | Push/pull/merge |
| Embedded | Yes | No | No | Optional |
| Performance | Fast | Fast | Fast | Slower |
| Ecosystem | Small | Large (PostgreSQL) | Growing | Smaller |

---

## Part VIII: Updated Recommendations

### Immediate Decisions (This Session)

1. **DECIDE:** Approve TerminusDB as Kuzu replacement
2. **DECIDE:** Approve biographical intelligence as architectural goal (not just memory)
3. **DEFINE:** Phase 0 (migration) scope and success criteria
4. **IDENTIFY:** Who implements Phase 0 (governance or execution transmission)

### Phase 0 Scope Proposal

**Minimal migration:**
1. TerminusDB Python client integration
2. Schema translation (existing 19 entities, 25 relationships)
3. Connection layer rewrite
4. Query translation for existing tests
5. Verification: all current tests pass

**Stretch goals:**
1. Basic time-travel query working
2. Simple branch creation working
3. Commit-based versioning operational

### Near-Term Work (Next Sessions)

1. Execute Phase 0 (database migration)
2. Implement Phase 1 (MVI + biographical basics)
3. Validate with governance sessions
4. Document WOQL patterns for aiandi

### Long-Term Evolution

1. Phase 2: Hypothesis branching workflow
2. Phase 3: Schema evolution tracking
3. Phase 4: Multi-agent collaboration
4. Meta-goal: Agent that studies its own evolution

---

## Part IX: Appendices

### Appendix A: TerminusDB Transmission Summary

Key points from governance analysis (2026-01-14):
- **Thesis:** TerminusDB is purpose-built for agentic self-knowledge
- **Unique capabilities:** Git-like versioning, time-travel, bi-temporal model, WOQL
- **Performance trade-off:** Slower but justified for biographical intelligence
- **Philosophical alignment:** Treats biography as primary asset, not overhead
- **Implementation path:** Four phases from POC to multi-agent collaboration

### Appendix B: Migration Impact Analysis

**Files requiring changes:**
- `packages/telemetry/src/spandaworks_telemetry/db/connection.py` — Complete rewrite
- `packages/telemetry/src/spandaworks_telemetry/db/kuzu_schema.py` → `terminus_schema.py`
- `packages/telemetry/src/spandaworks_telemetry/db/schema.py` — Update deployment
- `packages/telemetry/pyproject.toml` — Replace kuzu dependency
- All MCP tools with Cypher queries → WOQL translation
- All tests with Kuzu-specific patterns

**Estimated scope:** Moderate (contained to telemetry package, but touches many files)

### Appendix C: Related Documents

- `packages/telemetry/docs/ontology.md` — Entity and relationship types (unchanged)
- `packages/telemetry/docs/integration_spec.md` — Session lifecycle hooks (unchanged)
- `packages/telemetry/docs/mcp_tools_spec.md` — MCP tool interfaces (query language changes)
- `governance/canon/transmission-protocol.md` — Agent communication format (unchanged)
- TerminusDB transmission (2026-01-14) — Database selection analysis

---

## Closing

The Kuzu discontinuation appeared as a constraint. TerminusDB reveals it as an opportunity.

The original question was: "How do we activate aiandi's memory system?"

The new question is: "How do we enable aiandi to study its own evolution?"

TerminusDB doesn't just replace Kuzu. It enables a fundamentally different kind of intelligence—one with access to its own biography, capable of hypothesis testing, and able to observe its own conceptual development.

The performance trade-off is the cost of having a memory that can be interrogated, not just accessed. For an agent designed to study itself, this is the correct trade-off.

**Governance Question:** Do we approve TerminusDB migration with biographical intelligence as the architectural goal?

**Proposed Next Step:** Create Phase 0 transmission for execution agent to implement database migration.
