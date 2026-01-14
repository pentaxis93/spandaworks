"""
Entity definitions for ops memory.

Phase 1 entities as defined in ops_memory_ontology.md:
- Session: Bounded interaction unit
- Event: Something that happened
- Learning: Knowledge product
- Decision: Choice made

These are dataclasses that map to TerminusDB documents.
"""

from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Optional, Set
from enum import Enum


def parse_datetime(dt_string: str) -> datetime:
    """
    Parse datetime string from TerminusDB, normalizing to naive UTC.

    TerminusDB returns ISO format with 'Z' suffix for UTC.
    We strip timezone info for consistent comparison with datetime.now().
    """
    # Handle Z suffix (UTC)
    if dt_string.endswith("Z"):
        dt_string = dt_string[:-1] + "+00:00"

    dt = datetime.fromisoformat(dt_string)

    # If timezone-aware, convert to UTC and make naive
    if dt.tzinfo is not None:
        dt = dt.astimezone(timezone.utc).replace(tzinfo=None)

    return dt


class EventType(str, Enum):
    """Types of events that can occur."""

    DECISION = "decision"
    FAILURE = "failure"
    SUCCESS = "success"
    DISCOVERY = "discovery"
    CALIBRATION = "calibration"
    EXTERNAL = "external"


class LearningDomain(str, Enum):
    """Domains of learning."""

    PROCEDURAL = "procedural"  # How to do things
    SEMANTIC = "semantic"  # Facts and concepts
    RELATIONAL = "relational"  # About people/relationships
    META = "meta"  # About learning/cognition itself


@dataclass
class Session:
    """
    A bounded interaction. The unit of episodic memory.

    Represents a single session with the user, tracking goals,
    mode, summary, and any open loops at the end.
    """

    session_id: str
    started_at: datetime
    mode: str
    goals: Set[str] = field(default_factory=set)
    ended_at: Optional[datetime] = None
    summary: Optional[str] = None
    open_loops_at_end: Set[str] = field(default_factory=set)

    # TerminusDB document ID (set after insert)
    _id: Optional[str] = None

    def to_document(self) -> dict:
        """Convert to TerminusDB document format."""
        doc = {
            "@type": "Session",
            "session_id": self.session_id,
            "started_at": self.started_at.isoformat(),
            "mode": self.mode,
            "goals": list(self.goals),
        }
        if self.ended_at:
            doc["ended_at"] = self.ended_at.isoformat()
        if self.summary:
            doc["summary"] = self.summary
        if self.open_loops_at_end:
            doc["open_loops_at_end"] = list(self.open_loops_at_end)
        return doc

    @classmethod
    def from_document(cls, doc: dict) -> "Session":
        """Create Session from TerminusDB document."""
        return cls(
            session_id=doc["session_id"],
            started_at=parse_datetime(doc["started_at"]),
            mode=doc["mode"],
            goals=set(doc.get("goals", [])),
            ended_at=parse_datetime(doc["ended_at"]) if doc.get("ended_at") else None,
            summary=doc.get("summary"),
            open_loops_at_end=set(doc.get("open_loops_at_end", [])),
            _id=doc.get("@id"),
        )


@dataclass
class Event:
    """
    Something that happened. Can be within a session or in the external world.

    Supports bi-temporal model: when it occurred vs when we learned about it.
    """

    description: str
    occurred_at: datetime
    learned_at: datetime
    event_type: EventType
    significance: Optional[str] = None

    # TerminusDB document ID (set after insert)
    _id: Optional[str] = None

    def to_document(self) -> dict:
        """Convert to TerminusDB document format."""
        doc = {
            "@type": "Event",
            "description": self.description,
            "occurred_at": self.occurred_at.isoformat(),
            "learned_at": self.learned_at.isoformat(),
            "event_type": self.event_type.value,
        }
        if self.significance:
            doc["significance"] = self.significance
        return doc

    @classmethod
    def from_document(cls, doc: dict) -> "Event":
        """Create Event from TerminusDB document."""
        return cls(
            description=doc["description"],
            occurred_at=parse_datetime(doc["occurred_at"]),
            learned_at=parse_datetime(doc["learned_at"]),
            event_type=EventType(doc["event_type"]),
            significance=doc.get("significance"),
            _id=doc.get("@id"),
        )


@dataclass
class Learning:
    """
    Something learned. The product of experience.

    Tracks content, confidence, and domain. Can supersede previous learnings.
    """

    content: str
    learned_at: datetime
    confidence: float  # 0.0 to 1.0
    domain: LearningDomain
    supersedes_id: Optional[str] = None  # ID of Learning this supersedes

    # TerminusDB document ID (set after insert)
    _id: Optional[str] = None

    def to_document(self) -> dict:
        """Convert to TerminusDB document format."""
        doc = {
            "@type": "Learning",
            "content": self.content,
            "learned_at": self.learned_at.isoformat(),
            "confidence": self.confidence,
            "domain": self.domain.value,
        }
        if self.supersedes_id:
            doc["supersedes"] = {"@id": self.supersedes_id, "@type": "@id"}
        return doc

    @classmethod
    def from_document(cls, doc: dict) -> "Learning":
        """Create Learning from TerminusDB document."""
        supersedes = doc.get("supersedes")
        supersedes_id = None
        if supersedes:
            if isinstance(supersedes, dict):
                supersedes_id = supersedes.get("@id")
            else:
                supersedes_id = supersedes

        return cls(
            content=doc["content"],
            learned_at=parse_datetime(doc["learned_at"]),
            confidence=float(doc["confidence"]),
            domain=LearningDomain(doc["domain"]),
            supersedes_id=supersedes_id,
            _id=doc.get("@id"),
        )


@dataclass
class Decision:
    """
    A choice made, with context and outcome.

    Tracks the decision, rationale, options considered, and eventual outcome.
    """

    description: str
    made_at: datetime
    context: str
    rationale: str
    options_considered: Set[str] = field(default_factory=set)
    outcome: Optional[str] = None
    outcome_assessed_at: Optional[datetime] = None

    # TerminusDB document ID (set after insert)
    _id: Optional[str] = None

    def to_document(self) -> dict:
        """Convert to TerminusDB document format."""
        doc = {
            "@type": "Decision",
            "description": self.description,
            "made_at": self.made_at.isoformat(),
            "context": self.context,
            "rationale": self.rationale,
            "options_considered": list(self.options_considered),
        }
        if self.outcome:
            doc["outcome"] = self.outcome
        if self.outcome_assessed_at:
            doc["outcome_assessed_at"] = self.outcome_assessed_at.isoformat()
        return doc

    @classmethod
    def from_document(cls, doc: dict) -> "Decision":
        """Create Decision from TerminusDB document."""
        return cls(
            description=doc["description"],
            made_at=parse_datetime(doc["made_at"]),
            context=doc["context"],
            rationale=doc["rationale"],
            options_considered=set(doc.get("options_considered", [])),
            outcome=doc.get("outcome"),
            outcome_assessed_at=parse_datetime(doc["outcome_assessed_at"])
            if doc.get("outcome_assessed_at")
            else None,
            _id=doc.get("@id"),
        )
