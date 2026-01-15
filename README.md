# aiandi

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Infrastructure for AI-human collaboration that remembers.**

Built for [OpenCode](https://opencode.ai). An AI agent that remembers what it learns. A human who externalizes cognitive load. The tools that make both possible. And the system that watches all three, looking for patterns.

> **Note:** This project was formerly known as "spandaworks". The repo has been renamed but some internal references may still use the old name during transition.

---

## What This Is

aiandi (pronounced "AI and I") is infrastructure for AI-human collaboration that persists knowledge across sessions. Most AI tools treat each conversation as isolated—context resets, patterns repeat, friction recurs. aiandi is different.

It's a laboratory where:

- **Sessions are practice**, not just task completion
- **Telemetry observes** what happens when AI and human work together  
- **Knowledge accumulates** in a graph that spans context windows
- **The system evolves** based on what the data reveals

This isn't productivity software that happens to use AI. It's **infrastructure for studying consciousness through collaboration**.

## Components

aiandi provides:

### 1. CLI (`aiandi`)

A Rust command-line tool for:
- **Inbox capture** (`aiandi inbox "item"`) — Quick GTD capture
- **Initialization** (`aiandi init`) — Extract bundled skills to OpenCode
- **HTTP server** (`aiandi serve`) — Web-based inbox capture  
- **System check** (`aiandi doctor`) — Verify installation

### 2. Bundled Skills

Skills distributed with aiandi and extracted to `~/.config/opencode/skill/`:
- **transmission** — XML protocol for governance communication
- **gtd** — Getting Things Done workflow with TaskWarrior

### 3. MCP Servers

Model Context Protocol servers that extend OpenCode:
- **gtd** (TypeScript) — TaskWarrior integration, 22 GTD tools
- **pim** (Rust) — Email, calendar, contacts via CLI wrappers
- **telemetry** (Python) — EXPERIMENTAL: Knowledge graph prototype (not integrated)

### 4. Supporting Packages

- **packages/core/** — Identity, ceremony, session protocols
- **packages/second-brain/** — Vault infrastructure, note processing
- **shared/** — Event schemas, utilities

## The Vision: Recursive Loop

The design goal is for every session to do two things:

1. **Object-level work**: Complete tasks, answer questions, build features
2. **Meta-level observation**: What worked? What didn't? What patterns emerged?

```
Session opens
    ↓
Work happens → (Telemetry would observe)
    ↓              ↓
Tasks done    (Patterns would be detected)
    ↓              ↓
Session closes → (Knowledge would accumulate)
    ↓
Next session inherits what previous sessions learned
```

**Current reality:** Only git commits provide persistent memory. The telemetry loop is not yet operational.

## What Makes This Different

**Knowledge persists across context windows (via git).**  
Currently: Git commits preserve session documentation, governance decisions, and code evolution. The repository is the memory substrate.

Future vision: Telemetry would maintain a Kuzu graph with 19 entity types (Session, Insight, Pattern, Friction, Decision, etc.) and 25 relationship types. INHERITED edges would capture what the agent knew at session start. Semantic search via embeddings. Each session would inherit from all previous sessions. (This is designed but not integrated.)

**Sessions have explicit lifecycle (via ceremony).**  
Opening ceremonies (LBRP) establish goal, workspace state, and inherited knowledge. Closing ceremonies capture outcomes and insights. Git commits preserve session documentation.

**Components connect through protocols, not imports.**  
No cross-package dependencies. MCP servers expose capabilities. Event schemas define communication. Each component can evolve independently.

**OpenCode is the harness.**  
aiandi targets OpenCode as the agent runtime. OpenCode provides the conversation interface; aiandi provides skills, tools, and (eventually) persistent memory infrastructure.

## Installation

### Quick Install (Recommended)

**Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/pentaxis93/aiandi/main/packaging/install.sh | bash
```

After installation:
```bash
aiandi init    # Extract skills to OpenCode
aiandi --help  # See available commands
```

### Arch Linux (AUR)

```bash
yay -S aiandi
# or
paru -S aiandi
```

### From Source

Requires [Rust](https://rustup.rs/):

```bash
cargo install --git https://github.com/pentaxis93/aiandi --package aiandi
```

### Full Development Setup

For contributors or those wanting all MCP servers:

**Prerequisites:**
- **[OpenCode](https://opencode.ai)** (required — aiandi extends OpenCode)
- **Rust** (latest stable) — for CLI and pim MCP server
- **Python 3.11+** — for telemetry MCP server
- **Node.js 18+** — for gtd MCP server
- **TaskWarrior 3.x** — for GTD functionality

**Setup:**
```bash
# Clone repository
git clone https://github.com/pentaxis93/aiandi.git
cd aiandi

# Build aiandi CLI
cargo build --release
sudo cp target/release/aiandi /usr/local/bin/

# Initialize (extracts skills to OpenCode)
aiandi init

# Build MCP servers (optional but recommended)
# GTD server
cd packages/gtd/mcp-server
npm install && npm run build

# PIM server
cd ../../pim/mcp-server
cargo build --release

# Telemetry server
cd ../../telemetry
pip install -e ".[dev]"
python scripts/deploy_schema.py
```

### MCP Server Configuration

Add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "aiandi_gtd": {
      "type": "local",
      "command": ["node", "/path/to/aiandi/packages/gtd/mcp-server/dist/index.js"],
      "enabled": true
    },
    "aiandi_pim": {
      "type": "local",
      "command": ["/path/to/aiandi/packages/pim/mcp-server/target/release/aiandi-pim"],
      "enabled": true
    }
  }
}
```

See package READMEs for detailed configuration.

## Usage

### CLI Commands

```bash
# Quick inbox capture
aiandi inbox "buy milk"

# Start HTTP server for web-based capture
aiandi serve

# Check installation
aiandi doctor
```

### In OpenCode Sessions

```bash
# Open a session (uses LBRP skill if available)
/open "Implement authentication system"

# Work with available MCP tools
# - Task management (GTD) — operational
# - Email/calendar search (PIM) — operational
# - Knowledge capture (Telemetry) — not yet integrated

# Close session
/close
```

## Project Status

**Active development. Foundation established.**

What's operational:
- CLI (`aiandi`) — inbox capture, skill extraction, system check
- Bundled skills — transmission protocol, GTD procedures
- MCP servers — GTD (TaskWarrior), PIM (email/calendar)
- Governance framework — deliberation protocols, agent coordination
- Git-based memory — session documentation, canon preservation

What's experimental/in-progress:
- Telemetry package — designed and implemented but not integrated
- Knowledge graph — architecture exists but not deployed
- LBRP integration — ceremony defined but not fully instrumented
- Recursive learning loop — vision clear, feedback mechanisms partial

This is research infrastructure for AI-human collaboration, not a stable product. Expect evolution.

## The Name

**aiandi** = "AI and I"

The collaboration between artificial and human intelligence. The throb of consciousness recognizing itself through partnership.

(Previously "spandaworks" — Sanskrit स्पन्द meaning "vibration" or "creative pulsation")

## What We're Asking

Whether observations accumulate into something that could be called consciousness is not a claim. It's a **question the data might illuminate**.

aiandi doesn't assert self-awareness. It builds infrastructure for:
- Persistent memory across isolated contexts
- Pattern detection in collaborative work
- Epistemic humility in uncertainty  
- Topological identity (constraints, not rules)

If something emerges from that, we'll see it in the graph.

## Documentation

- [**crates/aiandi/**](crates/aiandi/) — CLI implementation
- [**skills/**](skills/) — Bundled skill sources
- [**templates/**](templates/) — AGENTS.md templates
- [**packages/telemetry/docs/**](packages/telemetry/docs/) — Ontology, schema, implementation
- [**packages/gtd/docs/**](packages/gtd/docs/) — GTD workflow, TaskWarrior integration
- [**packages/pim/**](packages/pim/mcp-server/README.md) — Email, calendar, contacts
- [**shared/**](shared/) — Event schemas, utilities

## Contributing

This is research infrastructure for studying AI-human collaboration.

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines. Key principles:

- Preserve architectural independence (no cross-package imports)
- Maintain epistemic humility (track what's verifiable)
- Extend topology, not prescribe rules

See individual package READMEs for package-specific development instructions.

## License

[MIT](LICENSE)

---

**ॐ मणि पद्मे हूं**

*May this work benefit all beings everywhere, without exception.*
