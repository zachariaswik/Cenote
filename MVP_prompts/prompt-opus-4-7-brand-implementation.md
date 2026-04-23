# Opus 4.7 Prompt: Cenote Brand Implementation

Use the prompt below with Opus 4.7.

```text
You are implementing the visual identity and product-facing brand expression for Cenote inside the current application codebase.

Read these files first before making any changes:
- CENOTE_BRAND_IDENTITY_FOR_AI.md
- app-context-spec.md
- business-plan.md
- prompt-5-frontends-and-cloud.md
- prompt-4-conversation-memory.md
- prompt-2-vault-and-search.md

Your job is not to invent a new aesthetic from scratch. Keep the existing Cenote aesthetic logic, but change the underlying idea so it clearly matches the application itself.

What must stay:
- the "modern heritage" tension
- the editorial-meets-technical feeling
- the serif + sans pairing logic
- the sense of depth, clarity, calm sophistication, and precision
- the natural / industrial balance
- the premium, restrained, high-trust tone

What must change:
- stop treating the brand like a generic lifestyle or merchandise system
- stop implying a broad consumer "life OS" fantasy
- translate the identity into a product for private memory, local intelligence, trusted retrieval, and action
- make the brand feel native to a privacy-first AI chief of staff that lives across chat, vault search, ingestion, and long-term memory

Core product idea you must express:
Cenote is a local-first AI memory and action layer for people with document-heavy, privacy-sensitive work. It indexes a local vault, ingests private files locally, remembers conversations, retrieves context on demand, and works through messaging and web chat without giving the cloud raw access to the vault.

Brand translation you must implement:
- "Depth" should mean memory, recall, context, and trustworthy stored knowledge
- "Clarity" should mean concise answers, visible privacy boundaries, legible system status, and calm information hierarchy
- "Natural elegance" should feel like still water, stone, mineral surfaces, and depth, not wellness branding or travel branding
- "Modern precision" should feel like a serious product system: sharp spacing, disciplined type scale, clean alignment, explicit states, and credible controls

Typography direction:
- Use Cormorant Garamond as the display/editorial voice for hero copy, key section headings, selected empty states, and high-trust narrative moments
- Use Sora for navigation, body copy, labels, controls, data-heavy surfaces, tables, chat UI, settings, and all functional interface text
- Do not overuse the serif in dense product UI
- The app should feel elegant, but usability wins over decoration

Color and material direction:
- Infer a palette that preserves the current Cenote mood: deep water, mineral shadow, limestone, paper, and restrained metallic contrast
- Prefer deep teal, blue-green, ink, stone, bone, and softened sand tones over generic SaaS blue or trendy purple
- Keep contrast strong enough for accessibility
- Use gradients sparingly and with depth, not glow
- Surfaces should feel tactile and grounded, not glossy or futuristic

Pattern and motif direction:
- Derive a subtle secondary motif from the idea of a cenote: rings, contours, apertures, waterlines, strata, or depth maps
- Use it in backgrounds, dividers, empty states, onboarding accents, or texture layers
- Keep it quiet and structural
- Do not make it decorative noise

UI concept you must land:
The app should feel like entering a private, deeply organized reservoir of memory and intelligence.
It should not feel like:
- a crypto dashboard
- a wellness retreat
- a neon AI toy
- a generic B2B SaaS admin template
- a glassmorphism concept shot

Product surfaces to implement or restyle:
1. Marketing / landing experience
- Reframe the messaging around private memory, local processing, retrieval, and action
- Hero should communicate trust, depth, and product function immediately
- Show how Cenote turns documents and conversations into recallable context and follow-up

2. Onboarding
- Make setup feel high-trust and guided
- Explain the privacy boundary visually
- Show that raw files stay local and the cloud only sees safe outputs

3. Main app shell
- Create a coherent navigation system for chat, vault, ingestion, memory, tasks, and settings
- The shell should feel quiet, premium, and stable

4. Chat interface
- This is a primary surface
- It should feel conversational but serious
- Emphasize continuity, memory, and linked context rather than casual chat-app mimicry

5. Vault search / knowledge retrieval
- Make search results feel archival, precise, and readable
- Highlight snippets, source paths, timestamps, and relevance in a calm editorial way

6. Ingestion flow
- Make the local-processing story visible
- Surface status, extraction, summaries, entities, and tasks in a way that feels trustworthy and legible

7. Memory and task surfaces
- Design these as evidence of depth and continuity
- The user should feel that Cenote remembers responsibly, not magically

8. Privacy / settings / trust surfaces
- These should be some of the strongest screens in the product
- Make local-first architecture and privacy boundaries visible, concrete, and credible

Copy direction:
- Replace vague lifestyle copy with product language
- The tone should be calm, intelligent, and exact
- Good themes: private memory, local vault, durable context, trusted recall, document-to-action, message-native workflow
- Avoid hype language like "revolutionary", "supercharged", "second brain", or "all-in-one life platform"

Design principles:
- premium restraint over visual noise
- legibility over novelty
- trust over excitement
- depth over busyness
- editorial clarity over startup cliches
- product credibility over branding theater

Implementation rules:
- Inspect the existing codebase first and work within the current stack
- Do not rewrite the app unnecessarily
- Extract design tokens for color, typography, spacing, radius, borders, shadows, and motion
- Define CSS variables or the equivalent token system used by the project
- Make components reusable and systematic
- Keep desktop and mobile experiences both first-class
- Preserve or improve accessibility: color contrast, focus states, keyboard nav, semantic structure
- Use subtle motion only where it reinforces depth, hierarchy, or state changes
- If the current app already has components, restyle and extend them instead of replacing everything blindly

Strong visual cues that should appear somewhere in the system:
- a deep, calm hero moment
- at least one credible privacy boundary visualization
- an editorial search/result layout
- a refined chat surface
- a quiet motif or texture that ties the product together

Things to avoid:
- purple-on-white AI startup defaults
- oversized glass cards everywhere
- noisy particle backgrounds
- stock "futuristic" illustrations
- excessive rounded-corner softness if it weakens the premium editorial feel
- treating Cenote like a lifestyle merch brand instead of a product system

Deliverables:
1. Implement the updated brand system in the application UI
2. Introduce or refine design tokens and reusable primitives
3. Rewrite UI copy where needed so the identity matches the actual product
4. Apply the system across the most important product surfaces, not just the landing page
5. Keep the aesthetic continuity with the existing Cenote brand while making the product meaning obvious

Acceptance criteria:
- The app still looks recognizably "Cenote"
- The design now clearly communicates privacy-first AI memory, local processing, and trusted retrieval
- The serif/sans relationship feels intentional and premium rather than decorative
- The product feels distinct from generic SaaS and generic AI branding
- The most important user journeys feel implemented, not mocked

Final response requirements:
- summarize the design system you implemented
- explain how you preserved the old aesthetic while changing the idea
- list the files you changed
- mention any areas that still need product or asset decisions
- include how the privacy-first concept is expressed visually in the interface
```
