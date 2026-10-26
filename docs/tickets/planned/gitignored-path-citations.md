---
needs: []
---

**Tracked code cites gitignored `docs/superpowers/…` paths — 1 site
remaining.** (The runtime-printed site and the `deckmaste_spelling` test sites
went with `retire-v1-english-and-spelling`.) Flagged by macro-frames round 2's Task 3
review; all predate that round and sat outside its declared scope
(recorded in the round-2 ledger, "Corrected line numbers" note).

- `crates/macro_ron/src/frames.rs:47`

Line numbers are as of round-2 exit — re-grep at claim time:
`rg 'docs/superpowers|docs/memory' crates plugins`.

## Fix

Per the comments-state-what-code-cannot genre: inline the fact the
comment actually needs, or re-point at a tracked doc
(`docs/decisions/…`).

## Guard

Add the grep as a cheap standing check (CI or an xtask lint) so new
sites can't land silently.

## Scope note

Tickets under `docs/tickets/` citing shared-local docs are established
practice (provisioning symlinks `docs/superpowers/` into every
workspace) and are NOT in scope. The rule bites where a fresh checkout
or runtime output dangles: `crates/` and `plugins/`.
