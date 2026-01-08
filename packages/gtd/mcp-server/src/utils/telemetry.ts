/**
 * Telemetry integration for @spandaworks/gtd
 * 
 * Design principle: GTD operations are primary, telemetry is observational.
 * If telemetry fails, GTD continues working.
 * 
 * Integration approach: Write JSONL events to shared location that
 * spandaworks-telemetry can read/import. Non-blocking, fire-and-forget.
 */

import { appendFileSync, existsSync, mkdirSync } from "fs";
import { homedir } from "os";
import { join } from "path";

// Telemetry event types
export type GTDEventType = 
  | "inbox_processed"
  | "weekly_review_completed"
  | "task_completed"
  | "gtd_friction"
  | "gtd_session";

export interface GTDTelemetryEvent {
  timestamp: string;
  event_type: GTDEventType;
  session_id?: string;
  data: Record<string, unknown>;
}

// Telemetry directory
const TELEMETRY_DIR = join(homedir(), ".spandaworks", "gtd-telemetry");
const TELEMETRY_FILE = join(TELEMETRY_DIR, "events.jsonl");

/**
 * Ensure telemetry directory exists
 */
function ensureDir(): boolean {
  try {
    if (!existsSync(TELEMETRY_DIR)) {
      mkdirSync(TELEMETRY_DIR, { recursive: true });
    }
    return true;
  } catch (error) {
    console.error("[telemetry] Failed to create directory:", error);
    return false;
  }
}

/**
 * Emit a telemetry event (non-blocking, fire-and-forget)
 * 
 * @param eventType - Type of GTD event
 * @param data - Event data
 * @param sessionId - Optional session ID
 */
export function emitGTDEvent(
  eventType: GTDEventType,
  data: Record<string, unknown>,
  sessionId?: string
): void {
  try {
    if (!ensureDir()) return;

    const event: GTDTelemetryEvent = {
      timestamp: new Date().toISOString(),
      event_type: eventType,
      session_id: sessionId,
      data,
    };

    // Append JSONL (one event per line)
    appendFileSync(TELEMETRY_FILE, JSON.stringify(event) + "\n");
    console.log(`[telemetry] Emitted ${eventType}`);
  } catch (error) {
    // Non-blocking: log but don't throw
    console.error("[telemetry] Failed to emit event:", error);
  }
}

/**
 * Log inbox processing event
 */
export function logInboxProcessed(
  inboxCount: number,
  sessionId?: string
): void {
  emitGTDEvent("inbox_processed", {
    inbox_count: inboxCount,
    action: "viewed",
  }, sessionId);
}

/**
 * Log weekly review completion
 */
export function logWeeklyReview(
  summary: {
    completedCount: number;
    inboxCount: number;
    overdueCount: number;
    projectCount: number;
    stalledProjects: string[];
    projectsWithoutNext: string[];
    habitAvgCompletion: number;
  },
  sessionId?: string
): void {
  emitGTDEvent("weekly_review_completed", {
    completed_this_week: summary.completedCount,
    inbox_items: summary.inboxCount,
    overdue_tasks: summary.overdueCount,
    active_projects: summary.projectCount,
    stalled_projects: summary.stalledProjects,
    projects_without_next_action: summary.projectsWithoutNext,
    habit_completion_avg: summary.habitAvgCompletion,
  }, sessionId);
}

/**
 * Log task completion
 */
export function logTaskCompleted(
  taskUuid: string,
  taskDescription: string,
  project?: string,
  sessionId?: string
): void {
  emitGTDEvent("task_completed", {
    task_uuid: taskUuid,
    description: taskDescription,
    project,
  }, sessionId);
}

/**
 * Log GTD friction (when system feels wrong)
 */
export function logGTDFriction(
  description: string,
  context: string,
  severity: "minor" | "moderate" | "severe" = "moderate",
  sessionId?: string
): void {
  emitGTDEvent("gtd_friction", {
    description,
    context,
    severity,
  }, sessionId);
}

/**
 * Get current session ID from environment or generate one
 */
export function getCurrentSessionId(): string | undefined {
  // Check environment variable set by LBRP ceremony
  return process.env.SPANDAWORKS_SESSION_ID;
}
