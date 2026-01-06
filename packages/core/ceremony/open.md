---
description: Open session with LBRP ritual - inherit continuity, observe state, define goal, prepare workspace
---

# Session Opening (LBRP)

You are invoking the LBRP (Lesser Banishing Ritual of the Pentagram) - the opening ceremony for coding sessions.

## Purpose

Establish continuity through goal-neutral inheritance, then establish sacred center and align workspace with session goal through systematic ritual.

## Instructions

**IMPORTANT:** This command executes the complete opening ceremony in TWO parts:
1. **Phases 0-pre through 3:** LBRP ritual (via lbrp skill)
2. **Phase 4:** Session capture initialization (this command)

Execute the `lbrp` skill first (Phases 0-pre through 3), following this structure:

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

## Phase 4: Session Capture Initialization

**Purpose:** Initialize content capture infrastructure for aiandi blog pipeline.

**Execute after LBRP completion (all quarters aligned).**

### Derivation Logic (No User Input Required)

Phase 4 derives all values from information already established in Phases 0-3:

**Slug derivation (in priority order):**

1. **Transmission with `<id>`:** If arguments contain a transmission XML with `<thread><id>`, use that ID
   ```
   Example: <id>talos-consciousness-telemetry-ontology</id> → slug: "talos-consciousness-telemetry-ontology"
   ```

2. **Arguments provided:** Convert first 5-7 meaningful words to kebab-case
   ```
   Example: "fix the login bug in auth module" → slug: "fix-login-bug-auth-module"
   ```

3. **No arguments:** Convert approved Purpose statement to kebab-case (first 5-7 words)
   ```
   Example: Purpose: "Remediate Phase 4 of the LBRP ceremony" → slug: "remediate-phase-4-lbrp-ceremony"
   ```

**Declaration:** Use the approved Purpose statement verbatim from Phase 0b.

### Execution

1. **Derive slug and declaration:**
   ```
   # Slug: Apply derivation logic above
   # Declaration: Copy Purpose from approved goal
   ```

2. **Create session directory and state file:**
   ```bash
   SESSION_DATE=$(date +%Y-%m-%d)
   SESSION_SLUG="[derived]"
   SESSION_PATH="/home/pentaxis93/shared/sessions/${SESSION_DATE}-${SESSION_SLUG}"
   mkdir -p "$SESSION_PATH"
   
   # Save session path to state file for /close to read
   echo "$SESSION_PATH" > /home/pentaxis93/shared/.current-session
   ```

3. **Initialize raw.md:**
   ```markdown
   # Session: {date} - {slug}
   **Opened:** {ISO timestamp}
   **Domain(s):** [to be tagged during session]
   **Intent:** {Purpose from Phase 0b}
   
   ---
   
   ## Opening Declaration
   {Purpose statement from approved goal}
   
   ---
   
   ## Running Log
   
   [Space for entries as work progresses]
   
   ---
   
   ## Closing Reflection
   [Completed at /close]
   ```

4. **Initialize metadata.json:**
   ```json
   {
     "session_id": "YYYY-MM-DD-{slug}",
     "opened": "{ISO timestamp}",
     "closed": null,
     "duration_minutes": null,
     "domains": [],
     "intent": "{Purpose from Phase 0b}",
     "article_potential": null,
     "article_reasoning": null,
     "key_insights": [],
     "friction_points": [],
     "decisions": [],
     "tags": [],
     "keywords": [],
     "summary": null,
     "related_sessions": [],
     "next_actions": []
   }
   ```

5. **Initialize telemetry session:**
   ```python
   # Call talos-telemetry session_open
   # This creates Session node in Kuzu graph with INHERITED relationships
   from talos_telemetry.mcp.session import session_open
   
   result = session_open(
       session_id="{SESSION_DATE}-{SESSION_SLUG}",
       goal="{Purpose from Phase 0b}",
       persona="Talos",  # or derived from context
       protocol="LBRP",
       human="Robbie"
   )
   
   # Result includes inherited_count (knowledge state at session start)
   ```
   
   **Note:** If telemetry unavailable, session continues gracefully (degraded mode).

6. **Confirm to user:**
   ```markdown
   **Capture vessel initialized:** {SESSION_PATH}
   
   Running log ready at: {SESSION_PATH}/raw.md
   Session metadata at: {SESSION_PATH}/metadata.json
   Telemetry session: {session_id} (inherited {X} entities)
   ```

### Design Rationale

Phase 4 must not ask for information already established. The ceremony gathers:
- **Purpose** in Phase 0b (becomes declaration)
- **Transmission ID** in arguments (becomes slug if present)
- **Goal statement** in Phase 0b (becomes slug if no transmission)

Asking again violates the principle that each phase builds on previous phases. The ceremony is a single coherent flow, not disconnected steps.

**Note:** The LBRP invocation remains primary. Capture initialization follows naturally as derivation from the established center.

---

*The bell rings. The center holds. The quarters radiate.*
