# Brand Identity Specification: Cenote

This document outlines the comprehensive visual identity of the **Cenote** brand. It is designed to serve as a foundational context for generating similar branding systems for new applications, maintaining the balance between elegance and modern functionality.

## 1. Brand Essence & Personality
Cenote is a "Modern Heritage" brand. It juxtaposes the organic, deep-rooted elegance of a natural cenote with the precision and clarity of modern design. 
- **Keywords**: Depth, Clarity, Sophistication, Natural, Modernity, Versatility.
- **Atmosphere**: Professional yet approachable, blending high-end editorial aesthetics with tech-forward usability.

## 2. Typography (The Dual-Core System)
The brand utilizes a strategic pairing of a serif and a sans-serif typeface to bridge the gap between tradition and innovation. Both typefaces are SIL Open Font Licensed and served from the Google Fonts CDN (`fonts.googleapis.com`), so they render correctly in any web view Claude or a browser produces — no local file upload required.

### Primary Serif: EB Garamond
- **Role**: Headlines, editorial copy, and high-impact branding.
- **Characteristics**: A direct Garamond derivative with high contrast, generous counters, and classic humanist proportions. Conveys luxury, history, and organic beauty.
- **Weights Used**: Regular (400), Medium (500), SemiBold (600), Bold (700), ExtraBold (800) — all with italics.
- **Source**: https://fonts.google.com/specimen/EB+Garamond
- **Fallback stack**: `'EB Garamond', Georgia, 'Times New Roman', serif`
- **Vibe**: Classic, sophisticated, "Old World" intelligence.

### Primary Sans-Serif: Manrope
- **Role**: Interface elements, body text, and functional signage.
- **Characteristics**: A geometric sans-serif with gently softened terminals — industrial precision with a friendly, modern warmth. Excellent legibility at UI sizes.
- **Weights Used**: ExtraLight (200) through ExtraBold (800) — comprehensive range for UI/UX.
- **Source**: https://fonts.google.com/specimen/Manrope
- **Fallback stack**: `'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif`
- **Vibe**: Precise, clean, modern, reliable.

### Loading in HTML
```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Manrope:wght@200..800&display=swap" rel="stylesheet">
```

### Loading in CSS
```css
@import url('https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Manrope:wght@200..800&display=swap');
```

## 3. Logo System
The logo is designed for extreme versatility across digital and physical mediums.
- **Variations**: 9 distinct versions (e.g., `Cenote Logo 01.png` through `09.svg`).
- **Formats**: Optimized in both Raster (PNG) for quick previews and Vector (SVG) for infinite scalability and production.
- **Applications**: 
    - Full Wordmarks for corporate presence.
    - Minimalist Icons/Logomarks for app icons and small-scale merchandise.

## 4. Visual Language & Marketing (Campaign)
The brand identity is extended through a robust multi-channel campaign system:
- **Environmental**: High-impact billboard designs (`Cenote Billboard 01-07`) showing the brand's scale in public spaces.
- **Digital/Social**: Dedicated Instagram post templates (`Instagram post - 4` to `9`) and a refined App Icon.
- **Imagery**: A mix of "Raw" photography and curated "Campaign" images that likely feature natural textures (water, stone) or high-clarity lifestyle shots.

## 5. Physical Touchpoints (Merchandise)
The Cenote brand is heavily manifested in "lifestyle" objects, emphasizing its presence in a user's daily life:
- **Office/Tech**: Mousepads, Pens, Sticker Logos.
- **Home/Lifestyle**: Clocks, Mugs, Tote Bags (including pattern variants), Caps.
- **Print**: Books, Business Cards.
- **Patterns**: The use of custom "Sticker Patterns" and "Tote Bag Patterns" indicates a secondary visual language of textures or repetitive motifs used to reinforce brand recognition.

## 6. Implementation Strategy for "Opus 4.7" Prompting
To replicate this brand's "vibe" for a new application, focus on:
1.  **Type Pairing**: Replicate the "Serif Headline + Sans-Serif UI" logic.
2.  **Asset Breadth**: Ensure the new brand has mockups for both high-end merchandise (watches/clocks) and everyday utility (tote bags/pens).
3.  **Natural/Industrial Balance**: Use names and visual cues that suggest natural depth (like "Cenote") but execute them with surgical, modern precision.
4.  **Pattern Extension**: Create a secondary pattern/motif derived from the logomark to be used on textiles and backgrounds.
