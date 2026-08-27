---
needs: []
---
# A dead-constructor audit for the Idris workbench

`cargo xtask map idris` (`crates/xtask/src/map.rs`) already inventories every
`data` declaration in an Idris source file — but only as a listing. It never
answers "which of these constructors does no bench card exercise?", and
nothing else in the repo does either: `idris/scripts/build` is a three-line
typecheck-only wrapper with no coverage reporting. That question is currently
unanswerable for any Idris module in the repo.

## The ask

Extend `crates/xtask/src/map.rs`'s `idris` subcommand (or add a sibling mode)
to cross-reference the constructor names `map idris` already finds in a
`data` declaration against occurrences in one or more given files — at
minimum `idris/src/Experimental/Cards.idr` (the evidence bench) and the pin
modules `idris/src/Experimental/Proofs*.idr` (a constructor a pin's *refusal*
exercises is exercised, not just one a bench witness's *success* exercises).
Report constructors named by neither as a plain list.

## What this tool is not, per standing rulings

- **Not a coverage percentage, and not a gate.** Per
  `docs/memory/rulings/measurements-live-in-pins.md`, "the workbench proves
  rules text is self-consistent... it is not a census of printed phrasings."
  This tool's output is a diagnostic list, never a ratio, never wired into
  `idris/scripts/build`'s exit code.
- **Not a pin generator.** An unexercised constructor is not evidence a pin
  is missing, and is not itself grounds to write one — a pin still needs a
  named CR rule making the *positive* term meaningless. An unexercised
  constructor is equally likely to mean "no round has needed it yet," which
  is a normal, un-alarming state for a workbench that (per the same ruling)
  grows by proof, not by corpus sampling.
- **Not a rendering or fidelity check.** Per
  `docs/memory/rulings/rendering-is-not-the-workbench.md`, this stays entirely
  inside constructor-name cross-referencing; it never touches printed Oracle
  text.

The Rust-side precedent for the shape (never the mechanism — no fixture
concept exists on the Idris side) is
`crates/deckmaste_plugin/tests/no_dead_grammar.rs`: every grammar constructor
is either exercised by a fixture or carries a named `DEFERRED` allowlist
entry with a reason. This ticket's tool answers the first half of that
question for Idris; it does not need to build the second half (an allowlist)
to be useful — a first run's findings can be triaged by hand.

## Consumption boundary

`crates/xtask/src/map.rs` (the tool). Reads `idris/src/Experimental/*.idr`
(at minimum `Cards.idr` and the `Proofs*.idr` modules) as plain text, the same
way `map idris` already does; writes no Idris source. No engine crate.

## Acceptance

- The tool reports, for a given Idris source file's `data` declarations,
  every constructor with zero occurrences across the given bench/pin files.
- Output is read-only and diagnostic: no build, gate, or CI step fails on its
  own from this tool's output.
- A first run against `idris/src/Experimental/Words.idr` (or another
  constructor-rich module the conductor picks) is triaged at least once —
  not every finding need be closed, but the findings are recorded here or
  routed to whichever ticket already owns each unexercised arm, so the first
  run demonstrates the tool is actionable rather than merely novel.

Standard constraints apply.
