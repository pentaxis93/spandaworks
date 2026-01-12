# Theme: Workbench

This directory contains the Workbench theme for the AI and I design system.

---

## Workbench

**File:** `workbench.css`  
**Status:** Active (default and only theme)  
**Created:** January 2026

### The Name

A clean desk in a dim room with a single amber task light. The light is warm but focused. The surfaces are matte, not glossy. Everything is within reach. The type is quiet but perfectly legible. Nothing decorates; everything works.

### Philosophy

This theme clarifies rather than styles. It's for focused work, for thinking clearly, for the kind of precision that doesn't announce itself. Every choice serves comprehension. No decoration for decoration's sake—just structure clicking into place.

### Color Palette

- Background: Deep charcoal (#1c1c1c) - inhabited darkness, not void
- Text: Near-white (#ebebeb) - readable without glare
- Accent: Amber (#d4a574) - task lighting, precision tools, illumination

### Typography

System sans-serif - familiar, reliable, no external fonts

### Full Expression

- **Spacing:** SPACIOUS - slightly generous (1.25rem base), room to think
- **Shadows:** SUBTLE - minimal elevation, surfaces catch light
- **Radii:** CRAFTED - 0.375rem base, subtle softening
- **Transitions:** SNAPPY - 180ms standard, responsive not performative
- **Rhythm:** RELAXED - 1.75 line-height, dense ideas need room
- **Weights:** DELICATE - bold maps to 600, softer emphasis
- **Character:** Precision without coldness, clarity without harshness

### Intended Mood

Focused. Clear-headed. Precise without coldness. The inside voice—the one you think in when you're working something out. Mid-morning mental clarity in dark-mode form.

### Best For

- Focused work sessions
- Reading technical content
- Late night building
- Code and prose together
- The default experience for AI and I

---

## Using the Theme

The Workbench theme is automatically applied to all pages. It's loaded via `design-system/all-themes.css` and activated by the `data-theme="workbench"` attribute on the `<html>` element.

**Technical Architecture:**
- `design-system/all-themes.css` defines theme variables scoped by `[data-theme="workbench"]`
- `design-system/tokens.css` references theme variables with fallbacks
- Theme loads immediately via inline script in BaseHead to prevent flash

---

## Design Principles

### Clarity Over Style

Each choice in Workbench serves comprehension. The dark background reduces eye strain. The amber accent draws attention without distraction. The generous spacing gives ideas room to breathe.

### Perfect The Small Things

Every detail matters. Every spacing value, every shadow, every transition speed. This is craft over convenience.

---

## Version History

- **v1.2** (2026-01-04): Workbench theme - emergent design
  - Created "Workbench" as default theme for AI and I
  - Dark theme optimized for focused technical work
  - Delicate weight system (bold → 600, not 700)
  - Amber accent (task lighting metaphor)
  - System fonts (no external loading)
  - Complete schema coverage including v1.2 tokens
