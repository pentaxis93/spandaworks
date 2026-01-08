"""MCP tools module - Model Context Protocol tools for Spanda Works."""

from spanda_telemetry.mcp.extract import journal_extract
from spanda_telemetry.mcp.friction import friction_log
from spanda_telemetry.mcp.journal import journal_query, journal_write
from spanda_telemetry.mcp.patterns import pattern_check
from spanda_telemetry.mcp.query import graph_query
from spanda_telemetry.mcp.reflect import get_recent_reflections, reflect
from spanda_telemetry.mcp.session import session_close, session_open

__all__ = [
    # Session lifecycle
    "session_open",
    "session_close",
    # Journal / Knowledge capture
    "journal_write",
    "journal_query",
    "journal_extract",
    # Friction tracking
    "friction_log",
    # Meta-cognitive
    "reflect",
    "get_recent_reflections",
    # Graph queries
    "graph_query",
    # Pattern detection
    "pattern_check",
]
