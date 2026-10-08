import Semantics.Triggers
import Semantics.Check.Phrase

/-!
# Semantics.Check.EventContext

Event and trigger context profiles, shared contexts for joined headers, and event-field
properties. The recursive event rules live with the phrase checker in `Check/PhraseRules`.
-/

namespace Semantics

def Door.intro (bs : Bindings) : Door → Bindings
  | .thisDoor => bs
  | .doorOf _ room => nomIntro bs room

def creationVoiceOk (bs : Bindings) (byEffect : Bool) (by_ under : Option NounPhrase) : Bool :=
  (match by_ with
   | none => true
   | some w => (NounPhrase.introduced bs w).isEmpty) &&
  (match under with
   | none => true
   | some u => u.plur.isOne && (NounPhrase.introduced bs u).isEmpty) &&
  !(byEffect && by_.isSome)

def causedByOk (byEffect : Bool) (by_ : Option NounPhrase) : Bool := !(byEffect && by_.isSome)

def optIntro (bs : Bindings) : Option NounPhrase → Bindings
  | none => bs
  | some m => nomIntro bs m

/-- Idris `Triggers.agentIntro`, not the phrase layer's: a plain `nomIntro` of the agent. -/
def optAgentIntro (bs : Bindings) : Option NounPhrase → Bindings
  | none => bs
  | some who => nomIntro bs who

def verbPatientOk (v : Deed) : Option NounPhrase → Bool
  | none => (actPatientKindsOf v).isEmpty
  | some _ => (actPatientKindsOf v).elem .object

def verbBecomesOk (v : Deed) : Option Predicate → Bool
  | none => true
  | some p => actIntransitiveOf v && p.says

def verbedVoiceOk (v : Deed) : Option NounPhrase → Option NounPhrase → Bool
  | some _, _ => true
  | none, what => what.isSome && (actNamesParticiple v || actIntransitiveOf v)

def HeaderPossessor.intro (bs : Bindings) : HeaderPossessor → Bindings
  | .noPossessor => bs
  | .byPlayer n => NounPhrase.selfSubjIntroduced n ++ agentIntro bs n
  | .byTurn _ => bs


mutual
/-- The stack a trigger's body reads while the event is happening. -/
def GameEvent.intro (bs : Bindings) : GameEvent → Bindings
  | .withBindings scope inputs body => closeOperands (body.intro (captureBindings bs scope inputs))
  | .inCaller scope body => leaveCaller bs (body.intro (enterCaller scope bs))
  | .damage _ none none => bs
  | .zoneChange n src dest observation =>
    let origin := (sourceZone src).orElse (fun _ =>
      if observation == .before then NounPhrase.zone bs n else none)
    let subjects := moveIntro bs none n origin
    let origins := OptEventSource.introduced subjects src ++ subjects
    OptZoneExpr.introduced origins dest ++ origins
  | .damage _ none (some to) => outcomeB .damageDealt :: selfSubjIntro bs to
  | .draws who => selfSubjIntro bs who
  | .losesGame who => selfSubjIntro bs who
  | .combat _ n none => selfSubjIntro bs n
  | .combat _ n (some m) => selfSubjIntro (nomIntro bs n) m
  | .attacksWith who whom attackers => selfSubjIntro (optIntro (nomIntro bs who) whom) attackers
  | .attachment _ n host => selfSubjIntro (nomIntro bs n) host
  | .damage _ (some n) none => outcomeB .damageDealt :: selfSubjIntro bs n
  | .damage _ (some n) (some m) => outcomeB .damageDealt :: selfSubjIntro (nomIntro bs n) m
  | .beginningOf _ _ _ => bs
  | .casts who none _ => selfSubjIntro bs who
  | .casts who (some what) _ => selfSubjIntro (nomIntro bs who) what
  | .becomesTarget n by_ => selfSubjIntro (nomIntro bs n) by_
  | .statusEvent n _ => selfSubjIntro bs n
  | .gameBecomes _ => bs
  | .stateHolds _ => bs
  | .counterEvent _ _ n .one _ _ => selfSubjIntro bs n
  | .counterEvent _ _ n .many _ _ => outcomeB .countersPut :: selfSubjIntro bs n
  | .counterEvent _ _ n .emptying _ _ => selfSubjIntro bs n
  | .tokensCreated n _ _ _ => selfSubjIntro bs n
  | .chapterMark _ => bs
  | .activates who what => selfSubjIntro (nomIntro bs who) what
  | .statBecomes n _ v => Amount.intro (nomIntro bs n) v
  | .flipsCoin who _ => selfSubjIntro bs who
  | .rollsDice who .one _ _ => selfSubjIntro bs who
  | .rollsDice who .many _ _ => outcomeB .diceRolled :: selfSubjIntro bs who
  | .paysCost who _ whose _ => selfSubjIntro (optAgentIntro bs who) whose
  | .paysLife who => selfSubjIntro bs who
  | .lifeChanges who dir => outcomeB dir.outcome :: selfSubjIntro bs who
  | .verbedEvent who _ none _ _ => optAgentIntro bs who
  | .verbedEvent who _ (some what) _ _ => selfSubjIntro (optAgentIntro bs who) what
  | .tappedForMana who what _ => selfSubjIntro (optAgentIntro bs who) what
  | .unlocksDoor who door => door.intro (nomIntro bs who)
  | .nthOccurrence _ _ ev => GameEvent.intro bs ev
  | .triggers what => selfSubjIntro bs what
  | .commitsCrime who => selfSubjIntro bs who
  | .causes by_ what => GameEvent.intro (Causing.intro bs by_) what
termination_by structural ev => ev

/-- The stack a trigger's body reads after the event has happened. -/
def GameEvent.after (bs : Bindings) : GameEvent → Bindings
  | .withBindings scope inputs body => closeOperands (body.after (captureBindings bs scope inputs))
  | .inCaller scope body => leaveCaller bs (body.after (enterCaller scope bs))
  | .damage _ none none => bs
  | .zoneChange n src to _ =>
    let subjects := moveIntro bs none n (to.bind ZoneExpr.sort)
    let origins := OptEventSource.introduced subjects src ++ subjects
    OptZoneExpr.introduced origins to ++ origins
  | .damage _ none (some to) => outcomeB .damageDealt :: selfSubjIntro bs to
  | .draws who => nomIntro bs who
  | .losesGame who => nomIntro bs who
  | .combat .attackerOf n (some whom) =>
    NounPhrase.introduced (nomIntro bs n) whom ++ selfSubjIntro bs n
  | .combat _ n none => selfSubjIntro bs n
  | .combat _ n (some m) => nomIntro (nomIntro bs n) m
  | .attacksWith who whom attackers => nomIntro (optIntro (nomIntro bs who) whom) attackers
  | .attachment _ n host => nomIntro (nomIntro bs n) host
  | .damage _ (some n) none => outcomeB .damageDealt :: selfSubjIntro bs n
  | .damage _ (some n) (some m) => outcomeB .damageDealt :: nomIntro (nomIntro bs n) m
  | .casts who none _ => nomIntro bs who
  | .casts who (some what) _ => nomIntro (nomIntro bs who) what
  | .becomesTarget n by_ => NounPhrase.introduced (nomIntro bs n) by_ ++ selfSubjIntro bs n
  | .beginningOf _ _ whose => whose.intro bs
  | .statusEvent n _ => selfSubjIntro bs n
  | .gameBecomes _ => bs
  | .stateHolds _ => bs
  | .counterEvent _ _ n .one _ _ => selfSubjIntro bs n
  | .counterEvent _ _ n .many _ _ => outcomeB .countersPut :: selfSubjIntro bs n
  | .counterEvent _ _ n .emptying _ _ => selfSubjIntro bs n
  | .tokensCreated n _ _ _ => nomIntro bs n
  | .chapterMark _ => bs
  | .activates who what => nomIntro (nomIntro bs who) what
  | .statBecomes n _ v => Amount.introduced (nomIntro bs n) v ++ selfSubjIntro bs n
  | .flipsCoin who none => outcomeB .coinFlipped :: nomIntro bs who
  | .flipsCoin who (some _) => nomIntro bs who
  | .rollsDice who _ _ _ => outcomeB .rollResult :: nomIntro bs who
  | .paysCost who _ whose _ => nomIntro (optAgentIntro bs who) whose
  | .paysLife who => outcomeB .lifeLost :: nomIntro bs who
  | .lifeChanges who dir => outcomeB dir.outcome :: nomIntro bs who
  | .verbedEvent who _ none _ _ => optAgentIntro bs who
  | .verbedEvent who v (some what) _ _ =>
    let bs' := optAgentIntro bs who
    moveIntro bs' (some v) what ((actDestOf v).elim (NounPhrase.zone bs' what) some)
  | .tappedForMana who what _ =>
    outcomeB .manaProduced :: stampIntro (optAgentIntro bs who) (featureLabel .tapping) what
  | .unlocksDoor who door => door.intro (nomIntro bs who)
  | .nthOccurrence _ _ ev => GameEvent.after bs ev
  | .triggers what => nomIntro bs what
  | .commitsCrime who => nomIntro bs who
  | .causes by_ what => GameEvent.after (Causing.intro bs by_) what
termination_by structural ev => ev

/-- The stack the caused event is spelled in (Idris `causingIntro`). -/
def Causing.intro (bs : Bindings) : Causing → Bindings
  | .source src => nomIntro bs src
  | .event ev => GameEvent.after bs ev
  | .anEffect => bs
termination_by structural c => c
end

def GameEvent.namesThisDoor : GameEvent → Bool
  | .withBindings _ _ body | .inCaller _ body => body.namesThisDoor
  | .unlocksDoor _ .thisDoor => true
  | .nthOccurrence _ _ ev => GameEvent.namesThisDoor ev
  | .causes _ what => GameEvent.namesThisDoor what
  | _ => false

def armsAgree (read : GameEvent → Bindings) (ds : Bindings) (alts : List GameEvent) : Bool :=
  alts.all fun a => ds == read a

def unionArms (read : GameEvent → Bindings) : Bindings → List GameEvent → Option Bindings
  | ds, [] => some ds
  | ds, a :: as =>
    match unionBindings ds (read a) with
    | some u => unionArms read u as
    | none => none

/-- The context an `or`-joined header shares: what the arms agree on, else their union, else
nothing. -/
def sharedCtx (bs : Bindings) (read : GameEvent → Bindings) (alts : List GameEvent)
    (ev : GameEvent) : Bindings :=
  if armsAgree read (read ev) alts then read ev
  else
    match unionArms read (read ev) alts with
    | some u => u
    | none => bs

def delayedCtx (bs : Bindings) (alts : List GameEvent) (ev : GameEvent) : Bindings :=
  settleTargets (sharedCtx bs (GameEvent.after bs) alts ev)

def interceptCtx (bs : Bindings) (alts : List GameEvent) (ev : GameEvent) : Bindings :=
  sharedCtx bs (GameEvent.intro bs) alts ev

def headerCtx (bs : Bindings) (alts : List GameEvent) (ev : GameEvent) : Bindings :=
  sharedCtx bs (GameEvent.after bs) alts ev

def Duration.intro (bs : Bindings) : Duration → Bindings
  | .thisTurn => bs
  | .restOfGame => bs
  | .until_ _ => bs
  | .forAsLongAs c => c.intro bs
  | .untilEvent ev => GameEvent.intro bs ev
  | .duringNextTurnOf who => nomIntro bs who

def JoinedHeader.after (bs : Bindings) (j : JoinedHeader) : Bindings := headerCtx bs j.alternatives
  j.event

def joinedCtx (bs : Bindings) : List JoinedHeader → Bindings → Bindings
  | [], ds => ds
  | j :: js, ds => if ds == j.after bs then joinedCtx bs js ds else bs

def Concurrent.namesThisDoor : Option Concurrent → Bool
  | some (.whileDoing ev) => ev.namesThisDoor
  | _ => false

def JoinedHeader.namesThisDoor (j : JoinedHeader) : Bool :=
  j.event.namesThisDoor || j.alternatives.any GameEvent.namesThisDoor ||
    Concurrent.namesThisDoor j.while_

/-! ## Number slots

Where an event's own `Amount` arguments sit in the two regimes of [CR#107.1b]; see
`Instruction.numberSlots` in `Check/Abilities` for the surface as a whole. -/

/-- The event's own direct `Amount` arguments and the regime each is read in [CR#107.1b].
Only `statBecomes` carries one: "whenever this creature's power becomes 3 or less" watches a
game value that may be below zero, so it is `signed`. Every arm is written out so that a new
event carrying an `Amount` cannot slip in unclassified. -/
def GameEvent.numberSlots : GameEvent → List (Amount × NumberRegime)
  | .withBindings _ _ body | .inCaller _ body => body.numberSlots
  | .zoneChange _ _ _ _ | .draws _ | .losesGame _ => []
  | .combat _ _ _ | .attacksWith _ _ _ | .attachment _ _ _ | .damage _ _ _ => []
  | .beginningOf _ _ _ | .casts _ _ _ | .becomesTarget _ _ | .statusEvent _ _ => []
  | .gameBecomes _ | .stateHolds _ => []
  | .counterEvent _ _ _ _ _ _ | .tokensCreated _ _ _ _ | .chapterMark _ | .activates _ _ => []
  -- A game value, not the result of an effect: it reads below zero [CR#107.1b].
  | .statBecomes _ _ value => [(value, .signed)]
  | .flipsCoin _ _ | .rollsDice _ _ _ _ | .paysCost _ _ _ _ | .paysLife _ => []
  | .lifeChanges _ _ | .verbedEvent _ _ _ _ _ | .tappedForMana _ _ _ | .unlocksDoor _ _ => []
  | .nthOccurrence _ _ _ | .triggers _ | .commitsCrime _ | .causes _ _ => []

/-- Idris `TokenPhrase n`: a counted or indefinite description that seeds tokens. -/
def NounPhrase.tokenPhrase : NounPhrase → Bool
  | .withBindings _ _ body | .inCaller _ body | .firstMention _ body => body.tokenPhrase
  | .described (.count _ _) p => p.seedsToken
  | .described (.a _) p => p.seedsToken
  | _ => false


end Semantics
