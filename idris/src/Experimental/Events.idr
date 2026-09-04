module Experimental.Events

import Experimental.Words

%default total


public export
data EventName = Death | Departure | DamageTaken
               | CardDrawn | Entry | AttackDeclaration | BlockDeclaration
               | CombatDamage | PartBeginning | SpellCast | StatusChange
               | TurnedFaceUp | PhasingChange | BlockedDeclaration
               | Attachment | Unattachment
               | LastCounterRemoval | LifeGain | LifeLoss | TimeShift
               | Placement
               | CounterPlacement | CounterRemoval
               | GameLoss
               | TokenCreation
               | ChapterArrival
               | AbilityActivation
               | StatValueChange | Regeneration
               | FlipWin | FlipLoss
               | CoinFlip
               | DiceRoll
               | CostPayment | CostNonpayment
               | LifePayment
               | BecomesTarget
               | DamageDealing
               | VerbedAct VerbLabel
               | StateMatch
               | AbilityTrigger
               | CrimeCommission
               | TappedForMana

public export
statusEventName : StatusCat -> EventName
statusEventName TapC = StatusChange
statusEventName FlipC = StatusChange
statusEventName FaceC = TurnedFaceUp
statusEventName PhaseC = PhasingChange

public export
data CounterMove = CounterPut | CounterTaken

public export
data CounterBatch = OneCounter | ManyCounters | LastCounter

public export
counterEventName : CounterMove -> CounterBatch -> EventName
counterEventName CounterPut _ = CounterPlacement
counterEventName CounterTaken LastCounter = LastCounterRemoval
counterEventName CounterTaken _ = CounterRemoval

-- only a removal empties a named kind, so only it has a last counter
public export
counterBatchOk : CounterBatch -> CounterMove -> Maybe CounterKind -> Bool
counterBatchOk LastCounter CounterTaken kind = isJust kind
counterBatchOk LastCounter _ _ = False
counterBatchOk _ _ _ = True

public export
flipEventName : FlipCall -> EventName
flipEventName WinsFlip = FlipWin
flipEventName LosesFlip = FlipLoss

public export
data DiceBatch = OneDie | ManyDice

public export
data RolledDie : Type where
  AnyDie : RolledDie
  SidedDie : (n : Nat) -> {auto 0 nz : IsSucc n} -> RolledDie
  PlanarDie : RolledDie

public export
dieHasResult : RolledDie -> Bool
dieHasResult PlanarDie = False
dieHasResult _ = True

public export
data PaymentOutcome = Paid | Unpaid

public export
paymentEventName : PaymentOutcome -> EventName
paymentEventName Paid = CostPayment
paymentEventName Unpaid = CostNonpayment

public export
data LifeMove = LifeGoesUp | LifeGoesDown

public export
lifeEventName : LifeMove -> EventName
lifeEventName LifeGoesUp = LifeGain
lifeEventName LifeGoesDown = LifeLoss

public export
lifeMoveOutcome : LifeMove -> OutcomeSort
lifeMoveOutcome LifeGoesUp = LifeGained
lifeMoveOutcome LifeGoesDown = LifeLost

public export
KeywordCost : KeywordLabel -> Type
KeywordCost k = So (keywordCosts k)

public export
eventIx : EventName -> Nat
eventIx Death = 0
eventIx Departure = 1
eventIx DamageTaken = 2
eventIx CardDrawn = 3
eventIx Entry = 4
eventIx AttackDeclaration = 5
eventIx BlockDeclaration = 6
eventIx CombatDamage = 7
eventIx PartBeginning = 8
eventIx SpellCast = 9
eventIx StatusChange = 10
eventIx TurnedFaceUp = 11
eventIx PhasingChange = 12
eventIx BlockedDeclaration = 13
eventIx Attachment = 14
eventIx Unattachment = 15
eventIx LastCounterRemoval = 16
eventIx LifeGain = 17
eventIx LifeLoss = 18
eventIx TimeShift = 19
eventIx Placement = 20
eventIx CounterPlacement = 21
eventIx CounterRemoval = 22
eventIx GameLoss = 23
eventIx TokenCreation = 24
eventIx ChapterArrival = 25
eventIx AbilityActivation = 26
eventIx StatValueChange = 27
eventIx Regeneration = 28
eventIx FlipWin = 29
eventIx FlipLoss = 30
eventIx CoinFlip = 31
eventIx DiceRoll = 32
eventIx CostPayment = 33
eventIx CostNonpayment = 34
eventIx LifePayment = 35
eventIx BecomesTarget = 36
eventIx DamageDealing = 37
eventIx (VerbedAct _) = 38
eventIx StateMatch = 39
eventIx AbilityTrigger = 40
eventIx CrimeCommission = 41
eventIx TappedForMana = 42

public export
sameEventName : EventName -> EventName -> Bool
sameEventName (VerbedAct a) (VerbedAct b) = a == b
sameEventName StateMatch _ = False
sameEventName a b = eventIx a == eventIx b

public export
sameLookback : Lookback -> Lookback -> Bool
sameLookback ThisTurn ThisTurn = True
sameLookback ThisTurn _ = False
sameLookback ThisCombat ThisCombat = True
sameLookback ThisCombat _ = False
sameLookback LastTurn LastTurn = True
sameLookback LastTurn _ = False
sameLookback ThisGame ThisGame = True
sameLookback ThisGame _ = False
sameLookback ThisWay ThisWay = True
sameLookback ThisWay _ = False
sameLookback Triggering Triggering = True
sameLookback Triggering _ = False

public export
record EventFacts where
  constructor MkEventFacts
  subjectKinds : List Kind
  complementKinds : List (Kind, Kind)
  bareRefused : List Kind
  hasMagnitude : Bool
  interceptable : Bool
  countable : Bool
  spannable : Bool
  underway : Bool

public export
eventFactsOf : EventName -> EventFacts
eventFactsOf Death = MkEventFacts [Object] [] [] False True False True False
eventFactsOf Departure = MkEventFacts [Object] [] [] False True False True False
eventFactsOf DamageTaken =
  MkEventFacts [Object, Player] [(Object, Object), (Player, Object)]
               [] True True False True False
eventFactsOf CardDrawn = MkEventFacts [Player] [] [] False True False True False
eventFactsOf Entry = MkEventFacts [Object] [] [] False True False True False
eventFactsOf AttackDeclaration =
  MkEventFacts [Object, Player]
               [(Player, Object), (Player, Player), (Object, Player)]
               [] False True False True False
eventFactsOf BlockDeclaration =
  MkEventFacts [Object] [(Object, Object)] [] False True False True False
eventFactsOf CombatDamage =
  MkEventFacts [Object] [(Object, Player)] [] True True False True False
eventFactsOf PartBeginning =
  MkEventFacts [Object, Player] [] [] False True False False False
eventFactsOf SpellCast =
  MkEventFacts [Player] [(Player, Object)] [] False True False True True
eventFactsOf StatusChange = MkEventFacts [] [] [] False True False True False
eventFactsOf TurnedFaceUp = MkEventFacts [] [] [] False True False True False
eventFactsOf PhasingChange = MkEventFacts [] [] [] False True False True False
eventFactsOf BlockedDeclaration =
  MkEventFacts [Object] [(Object, Object)] [] False True False True False
eventFactsOf Attachment =
  MkEventFacts [Object] [] [] False True False True False
eventFactsOf Unattachment =
  MkEventFacts [Object] [] [] False True False True False
eventFactsOf LastCounterRemoval =
  MkEventFacts [] [] [] False True False True False
eventFactsOf LifeGain = MkEventFacts [Player] [] [] True True False True False
eventFactsOf LifeLoss = MkEventFacts [Player] [] [] True True False True False
eventFactsOf TimeShift = MkEventFacts [] [] [] False True False True False
eventFactsOf Placement =
  MkEventFacts [Object] [] [Object] False True False True False
eventFactsOf CounterPlacement =
  MkEventFacts [] [] [] False True False True False
eventFactsOf CounterRemoval = MkEventFacts [] [] [] False True False True False
eventFactsOf GameLoss = MkEventFacts [Player] [] [] False True False True False
eventFactsOf TokenCreation =
  MkEventFacts [Player] [(Player, Object)] [Player] False True False True False
eventFactsOf ChapterArrival = MkEventFacts [] [] [] False False False True False
eventFactsOf AbilityActivation =
  MkEventFacts [Player] [(Player, Object)] [Player] False True False True True
eventFactsOf StatValueChange = MkEventFacts [] [] [] False True False True False
eventFactsOf Regeneration =
  MkEventFacts [Object] [] [] False True False True False
eventFactsOf FlipWin = MkEventFacts [Player] [] [] False True False True False
eventFactsOf FlipLoss = MkEventFacts [Player] [] [] False True False True False
eventFactsOf CoinFlip = MkEventFacts [Player] [] [] False True False True False
eventFactsOf DiceRoll = MkEventFacts [Player] [] [] False True False True False
eventFactsOf CostPayment = MkEventFacts [] [] [] False True False True False
eventFactsOf CostNonpayment = MkEventFacts [] [] [] False True False True False
eventFactsOf LifePayment =
  MkEventFacts [Player] [] [] True True False True False
eventFactsOf BecomesTarget =
  MkEventFacts [Object, Player]
               [(Object, Object), (Player, Object)]
               [] False True False True False
eventFactsOf DamageDealing =
  MkEventFacts [Object] [(Object, Object), (Object, Player)]
               [] True True False True False
eventFactsOf (VerbedAct v) =
  MkEventFacts (Player ::
                  (if elem Object (actPatientKindsOf v) then [Object] else []))
               (map (\k => (Player, k)) (actPatientKindsOf v))
               (if actNamesPatient v || actNamesLocus v then [Player] else [])
               False True False True (actStepwiseOf v)
eventFactsOf StateMatch = MkEventFacts [] [] [] False True False True False
eventFactsOf AbilityTrigger =
  MkEventFacts [Object] [] [] False False True True False
eventFactsOf CrimeCommission =
  MkEventFacts [Player] [] [] False True False True False
-- a mana ability with {T} in its cost resolving and producing mana [CR#106.12a]
eventFactsOf TappedForMana =
  MkEventFacts [Player, Object] [(Player, Object)] [Player] False True False True False

public export
kindIn : Kind -> List Kind -> Bool
kindIn (a \/ b) ks = kindIn a ks && kindIn b ks
kindIn k ks = elem k ks

public export
kindAny : Kind -> List Kind -> Bool
kindAny (a \/ b) ks = kindAny a ks || kindAny b ks
kindAny k ks = elem k ks

public export
kindPairIn : Kind -> Kind -> List (Kind, Kind) -> Bool
kindPairIn (a \/ b) kc ps = kindPairIn a kc ps && kindPairIn b kc ps
kindPairIn ks (a \/ b) ps = kindPairIn ks a ps && kindPairIn ks b ps
kindPairIn ks kc ps = elem (ks, kc) ps

public export
interceptOk : EventName -> Bool
interceptOk ev = interceptable (eventFactsOf ev)

public export
triggerCountOk : EventName -> Bool
triggerCountOk ev = countable (eventFactsOf ev)

public export
spanEventOk : EventName -> Bool
spanEventOk ev = spannable (eventFactsOf ev)

public export
eventUnderwayOk : EventName -> Bool
eventUnderwayOk ev = underway (eventFactsOf ev)

public export
eventHasMagnitude : EventName -> Bool
eventHasMagnitude ev = hasMagnitude (eventFactsOf ev)

public export
data TallyOp = TallyCount | TallySum

public export
tallyOk : TallyOp -> EventName -> Bool
tallyOk TallyCount _ = True
tallyOk TallySum ev = eventHasMagnitude ev

public export
data ReplUse = Repeatedly | NextTimeOnly

public export
data TriggerWord = When | Whenever | At

public export
lookbackSubjectOk : EventName -> Kind -> Bool
lookbackSubjectOk ev k = kindIn k (subjectKinds (eventFactsOf ev))

public export
data LookbackSubject : EventName -> Kind -> Type where
  MkLookbackSubject : {auto 0 ok : So (lookbackSubjectOk ev k)} ->
                      LookbackSubject ev k

public export
lookbackComplementOk : EventName -> Kind -> Kind -> Bool
lookbackComplementOk ev ks kc =
  kindPairIn ks kc (complementKinds (eventFactsOf ev))

public export
bareLookbackOk : EventName -> Kind -> Bool
bareLookbackOk ev k = not (kindAny k (bareRefused (eventFactsOf ev)))

public export
data LookbackComplement : EventName -> Kind -> Kind -> Type where
  MkLookbackComplement : {auto 0 ok : So (lookbackComplementOk ev ks kc)} ->
                         LookbackComplement ev ks kc


public export
partTriggerOk : TurnPart -> Bool
partTriggerOk Turn = False
partTriggerOk _ = True

public export
partAddable : TurnPart -> Bool
partAddable Turn = False
partAddable _ = True


public export
data DamageKind = AnyDamage | CombatOnly | NoncombatOnly



public export
data Role = Agent | Patient

public export
counterRole : Role -> Role
counterRole Agent = Patient
counterRole Patient = Agent


public export
deedRoleOf : VerbLabel -> Role -> DeedRole
deedRoleOf v Agent = maybe noRole agentRole (actFactsFor v)
deedRoleOf v Patient = maybe noRole patientRole (actFactsFor v)

public export
deedKindOk : VerbLabel -> Role -> Kind -> Bool
deedKindOk v r k = elem k (roleKinds (deedRoleOf v r))

public export
deedTypeOk : VerbLabel -> Role -> CardType -> Bool
deedTypeOk v r t = elem t (roleTypes (deedRoleOf v r))

public export
deedBareOk : VerbLabel -> Role -> Bool
deedBareOk v r = roleBare (deedRoleOf v r)

public export
deedAltOk : VerbLabel -> Role -> List CardType -> Bool
deedAltOk v r [] = deedBareOk v r
deedAltOk v r ts = any (deedTypeOk v r) ts

public export
deedHeadTysOk : VerbLabel -> Role -> List (List CardType) -> Bool
deedHeadTysOk v r [] = deedBareOk v r
deedHeadTysOk v r alts = all (deedAltOk v r) alts

public export
deedZoneOf : VerbLabel -> Role -> Maybe Zone
deedZoneOf v r = roleZone (deedRoleOf v r)

public export
deedDefendsOk : VerbLabel -> Bool
deedDefendsOk v = maybe False actDefends (actFactsFor v)

public export
deedTargetedOk : VerbLabel -> Bool
deedTargetedOk v = maybe False actTargeted (actFactsFor v)

public export
deedPremiseSort : VerbLabel -> Maybe PremiseSort
deedPremiseSort v = actFactsFor v >>= actCounterfactual

public export
deedPremiseOk : PremiseSort -> VerbLabel -> Bool
deedPremiseOk s v = deedPremiseSort v == Just s

public export
deedRidesOk : VerbLabel -> Bool
deedRidesOk v = maybe False actRides (actFactsFor v)

public export
deedPlaysOk : VerbLabel -> Bool
deedPlaysOk v = maybe False actPlays (actFactsFor v)

public export
deedBoundedOk : VerbLabel -> Bool
deedBoundedOk v = maybe False actBounded (actFactsFor v)

public export
Deeds : Type
Deeds = List VerbLabel

public export
KnownActs : Deeds -> Type
KnownActs = Data.List.Quantifiers.All.All KnownAct

public export
distinctDeeds : Deeds -> Bool
distinctDeeds [] = True
distinctDeeds (d :: ds) = not (elem d ds) && distinctDeeds ds

public export
deedsZone : Deeds -> Role -> Maybe Zone
deedsZone [] r = Nothing
deedsZone (d :: ds) r =
  case deedsZone ds r of
    Nothing => if null ds then deedZoneOf d r else Nothing
    Just z => if deedZoneOf d r == Just z then Just z else Nothing

||| The two deed roles an ability on the stack fills [CR#113.1c]; every other
||| role refuses one, and these two refuse anything else.
public export
deedAbilityRole : VerbLabel -> Role -> Bool
deedAbilityRole v Agent = v == "Trigger"
deedAbilityRole v Patient = v == "Activate"

public export
deedAbilityOk : VerbLabel -> Role -> Bool -> Bool
deedAbilityOk v r ab = ab == deedAbilityRole v r

public export
deedFits : Deeds -> Role -> Kind -> Bool -> List (List CardType) -> Maybe Zone -> Bool
deedFits ds r k ab ts z =
  all (\d => deedKindOk d r k && deedAbilityOk d r ab && deedHeadTysOk d r ts) ds &&
  zoneFits z (deedsZone ds r)

public export
DeedFits : Deeds -> Role -> Kind -> Bool -> List (List CardType) -> Maybe Zone -> Type
DeedFits ds r k ab ts z = So (deedFits ds r k ab ts z)

public export
data StaticKind = PtDelta | KeywordGrant | DeedRestriction | TypeAddition
                | ControlGrant | Replacement | Prevention
                | Conditional | EntryRider
                | CostModification
                | ManaPersistence
                | PtDefinition | BasePtSet | PtSwitch
                | TypeSet | TypeLoss | ColorSet | AbilityLoss | Coordination
                | CopyEffect
                | VisibilityRider
                | OutcomeImmunity
                | TriggerMultiplier
                | TurnSkip
                | LetterDefinition

public export
data CondMarking = AsLongAs | Unless | IfSo

public export
data PlayLimit = OnceEachYourTurn | OnceEachTurn

public export
data PlayWindow = WhileSearchingLibrary | DuringEachOfYourTurns

public export
playWindowOk : Maybe PlayLimit -> Maybe PlayWindow -> Bool
playWindowOk _ Nothing = True
playWindowOk (Just OnceEachYourTurn) (Just DuringEachOfYourTurns) = False
playWindowOk _ _ = True

public export
PlayWindowOk : Maybe PlayLimit -> Maybe PlayWindow -> Type
PlayWindowOk l w = So (playWindowOk l w)


public export
playableFrom : Maybe Zone -> Bool
playableFrom Nothing = True
playableFrom (Just Battlefield) = True
playableFrom (Just Graveyard) = True
playableFrom (Just Exile) = True
playableFrom (Just Hand) = True
playableFrom (Just Library) = True
playableFrom (Just Command) = True
playableFrom (Just Stack) = False

public export
PlayableFrom : Maybe Zone -> Type
PlayableFrom z = So (playableFrom z)

public export
complementLocates : Maybe Zone -> Bool
complementLocates Nothing = True
complementLocates (Just Battlefield) = False
complementLocates (Just Stack) = False
complementLocates (Just Graveyard) = True
complementLocates (Just Exile) = True
complementLocates (Just Hand) = True
complementLocates (Just Library) = True
complementLocates (Just Command) = True

public export
placementDestOk : Zone -> Bool
placementDestOk Graveyard = True
placementDestOk Exile = True
placementDestOk Library = True
placementDestOk Hand = True
placementDestOk Battlefield = False
placementDestOk Command = True
placementDestOk Stack = False

public export
placementOriginOk : Zone -> Bool
placementOriginOk Battlefield = True
placementOriginOk Graveyard = True
placementOriginOk Library = True
placementOriginOk Exile = True
placementOriginOk Command = True
placementOriginOk Hand = False
placementOriginOk Stack = False

public export
entryOriginOk : Zone -> Bool
entryOriginOk Battlefield = False
entryOriginOk Graveyard = True
entryOriginOk Library = True
entryOriginOk Hand = True
entryOriginOk Exile = True
entryOriginOk Command = True
entryOriginOk Stack = True

public export
lookbackOriginOk : EventName -> Zone -> Bool
lookbackOriginOk SpellCast z = playableFrom (Just z)
lookbackOriginOk Placement z = placementOriginOk z
lookbackOriginOk Entry z = entryOriginOk z
lookbackOriginOk _ _ = False

public export
eventNamesOrigin : EventName -> Bool
eventNamesOrigin ev =
  lookbackOriginOk ev Battlefield || lookbackOriginOk ev Graveyard ||
  lookbackOriginOk ev Library || lookbackOriginOk ev Hand ||
  lookbackOriginOk ev Exile || lookbackOriginOk ev Command ||
  lookbackOriginOk ev Stack

public export
lookbackDestOk : EventName -> Zone -> Bool
lookbackDestOk Placement z = placementDestOk z
lookbackDestOk _ _ = False

public export
lookbackLocusOk : EventName -> Zone -> Bool
lookbackLocusOk (VerbedAct v) z = elem z (actLociOf v)
lookbackLocusOk _ _ = False

public export
data ChoiceMode : Bindings -> Type where
  Unmarked : ChoiceMode bs
  TheirChoice : {auto 0 ch : countChoosers bs = 1} -> ChoiceMode bs
  AtRandom : ChoiceMode bs
  YourChoice : ChoiceMode bs


public export
data Possessable : Zone -> Type where
  HandIsOwned : Possessable Hand
  GraveyardIsOwned : Possessable Graveyard
  LibraryIsOwned : Possessable Library
