---
needs: []
---
# Read *up to that many*: Return up to that many target permanents that player controls to their owner's hand.

## Why

*that many* + noun reads, *up to* + cardinal + noun reads, their composition
does not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): key `at:to that` 13 sole-blocker
faces. `cargo xtask english-v3 grep 'up to that many'` finds **20** faces,
all unread; 20 failing lines match, **17** of them the only failing line of
their face.

## Goal

*up to* bounds the degree Determinative phrase *that many* exactly as it
bounds a cardinal, in every NP position where *up to two* reads (Object,
Subject, *for* Complement, before *target*).

## Analysis

CGEL Ch. 5 §11(c), p. 431: the degree determinatives *many*, *much*, *few*
and *little* "take a very similar range of degree modifiers to gradable
adjectives, including *very*, *so*, *too*, *how*, *this*, *that*". That backs
*that many* as a determinative phrase (`DegreeDeterminativePhrase`); this
ticket relies on no further passage. Letting *up to* take it is a project ruling
(landed for cardinals in `english-v3-scalar-cardinals`, for *that many* in
`english-v3-that-degree-quantifiers`).

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`DegreeDeterminativePhrase: QuantitativeDeterminer` (`form [modifier:
lexical(Determinative), " ", head: lexical(Determinative)]; require
modifier.DegreeUse = Modifier; require head.DegreeUse = Head`) builds *that
many*; the *up to* bound from `english-v3-scalar-cardinals` composes with a
cardinal quantity, not with a `QuantitativeDeterminer`.

Orchestrator rulings 2026-10-10:

- UM1. Refine the *up to* bound so its operand may be the
  `QuantitativeDeterminer` built by `DegreeDeterminativePhrase`; no new
  Construction and no multiword lexeme.
- UM2. The licence reads the operand's declared features (`DegreeUse`, its
  agreement number). No test on *that* or *many*. *up to that much* is
  unattested: no work, and no check to block it.
- UM3. One analysis per string. *up to that many target creatures* is one
  bounded Determiner over the Nominal. It must not also read with *up to* as
  a Preposition taking the NP *that many target creatures* (a goal or limit
  PP), nor with *that* as a demonstrative Determiner of *many target
  creatures*.
- UM4. Newly covered faces are gains. Reading-count increases are possible on
  covered faces with *up to* + cardinal or with *that many*; none is
  expected, and each is reported with its shape.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Cephalid Constable: "Whenever this creature deals combat damage to a
  player, return up to that many target permanents that player controls to
  their owner's hand." (main clause probed: 0)
- Heroic Feast (first sentence): "Whenever you gain life, choose up to that
  many target creatures you control." ("Choose up to that many target
  creatures you control." 0)
- Swashbuckler Extraordinaire (second sentence): "When you do, up to that
  many target creatures gain double strike until end of turn." (main clause
  0)
- Scapeshift (second sentence): "Search your library for up to that many
  land cards, put them onto the battlefield tapped, then shuffle." ("Search
  your library for up to that many land cards." 0)

Controls that must not change: "Return that many target permanents that
player controls to their owner's hand." 2; "Return up to two target
permanents that player controls to their owner's hand." 2; "Search your
library for that many land cards." 3; "Search your library for up to two land
cards." 3; "That many target creatures gain double strike until end of turn."
1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bup to\b` or `\bthat (many|much)\b` case-insensitively
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
UM4 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (*up to* bounding the degree Determinative phrase, that phrase as
Determiner of the Nominal, the UM3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *distribute up to that many +1/+1 counters among …* (Lathiel) and *pay any
  amount of {E} … choose up to that many* where another construction fails.
- *as many … as* comparatives (*draws as many cards as they discarded this
  way*, Forget).
- *up to X*, *up to one* as a modal header count
  (`english-v3-modal-header-quantities`).

## Landing record

1. **Census delta** (`xzwlutqp`; BEFORE is the untouched claim-parent tree
   `vrmzkzqv`, measured on its empty child `xzwlutqp`): bounded selector
   `\bup to\b|\bthat (many|much)\b`, case-insensitive, reaches 1,960 faces;
   300 covered controls from the supplied census, seed 20261008, give 2,240
   distinct faces. Before refresh: covered 1,884 → 1,899; Readings 107,418
   → 107,999; 15 gains. After refresh, measured on empty child `ssypzrxt`
   of refreshed `xzwlutqp`: covered 1,884 → 1,903; Readings 107,418 →
   108,150; 19 gains. Official `diff --lines` in both comparisons: lost
   none; decreases 0; increases on covered faces 0; digest-only 0;
   validation issues 0. The four refresh gains belong to `wxxvsntt`
   (starting-with adjunct): Collective Voyage, Disciple of Caelus Nin,
   The Horus Heresy and The Legend of Yangchen. The original 15 gains
   retain their counts and digests. Evidence under
   `target/english-v3/english-v3-up-to-that-many/`: `before.json`,
   `after.json`, `diff.json`, `after-refresh.json`, `diff-refresh.json`
   and `diff-trunk.json`. Integration refresh on empty `oqqxmsox`
   brings `lvsykyns` (degree-quantity arithmetic): 17 further gains,
   giving 36 gains against the claim parent; covered 1,920 and Readings
   108,300. Lost, decreases and covered-face increases remain 0. Its new
   optional arithmetic field accounts for 414 digest-only changes against
   the claim parent (429 against the reviewed census), with no count
   changes on covered faces. Evidence: `after-integration-refresh.json`,
   `diff-integration-refresh.json` and `diff-integration-incoming.json`
   in the same directory. Integration retry refresh on empty `ypzpynyq`
   imports `svlsnsmv` (property-pair each-equal-to); all 2,240 faces
   remain identical to the preceding census, including digests. The
   totals at that refresh remain 36 gains, 1,920 covered, 108,300 Readings, and
   no losses, decreases or covered-face increases. Evidence:
   `after-integration-refresh-2.json`, `diff-integration-refresh-2.json`
   and `diff-integration-incoming-2.json`. The next retry refresh on
   empty `rrustyrq` imports `mwtywluv` (facts deed patient roles;
   plugin/Lean changes); all 2,240 faces again remain identical, including
   digests. Evidence: `after-integration-refresh-3.json`,
   `diff-integration-refresh-3.json` and `diff-integration-incoming-3.json`.
   The next retry refresh on empty `uoryukwl` imports `xswzoyrx`
   (predeterminer all): 15 further gains; covered 1,935; Readings 108,499;
   cumulative gains 51. Lost and decreases remain 0; digest-only remains
   414, attributable to `lvsykyns`. One covered-face increase belongs to
   `xswzoyrx`: Phyrexian Scriptures 5 → 6, its affected line 1 → 2.
   Predeterminer ticket ruling PA7 expressly preserves both
   `[all [opponents' graveyards]]` and `[[all opponents]' graveyards]`.
   Evidence: `after-integration-refresh-4.json`,
   `diff-integration-refresh-4.json` and `diff-integration-incoming-4.json`.
   The direct sentence probe confirms both genitive attachments, complete
   enumeration and byte-exact roundtrips, without issues:
   `pa7-phyrexian-as-card.json` (4 Sentence Readings) and
   `pa7-phyrexian-control.json` (2 without the second *all*); each
   genitive attachment retains the control's independent PP attachment
   ambiguity (PA7, W9-A).
2. **Witnesses** (`xzwlutqp`): all four witnesses failed with 0 Readings
   before implementation. Main clauses after: Cephalid Constable 2, Heroic
   Feast 2, Swashbuckler Extraordinaire 1, Scapeshift 3; corresponding
   unbounded controls 2, 2, 1, 3. Complete ticket sentences after: 6, 2, 1,
   12; their unbounded controls also 6, 2, 1, 12 (W9-A). The five specified
   controls remain 2, 2, 3, 3, 1; every count is completely enumerated.
3. **Spot checks** (`xzwlutqp`): all 15 gains inspected, with the nested
   quantitative PP bounding the degree Determinative Phrase, then functioning
   as Determiner over the Nominal. Ten: Cephalid Constable (return Object),
   Heroic Feast (choose Object), Swashbuckler Extraordinaire (Subject),
   Scapeshift (selected *for* Complement), Loamcrafter Faun (return Object),
   Hexhaven Invigorator, Rites of Spring and Disorienting Choice (selected
   *for* Complements), Nyssa of Traken (tap Object), Miasma Demon (Subject).
   No unintended-analysis gains found.
4. **Deviations and STOPs** (`xzwlutqp`): no new or retired named
   Construction, Verb Frame or lexeme; no glossary gap or STOP. Two Category
   instances specialize the existing quantitative-PP schema because its
   compiler requires unique result Categories. Degree Use is projected through
   the operand; other determiners export None, preserving cardinal structure
   and excluding recursively bounded operands. Five tests added for roles,
   independent roundtrips/traversal, UM3 negatives, agreement and controls.
   Lathiel and Suppression Ray are correct incidental gains: their other
   constructions already compose on this claim parent. The binding user
   budget supplies seed 20261008 and bounded verification. Integration
   refresh adds the optional `arithmetic_modifier` field; our independent
   expected-tree helper now sets it to None, preserving every assertion.
   Predeterminer-all refresh conflicts were resolved by preserving its
   tables alongside this ticket's Degree Use default, and registering both
   test modules exactly once. No assertion was weakened.
5. **Tests** (`xzwlutqp`): before refresh, `cargo test -p
   deckmaste_english_v3` passed 1,033 tests and its clippy check passed.
   After refresh, `cargo build -p xtask` passed. Full `cargo test -p
   <crate>` and `cargo clippy -p <crate> --tests -- -D warnings` passed
   for each of `deckmaste_english_v3` (1,043 tests), `xtask`,
   `deckmaste_semantics_v2`, `deckmaste_construction_core` and
   `deckmaste_lexical_source`; no failures. English has no ignored tests;
   xtask retains its existing ignored
   `macros::templates::tests::macro_schema_census_count_matches_21`
   (live-corpus census cross-check, run on demand). These extra suites
   cover incoming crate, lexicon and plugin changes. `cargo xtask lean
   check` passed: all 243 cards prove `Card.check = []`. Results and
   exact commands: `target/english-v3/english-v3-up-to-that-many/refresh-checks.json`.
   Citation checks: noncompliant 0, stale 0; diff audit selected 0 new
   citation sites. Restored 0 / re-spelled 0 / newly ignored 0 / added 5 /
   removed 0. Own code
   changes are confined to `deckmaste_english_v3`. Integration refresh:
   `cargo build -p xtask` passed. The initial full English test build found
   our helper missing the incoming optional arithmetic field; setting it
   to None preserved all assertions. Full `cargo test -p
   deckmaste_english_v3` then passed 1,048 tests, no failures or ignored
   tests. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed after the helper fix. Evidence: `integration-build.log`,
   `integration-clippy-fixed.log`, `integration-test.log`
   (compile failure) and `integration-test-fixed.log` in the evidence
   directory. On the integration retry refresh, `cargo build -p xtask`
   and full `cargo test -p deckmaste_english_v3` passed (1,053 tests,
   no failures or ignored tests). `cargo clippy -p deckmaste_english_v3
   --tests -- -D warnings` passed; commands and logs are retained in
   `integration-checks-2.json` and `integration-2-*.log`. After the plugin
   refresh, `cargo build -p xtask` passed; full `cargo test -p <crate>`
   and `cargo clippy -p <crate> --tests -- -D warnings` passed for
   `deckmaste_english_v3`, `xtask`, `deckmaste_semantics_v2` and
   `deckmaste_construction_core`. `cargo xtask lean check` passed for
   all 244 cards. Exact commands and logs: `integration-checks-3.json`
   and `integration-3-*.log`. After resolving the predeterminer-all
   refresh conflicts, `cargo build -p xtask` passed; full `cargo test -p
   <crate>` and `cargo clippy -p <crate> --tests -- -D warnings` passed
   for `deckmaste_english_v3` (1,062 tests, no failures or ignored tests)
   and `deckmaste_lexical_source`. Exact commands and logs:
   `integration-checks-4.json` and `integration-4-*.log`.
6. **Timing** (`xzwlutqp`): bounded after census under `/usr/bin/time -v`,
   184 seconds 420 milliseconds wall; refreshed census on empty
   `ssypzrxt`, 229 seconds 450 milliseconds wall. Both used 6 workers, samples per face 0 and the same
   2,240-face identity manifest. The integration-refresh bounded census
   took 88 seconds 650 milliseconds wall; the integration retry census
   took 84 seconds 270 milliseconds wall; the plugin-refresh census
   took 70 seconds 960 milliseconds wall; the predeterminer-refresh
   census took 79 seconds 100 milliseconds wall, with the same settings. No full
   corpus run, as required by the binding verification budget.
