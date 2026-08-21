||| Event vocabulary and the decision tables (triggers, replacements, durations,
||| lookbacks) built on it.
module Experimental.Events

import Experimental.Words

%default total


public export
data EventName = Death | Departure | Destruction | DamageTaken
               | CardDrawn | Entry | AttackDeclaration | BlockDeclaration
               | CombatDamage | PartBeginning | SpellCast | StatusChange
               | TurnedFaceUp | PhasingChange | BlockedDeclaration
               | LastCounterRemoval | LifeGain | LifeLoss | TimeShift
               | Placement
               | CounterPlacement | CounterRemoval
               | GameLoss
               | TokenCreation
               | ChapterArrival
               | AbilityActivation

public export
statusEventName : StatusCat -> EventName
statusEventName TapC = StatusChange
-- FlipC is unreachable (statusEventOk refuses both flip values); folded
-- here rather than growing EventName with a name nothing can reach.
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
sameEventName : EventName -> EventName -> Bool
sameEventName Death Death = True
sameEventName Death _ = False
sameEventName Departure Departure = True
sameEventName Departure _ = False
sameEventName Destruction Destruction = True
sameEventName Destruction _ = False
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

public export
data EventUse = EventUnattested | EventUnclaimed | TriggeredOnly
              | InterceptedAndTriggered | InterceptedTriggeredAndDelayed
              | HeldTriggeredAndDelayed | TriggeredAndDelayed

public export
eventUse : EventName -> EventUse
eventUse Death = InterceptedTriggeredAndDelayed
eventUse Departure = HeldTriggeredAndDelayed
eventUse Destruction = EventUnclaimed
eventUse DamageTaken = TriggeredOnly
eventUse CardDrawn = InterceptedAndTriggered
eventUse GameLoss = InterceptedAndTriggered
eventUse Entry = InterceptedAndTriggered
eventUse AttackDeclaration = TriggeredOnly
eventUse BlockDeclaration = TriggeredOnly
eventUse CombatDamage = TriggeredOnly
eventUse PartBeginning = TriggeredAndDelayed
eventUse SpellCast = TriggeredOnly
eventUse StatusChange = TriggeredOnly
eventUse TurnedFaceUp = TriggeredOnly
eventUse PhasingChange = TriggeredOnly
eventUse BlockedDeclaration = TriggeredOnly
eventUse LastCounterRemoval = TriggeredOnly
eventUse Placement = InterceptedAndTriggered
eventUse CounterPlacement = InterceptedAndTriggered
eventUse CounterRemoval = TriggeredOnly
eventUse LifeGain = EventUnclaimed
eventUse LifeLoss = EventUnclaimed
eventUse TimeShift = TriggeredOnly
eventUse TokenCreation = InterceptedAndTriggered
eventUse ChapterArrival = TriggeredOnly
-- the delayed form is written, but no line that writes it is spellable:
-- three coordinate the activation with a cast or hang a payment rider,
-- and the fourth's complement is an exhaust ability, which the ability
-- vocabulary has no word for.
eventUse AbilityActivation = TriggeredOnly

public export
admitsIntercept : EventUse -> Bool
admitsIntercept EventUnattested = False
admitsIntercept EventUnclaimed = False
admitsIntercept TriggeredOnly = False
admitsIntercept InterceptedAndTriggered = True
admitsIntercept InterceptedTriggeredAndDelayed = True
admitsIntercept HeldTriggeredAndDelayed = False
admitsIntercept TriggeredAndDelayed = False

public export
admitsHold : EventUse -> Bool
admitsHold EventUnattested = False
admitsHold EventUnclaimed = False
admitsHold TriggeredOnly = False
admitsHold InterceptedAndTriggered = False
admitsHold InterceptedTriggeredAndDelayed = False
admitsHold HeldTriggeredAndDelayed = True
admitsHold TriggeredAndDelayed = False

public export
admitsTrigger : EventUse -> Bool
admitsTrigger EventUnattested = False
admitsTrigger EventUnclaimed = False
admitsTrigger TriggeredOnly = True
admitsTrigger InterceptedAndTriggered = True
admitsTrigger InterceptedTriggeredAndDelayed = True
admitsTrigger HeldTriggeredAndDelayed = True
admitsTrigger TriggeredAndDelayed = True

public export
admitsDelay : EventUse -> Bool
admitsDelay EventUnattested = False
admitsDelay EventUnclaimed = False
admitsDelay TriggeredOnly = False
admitsDelay InterceptedAndTriggered = False
admitsDelay InterceptedTriggeredAndDelayed = True
admitsDelay HeldTriggeredAndDelayed = True
admitsDelay TriggeredAndDelayed = True

||| One phrase, one slot: `Until (StartOf …)` already spells a turn
||| part's beginning, so the event form does not spell it a second time.
public export
spanEventOk : EventName -> Bool
spanEventOk PartBeginning = False
spanEventOk _ = True

public export
data ReplUse = Repeatedly | NextTimeOnly

public export
replUseOk : EventName -> ReplUse -> Bool
replUseOk Death Repeatedly = True
replUseOk Death NextTimeOnly = False
replUseOk Departure Repeatedly = True
replUseOk Departure NextTimeOnly = False
replUseOk Destruction Repeatedly = True
replUseOk Destruction NextTimeOnly = True
replUseOk DamageTaken Repeatedly = True
replUseOk DamageTaken NextTimeOnly = True
replUseOk CardDrawn Repeatedly = True
replUseOk CardDrawn NextTimeOnly = True
replUseOk GameLoss Repeatedly = True
replUseOk GameLoss NextTimeOnly = True
replUseOk SpellCast Repeatedly = False
replUseOk SpellCast NextTimeOnly = False
replUseOk Entry Repeatedly = True
replUseOk Entry NextTimeOnly = False
replUseOk AttackDeclaration Repeatedly = False
replUseOk AttackDeclaration NextTimeOnly = False
replUseOk BlockDeclaration Repeatedly = False
replUseOk BlockDeclaration NextTimeOnly = False
replUseOk CombatDamage Repeatedly = False
replUseOk CombatDamage NextTimeOnly = False
replUseOk PartBeginning Repeatedly = False
replUseOk PartBeginning NextTimeOnly = False
replUseOk StatusChange Repeatedly = False
replUseOk StatusChange NextTimeOnly = False
replUseOk TurnedFaceUp Repeatedly = False
replUseOk TurnedFaceUp NextTimeOnly = False
replUseOk PhasingChange Repeatedly = False
replUseOk PhasingChange NextTimeOnly = False
replUseOk BlockedDeclaration Repeatedly = False
replUseOk BlockedDeclaration NextTimeOnly = False
replUseOk LastCounterRemoval Repeatedly = False
replUseOk LastCounterRemoval NextTimeOnly = False
replUseOk Placement Repeatedly = True
replUseOk Placement NextTimeOnly = False
replUseOk CounterPlacement Repeatedly = True
replUseOk CounterPlacement NextTimeOnly = False
replUseOk CounterRemoval Repeatedly = False
replUseOk CounterRemoval NextTimeOnly = False
replUseOk LifeGain Repeatedly = False
replUseOk LifeGain NextTimeOnly = False
replUseOk LifeLoss Repeatedly = False
replUseOk LifeLoss NextTimeOnly = False
replUseOk TimeShift Repeatedly = False
replUseOk TimeShift NextTimeOnly = False
replUseOk TokenCreation Repeatedly = True
replUseOk TokenCreation NextTimeOnly = False
replUseOk ChapterArrival Repeatedly = False
replUseOk ChapterArrival NextTimeOnly = False
replUseOk AbilityActivation Repeatedly = False
replUseOk AbilityActivation NextTimeOnly = False


public export
data TriggerWord = When | Whenever | At

public export
triggerWordOk : EventName -> TriggerWord -> Bool
triggerWordOk Death When = True
triggerWordOk Death Whenever = True
triggerWordOk Death At = False
triggerWordOk Departure When = True
triggerWordOk Departure Whenever = True
triggerWordOk Departure At = False
triggerWordOk Destruction When = False
triggerWordOk Destruction Whenever = False
triggerWordOk Destruction At = False
triggerWordOk DamageTaken When = True
triggerWordOk DamageTaken Whenever = True
triggerWordOk DamageTaken At = False
triggerWordOk CardDrawn When = True
triggerWordOk CardDrawn Whenever = True
triggerWordOk CardDrawn At = False
triggerWordOk GameLoss When = True
triggerWordOk GameLoss Whenever = True
triggerWordOk GameLoss At = False
triggerWordOk Entry When = True
triggerWordOk Entry Whenever = True
triggerWordOk Entry At = False
triggerWordOk SpellCast When = True
triggerWordOk SpellCast Whenever = True
triggerWordOk SpellCast At = False
triggerWordOk AttackDeclaration When = True
triggerWordOk AttackDeclaration Whenever = True
triggerWordOk AttackDeclaration At = False
triggerWordOk BlockDeclaration When = True
triggerWordOk BlockDeclaration Whenever = True
triggerWordOk BlockDeclaration At = False
triggerWordOk CombatDamage When = True
triggerWordOk CombatDamage Whenever = True
triggerWordOk CombatDamage At = False
triggerWordOk PartBeginning When = False
triggerWordOk PartBeginning Whenever = False
triggerWordOk PartBeginning At = True
triggerWordOk StatusChange When = True
triggerWordOk StatusChange Whenever = True
triggerWordOk StatusChange At = False
triggerWordOk TurnedFaceUp When = True
triggerWordOk TurnedFaceUp Whenever = True
triggerWordOk TurnedFaceUp At = False
triggerWordOk PhasingChange When = True
triggerWordOk PhasingChange Whenever = True
triggerWordOk PhasingChange At = False
triggerWordOk BlockedDeclaration When = True
triggerWordOk BlockedDeclaration Whenever = True
triggerWordOk BlockedDeclaration At = False
triggerWordOk LastCounterRemoval When = True
triggerWordOk LastCounterRemoval Whenever = True
triggerWordOk LastCounterRemoval At = False
triggerWordOk Placement When = True
triggerWordOk Placement Whenever = True
triggerWordOk Placement At = False
triggerWordOk CounterPlacement When = True
triggerWordOk CounterPlacement Whenever = True
triggerWordOk CounterPlacement At = False
triggerWordOk CounterRemoval When = True
triggerWordOk CounterRemoval Whenever = True
triggerWordOk CounterRemoval At = False
triggerWordOk LifeGain When = True
triggerWordOk LifeGain Whenever = True
triggerWordOk LifeGain At = False
triggerWordOk LifeLoss When = True
triggerWordOk LifeLoss Whenever = True
triggerWordOk LifeLoss At = False
triggerWordOk TimeShift When = True
triggerWordOk TimeShift Whenever = True
triggerWordOk TimeShift At = False
triggerWordOk TokenCreation When = True
triggerWordOk TokenCreation Whenever = True
triggerWordOk TokenCreation At = False
triggerWordOk ChapterArrival When = True
triggerWordOk ChapterArrival Whenever = False
triggerWordOk ChapterArrival At = False
triggerWordOk AbilityActivation When = True
triggerWordOk AbilityActivation Whenever = True
triggerWordOk AbilityActivation At = False

public export
lookbackSubjectOk : EventName -> Kind -> Bool
lookbackSubjectOk Death Object = True
lookbackSubjectOk Death Player = False
lookbackSubjectOk Departure Object = True
lookbackSubjectOk Departure Player = False
lookbackSubjectOk Destruction Object = False
lookbackSubjectOk Destruction Player = False
lookbackSubjectOk DamageTaken Object = True
lookbackSubjectOk DamageTaken Player = True
lookbackSubjectOk CardDrawn Object = False
lookbackSubjectOk CardDrawn Player = True
lookbackSubjectOk GameLoss Object = False
lookbackSubjectOk GameLoss Player = False
lookbackSubjectOk Entry Object = True
lookbackSubjectOk Entry Player = False
lookbackSubjectOk AttackDeclaration Object = True
lookbackSubjectOk AttackDeclaration Player = True
lookbackSubjectOk BlockDeclaration Object = True
lookbackSubjectOk BlockDeclaration Player = False
lookbackSubjectOk CombatDamage Object = True
lookbackSubjectOk CombatDamage Player = False
lookbackSubjectOk PartBeginning Object = False
lookbackSubjectOk PartBeginning Player = False
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
lookbackSubjectOk LastCounterRemoval Object = False
lookbackSubjectOk LastCounterRemoval Player = False
lookbackSubjectOk Placement Object = False
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
lookbackSubjectOk _ (Quality _) = False
lookbackSubjectOk _ Outcome = False
lookbackSubjectOk _ Gap = False
lookbackSubjectOk _ TurnRef = False
lookbackSubjectOk _ Ability = False
lookbackSubjectOk _ (Letter _) = False
-- deferred, not decided: the joined kind has no payload until
-- workbench-joined-kind-binding lands one.
lookbackSubjectOk _ ObjectOrPlayer = False

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
lookbackComplementOk DamageTaken _ _ = False
lookbackComplementOk CombatDamage Object Player = True
lookbackComplementOk CombatDamage _ _ = False
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
lookbackComplementOk _ _ _ = False

public export
bareLookbackOk : EventName -> Kind -> Bool
bareLookbackOk Death Object = True
bareLookbackOk Departure Object = True
bareLookbackOk DamageTaken Object = True
bareLookbackOk DamageTaken Player = True
bareLookbackOk CardDrawn Player = True
bareLookbackOk Entry Object = True
bareLookbackOk AttackDeclaration Object = True
bareLookbackOk AttackDeclaration Player = True
bareLookbackOk BlockDeclaration Object = True
bareLookbackOk BlockedDeclaration Object = True
bareLookbackOk SpellCast Player = True
bareLookbackOk LifeGain Player = True
bareLookbackOk LifeLoss Player = True
-- the victim cannot be omitted: a bare "dealt combat damage" names no event.
bareLookbackOk CombatDamage Object = False
-- the object cannot be omitted: a bare "created" names no event.
bareLookbackOk TokenCreation Player = False
-- the ability cannot be omitted: a bare "activated" names no event.
bareLookbackOk AbilityActivation Player = False
-- permissive default: a lookback's complement may be omitted unless
-- naming the event requires it, as the two exceptions above do.
bareLookbackOk _ _ = True

public export
data LookbackComplement : EventName -> Kind -> Kind -> Type where
  MkLookbackComplement : {auto 0 ok : So (lookbackComplementOk ev ks kc)} ->
                         LookbackComplement ev ks kc


||| [CR#603.2b] triggers every "at the beginning of" ability as that
||| phase or step begins. A turn is neither a phase nor a step
||| [CR#500.1], so no header names a turn's beginning.
public export
partTriggerOk : TurnPart -> Bool
partTriggerOk Turn = False
partTriggerOk _ = True

||| [CR#500.8] adds a phase to a turn and [CR#500.9] adds a step to a
||| phase, each relative to another phase or step; a turn is neither
||| [CR#500.1], so it is no part to add, to follow, or to anchor on.
public export
partAddable : TurnPart -> Bool
partAddable Turn = False
partAddable _ = True


public export
data DamageKind = AnyDamage | CombatOnly | NoncombatOnly


public export
data Deed = Attack | Block

public export
data Role = Agent | Patient

public export
data CompTag = ForbidT | RequireT | GateT

public export
data PatientNeed = PatientRefused | PatientOptional | PatientRequired

public export
notRequired : PatientNeed -> Bool
notRequired PatientRequired = False
notRequired PatientOptional = True
notRequired PatientRefused = True

public export
admitsPatient : PatientNeed -> Bool
admitsPatient PatientRefused = False
admitsPatient PatientOptional = True
admitsPatient PatientRequired = True

public export
deonticPatientOk : CompTag -> Deed -> Role -> PatientNeed
deonticPatientOk ForbidT Attack Agent = PatientRefused
deonticPatientOk ForbidT Attack Patient = PatientRefused
deonticPatientOk ForbidT Block Agent = PatientOptional
deonticPatientOk ForbidT Block Patient = PatientRefused
deonticPatientOk RequireT Attack Agent = PatientRefused
deonticPatientOk RequireT Attack Patient = PatientRefused
deonticPatientOk RequireT Block Agent = PatientRequired
deonticPatientOk RequireT Block Patient = PatientRefused
deonticPatientOk GateT Attack Agent = PatientRefused
deonticPatientOk GateT Attack Patient = PatientRefused
deonticPatientOk GateT Block Agent = PatientOptional
deonticPatientOk GateT Block Patient = PatientRefused

public export
deedType : Deed -> Role -> CardType -> Bool
deedType Attack Agent Creature = True
deedType Attack Agent Artifact = False
deedType Attack Agent Land = False
deedType Attack Agent Enchantment = False
deedType Attack Agent Instant = False
deedType Attack Agent Sorcery = False
deedType Attack Agent Planeswalker = False
deedType Attack Agent Battle = False
deedType Attack Agent Kindred = False
deedType Attack Patient Creature = False
deedType Attack Patient Artifact = False
deedType Attack Patient Land = False
deedType Attack Patient Enchantment = False
deedType Attack Patient Instant = False
deedType Attack Patient Sorcery = False
deedType Attack Patient Planeswalker = True
deedType Attack Patient Battle = True
deedType Attack Patient Kindred = False
deedType Block Agent Creature = True
deedType Block Agent Artifact = False
deedType Block Agent Land = False
deedType Block Agent Enchantment = False
deedType Block Agent Instant = False
deedType Block Agent Sorcery = False
deedType Block Agent Planeswalker = False
deedType Block Agent Battle = False
deedType Block Agent Kindred = False
deedType Block Patient Creature = True
deedType Block Patient Artifact = False
deedType Block Patient Land = False
deedType Block Patient Enchantment = False
deedType Block Patient Instant = False
deedType Block Patient Sorcery = False
deedType Block Patient Planeswalker = False
deedType Block Patient Battle = False
deedType Block Patient Kindred = False

public export
data DeedParticipant : Deed -> Role -> Maybe CardType -> Type where
  Participant : {auto 0 ok : So (deedType d r t)} -> DeedParticipant d r (Just t)

public export
data StaticKind = PtDelta | KeywordGrant | DeedRestriction | TypeAddition
                | ControlGrant | Replacement | Prevention
                | Conditional | PlayPermission | EntryRider
                | CostModification
                | PtDefinition | BasePtSet | PtSwitch
                | TypeSet | AbilityLoss | Coordination
                | CopyEffect
                | VisibilityRider
                | LandAllowance
                | TurnSkip

public export
data CondMarking = AsLongAs | Unless

public export
data PlayVerb = Play | Cast

public export
data PlayAsThough = HadFlash

public export
data PlayLimit = OnceEachYourTurn | OnceEachTurn


public export
castComplementOk : Maybe Zone -> Bool
castComplementOk Nothing = True
castComplementOk (Just Battlefield) = False
-- a cast complement names what casting will make the object (a spell),
-- not where it is [CR#701.5b].
castComplementOk (Just Stack) = True
castComplementOk (Just Graveyard) = True
castComplementOk (Just Exile) = True
castComplementOk (Just Hand) = True
castComplementOk (Just Library) = True

public export
castableTy : PlayVerb -> Maybe CardType -> Bool
castableTy Play _ = True
castableTy Cast Nothing = True
castableTy Cast (Just Creature) = True
castableTy Cast (Just Artifact) = True
castableTy Cast (Just Land) = False
castableTy Cast (Just Enchantment) = True
castableTy Cast (Just Instant) = True
castableTy Cast (Just Sorcery) = True
castableTy Cast (Just Planeswalker) = True
castableTy Cast (Just Battle) = True
castableTy Cast (Just Kindred) = True

public export
staticAsAbility : StaticKind -> Bool
staticAsAbility CopyEffect = False
staticAsAbility PtDelta = True
staticAsAbility KeywordGrant = True
staticAsAbility DeedRestriction = True
staticAsAbility TypeAddition = True
staticAsAbility ControlGrant = False
staticAsAbility Replacement = True
staticAsAbility Prevention = True
staticAsAbility Conditional = True
staticAsAbility PlayPermission = True
staticAsAbility VisibilityRider = True
staticAsAbility LandAllowance = True
staticAsAbility TurnSkip = True
staticAsAbility EntryRider = True
staticAsAbility CostModification = True
staticAsAbility PtDefinition = True
staticAsAbility BasePtSet = True
staticAsAbility PtSwitch = False
staticAsAbility TypeSet = True
staticAsAbility AbilityLoss = True
staticAsAbility Coordination = True

public export
playableFrom : Maybe Zone -> Bool
playableFrom Nothing = True
playableFrom (Just Battlefield) = False
playableFrom (Just Graveyard) = True
playableFrom (Just Exile) = True
playableFrom (Just Hand) = True
playableFrom (Just Library) = True
-- an object on the stack has already been cast, so nothing may be
-- played from it [CR#112.1].
playableFrom (Just Stack) = False

public export
PlayableFrom : Maybe Zone -> Type
PlayableFrom z = So (playableFrom z)

public export
-- a complement's zone locates its object exactly when that zone could
-- have been the play source, so this reuses playableFrom's table.
complementLocates : Maybe Zone -> Bool
complementLocates z = playableFrom z

public export
data CastableTy : PlayVerb -> Maybe CardType -> Type where
  MkCastableTy : {auto 0 ok : So (castableTy v ty)} -> CastableTy v ty

public export
data ChoiceMode : Bindings -> Type where
  Unmarked : ChoiceMode bs
  TheirChoice : {auto 0 ch : countChoosers bs = 1} -> ChoiceMode bs
  AtRandom : ChoiceMode bs
  YourChoice : ChoiceMode bs


||| only Hand, Graveyard, and Library are per-player zones; the rest are
||| shared [CR#400.1].
public export
data Possessable : Zone -> Type where
  HandIsOwned : Possessable Hand
  GraveyardIsOwned : Possessable Graveyard
  LibraryIsOwned : Possessable Library
