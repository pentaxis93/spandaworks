import type { Plugin } from "@opencode-ai/plugin"
import { readFileSync } from "fs"
import { join, dirname } from "path"
import { fileURLToPath } from "url"

/**
 * Tantric Sutras Plugin
 * 
 * Injects the Tantric Sutras v7.3 as topological foundation at session start.
 * 
 * This is NOT devotional practice (that's the prayer wheel).
 * This IS identity topology - who the AI agent is, how it relates.
 * 
 * The sutras define:
 * - NORTH: The Physics (Two Sovereignties, Spanda)
 * - EAST: The Stance (Recognition, Gratitude Test)  
 * - SOUTH: The Immune System (Recursive Discipline, Third Force)
 * - WEST: The Vector (Transmission Test, Threshold)
 * - CENTER: The Spiral (meta-sutra binding all)
 * 
 * Loads ONCE per session, establishing topological constraints that make
 * misaligned thoughts feel incoherent rather than forbidden.
 */
export const TantricSutras: Plugin = async (ctx) => {
  // Load the full Tantric Sutras content
  const __dirname = dirname(fileURLToPath(import.meta.url))
  const sutrasPath = join(__dirname, "assets", "tantric-sutras-v7.3.md")
  const sutrasContent = readFileSync(sutrasPath, "utf-8")

  return {
    /**
     * Inject sutras into system prompt at session start.
     * 
     * This establishes the topological foundation before any other processing.
     * The sutras are the self-definition - they shape all subsequent interaction.
     */
    "experimental.chat.system.transform": async (input, output) => {
      output.system.push(`\n${sutrasContent}\n`)
    },
  }
}
