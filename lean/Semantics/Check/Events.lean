import Semantics.Check.Words
import Semantics.Abilities

/-!
# Semantics.Check.Events

Minimal core event facts table, closed event head-gating (`does`, `would`, `did`), and the lookback
and deed rules (`docs/decisions/semantics-v3.md` §3.6–§3.8).
-/

namespace Semantics

/-- Only a removal empties a named kind, so only it has a last counter [CR#310.12b,702.62a]. -/
def counterBatchOk : CounterBatch → CounterMove → Option CounterKind → Bool
  | .emptying, .removed, kind => kind.isSome
  | .emptying, _, _ => false
  | _, _, _ => true

def counterBatchSourceOk : CounterBatch → CounterMove → Option CounterKindSource → Bool
  | .emptying, .removed, some .anyKind => false
  | .emptying, .removed, some _ => true
  | .emptying, _, _ => false
  | _, _, _ => true

def LifeMove.outcome : LifeMove → OutcomeSort
  | .up => .lifeGained
  | .down => .lifeLost

def Delta.outcome : Delta Amount → Option OutcomeSort
  | .up _ => some .lifeGained
  | .down _ => some .lifeLost
  | .set _ => none

/-- Closed event head-gating (`semantics-v3.md` §3.6): `does`, `would`, and `did` only admit
event-capable `Instruction` heads (Table 3.6A) plus CR §701 keyword-action definitions wrapped in
`ownScope (some (.action name))`, refusing compound or control-flow instructions (`sequentially`,
`doIf`, `doForEach`, `establish`, etc.). -/
def Instruction.eventHeadOk (e : Instruction) (fuel : Nat := 16) : Bool :=
  match fuel, e with
  | 0, _ => false
  | n + 1, .inCaller body | n + 1, .bracketed body => body.eventHeadOk n
  | _ + 1, .ownScope (some _) _ => true
  | n + 1, .ownScope none body => body.eventHeadOk n
  | _ + 1, .enact .. => true
  | _ + 1, .dealDamage .. | _ + 1, .setStatus ..
  | _ + 1, .attack .. | _ + 1, .block ..
  | _ + 1, .choose .. | _ + 1, .changeLife .. | _ + 1, .draw ..
  | _ + 1, .expose .. | _ + 1, .createObject ..
  | _ + 1, .putCounters .. | _ + 1, .removeCounters ..
  | _ + 1, .move .. | _ + 1, .pay .. | _ + 1, .procedure .. => true
  | _ + 1, _ => false
termination_by structural fuel

/-- Whether an event-capable instruction carries a quantity or magnitude that `TallyMode.quantity`
can sum (`semantics-v3.md` §3.8). -/
def Instruction.eventHasMagnitude (e : Instruction) (fuel : Nat := 16) : Bool :=
  match fuel, e with
  | 0, _ => false
  | n + 1, .inCaller body | n + 1, .bracketed body | n + 1, .ownScope _ body | n + 1, .enact _ body _ _ =>
      body.eventHasMagnitude n
  | _ + 1, .dealDamage ..
  | _ + 1, .changeLife (.up _) | _ + 1, .changeLife (.down _)
  | _ + 1, .pay .. | _ + 1, .draw .. | _ + 1, .putCounters .. | _ + 1, .removeCounters ..
  | _ + 1, .createObject .. => true
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
  | _ + 1, .beginningOf .. | _ + 1, .endOf .. | _ + 1, .now | _ + 1, .stateHolds .. => {}
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

def deedRoleOf (v : Deed) (isAgent : Bool) : DeedRole :=
  if isAgent then (deedFacts v).elim noRole (·.agentRole)
  else (deedFacts v).elim noRole (·.patientRole)

def deedKindOk (v : Deed) (isAgent : Bool) (k : Kind) : Bool :=
  (deedRoleOf v isAgent).sort.elim false (·.domain.admits k)

def deedAgentKind (v : Deed) : Kind :=
  if !deedKindOk v true .player && deedKindOk v true .object then .object else .player

def deedAnimatedOk (dr : DeedRole) (isAgent : Bool) (t : CardType) : Bool :=
  if isAgent then dr.types.elem .creature && t.permanent else false

def deedTypeOk (v : Deed) (isAgent : Bool) (t : CardType) : Bool :=
  (deedRoleOf v isAgent).types.elem t || deedAnimatedOk (deedRoleOf v isAgent) isAgent t

def deedBareOk (v : Deed) (isAgent : Bool) : Bool := (deedRoleOf v isAgent).bare

def deedAltOk (v : Deed) (isAgent : Bool) : List CardType → Bool
  | [] => deedBareOk v isAgent
  | ts => ts.any (deedTypeOk v isAgent)

def deedHeadTysOk (v : Deed) (isAgent : Bool) : List (List CardType) → Bool
  | [] => deedBareOk v isAgent
  | alts => alts.all (deedAltOk v isAgent)

def deedZoneOf (v : Deed) (isAgent : Bool) : Option Zone := (deedRoleOf v isAgent).zone
def knownActs (ds : List Deed) : Bool := ds.all knownAct

def distinctDeeds : List Deed → Bool
  | [] => true
  | d :: ds => !ds.elem d && distinctDeeds ds

def deedClassOk (v : Deed) (isAgent : Bool) (ab : Bool) : Bool :=
  let classes := (deedRoleOf v isAgent).sort.elim [] (·.classes)
  if classes.isEmpty then !ab
  else if ab then classes.elem .ability
  else classes.any (· != .ability)

def deedFits (ds : List Deed) (isAgent : Bool) (k : Kind) (ab : Bool) (ts : List (List CardType))
    (z : Option Zone) : Bool :=
  ds.all fun d =>
    deedKindOk d isAgent k && deedClassOk d isAgent ab && deedHeadTysOk d isAgent ts &&
      zoneFits z (deedZoneOf d isAgent)

def Zone.possessable : Zone → Bool
  | .hand | .graveyard | .library => true
  | _ => false

def LinkedFacet.readsPlayer : LinkedFacet → Bool
  | .chosenPlayer _ => true
  | _ => false

def LinkedFacet.readsObjects : LinkedFacet → Bool
  | .objects .. => true
  | _ => false

def LinkedFacet.readsEntity (f : LinkedFacet) : Bool := f.readsPlayer || f.readsObjects

def LinkedFacet.entityKind : LinkedFacet → Kind
  | .objects .. => .object
  | _ => .player

def LinkedFacet.plur : LinkedFacet → Plurality
  | .objects _ _ p => p
  | _ => .one

def LinkedFacet.zone : LinkedFacet → Option Zone
  | .objects _ deed _ => actDestOf deed
  | _ => none

def LinkedFacet.binding (f : LinkedFacet) : Referent :=
  if f.readsObjects then { det := .the, plur := f.plur, payload := .object [] f.zone none none none none }
  else { det := .the, plur := .one, payload := .player false }

def kindBinding (k : Kind) : Referent :=
  if k == .player then { det := .the, plur := .one, payload := .player false }
  else { det := .the, plur := .one, payload := .object [] none none none none none }

end Semantics
