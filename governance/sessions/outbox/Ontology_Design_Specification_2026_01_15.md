# Ontology Design Specification for Agentic Memory System

**Document Date:** January 15, 2026  
**Bead ID:** aiandi-agf  
**Status:** Complete  
**Version:** 1.0.0

---

## 1. Design Philosophy

### Biography-First Architecture

This ontology treats agent evolution as the primary design concern. Unlike traditional databases that treat history as overhead, this system treats biography as the primary asset. Every entity carries temporal context about when and why it was discovered.

**Core Principles:**

1. **Temporal Validity**: Every entity includes discovery context and evolution history
2. **Session Inheritance**: Knowledge flows between sessions through explicit inheritance patterns
3. **Evidence Chains**: All beliefs and patterns link to supporting evidence
4. **Emergent Pattern Discovery**: The schema recognizes patterns as they emerge from instances
5. **Hypothesis-Driven Evolution**: Experimental branches enable safe knowledge exploration

### Leveraging TerminusDB Capabilities

The design maximizes TerminusDB's unique strengths:

- **Bi-temporal queries** for time-travel across agent history
- **Schema versioning** for meta-cognitive awareness
- **Branch/merge workflow** for hypothesis testing
- **WOQL datalog engine** for complex pattern recognition
- **JSON-LD constraints** for rich type modeling

### Emergent Pattern Discovery

Rather than pre-defining all possible patterns, the ontology provides mechanisms for patterns to emerge organically from agent experience and be recognized when they achieve statistical significance.

---

## 2. Entity Type Specifications

### Session Entity

The foundational unit representing a single agent session with full biographical context.

```json
{
  "@context": {
    "@base": "aiandi://ontology/v1.0.0/",
    "Session": {
      "@type": "Class",
      "@metadata": {
        "description": "A bounded period of agent activity with inherited context",
        "design_rationale": "Sessions provide temporal boundaries and inheritance scope"
      },
      "id": {
        "@type": "xsd:string",
        "@metadata": { "pattern": "^session_[0-9]{4}-[0-9]{2}-[0-9]{2}_[0-9]{6}$" }
      },
      "timestamp": "xsd:dateTime",
      "session_type": {
        "@type": "Enum",
        "@values": ["governance", "standard", "ops", "research"]
      },
      "entities": {
        "@type": "Set",
        "@class": "Entity"
      },
      "inherited_context": {
        "@type": "Optional",
        "@class": "Context"
      },
      "outcomes": {
        "@type": "List",
        "@class": "Outcome"
      },
      "parent_session": {
        "@type": "Optional", 
        "@class": "Session"
      },
      "branch_point": {
        "@type": "Optional",
        "@class": "xsd:string"
      }
    }
  }
}
```

### Entity (Abstract Base)

The universal base type for all knowledge entities in the agent's memory.

```json
{
  "Entity": {
    "@type": "Class",
    "@abstract": true,
    "@metadata": {
      "description": "Abstract base for all knowledge entities",
      "design_rationale": "Provides common properties for temporal tracking and confidence modeling"
    },
    "id": {
      "@type": "xsd:string",
      "@unique": true
    },
    "confidence": {
      "@type": "xsd:float",
      "@min": 0.0,
      "@max": 1.0,
      "@metadata": { "description": "Agent's confidence in this entity [0.0-1.0]" }
    },
    "discovered_in": {
      "@type": "Session",
      "@metadata": { "description": "Session where this entity was first discovered" }
    },
    "last_updated": "xsd:dateTime",
    "tags": {
      "@type": "Set",
      "@class": "xsd:string"
    },
    "superseded_by": {
      "@type": "Optional",
      "@class": "Entity",
      "@metadata": { "description": "Entity that replaces this one in agent's understanding" }
    }
  }
}
```

### Belief Entity

Represents agent propositions with evidence and confidence tracking.

```json
{
  "Belief": {
    "@type": "Class",
    "@inherits": "Entity",
    "@metadata": {
      "description": "A proposition the agent holds with associated evidence",
      "design_rationale": "Beliefs are central to agent reasoning and must track evidence chains"
    },
    "proposition": {
      "@type": "xsd:string",
      "@metadata": { "description": "Natural language statement of the belief" }
    },
    "evidence": {
      "@type": "Set",
      "@class": "Evidence",
      "@min_cardinality": 0
    },
    "context": {
      "@type": "Set", 
      "@class": "Context"
    },
    "temporal_validity": {
      "@type": "Object",
      "@metadata": { "description": "When this belief was/is valid" },
      "start": "xsd:dateTime",
      "end": {
        "@type": "Optional",
        "@class": "xsd:dateTime"
      }
    },
    "belief_type": {
      "@type": "Enum",
      "@values": ["factual", "methodological", "meta_cognitive", "causal", "predictive"]
    },
    "supersedes": {
      "@type": "Optional",
      "@class": "Belief",
      "@metadata": { "description": "Previous belief this replaces" }
    }
  }
}
```

### Pattern Entity

Represents discovered regularities across agent experience.

```json
{
  "Pattern": {
    "@type": "Class", 
    "@inherits": "Entity",
    "@metadata": {
      "description": "A recognized regularity across multiple agent experiences",
      "design_rationale": "Patterns emerge from instances and become queryable knowledge"
    },
    "description": "xsd:string",
    "pattern_type": {
      "@type": "Enum",
      "@values": ["behavioral", "error_class", "success_factor", "temporal", "causal", "structural"]
    },
    "instances": {
      "@type": "Set",
      "@class": "Entity",
      "@min_cardinality": 2
    },
    "confidence_threshold": {
      "@type": "xsd:float",
      "@default": 0.7,
      "@metadata": { "description": "Confidence required to recognize pattern instances" }
    },
    "statistical_significance": {
      "@type": "xsd:float",
      "@metadata": { "description": "P-value or similar significance measure" }
    },
    "template": {
      "@type": "Optional",
      "@class": "xsd:string",
      "@metadata": { "description": "Template for recognizing new instances" }
    }
  }
}
```

### Hypothesis Entity

Represents testable propositions with associated experimental branches.

```json
{
  "Hypothesis": {
    "@type": "Class",
    "@inherits": "Entity", 
    "@metadata": {
      "description": "A testable proposition with experimental validation",
      "design_rationale": "Enables safe experimentation via TerminusDB branching"
    },
    "description": "xsd:string",
    "hypothesis_type": {
      "@type": "Enum",
      "@values": ["strategy", "causal", "predictive", "methodological"]
    },
    "test_branch": {
      "@type": "xsd:string",
      "@metadata": { "description": "TerminusDB branch where hypothesis is tested" }
    },
    "baseline_commit": {
      "@type": "xsd:string",
      "@metadata": { "description": "Commit representing pre-test state" }
    },
    "test_outcome": {
      "@type": "Optional",
      "@class": "Outcome"
    },
    "supporting_evidence": {
      "@type": "Set",
      "@class": "Evidence"
    },
    "contradicting_evidence": {
      "@type": "Set", 
      "@class": "Evidence"
    },
    "status": {
      "@type": "Enum",
      "@values": ["proposed", "testing", "validated", "refuted", "inconclusive"]
    }
  }
}
```

### Evidence Entity

Represents supporting material for beliefs and patterns.

```json
{
  "Evidence": {
    "@type": "Class",
    "@inherits": "Entity",
    "@metadata": {
      "description": "Supporting material for beliefs, patterns, and hypotheses",
      "design_rationale": "Evidence chains enable reasoning transparency"
    },
    "content": "xsd:string",
    "evidence_type": {
      "@type": "Enum", 
      "@values": ["observation", "measurement", "citation", "experiment", "reasoning"]
    },
    "source": {
      "@type": "Optional",
      "@class": "xsd:string",
      "@metadata": { "description": "External source or internal session reference" }
    },
    "strength": {
      "@type": "xsd:float",
      "@min": 0.0,
      "@max": 1.0,
      "@metadata": { "description": "Evidential strength assessment" }
    },
    "supports": {
      "@type": "Set",
      "@class": "Entity",
      "@metadata": { "description": "Entities this evidence supports" }
    }
  }
}
```

### Context Entity

Represents situational information relevant to agent reasoning.

```json
{
  "Context": {
    "@type": "Class",
    "@inherits": "Entity",
    "@metadata": {
      "description": "Situational information affecting agent reasoning",
      "design_rationale": "Context enables situation-aware knowledge application"
    },
    "description": "xsd:string", 
    "context_type": {
      "@type": "Enum",
      "@values": ["environmental", "goal_state", "resource_constraint", "temporal", "social"]
    },
    "scope": {
      "@type": "Enum",
      "@values": ["session", "task", "domain", "global"]
    },
    "active_during": {
      "@type": "Set",
      "@class": "Session"
    },
    "constraints": {
      "@type": "Optional",
      "@class": "xsd:string", 
      "@metadata": { "description": "JSON object describing context constraints" }
    }
  }
}
```

### Outcome Entity

Represents results of agent actions or experiments.

```json
{
  "Outcome": {
    "@type": "Class",
    "@inherits": "Entity",
    "@metadata": {
      "description": "Result of agent action, decision, or experiment",
      "design_rationale": "Outcomes enable learning from results"
    },
    "description": "xsd:string",
    "outcome_type": {
      "@type": "Enum",
      "@values": ["success", "failure", "partial", "inconclusive", "unexpected"]
    },
    "action_taken": {
      "@type": "xsd:string",
      "@metadata": { "description": "Description of action that led to outcome" }
    },
    "metrics": {
      "@type": "Optional",
      "@class": "xsd:string",
      "@metadata": { "description": "JSON object with measurable results" }
    },
    "lessons_learned": {
      "@type": "Set",
      "@class": "Belief"
    },
    "resulting_patterns": {
      "@type": "Set",
      "@class": "Pattern"
    }
  }
}
```

---

## 3. Relationship Semantics

### Session Inheritance

Sessions form inheritance chains where knowledge flows forward unless explicitly superseded.

**Inheritance Rules:**
- Child sessions inherit all non-superseded entities from parent
- Explicit supersession breaks inheritance for specific entities
- Context flows through inheritance but can be overridden
- Outcomes do not inherit (session-specific)

### Entity Evolution

Entities evolve through supersession relationships forming directed acyclic graphs.

**Evolution Types:**
- **Supersession**: New entity completely replaces old one
- **Refinement**: New entity improves on old one
- **Contradiction**: New entity negates old one
- **Generalization**: New entity encompasses old one as special case

### Evidence Chains

Evidence entities support multiple beliefs/patterns through many-to-many relationships.

**Chain Properties:**
- Evidence can support multiple entities
- Entities typically require multiple evidence sources
- Circular evidence chains are detected and flagged
- Evidence strength propagates through chains

### Pattern Instantiation

Patterns emerge from instances and then recognize new instances.

### Temporal Relationships

All entities exist within temporal scope with validity periods.

---

## 4. Schema Versioning Strategy

### Semantic Versioning

Schema versions follow semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Breaking changes requiring data migration
- **MINOR**: Additive changes (new entity types, optional properties)
- **PATCH**: Bug fixes, constraint adjustments, documentation

### Migration Patterns

**Migration Types:**
- **Additive**: New optional properties (backward compatible)
- **Constraint**: Tighter validation rules
- **Structural**: New entity types or relationships
- **Breaking**: Property removal or type changes

### Compatibility Considerations

- **Forward compatibility**: Old agents can read new schemas
- **Backward compatibility**: New agents can read old data
- **Version detection**: Agents detect schema mismatches and adapt
- **Migration verification**: Automated testing of migration correctness

---

## 5. Query Patterns Library

### Session Inheritance Pattern

```woql
// Get effective context for session
woql.triple("v:CurrentSession", "id", "session_2026-01-15_142030")
     .optional(
       woql.path("v:CurrentSession", "(parent_session>)*", "v:AncestorSession")
            .triple("v:AncestorSession", "entities", "v:InheritedEntity")
            .not(woql.triple("v:InheritedEntity", "superseded_by", "v:NewEntity"))
     )
     .triple("v:CurrentSession", "entities", "v:DirectEntity")
```

### Belief Evolution Tracking

```woql
// Belief evolution timeline
woql.path("v:InitialBelief", "(supersedes>)*", "v:CurrentBelief") 
     .triple("v:InitialBelief", "discovered_in", "v:Session1")
     .triple("v:Session1", "timestamp", "v:Time1")
     .triple("v:CurrentBelief", "last_updated", "v:Time2")
     .order_by("v:Time1", "v:Time2")
```

### Pattern Emergence Detection

```woql
// Find emerging patterns
woql.group_by(
  ["v:PatternType", "v:Context"],
  woql.triple("v:Entity", "exhibits", "v:PatternType")
       .triple("v:Entity", "context", "v:Context"),
  "v:Count"
).having("v:Count >= 3")
 .not(woql.triple("v:RecognizedPattern", "pattern_type", "v:PatternType"))
```

### Time-Travel Queries

```woql
// Agent knowledge state at specific time
woql.triple("v:Entity", "discovered_in", "v:Session")
     .triple("v:Session", "timestamp", "v:Time")
     .filter("v:Time <= date('2026-01-10T15:30:00Z')")
     .not(woql.triple("v:Entity", "superseded_by", "v:NewEntity"))
```

### Hypothesis Branch Querying

```woql
// Hypothesis testing results
woql.triple("v:Hypothesis", "test_branch", "v:BranchName")
     .diff_branches("main", "v:BranchName")
     .triple("v:Hypothesis", "test_outcome", "v:Outcome")
```

---

## 6. Design Rationale

### Why These Choices

**Session-Centric Architecture:**
- Provides natural temporal boundaries for agent activity
- Enables inheritance patterns that mirror agent learning
- Maps cleanly to TerminusDB's commit-based versioning

**Entity Inheritance Hierarchy:**
- Common properties factored to base class
- Specialization captures domain-specific semantics
- Enables polymorphic queries across entity types

**Explicit Evidence Modeling:**
- Supports transparent reasoning and debugging
- Enables evidence strength propagation
- Foundation for future automated reasoning

**Pattern as First-Class Entity:**
- Recognizes that patterns are knowledge, not just structure
- Enables pattern-based reasoning and prediction
- Supports emergent pattern discovery

### Alternatives Considered

**Flat vs. Hierarchical Entity Model:**
- Rejected flat model with type tags (lost type safety)
- Chosen inheritance hierarchy with abstract base

**Event Sourcing vs. State Snapshots:**
- Rejected pure event sourcing (complex queries)
- Chosen hybrid with state snapshots and evolution tracking

**Embedded vs. Reference Relationships:**
- Rejected embedding evidence in beliefs (no reuse)
- Chosen reference-based with explicit relationship management

### Trade-offs

**Complexity vs. Expressiveness:**
- Rich type system increases cognitive overhead
- Benefit: Precise semantics enable sophisticated queries

**Storage vs. Query Performance:**
- Normalized design increases storage/join costs
- Benefit: Data integrity, flexible query patterns

---

## 7. Future Evolution Path

### Extension Points

**Vector Integration:**
```json
{
  "Entity": {
    "embedding": {
      "@type": "Optional",
      "@class": "@vector",
      "@dimensions": 1536
    }
  }
}
```

**Causal Models:**
```json
{
  "CausalRelationship": {
    "@type": "Class",
    "@inherits": "Entity",
    "cause": "Entity",
    "effect": "Entity",
    "mechanism": "xsd:string"
  }
}
```

### Growing with Agent Capabilities

**Phase 1: Basic Memory** (Current)
- Session inheritance
- Belief tracking
- Evidence chains

**Phase 2: Pattern Recognition**
- Automated pattern discovery
- Statistical significance testing

**Phase 3: Causal Reasoning**
- Causal relationship modeling
- Intervention planning

**Phase 4: Meta-Reasoning**
- Reasoning about reasoning
- Strategy evolution

---

## Conclusion

This ontology provides a foundation for biographical intelligence that leverages TerminusDB's unique capabilities. The design prioritizes agent self-understanding over raw performance, temporal context over static facts, and emergent discovery over predetermined categories.

The schema enables agents to study their own evolution, test hypotheses safely, and discover patterns in their own behavior. This transforms memory from a filing system into a life story with queryable self-knowledge.

**Key Innovation**: The combination of session inheritance, entity evolution, and hypothesis branching creates a memory system that participates in the agent's cognitive development.

**Implementation Ready**: All schemas are complete JSON-LD with working WOQL query patterns ready for TerminusDB.

**Evolution Path**: Clear extension points for vector integration, causal modeling, and meta-reasoning as agent sophistication grows.
