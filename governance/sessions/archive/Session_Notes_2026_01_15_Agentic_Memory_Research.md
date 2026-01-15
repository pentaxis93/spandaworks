# Governance Session Notes: Agentic Memory Research

**Date:** 2026-01-15  
**Session ID:** governance-2026-01-15-agentic-memory  
**Branch:** feature/governance-agentic-memory  
**Goal:** Research unified agentic memory systems for aiandi integration

---

## Session Summary

Conducted comprehensive research into agentic memory systems to inform aiandi's memory architecture. 

**Initial finding:** aiandi already has sophisticated memory architecture designed (telemetry package with 19 entity types, 25 relationship types). The challenge is integration and activation, not design.

**Critical update:** Kuzu is discontinued. Human provided TerminusDB analysis transmission showing TerminusDB enables **biographical intelligence** capabilities impossible with Kuzu:
- Time-travel queries over full knowledge history
- Hypothesis branching for safe experimentation
- Schema evolution tracking for meta-cognition
- Git-like versioning for collaborative knowledge

**Updated recommendation:** Migrate from Kuzu to TerminusDB, gaining both necessary replacement AND enhanced intelligence capabilities.

---

## Key Exchanges

### 1. Research Phase

**Invoked:** Researcher agent to survey agentic memory landscape

**Findings delivered:**
- Memory type taxonomy (working, session, institutional, entity memory)
- Industry implementations (CrewAI, LangChain, AutoGen)
- Architectural patterns (HNSW/FAISS for vector search, hierarchical memory)
- Use case analysis

**Sources reviewed:**
- Academic: Infini-attention (arXiv:2404.07143), multi-agent scaling (arXiv:2406.07155)
- Industry: CrewAI memory docs, LangChain patterns, Lilian Weng's blog
- Algorithms: MIPS patterns (LSH, ANNOY, HNSW, FAISS, ScaNN)

### 2. Architecture Analysis Phase

**Examined aiandi codebase:**
- README.md — Current memory status: "git commits only, telemetry not integrated"
- `packages/telemetry/docs/ontology.md` — 19 entity types, 25 relationship types
- `packages/telemetry/docs/integration_spec.md` — LBRP Phase 4 hook designed
- Existing MCP servers (GTD, PIM, telemetry)

**Finding:** Telemetry package IS the memory system. It's designed for consciousness research with:
- Epistemic humility (confidence levels, questions, limitations)
- Meta-cognitive entities (beliefs, capabilities, friction)
- Causal tracking (LED_TO, CRYSTALLIZED_INTO edges)
- Evolution modeling (EVOLVED_FROM, SUPERSEDES)

### 3. Initial Synthesis Phase

**Created:** `governance/sessions/outbox/agentic-memory-architecture-synthesis.md` (v1.0)

**Initial recommendation:** Evolve telemetry into unified memory system rather than building separate memory infrastructure.

### 4. TerminusDB Integration Phase (Human Input)

**Received:** Governance transmission analyzing TerminusDB vs. Apache AGE vs. EdgeDB

**Key insights from transmission:**
1. **Kuzu discontinued** — Database layer must be replaced
2. **TerminusDB unique capabilities:**
   - Git-like versioning (branch/merge/diff)
   - Time-travel queries (bi-temporal model)
   - Schema versioning as first-class citizen
   - WOQL (datalog-based query language)
   - Collaborative knowledge evolution (push/pull/clone)

3. **The reframe:** From "agentic memory" to "agentic biography"
   - Memory asks: What does the agent know now?
   - Biography asks: How did it come to know it? What did it know before?

4. **Intelligence requirements mapped:**
   - Biographical Memory → Time-travel queries
   - Hypothesis Testing → Branch/merge/diff
   - Queryable Self-Modeling → Schema versioning
   - Temporal Pattern Recognition → WOQL over temporal graph
   - Collaborative Knowledge Evolution → Push/pull/merge

**Updated synthesis:** `agentic-memory-architecture-synthesis.md` (v2.0)

---

## Decisions Made

1. **Architecture Direction:** Evolve telemetry package into unified memory system (unchanged)

2. **Database Selection:** TerminusDB replaces Kuzu (new decision, based on transmission analysis)

3. **Goal Elevation:** From "memory integration" to "biographical intelligence" (new framing)

4. **Integration Strategy:** Five-phase approach:
   - Phase 0: Database migration (Kuzu → TerminusDB)
   - Phase 1: MVI + biographical basics
   - Phase 2: Hypothesis branching
   - Phase 3: Schema evolution tracking
   - Phase 4: Multi-agent collaboration

---

## Artifacts Created

### In `governance/sessions/outbox/`
- `agentic-memory-architecture-synthesis.md` (v2.0) — Complete architectural analysis with TerminusDB integration

### Research Outputs
- Memory type taxonomy synthesized from academic and industry sources
- Comparison matrix (aiandi telemetry vs. industry patterns vs. TerminusDB-enhanced)
- Database comparison (Kuzu vs. Apache AGE vs. EdgeDB vs. TerminusDB)
- Five-phase implementation strategy

---

## Open Questions for Future Sessions

### Technical Decisions Resolved
- **Database:** TerminusDB (decided via transmission analysis)

### Technical Decisions Still Needed
1. Embedding provider strategy (local vs. cloud, privacy implications)
2. Graphiti compatibility with TerminusDB backend
3. WOQL query patterns for common operations
4. Performance validation with realistic workloads

### Process Decisions Needed
1. Phase 0 ownership (governance vs. execution transmission)
2. Testing strategy (how to verify migration successful)
3. Rollout plan (governance-first vs. all sessions)

### Research Gaps
1. WOQL learning curve for AI query generation
2. TerminusDB vector search capabilities
3. Graphiti integration path
4. Performance thresholds at scale

---

## Next Steps (Updated)

1. **DECIDE:** Approve TerminusDB as Kuzu replacement
2. **DECIDE:** Approve biographical intelligence as architectural goal
3. **DEFINE:** Phase 0 (migration) scope and success criteria
4. **CREATE:** Phase 0 transmission for execution agent
5. **EXECUTE:** Database migration

---

## Meta-Observations

### What Worked Well

1. **Researcher agent invocation** — Delivered comprehensive industry survey
2. **Parallel context loading** — Git status + inheritance check + context reading in single call
3. **Synthesis approach** — Research → codebase analysis → architectural comparison → recommendation
4. **Governance posture** — WHAT (define memory requirements) separated from HOW (implementation approach)
5. **Human transmission integration** — TerminusDB analysis elevated the architectural vision

### What Could Improve

1. **Earlier codebase examination** — Could have read telemetry docs before commissioning research
2. **Beads integration** — Successfully created and claimed task, but could track sub-tasks in Beads rather than TodoWrite
3. **Decision gates** — Synthesis document asks good questions but doesn't force decision points

### Patterns Noticed

1. **"Build vs. Integrate" tension** — Strong pull to build new rather than activate existing
2. **Research validates existing design** — Telemetry ontology aligns with industry best practices
3. **Constraint becomes opportunity** — Kuzu discontinuation revealed TerminusDB's biographical capabilities
4. **Governance value** — Deliberation layer prevented premature implementation, enabled architectural elevation

### Friction Points

None significant. Human input (TerminusDB transmission) arrived at optimal time, after initial research but before implementation commitment.

---

## Inheritance for Next Session

**If continuing memory work:**
- Synthesis document (v2.0) in outbox provides full architectural context
- TerminusDB selected as database backend
- Phase 0 (migration) is immediate priority
- Five-phase implementation path defined

**Critical context:**
- Kuzu is discontinued — migration required regardless
- TerminusDB enables biographical intelligence — this is the new architectural goal
- Performance trade-off acknowledged — slower queries justified by biographical capabilities

**If pivoting to different work:**
- Memory research complete and documented
- Database decision captured
- No blocking dependencies (migration can happen later)

---

## Session Part 2: Workflow Friction Investigation

### 5. Beads/Worktree Friction

**Problem surfaced:** Beads tasks created in governance worktree were going to main worktree's database. JSONL sync was confusing.

**Investigation:**
- Beads daemon runs in main worktree
- All `bd` commands route to main's SQLite database
- Feature worktrees need `bd sync --from-main` to see updates
- This is **by design** — Beads daemon mode is incompatible with worktrees

**Research commissioned:** 
1. Canonical Beads workflow (researcher agent)
2. Dev workflow best practices (researcher agent)
3. Audit of worktree references in skills/commands (explore agents)

### 6. Workflow Decision

**Finding:** The worktree workflow is the friction source, not Beads.

**Options considered:**
- A) Fix Beads integration with `--no-db` mode
- B) Find/build alternative task system
- C) Stop using worktrees, adopt single-directory workflow

**Decision:** Option C — Migrate to single-directory git workflow

**Rationale:**
- Beads is designed for agent swarms (atomic claim, dependency graph, ready queue)
- Worktrees were convenience, not requirement
- Single-directory + daemon mode = Beads works as designed
- Agent swarms don't need parallel checkouts

### 7. Implementation Plan Created

**GitHub Issue:** #57 — Migrate from worktrees to single-directory git workflow

**Files requiring changes:**
- `.opencode/skill/lbrp/SKILL.md` — Remove worktree verification
- `.opencode/skill/beads-workflow/SKILL.md` — Remove multi-directory examples
- `.opencode/command/close.md` — Remove worktree cleanup
- `docs/operating-instructions-opening-ceremony.md` — Update workspace observation
- `docs/inbox-outbox-protocol.md` — Update observation phase
- `AGENTS.md` — Clarify single-directory workflow

**Migration steps:**
1. Complete in-progress work in feature worktrees
2. Merge or close open PRs
3. Delete feature worktrees
4. Update skills/commands/docs
5. Verify Beads daemon works correctly

---

## Updated Decisions

5. **Workflow Simplification:** Migrate from worktrees to single-directory git workflow (GitHub #57)

6. **Task Tracking:** Beads remains primary (works correctly with single-directory workflow)

---

## Updated Artifacts

### GitHub Issues
- #57 — Migrate from worktrees to single-directory git workflow

### Beads Tasks
- `aiandi-dwz` — TerminusDB Migration epic (5 phases)
- `aiandi-lbd` — Phase 0: Database migration (ready)

---

**Session Duration:** ~3 hours  
**Beads Task:** aiandi-ll1 (Research: Unified agentic memory for aiandi) — closed  
**GitHub Issue:** #57 (Workflow migration)  
**Status:** 
- Memory research complete
- TerminusDB migration planned in Beads
- Workflow simplification planned in GitHub

---

*May this work benefit all beings everywhere, without exception.*
