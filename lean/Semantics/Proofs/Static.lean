import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Static

Port of `idris/src/Experimental/Proofs/Static.idr`: the pins of the Static family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Static

/-- "This creature gets +2/+0 as long as this creature is attacking." -/
theorem okGetsBattlefieldSubject :
    Ability.check []
      (.static
        (onlyWhile (getsPt thisCreature (.up (.lit 2)) (.up (.lit 0)))
          (.matches thisCreature attacking))) = [] := by
  decide

/-- "That creature gets +2/+0 as long as this creature is attacking." The Idris pin refutes
the first anaphor; unresolved, the subject has no zone, and the toughness half's `it` reads
back the same missing antecedent. -/
theorem badThatCreatureIsCondSubject :
    Ability.check []
      (.static
        (onlyWhile (getsPt (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)))
          (.matches thisCreature attacking)))
      = [ .anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield,
          .anaphor .bare .one 0, .zoneIs .battlefield ] := by
  decide

/-- "Equipped creature gets +1/+1." -/
theorem okEquippedCreature :
    Ability.check []
      (.static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 1)) (.up (.lit 1))))
      = [] := by
  decide

/-- "Equipped land gets +1/+1." -/
theorem badEquippedLand :
    Ability.check []
      (.static (getsPt (.attachHost .equipped (.type .land)) (.up (.lit 1)) (.up (.lit 1))))
      = [.attachHeadOk] := by
  decide

/-- "Fortified creature gets +1/+1." -/
theorem badFortifiedCreature :
    Ability.check []
      (.static (getsPt (.attachHost .fortified (.type .creature)) (.up (.lit 1)) (.up (.lit 1))))
      = [.attachHeadOk] := by
  decide

/-- "You can't lose the game." -/
theorem okUntargetedOutcomeGate :
    Ability.check [] (.static (playerCant (.core .loseGame) .you)) = [] := by decide

/-- "Target player can't lose the game." -/
theorem badTargetedOutcomeGate :
    Ability.check [] (.static (playerCant (.core .loseGame) (target .anyPlayer)))
      = [.nontarget] := by
  decide

/-- "where X is the number of creatures you control." -/
theorem okSingleStaticXRider :
    StaticSpec.check []
      (.andAlso none
        [ .modify thisCreature .power (.up (.letter .x)),
          .modify thisCreature .toughness (.up (.lit 0)),
          .definesLetter .x (countOf creatureYouControl) ]) = [] := by
  decide

theorem badDoubleStaticRider :
    StaticSpec.check []
      (.andAlso none
        [ .modify thisCreature .power (.up (.letter .x)),
          .modify thisCreature .toughness (.up (.lit 0)),
          .definesLetter .x (countOf creatureYouControl),
          .definesLetter .x (countOf creature) ]) = [.openLetter .x] := by
  decide

/-- "power and toughness are each equal to the number of creatures you control" -/
theorem okSelfDefinedPt :
    StaticSpec.check [] (.definesPt thisCreature .bothEach (countOf creatureYouControl)) = [] := by
  decide

theorem badGrantedPtDefinition :
    StaticSpec.check []
      (.definesPt (.attachHost .enchanted (.type .creature)) .bothEach (lifeTotalOf .you))
      = [.selfDefinedOk] := by
  decide

/-- "Creatures you control get +1/+1 until end of turn." -/
theorem okContinuousClause :
    Instruction.check []
      (.continuously (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))
        (some untilEndOfTurn)) = [] := by
  decide

theorem badPtDefinitionClause :
    Instruction.check []
      (.continuously (.definesPt thisCreature .bothEach (countOf creatureYouControl)) none)
      = [.clauseStatic] := by
  decide

/-- "You become the monarch." -/
theorem okBecomesMonarch :
    Instruction.check [] (.gainsDesignation .you "the monarch" .instructed none) = [] := by decide

/-- "You become goaded." The Idris pin refutes the scope; a player is not a holder of it
either. -/
theorem badGoadedPlayer :
    Instruction.check [] (.gainsDesignation .you "goaded" .instructed none)
      = [.designationScope "goaded", .designationHolder "goaded" .player] := by
  decide

/-- "… it becomes monstrous," conferred by the Monstrosity keyword action's own expansion. -/
theorem okMonstrousByDeed :
    Instruction.check []
      (.gainsDesignation thisCreature "monstrous" (.byDeed (.action "Monstrosity")) none)
      = [] := by
  decide

/-- The same conferral misattributed to a keyword ability: Monstrosity is a keyword action
(a deed), so no keyword row confers "monstrous". -/
theorem badMonstrousByKeyword :
    Instruction.check []
      (.gainsDesignation thisCreature "monstrous" (.byKeyword "Monstrosity") none)
      = [.designationChecked "monstrous"] := by
  decide

/-- "You get an enduring story," conferred by the storied keyword ability's own expansion. -/
theorem okEnduringStoryByStoried :
    Instruction.check [] (.gainsDesignation .you "an enduring story" (.byKeyword "Storied") none)
      = [] := by
  decide

/-- "Each land you control becomes a 2/2 creature. It's still a land." -/
theorem okStillALand :
    StaticSpec.check []
      (.becomes (allOf land) .sets
        (.bundle { characteristics :=
                   { types := [.creature], power := some (.lit 2), toughness := some (.lit 2) } }
          (some .land))) = [] := by
  decide

/-- "Target creature becomes a Coward until end of turn. It's still a land." -/
theorem badStillOnSubtypeSet :
    StaticSpec.check []
      (.becomes (target creature) .sets
        (.bundle { characteristics := { subtypes := [creatureType "Coward"] } } (some .land)))
      = [.becomesOk] := by
  decide

theorem badStillAnInstant :
    StaticSpec.check []
      (.becomes (target creature) .sets
        (.bundle { characteristics := { types := [.artifact] } } (some .instant)))
      = [.becomesOk] := by
  decide

/-- "Target creature gets +1/+0." -/
theorem okSingletonCoordination :
    StaticSpec.check [] (.andAlso none [.modify (target creature) .power (.up (.lit 1))])
      = [] := by
  decide

/-- a coordination of no statements -/
theorem badEmptyCoordination : StaticSpec.check [] (.andAlso none []) = [.nonEmpty] := by decide

/-- "Creatures you control are every creature type." -/
theorem okSingleExtension :
    StaticSpec.check []
      (.alsoOffBattlefield (.becomes (allOf creatureYouControl) .adds (.everyTypeOf .creature)))
      = [] := by
  decide

theorem badDoubleExtension :
    StaticSpec.check []
      (.alsoOffBattlefield
        (.alsoOffBattlefield
          (.becomes (allOf (.and [creature, .hasPossessor .controller .you])) .adds
            (.bundle { characteristics := { types := [.artifact] } } none)))) = [.notExtended] := by
  decide

/-- "If you would draw a card, draw two cards instead." -/
theorem okDrawReplacement :
    StaticSpec.check [] (.intercepts (.draws .you) [] none (draw .you (.lit 2)) .repeatedly none)
      = [] := by
  decide

/-- "If I — would happen, draw a card instead." -/
theorem badChapterReplacement :
    StaticSpec.check []
      (.intercepts (.chapterMark [1]) [] none (draw .you (.lit 1)) .repeatedly none)
      = [.interceptable] := by
  decide

/-- "unless you control an artifact" -/
theorem okUnlessOverNegatedCondition :
    StaticSpec.check []
      (.conditionally (.altCost .this none)
        (.not (exists_ (.and [artifact, .hasPossessor .controller .you]))) .unless_) = [] := by
  decide

theorem badUnlessConjunction :
    StaticSpec.check []
      (.conditionally (.altCost .this none)
        (.and
          [ exists_ (.and [artifact, .hasPossessor .controller .you]),
            exists_ (.and [enchantment, .hasPossessor .controller .you]) ]) .unless_)
      = [.markingOk] := by
  decide

/-- "This creature blocks an attacking creature." -/
theorem okCreatureBecomesBlocking :
    Instruction.check [] (.becomesBlocking thisCreature (a (.and [creature, attacking])))
      = [] := by
  decide

/-- "Target land blocks an attacking creature.": a land an effect has made a creature blocks
[CR#205.1b,509.1a]. -/
theorem okLandBecomesBlocking :
    Instruction.check [] (.becomesBlocking (target land) (a (.and [creature, attacking])))
      = [] := by
  decide

/-- "This creature blocks target planeswalker." -/
theorem badBecomesBlockingPlaneswalker :
    Instruction.check [] (.becomesBlocking thisCreature (target (.hasType .planeswalker)))
      = [.deedNounOk (.core .block)] := by
  decide

/-- "this creature gets +1/+0" -/
theorem okAddPtUpward :
    StaticSpec.check [] (.modify thisCreature .power (.up (.lit 1))) = [] := by decide

/-- "this creature's mana value is 2 more than it was"; a mana value is read off the mana cost
[CR#202.3], not modified in the layer that spells "gets". -/
theorem badModifyManaValue :
    StaticSpec.check [] (.modify thisCreature .manaValue (.up (.lit 2))) = [.modifyStat] := by
  decide

/-- "this planeswalker's loyalty becomes 3"; loyalty is the number of loyalty counters on it
[CR#306.5c], so a loyalty change belongs to the counter lane. -/
theorem badSetLoyalty :
    StaticSpec.check [] (.modify thisPlaneswalker .loyalty (.set (.lit 3))) = [.modifyStat] := by
  decide

/-- "During target opponent's next turn, …" -/
theorem okSingularNextTurnDuration :
    Duration.check [] (.duringNextTurnOf (target .opponent)) = [] := by decide

/-- "During each opponent's next turn, ..." -/
theorem badPluralNextTurnDuration :
    Duration.check [] (.duringNextTurnOf (each .opponent)) = [.singular] := by decide

/-- "Target opponent skips all combat phases of their next turn." -/
theorem okSkipDuringTheirNextTurn :
    Instruction.check []
      (throughout (.skips (target .opponent) .combat) (.duringNextTurnOf (that .player)))
      = [] := by
  decide

/-- "You skip all combat phases of their next turn." -/
theorem badSkipDuringUnboundNextTurn :
    Instruction.check [] (throughout (.skips .you .combat) (.duringNextTurnOf (that .player)))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Your opponents can't gain life." A rules-meaningful sentence with no printed card on the
bench. -/
theorem opponentsCantGainLife :
    StaticSpec.check [] (playerCant (.core .gainLife) (.playerGroup .yourOpponents)) = [] := by
  decide

end Semantics.Proofs.Static
