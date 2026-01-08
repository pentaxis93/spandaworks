# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-06

### Added

- Initial release of spanda-gtd
- 22 MCP tools for GTD workflow support:
  - Basic CRUD: `add_task`, `modify_task`, `mark_task_done`, `delete_task`, `list_tasks`, `get_task_details`, `start_task`, `stop_task`, `add_annotation`, `remove_annotation`
  - GTD Workflows: `process_inbox`, `get_next_actions`, `get_waiting_for`, `get_blocked_tasks`, `get_project_status`, `weekly_review`, `get_someday_maybe`
  - Dependencies: `add_dependency`, `remove_dependency`, `create_project_tree`, `batch_modify_tasks`
  - Habits: `get_recurring_tasks`
- TaskWarrior configuration template (`config/taskrc.template`)
- Shell aliases for GTD workflows (`config/aliases.sh`)
- Local telemetry integration (JSONL at `~/.spanda/gtd-telemetry/`)
- Comprehensive documentation:
  - Setup guide
  - GTD workflow guide
  - GTD cheat sheet (printable)
  - MCP tools reference
  - Trust protocol

### Attribution

- MCP server forked from [omniwaifu/taskwarrior-mcp](https://github.com/omniwaifu/taskwarrior-mcp) (MIT License)
- GTD patterns derived from [CS Syd's GTD with TaskWarrior series](https://cs-syd.eu/posts/2015-06-14-gtd-with-taskwarrior-part-1-intro)
- Configuration inspired by [hamlinux/taskwarrior-GTD](https://github.com/hamlinux/taskwarrior-GTD)

### Technical Notes

- Tag naming aligned with CS Syd patterns: `+in` (inbox), `+sdm` (someday/maybe)
- Requires TaskWarrior 3.x
- Requires Node.js 18+
