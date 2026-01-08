"""Telemetry module - OpenTelemetry event capture."""

from spandaworks_telemetry.telemetry.events import emit_event
from spandaworks_telemetry.telemetry.sink import TelemetrySink, get_sink

__all__ = ["emit_event", "TelemetrySink", "get_sink"]
