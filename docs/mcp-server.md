# aiandi MCP Server

The `aiandi serve` command starts an MCP (Model Context Protocol) server that exposes aiandi's capabilities to OpenCode and other MCP clients.

## What is MCP?

MCP (Model Context Protocol) is a JSON-RPC based protocol that allows AI assistants to use tools and access external capabilities. The aiandi MCP server makes aiandi's commands available as tools that OpenCode can invoke.

## Tools Exposed

The aiandi MCP server exposes three tools:

### 1. aiandi_init

Initialize aiandi by extracting bundled skills to `.opencode/skill/` directory.

**Parameters:**
- `skills` (optional string): Comma-separated list of skills to install (default: all)
- `force` (optional boolean): Overwrite existing files (default: false)

**Example:**
```json
{
  "skills": "transmission,gtd",
  "force": false
}
```

**Returns:** Success or error message

### 2. aiandi_inbox

Capture text to GTD inbox via TaskWarrior.

**Parameters:**
- `text` (required string): Text to capture to inbox
- `tags` (optional string): Additional tags, comma-separated
- `project` (optional string): Project to assign

**Example:**
```json
{
  "text": "Review quarterly goals",
  "tags": "review,planning",
  "project": "work"
}
```

**Returns:** Task ID or error message

### 3. aiandi_doctor

Run system health checks.

**Parameters:** None

**Example:**
```json
{}
```

**Returns:** Health check status

## Configuration

### OpenCode Configuration

Add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "aiandi": {
      "type": "local",
      "command": ["aiandi", "serve"],
      "enabled": true
    }
  }
}
```

### Verification

After configuring, restart OpenCode and verify the server is available:

1. OpenCode will automatically start the aiandi MCP server
2. Tools should appear in OpenCode's tool list
3. Check OpenCode logs if tools don't appear

## Usage from OpenCode

Once configured, OpenCode can invoke aiandi tools:

**Initialize skills:**
```
Use the aiandi_init tool to extract skills to my project
```

**Capture to inbox:**
```
Use aiandi_inbox to capture "Review project documentation" with tag "work"
```

**Health check:**
```
Run aiandi_doctor to check system health
```

## Technical Details

### Transport

The MCP server uses **stdio** (stdin/stdout) transport, not HTTP. This is the standard for MCP servers:

- **Input:** JSON-RPC messages on stdin
- **Output:** JSON-RPC responses on stdout
- **Logging:** All logging goes to stderr (never stdout)

### Protocol

The server implements the MCP JSON-RPC protocol:

1. **Initialization handshake**
   - Client sends `initialize` request
   - Server responds with capabilities
   - Client sends `initialized` notification

2. **Tool discovery**
   - Client sends `tools/list` request
   - Server responds with available tools and their schemas

3. **Tool invocation**
   - Client sends `tools/call` with tool name and parameters
   - Server executes tool and returns result

### Dependencies

The server uses:
- `rmcp` crate (v0.12) - MCP SDK for Rust
- `tokio` - Async runtime
- `serde_json` - JSON serialization

## Troubleshooting

### Server doesn't start

Check OpenCode logs:
```bash
tail -f ~/.config/opencode/logs/mcp-*.log
```

### Tools not appearing

1. Verify configuration in `opencode.json`
2. Ensure `aiandi` is in PATH
3. Restart OpenCode completely
4. Check for errors in logs

### Tool execution fails

The server logs to stderr. Check logs for error messages:
```bash
# If running manually for testing:
aiandi serve 2> aiandi-mcp.log
```

## Development

### Running manually

For testing, you can run the server manually:

```bash
aiandi serve
```

The server will wait for JSON-RPC messages on stdin. Send an initialize request:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
```

### Adding new tools

To add a new tool:

1. Define request type with `serde::Deserialize` and `schemars::JsonSchema`
2. Add tool method with `#[tool(...)]` attribute
3. Implement tool logic using existing command functions
4. Rebuild and test

See `crates/aiandi/src/commands/serve.rs` for examples.

## See Also

- [MCP Specification](https://modelcontextprotocol.io/)
- [OpenCode MCP Documentation](https://opencode.ai/docs/mcp)
- aiandi commands: `aiandi --help`
