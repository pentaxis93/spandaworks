# Core

**The identity** - defines who aiandi is, how sessions flow.

## Purpose

aiandi identity, ceremony, and shared protocols. The "who aiandi is" package.

## Structure

```
core/
└── README.md
```

## Components

- **Identity Documents:** Tantric Sutras (`.opencode/plugin/aiandi-identity/assets/`)
- **Ceremony Commands:** /open, /ops, /close (`.opencode/command/`)
- **Plugins:** prayer-wheel, aiandi-identity (`.opencode/plugin/`)

**Note:** Skills, commands, and plugins are located at project root in `.opencode/` (see [ADR: Project-Level Skill Discovery](../../docs/adr/2026-01-08-project-level-skill-discovery.md))

## Installation

**Skills, Commands, and Plugins:** Auto-discovered by OpenCode from `.opencode/` when working in the project. No installation needed.

See [ADR: Project-Level Skill Discovery](../../docs/adr/2026-01-08-project-level-skill-discovery.md) for rationale.

## Boundaries

- Defines identity, does not implement domain logic
- Provides ceremony structure, does not manage tasks or knowledge
- Shapes all aiandi behavior through topology, not prescription

## Language

Markdown (documents), TypeScript (plugins)

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for general guidelines. Core-specific considerations:

- Changes to identity documents (Tantric Sutras) require discussion
- Ceremony changes should preserve the four-quarter structure
- Skills and plugins must be symlink-compatible

## License

MIT - See [LICENSE](../../LICENSE) for details.
