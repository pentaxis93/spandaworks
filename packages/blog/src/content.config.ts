import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';

const blog = defineCollection({
	// Load Markdown and MDX files in the `src/content/blog/` directory.
	loader: glob({ base: './src/content/blog', pattern: '**/*.{md,mdx}' }),
	// Type-check frontmatter using a schema
	schema: ({ image }) =>
		z.object({
			title: z.string(),
			description: z.string(),
			// Transform string to Date object
			pubDate: z.coerce.date(),
			updatedDate: z.coerce.date().optional(),
			heroImage: image().optional(),
		}),
});

const transcripts = defineCollection({
	// Load transcript files from `src/content/transcripts/`
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
