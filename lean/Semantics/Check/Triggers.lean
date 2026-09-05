import Semantics.Triggers
import Semantics.Check.PhraseRules

/-!
# Semantics.Check.Triggers

The trigger layer of the checker: what an event introduces (`eventIntro`, `eventAfter`),
the shared context of an `or`-joined header, and one rule set per event constructor. Port of
the functions of `Triggers.idr`.
-/

namespace Semantics

def StatusCat.eventName : StatusCat → EventName
  | .tap => .statusChange
  | .flip => .statusChange
  | .face => .turnedFaceUp
  | .phase => .phasingChange

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

def putDestOk (z : ZoneExpr) : Bool := lookbackDestOk .placement z.sort

def putSourceOk : Option EventSource → Bool
  | none => true
  | some src => lookbackSourceOk .placement src

def entrySourceOk : Option EventSource → Bool
  | none => true
  | some src => lookbackSourceOk .entry src

/-- The combat relations that name an event: attacking, blocking, becoming blocked. -/
def CombatRelation.eventRelation : CombatRelation → Bool
  | .attackerOf | .blockerOf | .blockedBy => true
  | _ => false

def CombatRelation.eventName : CombatRelation → EventName
  | .attackerOf | .declaredAttacker | .attackedBy => .attackDeclaration
  | .blockerOf | .couldBlock => .blockDeclaration
  | .blockedBy | .couldBeBlockedBy => .blockedDeclaration

def AttachMove.eventName : AttachMove → EventName
  | .attached => .attachment
  | .unattached => .unattachment

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

def GameEvent.name : GameEvent → EventName
  | .dies _ => .death
  | .leaves _ _ => .departure
  | .isDealtDamage _ _ => .damageTaken
  | .draws _ => .cardDrawn
  | .losesGame _ => .gameLoss
  | .enters _ _ => .entry
  | .combat r _ _ => r.eventName
  | .attacksWith _ _ _ => .attackDeclaration
  | .attachment move _ _ => move.eventName
  | .dealsDamage .combatOnly _ _ => .combatDamage
  | .dealsDamage _ _ _ => .damageDealing
  | .beginningOf _ _ _ => .partBeginning
  | .casts _ _ _ => .spellCast
  | .becomesTarget _ _ => .becomesTarget
  | .statusEvent _ v => v.category.eventName
  | .gameBecomes _ => .gameDesignation
  | .stateHolds _ => .stateMatch
  | .putInto _ _ _ => .placement
  | .counterEvent dir _ _ batch _ _ => counterEventName dir batch
  | .tokensCreated _ _ _ _ => .tokenCreation
  | .chapterMark _ => .chapterArrival
  | .activates _ _ => .abilityActivation
  | .statBecomes _ _ _ => .statValueChange
  | .flipsCoin _ none => .coinFlip
  | .flipsCoin _ (some call) => call.eventName
  | .rollsDice _ _ _ _ => .diceRoll
  | .paysCost _ out _ _ => out.eventName
  | .paysLife _ => .lifePayment
  | .lifeChanges _ dir => dir.eventName
  | .verbedEvent _ v _ _ => .verbedAct v
  | .tappedForMana _ _ _ => .tappedForMana
  | .unlocksDoor _ _ => .verbedAct (.core .unlock)
  | .nthOccurrence _ _ ev => GameEvent.name ev
  | .triggers _ => .abilityTrigger
  | .commitsCrime _ => .crimeCommission
  | .causes _ what => GameEvent.name what
termination_by structural ev => ev

/-- The stack a trigger's body reads while the event is happening. -/
def GameEvent.intro (bs : Bindings) : GameEvent → Bindings
  | .dies n => selfSubjIntro bs n
  | .leaves n _ => selfSubjIntro bs n
  | .isDealtDamage _ to => outcomeB .damageDealt :: selfSubjIntro bs to
  | .draws who => selfSubjIntro bs who
  | .losesGame who => selfSubjIntro bs who
  | .enters n _ => selfSubjIntro bs n
  | .combat _ n none => selfSubjIntro bs n
  | .combat _ n (some m) => selfSubjIntro (nomIntro bs n) m
  | .attacksWith who whom attackers => selfSubjIntro (optIntro (nomIntro bs who) whom) attackers
  | .attachment _ n host => selfSubjIntro (nomIntro bs n) host
  | .dealsDamage _ n none => outcomeB .damageDealt :: selfSubjIntro bs n
  | .dealsDamage _ n (some m) => outcomeB .damageDealt :: selfSubjIntro (nomIntro bs n) m
  | .beginningOf _ _ _ => bs
  | .casts who what _ => selfSubjIntro (nomIntro bs who) what
  | .becomesTarget n by_ => selfSubjIntro (nomIntro bs n) by_
  | .statusEvent n _ => selfSubjIntro bs n
  | .gameBecomes _ => bs
  | .stateHolds _ => bs
  | .putInto n _ _ => selfSubjIntro bs n
  | .counterEvent _ _ n .one _ _ => selfSubjIntro bs n
  | .counterEvent _ _ n .many _ _ => outcomeB .countersPut :: selfSubjIntro bs n
  | .counterEvent _ _ n .last _ _ => selfSubjIntro bs n
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
  | .verbedEvent who _ none _ => optAgentIntro bs who
  | .verbedEvent who _ (some what) _ => selfSubjIntro (optAgentIntro bs who) what
  | .tappedForMana who what _ => selfSubjIntro (optAgentIntro bs who) what
  | .unlocksDoor who door => door.intro (nomIntro bs who)
  | .nthOccurrence _ _ ev => GameEvent.intro bs ev
  | .triggers what => selfSubjIntro bs what
  | .commitsCrime who => selfSubjIntro bs who
  | .causes _ what => GameEvent.intro bs what
termination_by structural ev => ev

/-- The stack a trigger's body reads after the event has happened. -/
def GameEvent.after (bs : Bindings) : GameEvent → Bindings
  | .dies n => moveIntro bs none n (some .graveyard)
  | .leaves n _ => moveIntro bs none n none
  | .isDealtDamage _ to => outcomeB .damageDealt :: selfSubjIntro bs to
  | .draws who => nomIntro bs who
  | .losesGame who => nomIntro bs who
  | .enters n _ => moveIntro bs none n (some .battlefield)
  | .combat .attackerOf n (some whom) =>
    NounPhrase.introduced (nomIntro bs n) whom ++ selfSubjIntro bs n
  | .combat _ n none => selfSubjIntro bs n
  | .combat _ n (some m) => nomIntro (nomIntro bs n) m
  | .attacksWith who whom attackers => nomIntro (optIntro (nomIntro bs who) whom) attackers
  | .attachment _ n host => nomIntro (nomIntro bs n) host
  | .dealsDamage _ n none => outcomeB .damageDealt :: selfSubjIntro bs n
  | .dealsDamage _ n (some m) => outcomeB .damageDealt :: nomIntro (nomIntro bs n) m
  | .casts who what _ => nomIntro (nomIntro bs who) what
  | .becomesTarget n by_ => NounPhrase.introduced (nomIntro bs n) by_ ++ selfSubjIntro bs n
  | .beginningOf _ _ whose => whose.intro bs
  | .statusEvent n _ => selfSubjIntro bs n
  | .gameBecomes _ => bs
  | .stateHolds _ => bs
  | .putInto n to _ => moveIntro bs none n (some to.sort)
  | .counterEvent _ _ n .one _ _ => selfSubjIntro bs n
  | .counterEvent _ _ n .many _ _ => outcomeB .countersPut :: selfSubjIntro bs n
  | .counterEvent _ _ n .last _ _ => selfSubjIntro bs n
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
  | .verbedEvent who _ none _ => optAgentIntro bs who
  | .verbedEvent who v (some what) _ =>
    let bs' := optAgentIntro bs who
    moveIntro bs' (some v) what ((actDestOf v).elim (NounPhrase.zone bs' what) some)
  | .tappedForMana who what _ =>
    outcomeB .manaProduced :: stampIntro (optAgentIntro bs who) (featureLabel .tapping) what
  | .unlocksDoor who door => door.intro (nomIntro bs who)
  | .nthOccurrence _ _ ev => GameEvent.after bs ev
  | .triggers what => nomIntro bs what
  | .commitsCrime who => nomIntro bs who
  | .causes _ what => GameEvent.after bs what
termination_by structural ev => ev

def Causing.intro (bs : Bindings) : Causing → Bindings
  | .source src => nomIntro bs src
  | .event ev => GameEvent.after bs ev
  | .anEffect => bs

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

def interceptArmsOk (alts : List GameEvent) : Bool := alts.all fun a => interceptOk a.name

def Duration.ok : Duration → Bool
  | .untilEvent ev => durationEventOk ev.name
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

def JoinedHeader.after (bs : Bindings) (j : JoinedHeader) : Bindings := headerCtx bs j.alternatives j.event

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
  | .dies _ | .leaves _ _ | .isDealtDamage _ _ | .draws _ | .losesGame _ | .enters _ _ => []
  | .combat _ _ _ | .attacksWith _ _ _ | .attachment _ _ _ | .dealsDamage _ _ _ => []
  | .beginningOf _ _ _ | .casts _ _ _ | .becomesTarget _ _ | .statusEvent _ _ => []
  | .gameBecomes _ | .stateHolds _ | .putInto _ _ _ => []
  | .counterEvent _ _ _ _ _ _ | .tokensCreated _ _ _ _ | .chapterMark _ | .activates _ _ => []
  -- A game value, not the result of an effect: it reads below zero [CR#107.1b].
  | .statBecomes _ _ value => [(value, .signed)]
  | .flipsCoin _ _ | .rollsDice _ _ _ _ | .paysCost _ _ _ _ | .paysLife _ => []
  | .lifeChanges _ _ | .verbedEvent _ _ _ _ | .tappedForMana _ _ _ | .unlocksDoor _ _ => []
  | .nthOccurrence _ _ _ | .triggers _ | .commitsCrime _ | .causes _ _ => []

/-- Idris `TokenPhrase n`: a counted or indefinite description that seeds tokens. -/
def NounPhrase.tokenPhrase : NounPhrase → Bool
  | .described (.count _ _) p => p.seedsToken
  | .described (.a _) p => p.seedsToken
  | _ => false

/-! ## Rules -/

def AttackDefender.check (bs : Bindings) : Option NounPhrase → List Refusal
  | none => []
  | some m =>
    NounPhrase.check none bs m ++ refuse m.plur.isOne .singular ++ refuse (m.attackable bs) .attackable

def DamagePatient.check (bs : Bindings) : Option NounPhrase → List Refusal
  | none => []
  | some m => NounPhrase.check none bs m ++ refuse (m.damageRecipient bs) .damageRecipient

def Door.check (bs : Bindings) : Door → List Refusal
  | .thisDoor => []
  | .doorOf _ room =>
    NounPhrase.check (some .object) bs room ++
      refuse (zoneIsB (NounPhrase.zone bs room) .battlefield) (.zoneIs .battlefield)

def RollWatch.check (bs : Bindings) : RollWatch → List Refusal
  | .resultIn q =>
    refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++ refuse q.literal .quantLiteral ++
      Quantity.check bs q
  | _ => []

def HeaderPossessor.check (bs : Bindings) : HeaderPossessor → List Refusal
  | .noPossessor => []
  | .byPlayer n => NounPhrase.check (some .player) bs n
  | .byTurn n => NounPhrase.check (some .turnRef) bs n

def ManaTypeTerm.check (bs : Bindings) : ManaTypeTerm → List Refusal
  | .colorless => []
  | .ofColor c => ColorTerm.check bs c

def OptManaTypeTerm.check (bs : Bindings) : Option ManaTypeTerm → List Refusal
  | none => []
  | some t => ManaTypeTerm.check bs t

def OptEventSource.check (bs : Bindings) : Option EventSource → List Refusal
  | none => []
  | some src => EventSource.check bs src

def OptZoneExpr.check (bs : Bindings) : Option ZoneExpr → List Refusal
  | none => []
  | some z => ZoneExpr.check bs z

mutual
  def GameEvent.check (bs : Bindings) : GameEvent → List Refusal
    | .dies n =>
      NounPhrase.check (some .object) bs n ++ refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield)
    | .leaves n from_ =>
      NounPhrase.check (some .object) bs n ++ OptEventSource.check bs from_ ++
        refuse (zoneFits (NounPhrase.zone bs n) (sourceZone from_)) .zoneFits
    | .isDealtDamage _ to => NounPhrase.check none bs to ++ refuse (to.damageRecipient bs) .damageRecipient
    | .draws who => NounPhrase.check (some .player) bs who
    | .losesGame who => NounPhrase.check (some .player) bs who
    | .enters n from_ =>
      NounPhrase.check (some .object) bs n ++ OptEventSource.check bs from_ ++
        refuse (zoneFits (NounPhrase.zone bs n) (some .battlefield)) .zoneFits ++
        refuse (entrySourceOk from_) .lookbackSource
    | .combat .attackerOf n whom =>
      NounPhrase.check none bs n ++ AttackDefender.check (nomIntro bs n) whom ++
        refuse (n.attackerOk bs) .attacker
    | .combat r n counterpart =>
      let bs' := nomIntro bs n
      NounPhrase.check (some .object) bs n ++ OptNoun.check (some .object) bs' counterpart ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield) ++
        refuse (patientZoneIsB bs' counterpart .battlefield) (.zoneIs .battlefield) ++
        refuse r.eventRelation .combatRelOk
    | .attacksWith who whom attackers =>
      let bs' := nomIntro bs who
      let bs'' := optIntro bs' whom
      NounPhrase.check (some .player) bs who ++ AttackDefender.check bs' whom ++
        NounPhrase.check (some .object) bs'' attackers ++
        refuse (zoneIsB (NounPhrase.zone bs'' attackers) .battlefield) (.zoneIs .battlefield)
    | .attachment .attached n host =>
      let kh := host.kindOr .object
      NounPhrase.check (some .object) bs n ++ NounPhrase.check none (nomIntro bs n) host ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield) ++
        refuse (Kind.lte kh (.join .object .player)) (.kindLte kh (.join .object .player))
    | .attachment .unattached n host =>
      NounPhrase.check (some .object) bs n ++ NounPhrase.check (some .object) (nomIntro bs n) host ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield)
    | .dealsDamage kind n to =>
      NounPhrase.check (some .object) bs n ++ DamagePatient.check (nomIntro bs n) to ++
        refuse (zoneFits (NounPhrase.zone bs n) kind.sourceZone) .zoneFits
    | .beginningOf _ part whose =>
      HeaderPossessor.check bs whose ++ refuse (part.proper && whose.ok) .windowOk
    | .casts who what from_ =>
      let bs' := nomIntro bs who
      NounPhrase.check (some .player) bs who ++ NounPhrase.check (some .object) bs' what ++
        OptZoneExpr.check (nomIntro bs' what) from_ ++
        refuse (zoneIsB (NounPhrase.zone bs' what) .stack) (.zoneIs .stack) ++
        refuse what.plur.isOne .singular ++ refuse (!what.targeted) .nontarget ++
        refuse (playableFrom (from_.map ZoneExpr.sort)) .playableFrom
    | .becomesTarget n by_ =>
      let kn := n.kindOr .object
      let kb := by_.kindOr .object
      NounPhrase.check none bs n ++ NounPhrase.check none (nomIntro bs n) by_ ++
        refuse kn.targetable (.targetable kn) ++ refuse kb.targeter (.targeter kb)
    | .statusEvent n v =>
      NounPhrase.check (some .object) bs n ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield) ++
        refuse v.markable .statusMarkable
    | .gameBecomes d => refuse d.gameWide (.designationScope d)
    | .stateHolds c => Condition.check bs c
    | .putInto n to from_ =>
      NounPhrase.check (some .object) bs n ++ ZoneExpr.check bs to ++ OptEventSource.check bs from_ ++
        refuse (putDestOk to) .lookbackDest ++ refuse (putSourceOk from_) .lookbackSource ++
        refuse (zoneFits (NounPhrase.zone bs n) (sourceZone from_)) .zoneFits
    | .counterEvent dir kind n batch by_ byEffect =>
      let k := n.kindOr .object
      NounPhrase.check none bs n ++ OptCounterKind.check kind ++
        refuse (counterKindNamed k kind) (.counterKindNamed k) ++
        OptNoun.check (some .player) bs by_ ++ refuse (eventAgentOk bs by_) .eventAgent ++
        refuse (counterBatchOk batch dir kind) .counterBatchOk ++ refuse (causedByOk byEffect by_) .causedByOk
    | .tokensCreated n byEffect by_ under =>
      NounPhrase.check (some .object) bs n ++ refuse n.tokenPhrase .tokenPhrase ++
        OptNoun.check (some .player) bs by_ ++ OptNoun.check (some .player) bs under ++
        refuse (creationVoiceOk bs byEffect by_ under) .verbedVoiceOk
    | .chapterMark ns => refuse (chapterMarksOk ns) .chapterMarks
    | .activates who what =>
      NounPhrase.check (some .player) bs who ++ NounPhrase.check (some .object) (nomIntro bs who) what ++
        refuse what.plur.isOne .singular ++ refuse (!what.targeted) .nontarget
    | .statBecomes n _ v =>
      NounPhrase.check (some .object) bs n ++ Amount.check (nomIntro bs n) v ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield)
    | .flipsCoin who _ => NounPhrase.check (some .player) bs who
    | .rollsDice who _ _ res => NounPhrase.check (some .player) bs who ++ RollWatch.check bs res
    | .paysCost who _ whose kw =>
      OptNoun.check (some .player) bs who ++ NounPhrase.check (some .object) (optAgentIntro bs who) whose ++
        refuse (keywordCosts kw) (.keywordCost kw) ++ refuse whose.plur.isOne .singular
    | .paysLife who => NounPhrase.check (some .player) bs who
    | .lifeChanges who _ => NounPhrase.check (some .player) bs who
    | .verbedEvent who v what becomes =>
      let bs' := optAgentIntro bs who
      OptNoun.check (some .player) bs who ++ OptNoun.check (some .object) bs' what ++
        OptPredicate.check .object bs' becomes ++ refuse (knownAct v) (.knownAct v) ++
        refuse (verbPatientOk v what) .verbPatientOk ++
        refuse (zoneFits (patientZone bs' what) (actZoneOf v)) .zoneFits ++
        refuse (verbedVoiceOk v who what) .verbedVoiceOk ++
        refuse (verbBecomesOk v becomes) .verbBecomesOk
    | .tappedForMana who what ty =>
      let bs' := optAgentIntro bs who
      OptNoun.check (some .player) bs who ++ NounPhrase.check (some .object) bs' what ++
        refuse (zoneFits (NounPhrase.zone bs' what) (some .battlefield)) .zoneFits ++
        OptManaTypeTerm.check (nomIntro bs' what) ty
    | .unlocksDoor who door => NounPhrase.check (some .player) bs who ++ Door.check (nomIntro bs who) door
    | .nthOccurrence ordinal _ ev =>
      refuse ordinal.ok .ordinalNonZero ++ GameEvent.check bs ev
    | .triggers what =>
      NounPhrase.check (some .object) bs what ++ refuse what.plur.isOne .singular ++
        refuse (!what.targeted) .nontarget
    | .commitsCrime who => NounPhrase.check (some .player) bs who ++ refuse who.plur.isOne .singular
    | .causes by_ what => Causing.check bs by_ ++ GameEvent.check (by_.intro bs) what
  termination_by structural ev => ev

  def Causing.check (bs : Bindings) : Causing → List Refusal
    | .source src => NounPhrase.check none bs src
    | .event ev => GameEvent.check bs ev
    | .anEffect => []
  termination_by structural c => c
end

def GameEvent.checkAll (bs : Bindings) (evs : List GameEvent) : List Refusal :=
  evs.flatMap (GameEvent.check bs)

def DurationEnd.check (bs : Bindings) : DurationEnd → List Refusal
  | .startOf _ w => OptNoun.check (some .player) bs w ++ refuse (durationPossessorOk w) .durationPossessor
  | .endOf _ w => OptNoun.check (some .player) bs w ++ refuse (durationPossessorOk w) .durationPossessor

def Duration.check (bs : Bindings) : Duration → List Refusal
  | .thisTurn | .restOfGame => []
  | .until_ e => DurationEnd.check bs e
  | .forAsLongAs c => Condition.check bs c
  | .untilEvent ev => GameEvent.check bs ev
  | .duringNextTurnOf who => NounPhrase.check (some .player) bs who ++ refuse who.plur.isOne .singular

def OptDuration.check (bs : Bindings) : Option Duration → List Refusal
  | none => []
  | some d => Duration.check bs d

def Timing.check (bs : Bindings) : Timing → List Refusal
  | .asSorcery | .asInstant => []
  | .duringPart p w => OptNoun.check (some .player) bs w ++ refuse (windowOk p w) .windowOk
  | .beforePart p w =>
    OptNoun.check (some .player) bs w ++ refuse p.proper .windowOk ++
      refuse (pointWindowOk w) .pointWindowOk

def OptTiming.check (bs : Bindings) : Option Timing → List Refusal
  | none => []
  | some t => Timing.check bs t

def Concurrent.check (bs : Bindings) : Concurrent → List Refusal
  | .whileTrue c => Condition.check bs c
  | .whileDoing ev => GameEvent.check bs ev ++ refuse (eventUnderwayOk ev.name) .eventUnderway

def OptConcurrent.check (bs : Bindings) : Option Concurrent → List Refusal
  | none => []
  | some c => Concurrent.check bs c

/-- Idris `HeaderNontarget` and `HeaderStatus` on a header event and each `or`-arm. -/
def headerEventCheck (bs : Bindings) (ev : GameEvent) : List Refusal :=
  GameEvent.check bs ev ++ refuse (ev.headerNontarget bs) .headerNontarget ++
    refuse ev.headerStatusOk .statusMarkable

def JoinedHeader.check (bs : Bindings) (j : JoinedHeader) : List Refusal :=
  headerEventCheck bs j.event ++ j.alternatives.flatMap (headerEventCheck bs) ++
    OptConcurrent.check (headerCtx bs j.alternatives j.event) j.while_ ++
    OptTiming.check bs j.timing

end Semantics
