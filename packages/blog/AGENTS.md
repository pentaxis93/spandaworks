# @aiandi/blog

The communication service for aiandi - an Astro-based blog.

## Tech Stack

- Astro 5 with MDX support
- Tailwind CSS v4 with semantic token design system
- TypeScript
- Bun as package manager

## Content Collections

- `src/content/blog/` - Blog posts (Markdown/MDX)
- `src/content/transcripts/` - Session transcripts

## Project Structure

- `src/pages/` - Astro pages
- `src/layouts/` - Page layouts
- `src/components/` - Reusable components
- `src/styles/` - Global styles
- `design-system/` - Semantic token architecture
- `public/` - Static assets

## Commands

All commands run from `packages/blog/`:

```bash
bun install              # Install dependencies
bun run dev --host 0.0.0.0  # Start dev server
bun run build            # Build for production
bun run preview          # Preview production build
```

## Design System

See `DESIGN.md` for the semantic token architecture. Key principle: schema (token names) vs. theme (values) separation.
