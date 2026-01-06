# MCP Tools Reference

Complete documentation for all 22 talos-gtd MCP tools.

---

## Basic CRUD Operations

### add_task

Add a new task with full GTD and habit support.

**Parameters:**
- `description` (required): Task description
- `due`: Due date (e.g., "friday", "2026-01-15")
- `priority`: H, M, or L
- `project`: Project name (e.g., "work.projectname")
- `tags`: Array of tags (e.g., `["in"]`, `["next", "@computer"]`)
- `scheduled`: When to start working on it
- `wait`: Hide until this date
- `until`: Task expires after this date
- `context`: Context tag without @ (e.g., "computer")
- `energy`: H, M, or L (brainpower required)
- `depends`: Array of UUIDs this task depends on
- `annotations`: Array of note strings
- `recur`: Recurrence pattern (e.g., "daily", "weekly")

**Example:**
```json
{
  "description": "Review quarterly goals",
  "project": "work.planning",
  "tags": ["next", "@computer"],
  "due": "friday",
  "energy": "H"
}
```

**GTD Context:** Use for Capture stage. For quick inbox capture, use `tags: ["in"]`.

---

### modify_task

Modify any task attributes by UUID.

**Parameters:**
- `uuid` (required): Task UUID
- `description`: New description
- `status`: pending, completed, deleted
- `due`, `priority`, `project`, `scheduled`, `wait`, `until`, `context`, `energy`, `recur`: Same as add_task
- `addTags`: Tags to add
- `removeTags`: Tags to remove
- `addDepends`: Dependencies to add
- `removeDepends`: Dependencies to remove

**Example:**
```json
{
  "uuid": "abc123",
  "removeTags": ["in"],
  "addTags": ["next", "@phone"],
  "project": "home.maintenance"
}
```

**GTD Context:** Use during Clarify/Organize to route inbox items.

---

### mark_task_done

Mark a task as completed.

**Parameters:**
- `uuid` (required): Task UUID

**GTD Context:** Use during Engage when completing work.

---

### delete_task

Delete a task permanently.

**Parameters:**
- `uuid` (required): Task UUID
- `confirm`: Skip confirmation (default: true)

**GTD Context:** Use during Clarify for items that aren't actionable and don't need to be kept.

---

### list_tasks

Get raw list of tasks with filters. Returns JSON array without analysis.

**Parameters:**
- `status`: pending, completed, deleted, all
- `project`: Filter by project
- `tags`: Array of required tags
- `due_before`, `due_after`: Date filters
- `limit`: Maximum tasks to return

**Note:** For actionable recommendations, use `get_next_actions` instead.

---

### get_task_details

Get detailed information for a specific task.

**Parameters:**
- `uuid` (required): Task UUID

---

### start_task

Mark a task as actively being worked on.

**Parameters:**
- `uuid` (required): Task UUID

---

### stop_task

Stop working on a task (remove active status).

**Parameters:**
- `uuid` (required): Task UUID

---

### add_annotation

Add a note to a task.

**Parameters:**
- `uuid` (required): Task UUID
- `annotation` (required): Note text

**Example:**
```json
{
  "uuid": "abc123",
  "annotation": "Discussed with Bob, he'll send specs by Friday"
}
```

---

### remove_annotation

Remove a note from a task.

**Parameters:**
- `uuid` (required): Task UUID
- `annotation` (required): Exact text of annotation to remove

---

## GTD Workflow Tools

### process_inbox

Get all inbox items with GTD clarification structure.

**Parameters:** None

**Returns:**
- List of `+in` tasks
- Clarification prompts for each item
- Decision structure (actionable? → project/context suggestions)
- Processing guidance

**GTD Context:** Core tool for Clarify stage. Use this instead of `list_tasks(tags: ["in"])` to get the full GTD workflow structure.

---

### get_next_actions

Get actionable next actions with AI-friendly analysis.

**Parameters:**
- `context`: Filter by context (e.g., "computer", "phone")
- `energy`: Filter by energy level (H, M, L)
- `time_available`: Minutes available

**Returns:**
- Enriched task list with actionability insights
- Context groupings
- AI recommendations
- Metadata for decision-making

**GTD Context:** Core tool for Engage stage. Answers "What should I do NOW?"

---

### get_waiting_for

Get tasks waiting on external factors (people, events).

**Parameters:**
- `group_by`: "blocker", "date", or "project"

**Returns:**
- Tasks with status:waiting or wait date set
- Grouped by specified dimension
- Follow-up recommendations

**GTD Context:** Essential for weekly review. Shows who you're waiting on and when to follow up.

---

### get_blocked_tasks

Get tasks blocked by other tasks (unmet dependencies).

**Parameters:**
- `include_waiting`: Also include external waiting items (default: false)

**Returns:**
- Tasks with unmet `depends` relationships
- Dependency chain analysis
- Recommendations for unblocking

**GTD Context:** Use to identify what's stuck due to internal task dependencies.

---

### get_project_status

Get health analysis for a specific project.

**Parameters:**
- `project` (required): Project name

**Returns:**
- Task counts (pending, completed, blocked)
- Next actions available
- Completion percentage
- Staleness warnings
- Recommendations

**GTD Context:** Use during project reviews to assess health.

---

### weekly_review

Generate comprehensive GTD weekly review in one call.

**Parameters:** None

**Returns:**
- Inbox count
- Completed tasks this week
- Stalled projects (no activity)
- Projects without next actions
- Waiting items needing follow-up
- Overdue tasks
- Habit completion statistics
- Broken streaks

**GTD Context:** Core tool for Reflect stage. Aggregates everything needed for weekly review ritual.

---

### get_someday_maybe

Get all someday/maybe items.

**Parameters:**
- `group_by`: Optional grouping

**Returns:**
- All `+sdm` tagged tasks
- Grouped as specified

**GTD Context:** Use during weekly review to scan deferred items for reactivation.

---

## Dependency Tools

### add_dependency

Create a dependency between two tasks.

**Parameters:**
- `task_uuid` (required): Task that will be blocked
- `depends_on_uuid` (required): Task that must complete first

**Example:** Task A depends on Task B → Task A can't start until B is done.

---

### remove_dependency

Remove a dependency between tasks.

**Parameters:**
- `task_uuid` (required): Task to unblock
- `depends_on_uuid` (required): Dependency to remove

---

## Batch Operations

### create_project_tree

Create a complete project with multiple tasks and dependencies.

**Parameters:**
- `project` (required): Project name
- `tasks` (required): Array of task definitions with optional `depends_on` references

**Example:**
```json
{
  "project": "home.renovation",
  "tasks": [
    {"id": "1", "description": "Get quotes from contractors"},
    {"id": "2", "description": "Compare quotes", "depends_on": ["1"]},
    {"id": "3", "description": "Select contractor", "depends_on": ["2"]}
  ]
}
```

**GTD Context:** Use when capturing a multi-step project to create full structure at once.

---

### batch_modify_tasks

Modify multiple tasks with the same changes.

**Parameters:**
- `uuids` (required): Array of task UUIDs
- `modifications`: Object with changes to apply to all

**Example:**
```json
{
  "uuids": ["abc123", "def456", "ghi789"],
  "modifications": {
    "project": "work.q1",
    "addTags": ["priority"]
  }
}
```

**GTD Context:** Efficient for bulk rescheduling, retagging, or project reassignment.

---

## Habit Tracking

### get_recurring_tasks

Get all recurring tasks with completion statistics.

**Parameters:**
- `group_by`: "frequency" or "project"

**Returns:**
- Template tasks (status:recurring)
- Completion rates
- Current streaks
- Broken streaks
- Frequency groupings

**GTD Context:** Essential for habit tracking and routine management.

---

## Tool Selection Guide

| I want to... | Use this tool |
|--------------|---------------|
| Capture a thought quickly | `add_task` with `tags: ["in"]` |
| Process my inbox | `process_inbox` |
| Find what to do next | `get_next_actions` |
| Check on delegated items | `get_waiting_for` |
| See what's blocked | `get_blocked_tasks` |
| Review a project | `get_project_status` |
| Do weekly review | `weekly_review` |
| Check someday list | `get_someday_maybe` |
| Track habits | `get_recurring_tasks` |
| Create a project | `create_project_tree` |
| Bulk update tasks | `batch_modify_tasks` |
