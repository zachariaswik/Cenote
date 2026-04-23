# Project Notes

Some thoughts on the Cenote daemon. #project #rust

## Goals

- Keep raw files local.
- Use Ollama for local extraction.
- Surface cloud-safe summaries only.

## Open items

- [ ] Wire up the OpenClaw simulator
- [ ] Document the privacy boundary in README
- [ ] Review hybrid ranking weights

## Context

The Cerebrum is cloud. The Nervous System is local. Raw vault content never leaves this machine unless the user explicitly exports a summary.
