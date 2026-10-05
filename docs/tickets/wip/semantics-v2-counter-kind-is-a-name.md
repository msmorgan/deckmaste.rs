---
needs: []
---
**A Counter's kind is a name; what it does is its Registry Definition, stated
for one counter.** Ruling (2026-10-04): `CounterKind`'s three shapes —
`Boost(power, toughness)`, `Keyword(keyword)`, `Named(label)` — collapse to
one, and every counter is name, holder and Conferrals. Its open questions were
decided with the owner on 2026-10-05 (below). Design-bearing (Lean first);
standard constraints apply. Designations get the same treatment in a separate
landing, `semantics-v2-designation-is-a-name`, which follows this one.

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

### Decided 2026-10-05 (the former "Open" questions)

- **What the name is: the declaration name, verbatim** (`p1p1Counter`,
  `chargeCounter`). Not the `spelling`, and not the capitalised third form
  `Named(label: "Charge")` writes today.
- **`spelling` stays the compound's STEM** ("+1/+1", "charge"). Every counter
  declaration already yields one compound-noun lexeme "<stem> counter"
  (`macros/meta/CounterKind.ron` `compound_noun`;
  `crates/deckmaste_lexical_source/src/compound.rs`), and
  `crates/deckmaste_construction_core/tests/builtin_v2_counter_kinds.rs`
  forbids a spelling ending in " counter". The owner first proposed
  "+1/+1 counter" as the spelling and withdrew it on this evidence.
- **Self-reference: a definition states what ONE counter confers, and the model
  applies it once per counter held.** Nothing reads its own count and nothing
  refers to itself: `counter(Object, [boost(up(1), up(1))])`. Today
  `p1p1Counter`'s conferral reads its count through `StatOf(axis:
  Counter(kind: Boost(…)), subject: This)`; that goes. Considered and
  rejected: a "this kind of counter" form, and a "this macro" expander hole
  (the owner floated it; it would be a new expander feature,
  `docs/decisions/macros-are-declarative.md`). The name of the helper that
  writes a counter definition is to be **proposed to the owner** before the
  landing; `counter(…)` above is a placeholder.
- **The name's type** (orchestrator's call; the owner did not object): counter
  kinds become a small proper type rather than a `String` alias, so the
  existing `denoted_by` marker (enum-only today) attaches to it. Designations
  get the same answer in `semantics-v2-designation-is-a-name`; the owner wants
  them read bare too ("I'd prefer to avoid stringly typed so yes").

## The change

1. **Lean.** `CounterKind` (`lean/Semantics/Words.lean` ~L523) becomes a name.
   `CounterKind.check` (`Check/PhraseRules.lean` ~L31) becomes one registry
   lookup; today it delegates to `CounterKind.known` (`Check/Keywords.lean`
   ~L84). The holder read takes every holder from `CounterFacts`
   (`Check/Words.lean`, `counterFactsFor`) instead of assuming an Object for
   boost and keyword kinds. A Counter definition's conferrals are per counter,
   and the model multiplies them by the count held. Re-spell the pins in
   `Proofs/Counters.lean` and `Proofs/Rules.lean` (and the other proofs that
   name a kind) against the new shape — same subjects, same outcomes. The
   names `plusOnePlusOne` / `minusOneMinusOne` remain in 15 Lean files;
   re-spell them to the declared names.
2. **Facts.** `cargo xtask facts generate` emits a `CounterFacts` row for every
   counter declaration: `counter_rows` in `crates/xtask/src/facts/lean.rs`
   emits rows for `Named` kinds only today.
3. **Mirror.** `deckmaste_semantics_v2::words::CounterKind` and `reads.rs`
   follow Lean; `lean_drift` holds.
4. **Declarations.** `plugins_v2/builtin/macros/counter_kinds/` has 73 files:
   2 `Boost` (`p1p1Counter`, `m1m1Counter`), 15 `Keyword`, 56 `Named`. Each
   takes its name and states its per-counter conferral. A +X/+Y kind with no
   declaration gets one when a card first needs it; only ±1/±1 are written in
   `plugins_v2/` today. If `semantics-v2-gets-both-deltas` has landed, the
   boost conferral uses its one-modification form.
5. **Bodies.** One file outside the declarations writes `Boost(` (canon
   `Luminarch Aspirant.ron`); re-spell it to `p1p1Counter`. 26 files outside
   write `p1p1Counter`/`m1m1Counter` already and should not change. Then add
   `Printed(kind:` to `no_source_file_writes_out_an_elided_constructor`
   (`crates/deckmaste_semantics_v2/tests/corpus.rs`). (The RON
   `plusOnePlusOne`/`minusOneMinusOne` helpers no longer exist.)

## Proof

- `cargo xtask lean-check` passes with every pin at its old outcome.
- `cargo xtask expansions` before/after: every counter declaration's term
  changes (listed); every card and keyword body is identical except Luminarch
  Aspirant, whose kind now prints as the declared name.
- The English side still reads every counter compound (the stems are
  untouched).

## Out of scope

- Designations (`semantics-v2-designation-is-a-name`).
- Counter annihilation (`semantics-v2-counter-annihilation-sba`).
- Shared-head coordination of counter kinds in English
  (`english-v3-counter-kind-shared-head`).
