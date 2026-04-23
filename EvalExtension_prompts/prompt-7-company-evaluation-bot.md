# Prompt 7: Business Owner Snapshot And Company Intelligence

Run this after `prompt-6-observability-and-delivery.md`.

## Prompt
Continue implementing Cenote in the same repository. Read the current code first and extend the product with a one-click business intelligence feature for company owners and operators.

Build this feature as a single-company business snapshot for the current authenticated company.
Optimize it for business owners and operators who want a clear view of the current state of their business.

## Goal
Add a one-click flow inside the SaaS application that generates a current business snapshot for the logged-in company.

The output should help a business owner understand:

- what is happening in the business right now
- how revenue, expenses, and cash movement are trending
- what changed since the last snapshot
- what important discussions, decisions, and risks are emerging
- what is happening in sales, operations, product, and engineering
- what requires attention next

It is an internal operating view built from the company's own private data, conversations, documents, and systems.

## Product Definition

Cenote already acts as the company's private memory layer:

- company communication is captured and persisted
- files and documents are stored in local or private hosted storage such as S3
- local models process sensitive information before anything cloud-facing is produced
- conversations, notes, tasks, and knowledge become retrievable over time

This feature should build directly on that foundation.

The business snapshot must use the company's own memory and records, including where available:

- sales history
- invoices
- incoming payments
- outgoing expenses
- bank or accounting exports
- CRM activity
- internal chats and discussions
- meeting transcripts
- notes and uploaded documents
- contracts and proposals
- project plans
- product discussions
- code and implementation history
- prior Cenote memories, tasks, and summaries

The experience should feel like pressing one button to ask:

`Show me the current state of my business.`

## Core UX

The primary entry point is a single action in the SaaS app, for example:

- `Generate Business Snapshot`

When the user clicks it, Cenote should:

1. gather the latest company data and memory
2. normalize structured metrics and relevant events
3. retrieve grounded evidence from communications and files
4. build a clear owner-facing summary
5. render charts and trend views
6. persist the snapshot so it can be compared with future runs

The result should be a visual and narrative dashboard with charts, cards, trends, and evidence-backed narrative sections.

## Deliverables

Implement the following:

1. **Single-company scope**
- Bind every snapshot run to the current authenticated tenant or company.
- Present the current business state for that company and compare it against prior snapshots from that same company.

2. **Company memory foundation**
- Extend Cenote's storage and retrieval layer so the company has a unified private memory surface.
- Support storage in:
  - local storage
  - private object storage such as S3
- Keep raw private artifacts under Cenote's policy control.
- Ensure local models can retrieve and process the company's knowledge across:
  - communications
  - documents
  - financial exports
  - CRM records
  - product and code artifacts
- Keep provenance so any business claim can be traced back to sources.

3. **Business data model**
- Define a normalized company intelligence schema, for example:
  - `CompanySnapshotRun`
  - `BusinessMetricSeries`
  - `BusinessEvent`
  - `BusinessInsight`
  - `EvidenceReference`
  - `ChartSpec`
- The model should support both:
  - structured numeric time-series data
  - unstructured evidence from discussions, notes, and files

4. **Data intake and normalization**
- Build a pipeline that can assemble a current view of the company from multiple input types.
- Support, where available:
  - revenue and sales records
  - expense records
  - cash in / cash out
  - invoices and payment status
  - pipeline and CRM data
  - support or customer conversations
  - team discussions
  - product roadmap discussions
  - engineering activity such as commits, tickets, changelogs, and release notes
- Normalize time windows so the system can compare:
  - week over week
  - month over month
  - quarter over quarter where enough data exists
- When exact numeric data is missing, surface qualitative findings from company memory and label them with confidence and freshness metadata.

5. **Rethought AI workflow**
- Implement a business snapshot workflow with these stages:

  1. **Data refresh**
  - load the latest structured and unstructured company data
  - validate freshness and note missing inputs

  2. **Metric synthesis**
  - compute canonical business metrics and time-series views
  - prepare chart-ready data for revenue, expenses, cash movement, pipeline, and other available metrics

  3. **Memory retrieval**
  - retrieve relevant discussions, decisions, tasks, files, and implementation history that explain what changed
  - include operational context from sales, finance, product, and engineering

  4. **Insight generation**
  - generate owner-facing findings grounded in both metrics and evidence
  - explain the drivers behind metric movement

  5. **Narrative assembly**
  - produce a concise executive snapshot and deeper drill-down sections
  - organize output by business area instead of by AI subsystem

  6. **Visualization and persistence**
  - render charts, cards, deltas, and supporting evidence links
  - store the run so the next snapshot can compare against it

6. **Owner-facing sections**
- The snapshot should be organized into practical sections a business owner would care about, such as:
  - overall business summary
  - revenue and sales
  - expenses and cash flow
  - customers and pipeline
  - operations and execution
  - product and engineering
  - risks, blockers, and unresolved decisions
  - next actions
- The exact sections can adapt to available data, but finance and core operations should always be first-class.

7. **Visualizations**
- The UI must include graphics alongside prose.
- Support charts such as:
  - sales over time
  - revenue by period
  - expenses by period
  - cash in vs cash out
  - pipeline funnel or stage distribution if CRM data exists
  - customer concentration if data exists
  - product or engineering throughput if data exists
- Every chart should indicate:
  - time range
  - freshness
  - source quality
- If data is incomplete, show a partial chart with a warning rather than pretending precision.

8. **Narrative intelligence**
- Generate a written owner summary that answers questions like:
  - What changed since the last snapshot?
  - Are sales improving or slipping?
  - Are expenses rising faster than revenue?
  - What big decisions were discussed recently?
  - What customer, product, or engineering issues are most likely to affect the business?
  - What should the owner pay attention to this week?
- Tie narrative claims to evidence from:
  - numeric trends
  - discussions
  - documents
  - task history
  - code or delivery records where relevant

9. **Evidence and drill-down**
- Every important insight should be inspectable.
- Let the user drill into:
  - source transactions or aggregates
  - referenced discussions or meeting summaries
  - related tasks
  - related documents
  - related product or engineering artifacts
- The product should feel like an intelligent dashboard backed by the company's own memory, with inspectable evidence for each important claim.

10. **Snapshot history**
- Persist every generated snapshot as a time-stamped business state.
- Let the UI compare the current snapshot against prior runs for the same company.
- Support trend deltas such as:
  - revenue up or down vs last snapshot
  - expense shifts
  - new risks
  - resolved blockers
  - changes in product delivery tempo
- Historical comparison must stay within the same company.

11. **Action layer**
- Each snapshot should produce a practical action layer for the owner, such as:
  - top issues to review
  - follow-ups to make
  - questions needing answers
  - tasks to create or confirm
- The action layer should connect back into Cenote's existing task and memory systems.

12. **Privacy and storage rules**
- Preserve Cenote's local-first privacy boundary.
- Raw private company data should remain in local or private controlled storage where possible.
- If cloud reasoning is used, it must operate only on safe, processed, policy-approved outputs.
- Keep logs limited to safe operational metadata, status information, and policy-approved diagnostics.
- Make storage location and privacy boundaries understandable in the UI.

13. **Operational behavior**
- The snapshot must be runnable on demand from one click.
- If the run takes time, process it in the background and stream status updates.
- Runs should be idempotent enough to survive retries without corrupting history.
- Missing data should degrade confidence, not break the feature.
- The system should distinguish clearly between:
  - measured facts
  - inferred findings
  - missing inputs

14. **Output formats**
- Store machine-readable outputs for the application and future automation.
- Store human-readable outputs for inspection and export.
- Output should include at least:
  - summary cards
  - chart data or chart specs
  - narrative sections
  - evidence references
  - confidence and freshness metadata
  - suggested actions

15. **Tests**
- Test that the snapshot can only run for the current company.
- Test structured metric ingestion and time-series generation.
- Test retrieval of supporting evidence from company memory.
- Test chart generation with complete and partial data.
- Test that narrative insights are linked to evidence and freshness metadata.
- Test history comparisons across multiple runs for the same company.
- Test privacy boundary behavior for local files and private object storage.

## Constraints

- Optimize for the question: "what is happening inside my business, why, and what should I do next?"
- Keep the implementation compatible with Cenote's existing local-first memory, ingestion, and chat architecture.

## Acceptance Criteria

- A logged-in company can click one button and generate a current business snapshot.
- The output includes visual charts for revenue, expenses, and other available business metrics.
- The output includes grounded narrative insights based on the company's own communications, files, and records.
- The system can use local or private hosted storage such as S3 as part of the company memory layer.
- The product can surface finance, sales, operations, and product or engineering context in one coherent view.
- Snapshot history is stored and comparable over time for the same company.
- The feature consistently presents the current company and its own historical business state.

## Final Response Requirements
When you finish, report:
- the final business snapshot workflow
- how company memory is stored and retrieved
- how structured metrics and unstructured evidence are combined
- the chart and dashboard surfaces added
- the storage model for snapshot history
- the privacy boundary for local and private hosted storage
- commands executed
- test results
- remaining implementation gaps
