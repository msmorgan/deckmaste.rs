---
needs: []
---
# Declare *waterbend* as a verb taking a cost: Waterbend {3}: This creature can't be blocked this turn.

A data ticket on one plugin definition. No `declarations.rs` edit is
expected.

## Why

*waterbend* is a keyword action whose Complement is a mana cost. Its owning
definition exists (`plugins_v2/builtin/macros/keyword_actions/waterbend.ron`,
`params: [Cost]`, `spelling: "waterbend <Param(0)>"`) but exports
`grammar: FixedTerm(surface: "waterbend")`, so no verb reads. Blocker report
at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): key `at:<start>
waterbend` 14, failing token `waterbend` 27.
`cargo xtask english-v3 grep -i '\bwaterbend'` finds **29** faces, all
unread, **29** with that line as their only failing line. By position:
cost of an activated ability *Waterbend {N}: …* 16 (2 after *Exhaust —*);
*As an additional cost to cast this spell, (you may) waterbend {N}.* 9;
*unless you waterbend {2}* 1; *Ward—Waterbend {4}.* 1; bare *Whenever you
waterbend, earthbend, firebend, or airbend* 1; gerund-participle *by
waterbending {X} rather than paying its mana cost* 1 (Hama, the
Bloodbender).

The sibling actions already export verbs: `earthbend.ron` (`Verb(bare:
"earthbend", frame_set: MeasureComplement)`, 35 covered lines) and
`airbend.ron` (`Verb(bare: "airbend", frame_set: Transitive)`).

## Goal

*waterbend* reads as a verb with a cost-symbol Complement wherever *pay
{N}* reads today: as a Cost component, as the imperative or *you may*
Predicate of an additional-cost sentence, under *unless you*, and after a
keyword dash.

## Analysis

No CGEL claim: a game term's verb export and its Complement class are
ownership and declaration (owner ruling V3 of
`docs/tickets/done/english-v3-vocabulary-batch-1.md`: game terms are declared
through the owning plugin definition's `grammar:` field, never in core).

Nearest existing declarations: `core-verb:Pay`
(`crates/deckmaste_lexical_source/lexicon/verbs.ron`, frames
`Predicate([ObjectNounPhrase])` and `Predicate([Role("CostSymbols")])`), the
route that reads "Pay {3}: Draw a card." 1; plugin `Verb` rows with a
`Custom` frame set that names a Role (`keyword_actions/surveil.ron`,
`[Role("MeasurePhrase")]`; `keyword_actions/scry.ron`, `[[], [Amount],
[ObjectNounPhrase]]`). `ObjectNounPhrase` is not the Complement: "Explore
{3}: Draw a card." 0.

Orchestrator rulings 2026-10-10:

- WB1. Change only `waterbend.ron`'s `grammar:` to a `Verb` row with `frame_set:
  Custom(frames: [[], [Role("CostSymbols")]])` (the bare frame for *whenever
  you waterbend*). `name`, `params`, `spelling` and `body` are untouched. No
  core lexeme, no entry in `lexicon/verbs.ron`.
- WB2. If the plugin loader does not accept `Role("CostSymbols")` in a plugin
  frame, the fix is in the loader's frame translation
  (`crates/deckmaste_lexical_source/src/legacy/plugins.rs`), reading the
  declared Role as it does for core verbs; record it under Deviations. A
  grammar Construction naming the verb is a STOP.
- WB3. One analysis per string. *Waterbend {3}: …* reads as an action Cost
  component (`ActionCost`) with the symbols as the verb's Complement; it must
  not also read as a `FixedTerm` noun followed by a `SymbolCost`. Retiring
  the `FixedTerm` export is intended: confirm no covered face used it
  (0 of the 29 faces are covered).
- WB4. Inflected forms: the plain form (28 faces) and the gerund-participle
  *waterbending* (1) are attested; *waterbends*, *waterbent* and
  *waterbended* are not (0 by grep). Declare the unattested forms
  `Unavailable` where the row allows it, as `keyword_actions/exert.ron` does.
- WB5. Newly covered faces are gains; no Reading-count change on any covered
  face is expected (the word occurs on none).

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Control probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data. Re-probe on the claim parent first.

- Giant Koi: "Waterbend {3}: This creature can't be blocked this turn."
  (control "Pay {3}: Draw a card." 1)
- Katara, Bending Prodigy: "Waterbend {6}: Draw a card."
- Water Whip: "As an additional cost to cast this spell, waterbend {5}."
  (control "As an additional cost to cast this spell, pay {5}." 1)
- Spirit Water Revival: "As an additional cost to cast this spell, you may
  waterbend {6}." (control "… you may pay {2}." 1)
- Waterbending Lesson: "Draw three cards. Then discard a card unless you
  waterbend {2}." (control "… unless you pay {2}." 1)
- The Unagi of Kyoshi Island: "Ward—Waterbend {4}." (control "Ward—Pay {4}."
  1)

Controls that must not change: Earth Village Ruffians "When this creature
dies, earthbend 2." 1 (census); Appa, Steadfast Guardian "When Appa enters,
airbend any number of other target nonland permanents you control." 5
(census); "Pay 2 life: Draw a card." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\bwaterbend|\bearthbend|\bairbend|\bfirebend|\bpay \{

(847 faces) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost, zero changed among covered faces. `cargo test
-p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, and `cargo xtask lean
check` (a plugin definition changes), once, at the end. Plugin `grammar:`
data loads at runtime, so WB1 needs no grammar rebuild. Tests assert the role
(the verb as head of a Cost component and of an additional-cost Predicate,
the symbols as its Complement), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Avatar Aang: "Whenever you waterbend, earthbend, firebend, or airbend, …"
  also needs a verb *firebend* (`keyword_abilities/firebending.ron` exports
  the keyword only) and four-way bare-verb coordination; the control
  "Whenever you scry, earthbend, or surveil, draw a card." probed 0.
- The semantics of the action (`body:`); semantics_v2 owns bodies.
- Other plugin-owned terms held with `english-v3-plugin-term-definitions`.

## Landing record

1. **Census delta** (`ulrnwvuw`, pre-refresh BEFORE on original claim-parent
   `ytpqnwnm`'s unchanged tree, grammar at `pnzmstsu`): bounded selector
   `(?i)\bwaterbend|\bearthbend|\bairbend|\bfirebend|\bpay \{`
   selects 847 faces; 300 random covered controls (seed 20261008) give 1,135
   distinct faces. Covered 1,022 → 1,049; total Readings 20,187 → 20,650.
   `cargo xtask english-v3 diff --lines`: 27 gained, 0 lost, 0 decreases,
   0 increases, 0 digest-only changes; both censuses have 0 issues. Lost
   faces: none. All 29
   waterbend faces were uncovered before retiring `FixedTerm`; no covered
   Reading used it. Unintended gains: none. Manifest, BEFORE, AFTER and
   diff: `target/english-v3/english-v3-waterbend-cost-verb/`.
   After Kata refresh (`znpnsztt`), the same 1,135-face manifest gives
   **1,022 → 1,051** covered and **20,187 → 20,662** Readings: 29 gains
   (27 feature + 2 trunk), 0 lost/decreases/increases, 4 digest-only, 0
   issues. All 27 waterbend gains retain their counts and digests. The
   exactly-cardinal landing `oyqrxmwz` contributes Jerren, Corrupted Bishop
   (8) and Pitfall Trap (4). The partitive landing `pusrqwml` contributes
   only the canonical `CardinalDeterminer` wrapper changes on Conspiracy
   Theorist (10), Hollow Specter (13), Marchesa, Dealer of Death (3), and
   Preferred Selection (9); their counts are unchanged. Attribution uses
   `refresh-diff.json`; the final claim-parent comparison is
   `diff-refreshed.json`, against `after-refreshed.json`.
2. **Witnesses** (`ulrnwvuw`, complete fragment probes): Giant Koi 0 → 5;
   Katara, Bending Prodigy 0 → 1; Water Whip 0 → 1; Spirit Water Revival
   0 → 1; Waterbending Lesson 0 → 1; The Unagi of Kyoshi Island 0 → 1.
   Hama's actual gerund-participial sentence 0 → 3. Giant Koi's otherwise
   identical pay control has 5; Hama's paying control has 3, preserving
   independent ambiguity under W9-A. The ticket's five pay controls and
   pay-life control stay at 1; Earth Village Ruffians stays at 1, scoped
   Appa at 5, bare coordination control at 0. No already-reading witness
   needed dropping.
3. **Spot checks** (`ulrnwvuw`): Giant Koi, Katara, Bending Prodigy, The
   Unagi of Kyoshi Island, Ruthless Waterbender, Flexible Waterbender and
   Geyser Leaper select `ActionCost` with waterbend as the verb head and
   `CostSymbols` as its Complement (Unagi inside `CostKeyword`). Water
   Whip selects an imperative additional-cost Predicate; Spirit Water
   Revival a modal additional-cost Predicate; Waterbending Lesson a
   finite Predicate under *unless*; Hama, the Bloodbender a
   gerund-participial Predicate in the *by* Means Adjunct. Each retains
   the selected cost-symbol Complement, with no twin `FixedTerm` route.
4. **Deviations and STOPs**: WB4 is superseded by the owner's 2026-10-10
   natural-class ruling: ordinary defaults declare *waterbends*,
   *waterbending*, and regular-derived *waterbended* for both past forms;
   grep attests neither past spelling nor *waterbends* (`ulrnwvuw`). Four
   structural/paradigm/roundtrip tests were added, including Hama and the
   attested bare frame. No Construction, loader, core lexeme or frame
   beyond WB1 was added. Avatar Aang's excluded coordination remains
   unread; Katara, Water Tribe's Hope's complete activated line and its
   pay counterpart both probe 0 (`ulrnwvuw`), an independent residual.
   STOPs: none. Glossary gaps: none.
5. **Tests** (`ulrnwvuw`): `cargo test -p deckmaste_english_v3` passed
   (983 tests); `cargo test -p deckmaste_lexical_source` passed (86 tests);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed;
   `cargo test -p xtask english_v3` passed (56 tests);
   `cargo xtask lean check` passed (242 cards).
   `cargo xtask cite check --list-noncompliant` found 0;
   `cargo xtask cite check` found 0 stale. Diff citation audit selected 0
   new citation sites. Restored 0 / re-spelled 0 / ignored 0 / added 4 /
   removed 0; removal names: none.
   After refresh (`znpnsztt`), `cargo build -p xtask`, both full crate test
   commands above (999 English tests, 86 lexical-source tests), and both
   clippy commands above passed again. Citation checks again found 0
   noncompliant strings and 0 stale citations. No verification used a
   filtered English or lexical-source suite.
6. **Timing** (`ulrnwvuw`): bounded AFTER census wall time **39.13 s**,
   6 workers, `--samples-per-face 0`, under `/usr/bin/time -v`. No full
   corpus census was run, per the binding verification budget.
