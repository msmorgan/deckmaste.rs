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

**This is an inventory to confirm, not a ruling.** It was taken read-only against the current
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
  `:1954`). It is pinned by `additionalCostDoesNotPublishAHiddenGroup`
  (`Check/AbilityRules.lean:53`).

## Unsure

- **U1. Shuffling forgets library referents** (`survivesShuffle`, `Check/Words.lean:1128-1133`;
  `afterShuffle`, `:1458-1464`; called at `Check/Phrase.lean:3242`). After a shuffle, every
  object referent in a library is dropped from discourse unless a library search found it. A
  shuffle only randomizes order [CR#701.24a] and moves nothing, so the object keeps its identity
  [CR#400.7]. Dropping it is hidden-zone tracking, which points to runtime. The other view is
  that "that card" after an unrelated shuffle has no antecedent Oracle writes, and that
  [CR#701.24b] singles out found cards. Pins that depend on it: `shuffleForgetsMaskedLibraryIdentity`,
  `revealedSearchResultSurvivesShuffleWithSameIdentity` (`Proofs/MovementScopes.lean:79,178`),
  `conditionalShuffleForgetsPrivateOperand` (`Proofs/ActionFamilies.lean:275`),
  `badReadsShuffledIntoLibraryCard` (`Proofs/Description.lean:330`) and
  `badReadsShuffledLibraryCard` (`Proofs/Anaphora.lean:660`).
- **U2. Characteristic reads through "card" and "spell" read only the current zone**
  (`stampedWordOk`, `Check/Words.lean:1104-1113`, via `reachesCharacteristic`, `:1145-1150`). A
  Card Type or "permanent" word may read a departed permanent by its last known information, but
  "spell" requires the stack now. By reading, "that spell's mana value" after the spell is
  countered (*Mana Drain*) finds no referent; confirm with a probe. The carrier word's zone is
  text meaning [CR#109.2b], but whether the countered spell's information is still available is
  last-known-information availability [CR#608.2h]. The owner decides which applies to a
  characteristic read.

## Keep (text-decidable)

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

Tally: 3 remove (all owned by the ninjutsu ticket), 2 unsure, 7 keep.

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

1. The owner confirms or reclasses U1, U2 and the keep rows.
2. Remove each row confirmed as runtime, rewriting or retiring its pins by name, and mark
   superseded in `docs/decisions/semantics-v3.md` and `lean/CONTRACTS.md` the text that
   describes it.
3. Move the out-of-scope pins named in the triage to engine tickets, and leave the denotation
   work in the wave-C tickets.

## Verification

`./lean/scripts/build`; `cargo xtask lean check`; the CR citation gates.
