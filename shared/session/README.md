# Session Protocol

Sessions are the container for practice work.

## States

```
UNOPENED -> OPENING -> OPEN -> CLOSING -> CLOSED
               |                  |
               +-- (ceremony) ----+
```

## Metadata

- session_id, opened_at, closed_at
- goal, persona, protocol (e.g., "LBRP")
- human (collaborator name)
- inherited_count (knowledge state at start)

## Session vs GTD

- Sessions are opened/closed via /open and /close
- GTD work happens independently of session state
- At session close, relevant GTD completions may be noted
- Sessions are for practice; GTD is for execution
