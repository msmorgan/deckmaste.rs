---
needs: []
---
# Checker rules that enforce runtime game state instead of text meaning

Standard constraints apply. The boundary is `docs/decisions/semantics-v3.md` §0.1 item 4 (owner
ruling 2026-10-22). Every decomposable game rule is data in the DSL, and Lean does not check that
such a rule is defined correctly. The checker owns the meaning of text: compositional denotation,
discourse and reference resolution, and sort and binder well-formedness. The engine owns
runtime state: hidden-zone visibility, the availability of last known information
[CR#608.2h], timing, and the order in which state-based actions and replacement effects apply
[CR#704.3,616.1].

**Ruling (owner, 2026-10-24).** K1 to K7: confirmed Keep (decidable from the text). U1:
confirmed Keep [CR#701.24a,701.24b], and keep the 5 shuffle pins. U2: remove the current-zone
restriction on "spell" for characteristic and possessor reads [CR#608.2h]; see U2 below. The
remaining sections are the inventory as taken.

**Inventory origin.** It was taken read-only against the current
code on 2026-10-22, from `lean/Semantics/Check/*.lean`, including `Check/Refusal.lean`, and from
a sweep of `Check/Phrase.lean` and `Check/Words.lean` for zone and visibility checks. Each row
gives the proposed class: **keep** (text-decidable), **remove** (runtime) or **unsure**. The owner
confirms or corrects each class before any removal beyond the one already ruled.

**Where the `publicOnly` removal lives.** Rows R1 to R3 are already ruled (Decision 3) and are
owned by [lean-v3-ninjutsu-defender-lki](lean-v3-ninjutsu-defender-lki.md). Do not repeat them
here. This ticket's work is to confirm the remaining classes and to remove what the owner
confirms as runtime.

No `Refusal` constructor is dedicated to runtime state. The runtime filters surface through
generic reference refusals (`anaphor`, `participialDescription`, `groupSizeInScope`,
`bindingIdentity`), so the inventory is of filters, not of constructors. The `/- zones -/` group in
`Refusal.lean` holds only `ordinalNonZero`.

## Remove (runtime): owned by the ninjutsu ticket

- **R1. `publicOnly`** (`Check/Words.lean:1456`, over `Zone.isPublic`/`Referent.isPublic`
  `:954-963`, which also hides face-down piles). Applied across an activated ability's colon
  (`Check/Phrase.lean:3761`), a special action's (`:3704`) and in `Ability.textContextWith`
  (`:2108`). Whether the engine can find an object a cost moved to a hidden zone is runtime
  visibility [CR#400.2,608.2h].
- **R2. The linked-result public-zone filter** (`linkedResult`, `Check/Phrase.lean:1928-1932`;
  used at `:2023`, `:2107`). The deed stamp it also requires is text meaning [CR#607.2a] and
  stays.
- **R3. The public-zone conjunct of `publicCostResult`** (`Check/Phrase.lean:1920-1924`; used at
  `:1954`). It was pinned by `additionalCostDoesNotPublishAHiddenGroup`
  (`Check/AbilityRules.lean:53`).

R1 to R3 landed with the ninjutsu ticket: `publicCostResult` is now `costResult`, and the pin is
rewritten as `additionalCostPublishesAGroupPutIntoALibrary`.

## Unsure (resolved 2026-10-24)

- **U1. Shuffling forgets library referents: KEEP (owner, 2026-10-24).**
  (`survivesShuffle`, `Check/Words.lean:1208-1213`; `afterShuffle`, `:1573-1579`; called at
  `Check/Phrase.lean:3922`). After a shuffle, every object referent in a library is dropped from
  discourse unless a library search found it. A shuffle only randomizes the cards so that no
  player knows their order [CR#701.24a], and the found cards are singled out
  [CR#701.24b]. The rule is text meaning: "that card" after an unrelated shuffle has no antecedent
  Oracle writes. Keep the 5 pins: `shuffleForgetsMaskedLibraryIdentity`,
  `revealedSearchResultSurvivesShuffleWithSameIdentity` (`Proofs/MovementScopes.lean:79,178`),
  `conditionalShuffleForgetsPrivateOperand` (`Proofs/ActionFamilies.lean:275`),
  `badReadsShuffledIntoLibraryCard` (`Proofs/Description.lean:417`) and
  `badReadsShuffledLibraryCard` (`Proofs/Anaphora.lean:786`). Line numbers re-located 2026-10-24.
- **U2. "spell" in characteristic and possessor reads: REMOVE the current-zone restriction
  (owner, 2026-10-24).** Today `stampedWordOk` (`Check/Words.lean:1184-1192`, via
  `reachesCharacteristic`, `:1226-1232`) lets a Card Type or "permanent" word read a departed
  permanent by its last known information, but `.spell` (and `.subtype (.spell _)`) requires
  `onStackZone zn`. The ruling: "that spell's mana value" or "that spell's controller" after the
  spell is countered (*Mana Drain*, *Spell Swindle*, *Essence Backlash*) reads the departed spell
  by last known information [CR#608.2h]: the spell was in the public zone it was expected to be
  in, and it has left it. Admit a departed stamped spell in `stampedWordOk` the way
  `wasField || leftField` admits a departed permanent (a stack analogue of the stamp's field
  flags). **Action slots still require the object to be on the stack** (K1,
  `slotZoneOk .spell`, `Check/Words.lean:1062-1065`): "counter that spell" of an already-countered
  spell stays refused. Pins: *Mana Drain*, *Spell Swindle* and *Essence Backlash* spelled in full;
  a refusal pin that an action slot on the departed spell is still refused; rewrite or retire by
  name any existing pin that asserts the old "spell requires the stack now" read for a
  characteristic.

## Keep (text-decidable; confirmed by the owner, 2026-10-24)

- **K1. Carrier-word and action-slot zone reach** (`slotZoneOk`, `Check/Words.lean:986-989`;
  `wordReaches`, `:1030`; ADR §1.5 `J1` items 3 and 4). A noun word denotes objects in a zone
  [CR#109.2], and the zone comes from the text's own moves. A card the text put into a hand is
  not a permanent the text can tap.
- **K2. Last-known-information characteristic reads of departed permanents**
  (`reachesCharacteristic`, `Check/Words.lean:1145-1150`). This rule admits a reference, it
  does not police one. Its ADR qualifier "visible" falls with R1.
- **K3. Zone-change identity** (`applyZoneMove`, `Check/Phrase.lean:523-576`): a moved object
  is a new object [CR#400.7], fixed by the text's own move.
- **K4. `lookbackLocus`** (`Check/Phrase.lean:1067-1070`): a lookback's written zone must be one
  the deed acts in, read from deed facts (rule data).
- **K5. `riddenEntering` and `entryRidersOnly`** (`Check/Phrase.lean:1779-1780`): the shape of
  a replacement that adds Entry Riders [CR#614.1c,614.12], not its runtime application.
- **K6. Rules-as-data checks** (`Check/Rules.lean:19-63`: `SbaRule.check`,
  `ConferralRule.check`, `Definition.check`). These check well-formedness only and verify no
  rule's correctness. This already matches the boundary.
- **K7. `Location.face`** (`Check/AbilityRules.lean:107-116`): default pile faces as rule data
  [CR#401.2,404.2]. Its only visibility consumer is `Referent.isPublic` (R1). The wrong
  `exiledWith` default is `lean-v3-checker-rules-scope-refusals` §2.

Confirmed absent: no checker rule simulates timing (`stepTimingWith`, `Check/Phrase.lean:905`,
only resolves the references a timing clause makes) or orders state-based actions or replacement
effects.

Not runtime, and not on this ticket's axis: the keyword card-class gates (`keywordCardOk`,
`Check/Card.lean:29-31`, over `Check/Facts.lean`) are corpus observations.
`lean-v3-checker-rules-scope-refusals` §5 removes them.

Tally (after the owner ruling): 3 removed (owned by the ninjutsu ticket), 1 remove (U2), 1 keep (U1), 7 keep (K1 to K7).

## Wave-C tickets against the boundary

**`lean-v3-checker-rules-scope-refusals`**
- §1 text changes reach creature types: text meaning (what a text change may denote), in scope.
- §2 "exiled with" is not face down: rule data and text meaning [CR#406.3], in scope.
- §3 self-cast flash permission on a spell card: card-frame well-formedness, in scope. Whether
  the permission applies at a given moment is the engine's.
- §4 linked referents inside token or granted text: reference resolution, in scope.
- §5 keyword card-class gates: not text meaning. Removing them agrees with the boundary.
- §6 revealed cost result readable in place: runtime visibility. **Superseded** by R3 in the
  ninjutsu ticket, and its "keep the public-zone requirement for moved ones" contradicts
  Decision 3. Only its pin (*Charge of the Forever-Beast*) remains in scope.
- §7 "enchanted creature card" is not on the battlefield: carrier-word meaning [CR#109.2], in
  scope.

**`lean-v3-resolution-riders-and-guards`**
- §1 "can't be regenerated": the denotation (a rider on the destroy event [CR#608.2c,701.19c]) is
  in scope. The pin "a shield created before the destroy does not apply" simulates a runtime
  replacement and is out of scope; it belongs to the engine.
- §2 a countered ability ceases to exist: rule data (the `counter` Definition [CR#701.6a]) and
  its discourse consequence (no graveyard referent), in scope.
- §3 a resolution-time guard reads current state: stating what the guard denotes is in scope.
  The *Affectionate Indrik* pin (the creature leaves before the trigger resolves) is runtime
  simulation and out of scope; the engine half is `engine-condition-lki-context`.
- §4 *Quenchable Fire* as a Special Action: the denotation and the missing "stop a delayed
  trigger" action are in scope [CR#116.2c]. When the action is available is the engine's.

**`lean-v3-event-ordinals-and-counter-thresholds`**
- §1 *Moonlit Meditation*'s ordinal on the replaced event: the `WouldClause` ordinal form is
  denotation, in scope. Its two pins (a token made before the Aura; declining the first
  replacement) simulate a turn's history and are out of scope (engine tests).
- §2 the Nth counter as a threshold: the threshold quantity is denotation [CR#122.7], in scope.
  Its pins (one counter removed in between; a batch of three from ten) are runtime simulation and
  out of scope.

## Work

1. ~~The owner confirms or reclasses U1, U2 and the keep rows.~~ Done 2026-10-24 (see the ruling above).
2. Remove each row confirmed as runtime (U2 only), rewriting or retiring its pins by name, and mark
   superseded in `docs/decisions/semantics-v3.md` and `lean/CONTRACTS.md` the text that
   describes it.
3. Move the out-of-scope pins named in the triage to engine tickets, and leave the denotation
   work in the wave-C tickets.

## Verification

`./lean/scripts/build`; `cargo xtask lean check`; the CR citation gates.

## Added 2026-10-25 (Atraxa and engine-creep audit triage)

Re-checked against `wmyykqnv` under the owner's 2026-10-25 direction (the workbench represents
what cards can say; a gameplay distinction stays only where reference resolution needs it).

- **K7 is dead code: delete it.** Since R1 landed, `Location.face`
  (`Check/AbilityRules.lean:103-116`) has no consumer: its only references are its own
  recursion (`:110`) and the pin `zonePilesFixTheirFaces` (`Proofs/Piles.lean:133-140`). Delete
  the def with its docstring and retire the pin (it pins a function nothing reads). Keep
  `faceOfReach` (`Check/Words.lean`, used by the stepper) and `Referent.face`
  (`Check/Words.lean:424`).
- **U2 confirmed by probe.** *Mana Drain* ("Counter target spell. At the beginning of your next
  main phase, add an amount of {C} equal to that spell's mana value.") is refused today with
  `[.anaphor (.word .spell) .one 0]`, while bare "its mana value" checks `[]`; "counter that
  spell" afterwards is refused, as K1 wants. The arm to change is the `.spell` /
  `.subtype (.spell _)` case of `stampedWordOk` (`Check/Words.lean:1228`): admit a stamped
  referent whose zone-change origin is the stack (the stamp built by `moveStamp`,
  `Check/Words.lean:1112-1115`). Change it for the characteristic read (`reachesCharacteristic`,
  `:1267`) only, not for `participialCandidates` (`:1396`). No existing pin asserts the old
  restriction for a characteristic read (checked: `prohibitionsStillRefuseMissingCallerParameters`,
  `exiledAbilityIsNotASpellOrCard`, `badRevealedAsAPlayerIsCast`). Update the stamped-word rule in
  `lean/CONTRACTS.md:415-418`.
- **K4 no longer exists under that name** (`lookbackLocus` is gone); nothing to classify.
- **K6** (`Check/Rules.lean`) stays as it is: rules rows stay as DSL items (decided 2026-10-25), so
  `SbaRule.check`, `ConferralRule.check` and `Definition.check` are neither removed nor reshaped here.
  `lean-v3-registry-definition-rename` renames `Definition` to `RegistryDefinition`; this ticket
  does not touch that name.
