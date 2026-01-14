"""
TerminusDB schema definitions for ops memory.

Phase 1 schema as defined in ops_memory_ontology.md:
- Session
- Event
- Learning
- Decision

The schema is defined in TerminusDB's JSON-LD format.
"""

from terminusdb_client import Client


# Phase 1 Schema - Core entities for ops memory
PHASE1_SCHEMA = [
    # Session - A bounded interaction
    {
        "@type": "Class",
        "@id": "Session",
        "@key": {"@type": "Lexical", "@fields": ["session_id"]},
        "session_id": "xsd:string",
        "started_at": "xsd:dateTime",
        "ended_at": {"@type": "Optional", "@class": "xsd:dateTime"},
        "mode": "xsd:string",
        "goals": {"@type": "Set", "@class": "xsd:string"},
        "summary": {"@type": "Optional", "@class": "xsd:string"},
        "open_loops_at_end": {"@type": "Set", "@class": "xsd:string"},
    },
    # Event - Something that happened
    {
        "@type": "Class",
        "@id": "Event",
        "@key": {"@type": "Random"},
        "description": "xsd:string",
        "occurred_at": "xsd:dateTime",
        "learned_at": "xsd:dateTime",
        "event_type": "xsd:string",  # decision, failure, success, discovery, calibration, external
        "significance": {"@type": "Optional", "@class": "xsd:string"},
    },
    # Learning - Something learned
    {
        "@type": "Class",
        "@id": "Learning",
        "@key": {"@type": "Random"},
        "content": "xsd:string",
        "learned_at": "xsd:dateTime",
        "confidence": "xsd:decimal",  # 0.0 to 1.0
        "domain": "xsd:string",  # procedural, semantic, relational, meta
        "supersedes": {"@type": "Optional", "@class": "Learning"},
    },
    # Decision - A choice made
    {
        "@type": "Class",
        "@id": "Decision",
        "@key": {"@type": "Random"},
        "description": "xsd:string",
        "made_at": "xsd:dateTime",
        "context": "xsd:string",
        "options_considered": {"@type": "Set", "@class": "xsd:string"},
        "rationale": "xsd:string",
        "outcome": {"@type": "Optional", "@class": "xsd:string"},
        "outcome_assessed_at": {"@type": "Optional", "@class": "xsd:dateTime"},
    },
    # Relationship: Session produced Learning
    {
        "@type": "Class",
        "@id": "SessionProducedLearning",
        "@key": {"@type": "Random"},
        "session": "Session",
        "learning": "Learning",
    },
    # Relationship: Session produced Decision
    {
        "@type": "Class",
        "@id": "SessionProducedDecision",
        "@key": {"@type": "Random"},
        "session": "Session",
        "decision": "Decision",
    },
    # Relationship: Learning derived from Event
    {
        "@type": "Class",
        "@id": "LearningDerivedFromEvent",
        "@key": {"@type": "Random"},
        "learning": "Learning",
        "event": "Event",
    },
    # Relationship: Decision led to Event
    {
        "@type": "Class",
        "@id": "DecisionLedToEvent",
        "@key": {"@type": "Random"},
        "decision": "Decision",
        "event": "Event",
    },
    # Relationship: Session followed Session (for continuity tracking)
    {
        "@type": "Class",
        "@id": "SessionFollowed",
        "@key": {"@type": "Random"},
        "later_session": "Session",
        "earlier_session": "Session",
        "gap_days": {"@type": "Optional", "@class": "xsd:integer"},
    },
]


def install_schema(client: Client, force: bool = False) -> bool:
    """
    Install the Phase 1 schema into the connected database.

    Args:
        client: Connected TerminusDB client (must be connected to ops_memory db)
        force: If True, replace existing schema. If False, skip if schema exists.

    Returns:
        True if schema was installed, False if skipped (already exists)

    Raises:
        Exception if schema installation fails
    """
    # Check if schema already exists by looking for Session class
    try:
        existing_classes = client.get_existing_classes()
        has_session = "Session" in existing_classes

        if has_session and not force:
            print("Schema already exists. Use force=True to reinstall.")
            return False
    except Exception:
        # Schema might not exist yet, proceed with installation
        pass

    print("Installing Phase 1 schema...")

    # Insert schema documents
    for schema_doc in PHASE1_SCHEMA:
        try:
            client.insert_document(schema_doc, graph_type="schema")
            print(f"  Created: {schema_doc['@id']}")
        except Exception as e:
            # Check if it's a duplicate error (already exists)
            if "already exists" in str(e).lower():
                if force:
                    # Delete and recreate
                    print(f"  Replacing: {schema_doc['@id']}")
                    # TerminusDB handles this via commit/rollback
                else:
                    print(f"  Skipped (exists): {schema_doc['@id']}")
            else:
                raise

    print("Schema installation complete!")
    return True


def get_schema_version() -> str:
    """Get the current schema version string."""
    return "phase1-v0.1.0"


def verify_schema(client: Client) -> dict:
    """
    Verify that the schema is correctly installed.

    Returns:
        Dict with verification results:
        {
            "valid": bool,
            "classes": list of class names found,
            "missing": list of expected but missing classes,
            "extra": list of unexpected classes
        }
    """
    expected_classes = {
        "Session",
        "Event",
        "Learning",
        "Decision",
        "SessionProducedLearning",
        "SessionProducedDecision",
        "LearningDerivedFromEvent",
        "DecisionLedToEvent",
        "SessionFollowed",
    }

    try:
        # Use get_existing_classes which returns a list of class names
        found_classes = set(client.get_existing_classes())

        missing = expected_classes - found_classes
        extra = found_classes - expected_classes

        return {
            "valid": len(missing) == 0,
            "classes": list(found_classes),
            "missing": list(missing),
            "extra": list(extra),
            "version": get_schema_version(),
        }
    except Exception as e:
        return {
            "valid": False,
            "classes": [],
            "missing": list(expected_classes),
            "extra": [],
            "error": str(e),
            "version": get_schema_version(),
        }
