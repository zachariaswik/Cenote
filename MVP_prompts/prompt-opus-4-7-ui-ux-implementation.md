# Opus 4.7 Prompt: Cenote UI / UX Implementation

Run this after `prompt-opus-4-7-brand-implementation.md`. That prompt owns the visual identity (typography, color, motif). This prompt owns the entire product UI/UX system — information architecture, screens, components, states, flows, motion, and copy — and implements it in the codebase.

## Prompt

```text
You are implementing the complete UI/UX system for Cenote inside the current application codebase. This is a product design and product implementation job, not a marketing or mockup job. You are not redesigning the brand — the brand is already established in CENOTE_BRAND_IDENTITY_FOR_AI.md and assets/brand/typography.css. You are designing and shipping the product surfaces users actually live in.

Read these files first before making any changes:
- README.md
- app-context-spec.md
- business-plan.md
- CENOTE_BRAND_IDENTITY_FOR_AI.md
- assets/brand/typography.css
- assets/system_prompt.md
- MVP_prompts/prompt-opus-4-7-brand-implementation.md
- MVP_prompts/prompt-5-frontends-and-cloud.md
- MVP_prompts/prompt-4-conversation-memory.md
- MVP_prompts/prompt-2-vault-and-search.md
- MVP_prompts/prompt-3-local-hive-ingestion.md
- src/integrations/ (messaging.rs, web_chat.rs, cloud.rs, orchestrator.rs)
- src/tools/ (canonical tool signatures the UI renders against)
- migrations/ (data model the UI surfaces)

# 1. PRODUCT TRUTH — what you are actually designing for

Cenote is a local-first AI memory and action layer. It runs as a Rust daemon on the user's machine, owns a personal vault of notes and documents, parses private content through local models (Ollama), and exposes a small set of MCP tools to a cloud reasoner that never sees raw files. The user interacts with Cenote primarily through messaging (WhatsApp, Telegram via OpenClaw) and secondarily through a browser chat surface. Every turn is persisted locally and recallable.

The canonical MCP tool surface the UI must render coherently:
- search_vault — hybrid FTS + semantic retrieval over local notes
- list_tasks — structured task records mined from notes and chat
- ingest_new_file — local parse + hive extract + sidecar; returns cloud-safe outputs
- commit_interaction — persist every user/assistant turn
- recall_memories — ranked episodic recall
- health_check — system readiness

The privacy boundary is load-bearing. The product must make it visible, not hidden.

# 2. USERS AND PRIMARY JOBS-TO-BE-DONE

Design for two user shapes and three jobs:

User shapes:
- The operator: document-heavy professional (lawyer, analyst, founder, researcher, clinician) who works from messaging and cannot tolerate leaking raw files to the cloud.
- The keeper: someone who wants a durable, private long-term memory across years of notes, conversations, and ingested files.

Primary jobs:
- Ask and answer — "what did I decide about X", "what did Y send me last month", "draft a reply using the context we have"
- Capture and process — forward a document, dictate a note, or drop a file; Cenote ingests it locally and surfaces structured outcomes
- Follow through — Cenote proposes tasks, reminders, or actions grounded in the vault and conversation history

Optimize every surface for these jobs. Reject designs that optimize for dashboard-browsing or analytics.

# 3. INFORMATION ARCHITECTURE

The product has three surfaces with different constraints. Design all three.

A. Messenger-native UI (WhatsApp / Telegram via OpenClaw) — primary
- Constrained to chat bubbles, limited inline formatting, and platform-native attachments
- Design the message grammar, not the layout: what does a reply, a search result, a task proposal, an ingestion receipt, or a privacy notice look like as a WhatsApp/Telegram message?
- Every message must be legible, skimmable on a phone, and complete without a follow-up query when possible
- Define templates, not prose — the daemon will fill them

B. Web chat UI (browser) — secondary, full-featured
- A single-pane conversational product with a thin sidebar for navigation
- Must feel like a private workspace, not a consumer chat app
- Must expose all the surfaces a messenger cannot: inspector panels, full search results, ingestion detail, memory timeline, task board, privacy/status dashboard

C. CLI — operator-only
- Visual design is minimal but typography-consistent — same token palette applied to terminal output where it renders (colored logs, doctor output, demo script framing)
- Not a full surface, but must not feel orphaned

Web navigation (primary nav):
1. Chat — the default entry
2. Vault — search + browse local notes
3. Memory — episodic memory and rolling summaries
4. Tasks — structured task records
5. Ingest — a drop target + history of ingested files
6. Privacy — the trust dashboard (always one click away)
7. Settings — config, providers, MCP clients, secrets

Secondary nav (within chat): session list, pinned conversations, thread-level inspector.

# 4. SCREENS / SURFACES TO IMPLEMENT

For each surface below, design and implement: layout, component composition, empty state, loading state, error state, success state, keyboard shortcuts, and mobile adaptation. Do not ship any screen that is missing an empty or loading state — those are where trust is made or lost.

4.1 Onboarding
- Three-step flow: (1) point Cenote at a vault folder, (2) confirm Ollama is reachable and pick local models, (3) optionally wire a messaging provider and a cloud reasoner
- Each step shows exactly what stays local and what leaves the machine
- Completion lands the user in chat with a pre-loaded "welcome from your vault" turn that proves the loop works

4.2 Chat (web)
- Center column: message stream with editorial typography for long replies, compact sans for system/tool lines
- Each assistant turn can expose: the tool calls it made, the snippets it cited, and the memories it recalled — collapsed by default, expandable inline
- Citations are first-class: every claim backed by vault content links to the source note path and line range
- Input composer supports: plain text, slash commands (/search, /ingest, /recall, /task), file drop, and voice-to-text hooks
- Left rail: session list, grouped by time (Today / This week / Older), with rolling summaries as session titles
- Right rail (collapsible): inspector — active tool calls, cited sources, recalled memories, privacy banner for the current turn
- Mobile: single column with swipe gestures to reach session list and inspector

4.3 Vault search
- Editorial result layout: result title in serif, path + timestamp in small sans, snippet in serif with matched terms highlighted using underline rather than background fill (stays calm)
- Filters: modality (notes, ingested docs, chat), time range, tag, entity
- Hybrid scoring is visible: a small "FTS / semantic / both" badge per result, with a tooltip explaining what matched
- Empty: a single-line prompt and a "try these" list of recent entities and tags mined from the vault
- Selecting a result opens a reading pane, not a new page — preserves conversational flow

4.4 Ingest
- Drop zone + recent ingests table
- Each ingest row shows: filename, ingest time, extraction status, entities extracted, tasks proposed, summary, and where its sidecar lives
- A full ingest detail view shows the local-processing story explicitly: parser used, local model used, chunks embedded, and what exactly the cloud will and will not see
- Errors are first-class: if Ollama is down, the UI says so plainly and offers degraded-mode options, not a generic toast

4.5 Memory
- Timeline view of episodic memories with rolling summaries at week/month boundaries
- Each memory links back to the conversation turn and the commit_interaction record
- A "recall" playground: type a query and see ranked memories with scores and provenance
- Editing/forgetting a memory is possible, auditable, and obvious — trust requires controllability

4.6 Tasks
- Three lanes: proposed, accepted, done
- Each task shows source (which note, which turn) and can be promoted back into chat as context
- Bulk operations via keyboard
- Filters: source type, entity, due window

4.7 Privacy dashboard
- The strongest screen in the product
- Four panels: what's local (vault path, db path, vector index, sidecars), what's outbound (cloud provider, messaging adapters, last outbound payload preview), what's cached (embeddings, summaries), what's purgeable (one-click forget)
- A live "last message" inspector shows the exact tool_context that went to the cloud for the most recent turn — proves the boundary holds
- Offer a "panic" action: pause all outbound integrations with one keystroke

4.8 Settings
- Sectioned: Vault, Models (Ollama), Cloud reasoner, Messaging providers, MCP clients, Secrets, Observability
- Every secret field has the same treatment: masked by default, click-to-reveal with audit log entry, copy without reveal
- Every section links to the relevant .env variable and the doc line that explains it

4.9 Health / doctor
- A standalone "system status" surface mirroring the cargo run -- doctor CLI
- Traffic-light states per subsystem; clicking a red light opens the exact remediation steps

4.10 Empty and error libraries
- Define a small library of empty-state compositions: blank vault, no results, no tasks, provider down, model missing
- Define the standard error taxonomy: TransientProvider, LocalModelMissing, PermissionDenied, SchemaMigrationNeeded, WebhookSignatureInvalid — each with a consistent card shape and recovery action

# 5. CHAT UX DEPTH (because it is the product)

Design the chat turn as a structured artifact, not a loose blob of text. A full assistant turn carries:
- the reply (editorial)
- tool calls invoked (ordered, with durations)
- cited sources (vault paths and line ranges)
- recalled memories (with scores)
- privacy note (what left the machine for this turn)
- proposed next actions (tasks, follow-ups)

In the web UI, these are disclosed progressively — the reply is always visible; the rest is one click away. In messenger UIs, they collapse into a short, scannable footer the user can query with a command ("show sources").

Streaming: tokens stream into the reply; tool-call stages stream into the inspector. Never show a spinner without a label — "searching vault", "recalling memories", "reasoning".

Interruption: the user can cancel a tool call or a cloud call mid-stream. Cancellation still runs commit_interaction with a truncation marker.

# 6. COMPONENT SYSTEM

Build a small, opinionated component library. Do not pull in a heavyweight UI kit. Prefer native semantics and minimal primitives.

Primitives to define (as tokens + styled elements):
- Type components: Display, Headline, Subhead, Lead, Body, Caption, Eyebrow (mirroring typography.css)
- Surface components: Card, Panel, Divider, Sheet, Modal, Drawer
- Control components: Button (primary, secondary, ghost, destructive), Input, Textarea, Select, Toggle, Kbd, Tag, Chip
- Chat components: MessageBubble (user, assistant, system), ToolCallStrip, CitationList, MemoryChip, PrivacyBanner, StreamingCursor
- Data components: ResultRow, TableRow, TimelineEntry, StatusDot, TrafficLight
- State components: EmptyState, ErrorCard, LoadingLine, ToastStack

Every component must declare its states explicitly (default, hover, focus, active, disabled, loading, error, empty) and pass keyboard-only operation.

# 7. DESIGN TOKENS

Extend assets/brand/typography.css with a parallel tokens file at assets/brand/tokens.css (or equivalent location matching the existing stack). Define:

Color tokens (names, not hexes — decide the palette per the brand prompt's direction: deep water, mineral shadow, limestone, paper):
- --cenote-ink (primary text)
- --cenote-ink-muted
- --cenote-ink-subtle
- --cenote-paper (primary surface)
- --cenote-paper-raised
- --cenote-paper-sunken
- --cenote-water (brand accent, deep teal family)
- --cenote-water-muted
- --cenote-stone (neutral divider)
- --cenote-sand (secondary accent)
- --cenote-signal-ok / --cenote-signal-warn / --cenote-signal-error / --cenote-signal-local / --cenote-signal-cloud
- Full dark-mode set with the same semantic names

Spacing scale: 4px base, steps at 4 / 8 / 12 / 16 / 24 / 32 / 48 / 64 / 96.
Radius scale: 0 / 2 / 4 / 8 / 12 — keep small; premium restraint.
Shadow scale: three steps — subtle, card, floating; never glowy.
Border weights: hairline (1px), standard (1.5px at 2x DPR), structural (2px).
Motion: --cenote-motion-fast (120ms), --cenote-motion-standard (200ms), --cenote-motion-slow (360ms); easing curves cubic-bezier(0.2, 0, 0, 1) default, ease-out for entrance, ease-in for exit.
Z-index scale with explicit layers: base, raised, sticky, overlay, modal, toast.

All tokens usable in CSS and in the Rust templates/HTML the web chat serves.

# 8. PRIVACY-FIRST VISUAL LANGUAGE (distinctive to this product)

Invent and enforce a consistent visual vocabulary for the privacy boundary. Suggested primitives:
- A "local/cloud" waterline motif: a thin horizontal rule with a label on either side. Appears anywhere data crosses the boundary.
- A privacy chip: a small inline badge on any piece of content that indicates "stayed local" vs "sent to cloud as summary" vs "sent to cloud as text" vs "never persisted".
- A provenance trace: for any surfaced answer, a collapsed trail of where each fragment came from and whether it crossed the boundary.

These elements should appear in: chat replies, vault search results, ingest detail, memory recall, and the privacy dashboard. Consistency across surfaces is how trust compounds.

# 9. COPY AND MICROCOPY

Tone: calm, exact, operator-to-operator. No exclamation points. No "let's", "great!", "awesome!". Use periods. Short clauses.

Microcopy patterns to define and apply:
- Empty states: one-line observation + one concrete next action
- Errors: what happened, where, what to do, in that order — no apologies
- Confirmations: state what will happen, in present tense; do not use "are you sure?"
- Tool names in UI use product verbs: "Search", "Recall", "Ingest" — never "search_vault" in user copy
- Privacy language is precise: "stayed on this machine", "sent to the cloud reasoner as a summary", "never persisted" — never marketing phrasing

Provide a microcopy glossary in the repo at assets/brand/copy.md.

# 10. ACCESSIBILITY

- WCAG AA minimum, AAA for body copy
- All controls reachable by keyboard; focus rings visible and on-brand, not browser-default
- All interactive elements have accessible names; all icons have text alternatives or aria-hidden with a labeled sibling
- Motion respects prefers-reduced-motion — disable decorative transitions, keep state-change transitions
- Color is never the only channel — signal states pair color with shape or label
- Screen reader order matches visual order on every screen
- Live regions on streaming chat and tool-call updates
- Tested with keyboard-only and with at least one screen reader before acceptance

# 11. RESPONSIVE AND MULTI-SURFACE STRATEGY

- Web: three breakpoints — compact (<720), comfortable (720–1200), wide (>1200). Layout collapses rails before it touches the message stream. The stream is never below 56ch or above 76ch measure.
- Messenger: design the message templates to render well at typical WhatsApp/Telegram widths on both phones and desktop clients. Avoid ASCII tables; prefer short labeled lines.
- CLI: use the token palette's color names to pick terminal ANSI mappings; keep doctor/demo output aligned and scannable.

# 12. MOTION

Motion earns its place by making state changes legible. Use it for: streaming tokens, tool-call progression, inspector open/close, drawer slides, toast entries, and the privacy waterline when a message is about to cross it. Do not animate decorative elements. No looping backgrounds. No parallax. Keep total on-screen motion on any surface under 2 simultaneous animations at a time.

# 13. IMPLEMENTATION RULES

- Work within the existing stack (Rust + Axum web_chat). If the web surface is server-rendered, use templated HTML + vanilla CSS; if a JS framework is added later, the token layer must port 1:1.
- Put design tokens and primitives in assets/brand/; put web chat templates in src/integrations/web_chat/ templates alongside existing code.
- Wire the primitives into the web chat endpoint end-to-end on at least the chat, vault search, ingest detail, and privacy dashboard surfaces. The remainder may ship as route stubs with real layouts but placeholder data handlers — but the design is not acceptable if it cannot be rendered live.
- For messaging adapters, implement the message templates as typed renderers (Rust types with explicit render methods per provider) in src/integrations/messaging.rs so layout changes are centralized.
- Dark mode is not optional. Implement both themes via the token layer.
- Do not introduce inline styles in templates; everything routes through tokens.
- Every new component has a test fixture in fixtures/ui/ that renders it in its full state matrix (default / loading / empty / error). A small harness renders the fixtures to a review page at /_/ui.
- Screenshots of the primary surfaces (chat, vault, ingest detail, privacy dashboard) go under assets/screenshots/ and are referenced from README.md.

# 14. DO NOT

- Do not import Tailwind, Bootstrap, Material, Chakra, or any design system that carries its own voice
- Do not add purple gradients, glassmorphism, glow effects, or animated particle backgrounds
- Do not mimic ChatGPT, Claude.ai, Perplexity, or Notion layouts; Cenote is its own product
- Do not show fake data as if it were real; use fixtures and mark them
- Do not hide the privacy boundary to reduce visual clutter
- Do not add tooltips to carry primary information
- Do not over-round corners; the product reads as editorial, not soft
- Do not design only for desktop; the primary surface is a phone messenger

# 15. DELIVERABLES

1. Extended design token file (assets/brand/tokens.css) with color, spacing, radius, shadow, motion, and z-index tokens; dark mode included
2. Primitive component library rendered by the existing web stack, with a live /_/ui review route
3. Fully implemented web surfaces for: Chat, Vault search, Ingest detail, Privacy dashboard
4. Route-stub implementations with real layout for: Onboarding, Memory, Tasks, Settings, Health
5. Messenger message-template renderers for: reply, search result, task proposal, ingestion receipt, privacy notice, error
6. assets/brand/copy.md microcopy glossary
7. Updated README.md with a UI section, screenshots, and links to the review route
8. Accessibility checklist (assets/brand/a11y.md) with pass/fail per surface

# 16. ACCEPTANCE CRITERIA

- A first-time developer can cargo run -- serve, open the browser, and reach every primary surface with real or fixture data
- The chat surface streams replies, shows tool calls, cites sources, and exposes the privacy inspector on every assistant turn
- The privacy dashboard visibly proves the local/cloud boundary with the most recent tool_context payload
- The messenger templates render correctly in a local simulator for reply / search result / task / ingestion receipt / privacy notice
- Keyboard-only operation works across Chat, Vault search, and Settings
- Both light and dark themes ship and pass contrast checks
- Typography and color come exclusively from the token layer; no inline styles
- The product is recognizably Cenote, recognizably a serious operator tool, and recognizably different from generic AI chat products

# 17. FINAL RESPONSE REQUIREMENTS

When you finish, report:
- the token set you defined (names only, not values) and where it lives
- the list of primitives you built and the surfaces they compose into
- the surfaces that are fully implemented vs. stubbed with real layout
- the messenger templates you implemented and how they render in the simulator
- how the privacy boundary is expressed visually, with specific file references
- accessibility results: what passes, what still needs work
- any product decisions that need the human to weigh in (e.g. exact palette hex values, copy approvals, provider-specific messenger constraints you discovered)
- the commands a reviewer should run to see the full UI end-to-end
```
