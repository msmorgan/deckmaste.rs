---
needs: []
---

**Tracked code cites gitignored `docs/superpowers/…` paths — 3 files, 4
sites, one printed at runtime.** Flagged by macro-frames round 2's Task 3
review; all predate that round and sat outside its declared scope
(recorded in the round-2 ledger, "Corrected line numbers" note).

- `crates/xtask/src/macros/pilot.rs:134` — the worst: the
  `G5_FINDINGS_FILE` constant holds a gitignored path and is **printed
  into gate output on failure**, directing users to a file absent from
  any fresh checkout.
- `crates/macro_ron/src/frames.rs:47`
- `crates/deckmaste_frames/tests/pilot.rs:22` and `:122`

Line numbers are as of round-2 exit — re-grep at claim time:
`rg 'docs/superpowers|docs/memory' crates plugins`.

## Fix

Per the comments-state-what-code-cannot genre: inline the fact the
comment actually needs, or re-point at a tracked doc
(`docs/decisions/…`). For `pilot.rs`, replace the printed pointer with
the inline divergence explanation the gate already computes (or a
tracked doc), so failure output is self-sufficient in a fresh checkout.

## Guard

Add the grep as a cheap standing check (CI or an xtask lint) so new
sites can't land silently.

## Scope note

Tickets under `docs/tickets/` citing shared-local docs are established
practice (provisioning symlinks `docs/superpowers/` into every
workspace) and are NOT in scope. The rule bites where a fresh checkout
or runtime output dangles: `crates/` and `plugins/`.
