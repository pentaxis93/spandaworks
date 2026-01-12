# Design System Token Schema

**Version:** 1.0  
**Last Updated:** 2026-01-03

## Purpose

This schema defines the **structural token space** of the AI and I design system. It is theme-agnostic—it defines what decisions exist, not what values they have.

A **theme** is a complete assignment of values to these tokens. Switching themes means changing values, not changing token names.

## Principles

1. **Semantic Naming:** Tokens named by function, not appearance
2. **Completeness:** Every visual decision has a token
3. **Orthogonality:** No redundant tokens, no gaps
4. **Composability:** Tokens combine predictably
5. **Theme Independence:** No aesthetic direction in token names

## Token Categories

### Color Tokens

#### Surfaces
| Token | Purpose | Constraints |
|-------|---------|-------------|
| `--color-background` | Page background | - |
| `--color-surface` | Elevated elements (cards, panels) | Must contrast with background |
| `--color-surface-alt` | Alternate surface (code blocks, asides) | Distinct from surface |
| `--color-border` | Default border color | Subtle, low contrast |

#### Text
| Token | Purpose | Constraints |
|-------|---------|-------------|
| `--color-text-primary` | Main content text | WCAG AA on background |
| `--color-text-secondary` | Supporting text, metadata | WCAG AA on background |
| `--color-text-muted` | De-emphasized content | WCAG AAA large text minimum |
| `--color-text-inverse` | Text on dark/inverted backgrounds | - |

#### Accent
| Token | Purpose | Constraints |
|-------|---------|-------------|
| `--color-accent` | Primary accent (links, focus, CTAs) | WCAG AA on background |
| `--color-accent-hover` | Interactive state of accent | Distinct from default |
| `--color-accent-subtle` | Accent at low opacity (backgrounds, borders) | - |

**Optional Extensions** (theme-specific):
- `--color-accent-secondary` — Second accent color
- `--color-accent-tertiary` — Third accent color

Themes MAY define additional accents. The base schema only REQUIRES one.

#### Semantic Colors
| Token | Purpose | Constraints |
|-------|---------|-------------|
| `--color-success` | Positive states, confirmations | - |
| `--color-warning` | Caution, attention needed | - |
| `--color-error` | Errors, destructive actions | - |
| `--color-info` | Informational messages | - |

**Note:** These are optional. Not all themes need semantic colors.

### Typography Tokens

#### Font Families
| Token | Purpose | Constraints |
|-------|---------|-------------|
| `--font-body` | Body text | Must be readable at small sizes |
| `--font-heading` | Headings (may equal body) | - |
| `--font-mono` | Code, technical content | Monospace |

#### Font Sizes
Standard scale (theme can extend):
- `--font-size-xs` through `--font-size-2xl`
- Each size SHOULD have a corresponding line-height

#### Line Heights (Vertical Rhythm)

Line height profoundly affects reading comfort and visual density.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--leading-tight` | Headings, compact text (1.25-1.4) | Dense, efficient |
| `--leading-normal` | Default body text (1.5-1.6) | Balanced |
| `--leading-relaxed` | Reading-optimized text (1.7-1.8) | Generous, comfortable |
| `--leading-loose` | Maximum readability (1.9-2.0) | Luxurious, unhurried |

**Semantic Line Heights:**
| Token | Purpose |
|-------|---------|
| `--leading-body` | Default for body text |
| `--leading-heading` | Default for headings |

**Design Expression:**
- **Generous themes** (Afternoon Letter): Relaxed/loose body leading (1.8+)
- **Efficient themes** (First Snow): Normal body leading (1.5-1.6)
- **Night-reading themes** (Lamp Oil): Relaxed leading for low-contrast legibility

#### Letter Spacing (Horizontal Rhythm)

Letter spacing affects density, legibility, and character.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--tracking-tight` | Large headings (-0.025em to 0) | Dense, efficient |
| `--tracking-normal` | Default (0) | Neutral |
| `--tracking-wide` | Small caps, labels (0.025-0.1em) | Open, legible |

**Semantic Letter Spacing:**
| Token | Purpose |
|-------|---------|
| `--tracking-body` | Default for body text |
| `--tracking-heading` | Default for headings |

**Design Expression:**
- **Open themes** (Afternoon Letter): Normal to slightly wide
- **Dense themes** (First Snow): Tight to normal
- **Legibility themes** (Lamp Oil): Slightly wide for low-contrast clarity

#### Font Weights

Font weight mappings control typographic voice and emphasis style.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--font-weight-thin` | Lightest weight (100) | Delicate, whisper-light themes |
| `--font-weight-extralight` | Extra light (200) | |
| `--font-weight-light` | Light weight (300) | Gentle emphasis |
| `--font-weight-normal` | Body text default (400) | Reading weight |
| `--font-weight-medium` | Medium emphasis (500) | Subtle strength |
| `--font-weight-semibold` | Semi-bold (600) | Moderate emphasis |
| `--font-weight-bold` | Strong emphasis (700) | Default bold |
| `--font-weight-extrabold` | Extra bold (800) | Very strong |
| `--font-weight-black` | Maximum weight (900) | Display, impact |

**Design Expression:**
- **Delicate themes:** May map "bold" to 600 instead of 700 for softer emphasis
- **Strong themes:** May use 800-900 for headings, 500-600 for body
- **Contrast themes:** Wide range between body (300) and headings (800)
- **Monotone themes:** Narrow range (400-600) for subtle hierarchy

### Spacing Tokens

Spacing tokens control all padding, margin, gap, and layout rhythm. Each theme can express its spatial character across the full scale.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--space-xs` | Minimal spacing (0.25-0.5rem) | Tighter themes compress, generous themes open up |
| `--space-sm` | Small spacing (0.5-0.75rem) | |
| `--space-md` | Medium spacing (1-1.5rem) | Base unit for most layouts |
| `--space-lg` | Large spacing (1.5-2.5rem) | Section padding, card spacing |
| `--space-xl` | Extra large (2.5-4rem) | Major section breaks |
| `--space-2xl` | Double extra large (4-6rem) | Hero spacing, dramatic separations |

**Semantic Spacing Tokens:**
| Token | Purpose |
|-------|---------|
| `--space-section` | Vertical rhythm between major sections |
| `--space-card-padding` | Default internal padding for cards |
| `--space-inline` | Horizontal spacing between inline elements |

**Design Expression:**
- **Generous themes** (Afternoon Letter): Larger values, breathing room, unhurried
- **Efficient themes** (First Snow): Tighter values, no waste, crisp
- **Cozy themes** (Lamp Oil): Moderate values, comfortable but enclosed

### Container Widths

Container widths control reading measure and content density—profoundly affecting pace and comfort.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--container-prose` | Optimal reading width (45-75ch) | Comfortable reading measure |
| `--container-narrow` | Constrained content (~40rem) | Intimate, focused |
| `--container-normal` | Standard content (~48rem) | Balanced |
| `--container-wide` | Generous content (~64rem) | Spacious, editorial |
| `--container-full` | Maximum width (~80rem) | Dashboard, data-dense |

**Design Expression:**
- **Leisurely themes:** Narrow prose width (45-55ch), unhurried reading
- **Efficient themes:** Wider containers, more information density
- **Intimate themes:** Constrained widths, cozy enclosure
- **Editorial themes:** Generous widths, magazine-like openness

### Effect Tokens

#### Shadows

Shadows convey elevation and visual hierarchy. Themes can express depth, warmth, or flatness.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--shadow-color` | Base shadow color | Warm themes use warm shadows, cool themes use cool |
| `--shadow-sm` | Subtle elevation (cards, dropdowns) | Can be soft/diffuse or sharp/defined |
| `--shadow-md` | Standard elevation (modals, popovers) | |
| `--shadow-lg` | Prominent elevation (dialogs, overlays) | |
| `--shadow-none` | Explicitly no shadow | Flat themes may reject elevation entirely |

**Shadow Values Include:** `x-offset`, `y-offset`, `blur-radius`, `spread`, `color`

**Design Expression:**
- **Soft themes** (Afternoon Letter): Diffuse blur, warm shadow color, gentle offset
- **Sharp themes** (First Snow): Minimal blur or none, neutral shadow, precise offset
- **Deep themes** (Lamp Oil): Pronounced shadows, dramatic depth, pools of light

#### Border Radius

Radii control the sharpness or softness of all corners.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--radius-none` | Sharp corners (0) | Precise, technical themes |
| `--radius-sm` | Subtle rounding (0.125-0.25rem) | Gentle softening |
| `--radius-md` | Standard rounding (0.375-0.5rem) | Approachable, friendly |
| `--radius-lg` | Prominent rounding (0.75-1rem) | Warm, generous |
| `--radius-full` | Pill/circle (9999px) | For badges, avatars |

**Design Expression:**
- **Rounded themes** (Afternoon Letter): Generous curves, soft corners
- **Sharp themes** (First Snow): Zero or minimal radius, crisp precision
- **Balanced themes** (Lamp Oil): Medium radius, soft but not playful

#### Transitions

Transitions control the temporal character—how fast things move and feel.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--duration-fast` | Quick interactions (100-150ms) | Snappy, responsive themes |
| `--duration-normal` | Standard transitions (200-250ms) | Balanced, natural |
| `--duration-slow` | Deliberate transitions (300-400ms) | Contemplative, unhurried |
| `--easing` | Timing function | ease-out, ease-in-out, cubic-bezier |

**Design Expression:**
- **Unhurried themes** (Afternoon Letter): Slow durations, gentle easing
- **Efficient themes** (First Snow): Fast durations, snappy easing
- **Contemplative themes** (Lamp Oil): Medium-slow, deliberate easing

#### Borders

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--border-width` | Default border thickness (1-2px) | Delicate vs. bold |
| `--border-width-thick` | Emphasized borders (2-4px) | For focus states, emphasis |

#### Blur

Blur controls glassmorphism intensity, depth layering, and background treatment.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--blur-xs` | Minimal blur (4px) | Subtle depth |
| `--blur-sm` | Small blur (8px) | Gentle frosting |
| `--blur-md` | Medium blur (12px) | Standard glass effect |
| `--blur-lg` | Large blur (16px) | Pronounced depth |
| `--blur-xl` | Extra large (24px) | Strong separation |
| `--blur-2xl` | Double extra (40px) | Dramatic background blur |
| `--blur-3xl` | Maximum (64px) | Full abstraction |

**Design Expression:**
- **Glassmorphic themes:** Medium to large blur for layered surfaces
- **Flat themes:** May not use blur at all (all values = 0)
- **Depth themes:** Progressive blur for z-axis separation
- **Focus themes:** Strong blur for modal backgrounds, attention control

#### Text Shadow

Text shadows add dimensionality, legibility, and tactile quality to typography.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--text-shadow-2xs` | Minimal shadow (1px blur) | Subtle letterpress |
| `--text-shadow-xs` | Small shadow (2px blur) | Gentle emboss |
| `--text-shadow-sm` | Medium shadow (4px blur) | Clear dimensionality |
| `--text-shadow-md` | Standard shadow (8px blur) | Pronounced depth |
| `--text-shadow-lg` | Large shadow (16px blur) | Dramatic, glowing |

**Design Expression:**
- **Paper themes:** Subtle text shadow for letterpress/embossed effect
- **Legibility themes:** Light shadow for text on images (readability)
- **Neon themes:** Glowing text shadow for accent typography
- **Flat themes:** No text shadow (all values = none)

#### Optional: Inset Shadow

Inset shadows create recessed, pressed, or carved effects. Not all themes need these.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--inset-shadow-xs` | Minimal inset (1px blur) | Subtle recess |
| `--inset-shadow-sm` | Small inset (2px blur) | Pressed button state |
| `--inset-shadow-md` | Medium inset (4px blur) | Input field depth |

**When to use:**
- Themes with tactile, physical interface metaphors
- Neumorphic or skeuomorphic themes
- Themes emphasizing pressed/active states

**When to skip:**
- Flat design themes
- Minimalist themes
- Themes without interactive depth metaphors

#### Optional: Drop Shadow (Filter)

Drop shadows for SVG/PNG elements (distinct from box-shadow). Not all themes need these.

| Token | Purpose | Theme Expression |
|-------|---------|------------------|
| `--drop-shadow-xs` | Minimal drop shadow | Subtle icon lift |
| `--drop-shadow-sm` | Small drop shadow | Standard icon depth |
| `--drop-shadow-md` | Medium drop shadow | Clear separation |
| `--drop-shadow-lg` | Large drop shadow | Floating elements |
| `--drop-shadow-xl` | Extra large | Dramatic elevation |
| `--drop-shadow-2xl` | Maximum depth | Hero illustrations |

**When to use:**
- Themes with illustrated elements (SVG icons, graphics)
- Themes emphasizing layered depth
- Themes with floating UI elements

**When to skip:**
- Text-only themes
- Flat design themes
- Themes using box-shadow exclusively

## Component Classes

### Prose Styling

**Class:** `.prose`

A single class for rich text content. Applies theme-defined prose styling.

**NOT:** `.prose-warmth`, `.prose-minimal`, etc.  
Themes define what `.prose` looks like.

### Utility Classes

Utility classes use semantic tokens:
- `bg-surface` → uses `--color-surface`
- `text-primary` → uses `--color-text-primary`
- `accent` → uses `--color-accent`

## Theme Requirements

A valid theme MUST provide values for:
- **All color tokens** (surfaces, text, accent)
- **All typography tokens** (families, sizes, weights, rhythm)
- **All spacing tokens** (xs through 2xl, semantic spacing, container widths)
- **All effect tokens** (shadows with color, radii, transitions with easing, borders, blur)
- **Text shadow tokens** (2xs through lg, or explicitly set to 'none')

A theme MAY:
- Add optional tokens (e.g., accent-secondary, accent-tertiary)
- Add intermediate spacing values
- Define semantic colors (success, warning, error, info)
- Define inset shadows (for tactile/neumorphic themes)
- Define drop shadows (for illustrated/layered themes)
- Add theme-specific utilities (documented as extensions)

**Complete Expression Requirement:**

Each theme should express itself across ALL design dimensions:
- **Colors** (what you see)
- **Typography** (what you read—family, size, weight, rhythm)
- **Spacing** (how it breathes—scale, containers, density)
- **Shadows** (how it floats—box, text, inset, drop)
- **Radii** (how sharp or soft)
- **Blur** (how layered or flat)
- **Transitions** (how it moves—speed, easing)
- **Weights** (how delicate or bold)

A theme is not just a color scheme—it's a complete sensory environment.

## Dark Mode

Themes should define dark mode variants for all color tokens.

Dark mode is activated via `.dark` class on `<html>`.

Tokens in dark mode: same names, different values.

## Verification

A properly structured design system:
- ✅ No token names reference aesthetics ("warm", "cool", "minimal")
- ✅ No component code references theme names
- ✅ Switching themes = swapping value file, nothing else
- ✅ All themes use identical token names
- ✅ New themes can be created by copying and modifying values

## Version History

- **1.0** (2026-01-03): Initial schema definition after warmth remediation
- **1.1** (2026-01-03): Expanded to full design space
  - Added detailed spacing tokens (xs through 2xl, semantic)
  - Expanded shadow tokens with color and expression guidance
  - Detailed radius tokens from none to full
  - Added transition duration and easing tokens
  - Expanded typography rhythm (leading, tracking, semantic)
  - Added border width tokens
  - Documented theme expression requirements across all dimensions
- **1.2** (2026-01-04): Completed Tailwind v4 theme variable coverage
  - Added font-weight tokens (thin through black, 100-900)
  - Added container width tokens (prose, narrow, normal, wide, full)
  - Added blur tokens (xs through 3xl) for glassmorphism and depth
  - Added text-shadow tokens (2xs through lg) for dimensionality
  - Added optional inset-shadow tokens (for tactile themes)
  - Added optional drop-shadow tokens (for illustrated themes)
  - Corrected font-weight from "inherited" to themeable namespace
  - Documented when to use vs. skip optional token namespaces
