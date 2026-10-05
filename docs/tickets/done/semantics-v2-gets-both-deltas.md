---
needs: []
---
**One modification carries both the power and the toughness delta, so "gets
+P/+T" writes its subject once.** Decided with the owner on 2026-10-05. Lean
first, then the mirror and RON. Standard constraints apply.

## The defect

The RON helper `gets(subject, power, toughness, duration)`
(`plugins_v2/builtin/macros/instructions/gets.ron`) writes the subject into
both modifications:

```ron
Establish(Conjunction(None, [Modification(Param(subject), Power, Param(power)),
                             Modification(Param(subject), Toughness, Param(toughness))]), Param(duration))
```

So `gets(target(creature), …)` is two target phrases, two targets where the
card has one. Lean's `getsPt` (`lean/Semantics/Macros.lean` ~L762) avoids this
by reading the toughness half's subject back as a computed pronoun
(`itsOther`, ~L755), which a RON macro cannot do (RON has no computed
arguments; see `semantics-v2-macro-capture-and-plurality`, which lists
`itsOther`). The six keyword bodies that call `gets` (bushido, exalted,
flanking, melee, prowess, rampage) all write the subject twice; it is harmless
only where the subject introduces no binding.

## What is decided

Change the model so ONE modification carries both deltas and the subject is
written once. `getsBoth(subject, delta, duration)` (same delta for both, from
`plugins-v2-keyword-helper-additions`) stays useful and is written over the
new form.

The exact constructor is the Lean landing's call: a new `StaticSpec` arm with
a power delta and a toughness delta, or a `Stat` that names both. It must also
carry `getsBase`'s set-both form ("has base power and toughness P/T"), which
Lean writes through `getsPt` with `.set` deltas. Whether a single-stat change
keeps the existing single-stat arm or becomes the new node with one side
unchanged is also the landing's call; record the choice and the reason.

## The work

1. **Lean.** Add the both-deltas modification in `lean/Semantics/Abilities.lean`;
   check it as one subject (`Check/`); re-spell `getsPt`, `getsBase` and their
   callers in the bench and pins. Every pin keeps its asserted outcome; add a
   pin that "Target creature gets +2/+2" introduces exactly one target.
2. **Mirror.** Follow in `crates/deckmaste_semantics_v2`; `lean_drift` holds.
3. **RON.** `gets` writes the new node with the subject once; the six keyword
   bodies follow. Counter conferrals that write a power and a toughness
   modification (`p1p1Counter`, `m1m1Counter`) may use it; that rewrite
   belongs to `semantics-v2-counter-kind-is-a-name`.

## Proof

`cargo xtask lean-check` passes; `cargo xtask expansions` before/after differs
only in the terms that wrote two modifications for one "gets", each listed.

## Landing record

The series, oldest first: `tlmsnnslpmyu` (Lean constructor, checker rules,
`getsPt`, pins, Rust mirror), `xszrzxyqwqxy` (RON alias, `gets`, `getsBoth`,
six keyword bodies, two reader tests), `wnrysrwmqllw` (ADR, CONTRACTS, the
sibling ticket's `getsBoth` item), and this record. Each stage was gated on
its own tree; the numbers below are stamped with the change they were taken
on. No English grammar, lexicon or corpus input changed.

**The node.** `StaticSpec.ptModification (subject : NounPhrase) (power :
Delta Amount) (toughness : Delta Amount)`, beside `modification` in
`lean/Semantics/Abilities.lean`; mirror `StaticSpec::PtModification {
subject, power, toughness }`; RON alias `ptModification`. A new constructor
rather than a change to `modification`: the single-stat form stays exactly as
it was (battle cry's "+1/+0" for each other attacker, the counter conferrals,
every single-stat pin), and `Stat` is shared with comparisons and stat reads,
where a "both" value would mean nothing. `getsBase`'s set-both form is the
same node with `.set` deltas [CR#613.4b]; a mixed pair is representable, as
it was before.

**Checker rules** (`tlmsnnslpmyu`).
- `Check/AbilityRules.lean:628`, `StaticSpec.check`: the subject checked once
  as an object; the power amount read after the subject's self-introduction;
  the battlefield zone check once; the toughness amount read after the power
  delta's introductions. This is the retired conjunction's check with the
  toughness half's pronoun check and zone check removed, in the same order,
  so the new refusal list is always a sublist of the old one.
- `Check/Abilities.lean:1298`, `StaticSpec.intro`: identical to the retired
  conjunction's (a pronoun's result leaves the context unchanged, so the
  toughness half added nothing).
- `Check/Abilities.lean:1548`, `StaticSpec.numberSlots`: both deltas' slots
  (the retired conjunction declared none; the function is read only by pins).
- Every other `StaticSpec` function falls to its wildcard, which equals what
  the conjunction gave (`clauseOk`, `introducedChoices`, `definedSlots`),
  except `isCoord`: the node is not a coordination, so it may stand as one
  part of a flat conjunction (`okGetsInsideCoordination`).

**Twin pins** (`lean/Semantics/Proofs/GetsBothDeltas.lean`, 63 theorems,
each by `decide` or `rfl`). The retired spelling is kept there as
`getsPtTwoHalves`, `getsBaseTwoHalves` and `getTwoHalves`. 50 twins assert
equal refusal lists, all proved: 19 for the existing pins that write `get`,
`getsPt` or `getsBase` (every one but the four below), 1 for bushido's
definition, 9 for pins that write "gets +P/+T" as two `modification`s by
hand (`okFlatCoordination`, `badNestedCoordination`,
`badThatCreatureIsStaticSubject`, `okCoordinatedPlural`,
`badCoordinatedHostPlural`, `sharedSubjectSurvivesSecondSingular`,
`okCoordinatedLandHostBlocks`, `okSingleStaticXRider`,
`badDoubleStaticRider`), 19 bench shapes (Somberwald Alpha, Deeproot
Warrior, Jeskai Ascendancy, Diminish and Cycle of Life through `getsBase`,
Bonds of Faith with "as long as it's a Human" reading after the node, Built
to Smash, an age-counter pump whose deltas read "it", Nyxathid as a whole
card, Archpriest of Iona, Nightmarish End and its X, Distortion Strike, an
equipped and an enchanted host, an "other creatures" anthem, an "each
creature" pump, and the three bench clauses written with `itsOther` by hand:
Awaken the Bear, Spidersilk Armor, `enchantedBecomesVanilla`), and 2 for
deltas that read their subject ("+X/+X where X is its power", alone and
after "destroy target artifact"). New: `okTargetGetsIntroducesOneTarget`
(the ticket's pin: one target), `subjectWrittenTwiceIntroducesTwoTargets`
(the old RON shape: two), `ptModificationSlots`, `okDeltasReadTheirSubject`,
`getsPtIsOneNode`, and `okGetsInsideCoordination` with its retired twin
`badGetsInsideCoordinationTwoHalves` (`[.notCoord]`).

**Pins whose expected value changed** (each still refused; the old value is
kept against the retired spelling in `GetsBothDeltas` as `…TwoHalves`):
- `Static.badThatCreatureIsCondSubject` and
  `Counters.badAltHeaderMixedReadback`: `[.anaphor (.word (.type .creature))
  .one 0, .zoneIs .battlefield, .anaphor .bare .one 0, .zoneIs .battlefield]`
  → `[.anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield]`.
- `Choice.badGetsGraveyard` and `Damage.badGetsSource`: `[.zoneIs
  .battlefield, .zoneIs .battlefield]` → `[.zoneIs .battlefield]`.
- Re-spelled over the node (same English): `Anaphora.badSharedSubjectTwoInDelta`
  `[.anaphor .bare .one 2, .anaphor .bare .one 2]` → `[.anaphor .bare .one 2]`;
  `Anaphora.okOwnReadsOneInDelta` `[]` → `[]`.
- STOPs: none. No pin, bench card or canon card went from refused to
  accepted.

**Proof.**
- `lean/scripts/build` on `tlmsnnslpmyu`: Build completed successfully (82
  jobs). `lean/Semantics/Cards` is unchanged.
- `cargo xtask lean-check` on `tlmsnnslpmyu` and on `xszrzxyqwqxy`:
  `plugins_v2/canon` 126/126, `plugins_v2/testing` 5/5 (85.7s, 85.2s).
- `cargo xtask facts check`: up to date on both.
- Gate: `cargo xtask gate --changed` on `tlmsnnslpmyu` derived `cargo test
  -p deckmaste_semantics_v2 -p xtask`; run with `--no-fail-fast`: 11 test
  binaries, 451 passed, 0 failed, 1 ignored (the test ignored before this
  landing). On `xszrzxyqwqxy` and again on `wnrysrwmqllw` it derived `cargo
  test -p deckmaste_construction_core -p deckmaste_lexical_source -p
  deckmaste_semantics_v2 -p deckmaste_construction_v3 -p
  deckmaste_english_v3 -p xtask`: 87 binaries, 1168 passed, 0 failed, 1
  ignored. `lean_drift` 4/4.
- `cargo xtask cite check`: 0 stale. `--list-noncompliant`: the 4 strings it
  lists are in `AGENTS.md` (the citation rule quoting its own bad examples),
  present before this landing. New citations: [CR#613.4b,613.4c] and
  [CR#107.1b], already registered; nothing blessed.
- `cargo xtask expansions` before (`ed928ae8`) and after (`xszrzxyqwqxy`):
  declarations 1503 → 1505 (printed 1455 → 1457), 0 failed. Added
  `ptModification`, `getsBoth`. Changed: `gets`, `bushido`, `exalted`,
  `flanking`, `melee`, `prowess`, `rampage`. Per-card term dumps (Rust
  `Debug` and emitted Lean): 6 canon cards changed, each through a keyword
  body (Akrasan Squire, Benalish Cavalry, Frost Giant, Kitsune Blademaster,
  Menagerie Liberator, Monastery Swiftspear), 0 testing cards. A normaliser
  that rewrites each `PtModification` node as the retired conjunction with
  the subject repeated makes all 7 changed declarations and all 6 cards (both
  dumps) byte-identical to the before tree: 13 nodes mapped, 0 unexplained.
- rustfmt: the changed Rust is clean.

**Tests.** Restored 0. Re-spelled 2 (`Anaphora.okOwnReadsOneInDelta`,
`Anaphora.badSharedSubjectTwoInDelta`), their old spellings kept beside the
twins. Expected value changed 4 (above), old values kept. Ignored 0. Added
63 Lean theorems and 2 reader tests
(`gets_writes_one_stat_change_naming_its_subject_once`,
`gets_both_is_gets_with_the_same_delta_twice`). Removed 0.

**Deviations and additions.**
1. The bench keeps its 26 hand-written two-modification clauses that read
   the toughness half through `itsOther` (Awaken the Bear, Spidersilk Armor,
   the Equipment and Aura clauses that coordinate a grant, and so on), so
   `itsOther` and `sameWindow` stay; the ticket's "re-spell their callers in
   the bench" is met for the callers of `getsPt`, `getsBase` and `get`,
   which change through the macros without changing their text. Three of
   those clauses are twinned.
2. `getsBoth` did not exist; it is added here (the sibling ticket
   `plugins-v2-keyword-helper-additions` listed it) and that ticket's item is
   marked done. It is used by all six keyword bodies, since bushido,
   exalted, flanking and prowess wrote the same delta twice as well.
3. The six canon cards that write two `modification`s by hand (Boar Umbra,
   Eldrazi Conscription, Flayer Husk, Sylvok Battle-Chair, Nyxborn
   Rollicker, Warrior's Sword) are unchanged; their subjects introduce
   nothing, so writing them twice is harmless. RON has no static `getsPt`.
4. `okGetsInsideCoordination`: the node may be one part of a flat
   conjunction, where the retired spelling was itself a conjunction and was
   refused `.notCoord` there. The English it spells ("gets +1/+1 and gains
   flying") was accepted before through the flat hand-written form.

**Glossary.** No new term; `docs/contexts/game-model/CONTEXT.md` defines
neither "modification" nor "power/toughness change", and this landing did
not need one.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed.

**Routed.**
- Whether the bench's hand-written two-modification clauses (deviation 1)
  and the six canon cards (deviation 3) should be re-spelled over the node,
  which would retire `itsOther`: for the owner.
- `get`, `getsBase`, `getsPt` now call no Lean-only macro and could port
  (§12.1 says so); the counter conferrals' two modifications remain
  `semantics-v2-counter-kind-is-a-name`.
