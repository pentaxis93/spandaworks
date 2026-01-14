# Blog Content Pipeline

*Status-based workflow for solo operation*

---

## Core Principle

**Each stage creates an artifact that persists progress to the next session.**

A forgetful human or an agent without context should be able to walk in at any time and pick up wherever we left off. This means:

- Every stage produces a file in `pipeline/`
- `PIPELINE.md` tracks current stage for each article
- Artifacts are self-contained (no context required to understand them)
- Progress survives session boundaries

---

## Pipeline Stages

```
IDEA → RESEARCH → WALK → OUTLINE → DRAFT → VOICE → TECHNICAL → POLISH → SLEEP → PROOF → PUBLISH → MAINTAIN
```

Each stage has a quality gate. Cannot advance until gate passes.

---

## Quality Gates

| Stage | Gate Question | Pass Criteria |
|-------|---------------|---------------|
| **Idea** | Can you state the teaching in one sentence? | Single-sentence purpose written |
| **Research** | Do you have examples for every claim? | All source material assembled, no gaps |
| **Walk** | Did you extract the heart of the story? | Walk recording transcribed, insights captured |
| **Outline** | Could someone else write from this? | Complete structure with section bullets |
| **Draft** | Is there prose for every outline point? | All sections filled, [TODO] markers resolved |
| **Voice** | Does it sound like you talking? | Read aloud, no stiff parts remain |
| **Technical** | Would you stake reputation on every sentence? | All facts verified, code tested, links checked |
| **Polish** | Does the writing feel alive? | No slop, voice verified, specificity earned |
| **Sleep** | Did you sleep on it? | At least one night between Polish and Proof |
| **Proof** | Does it still hold up with fresh eyes? | Final read-through, last edits made |
| **Publish** | Live and rendering correctly? | Deployed, verified, announced |
| **Maintain** | Still accurate? | 6-month review scheduled |

---

## Stage Ordering

**The pipeline order is canonical:** IDEA → RESEARCH → WALK → OUTLINE → ...

RESEARCH must precede WALK because research informs the walking guide. You need to know what material exists before designing questions that surface what's *not* yet articulated.

### Article Types

| Type | RESEARCH Focus | WALK Focus |
|------|----------------|------------|
| **Technical** | Gather code, commits, docs | Surface the "why" behind decisions |
| **Biographical** | Verify eterne context loaded, identify gaps | Extract tacit knowledge, emotional arc |
| **Methodological** | Document current practice, find examples | Surface teaching approach, find stories |

For biographical articles, RESEARCH may be lighter (context already exists in eterne), but it still happens:
- Verify author context is loaded
- Identify which claims need evidence
- Note gaps the WALK should explore

**There are no valid shortcuts that skip RESEARCH.** A light RESEARCH is still RESEARCH.

**User-provided materials:** During RESEARCH, user may provide additional documents, notes, or references. These go in `inbox/` for the session to process.

---

## Time Allocation (40-20-40)

- **40% Planning**: Idea + Research + Walk + Outline
- **20% Drafting**: Draft
- **40% Revision**: Voice + Technical + Polish

If you're spending 80% drafting, the outline isn't good enough.

---

## Current Articles

| # | Slug | Title | Stage | Notes |
|---|------|-------|-------|-------|
| 001 | `56-beginner` | 56, Beginner | SLEEP | Polish complete; sleeping on it |
| 002 | `cs50-decision-at-53` | The CS50 Decision at 53 | WALK | Walk guide ready, awaiting recording |

---

## Backlog (Ideas)

| Working Title | One-Sentence Teaching | Priority |
|---------------|----------------------|----------|
| | | |

---

## Skills by Stage

| Stage | Skills Used | Purpose |
|-------|-------------|---------|
| **Idea** | - | Human identifies story and teaching |
| **Research** | `research-gather` | Systematically gather source material |
| **Walk** | `question-design` | Core question principles |
| | `questions-[type]` | Story-type specific questions |
| | `walk-and-talk` | Format for mobile/walking |
| | `transcribe-walk` | Process recording into insights |
| **Outline** | `outline-review` | Review structure, flow, completeness |
| **Draft** | `the-art` (optional) | Synthesize research + voice |
| **Voice** | `voice` | Check consistency with Robbie's voice |
| | `dry-structural-irony` | Signature humor technique |
| **Technical** | `technical-review` | Fact-check, verify code, check links |
| **Polish** | `editors-table` | Kill slop, verify authenticity |
| **Publish** | `publish-article` | Frontmatter, deploy, verify, announce |

### AI Integration Points

**High Value:**
- **Research**: Gather GitHub history, session transcripts, technical context
- **Walk**: Design questions, generate walking guide
- **Outline**: Review structure for logic and flow
- **Voice**: Check sections for voice consistency
- **Technical**: Fact-check claims, verify code examples
- **Polish**: Kill slop, verify authenticity, audit specificity

**Low Value (Anti-patterns):**
- Generating complete drafts (loses voice)
- Coming up with ideas (you have real stories)
- Writing introductions (needs your perspective)

---

## Stage Details

Each stage has associated skills. Load the skill for full process documentation.

### Research
**Skill:** `research-gather`
**Artifact:** `pipeline/active/NNN-slug/02-research.md`
**Template:** `pipeline/_research-template.md`
**Pre-requisite:** Load context from `~/src/eterne/vault/03-resources/blog-context/author.md`

### Walk
**Skills:** `question-design` → `questions-[type]` → `walk-and-talk` → `transcribe-walk`

**Artifacts:**
| File | Purpose |
|------|---------|
| `pipeline/active/NNN-slug/03-walk.md` | Walking guide with questions |
| `pipeline/active/NNN-slug/03-walk-transcript.md` | Post-walk: cleaned transcript + extracted insights |

**Template:** `pipeline/_transcript-template.md`

| Story Type | Question Skill |
|------------|----------------|
| Journey ("How I learned X") | `questions-journey` |
| Insight ("I discovered X") | `questions-insight` |
| Evolution ("X changed to Y") | `questions-evolution` |
| Methodology ("How to do X") | `questions-methodology` |

**When to skip:** Purely technical content, already articulated, time pressure.

### Outline
**Skill:** `outline-review`
**Artifact:** `pipeline/active/NNN-slug/04-outline.md`

### Draft
**Skill:** `the-art` (optional - for synthesis of research + voice)
**Artifact:** `pipeline/active/NNN-slug/05-draft.md`

### Voice
**Skills:** `voice`, `dry-structural-irony`
**Method:** Read aloud, check against skill patterns, identify irony opportunities

### Technical
**Skill:** `technical-review`
**Method:** Verify code, check links, fact-check claims

### Polish
**Skill:** `editors-table`
**Method:** Four-pass review (slop detection, voice verification, specificity audit, synthesis check)
**Gate:** Does the writing feel alive? Would readers sense a specific human wrote this?

### Sleep
**Skill:** None (human only)
**Method:** Wait at least one night. Let the subconscious process.
**Gate:** Did you sleep on it?

### Proof
**Skill:** None (human only)
**Method:** Fresh-eyes read-through. Final edits. Read aloud one more time.
**Gate:** Does it still hold up? Any last changes before it goes live?

### Publish
**Skill:** `publish-article`
**Destination:** `src/content/blog/NNN-slug.md`
**Archive:** Move `pipeline/active/NNN-slug/` to `pipeline/published/NNN-slug/`

---

## Workflow Commands

### Start New Article
1. Add to Backlog with one-sentence teaching
2. When ready to work: create `pipeline/active/NNN-slug/` folder (next sequential number)
3. Add to Current Articles table at IDEA stage
4. Progress through stages, checking gates

### Stage Transitions
Before advancing, ask the gate question. If answer is "no", stay in current stage.

### The Art Integration
After DRAFT stage, can invoke `the-art` skill to mix Sol (research material) with Luna (voice) for synthesis. This is optional - some articles may flow naturally without explicit invocation.

---

## Directory Structure

```
packages/blog/
├── PIPELINE.md              # This file (status tracking)
├── src/content/
│   └── blog/                # Published articles (NNN-slug.md)
└── pipeline/
    ├── _research-template.md
    ├── _transcript-template.md
    ├── active/
    │   └── NNN-slug/
    │       ├── 01-idea.md
    │       ├── 02-research.md
    │       ├── 03-walk.md
    │       ├── 03-walk-transcript.md
    │       ├── 04-outline.md
    │       └── 05-draft.md
    └── published/           # Archived after publish
        └── NNN-slug/
```

Each article gets a sequentially numbered folder. All artifacts for that article live together. After publish, the folder moves to `published/` for reference.

### Context Loading

**Before any content generation**, load biographical context from eterne (private vault):

```
~/src/eterne/vault/03-resources/blog-context/
├── author.md           # Quick reference (always load)
├── timeline/overview.md # Full chronological arc
├── people/family.md    # Family relationships
└── themes/
    ├── spiritual.md    # Buddhist practice, Aka Dua
    └── career.md       # Professional history
```

**Why eterne?** Biographical context contains sensitive personal information (full name, family names, health history, location) that should not be in the public aiandi repo.

**What it provides:**
- Verified biographical facts (age, journey timeline, metrics)
- Current project status and scale
- Key claims with evidence
- Family and relationship context
- Spiritual/philosophical background
- Career arc (including illness era context)

This prevents hallucinated details and ensures consistency across articles.

---

## Anti-Patterns

| Pattern | Problem | Solution |
|---------|---------|----------|
| Writing without outline | Wandering structure | 40% time on outline |
| Editing while drafting | Never finish | Separate draft from edit |
| Skipping voice pass | Generic tech blog voice | Read aloud, rewrite stiff parts |
| Perfectionism loop | Never ship | Ship when gate passes, not when it feels perfect |
| No technical review | Eroded trust | Check every fact |

---

*The pipeline serves the writing. The writing serves the reader. The reader serves all beings.*
