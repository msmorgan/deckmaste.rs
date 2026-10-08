import Semantics.Macros
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

open Semantics Semantics.Macros

namespace Semantics.Proofs.Possessor

/-! ## Sacrifice [CR#701.21a] -/

/-- "Sacrifice a creature you don't control." The indefinite's own conjunct names another
controller than the performer. -/
theorem badSacrificeCreatureYouDontControl :
    Instruction.check [] (sacrifice (a creatureYouDontControl)) = [.patientPossessor] := by
  decide

/-- "Sacrifice a creature you control." -/
theorem okSacrificeCreatureYouControl :
    Instruction.check [] (sacrifice (a creatureYouControl)) = [] := by
  decide

/-- "Target opponent sacrifices a creature you control.": the performer is the opponent. -/
theorem badOpponentSacrificesCreatureYouControl :
    Instruction.check [] (act (target .opponent) (sacrifice (a creatureYouControl))) =
      [.patientPossessor] := by
  decide

/-- "Target opponent sacrifices a creature they control." -/
theorem okOpponentSacrificesCreatureTheyControl :
    Instruction.check [] (act (target .opponent) (sacrifice (a (.and [creature, actorControls])))) =
      [] := by
  decide

/-- "Choose a creature you don't control. Sacrifice that permanent.": the definite referent's
binding carries the possessor its choice recorded. -/
theorem badSacrificeChosenCreatureYouDontControl :
    Instruction.check []
      (.sequentially [choose (a creatureYouDontControl), sacrifice (that .permanent)]) =
      [.patientPossessor] := by
  decide

/-- "Choose a creature you control. Sacrifice that permanent." -/
theorem okSacrificeChosenCreatureYouControl :
    Instruction.check []
      (.sequentially [choose (a creatureYouControl), sacrifice (that .permanent)]) = [] := by
  decide

/-- "Its controller sacrifices it": a possessor the text leaves unknown is not refused. -/
theorem okControllerSacrificesTarget :
    Instruction.check [] (controllerSacrifices (target creatureYouDontControl)) = [] := by
  decide

/-! ## Discard [CR#701.9a] -/

/-- "Choose a card in your hand. Target opponent discards that card.": the card is in the
controller's hand, not the discarder's. -/
theorem badOpponentDiscardsCardInYourHand :
    Instruction.check []
      (.sequentially [choose (a (.inZone yourHand)), act (target .opponent) (discard (that .card))]) =
      [.patientPossessor] := by
  decide

/-- "Choose a card in your hand. Discard that card." -/
theorem okDiscardChosenCardInYourHand :
    Instruction.check [] (.sequentially [choose (a (.inZone yourHand)), discard (that .card)]) =
      [] := by
  decide

/-- "Target opponent chooses a card in their hand and discards that card." -/
theorem okOpponentDiscardsCardInTheirHand :
    Instruction.check []
      (act (target .opponent)
        (.sequentially [choose (a (.inZone (handOf .actor))), discard (that .card)])) = [] := by
  decide

/-- "Target opponent discards a card in your hand.": the indefinite's own conjunct. -/
theorem badOpponentDiscardsACardInYourHand :
    Instruction.check [] (act (target .opponent) (discard (a (.inZone yourHand)))) =
      [.patientPossessor] := by
  decide

/-- "Target opponent discards a card." names no hand: the deed's own [CR#701.9a]. -/
theorem okOpponentDiscardsACard :
    Instruction.check [] (act (target .opponent) (discard (a .isCard))) = [] := by
  decide

/-! ## What a binding records -/

/-- A card chosen from your hand records you as its possessor. -/
theorem chosenCardInYourHandIsYours :
    NounPhrase.possessor (Instruction.intro [] (choose (a (.inZone yourHand)))) (that .card) =
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

end Semantics.Proofs.Possessor
