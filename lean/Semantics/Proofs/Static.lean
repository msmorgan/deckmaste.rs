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
        (onlyWhile (getsPt thisCreature (.up 2) (.up 0))
          (.matches thisCreature isAttacking))) = [] := by
  decide

/-- "That creature gets +2/+0 as long as this creature is attacking." The Idris pin refutes
the anaphor. The subject is written once, so it is refused once. -/
theorem badThatCreatureIsCondSubject :
    Ability.check (DRS.ofList [])
      (.static
        (onlyWhile (getsPt (that (.type .creature)) (.up 2) (.up 0))
          (.matches thisCreature isAttacking)))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Equipped creature gets +1/+1." -/
theorem okEquippedCreature :
    Ability.check (DRS.ofList [])
      (.static (getsPt (.attachHost .equipped (.type .creature)) (.up 1) (.up 1)))
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
          Templates.modification thisCreature .toughness (.up 0),
          .letterDefinition .x (countOf (and [isCreature, youControl])) ]) = [] := by
  decide

theorem badDoubleStaticRider :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ Templates.modification thisCreature .power (.up (.letter .x)),
          Templates.modification thisCreature .toughness (.up 0),
          .letterDefinition .x (countOf (and [isCreature, youControl])),
          .letterDefinition .x (countOf isCreature) ]) = [.openLetter .x] := by
  decide

/-- "power and toughness are each equal to the number of creatures you control" -/
theorem okSelfDefinedPt :
    StaticSpec.check (DRS.ofList []) (ptDefinition thisCreature .bothEach (countOf (and [isCreature, youControl]))) = [] :=
        by
  decide

/-- "Creatures you control get +1/+1 until end of turn." -/
theorem okContinuousClause :
    Action.check (DRS.ofList [])
      (.establish (getsPt (allOf (and [isCreature, youControl])) (.up 1) (.up 1))
        (some untilEndOfTurn)) = [] := by
  decide

/-- "You become the monarch." -/
theorem okBecomesMonarch :
    Action.check (DRS.ofList []) (.gainDesignation .you (.named "monarch" (.heldBy .player)) none) = [] := by decide

/-- "You become goaded." The Idris pin refutes the scope: goaded is held by an object. -/
theorem badGoadedPlayer :
    Action.check (DRS.ofList []) (.gainDesignation .you (.named "goaded" (.heldBy .object)) none)
      = [.designationScope (.named "goaded" (.heldBy .object))] := by
  decide

/-- "… it becomes monstrous," the grant the Monstrosity keyword action's expansion writes. -/
theorem okBecomesMonstrous :
    Action.check (DRS.ofList []) (.gainDesignation thisCreature (.named "monstrous" (.heldBy .object)) none) = [] := by
  decide

/-- The commander is held by a card, not granted to an object, so the scope and holder
refusals land. That a rule confers it and no action may is a game rule the checker does not
assert (`lean-checker-sweep-game-rules`). -/
theorem badGainsCommander :
    Action.check (DRS.ofList []) (.gainDesignation thisCreature (.named "commander" .heldByCard) none)
      = [.designationScope (.named "commander" .heldByCard), .designationHolder (.named "commander" .heldByCard) .object] := by
  decide

/-- "You get an enduring story," the grant the storied keyword ability's expansion writes. -/
theorem okGetsAnEnduringStory :
    Action.check (DRS.ofList []) (.gainDesignation .you (.named "enduringStory" (.heldBy .player)) none) = [] := by
  decide

/-- "Each land you control becomes a 2/2 creature. It's still a land." -/
theorem okStillALand :
    StaticSpec.check (DRS.ofList [])
      (qualityChange (allOf isLand) .sets
        (.bundle { characteristics :=
                   { types := [.creature], power := some 2, toughness := some 2 } }
          (some .land))) = [] := by
  decide

/-- "Target creature gets +1/+0." -/
theorem okSingletonCoordination :
    StaticSpec.check (DRS.ofList []) (.conjunction none [Templates.modification (target isCreature) .power (.up 1)])
      = [] := by
  decide

/-- a coordination of no statements -/
theorem badEmptyCoordination : StaticSpec.check (DRS.ofList []) (.conjunction none []) = [.nonEmpty] := by decide

/-- Explicit controlled permanents, controlled spells, and owned cards outside the battlefield. -/
theorem okSingleExtension :
    StaticSpec.check (DRS.ofList [])
      (qualityChange
        (Primitives.NounPhrase.coord .and [
              allOf (Primitives.Predicate.coord .and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.coord .and [isCreature, isSpell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.coord .and [Primitives.Predicate.isCard, isCreature, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.in_ battlefield)]) ]) .adds (.quality (.everyTypeOf .creature))) = [] := by decide

/-- "If you would draw a card, draw two cards instead." -/
theorem okDrawReplacement :
    StaticSpec.check (DRS.ofList []) (replace (would you (draw 1)) (instead (draw 2)))
      = [] := by
  decide

/-! ### Replacements

"Instead" builds a `Replacement`, the only thing a replacement slot accepts, and nothing else
accepts one [CR#614.1a]. -/

/--
error: Application type mismatch: The argument
  instead (draw 1)
has type
  Replacement
but is expected to have type
  Action
in the application
  @spell instAuthoringContext (instead (draw 1))
-/
#guard_msgs in
example : Ability := spell (instead (draw 1))

/--
error: Application type mismatch: The argument
  instead (draw 1)
has type
  Replacement
but is expected to have type
  Action
in the application
  instead (instead (draw 1))
-/
#guard_msgs in
example : Replacement := instead (instead (draw 1))

/--
error: Application type mismatch: The argument
  draw 2
has type
  Action
but is expected to have type
  Replacement
in the application
  @Templates.StaticSpec.replace instAuthoringContext (would (actor you) (draw 1)) (draw 2)
-/
#guard_msgs in
example : StaticSpec := replace (would you (draw 1)) (draw 2)

/-- "Draw a card. If you control three or more artifacts, draw two cards instead.": the
condition is the replacement's own, so the draw happens unreplaced while it fails
[CR#614.15]. -/
theorem conditionalReplacementCarriesItsCondition :
    (replace (draw 1)
      (doIf (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3)
        (instead (draw 2))) : Action) =
      .replace (draw 1)
        (.onlyIf (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3)
          (.instead (draw 2))) := rfl

/-- The replacement's condition is read: one naming nothing it can resolve is refused. -/
theorem conditionalReplacementChecksItsCondition :
    Action.check (DRS.ofList [])
      (replace (draw 1) (doIf (is it isCreature) (instead (draw 2)))) =
      [.anaphor .bare .one 0] := by
  decide

private def nestCallers : Nat → Replacement → Replacement
  | 0, replacement => replacement
  | n + 1, replacement => .inCaller (nestCallers n replacement)

/-- A condition over a replacement nested more deeply than the checker reads is refused, rather
than read as a replacement that does nothing. -/
theorem conditionOverTooDeepReplacementIsRefused :
    StaticSpec.check (DRS.ofList [])
      (replace (would you (draw 1))
        (doIf (is you (hasDesignation citysBlessing))
          (nestCallers 9 (instead (draw 2))))) = [.recursionLimit] := by
  decide

/-! ### What a replacement makes of its event

"Instead" puts an action in the event's place; `before` does one before the event and `alongside`
does one as part of it, the event still happening; `withRiders` lets the entering happen with Entry
Riders. None restates the event
[CR#614.1a,614.1c,614.1d]. -/

/-- "Creatures you control enter with an additional +1/+1 counter on them": a plural subject's
entering is ridden the same way, with no pronoun to agree with it [CR#614.1d]. -/
theorem okPluralEnteringRidden :
    StaticSpec.check (DRS.ofList [])
      (entersWithAdditionalCounters (allOf (and [isCreature, youControl])) 1
        p1p1Counter) = [] := by
  decide

/-- Entry Riders ride only an entering: "If you would draw a card, it enters tapped" is refused
[CR#614.1c,614.12]. -/
theorem badRidersOverADraw :
    StaticSpec.check (DRS.ofList [])
      (replace (would you (draw 1)) (withRiders [.entersAs .tapped])) =
      [.riddenEntering] := by
  decide

/-- The same refusal for a spell's own replacement: "Draw a card; it enters tapped instead". -/
theorem badSelfReplacementRidersOverADraw :
    Action.check (DRS.ofList [])
      (replace (draw 1) (withRiders [.entersAs .tapped])) = [.riddenEntering] := by
  decide

/-- A move is not enough: "If this creature would die, it enters tapped" is refused, dying being
a move to a graveyard [CR#614.12,700.4]. -/
theorem badRidersOverADying :
    StaticSpec.check (DRS.ofList [])
      (replace (would thisCreature die) (withRiders [.entersAs .tapped])) =
      [.riddenEntering] := by
  decide

/-- A token's creation is an entering: "If you would create one or more tokens, they enter
tapped" [CR#111.1,614.1d]. -/
theorem okRidersOverATokenCreation :
    StaticSpec.check (DRS.ofList [])
      (replace
        (would you (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) [])))
        (withRiders [.entersAs .tapped])) = [] := by
  decide

/-- A Pile Placement is no Entry Rider: "this creature enters in any order" is refused
[CR#401.4,614.12]. -/
theorem badPilePlacementOverAnEntering :
    StaticSpec.check (DRS.ofList [])
      (replace (would thisCreature enter) (withRiders [.inOrder .anyOrder])) =
      [.entryRidersOnly] := by
  decide

/-- A condition over riders keeps them riding the move: "If it's night, this permanent enters
transformed" [CR#702.145b]. -/
theorem okConditionalRiders :
    StaticSpec.check (DRS.ofList [])
      (replace (would thisPermanent enter) (doIf (gameIs night) (withRiders [.entersTransformed])))
      = [] := by
  decide

/-- "As this creature enters, choose an opponent. That player may put an additional +1/+1
counter on it as it enters." (tribute 1): the player chosen before the entering is read in the
action done as part of it [CR#702.104a,614.12a]. -/
theorem okAlongsideReadsTheChoiceBefore :
    StaticSpec.check (DRS.ofList [])
      (replace (would thisCreature enter)
        (before (choose anOpponent)
          (some
            (alongside
              (instruct (the isChosenPlayer)
                (may (putCounters 1 (.printed p1p1Counter) thisCreature))))))) = [] := by
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
    StaticSpec.check (DRS.ofList []) (Templates.modification thisCreature .power (.up 1)) = [] := by decide

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

/-- "You skip all combat phases of your next turn.": the duration's "your" re-mentions the "you"
of the subject, one referent (`mentionYou`). "that player" never reads "you", a self-referent
(`Determiner.self`; owner ruling 2026-10-24). -/
theorem durationPossessorReadsTheExplicitPlayer :
    let action := establishThroughout (.partSkip .you .combat) (duringNextTurnOf .you)
    let (after, errors) := stepAction 64 .postEvent .obj (DRS.ofList []) action
    errors = [] ∧ (identityBinding "duringNextTurnOf.player" after).map (·.id) = some 0 ∧
      after.referents.map (·.id) = [0] ∧
      Action.check (DRS.ofList [])
        (establishThroughout (.partSkip .you .combat) (duringNextTurnOf (that .player))) =
        [.anaphor (.word .player) .one 0] := by decide

/-- "until that player's next turn" with no player to read: refused once, by the pronoun; the
template's later mention of what it bound nothing to adds no refusal of its own. -/
theorem unboundDurationPossessorIsRefused :
    Duration.check (DRS.ofList []) (duringNextTurnOf (that .player))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Your opponents can't gain life." A rules-meaningful sentence with no printed card on the
bench. -/
theorem opponentsCantGainLife :
    StaticSpec.check (DRS.ofList []) (cant (actor (.described .all (.isOpponentOf .you))) (gainLife anyAmount)) = [] := by
  decide

end Semantics.Proofs.Static
