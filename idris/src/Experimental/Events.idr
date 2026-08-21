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

public export
eventSpan : EventName -> SpanUse
eventSpan Death = Unattested
eventSpan Departure = Unclaimed
eventSpan Destruction = Unattested
eventSpan DamageTaken = Unattested
eventSpan CardDrawn = Unattested
eventSpan GameLoss = Unattested
eventSpan Entry = Unattested
eventSpan AttackDeclaration = Unattested
eventSpan BlockDeclaration = Unattested
eventSpan CombatDamage = Unattested
eventSpan SpellCast = Unattested
eventSpan PartBeginning = Unclaimed
eventSpan StatusChange = Unattested
eventSpan TurnedFaceUp = Unattested
eventSpan PhasingChange = Unattested
eventSpan BlockedDeclaration = Unattested
eventSpan LastCounterRemoval = Unattested
eventSpan Placement = Unattested
eventSpan CounterPlacement = Unattested
eventSpan CounterRemoval = Unattested
eventSpan LifeGain = Unattested
eventSpan LifeLoss = Unattested
eventSpan TimeShift = Unattested
eventSpan TokenCreation = Unattested
eventSpan ChapterArrival = Unattested

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
  MkLookbackSubject : {auto 0 ok : lookbackSubjectOk ev k = True} ->
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
-- permissive default: a lookback's complement may be omitted unless
-- naming the event requires it, as the two exceptions above do.
bareLookbackOk _ _ = True

public export
data LookbackComplement : EventName -> Kind -> Kind -> Type where
  MkLookbackComplement : {auto 0 ok : lookbackComplementOk ev ks kc = True} ->
                         LookbackComplement ev ks kc


public export
data PartUse = PartUnattested | PartUnclaimed | PartTriggered

public export
partUse : TurnPart -> Maybe Owner -> PartUse
partUse Turn Nothing = PartUnattested
partUse Turn (Just Yours) = PartUnattested
partUse Turn (Just ThatPlayers) = PartUnattested
partUse Turn (Just EachPlayers) = PartUnattested
partUse Turn (Just EachOpponents) = PartUnattested
partUse Turn (Just EachYours) = PartUnattested
partUse Turn (Just AnOpponents) = PartUnattested
partUse Turn (Just ThatTurns) = PartUnattested
partUse Upkeep Nothing = PartUnattested
partUse Upkeep (Just Yours) = PartTriggered
partUse Upkeep (Just ThatPlayers) = PartUnclaimed
partUse Upkeep (Just EachPlayers) = PartTriggered
partUse Upkeep (Just EachOpponents) = PartTriggered
partUse Upkeep (Just EachYours) = PartUnattested
partUse Upkeep (Just AnOpponents) = PartUnattested
partUse Upkeep (Just ThatTurns) = PartUnattested
partUse EndStep Nothing = PartTriggered
partUse EndStep (Just Yours) = PartTriggered
partUse EndStep (Just ThatPlayers) = PartUnattested
partUse EndStep (Just EachPlayers) = PartTriggered
partUse EndStep (Just EachOpponents) = PartTriggered
partUse EndStep (Just EachYours) = PartUnattested
partUse EndStep (Just AnOpponents) = PartUnattested
partUse EndStep (Just ThatTurns) = PartTriggered
partUse Combat Nothing = PartUnattested
partUse Combat (Just Yours) = PartTriggered
partUse Combat (Just ThatPlayers) = PartUnattested
partUse Combat (Just EachPlayers) = PartTriggered
partUse Combat (Just EachOpponents) = PartUnattested
partUse Combat (Just EachYours) = PartUnattested
partUse Combat (Just AnOpponents) = PartUnattested
partUse Combat (Just ThatTurns) = PartUnattested
partUse UntapStep Nothing = PartUnattested
partUse UntapStep (Just Yours) = PartUnattested
partUse UntapStep (Just ThatPlayers) = PartUnattested
partUse UntapStep (Just EachPlayers) = PartUnattested
partUse UntapStep (Just EachOpponents) = PartUnattested
partUse UntapStep (Just EachYours) = PartUnattested
partUse UntapStep (Just AnOpponents) = PartUnattested
partUse UntapStep (Just ThatTurns) = PartUnattested
partUse EndOfCombat Nothing = PartTriggered
partUse EndOfCombat (Just Yours) = PartUnattested
partUse EndOfCombat (Just ThatPlayers) = PartUnattested
partUse EndOfCombat (Just EachPlayers) = PartUnattested
partUse EndOfCombat (Just EachOpponents) = PartUnattested
partUse EndOfCombat (Just EachYours) = PartUnattested
partUse EndOfCombat (Just AnOpponents) = PartUnattested
partUse EndOfCombat (Just ThatTurns) = PartUnattested
partUse FirstMain Nothing = PartUnattested
partUse FirstMain (Just Yours) = PartTriggered
partUse FirstMain (Just ThatPlayers) = PartUnattested
partUse FirstMain (Just EachPlayers) = PartTriggered
partUse FirstMain (Just EachOpponents) = PartUnattested
partUse FirstMain (Just EachYours) = PartUnattested
partUse FirstMain (Just AnOpponents) = PartUnattested
partUse FirstMain (Just ThatTurns) = PartUnattested
partUse PostcombatMain Nothing = PartUnattested
partUse PostcombatMain (Just Yours) = PartTriggered
partUse PostcombatMain (Just ThatPlayers) = PartUnattested
partUse PostcombatMain (Just EachPlayers) = PartUnattested
partUse PostcombatMain (Just EachOpponents) = PartUnattested
partUse PostcombatMain (Just EachYours) = PartTriggered
partUse PostcombatMain (Just AnOpponents) = PartUnattested
partUse PostcombatMain (Just ThatTurns) = PartUnattested
partUse DrawStep Nothing = PartUnattested
partUse DrawStep (Just Yours) = PartTriggered
partUse DrawStep (Just ThatPlayers) = PartUnattested
partUse DrawStep (Just EachPlayers) = PartTriggered
partUse DrawStep (Just EachOpponents) = PartUnattested
partUse DrawStep (Just EachYours) = PartUnattested
partUse DrawStep (Just AnOpponents) = PartUnattested
partUse DrawStep (Just ThatTurns) = PartUnattested
partUse MainPhase Nothing = PartUnclaimed
partUse MainPhase (Just Yours) = PartUnclaimed
partUse MainPhase (Just ThatPlayers) = PartUnattested
partUse MainPhase (Just EachPlayers) = PartUnattested
partUse MainPhase (Just EachOpponents) = PartUnattested
partUse MainPhase (Just EachYours) = PartTriggered
partUse MainPhase (Just AnOpponents) = PartUnattested
partUse MainPhase (Just ThatTurns) = PartUnattested

public export
admitsPartTrigger : PartUse -> Bool
admitsPartTrigger PartUnattested = False
admitsPartTrigger PartUnclaimed = False
admitsPartTrigger PartTriggered = True


public export
data SkipUse = SkipUnattested | SkipScheduled | SkipStanding | SkipBoth

public export
skipUse : TurnPart -> SkipUse
skipUse Turn = SkipScheduled
skipUse Upkeep = SkipStanding
skipUse EndStep = SkipUnattested
skipUse Combat = SkipScheduled
skipUse UntapStep = SkipBoth
skipUse EndOfCombat = SkipUnattested
skipUse FirstMain = SkipUnattested
skipUse PostcombatMain = SkipUnattested
skipUse DrawStep = SkipBoth
skipUse MainPhase = SkipUnattested

public export
admitsScheduledSkip : SkipUse -> Bool
admitsScheduledSkip SkipUnattested = False
admitsScheduledSkip SkipScheduled = True
admitsScheduledSkip SkipStanding = False
admitsScheduledSkip SkipBoth = True

public export
admitsStandingSkip : SkipUse -> Bool
admitsStandingSkip SkipUnattested = False
admitsStandingSkip SkipScheduled = False
admitsStandingSkip SkipStanding = True
admitsStandingSkip SkipBoth = True


public export
data AddUse = AddUnwritten | AddedAndAnchors | FollowsAndAnchors | AddedOnly

public export
addUse : TurnPart -> AddUse
addUse Turn = AddUnwritten
addUse Upkeep = AddUnwritten
addUse EndStep = AddedOnly
addUse Combat = AddedAndAnchors
addUse UntapStep = AddUnwritten
addUse EndOfCombat = AddUnwritten
addUse FirstMain = AddUnwritten
addUse PostcombatMain = AddUnwritten
addUse DrawStep = AddUnwritten
addUse MainPhase = FollowsAndAnchors

public export
admitsAdded : AddUse -> Bool
admitsAdded AddUnwritten = False
admitsAdded AddedAndAnchors = True
admitsAdded FollowsAndAnchors = False
admitsAdded AddedOnly = True

public export
admitsFollower : AddUse -> Bool
admitsFollower AddUnwritten = False
admitsFollower AddedAndAnchors = False
admitsFollower FollowsAndAnchors = True
admitsFollower AddedOnly = False

public export
admitsAnchor : AddUse -> Bool
admitsAnchor AddUnwritten = False
admitsAnchor AddedAndAnchors = True
admitsAnchor FollowsAndAnchors = True
admitsAnchor AddedOnly = False


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
  Participant : {auto 0 ok : deedType d r t = True} -> DeedParticipant d r (Just t)

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
admitsSpan : StaticKind -> SpanUse -> Bool
admitsSpan CopyEffect Unattested = False
admitsSpan CopyEffect Unclaimed = False
admitsSpan CopyEffect GrantsControlAndTypeSet = False
admitsSpan CopyEffect GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan CopyEffect KeywordGrantAndTypeSet = False
admitsSpan CopyEffect RestrictionsShieldsPermissionsAndDelays = False
admitsSpan CopyEffect GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan CopyEffect ControlGrantAndPermission = False
admitsSpan CopyEffect GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan CopyEffect PermissionOnly = False
admitsSpan PtDelta Unattested = False
admitsSpan PtDelta Unclaimed = False
admitsSpan PtDelta GrantsControlAndTypeSet = True
admitsSpan PtDelta GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan PtDelta KeywordGrantAndTypeSet = False
admitsSpan PtDelta RestrictionsShieldsPermissionsAndDelays = False
admitsSpan PtDelta GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan PtDelta ControlGrantAndPermission = False
admitsSpan PtDelta GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan PtDelta PermissionOnly = False
admitsSpan KeywordGrant Unattested = False
admitsSpan KeywordGrant Unclaimed = False
admitsSpan KeywordGrant GrantsControlAndTypeSet = True
admitsSpan KeywordGrant GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan KeywordGrant KeywordGrantAndTypeSet = True
admitsSpan KeywordGrant RestrictionsShieldsPermissionsAndDelays = False
admitsSpan KeywordGrant GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan KeywordGrant ControlGrantAndPermission = False
admitsSpan KeywordGrant GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan KeywordGrant PermissionOnly = False
admitsSpan DeedRestriction Unattested = False
admitsSpan DeedRestriction Unclaimed = False
admitsSpan DeedRestriction GrantsControlAndTypeSet = False
admitsSpan DeedRestriction GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsSpan DeedRestriction KeywordGrantAndTypeSet = False
admitsSpan DeedRestriction RestrictionsShieldsPermissionsAndDelays = True
admitsSpan DeedRestriction GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan DeedRestriction ControlGrantAndPermission = False
admitsSpan DeedRestriction GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan DeedRestriction PermissionOnly = False
admitsSpan TypeAddition Unattested = False
admitsSpan TypeAddition Unclaimed = False
admitsSpan TypeAddition GrantsControlAndTypeSet = False
admitsSpan TypeAddition GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan TypeAddition KeywordGrantAndTypeSet = False
admitsSpan TypeAddition RestrictionsShieldsPermissionsAndDelays = False
admitsSpan TypeAddition GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan TypeAddition ControlGrantAndPermission = False
admitsSpan TypeAddition GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan TypeAddition PermissionOnly = False
admitsSpan ControlGrant Unattested = False
admitsSpan ControlGrant Unclaimed = False
admitsSpan ControlGrant GrantsControlAndTypeSet = True
admitsSpan ControlGrant GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan ControlGrant KeywordGrantAndTypeSet = False
admitsSpan ControlGrant RestrictionsShieldsPermissionsAndDelays = False
admitsSpan ControlGrant GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan ControlGrant ControlGrantAndPermission = True
admitsSpan ControlGrant GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan ControlGrant PermissionOnly = False
admitsSpan Replacement Unattested = False
admitsSpan Replacement Unclaimed = False
admitsSpan Replacement GrantsControlAndTypeSet = False
admitsSpan Replacement GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan Replacement KeywordGrantAndTypeSet = False
admitsSpan Replacement RestrictionsShieldsPermissionsAndDelays = True
admitsSpan Replacement GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan Replacement ControlGrantAndPermission = False
admitsSpan Replacement GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan Replacement PermissionOnly = False
admitsSpan Prevention Unattested = False
admitsSpan Prevention Unclaimed = False
admitsSpan Prevention GrantsControlAndTypeSet = False
admitsSpan Prevention GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsSpan Prevention KeywordGrantAndTypeSet = False
admitsSpan Prevention RestrictionsShieldsPermissionsAndDelays = True
admitsSpan Prevention GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan Prevention ControlGrantAndPermission = False
admitsSpan Prevention GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan Prevention PermissionOnly = False
admitsSpan Conditional Unattested = False
admitsSpan Conditional Unclaimed = False
admitsSpan Conditional GrantsControlAndTypeSet = False
admitsSpan Conditional GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsSpan Conditional KeywordGrantAndTypeSet = False
admitsSpan Conditional RestrictionsShieldsPermissionsAndDelays = False
admitsSpan Conditional GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan Conditional ControlGrantAndPermission = False
admitsSpan Conditional GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan Conditional PermissionOnly = False
admitsSpan PlayPermission Unattested = False
admitsSpan PlayPermission Unclaimed = False
admitsSpan PlayPermission GrantsControlAndTypeSet = False
admitsSpan PlayPermission GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan PlayPermission KeywordGrantAndTypeSet = False
admitsSpan PlayPermission RestrictionsShieldsPermissionsAndDelays = True
admitsSpan PlayPermission GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan PlayPermission ControlGrantAndPermission = True
admitsSpan PlayPermission GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan PlayPermission PermissionOnly = True
admitsSpan VisibilityRider Unattested = False
admitsSpan VisibilityRider Unclaimed = False
admitsSpan VisibilityRider GrantsControlAndTypeSet = False
admitsSpan VisibilityRider GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan VisibilityRider KeywordGrantAndTypeSet = False
admitsSpan VisibilityRider RestrictionsShieldsPermissionsAndDelays = True
admitsSpan VisibilityRider GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan VisibilityRider ControlGrantAndPermission = False
admitsSpan VisibilityRider GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan VisibilityRider PermissionOnly = False
admitsSpan LandAllowance Unattested = False
admitsSpan LandAllowance Unclaimed = False
admitsSpan LandAllowance GrantsControlAndTypeSet = False
admitsSpan LandAllowance GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsSpan LandAllowance KeywordGrantAndTypeSet = False
admitsSpan LandAllowance RestrictionsShieldsPermissionsAndDelays = True
admitsSpan LandAllowance GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan LandAllowance ControlGrantAndPermission = False
admitsSpan LandAllowance GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan LandAllowance PermissionOnly = False
admitsSpan TurnSkip Unattested = False
admitsSpan TurnSkip Unclaimed = False
admitsSpan TurnSkip GrantsControlAndTypeSet = False
admitsSpan TurnSkip GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsSpan TurnSkip KeywordGrantAndTypeSet = False
admitsSpan TurnSkip RestrictionsShieldsPermissionsAndDelays = False
admitsSpan TurnSkip GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan TurnSkip ControlGrantAndPermission = False
admitsSpan TurnSkip GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan TurnSkip PermissionOnly = False
admitsSpan EntryRider Unattested = False
admitsSpan EntryRider Unclaimed = False
admitsSpan EntryRider GrantsControlAndTypeSet = False
admitsSpan EntryRider GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsSpan EntryRider KeywordGrantAndTypeSet = False
admitsSpan EntryRider RestrictionsShieldsPermissionsAndDelays = False
admitsSpan EntryRider GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan EntryRider ControlGrantAndPermission = False
admitsSpan EntryRider GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan EntryRider PermissionOnly = False
admitsSpan CostModification Unattested = False
admitsSpan CostModification Unclaimed = False
admitsSpan CostModification GrantsControlAndTypeSet = False
admitsSpan CostModification GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsSpan CostModification KeywordGrantAndTypeSet = False
admitsSpan CostModification RestrictionsShieldsPermissionsAndDelays = False
admitsSpan CostModification GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan CostModification ControlGrantAndPermission = False
admitsSpan CostModification GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan CostModification PermissionOnly = False
admitsSpan PtDefinition Unattested = False
admitsSpan PtDefinition Unclaimed = False
admitsSpan PtDefinition GrantsControlAndTypeSet = False
admitsSpan PtDefinition GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsSpan PtDefinition KeywordGrantAndTypeSet = False
admitsSpan PtDefinition RestrictionsShieldsPermissionsAndDelays = False
admitsSpan PtDefinition GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan PtDefinition ControlGrantAndPermission = False
admitsSpan PtDefinition GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan PtDefinition PermissionOnly = False
admitsSpan BasePtSet Unattested = False
admitsSpan BasePtSet Unclaimed = False
admitsSpan BasePtSet GrantsControlAndTypeSet = False
admitsSpan BasePtSet GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan BasePtSet KeywordGrantAndTypeSet = False
admitsSpan BasePtSet RestrictionsShieldsPermissionsAndDelays = False
admitsSpan BasePtSet GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan BasePtSet ControlGrantAndPermission = False
admitsSpan BasePtSet GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan BasePtSet PermissionOnly = False
admitsSpan PtSwitch Unattested = False
admitsSpan PtSwitch Unclaimed = False
admitsSpan PtSwitch GrantsControlAndTypeSet = False
admitsSpan PtSwitch GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan PtSwitch KeywordGrantAndTypeSet = False
admitsSpan PtSwitch RestrictionsShieldsPermissionsAndDelays = False
admitsSpan PtSwitch GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsSpan PtSwitch ControlGrantAndPermission = False
admitsSpan PtSwitch GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsSpan PtSwitch PermissionOnly = False
admitsSpan TypeSet Unattested = False
admitsSpan TypeSet Unclaimed = False
admitsSpan TypeSet GrantsControlAndTypeSet = True
admitsSpan TypeSet GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan TypeSet KeywordGrantAndTypeSet = True
admitsSpan TypeSet RestrictionsShieldsPermissionsAndDelays = False
admitsSpan TypeSet GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan TypeSet ControlGrantAndPermission = False
admitsSpan TypeSet GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan TypeSet PermissionOnly = False
admitsSpan AbilityLoss Unattested = False
admitsSpan AbilityLoss Unclaimed = False
admitsSpan AbilityLoss GrantsControlAndTypeSet = False
admitsSpan AbilityLoss GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan AbilityLoss KeywordGrantAndTypeSet = False
admitsSpan AbilityLoss RestrictionsShieldsPermissionsAndDelays = False
admitsSpan AbilityLoss GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan AbilityLoss ControlGrantAndPermission = False
admitsSpan AbilityLoss GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan AbilityLoss PermissionOnly = False
admitsSpan Coordination Unattested = True
admitsSpan Coordination Unclaimed = True
admitsSpan Coordination GrantsControlAndTypeSet = True
admitsSpan Coordination GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = True
admitsSpan Coordination KeywordGrantAndTypeSet = True
admitsSpan Coordination RestrictionsShieldsPermissionsAndDelays = True
admitsSpan Coordination GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = True
admitsSpan Coordination ControlGrantAndPermission = True
admitsSpan Coordination GrantsRestrictionsControlPermissionTypeSetAndLoss = True
admitsSpan Coordination PermissionOnly = True

public export
absentOk : StaticKind -> Bool
absentOk CopyEffect = True
absentOk PtDelta = True
absentOk KeywordGrant = True
absentOk DeedRestriction = False
absentOk TypeAddition = True
absentOk ControlGrant = True
absentOk Replacement = False
absentOk Prevention = False
absentOk Conditional = False
absentOk PlayPermission = False
absentOk VisibilityRider = False
absentOk LandAllowance = False
absentOk TurnSkip = False
absentOk EntryRider = False
absentOk CostModification = False
absentOk PtDefinition = False
absentOk BasePtSet = True
absentOk PtSwitch = False
absentOk TypeSet = True
absentOk AbilityLoss = True
absentOk Coordination = True

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
data PlayableFrom : Maybe Zone -> Type where
  MkPlayableFrom : {auto 0 ok : playableFrom z = True} -> PlayableFrom z

public export
-- a complement's zone locates its object exactly when that zone could
-- have been the play source, so this reuses playableFrom's table.
complementLocates : Maybe Zone -> Bool
complementLocates z = playableFrom z

public export
data CastableTy : PlayVerb -> Maybe CardType -> Type where
  MkCastableTy : {auto 0 ok : castableTy v ty = True} -> CastableTy v ty

public export
admitsDelaySpan : SpanUse -> Bool
admitsDelaySpan Unattested = False
admitsDelaySpan Unclaimed = False
admitsDelaySpan GrantsControlAndTypeSet = False
admitsDelaySpan GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss = False
admitsDelaySpan KeywordGrantAndTypeSet = False
admitsDelaySpan RestrictionsShieldsPermissionsAndDelays = True
admitsDelaySpan GrantsRestrictionsReplacementBaseSetTypeSetAndLoss = False
admitsDelaySpan ControlGrantAndPermission = False
admitsDelaySpan GrantsRestrictionsControlPermissionTypeSetAndLoss = False
admitsDelaySpan PermissionOnly = False

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
