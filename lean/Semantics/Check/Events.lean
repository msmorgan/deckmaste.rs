import Semantics.Check.Words
import Semantics.Phrase

/-!
# Semantics.Check.Events

The event facts table and the lookback, deed, and play-window rules of `Events.idr`.
-/

namespace Semantics

/-- Only a removal empties a named kind, so only it has a last counter. -/
def counterBatchOk : CounterBatch → CounterMove → Option CounterKind → Bool
  | .emptying, .removed, kind => kind.isSome
  | .emptying, _, _ => false
  | _, _, _ => true

def LifeMove.outcome : LifeMove → OutcomeSort
  | .up => .lifeGained
  | .down => .lifeLost

structure EventFacts where
  hasMagnitude : Bool := false
  interceptable : Bool := true
  countable : Bool := false
  boundsDuration : Bool := true
  underway : Bool := false
  deriving Repr, BEq

/-- Event properties used by magnitudes, replacements, counts, durations, and concurrency.
Participant and payload obligations are checked directly on the event's fields. -/
def GameEvent.facts : GameEvent → EventFacts
  | .withBindings _ _ body | .inCaller _ body => body.facts
  | .damage _ _ _ | .lifeChanges _ _ | .paysLife _ =>
    { hasMagnitude := true }
  | .beginningOf _ _ _ => { boundsDuration := false }
  | .casts _ _ _ | .activates _ _ => { underway := true }
  | .chapterMark _ => { interceptable := false }
  | .triggers _ => { interceptable := false, countable := true }
  | .verbedEvent _ v _ _ _ => { underway := actStepwiseOf v }
  | .unlocksDoor _ _ => { underway := actStepwiseOf (.core .unlock) }
  | .nthOccurrence _ _ ev | .causes _ ev => ev.facts
  | .zoneChange _ _ _ _ | .draws _ | .losesGame _ | .combat _ _ _
  | .attacksWith _ _ _ | .attachment _ _ _ | .becomesTarget _ _ | .statusEvent _ _
  | .gameBecomes _ | .stateHolds _ | .counterEvent _ _ _ _ _ _
  | .tokensCreated _ _ _ _ | .statBecomes _ _ _ | .flipsCoin _ _ | .rollsDice _ _ _ _
  | .paysCost _ _ _ _ | .tappedForMana _ _ _ | .commitsCrime _ => {}
termination_by structural ev => ev

def GameEvent.scopeBody : GameEvent → GameEvent
  | .withBindings _ _ body | .inCaller _ body => body.scopeBody
  | body => body

def interceptOk (ev : GameEvent) : Bool := ev.facts.interceptable
def triggerCountOk (ev : GameEvent) : Bool := ev.facts.countable
def durationEventOk (ev : GameEvent) : Bool := ev.facts.boundsDuration
def eventUnderwayOk (ev : GameEvent) : Bool := ev.facts.underway
def eventHasMagnitude (ev : GameEvent) : Bool := ev.facts.hasMagnitude

def tallyOk : TallyOp → GameEvent → Bool
  | .count, _ => true
  | .sum, ev => eventHasMagnitude ev

def Role.counter : Role → Role
  | .agent => .patient
  | .patient => .agent

def deedRoleOf (v : Deed) : Role → DeedRole
  | .agent => (deedFacts v).elim noRole (·.agentRole)
  | .patient => (deedFacts v).elim noRole (·.patientRole)

def deedKindOk (v : Deed) (r : Role) (k : Kind) : Bool :=
  (deedRoleOf v r).sort.elim false (·.domain.admits k)

/-- An effect can make a permanent of any type a creature that is still its other types
[CR#205.1b], and an effect written on a noncreature permanent is created even while it isn't
one [CR#208.3a]. So the agent slot of a deed a creature performs fits any permanent head type;
what a deed is done to keeps the set the rules name. -/
def deedAnimatedOk (dr : DeedRole) : Role → CardType → Bool
  | .agent, t => dr.types.elem .creature && t.permanent
  | .patient, _ => false

def deedTypeOk (v : Deed) (r : Role) (t : CardType) : Bool :=
  (deedRoleOf v r).types.elem t || deedAnimatedOk (deedRoleOf v r) r t

def deedBareOk (v : Deed) (r : Role) : Bool := (deedRoleOf v r).bare

def deedAltOk (v : Deed) (r : Role) : List CardType → Bool
  | [] => deedBareOk v r
  | ts => ts.any (deedTypeOk v r)

def deedHeadTysOk (v : Deed) (r : Role) : List (List CardType) → Bool
  | [] => deedBareOk v r
  | alts => alts.all (deedAltOk v r)

def deedZoneOf (v : Deed) (r : Role) : Option Zone := (deedRoleOf v r).zone
/-- The deed a defender is declared against: attacking, the turn-based action [CR#508.1,508.1b]. -/
def deedDefendsOk (v : Deed) : Bool := v == .core .attack
/-- The deed that makes an object a target [CR#115.1]. -/
def deedTargetedOk (v : Deed) : Bool := v == .core .target
def deedPremiseSort (v : Deed) : Option PremiseSort := deedFacts v >>= (·.counterfactual)
def deedPremiseOk (s : PremiseSort) (v : Deed) : Bool := deedPremiseSort v == some s
def deedRidesOk (v : Deed) : Bool := (deedFacts v).elim false (·.rides)
def deedPlaysOk (v : Deed) : Bool := (deedFacts v).elim false (·.plays)
def deedBoundedOk (v : Deed) : Bool := (deedFacts v).elim false (·.bounded)

def knownActs (ds : Deeds) : Bool := ds.all knownAct

def distinctDeeds : Deeds → Bool
  | [] => true
  | d :: ds => !ds.elem d && distinctDeeds ds

/-- Whether a role's noun may be an ability on the stack [CR#113.1c] or must not be one, read off
the classes its sort declares. A role that declares no classes refuses an ability outright — the
two roles an ability on the stack fills (Activate's patient, Trigger's agent) declare `.ability`
and nothing else, so they take an ability and nothing else; a role whose classes include
`.ability` alongside another class, as Counter's and Copy's patients do [CR#113.9], takes either.
Classes other than `.ability` are declared but not yet checked against the noun beyond this
ability/non-ability split (the zone field carries permanent-ness today). -/
def deedClassOk (v : Deed) (r : Role) (ab : Bool) : Bool :=
  let classes := (deedRoleOf v r).sort.elim [] (·.classes)
  if classes.isEmpty then !ab
  else if ab then classes.elem .ability
  else classes.any (· != .ability)

def deedFits (ds : Deeds) (r : Role) (k : Kind) (ab : Bool) (ts : List (List CardType))
    (z : Option Zone) : Bool :=
  ds.all fun d =>
    deedKindOk d r k && deedClassOk d r ab && deedHeadTysOk d r ts &&
      zoneFits z (deedZoneOf d r)

def playWindowOk : Option PlayLimit → Option PlayTiming → Bool
  | _, none => true
  | some .onceEachYourTurn, some .duringEachOfYourTurns => false
  | _, _ => true

def playableFrom : Option Zone → Bool
  | some .stack => false
  | _ => true

def Zone.placementDestOk : Zone → Bool
  | .battlefield | .stack => false
  | _ => true

/-- Idris `Possessable z`: the zones a player owns. -/
def Zone.possessable : Zone → Bool
  | .hand | .graveyard | .library => true
  | _ => false

end Semantics
