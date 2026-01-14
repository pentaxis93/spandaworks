#!/usr/bin/env python3
"""
Test suite for ops memory operations.

Run from the terminusdb directory:
    ./.venv/bin/python -m ops_memory.test_ops_memory

Or with pytest:
    ./.venv/bin/pytest ops_memory/test_ops_memory.py -v
"""

import sys
from datetime import datetime, timedelta

# Add parent to path for imports when running as script
if __name__ == "__main__":
    import os

    sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from ops_memory.client import OpsMemory
from ops_memory.entities import (
    Session,
    Event,
    Learning,
    Decision,
    EventType,
    LearningDomain,
)
from ops_memory.schema import install_schema, verify_schema


def normalize_id(doc_id: str | None) -> str:
    """Normalize document ID to short form for comparison."""
    if not doc_id:
        return ""
    if "terminusdb:///" in doc_id:
        # Extract the short form (e.g., "Learning/abc123" from "terminusdb:///data/Learning/abc123")
        return doc_id.split("/data/")[-1] if "/data/" in doc_id else doc_id
    return doc_id


def ids_match(id1: str | None, id2: str | None) -> bool:
    """Check if two document IDs match (handling different formats)."""
    return normalize_id(id1) == normalize_id(id2)


def test_connection():
    """Test basic connection to TerminusDB."""
    print("Test: Connection...")
    memory = OpsMemory()
    assert memory._client is not None
    print("  PASS: Connected to TerminusDB")
    return memory


def test_schema_installation(memory: OpsMemory):
    """Test schema installation."""
    print("Test: Schema installation...")

    # Install schema (may already exist)
    memory.ensure_schema()

    # Verify
    result = verify_schema(memory.client)
    assert result["valid"], f"Schema invalid: {result}"
    print(f"  PASS: Schema is valid (version {result['version']})")
    print(f"  Classes: {', '.join(result['classes'])}")


def test_session_crud(memory: OpsMemory):
    """Test Session CRUD operations."""
    print("Test: Session CRUD...")

    # Create
    session_id = f"test-session-{datetime.now().strftime('%Y%m%d%H%M%S')}"
    session = memory.create_session(
        session_id=session_id,
        mode="test",
        goals={"Test goal 1", "Test goal 2"},
    )
    assert session._id is not None, "Session should have ID after creation"
    print(f"  Created session: {session_id}")

    # Read
    retrieved = memory.get_session(session_id)
    assert retrieved is not None, "Should retrieve session"
    assert retrieved.session_id == session_id
    assert retrieved.mode == "test"
    assert "Test goal 1" in retrieved.goals
    print(f"  Retrieved session: {retrieved.session_id}")

    # Update (close)
    closed = memory.close_session(
        session_id=session_id,
        summary="Test session completed successfully",
        open_loops={"Follow up on test"},
    )
    assert closed is not None
    assert closed.ended_at is not None
    assert closed.summary is not None
    print(f"  Closed session with summary")

    # List
    recent = memory.get_recent_sessions(limit=5)
    assert len(recent) > 0
    assert any(s.session_id == session_id for s in recent)
    print(f"  Listed {len(recent)} recent sessions")

    print("  PASS: Session CRUD working")


def test_learning_crud(memory: OpsMemory):
    """Test Learning CRUD operations."""
    print("Test: Learning CRUD...")

    # Create
    learning = memory.create_learning(
        content="Test learning: TerminusDB schema must be installed before documents",
        domain=LearningDomain.PROCEDURAL,
        confidence=0.9,
    )
    assert learning._id is not None
    print(f"  Created learning: {learning._id}")

    # Read
    retrieved = memory.get_learning(learning._id)
    assert retrieved is not None
    assert retrieved.content == learning.content
    assert retrieved.confidence == 0.9
    print(f"  Retrieved learning")

    # List by domain
    procedural = memory.get_learnings_by_domain(LearningDomain.PROCEDURAL, limit=5)
    assert any(ids_match(l._id, learning._id) for l in procedural)
    print(f"  Listed {len(procedural)} procedural learnings")

    # Search
    results = memory.search_learnings("TerminusDB")
    assert len(results) > 0
    print(f"  Search found {len(results)} results")

    print("  PASS: Learning CRUD working")
    return learning


def test_event_crud(memory: OpsMemory):
    """Test Event CRUD operations."""
    print("Test: Event CRUD...")

    # Create
    event = memory.create_event(
        description="Test event: Memory system operational",
        event_type=EventType.SUCCESS,
        significance="Milestone achieved",
    )
    assert event._id is not None
    print(f"  Created event: {event._id}")

    # Read
    retrieved = memory.get_event(event._id)
    assert retrieved is not None
    assert retrieved.event_type == EventType.SUCCESS
    print(f"  Retrieved event")

    # List by type
    successes = memory.get_events_by_type(EventType.SUCCESS, limit=5)
    assert len(successes) > 0
    print(f"  Listed {len(successes)} success events")

    print("  PASS: Event CRUD working")
    return event


def test_decision_crud(memory: OpsMemory):
    """Test Decision CRUD operations."""
    print("Test: Decision CRUD...")

    # Create
    decision = memory.create_decision(
        description="Use TerminusDB for biographical memory",
        context="Need persistent memory for ops mode",
        rationale="Git-like versioning and bi-temporal model fit the use case",
        options_considered={"SQLite", "MongoDB", "PostgreSQL", "TerminusDB"},
    )
    assert decision._id is not None
    print(f"  Created decision: {decision._id}")

    # Read
    retrieved = memory.get_decision(decision._id)
    assert retrieved is not None
    assert "TerminusDB" in retrieved.description
    print(f"  Retrieved decision")

    # Update outcome
    updated = memory.update_decision_outcome(
        doc_id=decision._id,
        outcome="System operational, decision validated",
    )
    assert updated is not None
    assert updated.outcome is not None
    print(f"  Updated decision outcome")

    # List
    recent = memory.get_recent_decisions(limit=5)
    assert len(recent) > 0
    print(f"  Listed {len(recent)} recent decisions")

    print("  PASS: Decision CRUD working")
    return decision


def test_relationships(memory: OpsMemory):
    """Test relationship creation."""
    print("Test: Relationships...")

    # Create test entities
    session = memory.create_session(
        session_id=f"rel-test-{datetime.now().strftime('%H%M%S')}",
        mode="test",
    )

    learning = memory.create_learning(
        content="Relationships in TerminusDB are first-class entities",
        domain=LearningDomain.SEMANTIC,
    )

    decision = memory.create_decision(
        description="Test decision for relationship",
        context="Testing relationship creation",
        rationale="Need to verify relationship mechanics",
    )

    event = memory.create_event(
        description="Relationship test completed",
        event_type=EventType.SUCCESS,
    )

    # Create relationships
    rel1 = memory.link_session_produced_learning(session._id, learning._id)
    print(f"  Linked session -> learning: {rel1}")

    rel2 = memory.link_session_produced_decision(session._id, decision._id)
    print(f"  Linked session -> decision: {rel2}")

    rel3 = memory.link_learning_derived_from_event(learning._id, event._id)
    print(f"  Linked learning -> event: {rel3}")

    rel4 = memory.link_decision_led_to_event(decision._id, event._id)
    print(f"  Linked decision -> event: {rel4}")

    # Query relationships
    session_learnings = memory.get_learnings_for_session(session._id)
    assert len(session_learnings) > 0
    print(f"  Retrieved {len(session_learnings)} learnings for session")

    session_decisions = memory.get_decisions_for_session(session._id)
    assert len(session_decisions) > 0
    print(f"  Retrieved {len(session_decisions)} decisions for session")

    print("  PASS: Relationships working")


def test_temporal_queries(memory: OpsMemory):
    """Test temporal query capabilities."""
    print("Test: Temporal queries...")

    # Create some learnings with different times
    now = datetime.now()
    past = now - timedelta(days=1)

    old_learning = memory.create_learning(
        content="Historical learning for temporal test",
        domain=LearningDomain.META,
        learned_at=past,
    )

    new_learning = memory.create_learning(
        content="Recent learning for temporal test",
        domain=LearningDomain.META,
    )

    # Query learnings before now (should include old, maybe include new)
    before_now = memory.get_learnings_before(now + timedelta(seconds=1))
    print(f"  Found {len(before_now)} learnings before now")

    # Query learnings before yesterday (should not include new)
    before_past = memory.get_learnings_before(past)
    for l in before_past:
        assert l._id != new_learning._id, "New learning should not appear in old query"
    print(f"  Temporal filtering working")

    # Test event range query
    events = memory.get_events_in_range(past, now + timedelta(days=1))
    print(f"  Found {len(events)} events in range")

    print("  PASS: Temporal queries working")


def test_ops_convenience_methods(memory: OpsMemory):
    """Test ops-specific convenience methods."""
    print("Test: Ops convenience methods...")

    # Log session start
    session = memory.log_session_start(
        session_id=f"ops-test-{datetime.now().strftime('%H%M%S')}",
        mode="ops",
        goals={"Test ops integration"},
    )
    print(f"  Logged session start: {session.session_id}")

    # Get context
    context = memory.get_context_for_session_start()
    assert "last_session" in context
    assert "recent_sessions" in context
    assert "open_loops" in context
    print(f"  Got context: {len(context['recent_sessions'])} recent sessions")

    # Log session end with learnings
    closed = memory.log_session_end(
        session_id=session.session_id,
        summary="Test ops session completed",
        learnings=["Ops convenience methods work well"],
        open_loops={"Continue testing in production"},
    )
    print(f"  Logged session end")

    # Verify learning was created and linked
    session_learnings = memory.get_learnings_for_session(closed._id)
    assert len(session_learnings) > 0
    print(f"  Session has {len(session_learnings)} linked learnings")

    # Get stats
    stats = memory.get_all_documents_count()
    print(f"  Stats: {stats}")

    print("  PASS: Ops convenience methods working")


def run_all_tests():
    """Run all tests."""
    print("=" * 60)
    print("Ops Memory Test Suite")
    print("=" * 60)
    print()

    try:
        # Connection
        memory = test_connection()
        print()

        # Schema
        test_schema_installation(memory)
        print()

        # CRUD operations
        test_session_crud(memory)
        print()

        test_learning_crud(memory)
        print()

        test_event_crud(memory)
        print()

        test_decision_crud(memory)
        print()

        # Relationships
        test_relationships(memory)
        print()

        # Temporal queries
        test_temporal_queries(memory)
        print()

        # Ops integration
        test_ops_convenience_methods(memory)
        print()

        print("=" * 60)
        print("ALL TESTS PASSED!")
        print("=" * 60)
        return 0

    except AssertionError as e:
        print(f"\nTEST FAILED: {e}")
        return 1
    except Exception as e:
        print(f"\nERROR: {e}")
        import traceback

        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(run_all_tests())
