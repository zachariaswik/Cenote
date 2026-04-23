# Cenote Business Plan

Based on the current product specification in this workspace and a market snapshot reviewed on April 23, 2026.

## 1. Executive Summary

Cenote is a local-first AI operating system for personal and small-team knowledge work. It combines:

- a cloud reasoning layer for planning and conversation
- a local Rust daemon that owns tools, storage, policy, and indexing
- local models for private file ingestion and extraction
- ubiquitous interfaces through WhatsApp, Telegram, and web chat

The product thesis is simple: people want an AI assistant that actually remembers context, works inside the tools they already use, and does not require sending raw private documents to a centralized SaaS.

The strongest initial business is not mass-market consumer AI. It is a premium product for privacy-sensitive solo professionals and small client-service teams that already operate across notes, PDFs, chats, and voice transcripts, and who value trust and continuity more than lowest-cost access.

Recommended initial wedge:

- consultants
- recruiters and executive search professionals
- boutique agencies
- founders and chiefs of staff
- small advisory and professional-service teams

Recommended business model:

- open-core local edition to build trust and adoption
- paid managed Cenote service for setup, orchestration, messaging integration, updates, and support
- higher-tier team and private-deployment plans for small firms

## 2. Product Summary

From the current specification, Cenote does five things that matter commercially:

1. It indexes a local vault of notes and documents.
2. It ingests new private files locally and only exposes summaries, tasks, entities, and snippets to the cloud layer.
3. It stores every conversation turn locally and builds long-term memory.
4. It exposes all capabilities through a controlled daemon and MCP tool surface.
5. It makes the primary interface messaging-native instead of app-native.

That makes Cenote more than a chatbot. It is a memory, retrieval, and task layer that sits between a user's knowledge base and their daily communication flow.

## 3. Problem

Knowledge workers have three persistent problems:

- their context is fragmented across notes, PDFs, messages, meeting summaries, and browser tabs
- most AI tools are stateless or weak at durable memory
- privacy-sensitive users do not want to upload raw client, company, or personal material into a generic cloud assistant

The result is a common failure mode: people use AI for one-off drafting, but not as a persistent operational system for follow-through, recall, and trusted context.

## 4. Why Now

Market timing is favorable, but still messy:

- McKinsey reported in November 2025 that 88% of surveyed organizations were using AI in at least one business function, yet only about one-third had started scaling AI across the enterprise. That means demand is real, but trust, workflow fit, and operationalization remain unresolved. [McKinsey](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai)
- The same McKinsey survey said 23% of respondents were already scaling agentic AI somewhere in the enterprise, and another 39% were experimenting. Cenote fits that shift toward agentic, multi-step assistance rather than single-turn chat. [McKinsey](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai)
- Cisco's April 2, 2025 privacy benchmark found that 90% of organizations viewed local storage as inherently safer, while 64% worried about accidentally sharing sensitive information through generative AI. That directly supports Cenote's local-first positioning. [Cisco](https://newsroom.cisco.com/c/r/newsroom/en/us/a/y2025/m04/cisco-2025-data-privacy-benchmark-study-privacy-landscape-grows-increasingly-complex-in-the-age-of-ai.html)
- Existing products are validating the category, but also leaving gaps. As of December 5, 2025, Limitless stopped selling new Pendants and made its service unavailable in the EU, UK, and several other regions, which creates whitespace for privacy-first memory products, especially in Europe. [Limitless](https://www.limitless.ai/)

## 5. Ideal Customer Profile

### Primary ICP

Solo professionals and micro-teams with high information load and moderate privacy sensitivity:

- 1 to 20 people
- document-heavy work
- heavy use of WhatsApp or Telegram for real work
- recurring follow-ups, meetings, client context, and deadlines
- reluctance to centralize all raw data in a third-party SaaS

### Best initial verticals

- recruiting and executive search
- strategy and operations consulting
- boutique agencies
- founder office / chief of staff workflows
- independent advisors, coaches, and analysts

These groups are attractive because they:

- feel pain immediately
- buy quickly without enterprise procurement
- live in chat plus documents
- can pay for clear time savings and better recall
- produce repeatable use cases that can later become templates

### Customers to avoid first

- broad enterprise rollouts
- highly regulated healthcare workflows
- legal workflows that require strong matter management and formal compliance guarantees
- generic consumer productivity positioning

Those markets may be viable later, but they will slow the first 12 months.

## 6. Positioning

Recommended positioning:

> Cenote is the privacy-first AI chief of staff that remembers your work, searches your knowledge, and turns conversations and documents into action, without giving the cloud raw access to your vault.

This position is stronger than "life OS" in a commercial setting because it communicates value in business terms:

- memory
- retrieval
- action
- privacy boundary
- ubiquitous interface

## 7. Competitive Landscape

As of April 23, 2026, the reference set looks like this:

| Product | Current position | What it proves | Gap Cenote can own |
| --- | --- | --- | --- |
| [Notion AI](https://www.notion.com/pricing) | Business plan at $20/seat/month with Notion Agent, AI Meeting Notes, and Enterprise Search included | Teams want AI inside existing workspaces | Notion is workspace-centric and SaaS-centric; Cenote can win where local files, sidecars, messaging, and private ingestion matter most |
| [Microsoft 365 Copilot Business](https://www.microsoft.com/en-us/microsoft-365-copilot/pricing) | $18/user/month annual promo or $25.20 monthly, with Work IQ and app integration | Buyers will pay for contextual AI when it is embedded into workflow | Microsoft is strongest inside Microsoft; Cenote should avoid direct enterprise-suite competition early |
| [Otter.ai](https://otter.ai/pricing) | Business plan at $19.99/user/month annual or $24 monthly | Meeting capture and summaries are a real budget line item | Otter is meeting-centric; Cenote can expand from meetings into full knowledge and task memory |
| [Mem](https://get.mem.ai/pricing) | Pro plan at $12/month | Users will pay for AI-native note recall | Mem is lighter-weight and less privacy-assertive than Cenote's local-first architecture |
| [Limitless](https://www.limitless.ai/) | Category pioneer in memory capture, but no new Pendant sales and no EU availability after December 5, 2025 | Demand exists for persistent memory products | Market credibility exists, but the field is still open for a trust-first and Europe-friendly offer |

Strategic conclusion:

- Do not compete on "best generic chat."
- Do not compete on "all-in-one team workspace."
- Compete on trusted memory, local privacy controls, and message-native access.

## 8. Business Model

### Recommended model

Use an open-core plus managed-service structure.

#### Community Edition

- self-hosted local daemon
- local vault indexing
- local ingestion pipeline
- CLI and local web simulator
- community support only

Purpose:

- trust
- developer adoption
- technical validation
- distribution through self-hosting and productivity communities

#### Cenote Pro

Suggested price: $29/month per user

Includes:

- managed cloud orchestration layer
- WhatsApp/Telegram integration
- guided onboarding
- secure update channel
- hosted templates and prompt packs
- email support

#### Cenote Team

Suggested price: $79/month workspace fee plus $24/user/month

Includes:

- shared team memory
- shared task extraction
- role-based admin controls
- audit and activity views
- priority support

#### Private Deployment / Concierge

Suggested price: from $12,000/year

Includes:

- private deployment support
- onboarding and migration
- policy and privacy review
- premium support

### Why this model fits the product

- The local-first architecture is a trust advantage, but it also adds setup friction.
- An open-core edition reduces trust friction.
- A managed paid layer monetizes convenience, onboarding, messaging delivery, and reliable operations.
- This avoids building a business that depends on hoarding user data, which would contradict the product thesis.

## 9. Pricing Logic

Cenote should not underprice itself against generic note AI. The product value is closer to:

- an AI notetaker
- a search layer
- a task extractor
- a memory system
- a lightweight chief-of-staff assistant

That is worth more than a commodity chat subscription if it actually becomes operationally sticky.

Pricing should stay:

- above lightweight note AI
- comparable to business AI utilities
- below enterprise-suite bundle pricing once seat minimums and setup complexity are considered

## 10. Go-To-Market Plan

### Phase 1: Design Partners

Goal: 20 to 30 design partners in one or two tightly chosen verticals.

Recommended starting verticals:

- recruiters
- boutique consultants

Offer:

- white-glove setup
- direct founder support
- discounted pilot pricing
- weekly feedback loop

Primary acquisition channels:

- founder-led outbound
- personal network
- productivity and self-hosting communities
- consultants and recruiter communities on LinkedIn and Telegram
- demo videos showing PDF-to-memory-to-follow-up flows

### Phase 2: Public Waitlist and Open-Core Distribution

Goal: turn privacy and architecture into a growth engine.

Tactics:

- publish the local daemon and privacy boundary clearly
- ship a local demo with fixture vault and messaging simulator
- create use-case templates by vertical
- publish comparisons against "upload everything to SaaS" workflows

### Phase 3: Small-Team Expansion

Once single-user retention is proven:

- add shared workspaces
- add team memory controls
- add admin and audit views
- sell into 5 to 20 person firms already using the product individually

## 11. Sales Motion

Recommended initial sales motion:

- founder-led sales
- high-touch onboarding
- short pilot cycles
- strong case studies

Sales narrative:

1. You already have the information.
2. Your current AI stack cannot remember or safely use it.
3. Cenote keeps private ingestion local, remembers conversations, and works inside chat.
4. That means less context switching and fewer dropped follow-ups.

This is a better early motion than pure self-serve because local-first products live or die on setup success and trust.

## 12. Product Roadmap Aligned To The Existing Spec

The current prompt pack already implies a commercial roadmap:

### Milestone 1: Local Foundation

Prompts 0 to 2

Ship:

- daemon
- schema
- vault indexing
- search
- task listing

Business use:

- searchable private knowledge base

### Milestone 2: Private Ingestion

Prompt 3

Ship:

- PDF/text/markdown ingestion
- local extraction
- summaries
- task candidates
- relationships

Business use:

- document-to-action workflow

### Milestone 3: Durable Memory

Prompt 4

Ship:

- transactional conversation logging
- local embeddings
- episodic recall

Business use:

- assistant that remembers commitments and prior context

### Milestone 4: Real Interfaces

Prompt 5

Ship:

- WhatsApp/Telegram path
- web chat path
- end-to-end orchestration

Business use:

- daily workflow adoption

### Milestone 5: Production Readiness

Prompt 6

Ship:

- packaging
- observability
- backup and restore
- runbooks

Business use:

- sellable pilots and supported deployments

## 13. Key Metrics

Do not measure success primarily by signups. Measure operating stickiness.

Core metrics:

- activation rate: percent of new users who connect a vault and ingest at least 10 documents in the first 7 days
- time to first useful answer
- weekly active users
- 4-week retention
- average recalled memories per active user
- tasks created per active user
- follow-up completion rate on extracted tasks
- messages per active day
- gross margin per active user
- support time per new account

Design-partner success thresholds:

- at least 60% weekly retention after 30 days
- at least 3 active days per week for retained users
- at least 10 meaningful interactions per active week
- at least 30% of sessions using search, recall, or ingestion rather than pure chat

## 14. Base-Case 3-Year Model

This is a planning model, not a forecast.

| Year | Primary objective | Paid individuals | Paid teams | End-of-year MRR | Annualized ARR |
| --- | --- | ---: | ---: | ---: | ---: |
| 1 | Prove activation and retention with design partners and early public launch | 250 | 20 | ~$13k to $16k | ~$160k to $190k |
| 2 | Turn repeatable pilots into small-team growth | 1,000 | 75 | ~$60k to $75k | ~$720k to $900k |
| 3 | Build a durable niche in privacy-sensitive professional services | 2,500 | 200 | ~$180k to $240k | ~$2.2M to $2.9M |

Assumptions:

- individual ARPU near $29/month
- small teams start around 5 to 10 seats plus workspace fee
- team mix grows over time
- churn improves only after onboarding and messaging reliability improve

This is intentionally conservative. The product should earn expansion through retention, not vanity growth.

## 15. Team Plan

Initial team:

- founder/CEO: product and GTM
- technical founder or lead engineer: Rust daemon, local infra, reliability
- product/full-stack engineer: onboarding, web UI, billing, admin
- AI/platform engineer: orchestration, local model pipeline, evaluation

Part-time or contract support:

- design
- security/privacy review
- developer relations / content

## 16. Main Risks

### 1. Setup friction

Risk:

- local daemons, vault paths, and model setup can kill adoption before value is proven

Mitigation:

- guided installer
- doctor command
- local demo mode
- white-glove onboarding for first customers

### 2. Messaging platform risk

Risk:

- WhatsApp or Telegram integrations may change policies, pricing, or reliability

Mitigation:

- keep provider-neutral adapter layer
- maintain web UI as a first-party fallback

### 3. Privacy promise must be provable

Risk:

- local-first claims are useless if users cannot verify them

Mitigation:

- open-core architecture
- explicit privacy docs
- tests proving raw content does not cross the cloud boundary
- simple audit logs

### 4. Competition from bundled suites

Risk:

- Microsoft and Notion can bundle adjacent features cheaply

Mitigation:

- avoid broad suite competition
- win where local files, messaging, and trust matter
- target users with cross-tool fragmentation and non-Microsoft/non-Notion realities

### 5. Weak moat if the product is too generic

Risk:

- local-first by itself is not a moat

Mitigation:

- build durable workflow lock-in through memory, tasks, templates, and messaging habits
- become excellent at one or two vertical workflows before expanding

## 17. Strategic Recommendation

The best version of Cenote is:

- open-core
- local-first
- premium-priced
- tightly focused on privacy-sensitive solo professionals and small firms
- sold through trust, daily workflow fit, and reliable memory

The worst version of Cenote is:

- generic
- broad
- cheap
- competing directly with Microsoft, Notion, and general-purpose chat apps

## 18. Immediate Next Business Steps

1. Rewrite the public product narrative from "hybrid life OS" to "privacy-first AI chief of staff for document-heavy work."
2. Pick one initial vertical and recruit 10 design partners before polishing for general release.
3. Build the first end-to-end demo around one killer loop:
   receive a document, extract tasks locally, recall it later from WhatsApp, and commit the interaction to memory.
4. Create pricing and packaging pages early, even if manual billing is used at first.
5. Publish the privacy architecture clearly enough that a skeptical technical buyer can understand it in five minutes.
6. Treat onboarding, observability, and supportability as part of the product, not post-launch cleanup.

## 19. Barcelona-Specific Founder-Led Sales Plan

This section assumes the founder lives in Barcelona, is fluent in English, speaks some Spanish, works in VC, and also has access to a local education-oriented network.

### Core conclusion

Do not start by trying to sell Cenote to "Barcelona" as a whole. Start by selling to the international, English-friendly, document-heavy professional layer inside Barcelona.

That is the right layer because:

- Barcelona is the best startup ecosystem in southern Europe and the 5th best in the EU according to ACCIÓ's 2026 ecosystem analysis
- the same ACCIÓ report says Barcelona is the top EU hub by percentage of international startup founders
- that means an English-first sales motion is commercially viable locally
- your VC network gives you warm access to founders, operators, recruiters, and small professional-service firms that match Cenote's ICP

### Best wedges for your situation

#### Wedge 1: VC-backed founders and operators in Barcelona

Best buyer types:

- founders
- chiefs of staff
- founder associates
- operations leads
- recruiting leads
- investor relations or platform operators

Why this wedge fits:

- they already live in English
- they handle fragmented context across decks, PDFs, notes, and chats
- they care about speed and follow-through more than polished enterprise procurement
- your VC role gives you the cleanest path to warm intros

Best promise:

- "Cenote remembers your work, keeps sensitive material local, and turns scattered documents and chats into searchable follow-up."

#### Wedge 2: Recruiters, executive search, and boutique advisors

Why this wedge fits:

- they are heavy users of WhatsApp, email, notes, candidate profiles, and follow-ups
- they have immediate pain around memory, recall, and action extraction
- they can buy quickly as solo professionals or micro-teams

Best promise:

- "Cenote is your private search and follow-up memory for candidate, client, and meeting context."

#### Wedge 3: Education-adjacent operators, not broad edtech

Use this wedge selectively.

Good early buyers:

- admissions
- student success
- career services
- programme managers
- training coordinators

Why this wedge is secondary:

- your local education network can give you discovery access
- but education institutions often buy more slowly than startups and recruiters
- use this channel first for interviews, pilot users, and credibility, not as the main revenue engine unless you already have direct access to decision-makers

### Positioning for Barcelona

Lead with:

- privacy-first
- local-first
- English-friendly
- WhatsApp and Telegram native
- built for people who run their work from documents and chat

Avoid leading with:

- "life OS"
- consumer productivity
- broad AI assistant language
- generic note-taking

Recommended one-line pitch:

> Cenote is a privacy-first AI chief of staff for founders, recruiters, and operators who work from documents and chat and do not want to send raw private files into a generic SaaS.

### Language strategy

Run the commercial motion in English first.

Specifically:

- keep the landing page and primary demo in English
- prepare a short Spanish one-pager and FAQ for local prospects who prefer to read in Spanish even if the demo is in English
- do not wait for perfect Spanish fluency before selling
- use Spanish mainly to reduce friction at the edges: follow-up emails, event intros, simple support material, and local admin conversations

The goal is not to look "fully local" on day one. The goal is to win inside the part of Barcelona that already transacts in English.

### 90-day execution plan

#### Days 1 to 14: sharpen the sales asset stack

Build the minimum founder-sales kit:

- one strong landing page
- one 2-minute demo video
- one privacy architecture page
- one design-partner offer
- one short onboarding checklist

Create three demo variants only:

- founder / chief of staff workflow
- recruiter workflow
- admissions / programme manager workflow

Each demo should show the same core loop:

1. ingest a private document locally
2. extract tasks or entities
3. ask a follow-up question later through chat
4. show recalled context and a concrete next action

#### Days 15 to 45: recruit the first 10 design partners

Target composition:

- 5 from your VC and founder network
- 3 from recruiters or consultants
- 2 from education-adjacent operators

Run this as a founder-led sprint:

- make a target list of 50 names
- ask for 3 to 5 warm intros per week
- run 4 to 6 live demos per week
- close 1 to 2 pilots per week

Pilot structure:

- 4 to 6 weeks
- white-glove onboarding
- weekly founder check-in
- explicit agreement to share feedback

Pricing recommendation:

- free only for the first 2 or 3 lighthouse users if they provide high-value feedback or credibility
- charge everyone else a paid design-partner fee, even if symbolic
- the point is to test seriousness, not maximize revenue yet

#### Days 46 to 90: turn pilots into references and repeatability

For every active pilot, capture:

- setup time
- first-use success
- number of recalled items
- number of tasks extracted
- specific moments where Cenote saved time or prevented a dropped follow-up

Convert that into:

- 2 written case studies
- 3 testimonial quotes
- 1 quantified ROI story per wedge

Then expand outbound using the winning wedge only.

### Practical outreach system

Use a narrow, repeatable cadence rather than broad awareness marketing.

Weekly targets:

- 10 warm reach-outs
- 10 cold but highly relevant reach-outs
- 5 live demos
- 2 pilot proposals
- 1 closed pilot

Best warm-intro script:

> I am building Cenote, a privacy-first AI chief of staff that keeps raw files local and helps founders and operators search prior context, extract follow-ups, and work from chat. I am looking for 5 design partners in Barcelona who handle lots of documents and sensitive context. Would you introduce me to one or two people who fit that profile?

Best cold-opening angle:

- mention one concrete workflow
- mention the privacy boundary
- ask for a short demo, not a purchase

Do not open with "AI assistant." Open with the broken workflow you fix.

### Local channel plan

Use Barcelona communities as distribution multipliers, not as the primary closing channel.

High-priority channels:

- your VC firm's portfolio, founder friends, operators, and service providers
- Tech Barcelona for density of startup operators and ecosystem visibility
- Startup Grind Barcelona because it is explicitly English-first and has a large local founder base
- ProductTank Barcelona for product and operations people who may become early users or referrals
- Barcelona Activa for advisor access, startup programs, and local ecosystem visibility

Secondary channels:

- Norrsken House Barcelona if you want impact, international founder, and community density
- AI Tinkerers Barcelona if you want technical early adopters, builders, and future advocates
- EdTech Congress Barcelona only after you have an education-specific proof point

### What to do at local events

Do not attend events to "network." Attend with one of three goals:

- book 5 demos
- test one sharper message
- find one partner community host for a private demo session

Useful event formats:

- breakfast demo for founders and chiefs of staff
- private roundtable on AI privacy for recruiters and advisors
- live workflow teardown: from PDF to follow-up in WhatsApp

The best event asset is not a booth. It is a small-room demo where the privacy story is obvious.

### What to avoid

- trying to sell to large Spanish enterprises early
- depending on broad paid ads
- translating everything before selling
- selling to students as end users first
- pitching "knowledge management" without a concrete action loop
- taking too many free pilots with no urgency

### Success criteria for the first local sales cycle

At the end of 90 days, success looks like:

- 10 design partners recruited
- at least 6 actively using the product after onboarding
- at least 3 users who say they would be unhappy if Cenote disappeared
- at least 2 written references from Barcelona-based users
- one wedge clearly outperforming the others on retention and urgency

If that happens, the next move is to double down on the winning wedge and only then broaden beyond the Barcelona founder network.

## 20. Sources

Official and current sources used for the market snapshot:

- [McKinsey, The State of AI 2025](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai)
- [Cisco 2025 Data Privacy Benchmark Study](https://newsroom.cisco.com/c/r/newsroom/en/us/a/y2025/m04/cisco-2025-data-privacy-benchmark-study-privacy-landscape-grows-increasingly-complex-in-the-age-of-ai.html)
- [Notion Pricing](https://www.notion.com/pricing)
- [Microsoft 365 Copilot Pricing](https://www.microsoft.com/en-us/microsoft-365-copilot/pricing)
- [Otter.ai Pricing](https://otter.ai/pricing)
- [Mem Pricing](https://get.mem.ai/pricing)
- [Limitless homepage](https://www.limitless.ai/)
- [OpenAI Enterprise Privacy](https://openai.com/enterprise-privacy/)
- [ACCIÓ, Analysis of the startup ecosystem in Catalonia 2026](https://www.accio.gencat.cat/web/.content/bancconeixement/documents/pindoles/ACCIO-analisi-ecosistema-startup-catalunya-2026-pindola-en.pdf)
- [Tech Barcelona](https://www.techbarcelona.com/en/)
- [Barcelona Activa Entrepreneurship](https://emprenedoria.barcelonactiva.cat/en/home)
- [Startup Grind Barcelona](https://www.startupgrind.com/barcelona/)
- [ProductTank Barcelona](https://www.meetup.com/producttank-barcelona/)
- [Norrsken House Barcelona](https://www.norrsken.org/houses/barcelona)
- [AI Tinkerers Barcelona](https://barcelona.aitinkerers.org/)
- [EdTech Congress Barcelona](https://www.edtechcongressbcn.com/?lang=en)
