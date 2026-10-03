---
needs: []
---
**A Counter's kind is a name; what it does is its Registry Definition.**
Ruling (2026-10-04): `CounterKind`'s three shapes — `Boost(power, toughness)`,
`Keyword(keyword)`, `Named(label)` — collapse to one, and every counter is
name, holder and Conferrals. Design-bearing (Lean first); standard constraints
apply.

## Why

The shapes let the checker validate a kind without a declaration: any +X/+Y is
well-formed by shape [CR#122.1a], a keyword counter is checked against the
keyword-counter list [CR#122.1b], and only `Named` consults the facts table.
Since `semantics-v2-definition-bodies` every counter declaration carries a
`Counter(kind, holder, confers)` definition, so the shape repeats what
`confers` already says: `p1p1Counter` writes `Boost(Up 1, Up 1)` and then the
power and toughness `Modification`s; `flyingCounter` writes `Keyword("Flying")`
and then the `AbilityGrant`. The `kind` field is doing the work of a name.

## Decisions already made

- A counter declaration's name reads bare at a counter position and denotes
  its kind (`denoted_by` on `CounterKind`, `semantics-v2.md` §12), and
  `CounterKindSource::Printed` is an injection (§11.1). Both rulings
  2026-10-03. Card and macro bodies therefore need not change when the kind's
  representation does.
- The Lean workbench is a work in progress, not a finished spec (2026-10-03):
  `CounterKind` changes in Lean first and the Rust mirror follows.
- The +1/+1 and -1/-1 annihilation rule [CR#704.5q] is not derived from the
  kind's shape and stays with `semantics-v2-counter-annihilation-sba`.

## The change

1. **Lean.** `CounterKind` (`Words.lean`) becomes a name. `CounterKind.check`
   (`Check/Keywords.lean`) becomes one registry lookup, and the holder read
   (`Check/Words.lean`) takes every holder from `CounterFacts` instead of
   assuming an Object for boost and keyword kinds. Re-spell the pins in
   `Proofs/Counters.lean` and `Proofs/Rules.lean` against the new shape — same
   subjects, same outcomes.
2. **Facts.** `cargo xtask facts generate` emits a `CounterFacts` row for
   every counter declaration, not only the `Named` ones.
3. **Mirror.** `deckmaste_semantics_v2::words::CounterKind` and `reads.rs`
   follow Lean; `lean_drift` holds.
4. **Declarations.** The 6 boost and 15 keyword declarations under
   `plugins_v2/builtin/macros/counter_kinds/` take a name like the 56 named
   ones. A +X/+Y kind with no declaration gets one when a card first needs it;
   only ±1/±1 are written anywhere in `plugins_v2/` today.
5. **Bodies.** Replace the written-out `Boost(…)` kinds (26 files outside the
   counter declarations) with the declared name, and add `Printed(kind:` to
   `no_source_file_writes_out_an_elided_constructor` once none remain. Retire
   the `plusOnePlusOne` / `minusOneMinusOne` helpers, which duplicate
   `p1p1Counter` / `m1m1Counter`.

## Open

- **The name's type.** `denoted_by` attaches to a `SupportsMacros` enum, so a
  bare string alias cannot carry it. Either `CounterKind` stays a
  one-constructor type, or the marker learns a label position. Designations
  (`DesignationLabel`) need the same answer; decide once.
- **Self-reference.** A boost counter's own `confers` reads its count through
  `StatOf(axis: Counter(kind: …))`. Decide whether a definition names itself
  there or the axis gains a "this counter" form.
- **What the name is.** The declaration's `spelling` ("+1/+1", "flying",
  "charge") or its macro name. `Named(label: "Charge")` uses a third,
  capitalised form today.
