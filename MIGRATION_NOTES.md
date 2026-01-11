# Migration Notes: spandaworks → aiandi

This document tracks the internal transformation from "spandaworks" to "aiandi".

## Completed

- [x] Repository renamed: `~/src/spandaworks` → `~/src/aiandi`
- [x] Cargo workspace created at repo root
- [x] CLI crate created: `crates/aiandi/`
- [x] Skills directory created: `skills/` with transmission and gtd
- [x] Templates created: `templates/minimal.md` and `templates/comprehensive.md`
- [x] Packaging reorganized: `aur-packages/` → `packaging/aur/`
- [x] README.md updated to explain aiandi
- [x] Key documentation updated (pim/README.md, telemetry/README.md)

## Still Using "spandaworks" (Intentionally)

These references cannot be changed yet without breaking functionality:

### Package Names

```toml
# packages/pim/mcp-server/Cargo.toml
name = "spandaworks-pim"      # Binary name used in MCP config
```

**Reason:** Existing installations reference this binary name. Will rename in coordinated release.

### Internal Library Names

```toml
# shared/utils/rust-cli/Cargo.toml (likely)
name = "spandaworks-cli"
```

**Reason:** Used as dependency by pim MCP server. Needs coordinated update.

### Python Package Names

```python
# packages/telemetry/ (setup.py or pyproject.toml)
from spandaworks_telemetry.mcp import ...
```

**Reason:** Import paths used by MCP integration. Needs version bump and migration path.

### Documentation References

Many docs still reference "spandaworks" in:
- Code examples
- File paths (~/.spandaworks/)
- Event schemas
- Architecture diagrams

**Strategy:** Update incrementally as features are implemented. Low priority for now.

## Future Work

1. **Coordinated binary rename**
   - Release v0.2.0 with `aiandi-pim` binary
   - Update MCP config examples
   - Provide migration guide

2. **Python package rename**
   - `spandaworks_telemetry` → `aiandi_telemetry`
   - Provide compatibility shim for transition

3. **Shared utilities**
   - `spandaworks-cli` → `aiandi-cli`
   - Update all dependent crates

4. **Documentation sweep**
   - Search all .md files for "spandaworks"
   - Update code examples
   - Update architecture diagrams

5. **Config/data paths**
   - Consider: ~/.spandaworks/ → ~/.aiandi/
   - Provide migration script for existing users

## Not Changed

These keep original names intentionally:

- Git commit history (preserved)
- Archived transmissions in outputs/
- Historical documentation references
- Blog posts (separate repo: aiandi-blog)

---

**Status:** Foundation established. Internal structure transformed. Binary/package names to follow in phased releases.
