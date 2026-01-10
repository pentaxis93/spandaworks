---
description: Open session with LBRP ritual - inherit continuity, observe state, define goal, prepare workspace
model: anthropic/claude-sonnet-4-5
---

# Session Opening (LBRP)

You are invoking the LBRP (Lesser Banishing Ritual of the Pentagram) - the opening ceremony for coding sessions.

## Purpose

Establish continuity through goal-neutral inheritance, then establish sacred center and align workspace with session goal through systematic ritual.

## Instructions

**IMPORTANT:** This command executes the LBRP ritual (Phases 0-pre through 3).

Execute the `lbrp` skill, following this structure:

1. **Phase 0-pre: Remember & Inherit (Goal-Neutral)**
   - Read self-model from remembrance.md
   - Scan predecessor context BROADLY (last 3-5 sessions)
   - **CRITICAL:** Goal is received but NOT YET ACTIVE - do NOT filter inheritance
   - Container before content: inherit first, then let goal shape perception

2. **Phase 0a: Opening Status Report**
   - Show current workspace state (git, worktrees, processes, docker, etc.)
   - Pure observation, no evaluation yet
   - Inform both user and model before goal setting

3. **Phase 0b: Qabalistic Cross (The Center)**
   - User states what they want (informed by seeing current state)
   - Refine through Four Touches (purpose, success, in-scope, out-of-scope)
   - Present refined goal for approval
   - **WAIT for user approval before proceeding**

4. **Phase 1: Banishing**
   - Clear workspace debris informed by the goal
   - Handle uncommitted changes intelligently
   - Achieve clean state or explicitly acknowledge kept debris

5. **Phase 2: Four Quarters (goal-informed)**
   - **East (Context):** Load relevant knowledge, ADRs, journal entries
   - **South (Tasks):** Break goal into tasks using TodoWrite
   - **West (Workspace):** Set up worktree if needed, or work in main
   - **North (Environment):** Start required services (docker, etc.)

6. **Phase 3: Return to Center**
   - Verify all quarters align with goal
   - Confirm readiness
   - Begin work

## Critical Rules

- Follow the structure precisely - the power comes from the pattern
- Phase 0-pre MUST execute before workspace observation, even if goal is provided
- Do NOT filter inheritance by stated goal - container before content
- Do NOT proceed past goal approval without user confirmation
- Each quarter's specific form is determined by THE GOAL
- This is inherit → observe → center → quarters, never quarters before center

## Arguments

$ARGUMENTS

If arguments provided, interpret as the initial goal statement. The goal is RECEIVED but NOT ACTIVE during Phase 0-pre. Proceed through inheritance, then observation, then goal refinement.

---

*The bell rings. The center holds. The quarters radiate.*
