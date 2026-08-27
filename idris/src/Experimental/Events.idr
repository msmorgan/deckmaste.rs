||| Event vocabulary and the decision tables (triggers, replacements, durations,
||| lookbacks) built on it.
module Experimental.Events

import Experimental.Words

%default total


||| The event CLASSIFIER, and deliberately the retrospective readers'
||| whole event vocabulary: `Happened`/`HappenedTo`/`EventCount`/`EventSum`
||| name an event kind, a subject and a window, because the lookback
||| clause's grammar is participial — `LookbackSubject` and
||| `LookbackComplement` are that grammar — and any richer narrowing rides
||| the subject's own predicate. The prospective readers (the trigger
||| header, `Intercepts`, `UntilEvent`, `Delayed`, `HeldUntil`, `ThisWay`)
||| take the full `GameEvent` term, and where a term meets a name-keyed
||| table it lifts through `eventName` (`Interceptable`, `durationOk`) —
||| the recorded pattern for any future widening.
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
               | FlipWin | FlipLoss
               | DiceRoll
               | CostPayment | CostNonpayment
               | LifePayment
               -- a thing being chosen as a target [CR#115.1]. Its own
               -- name and not a reading of `SpellCast`: [CR#115.1d]
               -- targets a triggered ability too, and [CR#702.21a]
               -- writes the header of the object that was targeted,
               -- which no casting event names.
               | BecomesTarget
               -- the DEALER's side of damage in general, which neither
               -- `DamageTaken` (the victim's side) nor `CombatDamage`
               -- (the combat dealer's) names. [CR#120.1] makes the object
               -- that deals damage the SOURCE of it, so the two sides are
               -- two events on `CombatDamage`'s own precedent, and not
               -- one event under a `Role` slot.
               | DamageDealing
               -- the EVENT reading of a keyword action, named by the act
               -- rather than by the transition it entails. The label is
               -- carried HERE rather than lifted into a row per verb:
               -- [CR#701.1] has a verb the rules never keyword use its
               -- standard English definition, so the vocabulary is open,
               -- and a lift would need a gate closing part of it again.
               -- The tables below that must tell one act from another
               -- read `verbFacts` instead.
               | VerbedAct VerbLabel

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

||| The two arms of a called flip [CR#705.2] as two events, on
||| `counterEventName`'s model: the call's arms are opposite outcomes, and
||| a name-keyed table has to be able to answer about each on its own.
public export
flipEventName : FlipCall -> EventName
flipEventName WinsFlip = FlipWin
flipEventName LosesFlip = FlipLoss

||| The determiner a roll event writes. Printed headers say both "a die"
||| and "one or more dice"; [CR#706.7] states the rules' own form of the
||| event with the plural. Spelling only -- the roll leaves one result to
||| read either way [CR#706.2].
public export
data DiceBatch = OneDie | ManyDice

||| The two outcomes of a stated cost's payment, as two events.
||| [CR#118.1] makes paying a cost an act a player carries out, and
||| [CR#118.12] reads a following "if [a player] doesn't" as a check on
||| whether that player chose to pay -- so declining is as determinate as
||| paying, and [CR#702.24a] writes exactly that pair into cumulative
||| upkeep. [CR#603.2] lets a header match a game event or a game state,
||| which is what admits the declined arm. Two names on `flipEventName`'s
||| model, so a name-keyed table can answer about each arm alone.
public export
data PaymentOutcome = Paid | Unpaid

public export
paymentEventName : PaymentOutcome -> EventName
paymentEventName Paid = CostPayment
paymentEventName Unpaid = CostNonpayment

||| Which keyword names a cost a clause may watch being paid: the ones
||| whose parameter IS a cost [CR#118.1]. A keyword with no cost parameter
||| -- flying, menace -- names nothing payable.
public export
KeywordCost : KeywordLabel -> Type
KeywordCost k = So (keywordCosts k)

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
-- two verbed acts are the same event exactly when they name the same
-- act, and the label is what names it: [CR#701.1] makes the keyword the
-- game term for the verb.
sameEventName (VerbedAct a) (VerbedAct b) = a == b
sameEventName (VerbedAct _) _ = False

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

||| [CR#614.1] hangs a replacement effect on an event that would
||| happen. A chapter's arrival is not an event: the chapter symbol is a
||| keyword ability standing for a triggered ability [CR#107.15], and the
||| event a replacement reaches there is the lore counter's placement
||| [CR#714.2b].
public export
interceptOk : EventName -> Bool
interceptOk ChapterArrival = False
-- an act is one of the actions a card's text describes [CR#701.1], so it
-- is a thing that would happen -- Bruvac the Grandiloquent's "if an
-- opponent would mill one or more cards".
interceptOk (VerbedAct _) = True
-- [CR#614.1] replaces an event that WOULD happen, and damage a source
-- would deal is that event. No prospective row spells the dealer's side
-- today, so the cell is stated rather than reached.
interceptOk DamageDealing = True
-- targets are chosen as the spell or ability is put on the stack
-- [CR#115.1], so becoming one is a thing that would happen; no printed
-- line replaces it, and no rule refuses the reading.
interceptOk BecomesTarget = True
interceptOk _ = True

||| One phrase, one slot: `Until (StartOf …)` already spells a turn
||| part's beginning, so the event form does not spell it a second time.
public export
spanEventOk : EventName -> Bool
spanEventOk PartBeginning = False
-- an act is a moment, so "until [who] discards a card" spells an endpoint
-- nothing else already writes.
spanEventOk (VerbedAct _) = True
-- "until [source] deals damage" is an endpoint no turn-part phrase
-- already spells.
spanEventOk DamageDealing = True
-- "until [n] becomes the target of a spell" is a moment no turn-part
-- phrase already spells [CR#603.2e].
spanEventOk BecomesTarget = True
spanEventOk _ = True

||| Which events HAPPEN in an amount, for the summed lookback to read:
||| damage is dealt in a number ([CR#120.8] makes a 0-damage deal no damage
||| event at all), life is gained in one ([CR#119.9] makes a 0-life gain no
||| life gain event) and lost in one ([CR#119.2] has a damaged player lose
||| "that much" life). Everything else is a
||| transition or an act with no number of its own -- a death is a zone
||| change [CR#700.4], not a quantity -- so "the amount of" has nothing
||| there to sum. A COUNT of such events is `EventCount`'s reading, not
||| this one.
public export
eventHasMagnitude : EventName -> Bool
eventHasMagnitude DamageTaken = True
eventHasMagnitude CombatDamage = True
-- the same number read from the dealer's side; [CR#120.8] makes a
-- 0-damage deal no event there either.
eventHasMagnitude DamageDealing = True
eventHasMagnitude LifeGain = True
eventHasMagnitude LifeLoss = True
eventHasMagnitude Death = False
eventHasMagnitude Departure = False
eventHasMagnitude Destruction = False
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
-- [CR#705.2] gives a flip a face and, when called, a winner, and nothing
-- numeric, so no arm of the call happens in an amount.
eventHasMagnitude FlipWin = False
eventHasMagnitude FlipLoss = False
-- a roll's number is not the amount the rolling happened in: [CR#706.2]
-- makes the result a number the roll PRODUCED, which is read back off
-- the roll itself, and how many dice were rolled is `EventCount`'s
-- reading.
eventHasMagnitude DiceRoll = False
-- what is paid is the cost the clause NAMES, whose size is the cost's
-- own [CR#118.1]; the paying happens in no number of its own, and
-- [CR#702.24a] refuses a partial payment outright.
eventHasMagnitude CostPayment = False
eventHasMagnitude CostNonpayment = False
-- a life payment is the one payment the rules give a number of its
-- own: [CR#118.3b] subtracts the indicated amount from a life total
-- and [CR#119.4] reads that back as losing that much life.
eventHasMagnitude LifePayment = True
-- a keyword action carries objects or looks at them; what varies is HOW
-- MANY, and [CR#603.2c] makes a multi-card act one event with that many
-- occurrences. Occurrences are counted, so "the number of cards
-- discarded this turn" is `EventCount`'s reading and no act of this
-- vocabulary happens in an amount to sum.
-- a target is DECLARED as the spell or ability goes on the stack
-- [CR#115.1]; the declaring carries no number of its own. What is
-- countable is how many times something was chosen [CR#115.9a], and a
-- count is `EventCount`'s reading.
eventHasMagnitude BecomesTarget = False
eventHasMagnitude (VerbedAct _) = False

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
-- "target creature that dealt damage to you this turn". [CR#120.1] gives
-- the dealing to an object and names no other dealer, so a player is
-- never what dealt damage.
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
-- [CR#705.2] gives the flip to the player who flipped it and to no one
-- else, so only a player is what won or lost one.
lookbackSubjectOk FlipWin Object = False
lookbackSubjectOk FlipWin Player = True
lookbackSubjectOk FlipLoss Object = False
lookbackSubjectOk FlipLoss Player = True
-- "if you rolled a die this turn": a player is who [CR#706.1] instructs
-- to roll.
lookbackSubjectOk DiceRoll Object = False
lookbackSubjectOk DiceRoll Player = True
-- a payment is a payment OF a stated cost, and the participial lookback
-- carries only a kind-to-kind complement, which cannot name the keyword
-- and the bearer that say WHICH cost. A bare "who paid this turn" names
-- no cost and so no event -- `bareLookbackOk`'s ground, reached one slot
-- earlier because the complement can never be written at all.
lookbackSubjectOk CostPayment Object = False
lookbackSubjectOk CostPayment Player = False
lookbackSubjectOk CostNonpayment Object = False
lookbackSubjectOk CostNonpayment Player = False
-- a life payment names its own paid thing, so a bare "paid life this
-- turn" leaves nothing unnamed; [CR#118.1] makes it a player's act,
-- and [CR#119.4] gives the life to that player alone.
lookbackSubjectOk LifePayment Object = False
lookbackSubjectOk LifePayment Player = True
-- an act's patient is what a participle names ([CR#701.17c] finds "a
-- milled card"), so the object side is open exactly where the act has a
-- patient. The player side is open everywhere: each act's own rule
-- states it of the player performing it [CR#701.17a,701.22a].
-- "creature that became the target of a spell this turn": [CR#115.1]
-- makes the targets objects and/or players, so either side is what was
-- targeted. Unwritten in the corpus -- no supported line spells any
-- retrospective reading of this event (0 of 98 headers) -- and refused
-- by no rule, so both cells stand open at their zero.
lookbackSubjectOk BecomesTarget Object = True
lookbackSubjectOk BecomesTarget Player = True
lookbackSubjectOk (VerbedAct v) Object = actPatientOf v == Just Object
lookbackSubjectOk (VerbedAct _) Player = True
lookbackSubjectOk _ (Quality _) = False
lookbackSubjectOk _ Outcome = False
lookbackSubjectOk _ Gap = False
lookbackSubjectOk _ TurnRef = False
lookbackSubjectOk _ Ability = False
lookbackSubjectOk _ (LetterK _) = False
-- a joined subject is a lookback subject only if BOTH halves are: "any
-- target" cannot be what died, since a player cannot die.
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
-- "each opponent and planeswalker it has dealt damage to this game" (The
-- Fallen): [CR#120.1] deals damage to a player, battle, creature or
-- planeswalker alike, so a phrase that may denote either side of that
-- list took it, and the dealer it names is the same one either way.
lookbackComplementOk DamageTaken (a \/ b) kc =
  lookbackComplementOk DamageTaken a kc && lookbackComplementOk DamageTaken b kc
lookbackComplementOk DamageTaken _ _ = False
lookbackComplementOk CombatDamage Object Player = True
lookbackComplementOk CombatDamage _ _ = False
-- the dealer's side names its recipient, which [CR#120.1] opens to a
-- battle, creature or planeswalker as well as a player -- "dealt damage
-- to it", "dealt damage to you". No dealer-side line writes a joined
-- complement, so this name takes the table's own refusal of one.
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
-- neither flip arm nor a roll names a second participant: [CR#705.2]
-- involves no other player, and [CR#706.1]'s dice are no phrase the
-- grammar mentions.
lookbackComplementOk FlipWin _ _ = False
lookbackComplementOk FlipLoss _ _ = False
lookbackComplementOk DiceRoll _ _ = False
-- life is the paid thing and the verb already carries it; the cost it
-- went to is no participant [CR#118.1].
lookbackComplementOk LifePayment _ _ = False
-- "you discarded a card this turn": the actor's complement is the act's
-- own patient and nothing else. Read from the patient's side the act is
-- already whole, so no second participant is named there.
-- the complement names what did the targeting, which [CR#115.1a,115.1c]
-- and [CR#115.1d] close to a spell or an ability; a player never
-- targets, since [CR#115.1] has a player choose the targets OF the
-- spell or ability. The joined complement is [CR#115.9b]'s own "spell or
-- ability" and rides the join the same way the subject side does -- this
-- name does not take the table's standing refusal of one, which no rule
-- backs here. All of it at its zero: the prospective header writes the
-- join 171 times and the retrospective reading not at all.
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

||| A lookback names its event. No rule fixes which subject a lookback
||| reads it from, so the complement may be dropped unless dropping it
||| leaves no event named, as a transitive verb with no object does.
public export
bareLookbackOk : EventName -> Kind -> Bool
-- a creation is a creation of something: a bare "created" names nothing.
bareLookbackOk TokenCreation Player = False
-- an activation is an activation of something.
bareLookbackOk AbilityActivation Player = False
-- a transitive act with its object dropped names nothing: a bare "you
-- discarded this turn" leaves out the card [CR#701.9a] asks for, while
-- "you scried this turn" is whole, because [CR#701.22a] gives the act no
-- patient to leave out. From the patient's own side the act is named
-- either way.
bareLookbackOk (VerbedAct v) Player = not (actNamesPatient v)
bareLookbackOk (VerbedAct _) Object = True
-- "target creature that dealt damage this turn", with the recipient
-- dropped, still names the event: [CR#120.1] makes every deal a deal to
-- something, so nothing is left indeterminate by leaving it out.
bareLookbackOk DamageDealing Object = True
-- "creature that became a target this turn", with the targeter dropped:
-- [CR#115.1] makes every target the target of some spell or ability, so
-- leaving it out leaves nothing indeterminate.
bareLookbackOk BecomesTarget Object = True
bareLookbackOk BecomesTarget Player = True
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
-- [CR#110.4] names no command-zone type among the permanent types, so
-- nothing of these types is ever on the battlefield to take either role.
deedType Attack Agent Conspiracy = False
deedType Attack Agent Dungeon = False
deedType Attack Agent Phenomenon = False
deedType Attack Agent Plane = False
deedType Attack Agent Scheme = False
deedType Attack Agent Vanguard = False
deedType Attack Patient Conspiracy = False
deedType Attack Patient Dungeon = False
deedType Attack Patient Phenomenon = False
deedType Attack Patient Plane = False
deedType Attack Patient Scheme = False
deedType Attack Patient Vanguard = False
deedType Block Agent Conspiracy = False
deedType Block Agent Dungeon = False
deedType Block Agent Phenomenon = False
deedType Block Agent Plane = False
deedType Block Agent Scheme = False
deedType Block Agent Vanguard = False
deedType Block Patient Conspiracy = False
deedType Block Patient Dungeon = False
deedType Block Patient Phenomenon = False
deedType Block Patient Plane = False
deedType Block Patient Scheme = False
deedType Block Patient Vanguard = False

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
                | LetterDefinition

public export
data CondMarking = AsLongAs | Unless

public export
data PlayVerb = Play | Cast

public export
data PlayAsThough = HadFlash

public export
data PlayLimit = OnceEachYourTurn | OnceEachTurn

||| A moment a static play permission is confined to. The one arm is a
||| library search [CR#701.23a], during which a card in that library may be
||| cast by a permission that functions from the library [CR#113.6b]
||| (Panglacial Wurm). No subrule of [CR#701.23] states that a player may
||| cast a spell mid-search; the search action itself is what the window
||| names.
public export
data PlayWindow = WhileSearchingLibrary


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
castComplementOk (Just Command) = True

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
-- [CR#311.2,312.2,313.2,314.2,315.3] and [CR#309.2c] each say outright
-- that the card can't be cast; a dungeon enters the game by the venture
-- keyword action instead [CR#309.2].
castableTy Cast (Just Conspiracy) = False
castableTy Cast (Just Dungeon) = False
castableTy Cast (Just Phenomenon) = False
castableTy Cast (Just Plane) = False
castableTy Cast (Just Scheme) = False
castableTy Cast (Just Vanguard) = False


public export
playableFrom : Maybe Zone -> Bool
playableFrom Nothing = True
playableFrom (Just Battlefield) = False
playableFrom (Just Graveyard) = True
playableFrom (Just Exile) = True
playableFrom (Just Hand) = True
playableFrom (Just Library) = True
-- [CR#903.8] states the permission outright: "a player may cast a
-- commander they own from the command zone".
playableFrom (Just Command) = True
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

||| Which zones a RETROSPECTIVE reader may name as an event's origin --
||| the "from [zone]" a `Happened`/`EventCount`/`HappenedTo` writes beside
||| the event it looks back on. A cast admits exactly `playableFrom`'s
||| zones, and for the same reason: [CR#601.2a] moves the card out of the
||| zone it was in, so the origin the clause names is a zone it could have
||| been cast from. [CR#903.8] is the family that reads it back -- "for
||| each previous time the player casting it has cast it from the command
||| zone that game" -- and the printed lines write the hand and a
||| graveyard alongside the command zone.
||| No other event names an origin here. A placement's clause does write
||| one ("put into your graveyard from the battlefield"), but
||| `lookbackSubjectOk` admits no subject for `Placement` at any kind, so
||| no reader reaches it; the rest have both ends fixed by their own rule
||| and write neither -- [CR#700.4] makes "dies" MEAN a move from the
||| battlefield to a graveyard, so a death clause names neither end.
public export
lookbackOriginOk : EventName -> Zone -> Bool
lookbackOriginOk SpellCast z = playableFrom (Just z)
lookbackOriginOk _ _ = False

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
