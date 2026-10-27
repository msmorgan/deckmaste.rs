---
needs: [plugins-v3-builtin-primitives-and-templates]
---
# "Those spells" after a cast permission, and Ice Cauldron's noted mana

Standard constraints apply. Design ticket, design proposed below. Found by the 2026-10-25
triage of an external audit ("Atraxa"), verified against `wmyykqnv`. Paths are under
`lean/Semantics/`. Two parts, each separately committable; neither depends on the other. Probes
quoted below are in
`/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/4e5532ee-6640-4aad-9769-888a72f918d1/scratchpad/q16/Those.lean`.

## Part 1: "those spells" / "that spell" after a cast or play permission

### The cards

- *Rogue Class* level 3 (`Cards/Choice.lean:1289-1294`, `rogueClassLevelThree`, pin
  `okRogueClassLevelThree` `:1294`): "You may play cards exiled with this Class, and you may spend
  mana as though it were mana of any color to cast those spells."
- *Summon: Esper Valigarmanda* chapters II-IV (`Cards/Choice.lean:1280-1286`,
  `summonEsperValigarmandaCast`, pin `okSummonEsperValigarmandaCast`): "You may cast an instant or
  sorcery card exiled with this Saga, and mana of any type can be spent to cast that spell."

Both re-describe the anaphor: `toCast (in_ (exiledWith thisClass))` and `toCast (and
[isInstantOrSorcery, in_ (exiledWith thisSaga)])`. Two things are missing:

1. `SpendPurpose.toCast (predicate : Predicate)` (`Abilities.lean:599-600`; Template
   `Templates.lean:2359`) cannot take an anaphor.
2. The checker refuses the anaphor even in a NounPhrase slot. Probed:
   `conjunction [can you (play (allOf (cardIn (exiledWith thisClass)))), can you (cast (those
   .spell))]` gives `[.anaphor (.word .spell) .many 0]`; Esper's `cast (that .spell)` gives
   `[.anaphor (.word .spell) .one 0]`. (`cast them` checks `[]`, but drops the printed
   "spells"; do not use it.)

### Design

1. **`toCast` takes a NounPhrase.** Change `SpendPurpose.toCast (predicate : Predicate)` to
   `SpendPurpose.toCast (spells : NounPhrase)`, and the Template to
   `semantic_template toCast (spells : NounPhrase) : SpendPurpose := .toCast spells`. Every
   current site writes a description; rewrite each to `bare <its predicate>` (the generic plural
   "to cast creature spells"), keeping the predicate:
   `Cards/Mana.lean:222`, `:229`, `:275`, `:286`, `:306`, `:956`, `:1029`;
   `Cards/Choice.lean:1285`, `:1293`, `:1421`; `Proofs/Lookback.lean:209`;
   `Proofs/Rules.lean:653`, `:667`; `Proofs/CardFidelity.lean:327`. Update the checker arm that
   steps `toCast` (find it by `SpendPurpose.toCast` in `Check/`) to step a NounPhrase in the
   caller's context instead of a Predicate at kind `.object`.
2. **A card a permission lets you cast is "that spell" to the text after it.** Casting moves the
   card to the stack, where it is the spell [CR#601.2a]; the checker already applies this to a
   performed `play`/`cast` (`applyProcedureDeed`, `Check/Phrase.lean:630-662`; pin
   `playRecordsALandPlayOrASpellCast`, `Proofs/Lookback.lean:906`). Extend it to a `can`
   permission over `cast`/`play`: when a `conjunction` (or a `doIf` continuation) steps
   `can <who> (cast|play X)`, X's referent is published to the later conjuncts with the
   procedure deed stamped, so the `.spell` head (and `.subtype (.spell _)`) reaches it through
   `stampedWordOk`'s procedure arm, as after a performed cast. The permission's own `allOf …` or
   `a …` determiner fixes plurality ("those spells" for *Rogue Class*'s `allOf`, "that spell" for
   Esper's `a`). Nothing else about a permission's referents changes; a `can` outside a
   conjunction publishes nothing new.
3. **Respell the two cards:**
   - *Rogue Class*: `conjunction [ can you (play (allOf (cardIn (exiledWith thisClass)))),
     can you (pay anyCost .once) (asThough := some (spendManaAs none .anyColor (toCast (those
     .spell)))) ]`. The `cardIn` head noun is also `lean-v3-card-head-noun-sweep` §1 row 5's;
     whichever lands second keeps it.
   - *Esper*: `conjunction [ can you (cast (a (and [isInstantOrSorcery, isCard, in_ (exiledWith
     thisSaga)]))), can you (pay anyCost .once) (asThough := some (spendManaAs none .anyType
     (toCast (that .spell)))) ]`.

### Pins

- Rewritten (same claim, new spelling): `okRogueClassLevelThree`, `okSummonEsperValigarmandaCast`,
  and every pin over a `toCast` site above.
- Added: `castPermissionPublishesThatSpell` (Esper's two conjuncts check `[]`, and the second
  resolves to the first's referent) and `thoseSpellsNeedThePermission` (`can you (pay anyCost
  .once) (asThough := some (spendManaAs none .anyColor (toCast (those .spell))))` alone is
  refused `[.anaphor (.word .spell) .many 0]`) in `Proofs/Choice.lean`.
- Must stay green: `playRecordsALandPlayOrASpellCast`, every pin in
  `Proofs/KeywordCastPermissions.lean`.

## Part 2: Ice Cauldron

### The card

*Ice Cauldron* (`Cards/Mana.lean:1023-1030`, fragment `iceCauldronNotedMana`, pin
`okIceCauldronNotedMana`): "{X}, {T}: You may exile a nonland card from your hand. You may cast
that card for as long as it remains exiled. Put a charge counter on this artifact and note the
type and amount of mana spent to pay this activation cost. Activate only if there are no charge
counters on this artifact. / {T}, Remove a charge counter from this artifact: Add this
artifact's last noted type and amount of mana. Spend this mana only to cast the last card exiled
with this artifact."

Today's spelling, `addMana 1 (ofTheLastNotedType thisArtifact) [spendOnly [toCast
exiledWithThisArtifact]]`, has three defects: the amount is a literal 1, "the last" is dropped,
and the first ability (which writes the note) is not authored. The two abilities are linked
[CR#607.2e,607.2a].

### Design

1. **Noting.** No action records information today (checked 2026-10-25: no "note" or
   "remember" action in `Abilities.lean` or `Templates.lean`). Add `Action.noteManaSpent`, no
   arguments, with Template `noteManaSpent : Action := .noteManaSpent`: "note the type and
   amount of mana spent to pay this activation cost" [CR#607.2e]. It is a linked-ability writer;
   the stepper treats it as an action that introduces no referent, and its readers are the
   existing `ProducedMana.lastNoted subject` (`Abilities.lean:730`, Template
   `ofTheLastNotedType`, `Templates.lean:2349`) and item 2's amount. Add it to the event-head
   list only if a card printing "whenever … note" is found (none was).
2. **The noted amount.** Add `Amount.lastNoted (subject : NounPhrase)` beside
   `ProducedMana.lastNoted`, with Template `theLastNotedAmountOf (subject : NounPhrase) : Amount`.
   It reads the amount the linked note recorded, as `ProducedMana.lastNoted` reads its type.
3. **"The last card exiled with this artifact."** No recency selector exists: `Quantity.theLast`
   (`Abilities.lean:463-469`) is "the removal that leaves none of its kind" for counters, and
   `Predicate.superlative` (`:298`) ranges over a `ProjAxis` with no time axis. An object
   receives a timestamp when it enters a zone [CR#613.7d]. Add `ProjAxis.zoneEntry`, the
   timestamp at which the object entered its current zone, and write "the last <p>" as
   `the (and [p, superlative .max .zoneEntry p])`; it composes with the existing `superlative`.
4. **Spell the whole card** in `Cards/Mana.lean` as a `spelled` card replacing the fragment:
   the first ability (exile up to one nonland card from your hand, the cast permission "for as
   long as it remains exiled", `putCounters 1 chargeCounter thisArtifact`, the note, and
   "Activate only if there are no charge counters on this artifact"); the second
   `addMana (theLastNotedAmountOf thisArtifact) (ofTheLastNotedType thisArtifact) [spendOnly
   [toCast (the (and [isCard, in_ (exiledWith thisArtifact), superlative .max .zoneEntry (and
   [isCard, in_ (exiledWith thisArtifact)])]))]]` (with `toCast` over a NounPhrase if Part 1 has
   landed, else over the predicate). If a step of this design does not check, STOP and report the
   refusal rather than reaching for a raw constructor.

### Pins

- Retired: `okIceCauldronNotedMana`, replaced by the spelled card (its claim, that the mana
  ability checks, is kept inside the card).
- Added: `iceCauldronAddsTheNotedAmount` (a `decide` that the second ability's amount is
  `Amount.lastNoted thisArtifact`), and `zoneEntryIsAStatAxis` (the new axis is accepted under
  `superlative` at kind `.object` and refused at `.player`), in `Proofs/Mana.lean`.

## Acceptance (each part)

`./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates (bless newly
cited rules). Glossary: name any new public item against `docs/contexts/game-model/CONTEXT.md`
first and add an entry through `domain-modeling` if it names a new concept.
