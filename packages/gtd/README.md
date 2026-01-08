# @spandaworks/gtd

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![TaskWarrior](https://img.shields.io/badge/TaskWarrior-3.x-green.svg)](https://taskwarrior.org/)
[![MCP](https://img.shields.io/badge/MCP-Compatible-purple.svg)](https://modelcontextprotocol.io/)

AI-augmented GTD (Getting Things Done) system using TaskWarrior as the core engine.

## Overview

@spandaworks/gtd extends TaskWarrior with 22 MCP tools for GTD workflows, enabling AI assistants to help with task management while maintaining a strict trust model: **AI suggests, human approves**.

### Key Features

- **22 MCP Tools** for complete GTD workflow support
- **Inbox Processing** with clarification prompts and decision structure
- **Project Health Monitoring** with staleness detection and next-action tracking
- **Weekly Review Aggregation** in a single tool call
- **Context-Aware Next Actions** filtered by location, energy, and time
- **Dependency Management** for complex project workflows
- **Habit/Recurring Task Tracking** with streak analysis
- **Local Telemetry** for usage analytics (optional, stays on your machine)

### Trust Model

All AI interactions are advisory. The MCP tools read TaskWarrior data and provide structured responses, but modifications require explicit user confirmation through the AI interface.

## Quick Start

### Prerequisites

- **TaskWarrior 3.x** ([installation guide](https://taskwarrior.org/download/))
- **Node.js 18+**
- **MCP-compatible client** (Claude Desktop, OpenCode, or similar)

### Installation (5 steps)

```bash
# 1. Clone repository
git clone https://github.com/pentaxis93/@spandaworks/gtd.git
cd @spandaworks/gtd

# 2. Build MCP server
cd mcp-server && npm install && npm run build && cd ..

# 3. Apply TaskWarrior configuration
cat config/taskrc.template >> ~/.taskrc

# 4. Add shell aliases
echo 'source ~/src/@spandaworks/gtd/config/aliases.sh' >> ~/.zshrc  # or ~/.bashrc

# 5. Configure your MCP client (see below)
```

### Configure MCP Client

**Claude Desktop** (`~/.config/claude/claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "@spandaworks/gtd": {
      "command": "node",
      "args": ["/path/to/@spandaworks/gtd/mcp-server/dist/index.js"]
    }
  }
}
```

**OpenCode** (`~/.config/opencode/opencode.json`):
```json
{
  "mcp": {
    "servers": {
      "spanda_gtd": {
        "type": "local",
        "command": ["node", "/path/to/@spandaworks/gtd/mcp-server/dist/index.js"]
      }
    }
  }
}
```

### Verify Installation

```bash
source ~/.zshrc  # reload shell
in "Test inbox item"  # capture to inbox
ti  # view inbox
task 1 done  # complete test task
```

## GTD Tags

| Tag | Purpose |
|-----|---------|
| `+in` | Inbox item awaiting processing |
| `+next` | Next action (ready to do) |
| `+sdm` | Someday/Maybe |
| `+waiting` | Waiting for someone/something |
| `+tickle` | Tickler item (used with `wait:`) |

## Context Tags

| Tag | Context |
|-----|---------|
| `+@computer` | At a computer |
| `+@online` | Internet available |
| `+@phone` | Can make calls |
| `+@home` | At home |
| `+@work` | At office |
| `+@errands` | Out running errands |
| `+@anywhere` | Location-independent |
| `+@focus` | Deep work, no interruptions |

## MCP Tools Overview

### Basic Operations
`add_task` · `modify_task` · `mark_task_done` · `delete_task` · `list_tasks` · `get_task_details` · `start_task` · `stop_task` · `add_annotation` · `remove_annotation`

### GTD Workflows
`process_inbox` · `get_next_actions` · `get_waiting_for` · `get_blocked_tasks` · `get_project_status` · `weekly_review` · `get_someday_maybe`

### Dependencies & Batch
`add_dependency` · `remove_dependency` · `create_project_tree` · `batch_modify_tasks`

### Habits
`get_recurring_tasks`

See [docs/mcp-tools.md](docs/mcp-tools.md) for complete tool documentation.

## Shell Aliases

| Alias | Purpose |
|-------|---------|
| `in` | Quick capture to inbox |
| `ti` | View inbox |
| `tn` | View next actions |
| `tw` | View waiting-for items |
| `ts` | View someday/maybe |
| `tnna` | Projects missing next action |
| `tcw/tch/tcf/tca/tcn` | Context switching |
| `process` | Process inbox item to next action |
| `defer` | Move to someday/maybe |
| `delegate` | Mark as waiting-for |

See [docs/gtd-cheatsheet.md](docs/gtd-cheatsheet.md) for the complete quick reference.

## Documentation

- [Setup Guide](docs/setup.md) - Detailed installation instructions
- [GTD Workflow](docs/workflow.md) - How to use @spandaworks/gtd for GTD
- [GTD Cheat Sheet](docs/gtd-cheatsheet.md) - Quick reference (printable)
- [MCP Tools Reference](docs/mcp-tools.md) - Complete tool documentation
- [Trust Protocol](docs/trust-protocol.md) - AI trust building process

## Architecture

```
Claude / LLM
    │
    │ MCP Protocol
    ▼
@spandaworks/gtd MCP Server (22 tools)
    │
    │ CLI / JSON
    ▼
TaskWarrior 3.x
    │
    ▼
~/.task (local database)
```

## Attribution

This project builds on excellent prior work:

- **MCP Server**: Forked from [omniwaifu/taskwarrior-mcp](https://github.com/omniwaifu/taskwarrior-mcp) (MIT License)
- **GTD Patterns**: Derived from [CS Syd's GTD with TaskWarrior series](https://cs-syd.eu/posts/2015-06-14-gtd-with-taskwarrior-part-1-intro)
- **Configuration**: Inspired by [hamlinux/taskwarrior-GTD](https://github.com/hamlinux/taskwarrior-GTD)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT - See [LICENSE](LICENSE) for details.
