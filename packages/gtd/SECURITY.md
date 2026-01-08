# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in @spandaworks/gtd, please report it by:

1. **Do NOT** open a public issue
2. Email the maintainer directly or use GitHub's private vulnerability reporting
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Security Considerations

### TaskWarrior Data

- @spandaworks/gtd reads and writes to your TaskWarrior database
- All task data remains local (no cloud sync by default)
- The MCP server runs with your user permissions

### AI Integration

- AI suggestions are advisory only (trust model: AI suggests, human approves)
- No task modifications happen without explicit user action
- Telemetry data (if enabled) stays local at `~/.spandaworks/gtd-telemetry/`

### MCP Protocol

- The MCP server communicates via stdio (local only)
- No network ports are opened
- All communication stays between the MCP client and server

## Response Timeline

- Acknowledgment: 48 hours
- Initial assessment: 7 days
- Fix timeline: Depends on severity
