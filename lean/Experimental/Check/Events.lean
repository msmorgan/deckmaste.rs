import Experimental.Check.Words

/-!
# Experimental.Check.Events

The event facts table and the lookback, deed, and play-window rules of `Events.idr`.
-/

namespace Mtg

def counterEventName : CounterMove → CounterBatch → EventName
  | .put, _ => .counterPlacement
  | .taken, .last => .lastCounterRemoval
  | .taken, _ => .counterRemoval

/-- Only a removal empties a named kind, so only it has a last counter. -/
def counterBatchOk : CounterBatch → CounterMove → Option CounterKind → Bool
  | .last, .taken, kind => kind.isSome
  | .last, _, _ => false
  | _, _, _ => true

def FlipCall.eventName : FlipCall → EventName
  | .wins => .flipWin
  | .loses => .flipLoss

def RolledDie.hasResult : RolledDie → Bool
  | .planar => false
  | _ => true

def PaymentOutcome.eventName : PaymentOutcome → EventName
  | .paid => .costPayment
  | .unpaid => .costNonpayment

def LifeMove.eventName : LifeMove → EventName
  | .up => .lifeGain
  | .down => .lifeLoss

def LifeMove.outcome : LifeMove → OutcomeSort
  | .up => .lifeGained
  | .down => .lifeLost

/-- `stateMatch` never equals another event, itself included. -/
def sameEventName : EventName → EventName → Bool
  | .stateMatch, _ => false
  | a, b => a == b

structure EventFacts where
  subjectKinds : List Kind
  complementKinds : List (Kind × Kind)
  bareRefused : List Kind
  hasMagnitude : Bool
  interceptable : Bool
  countable : Bool
  spannable : Bool
  underway : Bool
  deriving Repr, BEq

def EventName.facts : EventName → EventFacts
  | .death => ⟨[.object], [], [], false, true, false, true, false⟩
  | .departure => ⟨[.object], [], [], false, true, false, true, false⟩
  | .damageTaken =>
    ⟨[.object, .player], [(.object, .object), (.player, .object)], [], true, true, false, true, false⟩
  | .cardDrawn => ⟨[.player], [], [], false, true, false, true, false⟩
  | .entry => ⟨[.object], [], [], false, true, false, true, false⟩
  | .attackDeclaration =>
    ⟨[.object, .player], [(.player, .object), (.player, .player), (.object, .player)], [],
     false, true, false, true, false⟩
  | .blockDeclaration => ⟨[.object], [(.object, .object)], [], false, true, false, true, false⟩
  | .combatDamage => ⟨[.object], [(.object, .player)], [], true, true, false, true, false⟩
  | .partBeginning => ⟨[.object, .player], [], [], false, true, false, false, false⟩
  | .spellCast => ⟨[.player], [(.player, .object)], [], false, true, false, true, true⟩
  | .statusChange => ⟨[], [], [], false, true, false, true, false⟩
  | .turnedFaceUp => ⟨[], [], [], false, true, false, true, false⟩
  | .phasingChange => ⟨[], [], [], false, true, false, true, false⟩
  | .blockedDeclaration => ⟨[.object], [(.object, .object)], [], false, true, false, true, false⟩
  | .attachment => ⟨[.object], [], [], false, true, false, true, false⟩
  | .unattachment => ⟨[.object], [], [], false, true, false, true, false⟩
  | .lastCounterRemoval => ⟨[], [], [], false, true, false, true, false⟩
  | .lifeGain => ⟨[.player], [], [], true, true, false, true, false⟩
  | .lifeLoss => ⟨[.player], [], [], true, true, false, true, false⟩
  | .timeShift => ⟨[], [], [], false, true, false, true, false⟩
  | .placement => ⟨[.object], [], [.object], false, true, false, true, false⟩
  | .counterPlacement => ⟨[], [], [], false, true, false, true, false⟩
  | .counterRemoval => ⟨[], [], [], false, true, false, true, false⟩
  | .gameLoss => ⟨[.player], [], [], false, true, false, true, false⟩
  | .tokenCreation => ⟨[.player], [(.player, .object)], [.player], false, true, false, true, false⟩
  | .chapterArrival => ⟨[], [], [], false, false, false, true, false⟩
  | .abilityActivation =>
    ⟨[.player], [(.player, .object)], [.player], false, true, false, true, true⟩
  | .statValueChange => ⟨[], [], [], false, true, false, true, false⟩
  | .regeneration => ⟨[.object], [], [], false, true, false, true, false⟩
  | .flipWin => ⟨[.player], [], [], false, true, false, true, false⟩
  | .flipLoss => ⟨[.player], [], [], false, true, false, true, false⟩
  | .coinFlip => ⟨[.player], [], [], false, true, false, true, false⟩
  | .diceRoll => ⟨[.player], [], [], false, true, false, true, false⟩
  | .costPayment => ⟨[], [], [], false, true, false, true, false⟩
  | .costNonpayment => ⟨[], [], [], false, true, false, true, false⟩
  | .lifePayment => ⟨[.player], [], [], true, true, false, true, false⟩
  | .becomesTarget =>
    ⟨[.object, .player], [(.object, .object), (.player, .object)], [], false, true, false, true, false⟩
  | .damageDealing =>
    ⟨[.object], [(.object, .object), (.object, .player)], [], true, true, false, true, false⟩
  | .verbedAct v =>
    ⟨.player :: (if (actPatientKindsOf v).elem .object then [.object] else []),
     (actPatientKindsOf v).map (fun k => (.player, k)),
     (if actNamesPatient v || actNamesLocus v then [.player] else []),
     false, true, false, true, actStepwiseOf v⟩
  | .stateMatch => ⟨[], [], [], false, true, false, true, false⟩
  | .abilityTrigger => ⟨[.object], [], [], false, false, true, true, false⟩
  | .crimeCommission => ⟨[.player], [], [], false, true, false, true, false⟩
  /- a mana ability with {T} in its cost resolving and producing mana [CR#106.12a] -/
  | .tappedForMana =>
    ⟨[.player, .object], [(.player, .object)], [.player], false, true, false, true, false⟩

def kindIn : Kind → List Kind → Bool
  | .join a b, ks => kindIn a ks && kindIn b ks
  | k, ks => ks.elem k

def kindAny : Kind → List Kind → Bool
  | .join a b, ks => kindAny a ks || kindAny b ks
  | k, ks => ks.elem k

/-- The Idris splits the subject join first, then the complement join; two structural
recursions. -/
def kindPairInC (ks : Kind) : Kind → List (Kind × Kind) → Bool
  | .join a b, ps => kindPairInC ks a ps && kindPairInC ks b ps
  | kc, ps => ps.elem (ks, kc)

def kindPairIn : Kind → Kind → List (Kind × Kind) → Bool
  | .join a b, kc, ps => kindPairIn a kc ps && kindPairIn b kc ps
  | ks, kc, ps => kindPairInC ks kc ps

def interceptOk (ev : EventName) : Bool := ev.facts.interceptable
def triggerCountOk (ev : EventName) : Bool := ev.facts.countable
def spanEventOk (ev : EventName) : Bool := ev.facts.spannable
def eventUnderwayOk (ev : EventName) : Bool := ev.facts.underway
def eventHasMagnitude (ev : EventName) : Bool := ev.facts.hasMagnitude

def tallyOk : TallyOp → EventName → Bool
  | .count, _ => true
  | .sum, ev => eventHasMagnitude ev

def lookbackSubjectOk (ev : EventName) (k : Kind) : Bool := kindIn k ev.facts.subjectKinds
def lookbackComplementOk (ev : EventName) (ks kc : Kind) : Bool :=
  kindPairIn ks kc ev.facts.complementKinds
def bareLookbackOk (ev : EventName) (k : Kind) : Bool := !kindAny k ev.facts.bareRefused

def Role.counter : Role → Role
  | .agent => .patient
  | .patient => .agent

def deedRoleOf (v : VerbLabel) : Role → DeedRole
  | .agent => (actFactsFor v).elim noRole (·.agentRole)
  | .patient => (actFactsFor v).elim noRole (·.patientRole)

def deedKindOk (v : VerbLabel) (r : Role) (k : Kind) : Bool := (deedRoleOf v r).kinds.elem k

/-- An effect can make a permanent of any type a creature that is still its other types
[CR#205.1b], and an effect written on a noncreature permanent is created even while it isn't
one [CR#208.3a]. So the agent slot of a deed a creature performs fits any permanent head type;
what a deed is done to keeps the set the rules name. -/
def deedAnimatedOk (dr : DeedRole) : Role → CardType → Bool
  | .agent, t => dr.types.elem .creature && t.permanent
  | .patient, _ => false

def deedTypeOk (v : VerbLabel) (r : Role) (t : CardType) : Bool :=
  (deedRoleOf v r).types.elem t || deedAnimatedOk (deedRoleOf v r) r t

def deedBareOk (v : VerbLabel) (r : Role) : Bool := (deedRoleOf v r).bare

def deedAltOk (v : VerbLabel) (r : Role) : List CardType → Bool
  | [] => deedBareOk v r
  | ts => ts.any (deedTypeOk v r)

def deedHeadTysOk (v : VerbLabel) (r : Role) : List (List CardType) → Bool
  | [] => deedBareOk v r
  | alts => alts.all (deedAltOk v r)

def featureAltOk (f : DeedFeature) (r : Role) (ts : List CardType) : Bool :=
  (featureLabel f).elim false (fun v => deedAltOk v r ts)

def deedZoneOf (v : VerbLabel) (r : Role) : Option Zone := (deedRoleOf v r).zone
def deedDefendsOk (v : VerbLabel) : Bool := deedFeatureOf v == some .attacking
def deedTargetedOk (v : VerbLabel) : Bool := deedFeatureOf v == some .targeting
def deedPremiseSort (v : VerbLabel) : Option PremiseSort := actFactsFor v >>= (·.counterfactual)
def deedPremiseOk (s : PremiseSort) (v : VerbLabel) : Bool := deedPremiseSort v == some s
def deedRidesOk (v : VerbLabel) : Bool := (actFactsFor v).elim false (·.rides)
def deedPlaysOk (v : VerbLabel) : Bool := (actFactsFor v).elim false (·.plays)
def deedBoundedOk (v : VerbLabel) : Bool := (actFactsFor v).elim false (·.bounded)

def knownActs (ds : Deeds) : Bool := ds.all knownAct

def distinctDeeds : Deeds → Bool
  | [] => true
  | d :: ds => !ds.elem d && distinctDeeds ds

def deedsZone : Deeds → Role → Option Zone
  | [], _ => none
  | d :: ds, r =>
    match deedsZone ds r with
    | none => if ds.isEmpty then deedZoneOf d r else none
    | some z => if deedZoneOf d r == some z then some z else none

/-- The two deed roles an ability on the stack fills [CR#113.1c]; every other role refuses one,
and these two refuse anything else. -/
def deedAbilityRole (v : VerbLabel) (r : Role) : Bool :=
  (actFactsFor v >>= (·.abilityRole)).elim false (· == r)

def deedAbilityOk (v : VerbLabel) (r : Role) (ab : Bool) : Bool := ab == deedAbilityRole v r

def deedFits (ds : Deeds) (r : Role) (k : Kind) (ab : Bool) (ts : List (List CardType))
    (z : Option Zone) : Bool :=
  ds.all (fun d => deedKindOk d r k && deedAbilityOk d r ab && deedHeadTysOk d r ts) &&
    zoneFits z (deedsZone ds r)

def playWindowOk : Option PlayLimit → Option PlayWindow → Bool
  | _, none => true
  | some .onceEachYourTurn, some .duringEachOfYourTurns => false
  | _, _ => true

def playableFrom : Option Zone → Bool
  | some .stack => false
  | _ => true

def Zone.placementDestOk : Zone → Bool
  | .battlefield | .stack => false
  | _ => true

def Zone.placementOriginOk : Zone → Bool
  | .hand | .stack => false
  | _ => true

def Zone.entryOriginOk : Zone → Bool
  | .battlefield => false
  | _ => true

def lookbackOriginOk : EventName → Zone → Bool
  | .spellCast, z => playableFrom (some z)
  | .placement, z => z.placementOriginOk
  | .entry, z => z.entryOriginOk
  | _, _ => false

def eventNamesOrigin (ev : EventName) : Bool :=
  [Zone.battlefield, .graveyard, .library, .hand, .exile, .command, .stack].any (lookbackOriginOk ev)

def lookbackDestOk : EventName → Zone → Bool
  | .placement, z => z.placementDestOk
  | _, _ => false

def lookbackLocusOk : EventName → Zone → Bool
  | .verbedAct v, z => (actLociOf v).elem z
  | _, _ => false

/-- Idris `Possessable z`: the zones a player owns. -/
def Zone.possessable : Zone → Bool
  | .hand | .graveyard | .library => true
  | _ => false

end Mtg
