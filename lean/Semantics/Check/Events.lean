import Semantics.Check.Words
import Semantics.Check.CardFacts
import Semantics.Abilities

/-!
# Semantics.Check.Events

Closed event head-gating (`does`, `would`, `did`) and the lookback,
deed, and play-window rules (`docs/decisions/semantics-v3.md` §3.6–§3.8).
-/

namespace Semantics

/-- The life outcomes a life change publishes. Setting a life total gains or loses the necessary
amount of life [CR#119.5], and doubling one is written as that set [CR#701.10d], so a set publishes
both, as an exchange of life totals does. -/
def Delta.lifeOutcomes : Delta Amount → List OutcomeSort
  | .up _ => [.lifeGained]
  | .down _ => [.lifeLost]
  | .set _ => [.lifeGained, .lifeLost]

/-- Whether the head of `e` passes `admits`, the head read through its caller and scope-only
wrappers (`Action.scopeBody`). An Action Definition's own Reference Scope header names the one
deed it performs (`Action.ownDeed?`), and every head gate admits it alike, so an event, a lookback
and a deontic clause read a Definition's deed the same way (`semantics-v3.md` §2.1). -/
def Action.headGate (admits : Action → Bool) (e : Action) : Bool :=
  e.ownDeed?.isSome || admits e.scopeBody

/-- The event-capable `Action` heads (`semantics-v3.md` §3.6, Table 3.6A), refusing compound or
control-flow actions (`sequentially`, `doIf`, `doForEach`, `chooseModes`, `establish`, etc.). -/
def Action.eventHead : Action → Bool
  | .dealDamage .. | .markDamage .. | .clearDamage ..
  | .preventDamage .. | .redirectDamage ..
  | .setStatus .. | .turnOver .. | .turnFaceDown ..
  | .combat .. | .attack .. | .block .. | .attackUnblocked .. | .attackWith ..
  | .target .. | .tapForMana .. | .commitCrime | .attachment .. | .changeControl ..
  | .trigger | .crew .. | .saddle .. | .fullyUnlock ..
  | .gainDesignation .. | .gameGains _ | .unlock .. | .conclude ..
  | .choose .. | .vote .. | .copy .. | .changeLife .. | .addMana .. | .draw ..
  | .expose .. | .search .. | .shuffle .. | .flipCoins .. | .rollDice ..
  | .createObject .. | .putCounters .. | .distribute .. | .removeCounters .. | .moveCounters ..
  | .move .. | .pay .. | .procedure .. | .relate .. | .merge ..
  | .skipUntap .. | .skipPart .. | .addTurnPart .. | .exchange .. => true
  | _ => false

/-- Closed event head-gating (`semantics-v3.md` §3.6): `does`, `would`, and `did` admit an
Action Definition by its header and otherwise only an event-capable head (`Action.eventHead`). -/
def Action.eventHeadOk (e : Action) : Bool := e.headGate Action.eventHead

/-- Whether an event-capable action carries a quantity or magnitude that `TallyMode.quantity`
can sum (`semantics-v3.md` §3.8). -/
def Action.eventHasMagnitude (e : Action) (fuel : Nat := 16) : Bool :=
  match fuel, e with
  | 0, _ => false
  | n + 1, .inCaller body | n + 1, .ownScope _ body => body.eventHasMagnitude n
  | _ + 1, .dealDamage .. | _ + 1, .markDamage ..
  | _ + 1, .changeLife (.up _) | _ + 1, .changeLife (.down _)
  | _ + 1, .pay .. | _ + 1, .draw .. | _ + 1, .putCounters .. | _ + 1, .removeCounters ..
  | _ + 1, .moveCounters .. | _ + 1, .createObject .. | _ + 1, .addMana .. => true
  | _ + 1, _ => false
termination_by structural fuel

structure EventFacts where
  hasMagnitude : Bool := false
  deriving Repr, BEq

/-- Event properties: whether the event carries a magnitude a tally can sum. -/
def GameEvent.facts (ev : GameEvent) (fuel : Nat := 16) : EventFacts :=
  match fuel, ev with
  | 0, _ => {}
  | n + 1, .inCaller body => body.facts n
  | _ + 1, .does _ action => { hasMagnitude := action.eventHasMagnitude }
  | n + 1, .nthOccurrence _ _ ev | n + 1, .causes _ ev => ev.facts n
  | _ + 1, .beginningOf .. | _ + 1, .endOf .. | _ + 1, .now | _ + 1, .stateHolds ..
  | _ + 1, .chapterMark .. | _ + 1, .stateBasedActionsChecked => {}
termination_by structural fuel

def GameEvent.scopeBody (ev : GameEvent) (fuel : Nat := 16) : GameEvent :=
  match fuel, ev with
  | 0, body => body
  | n + 1, .inCaller body => body.scopeBody n
  | _ + 1, body => body
termination_by structural fuel

def eventHasMagnitude (ev : GameEvent) : Bool := ev.facts.hasMagnitude

def DidClause.hasMagnitude (c : DidClause) : Bool := c.action.eventHasMagnitude

def tallyOk : TallyMode → DidClause → Bool
  | .events, _ => true
  | .quantity, c => c.hasMagnitude

def NounPhrase.isYouFuel (fuel : Nat) : NounPhrase → Bool :=
  match fuel with
  | 0 => fun _ => false
  | fuel + 1 => fun
      | .you => true
      | .inCaller body | .firstMention _ body | .laterMention _ body => body.isYouFuel fuel
      | _ => false
termination_by structural fuel

def NounPhrase.isYou := NounPhrase.isYouFuel 64

def distinctDeeds : List Deed → Bool
  | [] => true
  | d :: ds => !ds.elem d && distinctDeeds ds

/-- A definite referent of one entity of kind `k`: a player, or an object. -/
def kindBinding (k : Kind) : Referent :=
  if k == .player then { det := .the, plur := .one, payload := .player false }
  else { det := .the, plur := .one, payload := .object [] none none none none }

end Semantics
