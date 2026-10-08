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
the anaphor. The subject is written once, so it is refused once (the two-modification spelling, which re-read it, is
`GetsBothDeltas.badThatCreatureIsCondSubjectTwoHalves`). -/
theorem badThatCreatureIsCondSubject :
    Ability.check []
      (.static
        (onlyWhile (getsPt (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)))
          (.matches thisCreature attacking)))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Equipped creature gets +1/+1." -/
theorem okEquippedCreature :
    Ability.check []
      (.static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 1)) (.up (.lit 1))))
      = [] := by
  decide

/-- "You can't lose the game." -/
theorem okUntargetedOutcomeGate :
    Ability.check [] (.static (playerCant (.core .loseGame) .you)) = [] := by decide

/-- "where X is the number of creatures you control." -/
theorem okSingleStaticXRider :
    StaticSpec.check []
      (.conjunction none
        [ .modification thisCreature .power (.up (.letter .x)),
          .modification thisCreature .toughness (.up (.lit 0)),
          .letterDefinition .x (countOf creatureYouControl) ]) = [] := by
  decide

theorem badDoubleStaticRider :
    StaticSpec.check []
      (.conjunction none
        [ .modification thisCreature .power (.up (.letter .x)),
          .modification thisCreature .toughness (.up (.lit 0)),
          .letterDefinition .x (countOf creatureYouControl),
          .letterDefinition .x (countOf creature) ]) = [.openLetter .x] := by
  decide

/-- "power and toughness are each equal to the number of creatures you control" -/
theorem okSelfDefinedPt :
    StaticSpec.check [] (.ptDefinition thisCreature .bothEach (countOf creatureYouControl)) = [] :=
        by
  decide

theorem badGrantedPtDefinition :
    StaticSpec.check []
      (.ptDefinition (.attachHost .enchanted (.type .creature)) .bothEach (lifeTotalOf .you))
      = [.selfDefinedOk] := by
  decide

/-- "Creatures you control get +1/+1 until end of turn." -/
theorem okContinuousClause :
    Instruction.check []
      (.establish (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))
        (some untilEndOfTurn)) = [] := by
  decide

/-- "You become the monarch." -/
theorem okBecomesMonarch :
    Instruction.check [] (.gainDesignation .you (.named "monarch") none) = [] := by decide

/-- "You become goaded." The Idris pin refutes the scope; a player is not a holder of it
either. -/
theorem badGoadedPlayer :
    Instruction.check [] (.gainDesignation .you (.named "goaded") none)
      = [.designationScope (.named "goaded"), .designationHolder (.named "goaded") .player] := by
  decide

/-- "… it becomes monstrous," the grant the Monstrosity keyword action's expansion writes. -/
theorem okBecomesMonstrous :
    Instruction.check [] (.gainDesignation thisCreature (.named "monstrous") none) = [] := by
  decide

/-- The commander is held by a card, not granted to an object, so the scope and holder
refusals land. That a rule confers it and no instruction may is a game rule the checker does not
assert (`lean-checker-sweep-game-rules`). -/
theorem badGainsCommander :
    Instruction.check [] (.gainDesignation thisCreature (.named "commander") none)
      = [.designationScope (.named "commander"), .designationHolder (.named "commander") .object] := by
  decide

/-- "You get an enduring story," the grant the storied keyword ability's expansion writes. -/
theorem okGetsAnEnduringStory :
    Instruction.check [] (.gainDesignation .you (.named "enduringStory") none) = [] := by
  decide

/-- "Each land you control becomes a 2/2 creature. It's still a land." -/
theorem okStillALand :
    StaticSpec.check []
      (Primitives.StaticSpec.qualityChange (allOf land) .sets
        (.bundle { characteristics :=
                   { types := [.creature], power := some (.lit 2), toughness := some (.lit 2) } }
          (some .land))) = [] := by
  decide

/-- "Target creature gets +1/+0." -/
theorem okSingletonCoordination :
    StaticSpec.check [] (.conjunction none [.modification (target creature) .power (.up (.lit 1))])
      = [] := by
  decide

/-- a coordination of no statements -/
theorem badEmptyCoordination : StaticSpec.check [] (.conjunction none []) = [.nonEmpty] := by decide

/-- Explicit controlled permanents, controlled spells, and owned cards outside the battlefield. -/
theorem okSingleExtension :
    StaticSpec.check []
      (Primitives.StaticSpec.qualityChange
        (Primitives.NounPhrase.and [
              allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [creature, spell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, creature, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.inZone battlefield)]) ]) .adds (.everyTypeOf .creature)) = [] := by decide

/-- "If you would draw a card, draw two cards instead." -/
theorem okDrawReplacement :
    StaticSpec.check [] (.replacement (.draws .you) [] none (draw (.lit 2))
        .repeatedly none)
      = [] := by
  decide

/-- "unless you control an artifact" -/
theorem okUnlessOverNegatedCondition :
    StaticSpec.check []
      (.conditional (.altCost .this none)
        (.not (exists_ (.and [artifact, .hasPossessor .controller .you]))) .unless_) = [] := by
  decide

theorem badUnlessConjunction :
    StaticSpec.check []
      (.conditional (.altCost .this none)
        (.and
          [ exists_ (.and [artifact, .hasPossessor .controller .you]),
            exists_ (.and [enchantment, .hasPossessor .controller .you]) ]) .unless_)
      = [.markingOk] := by
  decide

/-- "This creature blocks an attacking creature." -/
theorem okCreatureBecomesBlocking :
    Instruction.check [] (Primitives.Instruction.becomeBlocking thisCreature (a (.and [creature, attacking])))
      = [] := by
  decide

/-- "Target land blocks an attacking creature.": a land an effect has made a creature blocks
[CR#205.1b,509.1a]. -/
theorem okLandBecomesBlocking :
    Instruction.check [] (Primitives.Instruction.becomeBlocking (target land) (a (.and [creature, attacking])))
      = [] := by
  decide

/-- "This creature blocks target planeswalker." -/
theorem badBecomesBlockingPlaneswalker :
    Instruction.check [] (Primitives.Instruction.becomeBlocking thisCreature (target (.hasType .planeswalker)))
      = [.deedNounOk (.core .block)] := by
  decide

/-- "this creature gets +1/+0" -/
theorem okAddPtUpward :
    StaticSpec.check [] (.modification thisCreature .power (.up (.lit 1))) = [] := by decide

/-- "During target opponent's next turn, …" -/
theorem okSingularNextTurnDuration :
    Duration.check [] (duringNextTurnOf (target .opponent)) = [] := by decide

/-- "During each opponent's next turn, ...": the boundaries name whose turn, so a plural possessor
is the duration-possessor refusal of the unified boundary (was `.singular` on the retired
`Duration.duringNextTurnOf`). -/
theorem badPluralNextTurnDuration :
    Duration.check [] (duringNextTurnOf (each .opponent)) = [.durationPossessor] := by decide

/-- "Target opponent skips all combat phases of their next turn." -/
theorem okSkipDuringTheirNextTurn :
    Instruction.check []
      (establishThroughout (.partSkip (target .opponent) .combat) (duringNextTurnOf (that
          .player)))
      = [] := by
  decide

/-- "You skip all combat phases of their next turn.": the bench macro writes the unbound
"their" at both of the turn's boundaries, its beginning and its end, and each is refused (the
RON helper captures it once; a `Duration` has no binding scope to capture it in Lean). -/
theorem badSkipDuringUnboundNextTurn :
    Instruction.check [] (establishThroughout (.partSkip .you .combat) (duringNextTurnOf (that
        .player)))
      = [.anaphor (.word .player) .one 0, .anaphor (.word .player) .one 0] := by
  decide

/-- "Your opponents can't gain life." A rules-meaningful sentence with no printed card on the
bench. -/
theorem opponentsCantGainLife :
    StaticSpec.check [] (playerCant (.core .gainLife) (.playerGroup .yourOpponents)) = [] := by
  decide

end Semantics.Proofs.Static
