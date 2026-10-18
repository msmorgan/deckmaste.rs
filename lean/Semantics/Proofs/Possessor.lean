import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Possessor

An object binding records its possessor where the text fixes one: the player whose hand,
graveyard or library the card is in [CR#400.1,400.3], or the permanent's controller [CR#108.4], each
relative to the controller of the text ("you" [CR#109.5]). Two deeds read it, each in both forms
§7 of `docs/decisions/semantics-v2.md` requires (a conjunct on an indefinite patient and a demand
on a definite referent): a sacrifice's patient is a permanent its performer controls
[CR#701.21a], and a discard's patient is a card in its performer's own hand [CR#701.9a]. A
possessor known to be another player than the performer is refused `patientPossessor`; one the
text leaves unknown is not. Each pin is closed by `decide`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Possessor

/-! ## Sacrifice [CR#701.21a] -/

/-- "Sacrifice a creature you don't control." The indefinite's own conjunct names another
controller than the performer. -/
theorem badSacrificeCreatureYouDontControl :
    Action.check (DRS.ofList []) (sacrifice (a (and [isCreature, youDontControl]))) = [.patientPossessor] := by
  decide

/-- "Sacrifice a creature you control." -/
theorem okSacrificeCreatureYouControl :
    Action.check (DRS.ofList []) (sacrifice (a (and [isCreature, youControl]))) = [] := by
  decide

/-- "Target opponent sacrifices a creature you control.": the performer is the opponent. -/
theorem badOpponentSacrificesCreatureYouControl :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (sacrifice (a (and [isCreature, youControl])))) =
      [.patientPossessor] := by
  decide

/-- "Target opponent sacrifices a creature they control." -/
theorem okOpponentSacrificesCreatureTheyControl :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (sacrifice (a (.and [isCreature, actorControls])))) =
      [] := by
  decide

/-- "Choose a creature you don't control. Sacrifice that permanent.": the definite referent's
binding carries the possessor its choice recorded. -/
theorem badSacrificeChosenCreatureYouDontControl :
    Action.check (DRS.ofList [])
      (.sequentially [choose (a (and [isCreature, youDontControl])), sacrifice (that .permanent)]) =
      [.patientPossessor] := by
  decide

/-- "Choose a creature you control. Sacrifice that permanent." -/
theorem okSacrificeChosenCreatureYouControl :
    Action.check (DRS.ofList [])
      (.sequentially [choose (a (and [isCreature, youControl])), sacrifice (that .permanent)]) = [] := by
  decide

/-- "Its controller sacrifices it": a possessor the text leaves unknown is not refused. -/
theorem okControllerSacrificesTarget :
    Action.check (DRS.ofList []) (controllerSacrifices (target (and [isCreature, youDontControl]))) = [] := by
  decide

/-! ## Discard [CR#701.9a] -/

/-- "Choose a card in your hand. Target opponent discards that card.": the card is in the
controller's hand, not the discarder's. -/
theorem badOpponentDiscardsCardInYourHand :
    Action.check (DRS.ofList [])
      (.sequentially [choose (a (.in_ yourHand)), instruct (target (.isOpponentOf .you)) (discard (that .card))]) =
      [.patientPossessor] := by
  decide

/-- "Choose a card in your hand. Discard that card." -/
theorem okDiscardChosenCardInYourHand :
    Action.check (DRS.ofList []) (.sequentially [choose (a (.in_ yourHand)), discard (that .card)]) =
      [] := by
  decide

/-- "Target opponent chooses a card in their hand and discards that card." -/
theorem okOpponentDiscardsCardInTheirHand :
    Action.check (DRS.ofList [])
      (instruct (target (.isOpponentOf .you))
        (.sequentially [choose (a (.in_ (handOf .actor))), discard (that .card)])) = [] := by
  decide

/-- "Target opponent discards a card in your hand.": the indefinite's own conjunct. -/
theorem badOpponentDiscardsACardInYourHand :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (discard (a (.in_ yourHand)))) =
      [.patientPossessor] := by
  decide

/-- "Target opponent discards a card." names no hand: the deed's own [CR#701.9a]. -/
theorem okOpponentDiscardsACard :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (discard (a .isCard))) = [] := by
  decide

/-! ## What a binding records -/

/-- A card chosen from your hand records you as its possessor. -/
theorem chosenCardInYourHandIsYours :
    possessorOfReach (.word .card) .one
      (stepAction 64 .postEvent .obj (DRS.ofList []) (choose (a (.in_ yourHand)))).1 =
      some .you := by
  decide

/-- A move between a player's own zones keeps the possessor, the owner [CR#400.1,400.3]; a move onto
the battlefield does not [CR#110.2]. -/
theorem possessorThroughMoves :
    keptPossessor (some (.action "discard")) (some .hand) (some .graveyard) (some .you) =
        some .you ∧
      keptPossessor (some (.core .put)) (some .graveyard) (some .battlefield) (some .you) = none ∧
      keptPossessor (some (.core .gainControl)) (some .battlefield) (some .battlefield)
        (some .other) = none := by
  decide

theorem destroyedPermanentControllerReadUsesLastKnownInformation :
    let destroyed := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (destroy (target isArtifact))).1
    let (after, ids, errors) := stepNounPhrase 64 (nextClause destroyed) .obj true
      (controllerOf (that .permanent))
    errors = [] ∧ ids.length = 1 ∧
      (ids.filterMap after.findById?).all (fun r => r.kind == .player) ∧
      after.referents.any (fun r => r.id == 1 && r.zone == some .graveyard) ∧
      after.characteristicRead = false := by decide

theorem controllerReadDoesNotPermitUntappingDestroyedPermanent :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isArtifact),
        .dealDamage (.lit 2) (controllerOf (that .permanent)) .noncombatOnly,
        untap (that .permanent)]) = [.anaphor (.word .permanent) .one 0] := by decide

end Semantics.Proofs.Possessor
