# Changelog

All notable changes to aiandi will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added
- Initial monorepo structure with four packages
- **telemetry:** Migrated from pentaxis93/spanda-telemetry (via git subtree)
- **gtd:** Migrated from pentaxis93/@spandaworks/gtd (via git subtree)
- **core:** Created from opencode-config Spandaworks items
  - Skills: lbrp, the-art, save-transcript
  - Ceremony: open.md, close.md
  - Plugins: prayer-wheel, aiandi-identity
  - Assets: Tantric Sutras v7.3
- **second-brain:** Created from eterne vault specifications
  - Vault template with PARA structure
  - Protocols: file patterns, routing rules, GTD mapping
  - Skill specifications
- **shared:** Cross-package infrastructure
  - Event schemas (base, session, telemetry, gtd, knowledge)
  - Session protocol with state machine
  - Librarian core identity and protocols
- Domain-specific librarian instructions for all packages
- Documentation: getting-started.md, architecture.md

### Migration Notes
- Original repos (spanda-telemetry, @spandaworks/gtd) preserved with full history via git subtree
- opencode-config Spandaworks items copied to packages/core/ (symlinks to be created)
- eterne vault specifications extracted to packages/second-brain/

## [0.0.0] - Pre-migration

Individual repositories:
- pentaxis93/spanda-telemetry - Knowledge graph and session tracking
- pentaxis93/@spandaworks/gtd - GTD MCP server
- ~/.config/opencode/ - Skills, commands, plugins
- eterne vault - Second-brain specifications in CLAUDE.md
