# spanda-pim

MCP server for Personal Information Management - email, calendar, contacts.

Gives Spanda Works natural, fluid access to Robbie's PIM data by wrapping CLI tools:
- **Email**: notmuch (search/read), himalaya (send)
- **Calendar**: khal (list/create)
- **Contacts**: khard (search/get/create)

## Tools

### Calendar

| Tool | Description |
|------|-------------|
| `list_events` | List calendar events for a date range. Shows all calendars (personal, Meli, Dennis, holidays). |
| `create_event` | Create a new event on personal calendar. |

### Email

| Tool | Description |
|------|-------------|
| `search_emails` | Search emails using notmuch query syntax (from:, to:, subject:, date:, tag:). |
| `read_email` | Read the full content of an email thread. |
| `send_email` | Send an email. Requires explicit `confirm: true` for safety. |

### Contacts

| Tool | Description |
|------|-------------|
| `find_contact` | Search contacts by name, email, or phone. |
| `get_contact` | Get full details of a specific contact. |
| `create_contact` | Create a new contact. |

## Building

```bash
cd packages/pim/mcp-server
cargo build --release
```

Binary: `target/release/spanda-pim`

## OpenCode Configuration

Add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "spanda_pim": {
      "type": "local",
      "command": ["/path/to/spanda-works/packages/pim/mcp-server/target/release/spanda-pim"],
      "enabled": true
    }
  }
}
```

## Prerequisites

The following CLI tools must be installed and configured:

- **notmuch** - Email indexing and search
- **himalaya** - Email sending (at `~/.local/bin/himalaya`)
- **khal** - Calendar viewing and creation
- **khard** - Contact management

See the [PIM Skill documentation](../../core/skills/pim/SKILL.md) for setup instructions.

## Design Philosophy

> "I want the use of these tools to be as natural to Spanda as the use of my hand is to me."

Tools are named with simple verbs that match natural language:
- "What's on my calendar?" → `list_events`
- "Find Sarah's number" → `find_contact`
- "Search emails from John" → `search_emails`

## Safety

- **Email sending** requires explicit `confirm: true` parameter
- **Shared calendars** (Meli, Dennis) are read-only - event creation only works on personal calendar
- **No credentials in code** - uses `pass` for secrets

## License

MIT
