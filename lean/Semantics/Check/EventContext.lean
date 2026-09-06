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

def Door.namesHost : Door → Bool
  | .thisDoor => false
  | .doorOf _ _ => true

def creationVoiceOk (bs : Bindings) (byEffect : Bool) (by_ under : Option NounPhrase) : Bool :=
  (match by_ with
   | none => true
   | some w => (NounPhrase.introduced bs w).isEmpty) &&
  (match under with
   | none => true
   | some u => u.plur.isOne && (NounPhrase.introduced bs u).isEmpty) &&
  !(byEffect && by_.isSome)

def causedByOk (byEffect : Bool) (by_ : Option NounPhrase) : Bool := !(byEffect && by_.isSome)

def putDestOk (z : ZoneExpr) : Bool := z.sort.placementDestOk

def putSourceOk : Option EventSource → Bool
  | none => true
  | some src => src.zonesOk Zone.placementOriginOk

def castSourceOk : Option EventSource → Bool
  | none => true
  | some src => src.zonesOk (fun z => playableFrom (some z))

def entrySourceOk : Option EventSource → Bool
  | none => true
  | some src => src.zonesOk Zone.entryOriginOk

/-- The combat relations that name an event: attacking, blocking, becoming blocked. -/
def CombatRelation.eventRelation : CombatRelation → Bool
  | .attackerOf | .blockerOf | .blockedBy => true
  | _ => false

def optIntro (bs : Bindings) : Option NounPhrase → Bindings
  | none => bs
  | some m => nomIntro bs m

/-- Idris `Triggers.agentIntro`, not the phrase layer's: a plain `nomIntro` of the agent. -/
def optAgentIntro (bs : Bindings) : Option NounPhrase → Bindings
  | none => bs
  | some who => nomIntro bs who

def patientZone (bs : Bindings) : Option NounPhrase → Option Zone
  | none => none
  | some n => NounPhrase.zone bs n

def patientZoneIsB (bs : Bindings) : Option NounPhrase → Zone → Bool
  | none, _ => true
  | some n, z => zoneIsB (NounPhrase.zone bs n) z

def verbPatientOk (v : Deed) : Option NounPhrase → Bool
  | none => (actPatientKindsOf v).isEmpty
  | some _ => (actPatientKindsOf v).elem .object

def verbBecomesOk (v : Deed) : Option Predicate → Bool
  | none => true
  | some p => actIntransitiveOf v && p.says

def verbedVoiceOk (v : Deed) : Option NounPhrase → Option NounPhrase → Bool
  | some _, _ => true
  | none, what => what.isSome && (actNamesParticiple v || actIntransitiveOf v)

def DamageKind.sourceZone : DamageKind → Option Zone
  | .any => none
  | .combatOnly => some .battlefield
  | .noncombatOnly => none

def HeaderPossessor.intro (bs : Bindings) : HeaderPossessor → Bindings
  | .noPossessor => bs
  | .byPlayer n => NounPhrase.selfSubjIntroduced n ++ agentIntro bs n
  | .byTurn _ => bs

def HeaderPossessor.ok : HeaderPossessor → Bool
  | .noPossessor => true
  | .byPlayer n => partPossessorOk (some n)
  | .byTurn _ => true


mutual
/-- The stack a trigger's body reads while the event is happening. -/
def GameEvent.intro (bs : Bindings) : GameEvent → Bindings
  | .damage _ none none => bs
  | .dies n => selfSubjIntro bs n
  | .leaves n _ => selfSubjIntro bs n
  | .damage _ none (some to) => outcomeB .damageDealt :: selfSubjIntro bs to
  | .draws who => selfSubjIntro bs who
  | .losesGame who => selfSubjIntro bs who
  | .enters n _ => selfSubjIntro bs n
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
  | .putInto n _ _ => selfSubjIntro bs n
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
  | .damage _ none none => bs
  | .dies n => moveIntro bs none n (some .graveyard)
  | .leaves n _ => moveIntro bs none n none
  | .damage _ none (some to) => outcomeB .damageDealt :: selfSubjIntro bs to
  | .draws who => nomIntro bs who
  | .losesGame who => nomIntro bs who
  | .enters n _ => moveIntro bs none n (some .battlefield)
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
  | .putInto n to _ => moveIntro bs none n (some to.sort)
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

def interceptArmsOk (alts : List GameEvent) : Bool := alts.all fun a => interceptOk a

def Duration.ok : Duration → Bool
  | .untilEvent ev => durationEventOk ev
  | _ => true

def Duration.intro (bs : Bindings) : Duration → Bindings
  | .thisTurn => bs
  | .restOfGame => bs
  | .until_ _ => bs
  | .forAsLongAs c => c.intro bs
  | .untilEvent ev => GameEvent.intro bs ev
  | .duringNextTurnOf who => nomIntro bs who

def durationOk : Option Duration → Bool
  | none => true
  | some d => d.ok

def untriggeredLimitOk : Option UsageLimit → Bool
  | some .actionOncePerTurn => false
  | _ => true

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

def chapterDefaultsOk (ev : GameEvent) (alts : List GameEvent) (wh : Option Concurrent)
    (joins : List JoinedHeader) (w : Option Timing) (l : Option UsageLimit)
    (i : Option Condition) : Bool :=
  match ev with
  | .chapterMark _ =>
    alts.isEmpty && wh.isNone && joins.isEmpty && w.isNone && l.isNone && i.isNone
  | _ => true

def GameEvent.headerNontarget (bs : Bindings) (ev : GameEvent) : Bool :=
  !anyTargetedAt (GameEvent.intro bs ev)

def GameEvent.headerStatusOk : GameEvent → Bool
  | .statusEvent _ v => v.markable
  | _ => true

/-! ## Number slots

Where an event's own `Amount` arguments sit in the two regimes of [CR#107.1b]; see
`Instruction.numberSlots` in `Check/Abilities` for the surface as a whole. -/

/-- The event's own direct `Amount` arguments and the regime each is read in [CR#107.1b].
Only `statBecomes` carries one: "whenever this creature's power becomes 3 or less" watches a
game value that may be below zero, so it is `signed`. Every arm is written out so that a new
event carrying an `Amount` cannot slip in unclassified. -/
def GameEvent.numberSlots : GameEvent → List (Amount × NumberRegime)
  | .dies _ | .leaves _ _ | .draws _ | .losesGame _ | .enters _ _ => []
  | .combat _ _ _ | .attacksWith _ _ _ | .attachment _ _ _ | .damage _ _ _ => []
  | .beginningOf _ _ _ | .casts _ _ _ | .becomesTarget _ _ | .statusEvent _ _ => []
  | .gameBecomes _ | .stateHolds _ | .putInto _ _ _ => []
  | .counterEvent _ _ _ _ _ _ | .tokensCreated _ _ _ _ | .chapterMark _ | .activates _ _ => []
  -- A game value, not the result of an effect: it reads below zero [CR#107.1b].
  | .statBecomes _ _ value => [(value, .signed)]
  | .flipsCoin _ _ | .rollsDice _ _ _ _ | .paysCost _ _ _ _ | .paysLife _ => []
  | .lifeChanges _ _ | .verbedEvent _ _ _ _ _ | .tappedForMana _ _ _ | .unlocksDoor _ _ => []
  | .nthOccurrence _ _ _ | .triggers _ | .commitsCrime _ | .causes _ _ => []

/-- Idris `TokenPhrase n`: a counted or indefinite description that seeds tokens. -/
def NounPhrase.tokenPhrase : NounPhrase → Bool
  | .described (.count _ _) p => p.seedsToken
  | .described (.a _) p => p.seedsToken
  | _ => false


end Semantics
