# Prompt 5: Frontends, Messaging, And Cloud Orchestration

Run this after `prompt-4-conversation-memory.md`.

## Prompt
Continue implementing Cenote in the same repository. Read the current code first and wire the completed local capabilities into real interaction surfaces.

## Goal
Connect the Rust daemon to messaging and web frontends, and add the cloud orchestration layer that reasons over tool outputs without bypassing local privacy controls.

## Deliverables
Implement the following:

1. **Provider-neutral orchestration layer**
- Create an abstraction for the cloud reasoner so the daemon is not hard-coded to a single vendor.
- The cloud layer must only access vault data through MCP tools or other explicitly cloud-safe local APIs.
- Centralize the system prompt and safety policy in configuration or templated assets.

2. **System instruction contract**
- Use a system prompt equivalent to this policy:
  - you are the Cenote Cerebrum
  - you do not have direct file access
  - use `ingest_new_file` for new private content
  - use `search_vault` and `recall_memories` for context
  - use `list_tasks` for actionable work
  - every completed turn must be persisted through `commit_interaction`
- Keep the instruction editable without recompiling the whole app.

3. **Messaging adapter**
- Integrate OpenClaw, or build against its webhook/event model, for WhatsApp and Telegram delivery.
- Normalize inbound events into an internal message format.
- Normalize outbound replies back to provider-specific payloads.
- Include signature verification or webhook authentication where applicable.
- If a real provider handshake cannot be exercised locally, ship a local simulator so the rest of the flow is testable.

4. **Web chat adapter**
- Add a browser-facing chat integration using LobeChat or Open WebUI.
- Prefer the path that requires the least custom glue while preserving the daemon as the system of record.
- The web path must still use the same orchestration and permanence flow as messaging.

5. **End-to-end conversation loop**
- Implement the flow:
  - receive inbound user message
  - load session context
  - recall relevant memories
  - call the cloud reasoner with tool access and privacy rules
  - execute any tool calls through the daemon
  - persist the final turn through `commit_interaction`
  - send the reply back to the originating frontend

6. **Operational safeguards**
- Add timeouts, retries, and error handling for provider outages.
- Prevent partial sends where a reply is delivered but not persisted, or persisted but not associated with a session.
- Log tool and provider failures with traceable correlation IDs.

7. **Tests and local demo path**
- Add at least one integration test or scripted demo for a full conversation loop.
- Add a local development mode that can simulate inbound messages without real WhatsApp or Telegram accounts.

## Constraints
- Do not let frontend adapters bypass the daemon and talk directly to local storage.
- Do not let the cloud reasoner read raw private files.
- Keep orchestration modular so providers can be swapped later.

## Acceptance Criteria
- A local developer can run the daemon and simulate an inbound chat message end to end.
- Messaging and web adapters share the same core conversation flow.
- The system prompt and privacy policy are explicit and testable.
- Every completed exchange is written through `commit_interaction`.

## Final Response Requirements
When you finish, report:
- which web UI path you chose and why
- how OpenClaw was integrated or simulated
- the end-to-end message flow
- commands executed
- test results
- what remains before the system is production-ready
