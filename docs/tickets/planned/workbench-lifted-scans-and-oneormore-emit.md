---
needs: []
---
# Generate the lifted scans; fill the `OneOrMore` emit arm

Split out of `workbench-structure-phrase-and-toolchain` (2026-08-22): both
sections below are toolchain work — a Rust emit arm in
`crates/deckmaste_plugin/src/idris_emit.rs` and a generator feeding
`idris/scripts/build` — independent of that ticket's grammar merges, which
stay there. Neither touches a grammar constructor. The generator step belongs
beside `idris/scripts/emit-tables`, never in a deletion-slated crate. Read
finding 534 in `docs/memory/archive/binder-unification-probe-experiment-log.md`
before writing the `OneOrMore` arm (the counted-group/universal mark is
`CountD`).

## Generate the lifted scans instead of hand-writing them — NOT a grammar round

One predicate lifted over a mutually-recursive family of indexed datatypes costs
six hand-written total functions, each a case analysis whose non-leaf rows
delegate to the sibling at the argument's sort. Idris 2 has no generic-deriving
facility for this, so the six definitions are mechanical necessity rather than a
phrase costume — and that is precisely why they are a **generation** candidate
and not a macro one.

The structure-vs-phrase sweep that convicted eight other families ruled this one
genuinely distinct by Idris mechanics **but generatable, and recorded that as the
finding**: it is one predicate lifted over a mutually-recursive family, not a job
re-minted per container. Take that verdict as given; the costume lens should not
fire here.

- The named family: the six-function scan over the indexed datatypes, plus the
  roughly four more scans of the same shape beside it (the
  deep negation scan, the says/delta readers, and the zone scan). The multiplier
  is about four families × about six definitions, and the fix is **one
  generator**.
- The nine list-scanning witnesses over predicate lists, which the sweep flags as
  "likely a generation family of family 14's kind, not a costume" — confirm that
  before folding them in, and fold them in if it holds.
- Check the arity/flatness witnesses re-minted per layer **once**: their own
  docstring says strict positivity forbids a shared arity witness. That is a real
  Idris constraint that should be checked at one site rather than assumed at
  each; if it holds, record it once and stop re-litigating it per layer.

## Fill the `OneOrMore` arm in the Idris emit path — NOT a grammar round

Not a grammar round — a Rust-side gap of exactly one match arm. In
`crates/deckmaste_plugin/src/idris_emit.rs`, `emit_event_filter` currently
answers `EventFilter::OneOrMore(_) => return Err(gap(…))`, so every filter of
that shape refuses at emit time regardless of what the grammar can express.

- Implement the arm. The mark that separates a counted group from a universal is
  `CountD`; a taker should read finding 534's reasoning on that distinction
  before writing the arm, because getting it wrong silently emits a universal
  where the card writes "one or more".
- Nothing about the Idris side changes; this is the emitter catching up.

## Consumption boundary

`crates/deckmaste_plugin/src/idris_emit.rs`, the generator and
`idris/scripts/build` / `idris/scripts/emit-tables`. No grammar file changes.

## Acceptance

- `cargo test -p deckmaste_plugin` with a regression case distinguishing
  counted-group from universal; `idris/scripts/build` PASS; the generated
  scan definitions byte-identical to the hand-written ones on first
  generation (empty diff).

Standard constraints apply.
