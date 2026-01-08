"""Telemetry module - OpenTelemetry event capture."""

from spanda_telemetry.telemetry.events import emit_event
from spanda_telemetry.telemetry.sink import TelemetrySink, get_sink

__all__ = ["emit_event", "TelemetrySink", "get_sink"]
