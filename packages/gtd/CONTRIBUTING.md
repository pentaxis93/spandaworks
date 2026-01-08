# Contributing to spanda-gtd

Thank you for your interest in contributing to spanda-gtd!

## How to Contribute

### Reporting Bugs

1. Check existing [issues](https://github.com/pentaxis93/spanda-gtd/issues) first
2. Use the bug report template
3. Include TaskWarrior version (`task --version`)
4. Include steps to reproduce

### Suggesting Features

1. Open an issue using the feature request template
2. Describe the GTD workflow improvement
3. Explain how it fits the trust model (AI suggests, human approves)

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests: `cd mcp-server && npm test`
5. Run type check: `npm run typecheck`
6. Commit with clear message
7. Push and open a PR

### Development Setup

```bash
git clone https://github.com/pentaxis93/spanda-gtd.git
cd spanda-gtd/mcp-server
npm install
npm run build
npm run typecheck
```

### Code Style

- TypeScript for MCP server code
- Clear, descriptive function names
- Document GTD-specific logic

### Commit Messages

- Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`
- Reference issues when applicable

## Questions?

Open an issue with the question label.
