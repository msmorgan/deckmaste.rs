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
               | StatValueChange | Regeneration

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
sameEventName StatValueChange StatValueChange = True
sameEventName StatValueChange _ = False
sameEventName Regeneration Regeneration = True
sameEventName Regeneration _ = False

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

||| [CR#614.1] hangs a replacement effect on an event that would
||| happen. A chapter's arrival is not an event: the chapter symbol is a
||| keyword ability standing for a triggered ability [CR#107.15], and the
||| event a replacement reaches there is the lore counter's placement
||| [CR#714.2b].
public export
interceptOk : EventName -> Bool
interceptOk ChapterArrival = False
interceptOk _ = True

||| One phrase, one slot: `Until (StartOf …)` already spells a turn
||| part's beginning, so the event form does not spell it a second time.
public export
spanEventOk : EventName -> Bool
spanEventOk PartBeginning = False
spanEventOk _ = True

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
lookbackSubjectOk Destruction Object = False
lookbackSubjectOk Destruction Player = False
lookbackSubjectOk DamageTaken Object = True
lookbackSubjectOk DamageTaken Player = True
lookbackSubjectOk CardDrawn Object = False
lookbackSubjectOk CardDrawn Player = True
lookbackSubjectOk GameLoss Object = False
-- [CR#603.10f] a game loss is looked back on.
lookbackSubjectOk GameLoss Player = True
lookbackSubjectOk Entry Object = True
lookbackSubjectOk Entry Player = False
lookbackSubjectOk AttackDeclaration Object = True
lookbackSubjectOk AttackDeclaration Player = True
lookbackSubjectOk BlockDeclaration Object = True
lookbackSubjectOk BlockDeclaration Player = False
lookbackSubjectOk CombatDamage Object = True
lookbackSubjectOk CombatDamage Player = False
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
-- a bare "became" names no event: the value it reached is the
-- complement, and a lookback carries none. Same ground as StatusChange.
lookbackSubjectOk StatValueChange Object = False
lookbackSubjectOk StatValueChange Player = False
lookbackSubjectOk Regeneration Object = True
lookbackSubjectOk Regeneration Player = False
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

||| A lookback names its event. No rule fixes which subject a lookback
||| reads it from, so the complement may be dropped unless dropping it
||| leaves no event named, as a transitive verb with no object does.
public export
bareLookbackOk : EventName -> Kind -> Bool
-- a creation is a creation of something: a bare "created" names nothing.
bareLookbackOk TokenCreation Player = False
-- an activation is an activation of something.
bareLookbackOk AbilityActivation Player = False
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

||| The deed's other participant: an attack's defender against its
||| attacker, a block's attacker against its blocker [CR#506.3].
public export
counterRole : Role -> Role
counterRole Agent = Patient
counterRole Patient = Agent

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
