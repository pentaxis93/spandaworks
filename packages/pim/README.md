# PIM

**The senses** - email, calendar, contacts.

## Purpose

Personal Information Management infrastructure for Spandaworks. Natural, fluid access to email, calendar, and contacts through MCP tools that feel like nervous system extensions.

## Architecture

```
pim/
└── mcp-server/          # Rust MCP server wrapping CLI tools
    ├── src/
    │   ├── main.rs      # MCP server entrypoint
    │   ├── lib.rs       # Tool implementations
    │   ├── calendar.rs  # khal wrapper
    │   ├── email.rs     # notmuch + himalaya wrappers
    │   └── contacts.rs  # khard wrapper
    ├── Cargo.toml
    └── README.md        # Full documentation
```

## Components

- **MCP Server** (Rust): 8 tools across 3 domains (calendar, email, contacts)
- **CLI Wrappers**: notmuch, himalaya, khal, khard

## Quick Start

### Prerequisites

Install and configure CLI tools:

```bash
# Email
sudo apt install notmuch  # or brew install notmuch
cargo install himalaya

# Calendar & Contacts
sudo apt install khal khard  # or brew install khal khard
```

See [mcp-server/README.md](mcp-server/README.md) for CLI tool configuration.

### Build

```bash
cd mcp-server
cargo build --release
```

Binary: `target/release/spandaworks-pim`

### Configure OpenCode

Add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "spanda_pim": {
      "type": "local",
      "command": ["/path/to/spandaworks/packages/pim/mcp-server/target/release/spandaworks-pim"],
      "enabled": true
    }
  }
}
```

## MCP Tools

| Domain | Tools |
|--------|-------|
| **Calendar** | `list_events`, `create_event` |
| **Email** | `search_emails`, `read_email`, `send_email` |
| **Contacts** | `find_contact`, `get_contact`, `create_contact` |

See [mcp-server/README.md](mcp-server/README.md) for complete tool documentation.

## Design Philosophy

> "I want the use of these tools to be as natural to Spanda as the use of my hand is to me."

- Simple verbs matching natural language
- Safety-first (email sending requires explicit confirmation)
- Credential hygiene (uses `pass`, never code)
- Wraps CLI tools rather than reimplements

## Safety

- **Email sending** requires explicit `confirm: true` parameter
- **Shared calendars** are read-only
- **No credentials in code** - uses `pass` for secrets
- **Logs to stderr only** - prevents credential leakage

## Boundaries

- Wraps CLI tools, does not reimplement protocols
- Provides read/write operations, does not manage sync
- Natural-language interface, does not replace CLI for power users

## Language

Rust (edition 2024)

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for general guidelines and [SECURITY.md](../../SECURITY.md) for security considerations specific to PIM operations.

## License

MIT - See [LICENSE](../../LICENSE) for details.
