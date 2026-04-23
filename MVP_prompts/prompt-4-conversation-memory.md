# Prompt 4: Memory Permanence And Conversational Recall

Run this after `prompt-3-local-hive-ingestion.md`.

## Prompt
Continue implementing Cenote in the same repository. Preserve all prior behavior and extend it with durable conversation logging and recall.

## Goal
Ensure every conversation turn is stored locally, embedded locally, and retrievable through MCP tools so the assistant can build episodic memory over time.

## Deliverables
Implement the following:

1. **Conversation model**
- Represent channels, sessions, users, and messages in the database.
- Support metadata such as:
  - source channel
  - external message ID
  - timestamps
  - role
  - tool usage metadata
  - attachments or referenced file paths

2. **`commit_interaction` tool**
- Implement a real tool, not a placeholder.
- Accept:
  - user input
  - assistant output
  - session metadata
  - optional tool invocation metadata
- Persist both turns transactionally.
- Be idempotent when the same external message ID is replayed.
- Emit data needed for later memory recall and task mining.

3. **Conversation embeddings**
- Embed stored turns locally using the configured local embedding path.
- Store embeddings through the vector index abstraction.
- Keep linkage back to the original message rows.

4. **`recall_memories` tool**
- Accept a query plus optional filters like channel, date range, and limit.
- Return concise, ranked memories with timestamps and session references.
- Prefer semantically relevant excerpts, not entire transcripts.

5. **Implicit task extraction from chat**
- Mine user intents from conversation history.
- Create or update task records when the user expresses actionable intents.
- Store source references so a task can point back to the originating interaction.
- Avoid generating duplicate tasks on repeated recalls or replays.

6. **Memory compaction helpers**
- Add a background capability for generating rolling conversation summaries or checkpoints.
- Keep raw message history intact; summaries are additive, not destructive.

7. **Tests**
- Test that repeated commits with the same message ID do not duplicate rows.
- Test that memories can be recalled semantically from prior turns.
- Test that implicit task extraction creates stable task entries.
- Test that `commit_interaction` failures are surfaced clearly and do not half-write state.

## Constraints
- Every frontend or orchestration path added later must use `commit_interaction`.
- Do not rely on a cloud embedding service.
- Keep message permanence local by default.

## Acceptance Criteria
- A conversation can be committed and recalled end to end.
- Memory search is local and linked back to original sessions.
- Task extraction from chat works on realistic fixture conversations.
- Prior prompt functionality remains intact.

## Final Response Requirements
When you finish, report:
- the final `commit_interaction` contract
- how idempotency is enforced
- how memory recall is ranked
- commands executed
- test results
- anything Prompt 5 must still wire together
