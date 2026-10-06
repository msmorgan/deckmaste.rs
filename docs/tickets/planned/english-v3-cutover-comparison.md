---
needs: []
---
# Compare the legacy parser and v3 by identity, and inventory legacy consumers

`english-v3-production-cutover` requires an explicit list of every unit the
legacy parser read correctly and v3 does not, and a migration of every consumer.
Nothing produces either today. This ticket produces both as evidence; it
changes no grammar and migrates no consumer.

The legacy parser is the `deckmaste_english` crate (called v2 in older
tickets). It is still a dependency of `xtask` and of `deckmaste_spelling`.

1. Identity comparison. For every supported face, report whether the legacy
   parser reads it, whether v3 covers it, and for faces both read whether v3
   has a Reading with the legacy analysis's structure where that can be stated.
   Output the faces legacy reads and v3 does not as a named list. First
   establish what per-identity record of legacy coverage exists (the coverage
   lock, a runnable legacy corpus command, archived reports); if none can be
   recovered or regenerated, STOP and report, since the cutover criterion then
   needs restating by the user. "Read correctly" is not decidable by tooling:
   the list is of faces legacy read at all, and classifying a legacy analysis
   as wrong stays a per-identity judgement with a grammatical witness, made at
   cutover.

2. Consumer inventory. List every use of legacy parser types outside the
   `deckmaste_english` crate, with the file, what it consumes (parse result,
   projection, selection, rendering) and what its v3 equivalent would be or
   that none exists. Starting points found on 2026-10-05, not a complete list:
   `deckmaste_spelling` (the `ConstructionProjection` / `Projected*` types),
   and in `crates/xtask/src/` the `english/` subcommands (bracket, inspect,
   unknown, recovery, roundtrip, shapes, lint, probe, performance) and
   `macros/residuals.rs`. Note any planned ticket that says "parsed English"
   without naming the parser.

Acceptance: both lists written into `english-v3-production-cutover` (or a
report it links), with the commands that regenerate them; the comparison is
reproducible from a clean tree. Report artifacts stay in ignored paths.
Standard constraints apply.
