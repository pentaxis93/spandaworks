# Blog Content Pipeline

*Status-based workflow for solo operation*

---

## Pipeline Stages

```
IDEA → RESEARCH → WALK → OUTLINE → DRAFT → VOICE → TECHNICAL → POLISH → PUBLISH → MAINTAIN
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
| **Publish** | Live and rendering correctly? | Deployed, verified, announced |
| **Maintain** | Still accurate? | 6-month review scheduled |

---

## Time Allocation (40-20-40)

- **40% Planning**: Idea + Research + Walk + Outline
- **20% Drafting**: Draft
- **40% Revision**: Voice + Technical + Polish

If you're spending 80% drafting, the outline isn't good enough.

---

## Current Articles

| Slug | Title | Stage | Target | Notes |
|------|-------|-------|--------|-------|
| `i-dont-know-what-im-talking-about` | I Don't Know What I'm Talking About | PUBLISHED | - | Foundational piece |

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
**Artifact:** `pipeline/research/[slug]-research.md`
**Pre-requisite:** Load context from `~/src/eterne/vault/03-resources/blog-context/author.md`

### Walk
**Skills:** `question-design` → `questions-[type]` → `walk-and-talk` → `transcribe-walk`
**Artifact:** `pipeline/research/[slug]-walk.md`

| Story Type | Question Skill |
|------------|----------------|
| Journey ("How I learned X") | `questions-journey` |
| Insight ("I discovered X") | `questions-insight` |
| Evolution ("X changed to Y") | `questions-evolution` |
| Methodology ("How to do X") | `questions-methodology` |

**When to skip:** Purely technical content, already articulated, time pressure.

### Outline
**Skill:** `outline-review`
**Artifact:** `pipeline/outlines/[slug].md`

### Draft
**Skill:** `the-art` (optional - for synthesis of research + voice)
**Artifact:** `pipeline/drafts/[slug].md`

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

### Publish
**Skill:** `publish-article`
**Destination:** `src/content/blog/[slug].md`

---

## Workflow Commands

### Start New Article
1. Add to Backlog with one-sentence teaching
2. When ready to work: move to Current Articles at IDEA stage
3. Progress through stages, checking gates

### Stage Transitions
Before advancing, ask the gate question. If answer is "no", stay in current stage.

### The Art Integration
After DRAFT stage, can invoke `the-art` skill to mix Sol (research material) with Luna (voice) for synthesis. This is optional - some articles may flow naturally without explicit invocation.

---

## Directory Structure

```
packages/blog/
├── PIPELINE.md          # This file (status tracking)
├── src/content/
│   └── blog/            # Published articles
└── pipeline/
    ├── ideas/           # Stage: IDEA (one-pagers)
    ├── research/        # Stage: RESEARCH (gathered material)
    ├── outlines/        # Stage: OUTLINE (structured plans)
    └── drafts/          # Stage: DRAFT through POLISH
```

Artifacts move through directories as they advance. Published articles go to `src/content/blog/`.

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
