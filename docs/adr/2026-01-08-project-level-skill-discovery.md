# ADR: Project-Level Configuration (Skills, Commands, Plugins)

**Date:** 2026-01-08  
**Status:** Accepted  
**Context:** Installation and discovery of agent capabilities

---

## Context

OpenCode supports custom agent skills, commands, and plugins through a discovery mechanism. Initially, Spandaworks stored these components in various subdirectories of `packages/core/`:
- Skills: `packages/core/skills/`
- Commands: `packages/core/ceremony/` and `packages/core/commands/`
- Plugins: `packages/core/plugins/`

Users were required to manually symlink these files to global configuration directories (`~/.config/opencode/skill/`, `~/.config/opencode/command/`, `~/.config/opencode/plugin/`).

This approach had several problems:
1. **Manual setup required** - New users had to create dozens of symlinks manually
2. **Global pollution** - Capabilities installed globally rather than project-scoped
3. **Not collaborative** - Features wouldn't work when others cloned the repository
4. **Non-canonical** - Violated OpenCode's documented best practices for project-level configuration

## Decision

**Move all Spandaworks skills, commands, and plugins to the top-level `.opencode/` directory** and rely on OpenCode's native project-level discovery mechanism.

### OpenCode Discovery (Canonical)

Per the [OpenCode documentation](https://opencode.ai/docs/), OpenCode recursively searches up from the current directory for an `.opencode` folder containing:
- `skill/` - Custom agent skills
- `command/` - Custom slash commands
- `plugin/` - Plugins and extensions

### Our Structure

```
spandaworks/
├── .opencode/
│   ├── command/
│   │   ├── close.md
│   │   ├── open.md
│   │   ├── ops.md
│   │   ├── stand-down.md
│   │   └── the-art.md
│   ├── plugin/
│   │   ├── prayer-wheel/
│   │   └── spandaworks-identity/
│   └── skill/
│       ├── lbrp/
│       │   └── SKILL.md
│       ├── perspective-review/
│       │   ├── SKILL.md
│       │   ├── personas/
│       │   ├── templates/
│       │   └── examples/
│       ├── save-transcript/
│       │   └── SKILL.md
│       ├── spandaworks-docs/
│       │   └── SKILL.md
│       ├── taskwarrior-gtd/
│       │   └── SKILL.md
│       └── the-art/
│           └── SKILL.md
└── packages/
    └── core/
        └── assets/
            └── identity/
```

**Note:** `packages/core/skills/`, `packages/core/ceremony/`, `packages/core/commands/`, and `packages/core/plugins/` no longer exist.

## Implementation

**Migration steps:**
1. Created `.opencode/skill/`, `.opencode/command/`, and `.opencode/plugin/` directories
2. Moved all skills from `packages/core/skills/*` to `.opencode/skill/*`
3. Moved ceremony commands from `packages/core/ceremony/*` to `.opencode/command/*`
4. Moved synthesis command from `packages/core/commands/*` to `.opencode/command/*`
5. Moved all plugins from `packages/core/plugins/*` to `.opencode/plugin/*`
6. Removed the original source directories in `packages/core/`
7. Updated documentation references

**No symlinks required.** OpenCode auto-discovers all capabilities from `.opencode/` when working in the project.

## Consequences

### Positive

✓ **Zero manual setup** - Everything works immediately when cloning the repo  
✓ **Project-scoped** - No global config pollution  
✓ **Unified** - All agent capabilities live in one standard location  
✓ **Collaborative** - Works for all contributors automatically  
✓ **Canonical** - Follows OpenCode best practices  

### Neutral

- Capabilities only available when working within the Spandaworks project
- For global availability, users can manually symlink from `.opencode/` subdirectories to `~/.config/opencode/` if desired

### Maintenance

**For new capabilities:**
Simply add them to the appropriate directory in `.opencode/`:
- New skill: `.opencode/skill/my-skill/SKILL.md`
- New command: `.opencode/command/my-command.md`
- New plugin: `.opencode/plugin/my-plugin/`

**Verification:**
OpenCode automatically discovers changes on startup. No restart or reload required.

## References

- [OpenCode Skill Documentation](https://opencode.ai/docs/skills/)
- [OpenCode Configuration](https://opencode.ai/docs/config/)

---

**Replaced approach:**  
~~`packages/core/*` + manual global symlinks~~

**New approach:**  
`.opencode/` with native OpenCode discovery for skills, commands, and plugins
