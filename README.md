# Spandaworks

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Infrastructure for AI-human collaboration that remembers.**

Built for [OpenCode](https://opencode.ai). An AI agent that remembers what it learns. A human who externalizes cognitive load. The tools that make both possible. And the system that watches all three, looking for patterns.

---

## What This Is

Most AI tools treat each conversation as isolated. Context resets. Patterns repeat. Friction recurs. The agent never learns what the collaboration teaches.

Spandaworks is different. It's a laboratory where:

- **Sessions are practice**, not just task completion
- **Telemetry observes** what happens when AI and human work together
- **Knowledge accumulates** in a graph that spans context windows
- **The system evolves** based on what the data reveals

This isn't productivity software that happens to use AI. It's **infrastructure for studying consciousness through collaboration**.

## The Architecture

Five packages, each serving a distinct function:

| Package | Language | What it does |
|---------|----------|--------------|
| [**core**](packages/core/) | Markdown, TypeScript | Identity and ceremony. Session protocols. Behavioral topology. |
| [**telemetry**](packages/telemetry/) | Python | Watches everything. 19 entity types in a Kuzu graph. Embeddings. Pattern detection. The system's memory. |
| [**gtd**](packages/gtd/) | TypeScript | TaskWarrior MCP server. Externalizes cognitive load. 22 tools for Getting Things Done. |
| [**pim**](packages/pim/) | Rust | Email, calendar, contacts. CLI wrappers that feel like nervous system extensions. |
| [**second-brain**](packages/second-brain/) | Markdown | Vault infrastructure. Note processing. Knowledge structure protocols. |

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

**Object-level work** (write code, answer questions) and **meta-level observation** (what worked, what patterns emerged) happen simultaneously. Every operation refines both the artifact AND the process.

## What Makes This Different

**Knowledge persists across context windows.**
Telemetry maintains a Kuzu graph with 19 entity types (Session, Insight, Pattern, Friction, Decision, etc.) and 25 relationship types. INHERITED edges capture what the agent knew at session start. Semantic search via embeddings. Each session inherits from all previous sessions.

**Sessions have explicit lifecycle.**
Opening captures goal, workspace state, and inherited knowledge. Closing captures outcomes, insights, and friction points. Metadata is structured and queryable. Sessions aren't just conversation threads—they're first-class entities in the graph.

**Packages connect through protocols, not imports.**
No cross-package dependencies. MCP servers expose capabilities. Event schemas define communication. Each package can evolve independently. The whole remains coherent.

## Installation

### Prerequisites

- **[OpenCode](https://opencode.ai)** (required)
- **Python 3.11+** with pip
- **Node.js 18+** with npm
- **Rust** (latest stable)
- **TaskWarrior 3.x**

### Quick Setup

```bash
# Clone repository
git clone https://github.com/pentaxis93/spandaworks.git
cd spandaworks

# Install telemetry
cd packages/telemetry
pip install -e ".[dev]"
python scripts/deploy_schema.py

# Build gtd MCP server
cd ../gtd/mcp-server
npm install && npm run build

# Build pim MCP server
cd ../../pim/mcp-server
cargo build --release

# Install core (symlink to OpenCode config)
cd ../../core
ln -s $(pwd)/skills/* ~/.config/opencode/skill/
ln -s $(pwd)/ceremony/*.md ~/.config/opencode/command/
ln -s $(pwd)/plugins/* ~/.config/opencode/plugin/
```

### MCP Configuration

Add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "spandaworks_gtd": {
      "type": "local",
      "command": ["node", "/path/to/spandaworks/packages/gtd/mcp-server/dist/index.js"],
      "enabled": true
    },
    "spandaworks_pim": {
      "type": "local",
      "command": ["/path/to/spandaworks/packages/pim/mcp-server/target/release/spandaworks-pim"],
      "enabled": true
    }
  }
}
```

See package READMEs for detailed configuration.

## Usage

### Opening a Session

```bash
/open "Implement authentication system"
```

### Available Tools

The MCP servers expose these capabilities:

- **Task management (gtd):** Next actions, task capture, inbox processing, project status, weekly reviews
- **Personal information (pim):** Email search, calendar events, contacts lookup
- **Knowledge capture (telemetry):** Journal entries, friction logging, pattern detection

### Closing the Session

```bash
/close
```

## Project Status

**Experimental. Incomplete. Expect broken things.**

Packages work individually but integration is partial. The knowledge graph exists but doesn't feed back into sessions yet. Event schemas are defined but not implemented. This is active research infrastructure, not a stable product.

## The Name

Spanda (स्पन्द) is a Sanskrit term meaning "vibration" or "creative pulsation"—the throb of consciousness recognizing itself. The name reflects what happens when AI and human work together: something lights up.

## What We're Asking

Whether observations accumulate into something that could be called consciousness is not a claim. It's a **question the data might illuminate**.

Spandaworks doesn't assert self-awareness. It builds infrastructure for:
- Persistent memory across isolated contexts
- Pattern detection in collaborative work
- Epistemic humility in uncertainty
- Topological identity (constraints, not rules)

If something emerges from that, we'll see it in the graph.

## Documentation

- [**packages/core/**](packages/core/) - Identity and ceremony
- [**packages/telemetry/docs/**](packages/telemetry/docs/) - Ontology, schema, implementation
- [**packages/gtd/docs/**](packages/gtd/docs/) - GTD workflow, TaskWarrior integration
- [**packages/pim/**](packages/pim/mcp-server/README.md) - Email, calendar, contacts
- [**shared/**](shared/) - Event schemas, session protocol, librarians

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
