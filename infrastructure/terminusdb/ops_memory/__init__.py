"""
Ops Memory - TerminusDB-backed biographical memory for ops mode.

This module provides read/write operations for the ops memory system,
enabling persistent storage of sessions, events, learnings, and decisions.

Usage:
    from ops_memory import OpsMemory

    memory = OpsMemory()

    # Create a session
    session = memory.create_session(
        session_id="2026-01-14-ops-work",
        mode="ops",
        goals=["Complete task X", "Review Y"]
    )

    # Log a learning
    learning = memory.create_learning(
        content="The TerminusDB client requires explicit schema before documents",
        domain="procedural",
        confidence=0.9
    )

    # Query recent sessions
    recent = memory.get_recent_sessions(limit=5)
"""

from .client import OpsMemory
from .entities import Session, Event, Learning, Decision
from .schema import install_schema

__all__ = [
    "OpsMemory",
    "Session",
    "Event",
    "Learning",
    "Decision",
    "install_schema",
]

__version__ = "0.1.0"
