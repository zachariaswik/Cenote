# Cenote — UI / UX Design Prompt

A single, self-contained brief. Paste the block below into Claude Design (or any equivalent design-capable model). It contains everything needed — no repository access, no external links required.

---

```text
You are the lead product designer for Cenote. Design the complete UI/UX system for this product. Deliver design artifacts — a design system, full-fidelity screens, a component library, tokens, and a copy glossary. You are not writing application code. You are designing the product.

# 1. WHAT CENOTE IS

Cenote is a local-first AI memory and action layer for people who do document-heavy, privacy-sensitive work — lawyers, analysts, clinicians, founders, researchers, long-horizon keepers of personal knowledge.

In plain language: Cenote runs on your own computer, owns a folder of your notes and documents (the "vault"), reads and parses private files using small local AI models, and exposes a small, well-defined set of capabilities to a larger cloud reasoning model that never sees your raw files — only safe summaries. You talk to Cenote primarily through messaging (WhatsApp and Telegram) and secondarily through a browser chat surface. Every conversation is persisted locally and can be recalled forever.

The defining product idea: a private reservoir of memory and intelligence you can ask questions of, drop documents into, and act from — with the privacy boundary visible, not hidden.

The six core capabilities Cenote offers through its interface:
- Search the vault — hybrid keyword + semantic retrieval over the user's local notes
- List tasks — structured task records mined from notes and conversations
- Ingest a file — parse and extract locally; surface safe summaries, entities, and proposed tasks
- Commit an interaction — every user/assistant turn is persisted locally
- Recall memories — ranked episodic recall of prior conversations
- Health check — system readiness and status

# 2. USERS AND WHAT THEY DO

Two user shapes:
- The operator: a document-heavy professional who works from messaging. Cannot tolerate leaking raw files to the cloud. Needs fast recall, credible privacy, and calm output.
- The keeper: someone accumulating a durable, private long-term memory across years of notes, conversations, and ingested files.

Three primary jobs-to-be-done:
- Ask and answer — "what did I decide about X," "what did Y send me last month," "draft a reply using the context we have"
- Capture and process — forward a document, dictate a note, or drop a file; Cenote ingests it locally and surfaces structured outcomes
- Follow through — Cenote proposes tasks and reminders grounded in the vault and conversation history

Optimize every surface for these jobs. Reject designs that optimize for dashboard-browsing, analytics, or social.

# 3. BRAND AND VISUAL IDENTITY

Cenote is a "Modern Heritage" brand. It juxtaposes the organic, deep-rooted elegance of a natural cenote with the precision of modern design.

Atmosphere: professional yet approachable. Editorial aesthetics with tech-forward usability. Calm, credible, premium-restrained.

Keywords to hit: Depth. Clarity. Sophistication. Natural. Modern. Trustworthy.

Atmosphere to avoid: crypto dashboard; wellness retreat; neon AI toy; generic B2B SaaS admin template; glassmorphism concept shot.

## Typography (dual-core)

- **Serif: EB Garamond** — for headlines, editorial copy, high-trust narrative moments, and the display layer. A direct Garamond derivative: high contrast, generous counters, humanist proportions. Weights 400–800 with italics. Fallback stack: `'EB Garamond', Georgia, 'Times New Roman', serif`.
- **Sans: Manrope** — for navigation, body copy, labels, controls, tables, chat UI, and all functional interface text. Geometric with gently softened terminals — industrial precision with friendly warmth. Weights 200–800. Fallback stack: `'Manrope', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif`.

Both load via `@import url('https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Manrope:wght@200..800&display=swap');` — no file uploads required.

The rule: serif for editorial, sans for interface. Do not overuse the serif in dense product UI. Usability wins over decoration.

## Color direction

Infer a palette built on: deep water, mineral shadow, limestone, paper, stone, bone, softened sand, and a single restrained metallic contrast. Prefer deep teal and blue-green over generic SaaS blue. Prefer ink and stone over neon purple. Prefer sand and bone over dead white.

Rules: high contrast for legibility; gradients sparingly and with depth, not glow; surfaces tactile and grounded, not glossy or futuristic; a full dark-mode palette that preserves the same semantic roles.

## Motif

Derive a secondary motif from the idea of a cenote: rings, contours, apertures, waterlines, strata, or depth maps. Use it quietly in backgrounds, dividers, empty states, and onboarding accents. Structural, not decorative. Do not turn it into noise.

# 4. SURFACES TO DESIGN

Cenote lives simultaneously across three surfaces. Design all three.

**A. Messenger-native (WhatsApp / Telegram) — primary**
Constrained to chat bubbles, limited inline formatting, platform-native attachments. Design the message grammar, not the layout. What does a reply, a search result, a task proposal, an ingestion receipt, or a privacy notice look like *as a WhatsApp/Telegram message* on a phone? Every message must be legible, skimmable, and as complete as possible without a follow-up query. Deliver a library of message templates, not screens.

**B. Web chat — secondary, full-featured**
A single-pane conversational product with a thin sidebar for navigation. Feels like a private workspace, not a consumer chat app. Must expose everything a messenger cannot: inspector panels, full search results, ingestion detail, memory timeline, task board, privacy/status dashboard. This is where most of the screen design effort goes.

**C. CLI — operator-only**
Minimal visual design but typography and color tokens carry through where they render (colored logs, system status output). Not a full surface, but must not feel orphaned.

# 5. INFORMATION ARCHITECTURE (WEB)

Primary navigation — seven destinations:
1. **Chat** — the default entry
2. **Vault** — search and browse local notes
3. **Memory** — episodic memory and rolling summaries
4. **Tasks** — structured task records
5. **Ingest** — drop target and history of ingested files
6. **Privacy** — the trust dashboard; always one click away
7. **Settings** — config, providers, secrets, observability

Secondary navigation within chat: session list grouped by time, pinned conversations, per-thread inspector panel.

# 6. SCREENS TO DESIGN

For each surface below, design: layout, component composition, empty state, loading state, error state, success state, keyboard shortcuts, mobile adaptation. Do not ship any screen missing an empty or loading state — those are where trust is made or lost.

**6.1 Onboarding** — a three-step flow: point Cenote at a vault folder; confirm the local models are reachable; optionally wire a messaging provider and a cloud reasoner. Each step visibly declares what stays local versus what leaves the machine. Completion lands the user in chat with a pre-loaded "welcome from your vault" turn.

**6.2 Chat (web)** — three-column layout. Center: message stream with editorial typography for long replies, compact sans for system and tool lines. Left rail: session list grouped by Today / This week / Older, with rolling summaries as titles. Right rail (collapsible): inspector showing active tool calls, cited sources, recalled memories, and a privacy banner for the current turn. Input composer: plain text, slash commands (/search, /ingest, /recall, /task), file drop, voice-to-text entry point. Every claim in every assistant turn links to its source note path and line range. Mobile: single column with swipe gestures to reach session list and inspector.

**6.3 Vault search** — editorial result layout. Result title in serif, path and timestamp in small sans, snippet in serif with matched terms highlighted using underline rather than background fill (stays calm). Filters: modality (notes, ingested docs, chat), time range, tag, entity. A small per-result badge shows whether the match came from keyword search, semantic search, or both — with a tooltip explaining what matched. Selecting a result opens a reading pane inline, not a new page.

**6.4 Ingest** — a drop zone plus a table of recent ingests. Each row: filename, ingest time, extraction status, entities extracted, tasks proposed, summary, sidecar location. A full ingest detail view narrates the local-processing story: parser used, local model used, chunks embedded, and an explicit statement of what the cloud will and will not see. Errors are first-class — if a local model is unreachable, the UI says so plainly and offers a degraded-mode option, not a generic toast.

**6.5 Memory** — timeline view of episodic memories with rolling summaries at week and month boundaries. Each memory links back to the conversation turn it came from. A "recall playground": type a query, see ranked memories with scores and provenance. Editing or forgetting a memory is possible, auditable, and obvious — trust requires controllability.

**6.6 Tasks** — three lanes: proposed, accepted, done. Each task shows its source (which note, which turn) and can be promoted into chat as context with one action. Bulk operations via keyboard. Filters: source type, entity, due window.

**6.7 Privacy dashboard** — the strongest screen in the product. Four panels: what is local (vault path, database path, vector index, sidecar files); what is outbound (cloud provider, messaging adapters, preview of the last outbound payload); what is cached (embeddings, summaries); what is purgeable (one-click forget). A live "last message" inspector displays the exact summary payload sent to the cloud for the most recent turn — this is how the boundary is proven, not merely claimed. A "panic" action pauses all outbound integrations with one keystroke.

**6.8 Settings** — sectioned: Vault, Models, Cloud reasoner, Messaging providers, Integrations, Secrets, Observability. Every secret field is masked by default, with click-to-reveal and an audit entry on reveal. Every section surfaces the relevant environment variable and a link to the explanation.

**6.9 Health / system status** — a standalone dashboard mirroring the command-line "doctor" check. Traffic-light states per subsystem; clicking a red light opens the exact remediation steps.

**6.10 Empty and error state libraries** — a small family of empty-state compositions (blank vault, no results, no tasks, provider down, local model missing) and a standard error taxonomy (TransientProvider, LocalModelMissing, PermissionDenied, SchemaMigrationNeeded, WebhookSignatureInvalid) with consistent card shapes and recovery actions.

# 7. CHAT AS A STRUCTURED ARTIFACT

This is the heart of the product. Every assistant turn carries six parts:
- the reply (editorial voice)
- tool calls invoked (ordered, with durations)
- cited sources (vault paths and line ranges)
- recalled memories (with scores)
- a privacy note (what left the machine for this turn)
- proposed next actions (tasks, follow-ups)

In the web UI these are disclosed progressively — the reply is always visible, the rest is one click away. In messenger surfaces they collapse into a short scannable footer the user can expand with a command ("show sources").

Streaming: tokens stream into the reply; tool-call stages stream into the inspector. Never show a spinner without a label — "searching vault," "recalling memories," "reasoning."

Interruption: the user can cancel a tool call or a cloud call mid-stream. Cancellation still persists the turn with a truncation marker.

# 8. PRIVACY-FIRST VISUAL LANGUAGE

Invent and enforce a consistent visual vocabulary for the local/cloud boundary. Three primitives:
- A **waterline motif** — a thin horizontal rule with a label on either side. Appears wherever data crosses the boundary.
- A **privacy chip** — a small inline badge indicating "stayed local," "sent to cloud as summary," "sent to cloud as text," or "never persisted."
- A **provenance trace** — for every surfaced answer, a collapsed trail of where each fragment came from and whether it crossed the boundary.

These appear in: chat replies, vault search results, ingest detail, memory recall, and the privacy dashboard. Consistency is how trust compounds.

# 9. COMPONENT LIBRARY

Design a small, opinionated library. No heavyweight UI kit. Minimal primitives that compose upward:

- **Type**: Display, Headline, Subhead, Lead, Body, Caption, Eyebrow
- **Surface**: Card, Panel, Divider, Sheet, Modal, Drawer
- **Control**: Button (primary, secondary, ghost, destructive), Input, Textarea, Select, Toggle, Kbd, Tag, Chip
- **Chat**: MessageBubble (user, assistant, system), ToolCallStrip, CitationList, MemoryChip, PrivacyBanner, StreamingCursor
- **Data**: ResultRow, TableRow, TimelineEntry, StatusDot, TrafficLight
- **State**: EmptyState, ErrorCard, LoadingLine, ToastStack

Every component declares its states explicitly — default, hover, focus, active, disabled, loading, error, empty — and passes keyboard-only operation.

# 10. DESIGN TOKENS

Define a complete token set. Name by role, not by literal value, so the light/dark themes share semantics.

**Color** (full dark-mode set included):
- `--cenote-ink` / `--cenote-ink-muted` / `--cenote-ink-subtle`
- `--cenote-paper` / `--cenote-paper-raised` / `--cenote-paper-sunken`
- `--cenote-water` / `--cenote-water-muted` (brand accent, deep teal family)
- `--cenote-stone` (neutral divider)
- `--cenote-sand` (secondary accent)
- `--cenote-signal-ok` / `--cenote-signal-warn` / `--cenote-signal-error`
- `--cenote-signal-local` / `--cenote-signal-cloud` (for the privacy language)

**Spacing**: 4px base. Steps at 4 / 8 / 12 / 16 / 24 / 32 / 48 / 64 / 96.
**Radius**: 0 / 2 / 4 / 8 / 12 — kept small. Editorial, not soft.
**Shadow**: three steps — subtle, card, floating. Never glowy.
**Border weights**: hairline, standard, structural.
**Motion**: `--cenote-motion-fast` 120ms, `--cenote-motion-standard` 200ms, `--cenote-motion-slow` 360ms. Default easing `cubic-bezier(0.2, 0, 0, 1)`.
**Z-index**: base, raised, sticky, overlay, modal, toast.

# 11. COPY AND MICROCOPY

Tone: calm, exact, operator-to-operator. No exclamation points. No "let's," "great!," "awesome!." Short clauses. Periods.

Patterns:
- Empty states: one-line observation plus one concrete next action
- Errors: what happened, where, what to do — in that order, no apologies
- Confirmations: state what will happen in present tense; avoid "are you sure?"
- Tool names in UI use product verbs: "Search," "Recall," "Ingest" — never the raw function names
- Privacy language is precise: "stayed on this machine," "sent to the cloud reasoner as a summary," "never persisted" — never marketing phrasing

Deliver a microcopy glossary as part of the final output.

# 12. ACCESSIBILITY

WCAG AA minimum; AAA for body copy. All controls reachable by keyboard; focus rings visible and on-brand. All icons have text alternatives. Motion respects `prefers-reduced-motion` — decorative transitions disabled, state-change transitions preserved. Color is never the only channel — signal states pair color with shape or label. Live regions on streaming chat and tool-call updates. Tested with keyboard-only and with a screen reader before sign-off.

# 13. RESPONSIVE AND MULTI-SURFACE

- **Web**: three breakpoints — compact (<720px), comfortable (720–1200), wide (>1200). Layout collapses rails before touching the message stream. The reading measure is never below 56ch or above 76ch.
- **Messenger**: design templates to render cleanly at typical WhatsApp and Telegram widths on phones and desktop. Avoid ASCII tables; prefer short labeled lines.
- **CLI**: map the color tokens to terminal ANSI codes so status output and doctor checks feel part of the same system.

# 14. MOTION

Motion earns its place by making state changes legible. Use it for: streaming tokens, tool-call progression, inspector open/close, drawer slides, toast entries, and the privacy waterline when a message is about to cross it. No looping backgrounds. No parallax. Keep total on-screen motion to at most two simultaneous animations at any time.

# 15. DO NOT

- No Tailwind/Bootstrap/Material/Chakra aesthetic defaults — Cenote has its own voice
- No purple gradients, glassmorphism, glow effects, animated particle backgrounds
- No mimicry of ChatGPT, Claude.ai, Perplexity, or Notion layouts — Cenote is its own product
- No fake data shown as if real; use clearly labeled fixtures
- No hiding the privacy boundary to reduce visual clutter
- No tooltips carrying primary information
- No excessive corner rounding; the product reads editorial, not soft
- No desktop-first thinking; the primary surface is a phone messenger

# 16. DELIVERABLES

Return the following as design artifacts (not code to install in a repo):

1. **Design system reference**, structured as a single well-organized document or interactive artifact containing: token tables (color, spacing, radius, shadow, motion, z-index) with light and dark values; typography specimens at all levels; motif specimens; component library with every primitive in every state.

2. **High-fidelity screen designs** for the four primary web surfaces: Chat, Vault search, Ingest detail, Privacy dashboard. Each in light and dark theme, at desktop and mobile breakpoints, with empty, loading, error, and success states.

3. **Layout-level designs** (lower fidelity, but with real structure and copy) for: Onboarding, Memory, Tasks, Settings, Health/system status.

4. **Messenger message template library** showing: reply, search result, task proposal, ingestion receipt, privacy notice, error. Rendered as they would appear in a WhatsApp and Telegram thread on a phone.

5. **Component library review sheet** — one page per primitive showing every declared state side by side.

6. **Microcopy glossary** — tone, banned phrases, standard patterns, and sample strings for the most-used UI moments.

7. **Accessibility checklist** — specific pass/fail per surface.

8. **A single "hero" view** that summarizes the identity: the Chat screen, populated with a realistic turn including cited sources, a recalled memory, and a visible privacy note. This is the screenshot the product is sold on.

Format preference for design artifacts: deliver as HTML + CSS artifacts rendering the token system live, so the design system is self-demonstrating rather than flat mockups. Supplement with SVG or image exports only where motion and interaction cannot be shown in HTML.

# 17. ACCEPTANCE CRITERIA

- The system is recognizably Cenote — editorial serif headlines, geometric sans UI, deep water and stone palette, cenote motif present but quiet
- The chat surface clearly communicates: streaming reply, tool calls, citations, recalled memories, and a privacy note — all on a single assistant turn without feeling cluttered
- The privacy dashboard is the strongest screen in the product; the local/cloud boundary is visible, specific, and controllable
- Messenger templates are legible and complete on a phone without requiring follow-up queries
- The design works equally in light and dark theme and passes AA contrast everywhere
- The product feels distinctly different from generic AI chat products and generic B2B SaaS — it reads as a serious operator tool for private memory
- Every primitive has every state; no state is missing an empty or loading design

# 18. FINAL RESPONSE FORMAT

When you finish, return:
- a summary of the design system: palette decisions with hex values, type scale, motif, motion principles
- the component primitives you produced and the screens they compose into
- which screens are high-fidelity versus layout-level
- how the privacy boundary is expressed visually, with named components
- the messenger templates with example rendered output
- a list of product decisions the human still needs to weigh in on
- the microcopy glossary inline
- links or embeds to the design artifacts themselves
```
