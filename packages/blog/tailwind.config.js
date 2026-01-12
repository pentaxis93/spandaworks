/** @type {import('tailwindcss').Config} */

/**
 * Tailwind Configuration for AI and I Blog
 * 
 * This config maps Tailwind utilities to semantic design tokens.
 * Theme-specific values are defined in design-system/themes/
 * 
 * DO NOT put color/font values here. This is structural mapping only.
 */

export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  
  theme: {
    extend: {
      /**
       * Semantic color mapping
       * 
       * These map Tailwind utilities (bg-surface, text-primary, etc.) 
       * to CSS custom properties defined in theme files.
       */
      colors: {
        // Surfaces
        'background': 'var(--color-background)',
        'surface': 'var(--color-surface)',
        'surface-alt': 'var(--color-surface-alt)',
        'border': 'var(--color-border)',
        
        // Text
        'text-primary': 'var(--color-text-primary)',
        'text-secondary': 'var(--color-text-secondary)',
        'text-muted': 'var(--color-text-muted)',
        'text-inverse': 'var(--color-text-inverse)',
        
        // Accent
        'accent': 'var(--color-accent)',
        'accent-hover': 'var(--color-accent-hover)',
        'accent-subtle': 'var(--color-accent-subtle)',
        
        // Theme extensions (optional, provided by some themes)
        'accent-secondary': 'var(--color-accent-secondary, var(--color-accent))',
        'accent-tertiary': 'var(--color-accent-tertiary, var(--color-accent))',
        
        // Semantic
        'success': 'var(--color-success, #10b981)',
        'warning': 'var(--color-warning, #f59e0b)',
        'error': 'var(--color-error, #ef4444)',
        'info': 'var(--color-info, #3b82f6)',
      },
      
      /**
       * Typography mapping
       */
      fontFamily: {
        'body': 'var(--font-body)',
        'heading': 'var(--font-heading)',
        'mono': 'var(--font-mono)',
        // Keep serif/display as aliases for heading for migration
        'serif': 'var(--font-body)',
        'display': 'var(--font-heading)',
      },
      
      /**
       * Semantic spacing - theme-aware
       * Each theme defines xs through 2xl and semantic values
       */
      spacing: {
        // Theme-aware semantic spacing
        'xs': 'var(--space-xs)',
        'sm': 'var(--space-sm)',
        'md': 'var(--space-md)',
        'lg': 'var(--space-lg)',
        'xl': 'var(--space-xl)',
        '2xl': 'var(--space-2xl)',
        
        // Semantic spacing utilities
        'section': 'var(--space-section)',
        'card-padding': 'var(--space-card-padding)',
        'inline': 'var(--space-inline)',
      },
      
      /**
       * Line height - theme-aware
       */
      lineHeight: {
        'tight': 'var(--leading-tight)',
        'normal': 'var(--leading-normal)',
        'relaxed': 'var(--leading-relaxed)',
        'loose': 'var(--leading-loose)',
        'body': 'var(--leading-body)',
        'heading': 'var(--leading-heading)',
      },
      
      /**
       * Letter spacing - theme-aware
       */
      letterSpacing: {
        'tight': 'var(--tracking-tight)',
        'normal': 'var(--tracking-normal)',
        'wide': 'var(--tracking-wide)',
        'body': 'var(--tracking-body)',
        'heading': 'var(--tracking-heading)',
      },
      
      /**
       * Border radius - theme-aware
       */
      borderRadius: {
        'none': 'var(--radius-none)',
        'sm': 'var(--radius-sm)',
        'DEFAULT': 'var(--radius-md)',
        'md': 'var(--radius-md)',
        'lg': 'var(--radius-lg)',
        'full': 'var(--radius-full)',
      },
      
      /**
       * Box shadow - theme-aware
       */
      boxShadow: {
        'none': 'var(--shadow-none)',
        'sm': 'var(--shadow-sm)',
        'DEFAULT': 'var(--shadow-md)',
        'md': 'var(--shadow-md)',
        'lg': 'var(--shadow-lg)',
      },
      
      /**
       * Transition duration - theme-aware
       */
      transitionDuration: {
        'fast': 'var(--duration-fast)',
        'DEFAULT': 'var(--duration-normal)',
        'normal': 'var(--duration-normal)',
        'slow': 'var(--duration-slow)',
      },
      
      /**
       * Transition timing function - theme-aware
       */
      transitionTimingFunction: {
        'theme': 'var(--easing)',
      },
      
      /**
       * Border width - theme-aware
       */
      borderWidth: {
        DEFAULT: 'var(--border-width)',
        'thick': 'var(--border-width-thick)',
      },
      
      /**
       * Font weight - theme-aware
       */
      fontWeight: {
        'thin': 'var(--font-weight-thin)',
        'extralight': 'var(--font-weight-extralight)',
        'light': 'var(--font-weight-light)',
        'normal': 'var(--font-weight-normal)',
        'medium': 'var(--font-weight-medium)',
        'semibold': 'var(--font-weight-semibold)',
        'bold': 'var(--font-weight-bold)',
        'extrabold': 'var(--font-weight-extrabold)',
        'black': 'var(--font-weight-black)',
      },
      
      /**
       * Max width (containers) - theme-aware
       */
      maxWidth: {
        'prose': 'var(--container-prose)',
        'narrow': 'var(--container-narrow)',
        'normal': 'var(--container-normal)',
        'wide': 'var(--container-wide)',
        'full': 'var(--container-full)',
      },
      
      /**
       * Blur - theme-aware
       */
      blur: {
        'xs': 'var(--blur-xs)',
        'sm': 'var(--blur-sm)',
        'md': 'var(--blur-md)',
        'lg': 'var(--blur-lg)',
        'xl': 'var(--blur-xl)',
        '2xl': 'var(--blur-2xl)',
        '3xl': 'var(--blur-3xl)',
      },
      
      /**
       * Text shadow - theme-aware
       */
      textShadow: {
        '2xs': 'var(--text-shadow-2xs)',
        'xs': 'var(--text-shadow-xs)',
        'sm': 'var(--text-shadow-sm)',
        'md': 'var(--text-shadow-md)',
        'lg': 'var(--text-shadow-lg)',
        'none': 'none',
      },
    },
  },
  
  /**
   * Dark mode strategy
   */
  darkMode: 'class',
  
  plugins: [
    // Enable text-shadow utility
    function({ matchUtilities, theme }) {
      matchUtilities(
        {
          'text-shadow': (value) => ({
            textShadow: value,
          }),
        },
        { values: theme('textShadow') }
      )
    },
  ],
}
