"""
OpsMemory client - main interface for ops memory operations.

Provides CRUD operations for Session, Event, Learning, Decision entities
and relationship management.
"""

from datetime import datetime
from typing import Optional, List, Set
from terminusdb_client import Client

from .entities import Session, Event, Learning, Decision, EventType, LearningDomain
from .schema import install_schema, verify_schema


def normalize_id(doc_id: Optional[str]) -> str:
    """
    Normalize document ID to short form for comparison.

    TerminusDB returns full URIs from insert but short forms from queries:
    - Full: "terminusdb:///data/Learning/abc123"
    - Short: "Learning/abc123"

    This function normalizes to short form for consistent comparison.
    """
    if not doc_id:
        return ""
    if "terminusdb:///" in doc_id:
        return doc_id.split("/data/")[-1] if "/data/" in doc_id else doc_id
    return doc_id


def ids_match(id1: Optional[str], id2: Optional[str]) -> bool:
    """Check if two document IDs match (handling different formats)."""
    return normalize_id(id1) == normalize_id(id2)


# Default connection settings
DEFAULT_ENDPOINT = "http://localhost:6363"
DEFAULT_TEAM = "admin"
DEFAULT_USER = "admin"
DEFAULT_PASSWORD = "aiandi_memory_2024"
DEFAULT_DATABASE = "ops_memory"


class OpsMemory:
    """
    Main interface for ops biographical memory.

    Provides:
    - Entity CRUD (create, read, update, delete)
    - Relationship management
    - Temporal queries
    - Session logging
    - Learning capture

    Example:
        memory = OpsMemory()

        # Start a session
        session = memory.create_session(
            session_id="2026-01-14-ops",
            mode="ops",
            goals={"Handle inbox", "Plan week"}
        )

        # Log a learning
        learning = memory.create_learning(
            content="Weekly review is most effective Monday morning",
            domain=LearningDomain.PROCEDURAL,
            confidence=0.8
        )

        # Link them
        memory.link_session_produced_learning(session._id, learning._id)
    """

    def __init__(
        self,
        endpoint: str = DEFAULT_ENDPOINT,
        team: str = DEFAULT_TEAM,
        user: str = DEFAULT_USER,
        password: str = DEFAULT_PASSWORD,
        database: str = DEFAULT_DATABASE,
        auto_connect: bool = True,
    ):
        """
        Initialize OpsMemory client.

        Args:
            endpoint: TerminusDB server URL
            team: TerminusDB team/organization
            user: TerminusDB username
            password: TerminusDB password
            database: Database name
            auto_connect: If True, connect immediately
        """
        self.endpoint = endpoint
        self.team = team
        self.user = user
        self.password = password
        self.database = database

        self._client: Optional[Client] = None

        if auto_connect:
            self.connect()

    def connect(self) -> None:
        """Connect to the TerminusDB database."""
        self._client = Client(self.endpoint)
        self._client.connect(
            user=self.user,
            key=self.password,
            team=self.team,
            db=self.database,
        )

    def disconnect(self) -> None:
        """Disconnect from the database."""
        self._client = None

    @property
    def client(self) -> Client:
        """Get the underlying TerminusDB client."""
        if self._client is None:
            raise RuntimeError("Not connected. Call connect() first.")
        return self._client

    def ensure_schema(self, force: bool = False) -> bool:
        """
        Ensure the schema is installed.

        Args:
            force: If True, reinstall even if schema exists

        Returns:
            True if schema was installed, False if already existed
        """
        return install_schema(self.client, force=force)

    def verify_schema(self) -> dict:
        """Verify schema installation. Returns verification results dict."""
        return verify_schema(self.client)

    # =========================================================================
    # Session CRUD
    # =========================================================================

    def create_session(
        self,
        session_id: str,
        mode: str,
        goals: Optional[Set[str]] = None,
        started_at: Optional[datetime] = None,
    ) -> Session:
        """
        Create a new session.

        Args:
            session_id: Unique identifier (e.g., "2026-01-14-ops-morning")
            mode: Operating mode (e.g., "ops", "governance", "standard")
            goals: Set of session goals
            started_at: When session started (defaults to now)

        Returns:
            Created Session with _id set
        """
        session = Session(
            session_id=session_id,
            mode=mode,
            goals=goals or set(),
            started_at=started_at or datetime.now(),
        )

        doc = session.to_document()
        result = self.client.insert_document(doc)

        # Get the assigned ID
        if isinstance(result, list) and len(result) > 0:
            session._id = result[0]

        return session

    def get_session(self, session_id: str) -> Optional[Session]:
        """
        Get a session by its session_id.

        Args:
            session_id: The session's unique identifier

        Returns:
            Session if found, None otherwise
        """
        # Query by session_id field
        docs = list(
            self.client.query_document({"@type": "Session", "session_id": session_id})
        )

        if docs:
            return Session.from_document(docs[0])
        return None

    def get_session_by_id(self, doc_id: str) -> Optional[Session]:
        """
        Get a session by its TerminusDB document ID.

        Args:
            doc_id: The TerminusDB document ID

        Returns:
            Session if found, None otherwise
        """
        try:
            doc = self.client.get_document(doc_id)
            return Session.from_document(doc)
        except Exception:
            return None

    def update_session(
        self,
        session_id: str,
        ended_at: Optional[datetime] = None,
        summary: Optional[str] = None,
        open_loops_at_end: Optional[Set[str]] = None,
    ) -> Optional[Session]:
        """
        Update an existing session (typically to close it).

        Args:
            session_id: The session's unique identifier
            ended_at: When session ended
            summary: Session summary
            open_loops_at_end: Open loops at session end

        Returns:
            Updated Session, or None if not found
        """
        session = self.get_session(session_id)
        if not session:
            return None

        # Update fields
        if ended_at:
            session.ended_at = ended_at
        if summary:
            session.summary = summary
        if open_loops_at_end:
            session.open_loops_at_end = open_loops_at_end

        # Replace document
        doc = session.to_document()
        doc["@id"] = session._id
        self.client.update_document(doc)

        return session

    def close_session(
        self,
        session_id: str,
        summary: str,
        open_loops: Optional[Set[str]] = None,
    ) -> Optional[Session]:
        """
        Close a session with summary.

        Convenience method that sets ended_at to now.

        Args:
            session_id: The session's unique identifier
            summary: What happened in this session
            open_loops: Any open loops to carry forward

        Returns:
            Updated Session, or None if not found
        """
        return self.update_session(
            session_id=session_id,
            ended_at=datetime.now(),
            summary=summary,
            open_loops_at_end=open_loops or set(),
        )

    def get_recent_sessions(self, limit: int = 10) -> List[Session]:
        """
        Get most recent sessions, ordered by start time descending.

        Args:
            limit: Maximum number of sessions to return

        Returns:
            List of Sessions, most recent first
        """
        docs = list(self.client.query_document({"@type": "Session"}))

        sessions = [Session.from_document(doc) for doc in docs]
        sessions.sort(key=lambda s: s.started_at, reverse=True)

        return sessions[:limit]

    def get_last_session(self) -> Optional[Session]:
        """Get the most recent session."""
        sessions = self.get_recent_sessions(limit=1)
        return sessions[0] if sessions else None

    # =========================================================================
    # Event CRUD
    # =========================================================================

    def create_event(
        self,
        description: str,
        event_type: EventType,
        occurred_at: Optional[datetime] = None,
        learned_at: Optional[datetime] = None,
        significance: Optional[str] = None,
    ) -> Event:
        """
        Create a new event.

        Args:
            description: What happened
            event_type: Type of event
            occurred_at: When it happened (defaults to now)
            learned_at: When we learned about it (defaults to now)
            significance: Why this matters

        Returns:
            Created Event with _id set
        """
        now = datetime.now()
        event = Event(
            description=description,
            event_type=event_type,
            occurred_at=occurred_at or now,
            learned_at=learned_at or now,
            significance=significance,
        )

        doc = event.to_document()
        result = self.client.insert_document(doc)

        if isinstance(result, list) and len(result) > 0:
            event._id = result[0]

        return event

    def get_event(self, doc_id: str) -> Optional[Event]:
        """Get an event by its document ID."""
        try:
            doc = self.client.get_document(doc_id)
            return Event.from_document(doc)
        except Exception:
            return None

    def get_events_by_type(
        self,
        event_type: EventType,
        limit: int = 50,
    ) -> List[Event]:
        """Get events of a specific type."""
        docs = list(
            self.client.query_document(
                {
                    "@type": "Event",
                    "event_type": event_type.value,
                }
            )
        )

        events = [Event.from_document(doc) for doc in docs]
        events.sort(key=lambda e: e.occurred_at, reverse=True)

        return events[:limit]

    def get_recent_events(self, limit: int = 20) -> List[Event]:
        """Get most recent events."""
        docs = list(self.client.query_document({"@type": "Event"}))

        events = [Event.from_document(doc) for doc in docs]
        events.sort(key=lambda e: e.occurred_at, reverse=True)

        return events[:limit]

    # =========================================================================
    # Learning CRUD
    # =========================================================================

    def create_learning(
        self,
        content: str,
        domain: LearningDomain,
        confidence: float = 0.8,
        learned_at: Optional[datetime] = None,
        supersedes_id: Optional[str] = None,
    ) -> Learning:
        """
        Create a new learning.

        Args:
            content: What was learned
            domain: Domain of the learning
            confidence: Confidence level (0.0-1.0)
            learned_at: When it was learned (defaults to now)
            supersedes_id: ID of learning this supersedes

        Returns:
            Created Learning with _id set
        """
        learning = Learning(
            content=content,
            domain=domain,
            confidence=min(1.0, max(0.0, confidence)),  # Clamp to 0-1
            learned_at=learned_at or datetime.now(),
            supersedes_id=supersedes_id,
        )

        doc = learning.to_document()
        result = self.client.insert_document(doc)

        if isinstance(result, list) and len(result) > 0:
            learning._id = result[0]

        return learning

    def get_learning(self, doc_id: str) -> Optional[Learning]:
        """Get a learning by its document ID."""
        try:
            doc = self.client.get_document(doc_id)
            return Learning.from_document(doc)
        except Exception:
            return None

    def get_learnings_by_domain(
        self,
        domain: LearningDomain,
        limit: int = 50,
    ) -> List[Learning]:
        """Get learnings in a specific domain."""
        docs = list(
            self.client.query_document(
                {
                    "@type": "Learning",
                    "domain": domain.value,
                }
            )
        )

        learnings = [Learning.from_document(doc) for doc in docs]
        learnings.sort(key=lambda l: l.learned_at, reverse=True)

        return learnings[:limit]

    def get_recent_learnings(self, limit: int = 20) -> List[Learning]:
        """Get most recent learnings."""
        docs = list(self.client.query_document({"@type": "Learning"}))

        learnings = [Learning.from_document(doc) for doc in docs]
        learnings.sort(key=lambda l: l.learned_at, reverse=True)

        return learnings[:limit]

    def search_learnings(self, keyword: str) -> List[Learning]:
        """
        Search learnings by keyword in content.

        Note: This is a simple contains search. For more sophisticated
        search, consider using TerminusDB's full-text capabilities.
        """
        docs = list(self.client.query_document({"@type": "Learning"}))

        learnings = []
        keyword_lower = keyword.lower()
        for doc in docs:
            if keyword_lower in doc.get("content", "").lower():
                learnings.append(Learning.from_document(doc))

        learnings.sort(key=lambda l: l.learned_at, reverse=True)
        return learnings

    # =========================================================================
    # Decision CRUD
    # =========================================================================

    def create_decision(
        self,
        description: str,
        context: str,
        rationale: str,
        options_considered: Optional[Set[str]] = None,
        made_at: Optional[datetime] = None,
    ) -> Decision:
        """
        Create a new decision.

        Args:
            description: What was decided
            context: Situation/context of the decision
            rationale: Why this option was chosen
            options_considered: Other options that were considered
            made_at: When the decision was made (defaults to now)

        Returns:
            Created Decision with _id set
        """
        decision = Decision(
            description=description,
            context=context,
            rationale=rationale,
            options_considered=options_considered or set(),
            made_at=made_at or datetime.now(),
        )

        doc = decision.to_document()
        result = self.client.insert_document(doc)

        if isinstance(result, list) and len(result) > 0:
            decision._id = result[0]

        return decision

    def get_decision(self, doc_id: str) -> Optional[Decision]:
        """Get a decision by its document ID."""
        try:
            doc = self.client.get_document(doc_id)
            return Decision.from_document(doc)
        except Exception:
            return None

    def update_decision_outcome(
        self,
        doc_id: str,
        outcome: str,
        assessed_at: Optional[datetime] = None,
    ) -> Optional[Decision]:
        """
        Update a decision with its outcome.

        Args:
            doc_id: Decision document ID
            outcome: What happened as a result
            assessed_at: When outcome was assessed (defaults to now)

        Returns:
            Updated Decision, or None if not found
        """
        decision = self.get_decision(doc_id)
        if not decision:
            return None

        decision.outcome = outcome
        decision.outcome_assessed_at = assessed_at or datetime.now()

        doc = decision.to_document()
        doc["@id"] = doc_id
        self.client.update_document(doc)

        return decision

    def get_recent_decisions(self, limit: int = 20) -> List[Decision]:
        """Get most recent decisions."""
        docs = list(self.client.query_document({"@type": "Decision"}))

        decisions = [Decision.from_document(doc) for doc in docs]
        decisions.sort(key=lambda d: d.made_at, reverse=True)

        return decisions[:limit]

    # =========================================================================
    # Relationship Management
    # =========================================================================

    def link_session_produced_learning(
        self,
        session_id: str,
        learning_id: str,
    ) -> str:
        """
        Link a session to a learning it produced.

        Args:
            session_id: Session document ID
            learning_id: Learning document ID

        Returns:
            ID of the created relationship document
        """
        doc = {
            "@type": "SessionProducedLearning",
            "session": {"@id": session_id, "@type": "@id"},
            "learning": {"@id": learning_id, "@type": "@id"},
        }
        result = self.client.insert_document(doc)
        return result[0] if isinstance(result, list) else result

    def link_session_produced_decision(
        self,
        session_id: str,
        decision_id: str,
    ) -> str:
        """
        Link a session to a decision it produced.

        Args:
            session_id: Session document ID
            decision_id: Decision document ID

        Returns:
            ID of the created relationship document
        """
        doc = {
            "@type": "SessionProducedDecision",
            "session": {"@id": session_id, "@type": "@id"},
            "decision": {"@id": decision_id, "@type": "@id"},
        }
        result = self.client.insert_document(doc)
        return result[0] if isinstance(result, list) else result

    def link_learning_derived_from_event(
        self,
        learning_id: str,
        event_id: str,
    ) -> str:
        """
        Link a learning to the event it was derived from.

        Args:
            learning_id: Learning document ID
            event_id: Event document ID

        Returns:
            ID of the created relationship document
        """
        doc = {
            "@type": "LearningDerivedFromEvent",
            "learning": {"@id": learning_id, "@type": "@id"},
            "event": {"@id": event_id, "@type": "@id"},
        }
        result = self.client.insert_document(doc)
        return result[0] if isinstance(result, list) else result

    def link_decision_led_to_event(
        self,
        decision_id: str,
        event_id: str,
    ) -> str:
        """
        Link a decision to an event it led to.

        Args:
            decision_id: Decision document ID
            event_id: Event document ID

        Returns:
            ID of the created relationship document
        """
        doc = {
            "@type": "DecisionLedToEvent",
            "decision": {"@id": decision_id, "@type": "@id"},
            "event": {"@id": event_id, "@type": "@id"},
        }
        result = self.client.insert_document(doc)
        return result[0] if isinstance(result, list) else result

    def link_session_followed(
        self,
        later_session_id: str,
        earlier_session_id: str,
        gap_days: Optional[int] = None,
    ) -> str:
        """
        Link a session to the one it followed.

        Args:
            later_session_id: The later session's document ID
            earlier_session_id: The earlier session's document ID
            gap_days: Number of days between sessions

        Returns:
            ID of the created relationship document
        """
        doc = {
            "@type": "SessionFollowed",
            "later_session": {"@id": later_session_id, "@type": "@id"},
            "earlier_session": {"@id": earlier_session_id, "@type": "@id"},
        }
        if gap_days is not None:
            doc["gap_days"] = gap_days

        result = self.client.insert_document(doc)
        return result[0] if isinstance(result, list) else result

    def get_learnings_for_session(self, session_id: str) -> List[Learning]:
        """Get all learnings produced by a session."""
        docs = list(
            self.client.query_document(
                {
                    "@type": "SessionProducedLearning",
                }
            )
        )

        learnings = []
        for doc in docs:
            session_ref = doc.get("session")
            if isinstance(session_ref, dict):
                ref_id = session_ref.get("@id")
            else:
                ref_id = session_ref

            # Use ids_match for proper comparison (handles full vs short form)
            if ids_match(ref_id, session_id):
                learning_ref = doc.get("learning")
                if isinstance(learning_ref, dict):
                    learning_id = learning_ref.get("@id")
                else:
                    learning_id = learning_ref

                if learning_id:
                    learning = self.get_learning(learning_id)
                    if learning:
                        learnings.append(learning)

        return learnings

    def get_decisions_for_session(self, session_id: str) -> List[Decision]:
        """Get all decisions produced by a session."""
        docs = list(
            self.client.query_document(
                {
                    "@type": "SessionProducedDecision",
                }
            )
        )

        decisions = []
        for doc in docs:
            session_ref = doc.get("session")
            if isinstance(session_ref, dict):
                ref_id = session_ref.get("@id")
            else:
                ref_id = session_ref

            # Use ids_match for proper comparison (handles full vs short form)
            if ids_match(ref_id, session_id):
                decision_ref = doc.get("decision")
                if isinstance(decision_ref, dict):
                    decision_id = decision_ref.get("@id")
                else:
                    decision_id = decision_ref

                if decision_id:
                    decision = self.get_decision(decision_id)
                    if decision:
                        decisions.append(decision)

        return decisions

    # =========================================================================
    # Temporal Queries
    # =========================================================================

    def get_learnings_before(
        self,
        before: datetime,
        domain: Optional[LearningDomain] = None,
    ) -> List[Learning]:
        """
        Get learnings from before a specific time.

        This enables "what did I know at time T?" queries.

        Args:
            before: Get learnings from before this time
            domain: Optionally filter by domain

        Returns:
            List of learnings, most recent first
        """
        query = {"@type": "Learning"}
        if domain:
            query["domain"] = domain.value

        docs = list(self.client.query_document(query))

        learnings = []
        for doc in docs:
            learning = Learning.from_document(doc)
            if learning.learned_at < before:
                learnings.append(learning)

        learnings.sort(key=lambda l: l.learned_at, reverse=True)
        return learnings

    def get_events_in_range(
        self,
        start: datetime,
        end: datetime,
    ) -> List[Event]:
        """
        Get events that occurred within a time range.

        Args:
            start: Start of range (inclusive)
            end: End of range (inclusive)

        Returns:
            List of events, ordered by occurrence time
        """
        docs = list(self.client.query_document({"@type": "Event"}))

        events = []
        for doc in docs:
            event = Event.from_document(doc)
            if start <= event.occurred_at <= end:
                events.append(event)

        events.sort(key=lambda e: e.occurred_at)
        return events

    # =========================================================================
    # Convenience Methods for Ops Integration
    # =========================================================================

    def log_session_start(
        self,
        session_id: str,
        mode: str = "ops",
        goals: Optional[Set[str]] = None,
    ) -> Session:
        """
        Log the start of an ops session.

        Convenience method that creates a session and links it to the
        previous session if one exists.

        Args:
            session_id: Unique session identifier
            mode: Operating mode
            goals: Session goals

        Returns:
            Created Session
        """
        # Get previous session for linking
        previous = self.get_last_session()

        # Create new session
        session = self.create_session(
            session_id=session_id,
            mode=mode,
            goals=goals,
        )

        # Link to previous if exists
        if previous and previous._id and session._id:
            gap = (session.started_at - previous.started_at).days
            self.link_session_followed(session._id, previous._id, gap_days=gap)

        return session

    def log_session_end(
        self,
        session_id: str,
        summary: str,
        learnings: Optional[List[str]] = None,
        decisions: Optional[List[dict]] = None,
        open_loops: Optional[Set[str]] = None,
    ) -> Session:
        """
        Log the end of an ops session with learnings and decisions.

        Convenience method that:
        1. Closes the session
        2. Creates learning documents
        3. Creates decision documents
        4. Links everything together

        Args:
            session_id: Session to close
            summary: What happened
            learnings: List of learning content strings
            decisions: List of decision dicts with description, context, rationale
            open_loops: Open loops to carry forward

        Returns:
            Updated Session
        """
        session = self.close_session(session_id, summary, open_loops)
        if not session or not session._id:
            raise ValueError(f"Session {session_id} not found")

        # Create and link learnings
        if learnings:
            for content in learnings:
                learning = self.create_learning(
                    content=content,
                    domain=LearningDomain.PROCEDURAL,  # Default to procedural
                )
                if learning._id:
                    self.link_session_produced_learning(session._id, learning._id)

        # Create and link decisions
        if decisions:
            for dec in decisions:
                decision = self.create_decision(
                    description=dec.get("description", ""),
                    context=dec.get("context", ""),
                    rationale=dec.get("rationale", ""),
                    options_considered=set(dec.get("options", [])),
                )
                if decision._id:
                    self.link_session_produced_decision(session._id, decision._id)

        return session

    def get_context_for_session_start(self) -> dict:
        """
        Get relevant context for starting a new session.

        Returns recent sessions, open loops, and recent learnings
        to help ops orient.

        Returns:
            Dict with context information:
            {
                "last_session": Session or None,
                "recent_sessions": List[Session],
                "open_loops": Set[str],
                "recent_learnings": List[Learning],
                "recent_decisions": List[Decision],
            }
        """
        last = self.get_last_session()
        recent = self.get_recent_sessions(limit=5)

        # Aggregate open loops from recent sessions
        open_loops = set()
        for session in recent:
            if session.open_loops_at_end:
                open_loops.update(session.open_loops_at_end)

        return {
            "last_session": last,
            "recent_sessions": recent,
            "open_loops": open_loops,
            "recent_learnings": self.get_recent_learnings(limit=5),
            "recent_decisions": self.get_recent_decisions(limit=5),
        }

    def get_all_documents_count(self) -> dict:
        """
        Get counts of all document types.

        Returns:
            Dict with counts by type
        """
        counts = {}
        for doc_type in ["Session", "Event", "Learning", "Decision"]:
            docs = list(self.client.query_document({"@type": doc_type}))
            counts[doc_type] = len(docs)
        return counts
