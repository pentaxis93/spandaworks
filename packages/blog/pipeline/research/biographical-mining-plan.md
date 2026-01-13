# Biographical Mining Research Plan

*Deep extraction of life texture from email and calendar archives*

**Created:** 2026-01-13
**Status:** Active
**Purpose:** Build rich biographical context that makes blog content authentic

---

## Data Sources

| Source | Tool | Volume | Timespan |
|--------|------|--------|----------|
| Email | notmuch | 149,763 total | Decades |
| Email (recent) | notmuch | 57,015 | Last 10 years |
| Email (coding era) | notmuch | 16,322 | Since 2022 |
| Calendar | khal | ~8,000 events | Since 2020 |

---

## Research Objectives

### Primary: Life Arcs

Identify major biographical arcs that provide narrative backbone:
- Career transitions and pivots
- Geographic moves
- Relationship milestones
- Learning journeys (pre-coding and coding)
- Health/wellness arcs
- Spiritual/philosophical development

### Secondary: Texture & Details

Extract specific details that make stories come alive:
- Named people (mentors, collaborators, family)
- Specific places (cities, companies, institutions)
- Concrete dates (when things actually happened)
- Memorable events (conferences, launches, crises)
- Recurring patterns (habits, struggles, victories)

### Tertiary: Voice Markers

Find authentic voice patterns:
- How you write to different audiences
- Recurring phrases or metaphors
- Humor patterns
- Emotional expression style

---

## Phase 1: Reconnaissance (Explore agents)

**Goal:** Map the terrain before deep mining

### 1a. Email Ecosystem Survey

```bash
# Sender frequency (who do you communicate with most?)
notmuch address --output=sender --deduplicate=address '*' | head -50

# Recipient frequency (who do you write to most?)
notmuch address --output=recipients --deduplicate=address 'from:robbie' | head -50

# Tag landscape (how is mail organized?)
notmuch search --output=tags '*'

# Folder/mailbox structure
notmuch search --output=files '*' | cut -d/ -f1-6 | sort -u | head -20
```

**Agent task:** Explore email ecosystem, identify key correspondents, understand organization

### 1b. Calendar Pattern Survey

```bash
# Event types (what kinds of things appear?)
khal list 2020-01-01 2026-01-13 | grep -oP '^\S+' | sort | uniq -c | sort -rn | head -20

# Recurring events (what's habitual?)
khal list 2020-01-01 2026-01-13 | sort | uniq -c | sort -rn | head -30

# Dense periods (when was life busiest?)
khal list 2020-01-01 2026-01-13 | grep -oP '^\d{4}-\d{2}' | uniq -c | sort -rn | head -20
```

**Agent task:** Map calendar patterns, identify recurring commitments, find life rhythm

### 1c. Temporal Distribution

```bash
# Email by year
for year in 2015 2016 2017 2018 2019 2020 2021 2022 2023 2024 2025 2026; do
  echo -n "$year: "; notmuch count "date:$year"
done

# Email by month (last 3 years)
for ym in 2023-{01..12} 2024-{01..12} 2025-{01..12}; do
  echo -n "$ym: "; notmuch count "date:$ym"
done
```

**Agent task:** Build temporal map, identify busy/quiet periods, correlate with life events

---

## Phase 2: Arc Extraction (Research agents)

**Goal:** Identify major life arcs with evidence

### 2a. Career Arc

**Search patterns:**
- Job-related: `subject:(offer OR interview OR resignation OR hired OR promotion)`
- Company names: Search for known employers
- Professional identity: `from:robbie subject:(work OR project OR client)`

**Extract:**
- Employment dates and companies
- Role transitions
- Major projects
- Professional relationships

### 2b. Learning Arc

**Search patterns:**
- Education: `subject:(course OR class OR certificate OR degree OR CS50)`
- Coding: `subject:(code OR programming OR python OR dart OR flutter)`
- Learning: `subject:(learn OR tutorial OR bootcamp)`

**Extract:**
- Formal education timeline
- Self-directed learning phases
- Skill acquisition moments
- Breakthrough realizations

### 2c. Geographic Arc

**Search patterns:**
- Moving: `subject:(move OR moving OR apartment OR house OR lease)`
- Locations: Search for city/country names
- Travel: `subject:(flight OR hotel OR travel OR trip)`

**Extract:**
- Where you've lived and when
- Significant moves and their context
- Travel patterns

### 2d. Relationship Arc

**Calendar patterns:**
- Recurring people (names that appear frequently)
- Relationship milestones in calendar

**Email patterns:**
- High-frequency correspondents over time
- Tone/content evolution with key people

**Extract:**
- Key relationships (family, friends, mentors)
- When relationships formed/deepened
- Collaborative partnerships

### 2e. Philosophical/Spiritual Arc

**Search patterns:**
- Buddhism: `subject:(dharma OR meditation OR buddhist OR retreat)`
- Philosophy: `subject:(philosophy OR wisdom OR practice)`
- Personal growth: `subject:(therapy OR growth OR insight)`

**Extract:**
- When spiritual interests emerged
- Key teachers or influences
- Practice evolution

---

## Phase 3: Texture Mining (Research agents)

**Goal:** Extract specific details that bring stories alive

### 3a. The CS50 Era (2022)

Deep dive into the coding origin story:
- Emails around CS50 enrollment/completion
- Calendar entries during the course
- Correspondence about learning to code
- Emotional arc (frustration → breakthrough)

### 3b. The Framework Wandering (2023)

The year of trying everything:
- Next.js, React, RedwoodJS, Svelte mentions
- Project starts and abandonments
- Learning resources shared/discussed
- Moments of clarity or confusion

### 3c. The Zenvestor Crystallization (2024-2025)

When everything came together:
- First mentions of investment tracking
- Python → Dart decision point
- AI collaboration emergence
- Meta-level shift to building tools

### 3d. Named Details

Mine for specific, citable details:
- Teacher names (Mr. Horton confirmed, others?)
- Specific books, courses, resources
- Conference or event attendance
- Memorable quotes from correspondence

---

## Phase 4: Synthesis

**Goal:** Integrate findings into author context

### 4a. Update author.md

Expand with:
- Verified timeline with specific dates
- Named people and relationships
- Geographic history
- Career arc
- Pre-coding life context

### 4b. Create supporting files

If volume warrants:
- `pipeline/context/timeline.md` - Detailed chronology
- `pipeline/context/people.md` - Key relationships
- `pipeline/context/places.md` - Geographic context

### 4c. Extract quotable moments

Build a file of authentic voice samples:
- Memorable email snippets
- Calendar entry phrasings
- Recurring expressions

---

## Agent Delegation Strategy

### Explore Agents (Phase 1)

Fast, broad reconnaissance:
- "Survey email senders/recipients, report top 50 with counts"
- "Map calendar event types and frequencies"
- "Build temporal distribution of email volume"

### Research Agents (Phase 2-3)

Deep, focused investigation:
- "Extract career timeline with evidence from email"
- "Find all CS50-related correspondence, build narrative"
- "Identify named teachers, mentors, collaborators"

### Synthesis (Phase 4)

Main session integrates findings into context files.

---

## Privacy Considerations

- Extract patterns and facts, not raw correspondence
- Names of public figures or professional contacts: OK
- Private relationships: Use discretion, anonymize if needed
- Financial/health details: Aggregate patterns only
- No raw email content in context files

---

## Success Criteria

The research is complete when:
- [ ] Major life arcs identified with approximate dates
- [ ] Key relationships named (where appropriate)
- [ ] Geographic history documented
- [ ] Coding journey has specific texture (dates, struggles, breakthroughs)
- [ ] Voice patterns identified with examples
- [ ] author.md expanded with verified details
- [ ] Supporting context files created if needed

---

*The texture of lived experience is the antidote to AI slop.*
