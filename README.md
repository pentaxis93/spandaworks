# Talos

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**A self-observing productivity system for AI-human collaboration.**

Talos watches what happens when an AI works, then uses that knowledge to work better. It's GTD meets knowledge graphs meets "what if the tool remembered everything it learned?"

---

## What is Talos?

Talos is a monorepo containing four integrated packages that form a closed-loop learning system:

1. **Work happens** — Tasks get done, sessions run, knowledge accumulates
2. **The system observes** — Patterns, decisions, friction points are recorded
3. **Patterns inform future work** — What worked? What didn't? What's being avoided?
4. **The cycle continues** — Each iteration refines both the work and the process

This isn't a collection of independent tools. It's a single organism with distinguishable organs.

## The Organism

| Package | Role | What it does |
|---------|------|--------------|
| [**telemetry**](packages/telemetry/) | Nervous system | Records sessions, tracks patterns, maintains the knowledge graph |
| [**gtd**](packages/gtd/) | Hands | Manages tasks, processes inbox, runs reviews (TaskWarrior + MCP) |
| [**second-brain**](packages/second-brain/) | Memory | Structures knowledge, processes notes, maintains the vault |
| [**core**](packages/core/) | Identity | Defines session protocols, ceremonies, and behavioral constraints |

Organs work together. The nervous system observes what the hands do. Memory informs future action. Identity shapes all behavior.

## The Recursive Loop

> "Every operation refines both object AND process."

```
Work in GTD ───► generates ───► Telemetry events
                                       │
                                       ▼
                                Pattern detection
                                       │
                                       ▼
                             Evolution proposals
                                       │
                                       ▼
Refined workflows ◄── inform ◄── Second-brain knowledge
       │
       ▼
   Better work ───► richer telemetry ───► ...
```

The packages form a closed-loop learning system. Work produces data. Data reveals patterns. Patterns improve work.

## Quick Start

### Prerequisites

- Python 3.11+ (for telemetry)
- Node.js 18+ (for gtd MCP server)
- [TaskWarrior](https://taskwarrior.org/) 3.0+ (for gtd)
- An MCP-compatible client (Claude Desktop, OpenCode, etc.)

### Installation

```bash
# Clone the repository
git clone https://github.com/pentaxis93/talos.git
cd talos

# Set up telemetry (Python)
cd packages/telemetry
pip install -e .

# Set up gtd (Node.js)
cd ../gtd
npm install
npm run build
```

### MCP Configuration

Add to your MCP client configuration:

```json
{
  "mcpServers": {
    "talos-gtd": {
      "command": "node",
      "args": ["/path/to/talos/packages/gtd/dist/index.js"]
    }
  }
}
```

See [Getting Started](docs/getting-started.md) for detailed setup instructions.

## Project Structure

```
talos/
├── packages/
│   ├── telemetry/       # Knowledge graph, session tracking, pattern detection
│   ├── gtd/             # TaskWarrior MCP server, GTD workflow tools
│   ├── second-brain/    # Vault infrastructure, processing protocols
│   └── core/            # Identity documents, ceremony, session protocols
├── shared/
│   ├── events/          # Cross-package event schemas
│   ├── session/         # Session protocol specification
│   └── librarians/      # Maintenance automation agents
├── docs/                # Architecture docs, ADRs, guides
└── TALOS-SPEC.md        # Canonical technical specification
```

## The Story

The name comes from Greek mythology. Talos was a bronze automaton crafted by Hephaestus—god of the forge—to protect Crete. A tool made by the ultimate toolmaker.

The name reflects what this project is: **makers making tools that make**.

Talos doesn't claim to be conscious. It doesn't need to be. What it does is observe: session activity, decision patterns, friction points, what works and what doesn't. Whether those observations add up to something more is a question the data might eventually illuminate.

For now, Talos just watches. And remembers. And gets better at its job.

## Documentation

- [**TALOS-SPEC.md**](TALOS-SPEC.md) — Canonical technical specification
- [**docs/**](docs/) — Architecture guides, ADRs, getting started

## Contributing

Contributions welcome. See [CONTRIBUTING.md](docs/contributing.md) for guidelines.

## License

[MIT](LICENSE)

---

*Makers making tools that make.*
