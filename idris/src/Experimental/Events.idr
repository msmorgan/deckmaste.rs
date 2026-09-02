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

public export
statusEventName : StatusCat -> EventName
statusEventName TapC = StatusChange
statusEventName FlipC = StatusChange
statusEventName FaceC = TurnedFaceUp
statusEventName PhaseC = PhasingChange

public export
data CounterMove = CounterPut | CounterTaken

public export
counterEventName : CounterMove -> EventName
counterEventName CounterPut = CounterPlacement
counterEventName CounterTaken = CounterRemoval

public export
data CounterBatch = OneCounter | ManyCounters

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
sameEventName : EventName -> EventName -> Bool
sameEventName Death Death = True
sameEventName Death _ = False
sameEventName Departure Departure = True
sameEventName Departure _ = False
sameEventName DamageTaken DamageTaken = True
sameEventName DamageTaken _ = False
sameEventName CardDrawn CardDrawn = True
sameEventName CardDrawn _ = False
sameEventName GameLoss GameLoss = True
sameEventName GameLoss _ = False
sameEventName Entry Entry = True
sameEventName Entry _ = False
sameEventName AttackDeclaration AttackDeclaration = True
sameEventName AttackDeclaration _ = False
sameEventName BlockDeclaration BlockDeclaration = True
sameEventName BlockDeclaration _ = False
sameEventName CombatDamage CombatDamage = True
sameEventName CombatDamage _ = False
sameEventName DamageDealing DamageDealing = True
sameEventName DamageDealing _ = False
sameEventName PartBeginning PartBeginning = True
sameEventName PartBeginning _ = False
sameEventName SpellCast SpellCast = True
sameEventName SpellCast _ = False
sameEventName StatusChange StatusChange = True
sameEventName StatusChange _ = False
sameEventName TurnedFaceUp TurnedFaceUp = True
sameEventName TurnedFaceUp _ = False
sameEventName PhasingChange PhasingChange = True
sameEventName PhasingChange _ = False
sameEventName BlockedDeclaration BlockedDeclaration = True
sameEventName BlockedDeclaration _ = False
sameEventName Attachment Attachment = True
sameEventName Attachment _ = False
sameEventName Unattachment Unattachment = True
sameEventName Unattachment _ = False
sameEventName LastCounterRemoval LastCounterRemoval = True
sameEventName LastCounterRemoval _ = False
sameEventName Placement Placement = True
sameEventName Placement _ = False
sameEventName CounterPlacement CounterPlacement = True
sameEventName CounterPlacement _ = False
sameEventName CounterRemoval CounterRemoval = True
sameEventName CounterRemoval _ = False
sameEventName LifeGain LifeGain = True
sameEventName LifeGain _ = False
sameEventName LifeLoss LifeLoss = True
sameEventName LifeLoss _ = False
sameEventName TimeShift TimeShift = True
sameEventName TimeShift _ = False
sameEventName TokenCreation TokenCreation = True
sameEventName TokenCreation _ = False
sameEventName ChapterArrival ChapterArrival = True
sameEventName ChapterArrival _ = False
sameEventName AbilityActivation AbilityActivation = True
sameEventName AbilityActivation _ = False
sameEventName StatValueChange StatValueChange = True
sameEventName StatValueChange _ = False
sameEventName Regeneration Regeneration = True
sameEventName Regeneration _ = False
sameEventName FlipWin FlipWin = True
sameEventName FlipWin _ = False
sameEventName FlipLoss FlipLoss = True
sameEventName FlipLoss _ = False
sameEventName CoinFlip CoinFlip = True
sameEventName CoinFlip _ = False
sameEventName DiceRoll DiceRoll = True
sameEventName DiceRoll _ = False
sameEventName CostPayment CostPayment = True
sameEventName CostPayment _ = False
sameEventName CostNonpayment CostNonpayment = True
sameEventName CostNonpayment _ = False
sameEventName LifePayment LifePayment = True
sameEventName LifePayment _ = False
sameEventName BecomesTarget BecomesTarget = True
sameEventName BecomesTarget _ = False
sameEventName (VerbedAct a) (VerbedAct b) = a == b
sameEventName (VerbedAct _) _ = False
sameEventName StateMatch _ = False
sameEventName AbilityTrigger AbilityTrigger = True
sameEventName AbilityTrigger _ = False

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
interceptOk : EventName -> Bool
interceptOk ChapterArrival = False
interceptOk (VerbedAct _) = True
interceptOk DamageDealing = True
interceptOk BecomesTarget = True
interceptOk AbilityTrigger = False
interceptOk _ = True

public export
triggerCountOk : EventName -> Bool
triggerCountOk AbilityTrigger = True
triggerCountOk _ = False

public export
spanEventOk : EventName -> Bool
spanEventOk PartBeginning = False
spanEventOk (VerbedAct _) = True
spanEventOk DamageDealing = True
spanEventOk BecomesTarget = True
spanEventOk _ = True

public export
eventUnderwayOk : EventName -> Bool
eventUnderwayOk SpellCast = True
eventUnderwayOk AbilityActivation = True
eventUnderwayOk (VerbedAct v) = actStepwiseOf v
eventUnderwayOk _ = False

public export
eventHasMagnitude : EventName -> Bool
eventHasMagnitude DamageTaken = True
eventHasMagnitude CombatDamage = True
eventHasMagnitude DamageDealing = True
eventHasMagnitude LifeGain = True
eventHasMagnitude LifeLoss = True
eventHasMagnitude Death = False
eventHasMagnitude Departure = False
eventHasMagnitude CardDrawn = False
eventHasMagnitude Entry = False
eventHasMagnitude AttackDeclaration = False
eventHasMagnitude BlockDeclaration = False
eventHasMagnitude PartBeginning = False
eventHasMagnitude SpellCast = False
eventHasMagnitude StatusChange = False
eventHasMagnitude TurnedFaceUp = False
eventHasMagnitude PhasingChange = False
eventHasMagnitude BlockedDeclaration = False
eventHasMagnitude Attachment = False
eventHasMagnitude Unattachment = False
eventHasMagnitude LastCounterRemoval = False
eventHasMagnitude TimeShift = False
eventHasMagnitude Placement = False
eventHasMagnitude CounterPlacement = False
eventHasMagnitude CounterRemoval = False
eventHasMagnitude GameLoss = False
eventHasMagnitude TokenCreation = False
eventHasMagnitude ChapterArrival = False
eventHasMagnitude AbilityActivation = False
eventHasMagnitude StatValueChange = False
eventHasMagnitude Regeneration = False
eventHasMagnitude FlipWin = False
eventHasMagnitude FlipLoss = False
eventHasMagnitude CoinFlip = False
eventHasMagnitude DiceRoll = False
eventHasMagnitude CostPayment = False
eventHasMagnitude CostNonpayment = False
eventHasMagnitude LifePayment = True
eventHasMagnitude BecomesTarget = False
eventHasMagnitude (VerbedAct _) = False
eventHasMagnitude StateMatch = False
eventHasMagnitude AbilityTrigger = False

public export
data ReplUse = Repeatedly | NextTimeOnly

public export
data TriggerWord = When | Whenever | At

public export
lookbackSubjectOk : EventName -> Kind -> Bool
lookbackSubjectOk Death Object = True
lookbackSubjectOk Death Player = False
lookbackSubjectOk Departure Object = True
lookbackSubjectOk Departure Player = False
lookbackSubjectOk DamageTaken Object = True
lookbackSubjectOk DamageTaken Player = True
lookbackSubjectOk CardDrawn Object = False
lookbackSubjectOk CardDrawn Player = True
lookbackSubjectOk GameLoss Object = False
lookbackSubjectOk GameLoss Player = True
lookbackSubjectOk Entry Object = True
lookbackSubjectOk Entry Player = False
lookbackSubjectOk AttackDeclaration Object = True
lookbackSubjectOk AttackDeclaration Player = True
lookbackSubjectOk BlockDeclaration Object = True
lookbackSubjectOk BlockDeclaration Player = False
lookbackSubjectOk CombatDamage Object = True
lookbackSubjectOk CombatDamage Player = False
lookbackSubjectOk DamageDealing Object = True
lookbackSubjectOk DamageDealing Player = False
lookbackSubjectOk PartBeginning Object = True
lookbackSubjectOk PartBeginning Player = True
lookbackSubjectOk SpellCast Object = False
lookbackSubjectOk SpellCast Player = True
lookbackSubjectOk StatusChange Object = False
lookbackSubjectOk StatusChange Player = False
lookbackSubjectOk TurnedFaceUp Object = False
lookbackSubjectOk TurnedFaceUp Player = False
lookbackSubjectOk PhasingChange Object = False
lookbackSubjectOk PhasingChange Player = False
lookbackSubjectOk BlockedDeclaration Object = True
lookbackSubjectOk BlockedDeclaration Player = False
lookbackSubjectOk Attachment Object = True
lookbackSubjectOk Attachment Player = False
lookbackSubjectOk Unattachment Object = True
lookbackSubjectOk Unattachment Player = False
lookbackSubjectOk LastCounterRemoval Object = False
lookbackSubjectOk LastCounterRemoval Player = False
lookbackSubjectOk Placement Object = True
lookbackSubjectOk Placement Player = False
lookbackSubjectOk CounterPlacement Object = False
lookbackSubjectOk CounterPlacement Player = False
lookbackSubjectOk CounterRemoval Object = False
lookbackSubjectOk CounterRemoval Player = False
lookbackSubjectOk LifeGain Object = False
lookbackSubjectOk LifeGain Player = True
lookbackSubjectOk LifeLoss Object = False
lookbackSubjectOk LifeLoss Player = True
lookbackSubjectOk TimeShift Object = False
lookbackSubjectOk TimeShift Player = False
lookbackSubjectOk TokenCreation Object = False
lookbackSubjectOk TokenCreation Player = True
lookbackSubjectOk ChapterArrival Object = False
lookbackSubjectOk ChapterArrival Player = False
lookbackSubjectOk AbilityActivation Object = False
lookbackSubjectOk AbilityActivation Player = True
lookbackSubjectOk StatValueChange Object = False
lookbackSubjectOk StatValueChange Player = False
lookbackSubjectOk Regeneration Object = True
lookbackSubjectOk Regeneration Player = False
lookbackSubjectOk FlipWin Object = False
lookbackSubjectOk FlipWin Player = True
lookbackSubjectOk FlipLoss Object = False
lookbackSubjectOk FlipLoss Player = True
lookbackSubjectOk CoinFlip Object = False
lookbackSubjectOk CoinFlip Player = True
lookbackSubjectOk DiceRoll Object = False
lookbackSubjectOk DiceRoll Player = True
lookbackSubjectOk CostPayment Object = False
lookbackSubjectOk CostPayment Player = False
lookbackSubjectOk CostNonpayment Object = False
lookbackSubjectOk CostNonpayment Player = False
lookbackSubjectOk LifePayment Object = False
lookbackSubjectOk LifePayment Player = True
lookbackSubjectOk BecomesTarget Object = True
lookbackSubjectOk BecomesTarget Player = True
lookbackSubjectOk (VerbedAct v) Object = actPatientOf v == Just Object
lookbackSubjectOk (VerbedAct _) Player = True
lookbackSubjectOk StateMatch Object = False
lookbackSubjectOk StateMatch Player = False
lookbackSubjectOk AbilityTrigger Object = True
lookbackSubjectOk AbilityTrigger Player = False
lookbackSubjectOk _ (Quality _) = False
lookbackSubjectOk _ Outcome = False
lookbackSubjectOk _ Gap = False
lookbackSubjectOk _ TurnRef = False
lookbackSubjectOk _ Ability = False
lookbackSubjectOk _ (LetterK _) = False
lookbackSubjectOk ev (a \/ b) = lookbackSubjectOk ev a && lookbackSubjectOk ev b

public export
data LookbackSubject : EventName -> Kind -> Type where
  MkLookbackSubject : {auto 0 ok : So (lookbackSubjectOk ev k)} ->
                      LookbackSubject ev k

public export
lookbackComplementOk : EventName -> Kind -> Kind -> Bool
lookbackComplementOk SpellCast Player Object = True
lookbackComplementOk SpellCast _ _ = False
lookbackComplementOk DamageTaken Object Object = True
lookbackComplementOk DamageTaken Player Object = True
lookbackComplementOk DamageTaken (a \/ b) kc =
  lookbackComplementOk DamageTaken a kc && lookbackComplementOk DamageTaken b kc
lookbackComplementOk DamageTaken _ _ = False
lookbackComplementOk CombatDamage Object Player = True
lookbackComplementOk CombatDamage _ _ = False
lookbackComplementOk DamageDealing Object Object = True
lookbackComplementOk DamageDealing Object Player = True
lookbackComplementOk DamageDealing _ _ = False
lookbackComplementOk AttackDeclaration Player Object = True
lookbackComplementOk AttackDeclaration Player Player = True
lookbackComplementOk AttackDeclaration Object Player = True
lookbackComplementOk AttackDeclaration Object Object = False
lookbackComplementOk AttackDeclaration _ _ = False
lookbackComplementOk BlockDeclaration Object Object = True
lookbackComplementOk BlockDeclaration _ _ = False
lookbackComplementOk BlockedDeclaration Object Object = True
lookbackComplementOk BlockedDeclaration _ _ = False
lookbackComplementOk TokenCreation Player Object = True
lookbackComplementOk TokenCreation _ _ = False
lookbackComplementOk Death _ _ = False
lookbackComplementOk Departure _ _ = False
lookbackComplementOk Entry _ _ = False
lookbackComplementOk CardDrawn _ _ = False
lookbackComplementOk LifeGain _ _ = False
lookbackComplementOk LifeLoss _ _ = False
lookbackComplementOk Placement _ _ = False
lookbackComplementOk CounterPlacement _ _ = False
lookbackComplementOk CounterRemoval _ _ = False
lookbackComplementOk AbilityActivation Player Ability = True
lookbackComplementOk AbilityActivation _ _ = False
lookbackComplementOk FlipWin _ _ = False
lookbackComplementOk FlipLoss _ _ = False
lookbackComplementOk CoinFlip _ _ = False
lookbackComplementOk DiceRoll _ _ = False
lookbackComplementOk LifePayment _ _ = False
lookbackComplementOk BecomesTarget Object Object = True
lookbackComplementOk BecomesTarget Object Ability = True
lookbackComplementOk BecomesTarget Player Object = True
lookbackComplementOk BecomesTarget Player Ability = True
lookbackComplementOk BecomesTarget ks (a \/ b) =
  lookbackComplementOk BecomesTarget ks a && lookbackComplementOk BecomesTarget ks b
lookbackComplementOk BecomesTarget _ _ = False
lookbackComplementOk (VerbedAct v) Player kc = actPatientOf v == Just kc
lookbackComplementOk (VerbedAct _) _ _ = False
lookbackComplementOk _ _ _ = False

public export
bareLookbackOk : EventName -> Kind -> Bool
bareLookbackOk TokenCreation Player = False
bareLookbackOk AbilityActivation Player = False
bareLookbackOk (VerbedAct v) Player =
  not (actNamesPatient v) && not (actNamesLocus v)
bareLookbackOk (VerbedAct _) Object = True
bareLookbackOk DamageDealing Object = True
bareLookbackOk BecomesTarget Object = True
bareLookbackOk BecomesTarget Player = True
bareLookbackOk Placement Object = False
bareLookbackOk _ _ = True

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
verbForManaOk : VerbLabel -> Bool
verbForManaOk v = v == "Tap"

public export
data PremiseSort = ObjectPremise | ManaPremise | ValuePremise

public export
Eq PremiseSort where
  (==) ObjectPremise ObjectPremise = True
  (==) ObjectPremise _ = False
  (==) ManaPremise ManaPremise = True
  (==) ManaPremise _ = False
  (==) ValuePremise ValuePremise = True
  (==) ValuePremise _ = False

public export
record DeedRole where
  constructor MkDeedRole
  roleKinds : List Kind
  roleTypes : List CardType
  roleBare : Bool
  roleZone : Maybe Zone

public export
noRole : DeedRole
noRole = MkDeedRole [] [] False Nothing

public export
record DeedFacts where
  constructor MkDeedFacts
  deed : VerbLabel
  deedAgent : DeedRole
  deedPatient : DeedRole
  deedDefends : Bool
  deedTargeted : Bool
  deedCounterfactual : Maybe PremiseSort
  deedRides : Bool
  deedPlays : Bool

public export
deedFacts : List DeedFacts
deedFacts =
  [ MkDeedFacts "Attack"
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Planeswalker, Battle] False (Just Battlefield))
      True False (Just ObjectPremise) False False
  , MkDeedFacts "Block"
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Creature] False (Just Battlefield))
      False False (Just ObjectPremise) False False
  , MkDeedFacts "Target"
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object, Player] [Creature, Artifact, Land, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True Nothing)
      False True (Just ObjectPremise) False False
  , MkDeedFacts "Cast"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False (Just ObjectPremise) True True
  , MkDeedFacts "Play"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True Nothing)
      False False (Just ObjectPremise) True True
  , MkDeedFacts "Counter"
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False Nothing True False
  , MkDeedFacts "Copy"
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False Nothing False False
  , MkDeedFacts "Activate"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Ability] [] True Nothing)
      False False Nothing False False
  , MkDeedFacts "Regenerate"
      (MkDeedRole [] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing True False
  , MkDeedFacts "GainLife"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False
  , MkDeedFacts "DrawCard"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [] True (Just Library))
      False False Nothing False False
  , MkDeedFacts "Sacrifice"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing False False
  , MkDeedFacts "Trigger"
      (MkDeedRole [Ability] [] True Nothing) noRole
      False False Nothing False False
  , MkDeedFacts "Untap"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing False False
  , MkDeedFacts "SearchLibrary"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False
  , MkDeedFacts "LoseGame"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False
  , MkDeedFacts "WinGame"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False
  , MkDeedFacts "Spend"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False (Just ManaPremise) False False
  , MkDeedFacts "Crew"
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Artifact] True (Just Battlefield))
      False False (Just ValuePremise) False False
  , MkDeedFacts "Saddle"
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False (Just ValuePremise) False False
  ]

public export
deedIn : VerbLabel -> List DeedFacts -> Maybe DeedFacts
deedIn v [] = Nothing
deedIn v (f :: fs) = if deed f == v then Just f else deedIn v fs

public export
deedFactsFor : VerbLabel -> Maybe DeedFacts
deedFactsFor v = deedIn v deedFacts

public export
knownDeed : VerbLabel -> Bool
knownDeed v = isJust (deedFactsFor v)

public export
KnownDeed : VerbLabel -> Type
KnownDeed v = So (knownDeed v)

public export
deedRoleOf : VerbLabel -> Role -> DeedRole
deedRoleOf v Agent = maybe noRole deedAgent (deedFactsFor v)
deedRoleOf v Patient = maybe noRole deedPatient (deedFactsFor v)

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
deedHeadTysOk : VerbLabel -> Role -> List CardType -> Bool
deedHeadTysOk v r [] = deedBareOk v r
deedHeadTysOk v r ts = all (deedTypeOk v r) ts

public export
deedZoneOf : VerbLabel -> Role -> Maybe Zone
deedZoneOf v r = roleZone (deedRoleOf v r)

public export
deedDefendsOk : VerbLabel -> Bool
deedDefendsOk v = maybe False deedDefends (deedFactsFor v)

public export
deedTargetedOk : VerbLabel -> Bool
deedTargetedOk v = maybe False deedTargeted (deedFactsFor v)

public export
deedPremiseSort : VerbLabel -> Maybe PremiseSort
deedPremiseSort v = deedFactsFor v >>= deedCounterfactual

public export
deedCounterfactualOk : VerbLabel -> Bool
deedCounterfactualOk v = isJust (deedPremiseSort v)

public export
deedPremiseOk : PremiseSort -> VerbLabel -> Bool
deedPremiseOk s v = deedPremiseSort v == Just s

public export
deedRidesOk : VerbLabel -> Bool
deedRidesOk v = maybe False deedRides (deedFactsFor v)

public export
deedPlaysOk : VerbLabel -> Bool
deedPlaysOk v = maybe False deedPlays (deedFactsFor v)

public export
Deeds : Type
Deeds = List VerbLabel

public export
knownDeeds : Deeds -> Bool
knownDeeds ds = all knownDeed ds

public export
KnownDeeds : Deeds -> Type
KnownDeeds ds = So (knownDeeds ds)

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

public export
data DeedParticipant : Deeds -> Role -> Kind -> List CardType -> Type where
  Participant : {auto 0 kk : So (all (\d => deedKindOk d r k) ds)} ->
                {auto 0 ok : So (all (\d => deedHeadTysOk d r ts) ds)} ->
                DeedParticipant ds r k ts

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
                | LandAllowance
                | BlockAllowance
                | VoteAllowance
                | UntapGrant
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
