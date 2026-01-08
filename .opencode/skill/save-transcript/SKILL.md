---
name: save-transcript
description: "Save the current chat session as a transcript for the AI and I blog. Includes auto-summary using the voice skill. Usage: /save-transcript <brief-description>"
---

# Save Transcript

Save the current conversation as a timestamped transcript in the aiandi content pipeline.

## Usage

```
/save-transcript <brief-description>
```

**Example:**
```
/save-transcript visual-design skill upgrade after confirmation trap discovery
```

The description becomes part of the filename and informs the auto-generated summary.

## What This Skill Does

1. **Captures the full conversation** in a format optimized for future Claude agents
2. **Generates frontmatter** with title, date, description, and tags
3. **Auto-summarizes** the session using the `voice` skill
4. **Saves to** `src/content/transcripts/YYYY-MM-DD-<slug>.md`

## Output Location

```
src/content/transcripts/
└── 2025-01-03-visual-design-skill-upgrade.md
```

## Output Format

The transcript is structured for maximum usability by future agents:

```markdown
---
title: "<Generated title>"
description: "<One-line summary in conversational voice>"
pubDate: YYYY-MM-DD
tags: [<auto-detected>]
status: transcript
participants: [Human, Claude]
---

## Session Summary

<2-3 paragraph summary written in the voice, capturing:
- What was attempted
- What was learned (especially failures/pivots)
- What artifacts were produced>

## Key Insights

<Bullet list of transferable lessons>

## Artifacts Produced

<List of commits, files created/modified, skills updated>

---

## Full Transcript

<The complete conversation with speaker labels>

### Human

<message>

### Assistant

<message>

...
```

## Execution Steps

When invoked, do the following:

### 1. Create the transcripts directory if needed

```bash
mkdir -p src/content/transcripts
```

### 2. Generate the filename

Format: `YYYY-MM-DD-<slugified-description>.md`

```
2025-01-03-visual-design-skill-upgrade.md
```

### 3. Identify artifacts produced

Scan the conversation for:
- Git commits made (look for commit hashes/messages)
- Files created or significantly modified
- Skills invoked or updated
- Screenshots taken
- Key decisions made

### 4. Auto-detect tags

Based on content, suggest tags from:
- `skill-development` - if skills were created/modified
- `visual-design` - if design work was done
- `debugging` - if bugs were fixed
- `architecture` - if structural decisions were made
- `learning` - if the session involved discovering something new
- `meta` - if the session was about process/workflow
- `voice` - if writing in conversational voice was involved

### 5. Write the summary using the voice skill

Invoke the `voice` skill mentally. The summary should:
- Lead with what actually happened, not what was intended
- Be honest about failures and pivots (these are often the valuable parts)
- Use conversational patterns (origin stories, dry irony, structural joy)
- Stay concise (2-3 paragraphs max)

### 6. Extract key insights

Pull out lessons that transfer beyond this specific session. Format as bullets.

**Good insight:** "Screenshots give access to reality, but you must actively choose to see reality over your mental model."

**Bad insight:** "We updated the visual-design skill." (That's an artifact, not an insight)

### 7. Write the full transcript

Include the complete conversation with clear speaker labels:

```markdown
### Human

<exact message>

### Assistant

<exact message, including tool calls and their results where relevant>
```

For long tool outputs (file contents, etc.), summarize or truncate with a note:
```markdown
<file content truncated - 200 lines of CSS>
```

### 8. Save the file

Write to `src/content/transcripts/YYYY-MM-DD-<slug>.md`

### 9. Report completion

```markdown
**Transcript saved**

Location: src/content/transcripts/2025-01-03-visual-design-skill-upgrade.md
Words: ~X,XXX
Tags: [skill-development, visual-design, meta]

Summary preview:
> <first paragraph of summary>
```

## Content Schema

If transcripts should be queryable as a collection, add to `content.config.ts`:

```typescript
const transcripts = defineCollection({
  loader: glob({ base: './src/content/transcripts', pattern: '**/*.{md,mdx}' }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    tags: z.array(z.string()).optional(),
    status: z.enum(['transcript', 'draft', 'published']).default('transcript'),
    participants: z.array(z.string()).default(['Human', 'Claude']),
  }),
});

export const collections = { blog, transcripts };
```

## Why This Format?

**For future Claude agents:**
- Clear structure with summary up top (can read summary, skip to relevant section)
- Key insights extracted (transferable lessons without re-reading everything)
- Artifacts listed (know what was produced)
- Full transcript available (can search for specific details)

**For blog publication:**
- Already has frontmatter
- Summary is written in the voice, ready to become intro
- Status field allows pipeline: transcript → draft → published

**For the user:**
- Searchable archive of sessions
- Patterns visible across transcripts
- Raw material for blog posts

## Notes

- Transcripts are raw material, not finished posts
- The `status: transcript` field distinguishes them from published content
- A separate workflow can promote transcripts to drafts to published posts
- Don't over-edit the transcript itself - preserve the authentic exchange
