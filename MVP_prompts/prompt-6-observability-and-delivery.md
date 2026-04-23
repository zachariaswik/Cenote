# Prompt 6: Observability, Packaging, And Production Readiness

Run this after `prompt-5-frontends-and-cloud.md`.

## Prompt
Continue implementing Cenote in the same repository. This step is for hardening, packaging, and making the project runnable and maintainable by another engineer.

## Goal
Bring the system to a production-shaped state: observable, reproducible, documented, and safe to operate locally or on a small personal server.

## Deliverables
Implement the following:

1. **Observability**
- Expand tracing across ingestion, search, memory recall, tool execution, and frontend delivery.
- Add metrics or at least structured counters for:
  - ingestion jobs
  - tool invocations
  - search latency
  - recall latency
  - outbound delivery failures
- Expose a health/readiness surface suitable for local ops.

2. **Packaging**
- Add Docker and/or Compose assets for:
  - Rust daemon
  - Ollama
  - local persistent volumes
- Keep local bare-metal development working too.
- Add a sample bootstrap script for first-run setup if useful.

3. **Runbooks and docs**
- Update the README with:
  - architecture overview
  - setup steps
  - local development flow
  - how to configure providers
  - how to verify privacy boundaries
- Add troubleshooting notes for common failures such as missing models, DB locks, or webhook issues.

4. **Data lifecycle**
- Add backup and restore guidance for the local DB, vector index, and sidecar data.
- Ensure migrations are safe and documented.
- Add a reindex path for rebuilding search and memory indexes from source data.

5. **Quality gates**
- Add a test command sequence for CI or local verification.
- Include formatting, linting, and test instructions.
- If there is a CI configuration in repo scope, wire it up. If not, add one.

6. **Security and secrets handling**
- Keep secrets in env/config, not in code.
- Document provider secret requirements.
- Ensure logs avoid dumping raw private content unless an explicit debug mode is enabled.

7. **Demo and verification artifacts**
- Add a small end-to-end demo script or walkthrough that proves:
  - vault indexing works
  - private ingestion works
  - memory permanence works
  - chat routing works

## Constraints
- Harden what already exists; do not replace working components without a strong reason.
- Keep the local-first privacy model intact.
- Do not leave the system dependent on undocumented manual steps.

## Acceptance Criteria
- A new developer can set up and run the full stack from the repository docs.
- The project has clear health checks, logs, and verification steps.
- Backup/reindex paths are documented and tested where practical.
- The system is ready for iterative real-world use, not just a prototype demo.

## Final Response Requirements
When you finish, report:
- runtime topology
- how to start the stack
- quality gates added
- commands executed
- test results
- remaining production risks, if any
