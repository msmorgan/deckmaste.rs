---
needs: [lean-v3-proofs-and-cards]
---
**[port-required: semantics-v3] Add a read-only xtask check that compares every
Lean workbench card witness's printed frame to `data/derived/cards.jsonl`.**
Cleanroom review 2026-09-03, R7, ruled in scope now; port from Idris
`Macros.card` to Lean card witnesses in `lean/Semantics/Cards/` (`semantics-v3`).

Nothing today compares a Lean card witness in `lean/Semantics/Cards/` to the
corpus, so wrong printed frames in bench witnesses typecheck. The check
compares **data, not rendering**: name → printed mana cost, P/T box,
supertypes, type line. The `rendering-is-not-the-workbench` ruling rules out
diffing bench text against oracle text; comparing printed characteristics is
not that, and this ticket does not do it.

## Shape

A new `cargo xtask` subcommand beside `map` (`crates/xtask/src/`, registered in
the `Cmd` enum in `crates/xtask/src/bin/cargo-xtask.rs`). It parses the card
witness headers out of `lean/Semantics/Cards/`, looks each name up in
`data/derived/cards.jsonl`, and reports every mismatch as `card — field:
bench=… corpus=…`. Read-only: it writes nothing, and it runs from the
workspace root so a feature workspace's symlinked `data` resolves.

A name absent from the corpus is a reported skip, not a failure — the bench
carries synthetic witnesses on purpose. Exit nonzero only on a real mismatch,
so the command is usable as a positive gate artifact in a landing record.

Note for the implementer: `Macros.card`'s cost slot spells `{0}` two ways in
the bench today (`Nothing` and `Just []`); decide which the check treats as
canonical and say so in the landing record — correcting the bench itself is
`workbench-bench-fidelity`.

Size: M.

Done when: the subcommand exists, is registered, and `cargo xtask <name>
--help` documents it; run from the workspace root it reads
`data/derived/cards.jsonl` and no other input, writes nothing, and prints one
line per mismatch plus a total; it exits nonzero on a mismatch and zero on a
clean bench; its current output — the mismatch count over today's bench — is
recorded in the landing record as the baseline
`workbench-bench-fidelity` must drive to zero; a unit test covers one seeded
mismatch of each compared field. Standard constraints apply.
