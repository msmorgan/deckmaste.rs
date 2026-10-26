---
needs: []
---
# A shuffle keeps last known information for characteristic reads (*Zoyowa's Justice*)

Standard constraints apply. Decided by the owner in the 2026-10-25 triage follow-up (Q8); the
change was prototyped on a scratch copy and `lake build --wfail` was green. Paths are under
`lean/Semantics/` unless they start with `lean/` or `docs/`; find each site by name.

## The defect

*Zoyowa's Justice* ({1}{R} Instant), Oracle text from `data/derived/cards.jsonl`:

> The owner of target artifact or creature with mana value 1 or greater shuffles it into their
> library. Then that player discovers X, where X is its mana value. (They exile cards from the top
> of their library until they exile a nonland card with that mana value or less. They cast it
> without paying its mana cost or put it into their hand. They put the rest on the bottom in a
> random order.)

The shuffle randomizes the card [CR#701.24a], so no later "it" names it as a card in the library
(`afterShuffle`, `Check/Words.lean:1615`, removes the shuffled referents). But "its mana value"
reads the object by its last known information, because the effect moved it from a public zone to
a hidden one [CR#608.2h]. Today the wipe also removes the referent for a characteristic read, so
the card is refused. The new pin below fails today and passes after.

## The change (`Check/Words.lean` unless stated)

1. New `DRS` field, after `readsLastKnown` (`:530`), before `handedOrigin`:
   ```lean
   /-- Objects a shuffle randomized after a move took them out of a public zone, kept as they
   were last known: a characteristic read finds one where no current referent fits ("shuffles it
   into their library. Then … where X is its mana value") [CR#608.2h,701.24a]. No other read
   reaches them. -/
   shuffledLastKnown : List Referent := []
   ```
2. Before `countReach` (`:1428`), add:
   ```lean
   /-- The objects a shuffle randomized (`DRS.shuffledLastKnown`) that a characteristic read of
   `r` finds by their last known information [CR#608.2h,701.24a]; none outside a characteristic
   read. -/
   def shuffledLastKnownCandidates (r : Reach) (pl : Plurality) (drs : DRS) : List Referent :=
     if drs.characteristicRead then drs.shuffledLastKnown.filter (reachesCharacteristic r pl)
     else []

   /-- The antecedents a pronoun resolves to: `resolveCandidates`, or, where none is found, the
   objects a shuffle randomized that a characteristic read finds (`shuffledLastKnownCandidates`). -/
   def resolvePronoun (r : Reach) (pl : Plurality) (drs : DRS) (inArgSlot : Bool := false) :
       List Referent :=
     match resolveCandidates r pl drs inArgSlot with
     | [] => shuffledLastKnownCandidates r pl drs
     | live => live
   ```
   `resolveCandidates` itself must not change (changing it broke the simp proofs).
3. Before `afterShuffle` (`:1615`), add `leftPublicZone`, and rewrite `afterShuffle` to keep the
   public-origin referents:
   ```lean
   /-- Whether a referent a shuffle randomizes was moved there out of a public zone, so that its
   last known information stands [CR#608.2h]. -/
   def leftPublicZone (b : Referent) : Bool :=
     match b.payload with
     | .object _ _ (some st) _ _ =>
       st.zoneChange.any fun zc => zc.origin.any fun z =>
         z == .battlefield || z == .graveyard || z == .exile || z == .stack
     | _ => false

   def afterShuffle (drs : DRS) : DRS :=
     let refs := drs.referents.filter survivesShuffle
     let keptIds := refs.map (·.id)
     let lastKnown := (drs.referents.filter fun b => !survivesShuffle b && leftPublicZone b).map
       fun b => { b with idents := [] }
     { drs with
       referents := refs
       shuffledLastKnown := drs.shuffledLastKnown ++ lastKnown
       pendingRoles := drs.pendingRoles.filter (fun p => keptIds.elem p.1)
       resolvedIds := drs.resolvedIds.filter keptIds.elem }
   ```
   The command zone is not added to `leftPublicZone`.
4. `Check/Phrase.lean:3066`, the `.pro reach plur` arm only: `let winners := resolveCandidates
   reach plur drs inArgSlot` becomes `let winners := resolvePronoun reach plur drs inArgSlot`. No
   other `resolveCandidates` call site changes.

## Proof adaptations

These proofs unfold the `.pro` arm and must follow it (each is in the prototype):

- `Proofs/ComparisonBox.lean:115` (`proStep`): `let winners := resolveCandidates .bare .one drs`
  becomes `resolvePronoun .bare .one drs`; `:139`: `rw [powerErrors, proStep, winners]` becomes
  `rw [powerErrors, proStep, resolvePronoun, winners]`.
- `Proofs/ReferenceScopes.lean:91`: `change (let winners := resolveCandidates …` becomes
  `resolvePronoun`; `:95`: `rw [ownStatIdentityStableForEveryOuterContext …]` becomes
  `rw [resolvePronoun, ownStatIdentityStableForEveryOuterContext …]`.
- `Proofs/ReferenceScopes.lean:390` and `:394`: the same two edits for
  `ownStatActionIdentityStableForEveryOuterContext`.

## Added pin: *Zoyowa's Justice*, spelled

In `Cards/Anaphora.lean`, before the Tuktuk Scrapper docstring (`:842`):
```lean
/-- Zoyowa's Justice: "The owner of target artifact or creature with mana value 1 or greater
shuffles it into their library. Then that player discovers X, where X is its mana value." The
shuffle randomizes the card [CR#701.24a], so no later "it" names it as a card in the library, but
"its mana value" reads the permanent by its last known information, as it was moved out of a
public zone into a hidden one [CR#608.2h] (`DRS.shuffledLastKnown`). -/
def zoyowasJusticeBody : Action :=
  sequentially
    [ instruct (ownerOf (target (and [or [isArtifact, isCreature], hasManaValue .atLeast 1])))
        (shuffleInto it),
      instruct (that player) (discover (statOf (.stat .manaValue) it)) ]
def zoyowasJustice : Spelled := spelled <| singleFacedCard
  { name := "Zoyowa's Justice", cost := [generic 1, pip .red], types := [.instant],
    text := [spell zoyowasJusticeBody] }
theorem okZoyowasJustice : zoyowasJustice.card.check = [] := zoyowasJustice.ok
```
(`discover` is `Definitions/Actions.lean:345`.) Add nothing else: the two unprinted refusal pins
the prototype carried (`shuffledCardIsNoActionAntecedent`,
`shuffledLibraryCardHasNoLastKnownCharacteristic`) are **not** wanted; no printed card needs them.

## Docs

`lean/CONTRACTS.md`, after the shuffle sentence ending at `:330` ("... `exiledPileCanBeShuffled`).
A cast may leave its spell unspecified, which introduces no spell reference."): add a sentence
documenting that a shuffle removes the shuffled referents from scope, except that the ones a move
took out of the battlefield, a graveyard, exile or the stack are kept in `DRS.shuffledLastKnown`
and are found by a characteristic read ("its mana value") by their last known information, as
*Zoyowa's Justice* needs [CR#608.2h,701.24a]; no other read reaches them.

## Must stay green, unchanged

The U1 shuffle pins (`Proofs/Anaphora.lean`: `okShuffledYourLibraryThisWay`,
`badShuffledYourGraveyardThisWay`, `okShuffledPlain`, `badReadsShuffledLibraryCard`;
`Proofs/Description.lean` `badReadsShuffledIntoLibraryCard`), the forgotten-operand pins
(`Proofs/ActionFamilies.lean`: `forgottenOperandDoesNotRedirectToNeighbor`,
`forgottenAliasesAreBothRefused`, `forgottenNestedAliasIsRefused`,
`conditionalShuffleForgetsPrivateOperand`; `Proofs/CallerScope.lean`:
`forgottenParameterDoesNotSurvivePrivately`, `callerExitDoesNotResurrectForgottenMentions`), the
Weeping Angel pins (`Proofs/DrtCards.lean:269-299`), `shuffleForgetsMaskedLibraryIdentity` and
`revealedSearchResultSurvivesShuffleWithSameIdentity` (`Proofs/MovementScopes.lean`), and the
`shuffleZone` pins (`Proofs/Lookback.lean`: `shuffledGraveyardNamesNoShuffle`,
`exiledPileCanBeShuffled`).

## Acceptance

- `./lean/scripts/build` (`--wfail`) is green, `okZoyowasJustice` included.
- `grep -rn 'shuffledLastKnown' lean/Semantics lean/CONTRACTS.md` shows the field, its two uses
  in `afterShuffle` and `shuffledLastKnownCandidates`, and the CONTRACTS sentence;
  `grep -rn 'resolvePronoun' lean/Semantics/Check/Phrase.lean` shows exactly the `:3066` call.
- `cargo xtask lean check`.
- The cite gates: `cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite check` 0
  stale, `jj --no-pager diff --git | cargo xtask cite audit --diff` read rule by rule.
