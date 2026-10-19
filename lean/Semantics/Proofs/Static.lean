import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Static

Port of `idris/src/Experimental/Proofs/Static.idr`: the pins of the Static family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Static

/-- "This creature gets +2/+0 as long as this creature is attacking." -/
theorem okGetsBattlefieldSubject :
    Ability.check (DRS.ofList [])
      (.static
        (onlyWhile (getsPt thisCreature (.up (.lit 2)) (.up (.lit 0)))
          (.matches thisCreature isAttacking))) = [] := by
  decide

/-- "That creature gets +2/+0 as long as this creature is attacking." The Idris pin refutes
the anaphor. The subject is written once, so it is refused once. -/
theorem badThatCreatureIsCondSubject :
    Ability.check (DRS.ofList [])
      (.static
        (onlyWhile (getsPt (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)))
          (.matches thisCreature isAttacking)))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Equipped creature gets +1/+1." -/
theorem okEquippedCreature :
    Ability.check (DRS.ofList [])
      (.static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 1)) (.up (.lit 1))))
      = [] := by
  decide

/-- "You can't lose the game." -/
theorem okUntargetedOutcomeGate :
    Ability.check (DRS.ofList []) (.static (cant you loseGame)) = [] := by decide

/-- "where X is the number of creatures you control." -/
theorem okSingleStaticXRider :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ Templates.modification thisCreature .power (.up (.letter .x)),
          Templates.modification thisCreature .toughness (.up (.lit 0)),
          .letterDefinition .x (countOf (and [isCreature, youControl])) ]) = [] := by
  decide

theorem badDoubleStaticRider :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ Templates.modification thisCreature .power (.up (.letter .x)),
          Templates.modification thisCreature .toughness (.up (.lit 0)),
          .letterDefinition .x (countOf (and [isCreature, youControl])),
          .letterDefinition .x (countOf isCreature) ]) = [.openLetter .x] := by
  decide

/-- "power and toughness are each equal to the number of creatures you control" -/
theorem okSelfDefinedPt :
    StaticSpec.check (DRS.ofList []) (.ptDefinition thisCreature .bothEach (countOf (and [isCreature, youControl]))) = [] :=
        by
  decide

/-- "Creatures you control get +1/+1 until end of turn." -/
theorem okContinuousClause :
    Action.check (DRS.ofList [])
      (.establish (getsPt (allOf (and [isCreature, youControl])) (.up (.lit 1)) (.up (.lit 1)))
        (some untilEndOfTurn)) = [] := by
  decide

/-- "You become the monarch." -/
theorem okBecomesMonarch :
    Action.check (DRS.ofList []) (.gainDesignation .you (.named "monarch") none) = [] := by decide

/-- "You become goaded." The Idris pin refutes the scope: goaded is held by an object. -/
theorem badGoadedPlayer :
    Action.check (DRS.ofList []) (.gainDesignation .you (.named "goaded") none)
      = [.designationScope (.named "goaded")] := by
  decide

/-- "… it becomes monstrous," the grant the Monstrosity keyword action's expansion writes. -/
theorem okBecomesMonstrous :
    Action.check (DRS.ofList []) (.gainDesignation thisCreature (.named "monstrous") none) = [] := by
  decide

/-- The commander is held by a card, not granted to an object, so the scope and holder
refusals land. That a rule confers it and no action may is a game rule the checker does not
assert (`lean-checker-sweep-game-rules`). -/
theorem badGainsCommander :
    Action.check (DRS.ofList []) (.gainDesignation thisCreature (.named "commander") none)
      = [.designationScope (.named "commander"), .designationHolder (.named "commander") .object] := by
  decide

/-- "You get an enduring story," the grant the storied keyword ability's expansion writes. -/
theorem okGetsAnEnduringStory :
    Action.check (DRS.ofList []) (.gainDesignation .you (.named "enduringStory") none) = [] := by
  decide

/-- "Each land you control becomes a 2/2 creature. It's still a land." -/
theorem okStillALand :
    StaticSpec.check (DRS.ofList [])
      (qualityChange (allOf isLand) .sets
        (.bundle { characteristics :=
                   { types := [.creature], power := some (.lit 2), toughness := some (.lit 2) } }
          (some .land))) = [] := by
  decide

/-- "Target creature gets +1/+0." -/
theorem okSingletonCoordination :
    StaticSpec.check (DRS.ofList []) (.conjunction none [Templates.modification (target isCreature) .power (.up (.lit 1))])
      = [] := by
  decide

/-- a coordination of no statements -/
theorem badEmptyCoordination : StaticSpec.check (DRS.ofList []) (.conjunction none []) = [.nonEmpty] := by decide

/-- Explicit controlled permanents, controlled spells, and owned cards outside the battlefield. -/
theorem okSingleExtension :
    StaticSpec.check (DRS.ofList [])
      (qualityChange
        (Primitives.NounPhrase.and [
              allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [isCreature, isSpell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, isCreature, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.in_ battlefield)]) ]) .adds (.everyTypeOf .creature)) = [] := by decide

/-- "If you would draw a card, draw two cards instead." -/
theorem okDrawReplacement :
    StaticSpec.check (DRS.ofList []) (.instead (.would (.actor .you) (.draw .anyAmount)) (draw (.lit 2)) none
        .repeatedly none)
      = [] := by
  decide

/-- "unless you control an artifact" -/
theorem okUnlessOverNegatedCondition :
    StaticSpec.check (DRS.ofList [])
      (.conditional (.altCost .this none)
        (.not (exists_ (.and [isArtifact, .hasPossessor .controller .you]))) .unless_) = [] := by
  decide

theorem badUnlessConjunction :
    StaticSpec.check (DRS.ofList [])
      (.conditional (.altCost .this none)
        (.and
          [ exists_ (.and [isArtifact, .hasPossessor .controller .you]),
            exists_ (.and [isEnchantment, .hasPossessor .controller .you]) ]) .unless_)
      = [.markingOk] := by
  decide

/-- "This creature blocks an attacking creature." -/
theorem okCreatureBecomesBlocking :
    Action.check (DRS.ofList []) (becomeBlocking thisCreature (a (.and [isCreature, isAttacking])))
      = [] := by
  decide

/-- "Target land blocks an attacking creature.": a land an effect has made a creature blocks
[CR#205.1b,509.1a]. -/
theorem okLandBecomesBlocking :
    Action.check (DRS.ofList []) (becomeBlocking (target isLand) (a (.and [isCreature, isAttacking])))
      = [] := by
  decide

/-- "this creature gets +1/+0" -/
theorem okAddPtUpward :
    StaticSpec.check (DRS.ofList []) (Templates.modification thisCreature .power (.up (.lit 1))) = [] := by decide

/-- "During target opponent's next turn, …" -/
theorem okSingularNextTurnDuration :
    Duration.check (DRS.ofList []) (duringNextTurnOf (target (.isOpponentOf .you))) = [] := by decide

/-- "During each opponent's next turn, ...": the boundaries name whose turn, so a plural possessor
is the duration-possessor refusal of the unified boundary (was `.singular` on the retired
`Duration.duringNextTurnOf`). -/
theorem badPluralNextTurnDuration :
    Duration.check (DRS.ofList []) (duringNextTurnOf (each (.isOpponentOf .you))) = [.durationPossessor] := by decide

/-- "Target opponent skips all combat phases of their next turn." -/
theorem okSkipDuringTheirNextTurn :
    Action.check (DRS.ofList [])
      (establishThroughout (.partSkip (target (.isOpponentOf .you)) .combat) (duringNextTurnOf (that
          .player)))
      = [] := by
  decide

/-- The duration's possessor can read the explicit "you" in the preceding subject slot. -/
theorem durationPossessorReadsTheExplicitPlayer :
    let action := establishThroughout (.partSkip .you .combat) (duringNextTurnOf (that .player))
    let (after, errors) := stepAction 64 .postEvent .obj (DRS.ofList []) action
    errors = [] ∧ (identityBinding "duringNextTurnOf.player" after).map (·.id) = some 0 ∧
      after.referents.map (·.id) = [0] := by decide

theorem unboundDurationPossessorIsRefused :
    Duration.check (DRS.ofList []) (duringNextTurnOf (that .player))
      = [.anaphor (.word .player) .one 0, .bindingIdentity "duringNextTurnOf.player"] := by
  decide

/-- "Your opponents can't gain life." A rules-meaningful sentence with no printed card on the
bench. -/
theorem opponentsCantGainLife :
    StaticSpec.check (DRS.ofList []) (cant (actor (.described .all (.isOpponentOf .you))) (gainLife anyAmount)) = [] := by
  decide

end Semantics.Proofs.Static
