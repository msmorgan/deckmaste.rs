---
needs: []
---
**One cost-substitute mechanism: an action that pays for mana.** Ruling
2026-10-08 ("A cost substitute is one mechanism", `docs/decisions/semantics-v2.md`
§7). It replaces `Cost.tapForGeneric`, the residues landing's waterbend-only
node. Standard constraints apply.

## Shape

- `ManaSubstitute (action : Instruction) (pays : List PipPaid)`, with
  `PipPaid = generic | ofColor (color : ColorTerm)`. The action comes first;
  `pays` reads the acted-on object back by anaphora ("that creature's
  color"). `ColorTerm` (today `lit | chosen`) gains an object's colour,
  `colorOf (object : NounPhrase)`. One row per keyword, no per-colour
  expansion.
- Carriers:
  - `StaticSpec.paySubstitute (subject : NounPhrase) (substitute :
    ManaSubstitute)`, on a spell. It applies after the total cost is locked
    and is neither an additional nor an alternative cost
    [CR#702.51b,702.66b,702.126b].
  - `Cost.withSubstitute (cost : Cost) (substitute : ManaSubstitute)`, for a
    named cost. It pays only that cost's generic mana [CR#701.67b].
- Checker: cases for the anaphor in `pays` (it must resolve to the action's
  patient) and for kinds. It refuses nothing else.
- Rust mirror, RON helper `substitute`, and bench macros spelled like the
  RON target.

## Retires

`Cost.tapForGeneric` (Lean and Rust mirror), the RON helper
`plugins_v2/builtin/macros/abilities/tapForGeneric.ron`, and the checker
cases for both.

## Re-spells (same asserted outcome)

- `waterbend.ron`: `withSubstitute(<cost>, substitute(tap(an(and([untapped,
  or([artifact, creature]), youControl]))), [generic]))` [CR#701.67a].
- Canon: Geyser Leaper.
- Pins in `lean/Semantics/Proofs/KeywordActionResidues.lean`: `okWaterbend`,
  and `badTapForGenericWithPlayers` re-spelled as a substitute whose action
  names players. Also every Rust test that names `tapForGeneric`
  (`crates/deckmaste_construction_core/tests/builtin_v2_keyword_actions.rs`,
  `builtin_v2_keyword_spelling_templates.rs`).

## Bodies

- convoke: `paySubstitute(this, substitute(tap(an(and([untapped, creature,
  youControl]))), [generic, ofColor(colorOf(that(Creature)))]))` [CR#702.51a].
- delve: `substitute(exile(a(cardIn(yourGraveyard))), [generic])` [CR#702.66a].
- improvise: tap an untapped artifact you control, `[generic]` [CR#702.126a].
- assist stays bodyless: another player pays, and no action does
  [CR#702.132a]. Record it as a deliberate residue.

## Witnesses

Geyser Leaper (waterbend), plus one canon card each for convoke, delve and
improvise, added if canon lacks one. Pins: convoke's colour readback
resolves; a substitute whose `pays` anaphor has no antecedent is refused.

## Deltas

A, B and C all touch `Words`/`Phrase`/`Abilities` and run in parallel.
Refresh before integrate and expect conflicts there.

## Landing record

Measured on `omwvxtrt` (canon commit, the tip under this record).

1. **Census delta**: not applicable. No English grammar, lexicon or keyword
   spelling changed, so no english-v3 census was run; covered faces and
   Readings are untouched. Lost faces: none.
2. **Witnesses**: English Readings do not apply (semantics-v2 only). Lean:
   `cargo xtask lean check` proves 163/163 cards (155 canon, 8 testing),
   among them Geyser Leaper (waterbend), Damocles Base, Sword of Kang (crew
   3) and the three new canon witnesses Stoke the Flames (convoke),
   Treasure Cruise (delve) and Reverse Engineer (improvise).
3. **Spot checks** (bodies as they now expand): convoke =
   `static(paySubstitute(thisSpell, substitute(tap(a(and([untapped,
   creature, youControl]))), [generic, ofColor(colorOf(that(Creature)))])))`;
   delve = `substitute(exile(a(cardIn(graveyardOf(you)))), [generic])`;
   improvise = tap an untapped artifact you control, `[generic]`; waterbend =
   `pay(withSubstitute(Param(0), substitute(tap(a(and([untapped,
   or([artifact, creature]), actorControls]))), [generic])), Once)`; crew =
   `activated(perform(tap(withTotal(Stat(Power), AtLeast, Param(0),
   allOf(and([untapped, otherCreatureYouControl(thisPermanent)]))))),
   establish(characteristicChange(thisPermanent, [TypeLine(Adds, (types:
   [Artifact, Creature]))]), untilEndOfTurn))` [CR#702.122a]; assist stays
   bodyless as a recorded residue [CR#702.132a]. Fewer than ten: the ticket
   adds five bodies and three cards, no English faces.
4. **Deviations and STOPs**: no STOP. `ColorTerm` and `ManaTypeTerm` moved
   into `Phrase.lean`'s mutual block (ColorTerm now holds a `NounPhrase`), so
   ColorTerm derives `Repr, BEq` with the block instead of `DecidableEq`, and
   `ColorTerm.check` became `ColorTerm.checkIn` inside the PhraseRules mutual
   block with an abbrev. `ManaSubstitute` is a `structure` (as `RollRow`).
   The checker reads `pays` against only the bindings the action introduced,
   refuses an empty `pays` (`nonEmpty`), and checks `colorOf` as one object.
   `StaticSpec.paySubstitute` is an at-casting component in
   `StaticSpec.regimeFits` and xtask's `static_regime` [CR#702.51b,601.2h];
   `cargo xtask facts generate` then gave convoke, delve, improvise a
   `[static atCasting]` definition and crew `[activated]`. `Cost.intro` of
   `withSubstitute` is its cost's intro (tapForGeneric dropped it). Spelling
   deltas from the ticket's letter: `a(…)` for `an(…)` (no `an` helper),
   `graveyardOf(you)` for `yourGraveyard` (no such helper), `thisSpell` for
   `this` (affinity's subject), waterbend keeps `actorControls` (a keyword
   action names its performer `actor`). Crew uses the existing `withTotal`
   helper, which is `someOf(totalling(…))`. Ticket's Rust tests naming
   `tapForGeneric` did not exist (none named it). Glossary: Mana Substitute
   is defined; `PipPaid` ("the mana one act pays") has no entry of its own.
5. **Tests**: `cargo test -p deckmaste_semantics_v2`, `-p xtask` (382 + 18,
   1 pre-existing ignore), `-p deckmaste_construction_core`,
   `-p deckmaste_lexical_source`: all pass. `cargo clippy -p <each> --tests
   -- -D warnings`: clean. `lake build --wfail`: clean. `cargo xtask facts
   check`: up to date. `cargo xtask cite check`: 0 stale, 0 non-compliant.
   Counts: restored 0; re-spelled 2 (`okWaterbend` keeps its statement over
   the new waterbend; `badTapForGenericWithPlayers` -> `badSubstituteWithPlayers`,
   same refusal `[.kindMismatch .object .player]`); ignored 0; added 4
   (`okConvokeColorReadback`, `badSubstitutePaysWithoutAntecedent`,
   `okCrewCost`, Rust `a_substitutes_pips_read_at_their_own_kind`); removed 0.
6. **Timing**: no census run. `cargo xtask lean check`: 132.3 s;
   `cargo xtask lean definitions`: 29.1 s (553 definitions).
