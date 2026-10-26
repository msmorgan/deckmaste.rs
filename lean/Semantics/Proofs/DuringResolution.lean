import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.Helpers

/-!
# Semantics.Proofs.DuringResolution

"During its resolution" (haunt's spell half [CR#702.55a]) and "as it resolves" (buyback
[CR#702.27a], rebound [CR#702.88a]) name the move that puts an instant or sorcery spell into its
owner's graveyard as the final part of its resolution [CR#608.2n]. Each is written as that move
caused by the spell's resolution, `Causing.event (does .any (resolve thisSpell))`, so a countered
spell's move to the graveyard, which no resolution causes [CR#701.6a], is not the event these
abilities watch. The Lean pins are on the terms; whether a countered spell triggers haunt or is
replaced by buyback or rebound is the engine mirror's to execute.

The move is written with `putInto … (from_ := stack)`, so its observation point is computed
(`Location.observedOnDeparture`): a spell put into a graveyard from the stack is none of the
look-back zone changes, so it is observed on arrival [CR#603.10a].
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.DuringResolution

/-- The trigger events of a keyword's triggered halves, in order. -/
def triggerEvents (a : Ability) : List GameEvent :=
  (halves a).filterMap fun
    | .triggered ev .. => some ev
    | _ => none

/-- The would-clauses of a keyword's static replacement halves, in order. -/
def wouldClauses (a : Ability) : List WouldClause :=
  (halves a).filterMap fun
    | .static (.replace wc ..) => some wc
    | _ => none

/-- Is this action, under its own scope, the resolution of a spell or ability? -/
def isResolution (a : Action) : Bool :=
  match a.scopeBody with
  | .procedure .resolve .. => true
  | _ => false

/-- An event caused by a resolution [CR#608.2n]. -/
def eventResolutionCaused : GameEvent → Bool
  | .causes (.event (.does _ a)) _ => isResolution a
  | _ => false

/-- A would-clause caused by a resolution [CR#608.2n]. -/
def wouldResolutionCaused : WouldClause → Bool
  | .causedBy (.event (.does _ a)) _ => isResolution a
  | _ => false

/-- The observation point of a move. -/
def moveAt? : Action → Option ObservationPoint
  | .move _ _ _ at_ _ => some at_
  | _ => none

/-- The observation point of the move an event names, bare or under a cause. -/
def eventMoveAt? : GameEvent → Option ObservationPoint
  | .causes _ (.does _ a) | .does _ a => moveAt? a
  | _ => none

/-- The observation point of the move a would-clause names, bare or under a cause. -/
def wouldMoveAt? : WouldClause → Option ObservationPoint
  | .causedBy _ (.would _ a) | .would _ a => moveAt? a
  | _ => none

/-- The `counter` Definition's own stack-to-graveyard move, as the event "is countered"
[CR#701.6a]. -/
def countered : GameEvent := does .any (counter thisSpell)

/-! ## Observation points [CR#603.10a] -/

/-- Haunt's spell half watches a move observed on arrival; its permanent half, a dies trigger,
looks back to the permanent as it left the battlefield [CR#603.10a,700.4]. -/
theorem hauntSpellHalfObservesOnArrival :
    (triggerEvents haunt).map eventMoveAt? = [some .departure, some .arrival] := by
  rfl

/-- Buyback's replacement watches a move observed on arrival. -/
theorem buybackMoveObservesOnArrival :
    (wouldClauses (buyback (.mana [generic 3]))).map wouldMoveAt? = [some .arrival] := by
  rfl

/-- Rebound's replacement watches a move observed on arrival. -/
theorem reboundMoveObservesOnArrival :
    (wouldClauses rebound).map wouldMoveAt? = [some .arrival] := by
  rfl

/-! ## A countered spell [CR#608.2n,701.6a] -/

/-- Haunt's spell half is caused by the spell's resolution and its permanent half is not;
countering the spell causes its move to the graveyard by no resolution, so haunt's spell half
does not trigger on it [CR#702.55a,608.2n,701.6a]. -/
theorem counteredHauntSpellDoesNotTrigger :
    (triggerEvents haunt).map eventResolutionCaused = [false, true] ∧
      eventResolutionCaused countered = false := by
  decide

/-- Buyback replaces only the move its spell's resolution causes, not a countered spell's
[CR#702.27a,608.2n,701.6a]. -/
theorem counteredBuybackSpellDoesNotReplace :
    (wouldClauses (buyback (.mana [generic 3]))).map wouldResolutionCaused = [true] ∧
      eventResolutionCaused countered = false := by
  decide

/-- Rebound replaces only the move its spell's resolution causes, not a countered spell's
[CR#702.88a,608.2n,701.6a]. -/
theorem counteredReboundSpellDoesNotReplace :
    (wouldClauses rebound).map wouldResolutionCaused = [true] ∧
      eventResolutionCaused countered = false := by
  decide

/-! ## The resolve Procedure [CR#608.1,608.2] -/

/-- `does .any (resolve thisSpell)` checks clean as an event, as a cause, and in a timing; and
`resolve` is admitted wherever `cast` is, as an action and as an event's action. -/
theorem resolveProcedureChecks :
    GameEvent.check (DRS.ofList []) (does .any (resolve thisSpell)) = [] ∧
      Action.check (DRS.ofList []) (resolve thisSpell) = [] ∧
      Action.check (DRS.ofList []) (cast thisSpell) = [] ∧
      GameEvent.check (DRS.ofList [])
        (.causes (.event (does .any (resolve thisSpell)))
          (does .any (putInto thisSpell graveyard (from_ := stack)))) = [] ∧
      WouldClause.check (DRS.ofList [])
        (.causedBy (.event (does .any (resolve thisSpell)))
          (.would .any (putInto thisSpell graveyard (from_ := stack)))) = [] ∧
      Timing.check (DRS.ofList []) (.whileDoing (does .any (resolve thisSpell))) = [] ∧
      Timing.check (DRS.ofList []) (.whileDoing (does .any (cast thisSpell))) = [] := by
  decide

/-- Haunt, buyback and rebound check clean, and each on an instant card. -/
theorem okResolutionKeywords :
    Ability.check (DRS.ofList []) haunt = [] ∧
      Ability.check (DRS.ofList []) (buyback (.mana [generic 3])) = [] ∧
      Ability.check (DRS.ofList []) rebound = [] ∧
      Card.check
        (.singleFaced
          { characteristics :=
              { name := some "", cost := some [generic 2], types := [.instant],
                text := [haunt, buyback (.mana [generic 3]), rebound] } }) = [] := by
  decide

end Semantics.Proofs.DuringResolution
