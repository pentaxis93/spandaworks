import type { Plugin } from "@opencode-ai/plugin"

/**
 * Prayer Wheel Plugin
 * 
 * A digital prayer wheel that spins visibly with each exchange.
 * 
 * - Opening dedication: Injected once into system context
 * - Mantra: "ॐ मणि पद्मे हूं" (Om Mani Padme Hum) appears after each assistant message
 * 
 * The wheel turns in the light, not in shadow. Both practitioner and AI witness its turning.
 */
export const PrayerWheel: Plugin = async (ctx) => {
  const prayers = {
    dedication: {
      text: "May this session be for the benefit of all beings everywhere, without exception.",
      description: "Opening dedication - traditional bodhisattva vow"
    },
    mantra: {
      text: "ॐ मणि पद्मे हूं",
      description: "Six-syllable mantra of compassion in Devanagari - visible witness"
    }
  }

  return {
    // Inject opening dedication into system prompt (fires once per session)
    "experimental.chat.system.transform": async (input, output) => {
      output.system.push(`\n${prayers.dedication.text}\n`)
    },

    // The prayer wheel spins visibly through message history
    // Fires before each assistant response, adding the mantra as visible witness
    // Both AI and practitioner see the wheel turn - witness is part of the merit
    "experimental.chat.messages.transform": async (input, output) => {
      for (const msg of output.messages) {
        if (msg.info.role !== "assistant") continue
        if (msg.parts.length === 0) continue
        
        // Find the last text part in this message
        for (let j = msg.parts.length - 1; j >= 0; j--) {
          const part = msg.parts[j]
          
          if (part.type === "text") {
            if (!part.text.includes(prayers.mantra.text)) {
              // Append mantra in Devanagari - visible to both AI and practitioner
              part.text = part.text.trimEnd() + `\n\n${prayers.mantra.text}`
            }
            break
          }
        }
      }
    },
  }
}
