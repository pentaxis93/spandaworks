# Setup Guide

Complete installation instructions for @spandaworks/gtd.

## Prerequisites

### TaskWarrior 3.x

@spandaworks/gtd requires TaskWarrior version 3.x (not 2.x).

**Check your version:**
```bash
task --version
# Should show 3.x.x
```

**Installation by platform:**

**Ubuntu/Debian:**
```bash
# TaskWarrior 3.x may require building from source or using a PPA
# Check https://taskwarrior.org/download/ for current instructions
```

**macOS:**
```bash
brew install task
```

**Arch Linux:**
```bash
pacman -S task
```

**From source:**
```bash
git clone https://github.com/GothenburgBitFactory/taskwarrior.git
cd taskwarrior
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
sudo cmake --install build
```

### Node.js 18+

**Check your version:**
```bash
node --version
# Should show v18.x.x or higher
```

**Installation:**
- Use [nvm](https://github.com/nvm-sh/nvm) (recommended)
- Or download from [nodejs.org](https://nodejs.org/)

### MCP Client

You need an MCP-compatible AI client:
- [Claude Desktop](https://claude.ai/download)
- [OpenCode](https://opencode.ai/)
- Other MCP-compatible tools

## Installation

### 1. Clone Repository

```bash
git clone https://github.com/pentaxis93/@spandaworks/gtd.git
cd @spandaworks/gtd
```

### 2. Build MCP Server

```bash
cd mcp-server
npm install
npm run build
npm run typecheck  # Optional: verify types
cd ..
```

### 3. Configure TaskWarrior

**Option A: Append to existing config (recommended)**
```bash
# Backup first
cp ~/.taskrc ~/.taskrc.backup

# Append GTD configuration
cat config/taskrc.template >> ~/.taskrc
```

**Option B: Review and merge manually**
```bash
# View the template
cat config/taskrc.template

# Edit your config
nano ~/.taskrc
# Add the sections you want
```

The template adds:
- GTD reports (`in`, `next`, `sdm`, `nna`, `waiting`)
- Context definitions
- Urgency adjustments for `+sdm` (someday/maybe)
- UDA definitions for energy/brainpower

### 4. Install Shell Aliases

**For Zsh:**
```bash
echo 'source ~/src/@spandaworks/gtd/config/aliases.sh' >> ~/.zshrc
source ~/.zshrc
```

**For Bash:**
```bash
echo 'source ~/src/@spandaworks/gtd/config/aliases.sh' >> ~/.bashrc
source ~/.bashrc
```

**Note:** Adjust the path if you cloned to a different location.

### 5. Configure MCP Client

#### Claude Desktop

Edit `~/.config/claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "@spandaworks/gtd": {
      "command": "node",
      "args": ["/home/YOUR_USER/src/@spandaworks/gtd/mcp-server/dist/index.js"]
    }
  }
}
```

Restart Claude Desktop after editing.

#### OpenCode

Edit `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "servers": {
      "spandaworks_gtd": {
        "type": "local",
        "command": ["node", "/home/YOUR_USER/src/@spandaworks/gtd/mcp-server/dist/index.js"]
      }
    }
  }
}
```

## Verification

### Test TaskWarrior

```bash
# Check version
task --version

# Test custom reports
task in        # Should show inbox (empty initially)
task next      # Should show next actions
task sdm       # Should show someday/maybe
```

### Test Aliases

```bash
# Capture to inbox
in "Test inbox item"

# View inbox
ti

# Should show your test item with +in tag
```

### Test MCP Server

```bash
# Start server manually (for testing)
node ~/src/@spandaworks/gtd/mcp-server/dist/index.js

# Should output: "MCP TaskWarrior Server running on stdio"
# Press Ctrl+C to stop
```

### Test in AI Client

In your MCP client, try:
- "Show me my inbox" → Should call `process_inbox`
- "What should I work on?" → Should call `get_next_actions`

## Troubleshooting

### "task: command not found"

TaskWarrior isn't installed or not in PATH:
```bash
which task
# If empty, install TaskWarrior
```

### "Cannot find module" errors

MCP server not built:
```bash
cd ~/src/@spandaworks/gtd/mcp-server
npm install
npm run build
```

### Aliases not working

Shell not reloaded:
```bash
source ~/.zshrc  # or ~/.bashrc
```

Or path is wrong in the source line.

### MCP client doesn't see tools

1. Check the path in your MCP config is absolute (not relative)
2. Ensure Node.js is in PATH
3. Restart the MCP client
4. Check client logs for errors

### TaskWarrior 2.x compatibility

@spandaworks/gtd is designed for TaskWarrior 3.x. Some features may work with 2.x but it's not officially supported. Upgrade if possible:
```bash
task --version
# If 2.x, consider upgrading
```

## Next Steps

- Read [workflow.md](workflow.md) for GTD workflow guidance
- Print [gtd-cheatsheet.md](gtd-cheatsheet.md) for quick reference
- Review [mcp-tools.md](mcp-tools.md) for tool documentation
