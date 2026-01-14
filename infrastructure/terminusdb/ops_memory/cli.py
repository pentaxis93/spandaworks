#!/usr/bin/env python3
"""
CLI tool for ops memory operations.

Usage:
    # Install schema
    python -m ops_memory.cli schema install

    # Verify schema
    python -m ops_memory.cli schema verify

    # Create session
    python -m ops_memory.cli session create --id "2026-01-14-ops" --mode ops --goals "Task 1" "Task 2"

    # Close session
    python -m ops_memory.cli session close --id "2026-01-14-ops" --summary "Completed task 1, started task 2"

    # List sessions
    python -m ops_memory.cli session list

    # Create learning
    python -m ops_memory.cli learning create --content "New insight" --domain procedural

    # List learnings
    python -m ops_memory.cli learning list

    # Get context for session start
    python -m ops_memory.cli context

    # Show stats
    python -m ops_memory.cli stats
"""

import argparse
import json
import sys
from datetime import datetime
from typing import Optional

from .client import OpsMemory
from .entities import LearningDomain, EventType


def format_datetime(dt: Optional[datetime]) -> str:
    """Format datetime for display."""
    if dt is None:
        return "N/A"
    return dt.strftime("%Y-%m-%d %H:%M")


def format_session(session) -> str:
    """Format a session for display."""
    status = "OPEN" if session.ended_at is None else "CLOSED"
    lines = [
        f"Session: {session.session_id}",
        f"  Status: {status}",
        f"  Mode: {session.mode}",
        f"  Started: {format_datetime(session.started_at)}",
        f"  Ended: {format_datetime(session.ended_at)}",
    ]
    if session.goals:
        lines.append(f"  Goals: {', '.join(session.goals)}")
    if session.summary:
        lines.append(f"  Summary: {session.summary}")
    if session.open_loops_at_end:
        lines.append(f"  Open Loops: {', '.join(session.open_loops_at_end)}")
    return "\n".join(lines)


def format_learning(learning) -> str:
    """Format a learning for display."""
    return (
        f"Learning ({learning.domain.value}, {learning.confidence:.0%} confidence):\n"
        f"  {learning.content}\n"
        f"  Learned: {format_datetime(learning.learned_at)}"
    )


def format_decision(decision) -> str:
    """Format a decision for display."""
    lines = [
        f"Decision: {decision.description}",
        f"  Made: {format_datetime(decision.made_at)}",
        f"  Context: {decision.context}",
        f"  Rationale: {decision.rationale}",
    ]
    if decision.options_considered:
        lines.append(f"  Options: {', '.join(decision.options_considered)}")
    if decision.outcome:
        lines.append(f"  Outcome: {decision.outcome}")
    return "\n".join(lines)


def cmd_schema_install(args, memory: OpsMemory) -> int:
    """Install the schema."""
    try:
        installed = memory.ensure_schema(force=args.force)
        if installed:
            print("Schema installed successfully.")
        else:
            print("Schema already exists. Use --force to reinstall.")
        return 0
    except Exception as e:
        print(f"Error installing schema: {e}", file=sys.stderr)
        return 1


def cmd_schema_verify(args, memory: OpsMemory) -> int:
    """Verify the schema."""
    result = memory.verify_schema()
    print(f"Schema Version: {result.get('version', 'unknown')}")
    print(f"Valid: {result['valid']}")
    print(f"Classes: {', '.join(result['classes'])}")
    if result["missing"]:
        print(f"Missing: {', '.join(result['missing'])}")
    if result["extra"]:
        print(f"Extra: {', '.join(result['extra'])}")
    if result.get("error"):
        print(f"Error: {result['error']}")
    return 0 if result["valid"] else 1


def cmd_session_create(args, memory: OpsMemory) -> int:
    """Create a new session."""
    try:
        session = memory.create_session(
            session_id=args.id,
            mode=args.mode,
            goals=set(args.goals) if args.goals else None,
        )
        print(f"Created session: {session.session_id}")
        print(f"Document ID: {session._id}")
        return 0
    except Exception as e:
        print(f"Error creating session: {e}", file=sys.stderr)
        return 1


def cmd_session_close(args, memory: OpsMemory) -> int:
    """Close a session."""
    try:
        open_loops = set(args.open_loops) if args.open_loops else None
        session = memory.close_session(
            session_id=args.id,
            summary=args.summary,
            open_loops=open_loops,
        )
        if session:
            print(f"Closed session: {session.session_id}")
            return 0
        else:
            print(f"Session not found: {args.id}", file=sys.stderr)
            return 1
    except Exception as e:
        print(f"Error closing session: {e}", file=sys.stderr)
        return 1


def cmd_session_list(args, memory: OpsMemory) -> int:
    """List recent sessions."""
    sessions = memory.get_recent_sessions(limit=args.limit)
    if not sessions:
        print("No sessions found.")
        return 0

    for session in sessions:
        print(format_session(session))
        print()
    return 0


def cmd_session_get(args, memory: OpsMemory) -> int:
    """Get a specific session."""
    session = memory.get_session(args.id)
    if session:
        print(format_session(session))
        return 0
    else:
        print(f"Session not found: {args.id}", file=sys.stderr)
        return 1


def cmd_learning_create(args, memory: OpsMemory) -> int:
    """Create a new learning."""
    try:
        domain = LearningDomain(args.domain)
        learning = memory.create_learning(
            content=args.content,
            domain=domain,
            confidence=args.confidence,
        )
        print(f"Created learning with ID: {learning._id}")
        return 0
    except Exception as e:
        print(f"Error creating learning: {e}", file=sys.stderr)
        return 1


def cmd_learning_list(args, memory: OpsMemory) -> int:
    """List recent learnings."""
    if args.domain:
        domain = LearningDomain(args.domain)
        learnings = memory.get_learnings_by_domain(domain, limit=args.limit)
    else:
        learnings = memory.get_recent_learnings(limit=args.limit)

    if not learnings:
        print("No learnings found.")
        return 0

    for learning in learnings:
        print(format_learning(learning))
        print()
    return 0


def cmd_learning_search(args, memory: OpsMemory) -> int:
    """Search learnings."""
    learnings = memory.search_learnings(args.keyword)
    if not learnings:
        print(f"No learnings found matching '{args.keyword}'.")
        return 0

    for learning in learnings:
        print(format_learning(learning))
        print()
    return 0


def cmd_decision_create(args, memory: OpsMemory) -> int:
    """Create a new decision."""
    try:
        options = set(args.options) if args.options else None
        decision = memory.create_decision(
            description=args.description,
            context=args.context,
            rationale=args.rationale,
            options_considered=options,
        )
        print(f"Created decision with ID: {decision._id}")
        return 0
    except Exception as e:
        print(f"Error creating decision: {e}", file=sys.stderr)
        return 1


def cmd_decision_list(args, memory: OpsMemory) -> int:
    """List recent decisions."""
    decisions = memory.get_recent_decisions(limit=args.limit)
    if not decisions:
        print("No decisions found.")
        return 0

    for decision in decisions:
        print(format_decision(decision))
        print()
    return 0


def cmd_context(args, memory: OpsMemory) -> int:
    """Get context for session start."""
    context = memory.get_context_for_session_start()

    print("=== Context for New Session ===\n")

    if context["last_session"]:
        print("Last Session:")
        print(format_session(context["last_session"]))
        print()
    else:
        print("No previous sessions.\n")

    if context["open_loops"]:
        print("Open Loops from Recent Sessions:")
        for loop in context["open_loops"]:
            print(f"  - {loop}")
        print()

    if context["recent_learnings"]:
        print(f"Recent Learnings ({len(context['recent_learnings'])}):")
        for learning in context["recent_learnings"]:
            print(f"  - {learning.content[:80]}...")
        print()

    if context["recent_decisions"]:
        print(f"Recent Decisions ({len(context['recent_decisions'])}):")
        for decision in context["recent_decisions"]:
            print(f"  - {decision.description}")
        print()

    return 0


def cmd_stats(args, memory: OpsMemory) -> int:
    """Show database statistics."""
    counts = memory.get_all_documents_count()
    print("=== Ops Memory Statistics ===\n")
    for doc_type, count in counts.items():
        print(f"  {doc_type}: {count}")
    print(f"\n  Total: {sum(counts.values())}")
    return 0


def cmd_log_start(args, memory: OpsMemory) -> int:
    """Log session start (convenience command for ops)."""
    try:
        goals = set(args.goals) if args.goals else None
        session = memory.log_session_start(
            session_id=args.id,
            mode=args.mode,
            goals=goals,
        )
        print(f"Session started: {session.session_id}")
        print(f"Document ID: {session._id}")

        # Show context
        context = memory.get_context_for_session_start()
        if context["open_loops"]:
            print("\nOpen loops to address:")
            for loop in context["open_loops"]:
                print(f"  - {loop}")

        return 0
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1


def cmd_log_end(args, memory: OpsMemory) -> int:
    """Log session end (convenience command for ops)."""
    try:
        learnings = args.learnings if args.learnings else None
        open_loops = set(args.open_loops) if args.open_loops else None

        session = memory.log_session_end(
            session_id=args.id,
            summary=args.summary,
            learnings=learnings,
            open_loops=open_loops,
        )
        print(f"Session ended: {session.session_id}")
        if learnings:
            print(f"Learnings logged: {len(learnings)}")
        return 0
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1


def main():
    parser = argparse.ArgumentParser(
        description="Ops Memory CLI - manage biographical memory for ops mode"
    )
    subparsers = parser.add_subparsers(dest="command", help="Commands")

    # Schema commands
    schema_parser = subparsers.add_parser("schema", help="Schema management")
    schema_sub = schema_parser.add_subparsers(dest="schema_command")

    install_parser = schema_sub.add_parser("install", help="Install schema")
    install_parser.add_argument("--force", action="store_true", help="Force reinstall")

    schema_sub.add_parser("verify", help="Verify schema")

    # Session commands
    session_parser = subparsers.add_parser("session", help="Session management")
    session_sub = session_parser.add_subparsers(dest="session_command")

    create_parser = session_sub.add_parser("create", help="Create session")
    create_parser.add_argument("--id", required=True, help="Session ID")
    create_parser.add_argument("--mode", default="ops", help="Operating mode")
    create_parser.add_argument("--goals", nargs="*", help="Session goals")

    close_parser = session_sub.add_parser("close", help="Close session")
    close_parser.add_argument("--id", required=True, help="Session ID")
    close_parser.add_argument("--summary", required=True, help="Session summary")
    close_parser.add_argument("--open-loops", nargs="*", help="Open loops")

    list_parser = session_sub.add_parser("list", help="List sessions")
    list_parser.add_argument("--limit", type=int, default=10, help="Max results")

    get_parser = session_sub.add_parser("get", help="Get session by ID")
    get_parser.add_argument("--id", required=True, help="Session ID")

    # Learning commands
    learning_parser = subparsers.add_parser("learning", help="Learning management")
    learning_sub = learning_parser.add_subparsers(dest="learning_command")

    learn_create = learning_sub.add_parser("create", help="Create learning")
    learn_create.add_argument("--content", required=True, help="What was learned")
    learn_create.add_argument(
        "--domain",
        required=True,
        choices=["procedural", "semantic", "relational", "meta"],
        help="Learning domain",
    )
    learn_create.add_argument(
        "--confidence", type=float, default=0.8, help="Confidence (0-1)"
    )

    learn_list = learning_sub.add_parser("list", help="List learnings")
    learn_list.add_argument("--limit", type=int, default=20, help="Max results")
    learn_list.add_argument(
        "--domain",
        choices=["procedural", "semantic", "relational", "meta"],
        help="Filter by domain",
    )

    learn_search = learning_sub.add_parser("search", help="Search learnings")
    learn_search.add_argument("keyword", help="Search keyword")

    # Decision commands
    decision_parser = subparsers.add_parser("decision", help="Decision management")
    decision_sub = decision_parser.add_subparsers(dest="decision_command")

    dec_create = decision_sub.add_parser("create", help="Create decision")
    dec_create.add_argument("--description", required=True, help="What was decided")
    dec_create.add_argument("--context", required=True, help="Decision context")
    dec_create.add_argument("--rationale", required=True, help="Why this choice")
    dec_create.add_argument("--options", nargs="*", help="Options considered")

    dec_list = decision_sub.add_parser("list", help="List decisions")
    dec_list.add_argument("--limit", type=int, default=20, help="Max results")

    # Convenience commands
    subparsers.add_parser("context", help="Get context for session start")
    subparsers.add_parser("stats", help="Show database statistics")

    # Log commands (for ops integration)
    log_parser = subparsers.add_parser("log", help="Logging shortcuts for ops")
    log_sub = log_parser.add_subparsers(dest="log_command")

    log_start = log_sub.add_parser("start", help="Log session start")
    log_start.add_argument("--id", required=True, help="Session ID")
    log_start.add_argument("--mode", default="ops", help="Operating mode")
    log_start.add_argument("--goals", nargs="*", help="Session goals")

    log_end = log_sub.add_parser("end", help="Log session end")
    log_end.add_argument("--id", required=True, help="Session ID")
    log_end.add_argument("--summary", required=True, help="Session summary")
    log_end.add_argument("--learnings", nargs="*", help="Things learned")
    log_end.add_argument("--open-loops", nargs="*", help="Open loops")

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return 1

    # Connect to database
    try:
        memory = OpsMemory()
    except Exception as e:
        print(f"Error connecting to TerminusDB: {e}", file=sys.stderr)
        print("Is TerminusDB running? Try: sudo docker-compose up -d", file=sys.stderr)
        return 1

    # Route to command handler
    if args.command == "schema":
        if args.schema_command == "install":
            return cmd_schema_install(args, memory)
        elif args.schema_command == "verify":
            return cmd_schema_verify(args, memory)

    elif args.command == "session":
        if args.session_command == "create":
            return cmd_session_create(args, memory)
        elif args.session_command == "close":
            return cmd_session_close(args, memory)
        elif args.session_command == "list":
            return cmd_session_list(args, memory)
        elif args.session_command == "get":
            return cmd_session_get(args, memory)

    elif args.command == "learning":
        if args.learning_command == "create":
            return cmd_learning_create(args, memory)
        elif args.learning_command == "list":
            return cmd_learning_list(args, memory)
        elif args.learning_command == "search":
            return cmd_learning_search(args, memory)

    elif args.command == "decision":
        if args.decision_command == "create":
            return cmd_decision_create(args, memory)
        elif args.decision_command == "list":
            return cmd_decision_list(args, memory)

    elif args.command == "context":
        return cmd_context(args, memory)

    elif args.command == "stats":
        return cmd_stats(args, memory)

    elif args.command == "log":
        if args.log_command == "start":
            return cmd_log_start(args, memory)
        elif args.log_command == "end":
            return cmd_log_end(args, memory)

    parser.print_help()
    return 1


if __name__ == "__main__":
    sys.exit(main())
