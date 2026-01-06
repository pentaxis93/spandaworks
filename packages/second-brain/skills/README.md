# Second-Brain Skills

Knowledge processing skills for vault operations.

## Alchemical Skills (Opus-class required)

These skills synthesize wisdom and require Opus-class model:

| Skill | Purpose |
|-------|---------|
| **fuse** | Knowledge consolidation from scattered insights |
| **integrate** | Pattern recognition and vault integration |
| **distill** | Extract themed content into alembic |
| **prism** | Multi-perspective analysis (10 lenses) |
| **debrief** | Weekly cross-domain reflection |
| **archive** | Direct permanent note creation |

### fuse

Synthesizes scattered insights into integration candidates.

**Trigger:** `/second-brain:fuse` or "consolidate", "fuse knowledge"

**Input:** Dumps, captures, alembic extracts  
**Output:** `00-inbox/integration/candidate-*.md`

**Process:**
1. Gather source material
2. Identify synthesis opportunities
3. Create candidate notes
4. Prompt for `/integrate`

### integrate

Smith's Loop refinement mechanism.

**Trigger:** `/second-brain:integrate` or "integrate candidates"

**Input:** Integration candidates  
**Output:** Permanent notes, frameworks, patterns

**Decisions:**
- MERGE: Combine with existing note
- LINK: Connect to existing without merging
- MULTI-PATH: Create multiple linked notes
- CREATE NEW: New standalone note

### distill

Surgical extraction into themed containers.

**Trigger:** `/second-brain:distill` or "extract to alembic"

**Input:** Captures with mixed content  
**Output:** `00-inbox/alembic/extract-*.md`

**Alembic States:**
- `accumulating` - Extracts being added
- `ready-for-fusion` - Ready for fuse
- `held` - Post-fusion, multi-pass available

### prism

10-lens analytical framework.

**Trigger:** `/second-brain:prism` or "analyze from multiple angles"

**Lenses:**
1. Actionable
2. Timeline
3. Relationship
4. Decision
5. Content
6. Learning
7. Resource
8. Strategic
9. Pattern
10. Emotional

### debrief

Weekly cross-domain pattern recognition.

**Trigger:** `/second-brain:debrief` or "weekly review"

**Output:** `archives/intelligence/checkins/debrief-*.md`

**Process:** Gather context → Guided reflection → Pattern recognition → Forward planning

### archive

Direct permanent note creation with full integration.

**Trigger:** "archive", "archive this"

**Process:**
1. Extract atomic idea
2. Write in own words
3. Apply frontmatter
4. Create 3+ links
5. Place correctly
6. Verify integrity

## Reconnaissance Skills (Any model)

| Skill | Purpose |
|-------|---------|
| **dump** | Raw thought capture with classification |
| **intel** | External news intelligence |
| **audit** | Internal vault health assessment |

### dump

Raw thought capture.

**Trigger:** `/second-brain:dump` or "braindump"

**Output:** `00-inbox/dumps/dump-*.md`

### intel

External news gathering.

**Trigger:** `/second-brain:intel` or "news"

**Reads:** Profile.md, Interests.md, Watchlist.md  
**Output:** `archives/intelligence/intel/intel-*.md`

### audit

Vault health assessment.

**Trigger:** `/second-brain:audit` or "vault survey"

**Output:** `archives/intelligence/audit/audit-*.md`

**Scans:** Inbox, projects, areas, knowledge assets
