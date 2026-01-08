# ADR: Project-Level Skill Discovery

**Date:** 2026-01-08  
**Status:** Accepted  
**Context:** Skill installation and discovery mechanism

---

## Context

OpenCode supports custom agent skills through a discovery mechanism that searches multiple locations for `SKILL.md` files. Initially, Spandaworks skills were stored in `packages/core/skills/` and manually symlinked to the global `~/.config/opencode/skill/` directory.

This approach had several problems:
1. **Manual setup required** - New users had to create symlinks manually
2. **Global pollution** - Skills installed globally rather than project-scoped
3. **Not collaborative** - Skills wouldn't work when others cloned the repository
4. **Non-canonical** - Violated OpenCode's documented best practices

## Decision

**Move all Spandaworks skills from `packages/core/skills/` to `.opencode/skill/`** and rely on OpenCode's native project-level discovery mechanism.

### OpenCode Skill Discovery (Canonical)

Per the [OpenCode documentation](https://opencode.ai/docs/skills/), OpenCode searches these locations in order:

**Project-level (auto-discovered):**
- `.opencode/skill/<name>/SKILL.md`
- `.claude/skills/<name>/SKILL.md`

**Global (manual installation):**
- `~/.config/opencode/skill/<name>/SKILL.md`
- `~/.claude/skills/<name>/SKILL.md`

OpenCode walks up from the current working directory to the git worktree root, discovering any skills along the way.

### Our Structure

```
spandaworks/
├── .opencode/
│   └── skill/
│       ├── lbrp/
│       │   └── SKILL.md
│       ├── perspective-review/
│       │   ├── SKILL.md
│       │   ├── personas/
│       │   ├── templates/
│       │   └── examples/
│       ├── save-transcript/
│       │   └── SKILL.md
│       ├── spandaworks-docs/
│       │   └── SKILL.md
│       ├── taskwarrior-gtd/
│       │   └── SKILL.md
│       └── the-art/
│           └── SKILL.md
└── packages/
    └── core/
        ├── ceremony/
        ├── commands/
        └── plugins/
```

**Note:** `packages/core/skills/` no longer exists.

## Implementation

**Migration steps:**
1. Created `.opencode/skill/` directory
2. Moved all skills from `packages/core/skills/*` to `.opencode/skill/*`
3. Removed `packages/core/skills/` directory
4. Deleted global symlinks from `~/.config/opencode/skill/`
5. Updated documentation references

**No symlinks required.** OpenCode auto-discovers skills from `.opencode/skill/` when working in the project.

## Consequences

### Positive

✓ **Zero manual setup** - Skills work immediately when cloning the repo  
✓ **Project-scoped** - No global config pollution  
✓ **Collaborative** - Works for all contributors automatically  
✓ **Canonical** - Follows OpenCode best practices  
✓ **Version controlled** - Skills are part of the repository  
✓ **Discoverable** - Standard OpenCode discovery mechanism

### Neutral

- Skills only available when working within the Spandaworks project
- For global availability, users can manually symlink from `.opencode/skill/` to `~/.config/opencode/skill/` if desired

### Maintenance

**For new skills:**
```bash
# Create skill directory
mkdir -p .opencode/skill/my-new-skill

# Create SKILL.md with frontmatter
cat > .opencode/skill/my-new-skill/SKILL.md << 'EOF'
---
name: my-new-skill
description: Brief description for skill discovery
---

# My New Skill

[Skill implementation...]
EOF
```

**Verification:**
OpenCode automatically discovers skills on startup. No restart or reload required.

## References

- [OpenCode Skill Documentation](https://opencode.ai/docs/skills/)
- [Skill Frontmatter Requirements](https://opencode.ai/docs/skills/#write-frontmatter)
- [Skill Discovery Mechanism](https://opencode.ai/docs/skills/#understand-discovery)

---

**Replaced approach:**  
~~`packages/core/skills/` + manual global symlinks~~

**New approach:**  
`.opencode/skill/` with native OpenCode discovery
