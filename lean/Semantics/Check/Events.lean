import Semantics.Check.Words
import Semantics.Abilities

/-!
# Semantics.Check.Events

The event facts table, closed event head-gating (`does`, `would`, `did`), and the lookback,
deed, and play-window rules (`docs/decisions/semantics-v3.md` §3.6–§3.8).
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
event-capable `Action` heads (Table 3.6A) plus CR §701 keyword-action definitions wrapped in
`ownScope (some (.action name))`, refusing compound or control-flow actions (`sequentially`,
`doIf`, `doForEach`, `chooseModes`, `establish`, etc.). -/
def Action.eventHeadOk (e : Action) (fuel : Nat := 16) : Bool :=
  match fuel, e with
  | 0, _ => false
  | n + 1, .inCaller body | n + 1, .bracketed body => body.eventHeadOk n
  | _ + 1, .ownScope (some _) _ => true
  | n + 1, .ownScope none body => body.eventHeadOk n
  | _ + 1, .enact .. => true
  | _ + 1, .dealDamage .. | _ + 1, .markDamage .. | _ + 1, .clearDamage ..
  | _ + 1, .setStatus .. | _ + 1, .turnOver .. | _ + 1, .turnFaceDown ..
  | _ + 1, .combat .. | _ + 1, .attack .. | _ + 1, .block .. | _ + 1, .attackUnblocked .. | _ + 1, .attackWith ..
  | _ + 1, .target .. | _ + 1, .tapForMana .. | _ + 1, .commitCrime | _ + 1, .attachment .. | _ + 1, .changeControl ..
  | _ + 1, .trigger | _ + 1, .crew .. | _ + 1, .saddle .. | _ + 1, .fullyUnlock ..
  | _ + 1, .gainDesignation .. | _ + 1, .unlock .. | _ + 1, .setGameDesignation .. | _ + 1, .conclude ..
  | _ + 1, .choose .. | _ + 1, .vote .. | _ + 1, .copy .. | _ + 1, .changeLife .. | _ + 1, .addMana .. | _ + 1, .draw ..
  | _ + 1, .expose .. | _ + 1, .search .. | _ + 1, .shuffle .. | _ + 1, .flipCoins .. | _ + 1, .rollDice ..
  | _ + 1, .createObject .. | _ + 1, .putCounters .. | _ + 1, .distribute .. | _ + 1, .removeCounters ..
  | _ + 1, .moveCounters .. | _ + 1, .doubleCounters ..
  | _ + 1, .move .. | _ + 1, .pay .. | _ + 1, .procedure .. | _ + 1, .relate .. | _ + 1, .merge ..
  | _ + 1, .skipUntap .. | _ + 1, .skipPart .. | _ + 1, .addTurnPart .. | _ + 1, .exchange .. => true
  | _ + 1, _ => false
termination_by structural fuel

/-- The closed action heads which a deontic clause can constrain. -/
def Action.deonticHeadOk (e : Action) (fuel : Nat := 16) : Bool :=
  match fuel, e with
  | 0, _ => false
  | n + 1, .inCaller body | n + 1, .bracketed body => body.deonticHeadOk n
  | _ + 1, .ownScope (some (.action _)) _ => true
  | n + 1, .ownScope none body => body.deonticHeadOk n
  | _ + 1, .attack .. | _ + 1, .block .. | _ + 1, .target .. | _ + 1, .attachment ..
  | _ + 1, .procedure .. | _ + 1, .copy .. | _ + 1, .draw .. | _ + 1, .changeLife ..
  | _ + 1, .dealDamage .. | _ + 1, .putCounters .. | _ + 1, .removeCounters ..
  | _ + 1, .search .. | _ + 1, .turnOver .. | _ + 1, .conclude ..
  | _ + 1, .changeControl .. | _ + 1, .tapForMana .. | _ + 1, .unlock ..
  | _ + 1, .trigger | _ + 1, .crew .. | _ + 1, .saddle .. | _ + 1, .fullyUnlock ..
  | _ + 1, .vote .. | _ + 1, .pay .. => true
  | _ + 1, .setStatus .tapped _ | _ + 1, .setStatus .untapped _
  | _ + 1, .setStatus .phasedIn _ | _ + 1, .setStatus .phasedOut _ => true
  | _ + 1, .move _ _ (some (.zone .battlefield _)) _ _ _ => true
  | _, _ => false
termination_by structural fuel

/-- Whether an event-capable action carries a quantity or magnitude that `TallyMode.quantity`
can sum (`semantics-v3.md` §3.8). -/
def Action.eventHasMagnitude (e : Action) (fuel : Nat := 16) : Bool :=
  match fuel, e with
  | 0, _ => false
  | n + 1, .inCaller body | n + 1, .bracketed body | n + 1, .ownScope _ body | n + 1, .enact _ body _ _ =>
      body.eventHasMagnitude n
  | _ + 1, .dealDamage .. | _ + 1, .markDamage ..
  | _ + 1, .changeLife (.up _) | _ + 1, .changeLife (.down _)
  | _ + 1, .pay .. | _ + 1, .draw .. | _ + 1, .putCounters .. | _ + 1, .removeCounters ..
  | _ + 1, .createObject .. | _ + 1, .addMana .. => true
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
  | _ + 1, .beginningOf .. | _ + 1, .endOf .. | _ + 1, .now | _ + 1, .stateHolds .. | _ + 1, .statBecomes ..
  | _ + 1, .gameBecomes .. | _ + 1, .chapterMark .. | _ + 1, .triggers .. | _ + 1, .stateBasedActionsChecked => {}
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

def knownActs (ds : List Deed) : Bool := ds.all knownAct

def distinctDeeds : List Deed → Bool
  | [] => true
  | d :: ds => !ds.elem d && distinctDeeds ds

/-! ## Linked Facets -/

/-- What a facet reads: a player. -/
def LinkedFacet.readsPlayer : LinkedFacet → Bool
  | .chosenPlayer _ => true
  | _ => false

/-- The facet reads the objects a deed acted on. -/
def LinkedFacet.readsObjects : LinkedFacet → Bool
  | .objects .. => true
  | _ => false

/-- The facet reads an entity, which a noun phrase denotes: a player or objects. -/
def LinkedFacet.readsEntity (f : LinkedFacet) : Bool := f.readsPlayer || f.readsObjects

/-- The kind of entity a noun-phrase facet denotes. -/
def LinkedFacet.entityKind : LinkedFacet → Kind
  | .objects .. => .object
  | _ => .player

/-- The number a noun-phrase facet denotes: one chosen player, or the objects as the text
numbers them. -/
def LinkedFacet.plur : LinkedFacet → Plurality
  | .objects _ _ p => p
  | _ => .one

/-- The zone the objects a facet reads were put into by its deed ("the exiled card" is in
exile); none for a facet that reads a player or an amount. -/
def LinkedFacet.zone : LinkedFacet → Option Zone
  | .objects _ deed _ => actDestOf deed
  | _ => none

/-- The referent a noun-phrase facet introduces: the chosen player, or the objects its deed acted
on, as many as the text says. -/
def LinkedFacet.binding (f : LinkedFacet) : Referent :=
  if f.readsObjects then { det := .the, plur := f.plur, payload := .object [] f.zone none none none }
  else { det := .the, plur := .one, payload := .player false }

/-- A definite referent of one entity of kind `k`: a player, or an object. -/
def kindBinding (k : Kind) : Referent :=
  if k == .player then { det := .the, plur := .one, payload := .player false }
  else { det := .the, plur := .one, payload := .object [] none none none none }

end Semantics
