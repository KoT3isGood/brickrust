# CLAUDE.md

This file guides the Codex/Claude engineering skills that read from and write to this repo.
See the per-topic files under `docs/agents/` for the details each skill depends on.

## Agent skills

### Issue tracker

Issues and specs live as GitHub Issues in `KoT3isGood/brickrust`, driven by the `gh` CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

Five canonical triage role labels (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`), each string equal to its name. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context layout: one `CONTEXT.md` at the repo root plus `docs/adr/`. See `docs/agents/domain.md`.
