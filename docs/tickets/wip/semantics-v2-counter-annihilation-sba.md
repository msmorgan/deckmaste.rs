---
needs: [semantics-v2-definition-bodies]
---
**The +1/+1 and -1/-1 annihilation rule has no home.** STOP recorded by
`semantics-v2-definition-bodies` while converting `counter_kinds/m1m1Counter.ron`
to a `Definition` body. [CR#704.5q]: "If a permanent has both a +1/+1 counter and
a -1/-1 counter on it, N +1/+1 and N -1/-1 counters are removed from it, where N
is the smaller of the number of +1/+1 and -1/-1 counters on it."

Two things block it, and the second is the interesting one.

1. **`Amount` cannot say "the smaller of".** `ArithOp` is
   `plus | minus | times | differenceBetween`; there is no minimum over two
   amounts, and `AggregateOp` (`sum | min | max`) ranges over a group or a
   domain, not over a pair of amounts. The v1 record wrote
   `Min(CounterCount(This, p1p1Counter), CounterCount(This, m1m1Counter))`.
2. **It is not a conferral.** A `Conferral` scopes the definition itself — the
   counter's bearer — but [CR#704.5q] fires on a permanent that has BOTH kinds,
   which is a predicate over the permanent. Its home is a `SbaRule` in
   `plugins_v2/builtin/rules/sba/`, beside the three the workbench already
   writes, not the -1/-1 counter's `confers` list where v1 put it.

So: decide whether `Amount` grows a binary minimum (and what its Lean name and
CR citation are — [CR#704.5q] is the only rule that needs one today), then write
the row as an `SbaRule` with `scope := .and [+1/+1 counters, -1/-1 counters]`
and a pin in `Proofs/Rules.lean` beside `toughnessZero`. The v1 record is
preserved verbatim in `m1m1Counter.ron`'s STOP comment until then.

`Proofs/Rules.lean` already omits the two lethal-damage rows [CR#704.5g,704.5h]
for the same class of reason; a resume should consider whether one amount
addition unblocks more than one row. Standard constraints apply.

## Landing record

1. **Census delta** (at `oksqkoll`): no grammar, lexicon or construction
   source changed; the counter declarations' `spelling` and compound fields
   are untouched, so no English census was run. Lost faces: none.
2. **Witnesses**: `m1m1Counter`'s definition carries the annihilation rule,
   naming its own kind bare, and proves in `lean definitions` (218/218
   counters, 711/711 in all); Misfortune (canon, new, both counter kinds)
   proves `Card.check = []`; Snowblind's "whichever is smaller" is pinned
   (`Proofs/Counters.okSnowblindWhicheverIsSmaller`). English Readings: not
   measured (no grammar change).
3. **Spot checks**: no newly covered English faces. Semantic gain: the
   -1/-1 counter's conferrals are `boost(down(1), down(1))` and
   `stateBased(matches(This, and([hasCounters(p1p1Counter),
   hasCounters(m1m1Counter)])), simultaneously([removeCounters(exactlyOf(
   smallerOf(countersOn(p1p1Counter, This), countersOn(m1m1Counter, This))),
   p1p1Counter, This), removeCounters(…, m1m1Counter, This)]))`; the
   declared name `m1m1Counter` is `CounterKind.named "m1m1Counter"` in
   `Declared.lean` (pin `m1m1CounterNameIsItsKind`).
4. **Deviations and STOPs**: the ticket's SbaRule-in-`rules/sba/` shape is
   superseded by the owner's ruling of 2026-10-10 (recorded in §7 first),
   which puts the rule on `m1m1Counter`'s definition as a state-based
   conferral; the ticket's STOP is cleared. "Both kinds as one event" is
   `simultaneously` over two `removeCounters` (accepted by the owner). STOP
   raised and resolved: a declaration could not name its own kind in its
   body, because its name was a macro whose body was the whole definition
   record (read back at term positions by `denoted_by`); the expander
   overflowed the stack on it. Resolved by two owner rulings: (a) the
   expander reports a nested self-reference as a cycle (`macro_ron`
   provenance chains, per-thread so a validator's nested read continues the
   chain), and (b) Q80 option A, recorded in §7: a counter, subtype,
   designation or relation declaration registers its name as its term and
   the registry holds the definition record apart
   (`reader::Declaration::{definition, registered}`); the four `denoted_by`
   markers and the Rust and Lean `Definition.*Term` projections are retired.
   `Declared.lean` grows by the 700 term macros (one per counter, subtype,
   designation and relation; a term that shares a name with another macro
   is qualified by its type, and `seal` joins the escaped Lean keywords).
   `lean macros align` reports one new drift, `Actor.army`, a hand macro now
   name-matched against the declared subtype term `army`; not a real drift.
   The corpus ratchet `no_source_file_writes_out_an_elided_constructor` is
   back to its original assertion. Companion reworded as deferred, not out
   of scope, in the 2026-10-09 routing ruling (owner). Glossary: Registry
   Definition entry amended (per-holder state-based conferrals; the name
   means its term, the record is held apart).
5. **Tests** (at `qmxzrwqr`, after the refresh onto the subtype-rules, protection and
   devour-quality landings; generated files regenerated, unchanged): `cargo test -p macro_ron` ok (149);
   `cargo test -p deckmaste_semantics_v2` ok (48 + 7 + 1 + 4 + 46);
   `cargo test -p xtask` ok (407 + 4 + 14, 1 pre-existing ignore);
   `cargo test -p deckmaste_construction_core` ok (437 + 37 across
   integration binaries); `cargo clippy -p <crate> --tests -- -D warnings`
   clean for all four; `lake build --wfail` ok; `cargo xtask lean check`
   245/245 cards (237 canon, 8 testing); `cargo xtask lean definitions`
   711/711; `lean macros check` and `facts check` up to date (facts rows
   unchanged); `cite check` 0 stale, 0 noncompliant. Every other crate
   that reads through `macro_ron` was run against the expander change:
   `cargo test -p` `deckmaste_plugin`, `deckmaste_semantics`,
   `deckmaste_english_v3`, `deckmaste_lexical_source`, `deckmaste_spelling`,
   `deckmaste_card`, `deckmaste_lowering`, `deckmaste_legacy_render` and
   `macro_ron_lsp`, all ok. Counts: restored 1
   (the corpus ratchet, narrowed and restored within this landing);
   re-spelled 0; ignored 0; added 7 Lean pins (`okAnnihilationCount`,
   `okAnnihilation`, `okM1m1CounterDefinition`,
   `badSmallerOfUnscopedThatMuch`, `okSnowblindWhicheverIsSmaller`,
   `okGreaterOf`, `m1m1CounterNameIsItsKind`), 3 alias reads in
   `every_ported_alias_expands`, and 3 `macro_ron` tests
   (`nested_self_reference_is_a_reported_error_not_an_overflow`,
   `nested_mutual_reference_is_a_reported_error_not_an_overflow`,
   `nesting_in_arguments_is_not_a_cycle`); removed 0.
6. **Timing**: no census; `lean check` wall 523 s (one lake job pool).
