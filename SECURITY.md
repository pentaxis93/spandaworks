# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in Spandaworks, please report it responsibly:

1. **Do NOT** open a public issue
2. Email the maintainer directly or use GitHub's private vulnerability reporting
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Security Considerations by Package

### GTD (TaskWarrior MCP)

- Reads and writes to your local TaskWarrior database (`~/.task`)
- All task data remains local (no cloud sync by default)
- MCP server runs with your user permissions
- No network ports opened (stdio only)
- AI suggestions are advisory only (trust model: AI suggests, human approves)

See [packages/gtd/SECURITY.md](packages/gtd/SECURITY.md) for details.

### PIM (Email, Calendar, Contacts)

- **Email sending requires explicit `confirm: true`** parameter
- Credentials stored in `pass` (never in code or logs)
- CLI tools (notmuch, himalaya, khal, khard) run with user permissions
- No network access beyond what underlying CLI tools use
- Logs to stderr only (never stdout, to avoid credential leakage)

### Telemetry (Knowledge Graph)

- All data stored locally in Kuzu graph database
- No external network calls unless optional extraction features enabled
- Embeddings computed locally via sentence-transformers
- Optional OpenAI/Anthropic extraction requires explicit opt-in
- Session data includes metadata but not full conversation content

### Core (Identity & Ceremony)

- Markdown and TypeScript plugins (no credentials)
- Symlinked to OpenCode config (follows OpenCode security model)
- No network access, no data storage

### Second Brain (Vault Infrastructure)

- Operates on local Obsidian vault
- No credentials, no network access
- Processing workflows are read-only or explicitly modify vault

## General Security Practices

### MCP Protocol

- All MCP servers communicate via stdio (local only)
- No network ports opened
- Communication between MCP client and server stays local

### Data Locality

- All Spandaworks packages operate on local data by default
- No telemetry to external services (unless explicitly enabled for extraction features)
- Knowledge graph, tasks, PIM data all remain on your machine

### Credential Management

- Use `pass` for all credential storage (PIM package)
- Never hardcode credentials
- Environment variables acceptable for non-sensitive configuration

## Response Timeline

- Acknowledgment: 48 hours
- Initial assessment: 7 days
- Fix timeline: Depends on severity (critical issues prioritized)

## Disclosure Policy

Once a vulnerability is fixed, we will:

1. Release a patch version
2. Credit the reporter (unless anonymity requested)
3. Publish a security advisory with mitigation steps

---

**Thank you for helping keep Spandaworks secure.**
