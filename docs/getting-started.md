# Getting Started with Spanda Works

## What is Spanda Works?

Spanda Works is infrastructure for AI-human collaboration that remembers. It combines:

- **Telemetry:** Self-reflection infrastructure (knowledge graph)
- **GTD:** AI-augmented task management
- **Second-Brain:** Knowledge infrastructure (Obsidian vault)
- **Core:** Identity and ceremony

## Prerequisites

- Python 3.11+ (for telemetry)
- Node.js 18+ (for GTD MCP server)
- Obsidian (for second-brain vault)
- OpenCode (for AI collaboration)

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/pentaxis93/spanda-works.git ~/spanda-works
```

### 2. Set up telemetry

```bash
cd ~/spanda-works/packages/telemetry
python -m venv .venv
source .venv/bin/activate
pip install -e .
```

### 3. Set up GTD

```bash
cd ~/spanda-works/packages/gtd/mcp-server
npm install
npm run build
```

Configure in your MCP settings (e.g., `~/.config/opencode/mcp.json`):

```json
{
  "mcpServers": {
    "spanda_gtd": {
      "command": "node",
      "args": ["~/spanda-works/packages/gtd/mcp-server/dist/index.js"]
    }
  }
}
```

### 4. Set up core (OpenCode integration)

Symlink Spanda Works components to OpenCode config:

```bash
# Skills
ln -sf ~/spanda-works/packages/core/skills/* ~/.config/opencode/skill/

# Ceremony commands
ln -sf ~/spanda-works/packages/core/ceremony/*.md ~/.config/opencode/command/

# Other commands
ln -sf ~/spanda-works/packages/core/commands/*.md ~/.config/opencode/command/

# Plugins
ln -sf ~/spanda-works/packages/core/plugins/* ~/.config/opencode/plugin/
```

### 5. Set up vault

Copy vault template to your Obsidian vault location:

```bash
cp -r ~/spanda-works/packages/second-brain/vault-template/* ~/your-vault/
```

Or reference the protocols directly in your vault's CLAUDE.md.

## Usage

### Opening a Session

Use the `/open` command in OpenCode to begin a practice session with LBRP ceremony.

### GTD Operations

- `/gtd:process-gtd` - Process inbox
- `/gtd:plan-gtd` - Time-aware planning

### Knowledge Operations

- `/second-brain:dump` - Capture thoughts
- `/second-brain:fuse` - Consolidate knowledge
- `/second-brain:integrate` - Integrate candidates

### Closing a Session

Use the `/close` command to seal the session with harvest and dedication.

## Next Steps

- Read `architecture.md` for system design
- See `SPANDA-WORKS-SPEC.md` for full specification
- Explore individual package READMEs
