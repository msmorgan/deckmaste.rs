import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Mana

Port of `idris/src/Experimental/Proofs/Mana.idr`: the pins of the Mana family, in theorem form,
each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Mana

private def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

/-- An activated ability with no limit, guard, or activator. -/
def activatedAbility (cost : Cost) (action : Action) (timing : Option Timing := none) : Ability :=
  .activated cost action timing none none none

/-- "your hand" -/
theorem okOwnedHand : Location.check (DRS.ofList []) (.zone .hand (some .you)) = [] := by decide

/-- "Discard a card at random. This deals damage equal to the discarded card's mana value to any
target." -/
theorem okDiscardedCardWord :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (discard (aAtRandom (.and [.isCard, .in_ yourHand]))))
        (Action.instruct .this (.dealDamage (.statOf (.stat .manaValue) (theVerbed (.action "discard") .one (some .card))) (target isAnyTarget) .any))) = [] := by
  decide

theorem badDiscardedCreatureWord :
    Ability.check (DRS.ofList [])
      (activatedAbility (.perform (discard (aAtRandom (.and [isCreature, .in_ yourHand]))))
        (Action.instruct .this (.dealDamage (.statOf (.stat .manaValue)
            (theVerbed (.action "discard") .one (some (.type .creature)))) (target isAnyTarget) .any)))
      = [.anaphor (.verbed (.action "discard") (some (.type .creature)) .attributive) .one 0] := by
  decide

/-- "{1}, {T}: Draw a card." -/
theorem okSingleTapCost :
    Ability.check (DRS.ofList []) (activatedAbility (.compound [.mana [generic 1], .tapSymbol]) (draw (.lit 1)))
      = [] := by
  decide

/-- "{1}: Draw a card." -/
theorem okNonEmptyManaCost :
    Ability.check (DRS.ofList []) (activatedAbility (.mana [generic 1]) (draw (.lit 1))) = [] := by decide

/-- ": Draw a card.": an empty mana run is no cost at all, refused as an empty list is. -/
theorem badEmptyManaCost :
    Ability.check (DRS.ofList []) (activatedAbility (.mana []) (draw (.lit 1))) = [.nonEmpty] := by decide

/-- "Exile a creature you control. If you do, return it to the battlefield." -/
theorem okIfDoneReadsDoneBody :
    Action.check (DRS.ofList [])
      (doIfDone (exile (a (and [isCreature, youControl]))) (some (putInto it battlefield))) = [] := by
  decide

/-- "Sacrifice a creature. If you don't, exile it." -/
theorem badIfNotReadsMandatoryBody :
    Action.check (DRS.ofList []) (doIfDone (sacrifice (a isCreature)) none (some (exile it)))
      = [.anaphor .bare .one 0] := by
  decide

/-- "Counter target spell unless its controller pays {3}." -/
theorem okUnlessManaCost :
    Action.check (DRS.ofList [])
      (Templates.instruct (controllerOf (target isSpell)) (doUnless (counter it) (.mana [generic 3]))) =
          [] := by
  decide

/-- "Activate only before the combat damage step." -/
theorem okBeforeCombatDamage :
    Ability.check (DRS.ofList [])
      (activatedAbility (.mana [generic 2]) (draw (.lit 1)) (some (beforePart .combatDamage
          none)))
      = [] := by
  decide

/-- "Activate only before each player's attackers are declared." -/
theorem okDistributiveAttackWindow :
    Ability.check (DRS.ofList [])
      (activatedAbility (.mana [generic 2]) (draw (.lit 1))
        (some (beforePart .declareAttackers (some (each .anyPlayer))))) = [] := by
  decide

/-- "Cast this spell only before the combat damage step." -/
theorem okCastBeforeCombatDamage :
    Ability.check (DRS.ofList []) (.spell (some (beforePart .combatDamage none)) (draw (.lit 1)))
      = [] := by
  decide

/-- "Ward {2}" -/
theorem okWardCost : Ability.check (DRS.ofList []) (keywordCosting (.named "ward") (.mana [generic 2])) = [] := by
  decide

/-- "Ward" -/
theorem badBareWardLine :
    Ability.check (DRS.ofList []) (.keyword (.named "ward") [] []) = [.keywordParamFits (.named "ward")] := by decide

/-- "Ward red" -/
theorem badWardQuality :
    Ability.check (DRS.ofList []) (.keyword (.named "ward") [.quality (.colorIs .red)] [])
      = [.keywordParamFits (.named "ward")] := by
  decide

/-- "Flying {2}" -/
theorem badParamOnNullaryKeyword :
    Ability.check (DRS.ofList []) (keywordCosting (.named "flying") (.mana [generic 2]))
      = [.keywordParamFits (.named "flying")] := by
  decide

/-- "Flyign" -/
theorem badUnknownKeywordLabel :
    Ability.check (DRS.ofList []) (.keyword (.named "flyign") [] []) = [.keywordParamFits (.named "flyign")] := by decide

/-- "Add {R}." -/
theorem okSingleProduction :
    Action.check (DRS.ofList []) (.addMana (.lit 1) (.runs [[.of .red]]) []) = [] := by
        decide

/-- "Add." -/
theorem badEmptyProduction :
    Action.check (DRS.ofList []) (.addMana (.lit 1) (.runs []) []) = [.producedRuns] := by
        decide

/-- "Add {R} or ." -/
theorem badEmptyAlternative :
    Action.check (DRS.ofList []) (.addMana (.lit 1) (.runs [[.of .red], []]) [])
      = [.producedRuns] := by
  decide

/-- "Add {C}. Spend this mana only to activate abilities." -/
theorem okSpendPurpose :
    Action.check (DRS.ofList [])
      (.addMana (.lit 1) (.runs [[.colorless]]) [.spendOnly [.toActivate none]]) =
          [] := by
  decide

/-- "Add {C}. Spend this mana only." -/
theorem badPurposelessSpend :
    Action.check (DRS.ofList []) (.addMana (.lit 1) (.runs [[.colorless]]) [.spendOnly []])
      = [.nonEmpty] := by
  decide

/-- An enchantment with the given text. -/
def enchantmentWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.enchantment], text } }

/-- "{U}: Counter target spell with the chosen name." -/
def counterChosenName : Ability :=
  activatedAbility (.mana [pip .blue]) (counter (target (.and [isSpell, .named .chosen])))

/-- "… Counter target spell with the chosen name." -/
theorem okNameMatchAfterChooser :
    Card.check
      (enchantmentWith
        [ .static (entersChoosing thisEnchantment .cardName),
          counterChosenName ]) = [] := by
  decide

theorem badNameMatchBeforeChooser :
    Card.check
      (enchantmentWith
        [ counterChosenName,
          .static (entersChoosing thisEnchantment .cardName) ])
      = [.choiceRef .theChoice (.quality .cardName) 0] := by
  decide

theorem badNameMatchWrongSort :
    Card.check
      (enchantmentWith
        [ .static (entersChoosing thisEnchantment .color),
          counterChosenName ]) = [.choiceRef .theChoice (.quality .cardName) 0] := by
  decide

/-- "… rather than pay this spell's mana cost, pay {2}." -/
theorem okPlayPaymentMana : PlayPayment.check (DRS.ofList []) (.payingInstead (.mana [generic 2])) = [] := by
  decide

/-- "As an additional cost to cast this spell, sacrifice an artifact." -/
theorem okAddedCostSacrifice :
    StaticSpec.check (DRS.ofList []) (.addedCost (.perform (sacrifice (a isArtifact))) false .once) = []
        := by
  decide

/-- "Creatures you control get +1/+1 until end of turn." -/
theorem okContinuousPumpClause :
    Action.check (DRS.ofList [])
      (.establish (getsPt (allOf (and [isCreature, youControl])) (.up (.lit 1)) (.up (.lit 1)))
        (some untilEndOfTurn)) = [] := by
  decide

/-- A land with the given text. -/
def landWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.land], text } }

/-- "{T}: Add one mana of the chosen color." -/
def tapForChosenColor : Ability := activatedAbility .tapSymbol (.addMana (.lit 1) (.ofChosenColor none) [])

/-- "…choose a color. {T}: Add one mana of the chosen color." -/
theorem okChosenColorAfterChooser :
    Card.check
      (landWith
        [.static (entersChoosing thisLand .color), tapForChosenColor])
      = [] := by
  decide

/-- "{T}: Add one mana of the chosen color." -/
theorem badChosenColorNoChooser :
    Card.check (landWith [tapForChosenColor]) = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

/-- "Cumulative upkeep {2}" -/
theorem okCostedCumulativeUpkeep :
    Ability.check (DRS.ofList []) (keywordCosting (.named "cumulativeUpkeep") (.mana [generic 2])) = [] := by decide

/-- "Cumulative upkeep" -/
theorem badBareCumulativeUpkeep :
    Ability.check (DRS.ofList []) (.keyword (.named "cumulativeUpkeep") [] [])
      = [.keywordParamFits (.named "cumulativeUpkeep")] := by
  decide

/-- "… pays {1} for each artifact you control." -/
theorem okForEachScaledMana :
    Cost.check (DRS.ofList [])
      (Primitives.Cost.scaled (.mana [generic 1]) (forEach (.lit 1) (.and [isArtifact, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "Counter target spell unless its controller pays {2}." -/
theorem badLiteralScaledMana :
    Cost.check (DRS.ofList []) (Primitives.Cost.scaled (.mana [generic 1]) (.lit 2)) = [.forEachAmount] := by decide

theorem badBareCountScaledMana :
    Cost.check (DRS.ofList []) (Primitives.Cost.scaled (.mana [generic 1]) (countOf (.and [isArtifact, .hasPossessor .controller .you])))
      = [.forEachAmount] := by
  decide

/-- "Creature cards in your graveyard have unearth {2}." -/
theorem okUnearthGrantInGraveyard :
    Ability.check (DRS.ofList [])
      (.static
        (Templates.abilityGrant (allOf (.and [isCreature, .in_ (graveyardOf .you)]))
          (keywordCosting (.named "unearth") (.mana [generic 2])))) = [] := by
  decide

/-- "permanents you control" -/
def permanentsYouControl : NounPhrase := allOf (.and [isPermanent, .hasPossessor .controller .you])

/-- "For each color among permanents you control, add one mana of that color." -/
theorem okChosenColorPerColor :
    Action.check (DRS.ofList [])
      (.doForEachKind .color (some permanentsYouControl) .color
        (.addMana (.lit 1) (.ofChosenColor none) [])) = [] := by
  decide

/-- "For each color among permanents you control, add one mana of that color" -/
theorem badRepeatedCarriesNoColor :
    Action.check (DRS.ofList [])
      (repeatTimes (.distinctCount .color permanentsYouControl)
        (.addMana (.lit 1) (.ofChosenColor none) []))
      = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

/-- "For each color among permanents you control, … of that creature type." -/
theorem badAxisValueCrossing :
    Action.check (DRS.ofList [])
      (.doForEachKind .color (some permanentsYouControl) (.subtype .creature) (draw (.lit 1)))
      = [.kindAxisSort] := by
  decide

/-- "For each creature type, …" -/
theorem badDomainlessOpenAxis :
    Action.check (DRS.ofList [])
      (.doForEachKind (.subtype .creature .any) none (.subtype .creature) (draw (.lit 1)))
      = [.kindDomainOk] := by
  decide

/-- "Cumulative upkeep {2}". The Idris reference also wrote out the keyword's expansion; here the
keyword line carries the law directly. -/
theorem okManaCumulativeUpkeep :
    Ability.check (DRS.ofList []) (keywordCosting (.named "cumulativeUpkeep") (.mana [generic 2])) = [] := by decide

/-- "Whenever a player taps a land for mana, …" -/
theorem okTappedForMana :
    GameEvent.check (DRS.ofList []) (.does (.by (a .anyPlayer)) (.tapForMana (a isLand) none)) = [] := by decide

/-- An artifact with the given text. -/
def artifactWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.artifact], text } }

/-- "Whenever a basic land is tapped for mana of the chosen color, draw a card." -/
def drawOnTapForChosenColor : Ability :=
  whenever
    (.does .any (.tapForMana (a (.and [isLand, .hasSupertype .basic])) (some (.ofColor thatColor))))
    (draw (.lit 1))

/-- "As this artifact enters, choose a color. Whenever a basic land is tapped for mana of the
chosen color, draw a card." -/
theorem okTapForChosenColorMana :
    Card.check
      (artifactWith [.static (entersChoosing thisArtifact .color), drawOnTapForChosenColor])
      = [] := by
  decide

/-- "As this artifact enters, choose a creature type. Whenever a basic land is tapped for mana
of the chosen type, draw a card." The six mana types are the five colors and colorless
[CR#106.1b], so `ofColor` reads a chosen COLOR alone; "mana of the chosen color" is
`okTapForChosenColorMana`. -/
theorem badTapForChosenNonManaType :
    Card.check
      (artifactWith
        [.static (entersChoosing thisArtifact (.subtype .creature)), drawOnTapForChosenColor])
      = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

def afterALandTapForMana : DRS :=
  GameEvent.extend (DRS.ofList []) (.does (.by (a .anyPlayer)) (.tapForMana (a isLand) none))

def afterAPlainLandTap : DRS :=
  GameEvent.extend (DRS.ofList []) (.does (.actor (a .anyPlayer)) (.setStatus .tapped (a isLand)))

/-- "one mana of any type that land produced" -/
theorem okProducedByTapEvent :
    ProducedMana.check afterALandTapForMana (.producedByEvent (that (.type .land))) = [] := by
  decide

/-- "Whenever a player taps a land, add one mana of any type that land produced": a tap that is
not a mana ability resolving produces no mana [CR#106.12a]. -/
theorem badProducedByPlainTap :
    ProducedMana.check afterAPlainLandTap (.producedByEvent (that (.type .land)))
      = [.outcomeInScope .manaProduced 0] := by
  decide

/-- "Add one mana of any type that land produced" -/
theorem badProducedByEventWithoutEvent :
    ProducedMana.check (DRS.ofList []) (.producedByEvent (a isLand)) = [.outcomeInScope .manaProduced 0] := by
  decide

def afterManaAdded : DRS :=
  actionResult (DRS.ofList []) (.addMana (.lit 1) (.runs [[.colorless]]) [])

/-- "You don't lose this mana as steps and phases end." -/
theorem okThisManaAfterAdd :
    StaticSpec.check afterManaAdded (.manaRetention .you .thisMana) = [] := by decide

/-- "You don't lose this mana as steps and phases end" -/
theorem badThisManaWithoutAdd :
    StaticSpec.check (DRS.ofList []) (.manaRetention .you .thisMana)
      = [.outcomeInScope .manaAdded 0] := by
  decide

/-- "Target player sacrifices a creature of their choice." -/
theorem okBoundTheirChoice :
    Action.check (DRS.ofList []) (Templates.instruct (target .anyPlayer) (sacrifice (aTheirChoice isCreature))) = [] :=
        by decide

/-- "Destroy a creature of their choice." -/
theorem badUnboundTheirChoice :
    Action.check (DRS.ofList []) (destroy (aTheirChoice isCreature)) = [.chooserInScope 0] := by decide

/-- "Target creature becomes a black Zombie in addition to its other types." -/
theorem okUnnamedAddition :
    Action.check (DRS.ofList [])
      (become (target isCreature)
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none)
      = [] := by
  decide

theorem badRepeatedAdditionColor :
    Action.check (DRS.ofList [])
      (become (target isCreature)
        { characteristics :=
          { colors := [.black, .black], subtypes := [creatureType "Zombie"] } } none)
      = [.distinct] := by
  decide

theorem badRepeatedAdditionType :
    Action.check (DRS.ofList []) (become (target isCreature) { characteristics :=
                                                      { types := [.artifact, .artifact] } } none)
      = [.distinct] := by
  decide

end Semantics.Proofs.Mana
