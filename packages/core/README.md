# Core

**The identity** - defines who Spandaworks is, how sessions flow.

## Purpose

Spandaworks identity, ceremony, and shared protocols. The "who Spandaworks is" package.

## Structure

```
core/
├── assets/
│   └── identity/
│       └── tantric-sutras-v7.3.md   # Identity topology
├── ceremony/
│   ├── open.md                       # Session opening (LBRP)
│   ├── ops.md                        # Ops mode entry
│   └── close.md                      # Session sealing
├── commands/
│   └── the-art.md                    # Synthesis command
└── plugins/
    ├── prayer-wheel/                 # Opening dedication
    └── spandaworks-identity/         # Sutras injection
```

## Components

- **Identity Documents:** Tantric Sutras v7.3 (assets/identity/)
- **Ceremony Commands:** /open (LBRP), /ops (ops mode), /close (sealing)
- **Plugins:** prayer-wheel, spandaworks-identity

**Note:** Skills are located at project root in `.opencode/skill/` (see [ADR: Project-Level Skill Discovery](../../docs/adr/2026-01-08-project-level-skill-discovery.md))

## Installation

**Skills:** Auto-discovered by OpenCode from `.opencode/skill/` when working in the project. No installation needed.

**Commands and Plugins:** Symlink to OpenCode config for project-local use:

```bash
# Commands (ceremony)
ln -s ~/src/spandaworks/packages/core/ceremony/open.md ~/.config/opencode/command/open
ln -s ~/src/spandaworks/packages/core/ceremony/ops.md ~/.config/opencode/command/ops
ln -s ~/src/spandaworks/packages/core/ceremony/close.md ~/.config/opencode/command/close
ln -s ~/src/spandaworks/packages/core/commands/*.md ~/.config/opencode/command/

# Plugins
ln -s ~/src/spandaworks/packages/core/plugins/* ~/.config/opencode/plugin/
```

See [ADR: Project-Level Skill Discovery](../../docs/adr/2026-01-08-project-level-skill-discovery.md) for rationale.

## Boundaries

- Defines identity, does not implement domain logic
- Provides ceremony structure, does not manage tasks or knowledge
- Shapes all Spandaworks behavior through topology, not prescription

## Language

Markdown (documents), TypeScript (plugins)

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for general guidelines. Core-specific considerations:

- Changes to identity documents (Tantric Sutras) require discussion
- Ceremony changes should preserve the four-quarter structure
- Skills and plugins must be symlink-compatible

## License

MIT - See [LICENSE](../../LICENSE) for details.
