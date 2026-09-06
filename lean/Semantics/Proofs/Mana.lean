import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Mana

Port of `idris/src/Experimental/Proofs/Mana.idr`: the pins of the Mana family, in theorem form,
each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Mana

/-- An activated ability with no limit, guard, or activator. -/
def act (cost : Cost) (instruction : Instruction) (timing : Option Timing := none) : Ability :=
  .activated cost instruction timing none none none

/-- "your hand" -/
theorem okOwnedHand : ZoneExpr.check [] (.zone .hand (.possessedBy .you)) = [] := by decide

/-- "your battlefield" -/
theorem badOwnedBattlefield :
    ZoneExpr.check [] (.zone .battlefield (.possessedBy .you)) = [.possessable .battlefield] := by
  decide

/-- "Discard a card at random. This deals damage equal to the discarded card's mana value to any
target." -/
theorem okDiscardedCardWord :
    Ability.check []
      (act (.perform (discard (aAtRandom (.inZone hand)) (agent := .you)))
        (.dealDamage .this
          (.statOf (.stat .manaValue) (theVerbed (.action "Discard") .card .attributive .one))
          (target anyTarget))) = [] := by
  decide

theorem badDiscardedCreatureWord :
    Ability.check []
      (act (.perform (discard (aAtRandom (.and [creature, .inZone hand])) (agent := .you)))
        (.dealDamage .this
          (.statOf (.stat .manaValue)
            (theVerbed (.action "Discard") (.type .creature) .attributive .one))
          (target anyTarget)))
      = [.anaphor (.verbed (.action "Discard") (.type .creature) .attributive) .one 0] := by
  decide

/-- "{1}, {T}, {T}: Draw a card." written with the second {T} a nesting level down. -/
theorem nestedCompoundCost :
    Ability.check []
      (act (.compound [.compound [.mana [generic 1], .tapSymbol], .tapSymbol]) (draw (.lit 1) (agent
          := .you)))
      = [.costTapOnce] := by
  decide

/-- "{1}, {T}: Draw a card." -/
theorem okSingleTapCost :
    Ability.check [] (act (.compound [.mana [generic 1], .tapSymbol]) (draw (.lit 1) (agent :=
        .you)))
      = [] := by
  decide

/-- "{T}, {T}: Draw a card." -/
theorem badDoubleTapCost :
    Ability.check [] (act (.compound [.tapSymbol, .tapSymbol]) (draw (.lit 1) (agent := .you)))
      = [.costTapOnce] := by
  decide

/-- "{1}: Draw a card." -/
theorem okNonEmptyManaCost :
    Ability.check [] (act (.mana [generic 1]) (draw (.lit 1) (agent := .you))) = [] := by decide

/-- ": Draw a card." -/
theorem badEmptyManaCost :
    Ability.check [] (act (.mana []) (draw (.lit 1) (agent := .you))) = [.manaRun] := by decide

/-- "{W/U/P}" -/
theorem okDistinctPhyrexian : ManaSymbol.check (.phyrexian .white (some .blue)) = [] := by decide

/-- "{W/W/P}" -/
theorem badSameColorPhyrexian :
    ManaSymbol.check (.phyrexian .white (some .white)) = [.manaSymbolOk] := by decide

/-- "{U/B}" -/
theorem okDistinctHybrid : ManaSymbol.check (hybridPip .blue .black) = [] := by decide

/-- "{U/U}" -/
theorem badSameColorHybrid : ManaSymbol.check (hybridPip .blue .blue) = [.manaSymbolOk] := by
  decide

/-- "Exile a creature you control. If you do, return it to the battlefield." -/
theorem okIfDoneReadsDoneBody :
    Instruction.check []
      (Primitives.Instruction.doIfDone (exile (a creatureYouControl)) (some (move it battlefield)) none) = [] := by
  decide

/-- "Sacrifice a creature. If you don't, exile it." -/
theorem badIfNotReadsMandatoryBody :
    Instruction.check [] (Primitives.Instruction.doIfDone (sacrifice (a creature) (agent := .you)) none (some (exile it)))
      = [.anaphor .bare .one 0] := by
  decide

/-- "Counter target spell unless its controller pays {3}." -/
theorem okUnlessManaCost :
    Instruction.check []
      (doUnless (Primitives.Instruction.counterSpell it) (.mana [generic 3]) (agent := (controllerOf (target spell)))) =
          [] := by
  decide

/-- "Counter target spell unless its controller taps." -/
theorem badUnlessTapSymbol :
    Instruction.check [] (doUnless (Primitives.Instruction.counterSpell it) .tapSymbol (agent := (controllerOf (target
        spell))))
      = [.payable] := by
  decide

/-- "Activate only before the combat damage step." -/
theorem okBeforeCombatDamage :
    Ability.check []
      (act (.mana [generic 2]) (draw (.lit 1) (agent := .you)) (some (.beforePart .combatDamage
          none)))
      = [] := by
  decide

/-- "Activate only before the turn." A turn is made of its phases [CR#500.1], so a point before
the turn one is already taking is not a window inside it; a step or phase is
(`okBeforeCombatDamage`). -/
theorem badBeforeTheTurn :
    Ability.check [] (act (.mana [generic 2]) (draw (.lit 1) (agent := .you)) (some (.beforePart
        .turn none)))
      = [.windowOk] := by
  decide

/-- "Activate only before each player's attackers are declared." -/
theorem okDistributiveAttackWindow :
    Ability.check []
      (act (.mana [generic 2]) (draw (.lit 1) (agent := .you))
        (some (.beforePart .declareAttackers (some (each .anyPlayer))))) = [] := by
  decide

/-- "Activate only before all players' attackers are declared." A turn part has one active
player [CR#102.1]. -/
theorem badPluralAttackWindow :
    Ability.check []
      (act (.mana [generic 2]) (draw (.lit 1) (agent := .you))
        (some (.beforePart .declareAttackers (some (allOf .anyPlayer))))) = [.pointWindowOk] := by
  decide

/-- "Cast this spell only before the combat damage step." -/
theorem okCastBeforeCombatDamage :
    Ability.check [] (.spell (some (.beforePart .combatDamage none)) (draw (.lit 1) (agent :=
        .you)))
      = [] := by
  decide

/-- "Cast this spell only before the turn." A turn is made of its phases [CR#500.1], so a point
before the turn the spell would be cast in is not a window inside it
(`okCastBeforeCombatDamage`). -/
theorem badCastBeforeTheTurn :
    Ability.check [] (.spell (some (.beforePart .turn none)) (draw (.lit 1) (agent := .you)))
      = [.windowOk] := by
  decide

/-- "Ward {2}" -/
theorem okWardCost : Ability.check [] (keywordCosting "Ward" (.mana [generic 2])) = [] := by
  decide

/-- "Ward" -/
theorem badBareWardLine :
    Ability.check [] (.keyword "Ward" [] none) = [.keywordParamFits "Ward"] := by decide

/-- "Ward red" -/
theorem badWardQuality :
    Ability.check [] (.keyword "Ward" [.quality (.colorIs .red)] none)
      = [.keywordParamFits "Ward"] := by
  decide

/-- "Flying {2}" -/
theorem badParamOnNullaryKeyword :
    Ability.check [] (keywordCosting "Flying" (.mana [generic 2]))
      = [.keywordParamFits "Flying"] := by
  decide

/-- "Flyign" -/
theorem badUnknownKeywordLabel :
    Ability.check [] (.keyword "Flyign" [] none) = [.keywordParamFits "Flyign"] := by decide

/-- "Add {R}." -/
theorem okSingleProduction :
    Instruction.check [] (.addMana (.lit 1) (.runs [[.of .red]]) [] (agent := .you)) = [] := by
        decide

/-- "Add." -/
theorem badEmptyProduction :
    Instruction.check [] (.addMana (.lit 1) (.runs []) [] (agent := .you)) = [.producedRuns] := by
        decide

/-- "Add {R} or ." -/
theorem badEmptyAlternative :
    Instruction.check [] (.addMana (.lit 1) (.runs [[.of .red], []]) [] (agent := .you))
      = [.producedRuns] := by
  decide

/-- "Add {C}. Spend this mana only to activate abilities." -/
theorem okSpendPurpose :
    Instruction.check []
      (.addMana (.lit 1) (.runs [[.colorless]]) [.spendOnly [.toActivate none]] (agent := .you)) =
          [] := by
  decide

/-- "Add {C}. Spend this mana only." -/
theorem badPurposelessSpend :
    Instruction.check [] (.addMana (.lit 1) (.runs [[.colorless]]) [.spendOnly []] (agent := .you))
      = [.nonEmpty] := by
  decide

/-- An enchantment with the given text. -/
def enchantmentWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.enchantment], text } }

/-- "{U}: Counter target spell with the chosen name." -/
def counterChosenName : Ability :=
  act (.mana [pip .blue]) (Primitives.Instruction.counterSpell (target (.and [spell, .named .chosen])))

/-- "… Counter target spell with the chosen name." -/
theorem okNameMatchAfterChooser :
    Card.check
      (enchantmentWith
        [ .static (Primitives.StaticSpec.entryChoice thisEnchantment (.quality .cardName) none .openly),
          counterChosenName ]) = [] := by
  decide

theorem badNameMatchBeforeChooser :
    Card.check
      (enchantmentWith
        [ counterChosenName,
          .static (Primitives.StaticSpec.entryChoice thisEnchantment (.quality .cardName) none .openly) ])
      = [.choiceRef .theChoice (.quality .cardName) 0] := by
  decide

theorem badNameMatchWrongSort :
    Card.check
      (enchantmentWith
        [ .static (Primitives.StaticSpec.entryChoice thisEnchantment (.quality .color) none .openly),
          counterChosenName ]) = [.choiceRef .theChoice (.quality .cardName) 0] := by
  decide

/-- "… rather than pay this spell's mana cost, pay {2}." -/
theorem okPlayPaymentMana : PlayPayment.check [] (.payingInstead (.mana [generic 2])) = [] := by
  decide

theorem badPlayPaymentTapSymbol :
    PlayPayment.check [] (.payingInstead .tapSymbol) = [.altPayment] := by decide

/-- "As an additional cost to cast this spell, sacrifice an artifact." -/
theorem okAddedCostSacrifice :
    StaticSpec.check [] (.addedCost (.perform (sacrifice (a artifact) (agent := .you))) false) = []
        := by
  decide

/-- "As an additional cost to cast this spell, {T}." -/
theorem badAddedCostTapSymbol :
    StaticSpec.check [] (.addedCost .tapSymbol false) = [.addedPayment] := by decide

/-- "As an additional cost to cast this spell, [+1]." -/
theorem badAddedCostLoyaltySymbol :
    StaticSpec.check [] (.addedCost (.loyaltySymbol (.up 1)) false) = [.addedPayment] := by
  decide

/-- "Creatures you control get +1/+1 until end of turn." -/
theorem okContinuousPumpClause :
    Instruction.check []
      (.establish (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))
        (some untilEndOfTurn)) = [] := by
  decide

/-- "You may sacrifice a Mountain rather than pay this spell's mana cost" -/
theorem badAltCostClause :
    Instruction.check []
      (.establish
        (.altCost .this
          (some (.perform (sacrifice (a (.and [land, .hasSubtype (landType "Mountain")])) (agent :=
              .you)))))
        none) = [.clauseStatic] := by
  decide

/-- A land with the given text. -/
def landWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.land], text } }

/-- "{T}: Add one mana of the chosen color." -/
def tapForChosenColor : Ability := act .tapSymbol (.addMana (.lit 1) (.ofChosenColor none) [] (agent
    := .you))

/-- "…choose a color. {T}: Add one mana of the chosen color." -/
theorem okChosenColorAfterChooser :
    Card.check
      (landWith
        [.static (Primitives.StaticSpec.entryChoice thisLand (.quality .color) none .openly), tapForChosenColor])
      = [] := by
  decide

/-- "{T}: Add one mana of the chosen color." -/
theorem badChosenColorNoChooser :
    Card.check (landWith [tapForChosenColor]) = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

/-- "Cumulative upkeep {2}" -/
theorem okCostedCumulativeUpkeep :
    Ability.check [] (keywordCosting "CumulativeUpkeep" (.mana [generic 2])) = [] := by decide

/-- "Cumulative upkeep" -/
theorem badBareCumulativeUpkeep :
    Ability.check [] (.keyword "CumulativeUpkeep" [] none)
      = [.keywordParamFits "CumulativeUpkeep"] := by
  decide

/-- "… pays {1} for each artifact you control." -/
theorem okForEachScaledMana :
    Cost.check []
      (scaledMana .generic (forEach 1 (.and [artifact, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "Counter target spell unless its controller pays {2}." -/
theorem badLiteralScaledMana :
    Cost.check [] (scaledMana .generic (.lit 2)) = [.forEachAmount] := by decide

theorem badBareCountScaledMana :
    Cost.check [] (scaledMana .generic (countOf (.and [artifact, .hasPossessor .controller .you])))
      = [.forEachAmount] := by
  decide

/-- "Creature cards in your graveyard have unearth {2}." -/
theorem okUnearthGrantInGraveyard :
    Ability.check []
      (.static
        (.abilityGrant (allOf (.and [creature, .inZone (graveyardOf .you)]))
          (keywordCosting "Unearth" (.mana [generic 2])))) = [] := by
  decide

/-- "Creature cards in your graveyard have warp {2}." -/
theorem badWarpGrantInGraveyard :
    Ability.check []
      (.static
        (.abilityGrant (allOf (.and [creature, .inZone (graveyardOf .you)]))
          (keywordCosting "Warp" (.mana [generic 2])))) = [.grantSubject] := by
  decide

/-- "permanents you control" -/
def permanentsYouControl : NounPhrase := allOf (.and [permanent, .hasPossessor .controller .you])

/-- "For each color among permanents you control, add one mana of that color." -/
theorem okChosenColorPerColor :
    Instruction.check []
      (.doForEachKind .color (some permanentsYouControl) .color
        (.addMana (.lit 1) (.ofChosenColor none) [] (agent := .you))) = [] := by
  decide

/-- "For each color among permanents you control, add one mana of that color" -/
theorem badRepeatedCarriesNoColor :
    Instruction.check []
      (Primitives.Instruction.repeatTimes (.distinctCount .color permanentsYouControl)
        (.addMana (.lit 1) (.ofChosenColor none) [] (agent := .you)))
      = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

/-- "For each color among permanents you control, … of that creature type." -/
theorem badAxisValueCrossing :
    Instruction.check []
      (.doForEachKind .color (some permanentsYouControl) (.subtype .creature) (draw (.lit 1) (agent
          := .you)))
      = [.kindAxisSort] := by
  decide

/-- "For each creature type, …" -/
theorem badDomainlessOpenAxis :
    Instruction.check []
      (.doForEachKind (.subtype .creature .any) none (.subtype .creature) (draw (.lit 1) (agent :=
          .you)))
      = [.kindDomainOk] := by
  decide

/-- "Cumulative upkeep {2}". The Idris macro also wrote out the keyword's expansion; here the
keyword line carries the law directly. -/
theorem okManaCumulativeUpkeep :
    Ability.check [] (keywordCosting "CumulativeUpkeep" (.mana [generic 2])) = [] := by decide

/-- "Cumulative upkeep — an opponent loses 1 life." -/
theorem badOpponentPaysYourCost :
    Ability.check []
      (keywordCosting "CumulativeUpkeep" (.perform (loseLife (.lit 1) (agent := anOpponent))))
      = [.keywordCostPaidByYou "CumulativeUpkeep"] := by
  decide

/-- "Whenever a player taps a land for mana, …" -/
theorem okTappedForMana :
    GameEvent.check [] (.tappedForMana (some (a .anyPlayer)) (a land) none) = [] := by decide

/-- "Whenever a card in a graveyard is tapped for mana, …": only a permanent is tapped for mana
[CR#106.12]. -/
theorem badTappedForManaOffField :
    GameEvent.check [] (.tappedForMana (some .you) (a (.inZone graveyard)) none)
      = [.zoneFits] := by
  decide

/-- An artifact with the given text. -/
def artifactWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.artifact], text } }

/-- "Whenever a basic land is tapped for mana of the chosen color, draw a card." -/
def drawOnTapForChosenColor : Ability :=
  whenever
    (.tappedForMana none (a (.and [land, .hasSupertype .basic])) (some (.ofColor thatColor)))
    (draw (.lit 1) (agent := .you))

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

def afterALandTapForMana : Bindings :=
  GameEvent.after [] (.tappedForMana (some (a .anyPlayer)) (a land) none)

def afterAPlainLandTap : Bindings :=
  GameEvent.after [] (.verbedEvent (some (a .anyPlayer)) (.action "Tap") (some (a land)) none none)

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
    ProducedMana.check [] (.producedByEvent (a land)) = [.outcomeInScope .manaProduced 0] := by
  decide

def afterManaAdded : Bindings :=
  Instruction.intro [] (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you))

/-- "You don't lose this mana as steps and phases end." -/
theorem okThisManaAfterAdd :
    StaticSpec.check afterManaAdded (.manaRetention .you .thisMana) = [] := by decide

/-- "You don't lose this mana as steps and phases end" -/
theorem badThisManaWithoutAdd :
    StaticSpec.check [] (.manaRetention .you .thisMana)
      = [.outcomeInScope .manaAdded 0] := by
  decide

/-- "Target player sacrifices a creature of their choice." -/
theorem okBoundTheirChoice :
    Instruction.check [] (sacrifice (aTheirChoice creature) (agent := (target .anyPlayer))) = [] :=
        by decide

/-- "Destroy a creature of their choice." -/
theorem badUnboundTheirChoice :
    Instruction.check [] (destroy (aTheirChoice creature)) = [.chooserInScope 0] := by decide

/-- "Target creature becomes a black Zombie in addition to its other types." -/
theorem okUnnamedAddition :
    Instruction.check []
      (become (target creature)
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none)
      = [] := by
  decide

/-- "Target creature becomes a Zombie named Bob in addition to its other types." -/
theorem badNamedAddition :
    Instruction.check []
      (become (target creature)
        { characteristics := { name := some "Bob", subtypes := [creatureType "Zombie"] } } none)
      = [.becomesOk] := by
  decide

theorem badRepeatedAdditionColor :
    Instruction.check []
      (become (target creature)
        { characteristics :=
          { colors := [.black, .black], subtypes := [creatureType "Zombie"] } } none)
      = [.becomesOk] := by
  decide

theorem badRepeatedAdditionType :
    Instruction.check [] (become (target creature) { characteristics :=
                                                      { types := [.artifact, .artifact] } } none)
      = [.becomesOk] := by
  decide

end Semantics.Proofs.Mana
