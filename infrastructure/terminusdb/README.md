# TerminusDB Agent Memory Infrastructure

Local TerminusDB installation for agent biographical memory.

## Quick Reference

| Item | Value |
|------|-------|
| **Endpoint** | `http://localhost:6363` |
| **Database** | `ops_memory` |
| **User** | `admin` |
| **Password** | `aiandi_memory_2024` |
| **Version** | TerminusDB 12.0.2 |

## Start/Stop Commands

```bash
# Start TerminusDB
cd /home/pentaxis93/src/aiandi/infrastructure/terminusdb
sudo docker-compose up -d

# Stop TerminusDB
sudo docker-compose down

# View logs
sudo docker logs aiandi-terminusdb

# Check status
sudo docker ps --filter name=aiandi-terminusdb
```

## Ops Memory Module

The `ops_memory` module provides read/write operations for ops mode biographical memory.

### Python API

```python
from ops_memory import OpsMemory
from ops_memory.entities import LearningDomain, EventType

memory = OpsMemory()

# Log session start (links to previous session automatically)
session = memory.log_session_start(
    session_id="2026-01-14-ops-morning",
    mode="ops",
    goals={"Handle inbox", "Plan week"}
)

# Create learnings
learning = memory.create_learning(
    content="Weekly review is most effective Monday morning",
    domain=LearningDomain.PROCEDURAL,
    confidence=0.85
)

# Create decisions
decision = memory.create_decision(
    description="Use TerminusDB for biographical memory",
    context="Need persistent memory for ops mode",
    rationale="Git-like versioning fits the use case",
    options_considered={"SQLite", "MongoDB", "TerminusDB"}
)

# Log session end with learnings
memory.log_session_end(
    session_id="2026-01-14-ops-morning",
    summary="Completed inbox processing, planned week",
    learnings=["Email batching saves context switches"],
    open_loops={"Follow up with Gerald"}
)

# Get context for new session
context = memory.get_context_for_session_start()
print(f"Last session: {context['last_session'].summary}")
print(f"Open loops: {context['open_loops']}")
```

### CLI Usage

```bash
# From terminusdb directory
cd infrastructure/terminusdb

# Install schema (first time only)
./.venv/bin/python -m ops_memory.cli schema install

# Verify schema
./.venv/bin/python -m ops_memory.cli schema verify

# Show database statistics
./.venv/bin/python -m ops_memory.cli stats

# Get context for session start
./.venv/bin/python -m ops_memory.cli context

# Session management
./.venv/bin/python -m ops_memory.cli session list
./.venv/bin/python -m ops_memory.cli session create --id "2026-01-14-ops" --mode ops --goals "Task 1" "Task 2"
./.venv/bin/python -m ops_memory.cli session close --id "2026-01-14-ops" --summary "Completed tasks"

# Log session start/end (convenience commands)
./.venv/bin/python -m ops_memory.cli log start --id "my-session" --mode ops
./.venv/bin/python -m ops_memory.cli log end --id "my-session" --summary "Done" --learnings "Learned X"

# Learnings
./.venv/bin/python -m ops_memory.cli learning list
./.venv/bin/python -m ops_memory.cli learning create --content "New insight" --domain procedural
./.venv/bin/python -m ops_memory.cli learning search "keyword"

# Decisions
./.venv/bin/python -m ops_memory.cli decision list
./.venv/bin/python -m ops_memory.cli decision create --description "Did X" --context "Because Y" --rationale "Chose Z"
```

### Running Tests

```bash
cd infrastructure/terminusdb
./.venv/bin/python -m ops_memory.test_ops_memory
```

## Entity Types (Phase 1)

| Entity | Purpose |
|--------|---------|
| **Session** | Bounded interaction unit with goals, summary, open loops |
| **Event** | Something that happened (bi-temporal: occurred vs learned) |
| **Learning** | Knowledge product with confidence and domain |
| **Decision** | Choice made with context, rationale, outcome |

Relationships:
- Session -> produces -> Learning, Decision
- Learning -> derived_from -> Event
- Decision -> led_to -> Event
- Session -> followed -> Session

## Files

- `docker-compose.yml` - Docker configuration
- `.env` - Environment variables (passwords)
- `setup_database.py` - Initial database creation
- `test_connection.py` - Connectivity verification
- `ops_memory/` - Ops memory Python module
  - `__init__.py` - Module exports
  - `client.py` - OpsMemory client class
  - `entities.py` - Entity dataclasses
  - `schema.py` - TerminusDB schema definitions
  - `cli.py` - Command-line interface
  - `test_ops_memory.py` - Test suite
- `schema/` - Ontology documentation
  - `ops_memory_ontology.md` - Full ontology design

## Why TerminusDB?

TerminusDB was chosen for agent memory because:

1. **Git-like versioning** - Every change is a commit, enabling time-travel queries
2. **Bi-temporal model** - Track both when facts are true and when we learned them
3. **JSON documents** - Natural fit for agent observations and memories
4. **Graph queries** - Connect memories through semantic relationships
5. **Local-first** - Full sovereignty over agent memory data

## Troubleshooting

### Container won't start
```bash
# Check if port is in use
sudo lsof -i :6363

# Check Docker logs
sudo docker logs aiandi-terminusdb
```

### Permission issues
User must be in docker group:
```bash
sudo usermod -aG docker $USER
# Then logout/login or: newgrp docker
```

### Reset database
```bash
# WARNING: Destroys all data
sudo docker-compose down -v
sudo docker-compose up -d
./.venv/bin/python setup_database.py
./.venv/bin/python -m ops_memory.cli schema install
```

### Test everything
```bash
# Quick connectivity test
curl http://localhost:6363/api/info

# Full Python client test
./.venv/bin/python test_connection.py

# Ops memory test suite
./.venv/bin/python -m ops_memory.test_ops_memory
```
