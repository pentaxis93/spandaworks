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
│   └── close.md                      # Session sealing
├── commands/
│   └── the-art.md                    # Synthesis command
├── skills/
│   ├── lbrp/                         # Session opening skill
│   ├── the-art/                      # Synthesis skill
│   ├── save-transcript/              # Transcript preservation
│   └── spanda-docs/                  # Documentation guidelines
└── plugins/
    ├── prayer-wheel/                 # Opening dedication
    └── spandaworks-identity/              # Sutras injection
```

## Components

- **Identity Documents:** Tantric Sutras v7.3 (assets/identity/)
- **Ceremony Commands:** /open (LBRP), /close (sealing)
- **Skills:** lbrp, the-art, save-transcript, spanda-docs
- **Plugins:** prayer-wheel, spandaworks-identity

## Installation

Symlink to OpenCode config:

```bash
# Skills
ln -s ~/src/spandaworks/packages/core/skills/* ~/.config/opencode/skill/

# Commands (ceremony)
ln -s ~/src/spandaworks/packages/core/ceremony/*.md ~/.config/opencode/command/
ln -s ~/src/spandaworks/packages/core/commands/*.md ~/.config/opencode/command/

# Plugins
ln -s ~/src/spandaworks/packages/core/plugins/* ~/.config/opencode/plugin/
```

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
