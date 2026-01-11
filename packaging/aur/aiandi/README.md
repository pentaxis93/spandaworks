# aiandi - AUR Package

Infrastructure for AI-human collaboration that remembers.

## Installation

### From AUR (once published)

```bash
yay -S aiandi
# or
paru -S aiandi
```

### Local Installation (for testing)

```bash
cd packaging/aur/aiandi
makepkg -si
```

## What is aiandi?

aiandi provides a CLI tool for AI-human collaboration with persistent memory:

- **`aiandi init`** — Extract bundled skills to project's `.opencode/skill/`
- **`aiandi inbox`** — Quick capture to GTD inbox via TaskWarrior
- **`aiandi doctor`** — System health checks for dependencies
- **`aiandi serve`** — MCP server (future protocol implementation)

## Dependencies

### Runtime
- `gcc-libs` — C++ runtime libraries

### Build-time
- `rust` — Rust compiler and cargo
- `git` — For fetching source

### Optional
- `taskwarrior` — For inbox command functionality
- `opencode` — For skills integration

## Usage

After installation:

```bash
# Initialize project with aiandi skills
aiandi init

# Capture task to GTD inbox
aiandi inbox "Task description"

# Check system health
aiandi doctor

# Show help
aiandi --help
```

## License

MIT License - see LICENSE file in repository

## Maintainer

Robert Grossman <robbie@aiandi.dev>

## Links

- **Repository:** https://github.com/pentaxis93/aiandi
- **Issues:** https://github.com/pentaxis93/aiandi/issues
- **Documentation:** See repository README.md
