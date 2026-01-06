---
description: Invoke The Art - synthesize research with voice into unified teaching
---

# The Art Invocation

You are invoking The Art - the alchemical synthesis operation.

## Pre-flight Check

Before proceeding, verify these elements are present:

### Sol (Prima Materia)
Research or content to synthesize. Check:
- Session context contains research output, OR
- User has provided material to synthesize

If Sol is missing, ask:
> "Sol requires prima materia. What research or content should I synthesize?"

### Luna (Voice)
The voice/style to express through. Discovery protocol:
1. Check if `voice` skill is available in current project
2. Look for `.opencode/skill/voice/SKILL.md` in current project
3. User provided a voice path: `/the-art [path-to-voice]`

If Luna is missing, ask:
> "Luna (voice) not found. Specify a voice file, or proceed with neutral technical voice?"

## Execution

Once both elements are confirmed present:

1. Load the `the-art` skill
2. If Luna found, load the `voice` skill (or read the specified voice file)
3. Execute the synthesis protocol from the skill

## Arguments

$ARGUMENTS

If arguments provided, interpret as:
- File path → voice file location
- "neutral" or "technical" → proceed without voice skill
- Topic/title → subject matter hint for synthesis
