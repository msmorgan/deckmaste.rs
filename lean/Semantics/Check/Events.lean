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
  deriving Repr, BEq

/-- Event properties: whether the event carries a magnitude a tally can sum.
Participant and payload obligations are checked directly on the event's fields. -/
def GameEvent.facts : GameEvent → EventFacts
  | .withBindings _ _ body | .inCaller _ body => body.facts
  | .damage _ _ _ | .lifeChanges _ _ | .paysLife _ =>
    { hasMagnitude := true }
  | .nthOccurrence _ _ ev | .causes _ ev => ev.facts
  | .beginningOf _ _ _ | .endOf _ _ _ | .now | .casts _ _ _ | .activates _ _ | .chapterMark _ | .triggers _
  | .verbedEvent _ _ _ _ _ | .unlocksDoor _ _
  | .zoneChange _ _ _ _ | .draws _ | .losesGame _ | .combat _ _ _
  | .attacksWith _ _ _ | .attachment _ _ _ | .becomesTarget _ _ | .statusEvent _ _
  | .gameBecomes _ | .stateHolds _ | .counterEvent _ _ _ _ _ _
  | .tokensCreated _ _ _ _ | .statBecomes _ _ _ | .flipsCoin _ _ | .rollsDice _ _ _ _
  | .paysCost _ _ _ _ | .tappedForMana _ _ _ | .commitsCrime _ => {}
termination_by structural ev => ev

def GameEvent.scopeBody : GameEvent → GameEvent
  | .withBindings _ _ body | .inCaller _ body => body.scopeBody
  | body => body

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

/-- The kind a deed's performer is written as: an object where the deed's row admits only
objects ("whenever a creature you control explores" [CR#701.44a]), otherwise a player. -/
def deedAgentKind (v : Deed) : Kind :=
  if !deedKindOk v .agent .player && deedKindOk v .agent .object then .object else .player

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

/-- Idris `Possessable z`: the zones a player owns. -/
def Zone.possessable : Zone → Bool
  | .hand | .graveyard | .library => true
  | _ => false

end Semantics
