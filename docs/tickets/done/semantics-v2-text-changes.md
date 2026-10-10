---
needs: []
---
**Text-changing effects as specified rewrites over bracketed text.** Ruling
2026-10-09 ("A text-changing effect is a specified term rewrite over
bracket markers written on the card", `docs/decisions/semantics-v2.md` §7).
Bodies cleave, overload and splice. Standard constraints apply.

## Shape

- Markers `Predicate.bracketed (p : Predicate)` and `Instruction.bracketed
  (i : Instruction)`; in Rust one generic `Bracketed<T>`. Transparent to the
  checker (it checks the inner term), printed by spelling with its brackets.
  A card is written once, brackets in place.
- `StaticSpec.textChange (subject : NounPhrase) (change : TextChange)`,
  applied at layer 3 [CR#612.1,613.1c]. `TextChange` rows, each a specified
  term rewrite:
  - `removeBracketed`: every `bracketed` term is deleted (cleave
    [CR#702.148a]);
  - `targetToEach`: every `target(…)` determiner becomes `each(…)`
    (overload [CR#702.96a]); `target` is the constructor the rewrite finds;
  - `appendTextOf (source : NounPhrase)`: the source's rules text follows
    the spell's own (splice [CR#702.47a,612.10]);
  - `replaceWord (sort : QualitySort) (from : Option value) (to : Option
    value)`: a color word or a basic land type word replaced
    [CR#612.2] (Magical Hack, Mind Bend, Alter Reality; 13 cards).
- Checker: kinds only; the rewrite's result is not re-checked by the static.
- Spelling learns to print `bracketed` (the English side reads nothing new).

## Known limit

The alternative of storing a card's alternative readings was rejected
because it loses bracket positions. A rewrite must therefore be defined on
the term, and spelling prints the brackets. Record any card whose rewrite
cannot be stated over its terms as a STOP in its file.

## Re-spells (never deleted)

`Exchanged.textBoxes` (`lean/Semantics/Abilities.lean`,
`Check/AbilityRules.lean`, `Check/Abilities.lean`, `Cards/Keyword.lean`,
`Macros/Declared.lean`; Exchange of Words [CR#612.5]) and its probe are
re-spelled against `textChange` where the exchange is a text change,
keeping their asserted outcome.

## Bodies

- cleave: an alternative cost plus `textChange(thisSpell, removeBracketed)`
  under `keywordCostPaid(cleave, this)` [CR#702.148a] (12 cards).
- overload: an alternative cost plus `textChange(thisSpell, targetToEach)`
  [CR#702.96a] (27 cards).
- splice: reveal this card from hand as you cast a [quality] spell,
  `addedCost(Param(1), …)` on that spell, `textChange(thatSpell,
  appendTextOf(this))` [CR#702.47a,612.10] (30 cards).

## Witnesses (canon)

A cleave card with its brackets, an overload card, a splice card, and Mind
Bend (`replaceWord`). Pins: `bracketed` is transparent (a bracketed
mismatched kind is still refused); `removeBracketed` on a card with no
brackets is a no-op that still checks.

## Siblings

None by name. Refresh before integrate; expect conflicts in
`Words`/`Phrase`/`Abilities`.

## Landing record

Measured on `zrlsqmuu` (stages `yylkqrtz`, `loxmlxys`, `lvqmtkuq`, `oxnuosrr`,
`zrlsqmuu`; record `ylootmny`), and on `@` after the 2026-10-09 owner ruling
round (the last commit).

1. **Census delta**: none. No English grammar, lexicon or spelling changed, so
   no census was run; covered faces and Readings are unchanged. Lost faces:
   none.
2. **Witnesses** (each proved by `cargo xtask lean check`, canon 174/174 on
   `zrlsqmuu`): cleave with a predicate bracket, Wash Away ("spell
   [that wasn't cast from its owner's hand]"); cleave with a predicate and an
   instruction bracket, Dig Up ("[basic land]", "[reveal it,]"); overload,
   Vandalblast; splice with a non-mana cost, Torrent of Stone ("Splice onto
   Arcane—Sacrifice two Mountains"); `replaceWord`, Mind Bend (a color word
   or a basic land type), Magical Hack (target spell or permanent) and
   Balduvian Shaman. The re-spelled Text Box Exchange Probe
   proves in testing 8/8. Pins in `lean/Semantics/Proofs/TextChanges.lean`:
   `bracketedKindMismatchStillRefused` (a bracketed mismatched kind is refused
   exactly as without brackets) and `okRemoveBracketedWithoutBrackets` (the
   ticket's two), plus the cleave, overload and splice definitions checked
   whole.
3. **Spot checks** (the five new canon faces): Wash Away `[cleave({1}{U}{U}),
   spell(counter(target(and([spell, bracketed(not(castFrom(hand)))]))))]`;
   Dig Up `spell(sequentially([searchLibraryFor(exactly(1), and([card,
   bracketed(and([hasSupertype(Basic), land]))])), bracketed(reveal(it)),
   move(it, wherever, hand), shuffle]))`; Vandalblast
   `spell(destroy(target(and([artifact, not(youControl)])))), overload({4}{R})`;
   Torrent of Stone `spell(dealDamage(this, 4, target(creature))),
   splice(hasSubtype(arcane), perform(sacrifice(counted(exactly(2),
   and([hasSubtype(mountain), youControl])))))`; Balduvian Shaman `{T}:
   establish(textChange(target(white enchantment you control without
   cumulative upkeep), replaceWord([Color])))`, then the grant of cumulative
   upkeep {1}; Mind Bend `spell(establish(textChange(target(permanent),
   replaceWord([Color, Subtype(host: Land)]))))`; Magical Hack
   `spell(establish(textChange(target(or([spell, permanent])),
   replaceWord([Subtype(host: Land)]))))`. Only seven faces were added, so
   there are seven, not ten.
4. **Deviations and STOPs**.
   - The STOP first recorded here (Mind Bend's two kinds of word, and
     "spell or permanent" refused by `parallelDisjuncts`) is cleared by the
     owner ruling of 2026-10-09: `replaceWord` takes `sorts : List
     QualitySort` ("one of these kinds of word"), and `parallelDisjuncts` is
     retired under the checker charter (a join of objects is a legal
     subject), recorded as a dated note in §7's checker-sweep paragraph. The
     refusal constructor and its helpers `seedsUniform`, `parallelDisjuncts`
     and `Predicate.armPresupposes` are deleted.
   - `replaceWord` also refuses an empty `sorts` (`nonEmpty`) and a replaced
     word whose kind differs from the replacing word's (`kindMismatch`).
   - Added a fifth `TextChange` row, `exchangeWith (other)`
     [CR#612.5,701.12h], because the ruling re-spells `Exchanged.textBoxes`
     against `textChange` and none of the four named rows exchanges two texts.
     `Exchanged.textBoxes` is retired; `exchangeTextBoxes` (RON), Deadpool,
     Trading Card (`Cards/Keyword.lean`), `okExchangeTextBoxes`
     (`Proofs/Zone.lean`) and the generated `Declared.lean` are re-spelled to
     `establish(textChange(left, exchangeWith(right)), none)`, outcomes
     unchanged.
   - Added `TextWord = color | basicLandType (subtype)` as `replaceWord`'s
     value type, and the refusal of a word whose sort is not the row's, or a
     row sort other than color or land type, as `kindMismatch` (kinds only).
   - Rust mirror: two variants (`Predicate::Bracketed`,
     `Instruction::Bracketed`), not a generic `Bracketed<T>`, because
     `macro_ron`'s derives reject generic types (`reject_generics`); the note
     is on `Predicate::Bracketed`. `TextChange` derives `SupportsMacros` and is
     a registered kind (as `PipPaid` is) so its rows are lowercase helpers.
   - Transparency follows the `inCaller` precedent: every Predicate and
     Instruction checker function with an `inCaller` arm, plus
     `Instruction.performed`, `keyword`, `replacedCtx` and `unscoped`, reads a
     bracket's contents; the Rust `kind_of_predicate` likewise.
   - Splice's "as you cast a [quality] spell" is the `replacement` of the
     cast whose instruction is `may(reveal this, establish(conjunction(None,
     [addedCost(cost, false), textChange(that(Spell), appendTextOf(this))])))`,
     the shape Deadpool's "as this enters, you may" already has; the
     off-stack `asEvent` static of the 2026-10-09 ruling is not landed.
   - Cleave and overload put the Text Change under
     `keywordCostPaid(<keyword>, thisSpell)`; their facts rows gain
     `open_regimes: [(0, AtCasting)]` for the cast permission, and Facts were
     regenerated (cleave and overload `[static atCasting, static none]`,
     splice `[static none]`).
   - Wash Away's "its owner's hand" is the bare hand zone (`castFrom(hand)`):
     `castFrom(handOf(ownerOf(it)))` is refused (`anaphor bare one 0`), a
     predicate having no pronoun for its own member.
   - Spelling: no v2-to-English printer exists, so "spelling prints
     `bracketed`" is the `bracketed` helper (Predicate and Instruction kinds)
     a card writes its brackets with; the expanded and emitted term keeps them.
   - The known limit is as the ticket states: a rewrite is defined on terms,
     and a card prints its brackets where they stand.
   - Glossary: Text Change amended to list the text-box exchange
     [CR#612.5,701.12h] and to say a word is replaced by another of its kind. Newly blessed rules: [CR#702.96b,702.148b], each read
     against its citing claim.
   - `Declared.lean` regeneration also refreshed the stale `typecycling`
     module from trunk, as briefed.
5. **Tests** (on `zrlsqmuu` plus the record): `cargo test -p
   deckmaste_semantics_v2` 100 passed; `cargo test -p xtask` 416 passed, 1
   ignored (existing); `cargo test -p deckmaste_construction_core` 468
   passed; `cargo test -p deckmaste_lexical_source` 64 passed; `cargo clippy
   -p <each> --tests -- -D warnings` clean for all four; `lake build --wfail`
   clean (93 jobs); `cargo xtask lean check` canon 174/174, testing 8/8;
   `cargo xtask lean definitions` 708/708; `cargo xtask facts check`,
   `cargo xtask lean macros check` up to date; `cargo xtask lean macros align`
   333 of 381 aligned, the six new bench macros among them (48 drifted, none
   of them this landing's); `cargo xtask cite check` 0 stale,
   `--list-noncompliant` empty. Counts: restored 0; re-spelled 2
   (`okExchangeTextBoxes`, the Deadpool, Trading Card bench card), with the
   Text Box Exchange Probe re-spelled through its helper and its card file
   unchanged; ignored 0; added 12 Lean pins
   (`bracketedKindMismatchStillRefused`, `bracketedKindMismatchIsRefused`,
   `okBracketedDescription`, `okBracketedInstruction`,
   `bracketedInstructionMismatchStillRefused`,
   `okRemoveBracketedWithoutBrackets`, `okReplaceLandTypeWord`,
   `badReplaceWordSort`, `badReplaceWordCardName`, `okCleaveDefinition`,
   `okOverloadDefinition`, `okSpliceDefinition`) and 5 canon cards; removed 0.
   After the refreshes onto the integrated `semantics-v2-modal-choice`
   (conflicts in `Abilities.lean`, the mirror's `StaticSpec`, the expansion
   samples and the two generated modules) and `semantics-v2-sharing` (one in
   `Check/Phrase.lean`'s `qualityReadOk`), each resolved keeping both sides and
   the generated modules regenerated,
   `lake build --wfail`, the four crate test suites (`deckmaste_semantics_v2`
   now 101) and clippy runs, `facts check`, `lean macros check` and `cite
   check` were rerun with the same outcomes; `lean check` and `lean
   definitions` were not rerun.
   Ruling round (on `@`, the last commit): `lake build --wfail` clean (94
   jobs); `cargo test -p deckmaste_semantics_v2` 101 passed; `cargo test -p
   xtask` 416 passed, 1 ignored (existing); clippy clean for both; `cargo
   xtask lean check` canon 190/190 (trunk's cards plus Mind Bend and Magical
   Hack), testing 8/8; `cargo xtask lean definitions` 708/708; `lean macros
   check`, `facts check`, `cite check` (0 stale, none non-compliant) pass.
   Re-spelled by the ruling, each a deliberate retirement with a new
   asserted outcome, none deleted: `badSpellOrPermanentSubject` →
   `okSpellOrPermanentSubject`, `badHeadlessDisjunct` → `okHeadlessDisjunct`,
   `badCrossZoneDisjunction` → `okCrossZoneDisjunction` (`Proofs/Zone.lean`),
   `badPartialZoneJoin` → `okPartialZoneJoin` (`Proofs/Description.lean`),
   `badMixedCharacteristicDisjunct` → `okMixedCharacteristicDisjunct`
   (`Proofs/Deontic.lean`), all now `= []`, and `badMixedDisjunctionActivated`
   (`Proofs/Turn.lean`) from `[parallelDisjuncts, deedFits]` to `[deedFits]`;
   `okReplaceLandTypeWord`, `badReplaceWordSort` and `badReplaceWordCardName`
   re-spelled to the list form, same outcomes. Added
   `okReplaceColorOrLandTypeWord` and `badReplaceAcrossKinds`, and the Mind
   Bend and Magical Hack canon cards. Totals for the landing: restored 0,
   re-spelled 11 (2 + 6 + 3), ignored 0, added 14 pins and 7 canon cards,
   removed 0. After the ruling round's refresh onto `semantics-v2-linked-readback`
   (conflicts beside `addsTarget` in `Abilities.lean` and the mirror, the
   expansion samples and the generated modules, resolved keeping both sides
   and regenerating), `lake build --wfail` (95 jobs), `cargo test -p
   deckmaste_semantics_v2` (104 passed), `cargo test -p xtask` (416 passed, 1
   ignored), clippy, `lean macros check`, `facts check` and `cite check` were
   rerun clean; `lean check` and `lean definitions` were not rerun.
6. **Timing**: no census run. `cargo xtask lean check` 333 s wall on the
   ruling round (5 min 32.7), one `lake` process.
