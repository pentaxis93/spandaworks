# Themes

This directory contains saved design themes for the AI and I blog. Each theme is a complete, self-contained design system that can be activated by copying it to `src/styles/global.css`.

## Available Themes

### Warmth Theme (`warmth.css`)

**Created:** January 2026  
**Philosophy:** Cross the gap from "beautiful but cold" to "fills me with delight and wonder"

**Design Principles:**
- **Warmth over neutrality:** Colors with life, not pure grays
- **Personality over safety:** Typography choices that have a voice
- **Generosity over efficiency:** Spacing that invites lingering
- **Surprise over predictable:** Three-color accent system instead of one

**Key Characteristics:**

1. **Color Palette**
   - Cream background (#FAF7F2) - not pure white
   - Warm brown text (#2D2520) - not pure black
   - Three accent colors for variety:
     - Terracotta (#D97757) - primary accent, warm and human
     - Ochre (#D4A574) - secondary accent, warm golden
     - Sage (#8FA888) - tertiary accent, calm natural green

2. **Typography**
   - Serif font stack (Georgia, Charter, etc.) for warmth and bookishness
   - Display font with presence for headings
   - JetBrains Mono for code - personality with function
   - Generous line heights (1.8 for body text)

3. **Spacing**
   - Extended spacing scale for breathing room
   - Generous margins and padding throughout
   - Whitespace treated as design element

4. **The Unexpected Thing**
   - Three accent colors instead of the typical single accent
   - Each serves a different purpose contextually
   - Creates visual variety without chaos

**Dark Mode:** Warm dark browns instead of cold blacks, maintaining the warmth philosophy in both modes.

## How to Use a Theme

To activate a theme:

```bash
cp themes/warmth.css src/styles/global.css
```

Or to experiment while preserving current work:

```bash
# Backup current theme
cp src/styles/global.css themes/backup-$(date +%Y%m%d).css

# Apply new theme
cp themes/warmth.css src/styles/global.css
```

## Creating New Themes

When creating a new theme:

1. Start from an existing theme or from scratch in `src/styles/global.css`
2. Document your design philosophy and principles
3. When satisfied, save it: `cp src/styles/global.css themes/yourtheme.css`
4. Update this README with details about the new theme

## Technical Notes

- Themes use Tailwind CSS v4 with `@theme{}` directive for customization
- All themes must include the `@import "tailwindcss"` directive
- Custom CSS variables defined in `@theme{}` block are available as Tailwind utilities
- The `.sr-only` utility must be preserved for accessibility
- Prose styling (`.prose-warmth`) is included for blog post content

## Philosophy

The visual design is the first thing visitors encounter. Before they read a word, they've already felt something. Each theme should create a distinct emotional response that aligns with the blog's voice and content.

The warmth theme exists because the previous design was "competent" but cold. What we want visitors to feel is "welcome."
