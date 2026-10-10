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

1. **Census delta** (at `vptuqotw`): no grammar, lexicon or construction
   source changed; the counter declarations' `spelling` and compound fields
   are untouched, so no English census was run. Lost faces: none.
2. **Witnesses**: `m1m1Counter`'s definition now carries the annihilation
   rule and proves in `lean definitions` (218/218 counters, 711/711 in
   all); Misfortune (canon, new, both counter kinds) proves
   `Card.check = []`; Snowblind's "whichever is smaller" is pinned
   (`Proofs/Counters.okSnowblindWhicheverIsSmaller`). English Readings: not
   measured (no grammar change).
3. **Spot checks**: no newly covered English faces. Semantic gain: the
   -1/-1 counter's conferrals are `boost(down(1), down(1))` and
   `stateBased(matches(This, and([hasCounters(p1p1Counter),
   hasCounters(m1m1)])), simultaneously([removeCounters(exactlyOf(
   smallerOf(countersOn(p1p1Counter, This), countersOn(m1m1, This))),
   p1p1Counter, This), removeCounters(…, m1m1, This)]))`.
4. **Deviations and STOPs**: the ticket's SbaRule-in-`rules/sba/` shape is
   superseded by the owner's ruling of 2026-10-10 (recorded in §7 first),
   which puts the rule on `m1m1Counter`'s definition as a state-based
   conferral; the ticket's STOP is cleared. "Both kinds as one event" is
   written as `simultaneously` over two `removeCounters` (one action of the
   check [CR#704.3]), not as a new counter-kind-list node. A declaration
   cannot read its own bare name in its body (the name is a macro whose
   body is that definition; it overflowed the expander's stack, which
   rejects declared cycles but not this one), so `m1m1Counter.ron` writes
   its own kind as `Named(name: "m1m1Counter")`, and the corpus ratchet
   `no_source_file_writes_out_an_elided_constructor` now permits a file to
   write out its own declared name, and nothing else. Helpers named
   `smallerOf`/`greaterOf` per the owner. Glossary: the Registry Definition
   entry in `docs/contexts/game-model/CONTEXT.md` amended for per-holder
   state-based conferrals.
5. **Tests** (at `vptuqotw`, after refresh): `cargo test -p
   deckmaste_semantics_v2` ok (47 + 7 + 1 + 4 + 46); `cargo test -p xtask`
   ok (407 + 4 + 14, 1 pre-existing ignore); `cargo test -p
   deckmaste_construction_core` ok (437 + 37 across integration binaries);
   `cargo clippy -p <crate> --tests -- -D warnings` clean for all three;
   `lake build --wfail` ok; `cargo xtask lean check` 241/241 cards (233
   canon, 8 testing); `cargo xtask lean definitions` 711/711; `lean macros
   check` and `facts check` up to date (facts rows unchanged); `lean macros
   align`: the new macros align, its 48 drifts are pre-existing; `cite
   check` 0 stale, 0 noncompliant, no new rules blessed. Counts: restored
   0; re-spelled 0; ignored 0; added 6 Lean pins (`okAnnihilationCount`,
   `okAnnihilation`, `okM1m1CounterDefinition`,
   `badSmallerOfUnscopedThatMuch`, `okSnowblindWhicheverIsSmaller`,
   `okGreaterOf`) and 3 alias reads in `every_ported_alias_expands`;
   removed 0.
6. **Timing**: no census; `lean check` 498.4 s wall (one lake job pool).
