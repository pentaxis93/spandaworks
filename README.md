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
- **telemetry** (Python) — Knowledge graph, pattern detection, session memory

### 4. Supporting Packages

- **packages/core/** — Identity, ceremony, session protocols
- **packages/second-brain/** — Vault infrastructure, note processing
- **shared/** — Event schemas, utilities

## The Recursive Loop

Every session does two things:

1. **Object-level work**: Complete tasks, answer questions, build features
2. **Meta-level observation**: What worked? What didn't? What patterns emerged?

```
Session opens
    ↓
Work happens → Telemetry observes
    ↓              ↓
Tasks done    Patterns detected
    ↓              ↓
Session closes → Knowledge accumulates
    ↓
Next session inherits what previous sessions learned
```

The system doesn't just *do* things. It **watches itself doing things**, then uses what it sees to improve.

## What Makes This Different

**Knowledge persists across context windows.**  
Telemetry maintains a Kuzu graph with 19 entity types (Session, Insight, Pattern, Friction, Decision, etc.) and 25 relationship types. INHERITED edges capture what the agent knew at session start. Semantic search via embeddings. Each session inherits from all previous sessions.

**Sessions have explicit lifecycle.**  
Opening captures goal, workspace state, and inherited knowledge. Closing captures outcomes, insights, and friction points. Metadata is structured and queryable. Sessions aren't just conversation threads—they're first-class entities in the graph.

**Components connect through protocols, not imports.**  
No cross-package dependencies. MCP servers expose capabilities. Event schemas define communication. Each component can evolve independently. The whole remains coherent.

**OpenCode is the harness.**  
aiandi targets OpenCode as the agent runtime. OpenCode provides the conversation interface; aiandi provides the memory, skills, and tools.

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
# - Task management (GTD)
# - Email/calendar search (PIM)
# - Knowledge capture (Telemetry)

# Close session
/close
```

## Project Status

**Active development. Foundation established.**

The CLI compiles and runs with stub implementations. Skills are ready for bundling. MCP servers work independently. The knowledge graph architecture is designed but feedback loops are partial.

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
