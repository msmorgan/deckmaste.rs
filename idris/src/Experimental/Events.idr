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
data EventName = Death | Departure | DamageTaken
               | CardDrawn | Entry | AttackDeclaration | BlockDeclaration
               | CombatDamage | PartBeginning | SpellCast | StatusChange
               | TurnedFaceUp | PhasingChange | BlockedDeclaration
               -- the two ends of an attachment, as two names on
               -- `CounterPlacement`/`CounterRemoval`'s model: [CR#701.3a]
               -- puts an Aura, Equipment or Fortification onto an object
               -- and [CR#701.3d] moves an Equipment away from one, and a
               -- name-keyed table has to be able to answer about each on
               -- its own.
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
               -- the flipping ACT, which is not either arm of the call:
               -- [CR#705.1] makes flipping a coin a thing an effect
               -- instructs and [CR#705.2] reads a winner off it only
               -- afterwards, so the act happens even where no player
               -- wins or loses. It is the event a replacement reaches
               -- ("If you would flip a coin, instead ...").
               | CoinFlip
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
               -- read `verbFacts` instead. `Destruction` retired into
               -- this arm as `VerbedAct "Destroy"` [CR#701.8a].
               | VerbedAct VerbLabel
               -- a game STATE being true rather than an event happening
               -- [CR#603.8]. Its own name because it is not any event's:
               -- the header that writes it names no occurrence, so every
               -- table keyed by an event's participants, zones or moment
               -- answers this name negatively and says why once.
               | StateMatch
               -- a triggered ability TRIGGERING. [CR#603.2] makes the
               -- match between a game event and a trigger condition its
               -- own moment -- the ability "automatically triggers" and
               -- does nothing yet -- and [CR#603.2d] is a rule about how
               -- many times that moment happens, so the rules count it.
               -- Its own name and not `AbilityActivation`'s: [CR#113.3c]
               -- puts a triggered ability on the stack without anyone
               -- activating it, which is the difference the two names
               -- record.
               | AbilityTrigger

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

||| Which die a roll event NAMES. [CR#706.1] has a rolling instruction
||| specify what kind of die to roll, and a header may repeat that kind
||| back ("one or more six-sided dice", "one or more planar dice", "the
||| planar die") or leave it out.
||| `AnyDie` is the kind left UNWRITTEN and not "some numbered die":
||| [CR#706.7] and [CR#901.9d] both say that rolling the planar die
||| causes any ability that triggers whenever a player rolls one or more
||| dice to trigger, so the unnarrowed header watches the planar roll
||| too, and narrowing is what a written kind does.
||| `SidedDie` carries [CR#706.1a]'s positivity on `SidesOf`'s model. It
||| is the die that rule DESCRIBES -- N equally likely outcomes numbered
||| from 1 to N -- which is why `PlanarDie` is a row beside it and not
||| `SidedDie 6`, even though [CR#901.3a] calls the planar die
||| six-sided: its faces carry no numbers.
||| Spelling only, like `DiceBatch`, and Bindings-free: a header writes
||| the kind outright or not at all, and the anaphoric kind `DieSides`
||| carries is the replacement BODY's word, never the watched event's.
||| -- spelling: with `SidedDie`, "[n]-sided dice" or "d[n]"; with
||| `PlanarDie`, "the planar die" / "planar dice"; `AnyDie` writes
||| nothing.
public export
data RolledDie : Type where
  AnyDie : RolledDie
  SidedDie : (n : Nat) -> {auto 0 nz : IsSucc n} -> RolledDie
  PlanarDie : RolledDie

||| Whether a roll of the named die leaves a number behind. Every
||| numbered die does [CR#706.2]; the planar die does not, because
||| [CR#706.7] has any effect that refers to a numerical result of a die
||| roll -- including one that compares that result to a given number --
||| ignore the rolling of the planar die [CR#901.9d].
public export
dieHasResult : RolledDie -> Bool
dieHasResult PlanarDie = False
dieHasResult _ = True

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

||| The two directions a life total moves, as two events. [CR#119.3]
||| adjusts a player's life total for a gain and for a loss in one
||| sentence, so the two are one change read two ways rather than two
||| unrelated happenings; [CR#119.9] then writes the gain header in the
||| rules' own words ("Whenever [a player] gains life, . . . ."), which is
||| what makes the change watchable and not merely instructable.
||| Two names on `paymentEventName`'s model, so a name-keyed table can
||| answer about each direction alone.
public export
data LifeMove = LifeGoesUp | LifeGoesDown

public export
lifeEventName : LifeMove -> EventName
lifeEventName LifeGoesUp = LifeGain
lifeEventName LifeGoesDown = LifeLoss

||| The outcome each direction of a life change leaves for a following
||| clause to read: [CR#119.3] moves the total by an amount, and that
||| amount is what "that much life" names.
public export
lifeMoveOutcome : LifeMove -> OutcomeSort
lifeMoveOutcome LifeGoesUp = LifeGained
lifeMoveOutcome LifeGoesDown = LifeLost

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
-- two verbed acts are the same event exactly when they name the same
-- act, and the label is what names it: [CR#701.1] makes the keyword the
-- game term for the verb.
sameEventName (VerbedAct a) (VerbedAct b) = a == b
sameEventName (VerbedAct _) _ = False
-- [CR#603.8] separates a state trigger from every event outright --
-- these abilities trigger "rather than triggering when an event
-- occurs" -- and the states two headers watch are the conditions they
-- write, which this name does not carry. So no two state triggers are
-- taken to name one event.
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
-- would deal is that event. `DealsDamage` is the prospective row that
-- reaches this cell.
interceptOk DamageDealing = True
-- targets are chosen as the spell or ability is put on the stack
-- [CR#115.1], so becoming one is a thing that would happen; no printed
-- line replaces it, and no rule refuses the reading.
interceptOk BecomesTarget = True
-- an ability's TRIGGERING is not a thing a replacement reaches.
-- [CR#603.2] makes it automatic on the match, and the rules' own device
-- for changing it is [CR#603.2d], which counts the triggering rather
-- than replacing it. The two are not one mechanism said twice: a
-- replacement gets its one opportunity at the event "or any modified
-- events that may replace that event" [CR#614.5], while [CR#603.2d]
-- says a trigger multiplier "doesn't apply to other effects that affect
-- how many times an ability triggers" -- the opposite reach.
interceptOk AbilityTrigger = False
interceptOk _ = True

||| Which event a [CR#603.2d] trigger multiplier may count. One name:
||| the rule is about a triggered ability triggering additional times and
||| nothing else, so the table is the row's whole gate. Keyed by the
||| NAME, which is what lets the periphrastic causation carry it -- "if a
||| land entering causes a triggered ability … to trigger" names the
||| triggering through the wrapper.
public export
triggerCountOk : EventName -> Bool
triggerCountOk AbilityTrigger = True
triggerCountOk _ = False

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
-- `StatusChange` is the catch-all's one ATTESTED row, and it took one
-- line to attest: Vesuvan Shapeshifter's "until this creature is turned
-- face down" is the only supported endpoint naming a status transition
-- (measured 2026-08-28). It reads the transition through
-- `statusEventOk`, which the trigger header's own `statusHeaderOk`
-- refuses at that value -- the split is what lets this row be attested
-- while the header stays at zero.
spanEventOk _ = True

||| Which events have an INSIDE -- a span of ordered steps a player is
||| partway through, which is what the header's concurrent clause names
||| when it names an act rather than a state ("while you're activating a
||| craft ability", "while casting a spell with emerge", "while
||| scrying").
||| Three rules open the cells and no fourth is claimed. [CR#601.2] has a
||| player cast a spell by following "the steps listed below, in order",
||| so a spell being cast is a process with moments inside it;
||| [CR#602.2] says the same of activating an ability in the same words.
||| The keyword actions answer for themselves through `actStepwiseOf`:
||| [CR#701.22a] looks and THEN puts, which is the moment "while
||| scrying" names, while [CR#701.8a]'s destruction states one change and
||| leaves nothing between a beginning and an end.
||| Everything else is False, and the argument is [CR#603.2]'s: a game
||| event MATCHES a trigger event when it occurs, at a moment, and a
||| moment has no inside for "while" to name.
public export
eventUnderwayOk : EventName -> Bool
eventUnderwayOk SpellCast = True
eventUnderwayOk AbilityActivation = True
eventUnderwayOk (VerbedAct v) = actStepwiseOf v
eventUnderwayOk _ = False

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
-- [CR#701.3a] and [CR#701.3d] each move ONE object onto or away from
-- one other; neither states a number, so neither happens in an amount.
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
-- [CR#705.2] gives a flip a face and, when called, a winner, and nothing
-- numeric, so no arm of the call happens in an amount.
eventHasMagnitude FlipWin = False
eventHasMagnitude FlipLoss = False
-- nor does the flipping itself: [CR#705.1] makes a coin a two-sided
-- randomiser, so how many coins were flipped is a count and the flip
-- happens in no amount at all.
eventHasMagnitude CoinFlip = False
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
-- [CR#603.8]'s trigger condition is a game state being true, which
-- happens in no amount at all.
eventHasMagnitude StateMatch = False
-- [CR#603.2] has the ability trigger and do nothing at that point, so
-- the triggering carries no number; [CR#603.2d]'s multiplier is a COUNT
-- of triggerings, which is `EventCount`'s reading and not this one.
eventHasMagnitude AbilityTrigger = False

public export
data ReplUse = Repeatedly | NextTimeOnly

||| The three words a triggered ability may begin with. The list is
||| CLOSED by the rules and not by measurement: [CR#603.1] writes the
||| shape outright as "[When/Whenever/At] [trigger condition or event],
||| [effect]", and [CR#113.3c] repeats it -- a triggered ability
||| "include[s] (and usually begins with) the word 'when,' 'whenever,' or
||| 'at.'"
||| The corpus prints a fourth word, "After", and it is not a fourth row
||| and not a spelling of one of these three, because the lines that
||| write it are not triggered abilities at all. "After you roll a die,
||| you may pay 1 life. If you do, increase or decrease the result by 1"
||| (Xenosquirrels, Night Shift of the Living Dead) is a roll MODIFIER:
||| [CR#706.2] has the final result reached only "after considering all
||| applicable modifiers", and [CR#706.2b] names this very shape among
||| them -- "effects that modify the result of a die roll by increasing
||| or decreasing that result by a specified amount". The word marks
||| where in that sequence the modifier applies, not a trigger event.
||| The other two printed "After"s -- "After this main phase, there are
||| two additional combat phases" (Full Throttle, World at War) -- are
||| the added-phase statement, which is `AdditionalPart` and likewise no
||| header.
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
-- what becomes attached or unattached is an Aura, Equipment or
-- Fortification [CR#701.3a] -- an object, never a player. Both cells are
-- at zero in the corpus and the object cell is opened by the rule
-- rather than by a line, exactly as `Placement`'s is.
lookbackSubjectOk Attachment Object = True
lookbackSubjectOk Attachment Player = False
lookbackSubjectOk Unattachment Object = True
lookbackSubjectOk Unattachment Player = False
lookbackSubjectOk LastCounterRemoval Object = False
lookbackSubjectOk LastCounterRemoval Player = False
-- "a creature card was put into your graveyard from anywhere this turn":
-- [CR#400.7] makes a zone change a move of an OBJECT from one zone to
-- another, so what was put somewhere is an object and never a player.
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
-- [CR#705.2] gives the flip to "the player who flips the coin", so a
-- player is who flipped one; an object never flips.
lookbackSubjectOk CoinFlip Object = False
lookbackSubjectOk CoinFlip Player = True
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
-- nothing HAPPENED to look back on: [CR#603.8] has the ability trigger
-- on a game state rather than on an event, so no participant of it can
-- be named afterwards.
lookbackSubjectOk StateMatch Object = False
lookbackSubjectOk StateMatch Player = False
-- the ability that triggered is an object [CR#109.1] and would be a
-- nameable participant; no printed line looks back on one, and no rule
-- refuses it, so the object cell stands open at its zero. No player
-- triggers an ability [CR#113.3c], so its cell is closed.
lookbackSubjectOk AbilityTrigger Object = True
lookbackSubjectOk AbilityTrigger Player = False
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
-- an entry and a placement each name where the object went and where it
-- came from, and no second PARTICIPANT: [CR#400.7]'s move has an object
-- and two zones and nothing else. Their zones ride `FromZones`/`IntoZone`
-- instead, which is a payload and not a participant.
lookbackComplementOk Entry _ _ = False
lookbackComplementOk CardDrawn _ _ = False
lookbackComplementOk LifeGain _ _ = False
lookbackComplementOk LifeLoss _ _ = False
lookbackComplementOk Placement _ _ = False
-- the counter pair's complement names the KIND put or removed ("a +1/+1
-- counter was put on a permanent under your control this turn", 3 lines).
-- `QualitySort` carries that kind now, and the cells still stand shut on
-- two further counts: `lookbackSubjectOk` refuses both events at both
-- kinds, so no reader reaches one; and a noun at the kind sort spells
-- only "a kind of counter", no predicate naming the ONE kind these lines
-- write.
lookbackComplementOk CounterPlacement _ _ = False
lookbackComplementOk CounterRemoval _ _ = False
lookbackComplementOk AbilityActivation Player Ability = True
lookbackComplementOk AbilityActivation _ _ = False
-- neither flip arm nor a roll names a second participant: [CR#705.2]
-- involves no other player, and [CR#706.1]'s dice are no phrase the
-- grammar mentions.
lookbackComplementOk FlipWin _ _ = False
lookbackComplementOk FlipLoss _ _ = False
lookbackComplementOk CoinFlip _ _ = False
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
-- An act performed on a ZONE drops just as much by leaving it out:
-- [CR#701.23a] searches a NAMED zone, so a bare "if you searched this
-- way" leaves out the place, exactly as a bare "you discarded" leaves
-- out the card. So the cell is both facts, not the patient alone.
bareLookbackOk (VerbedAct v) Player =
  not (actNamesPatient v) && not (actNamesLocus v)
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
-- a placement is a move from one zone to ANOTHER [CR#400.7], and its
-- clause writes at least one of those ends -- "put into your graveyard",
-- "put there from the battlefield". With neither written, "was put this
-- turn" names no move. Same ground as the bare creation, one slot later:
-- the complement CAN be written here, so this is where dropping it is
-- refused.
bareLookbackOk Placement Object = False
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



||| Which end of a deed a participant is at.
public export
data Role = Agent | Patient

||| The deed's other participant: an attack's defender against its
||| attacker, a block's attacker against its blocker [CR#506.3].
public export
counterRole : Role -> Role
counterRole Agent = Patient
counterRole Patient = Agent

||| ==== The deed vocabulary ====
|||
||| A DEED is what a deontic statement permits, forbids, requires or
||| gates: attacking, being blocked, being targeted, casting, playing a
||| land, activating, countering, regenerating, gaining life, drawing,
||| searching, losing or winning the game. The slot is a `VerbLabel` and
||| the vocabulary is OPEN for `VerbLabel`'s own reasons -- a deed needs
||| no row in a total table and no rules entry of its own, only a name
||| and the facts its own rule states. The gate is `KnownDeed` against
||| `deedFacts`, `KnownVerb`'s twin: fail-closed membership, typo-safety
||| without a per-deed type, exhaustiveness traded for the gate.
|||
||| The closed `Deed = Attack | Block` this replaced cost 76 rows of
||| `deedType` to say what two rules say in two sentences, and had no
||| room for the eight deeds the corpus writes beside them.
|||
||| It is a SECOND table rather than rows in `verbFacts`, and the reason
||| is what each table is READ BY. `verbFacts` is the KEYWORD-ACTION
||| table [CR#701.1]: `Enact`, `Does`, the verbed event and the
||| participle anaphors all key off it, so a deed row added there would
||| make "you attack", "the targeted card" and "whenever you lose the
||| game" spellable at those four seats -- a widening no line asks for,
||| and the opposite of a fail-closed gate. The two tables share the
||| label TYPE, which is what keeps ONE open act vocabulary rather than
||| two parallel ones; they do not share the facts, because what is
||| asked of a keyword action (its participle, the zone its rule finds a
||| patient in, where it leaves it, whether it has steps) and what is
||| asked of a deed (who may fill each role, where, whether a defender
||| or a targeter rides, whether a counterfactual may) are different
||| questions with no field in common.

||| Whether the act may be narrowed by the "for mana" adjunct.
||| [CR#106.12] defines the phrase for ONE act and defines it as a
||| different act -- "to 'tap [a permanent] for mana' is to activate a
||| mana ability of that permanent that includes the {T} symbol in its
||| activation cost" -- so the adjunct is not an adverb that could ride
||| any verb, but a rule attaching a second reading to one of them.
||| Every other label answers False, which is the fail-closed answer and
||| the one that keeps "whenever you sacrifice a creature for mana" out
||| of the grammar.
|||
||| 58 supported lines write the narrowed act over 57 cards (measured
||| 2026-08-28) -- 23 in the passive ("whenever enchanted land is tapped
||| for mana") and 35 in the active ("whenever a player taps a land for
||| mana", "whenever you tap a land for mana"). One header serves both,
||| because `VerbedVoice` already makes the voice a spelling.
public export
verbForManaOk : VerbLabel -> Bool
verbForManaOk v = v == "Tap"

||| The SORT of premise [CR#609.4]'s counterfactual carries at a deed.
||| The rule states no single payload: its own body is about an object or
||| a player being treated as something it is not, and [CR#609.4b] gives
||| spending mana its own sentence and its own payload -- "as though it
||| were mana of any [type or color]", which affects "only how the player
||| may pay a cost" and is a matcher over mana rather than a description
||| of an object.
|||
||| So the sort is a DEED FACT and not a free choice at the carrier: what
||| a permission may counterfactualise is fixed by the deed's own rule,
||| and a premise of the wrong sort is a sentence the rules cannot read
||| ("this creature can attack as though it were mana of any color").
||| The alternative -- one payload union admitted everywhere -- would
||| make every such pairing spellable and pin nothing.
||| The third sort is the VALUE one: "this creature crews Vehicles as
||| though its power were 2 greater" counterfactualises a QUANTITY the
||| deed's own cost is paid in, not a description of an object and not a
||| mana matcher. [CR#702.122a] makes the crew cost a total power and
||| [CR#702.171a] does the same for saddle, so a premise at those deeds
||| has a number to shift and nothing else; a predicate there would say
||| what the creature IS, which is not what the sentence says.
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

||| One ROLE's facts. [CR#506.3] states attacking's and blocking's
||| outright -- "Only a creature can attack or block. Only a player, a
||| planeswalker, or a battle can be attacked" -- and every other deed's
||| own rule states its own.
public export
record DeedRole where
  constructor MkDeedRole
  ||| the KINDS of referent the deed's rule admits in this role: a
  ||| player casts [CR#601.2], an ability is activated [CR#602.2], a
  ||| creature attacks [CR#506.3]. Empty where nothing fills the role at
  ||| all, which is the fail-closed answer for a label with no row.
  roleKinds : List Kind
  ||| the card types the deed's rule admits in this role. Empty where
  ||| the role names no card ([CR#109.1] makes an ability on the stack
  ||| an object with no card type) or where nothing may fill it.
  roleTypes : List CardType
  ||| whether a participant whose card TYPE is unstated may fill the
  ||| role. False where the rule names the types outright ([CR#506.3]'s
  ||| two), True where what fills the role is a player [CR#400.1], an
  ||| ability [CR#109.1], or an object the rule describes some other way
  ||| (the spell of [CR#601.2], the card of [CR#701.23a]).
  roleBare : Bool
  ||| where the deed's own rule has the participant BE while it is
  ||| performed: [CR#506.3]'s combatants are on the battlefield,
  ||| [CR#601.2] puts a spell on the stack. `Nothing` where the rule
  ||| names no zone -- a player is in none [CR#400.1], and a target is
  ||| wherever it already is [CR#115.1].
  roleZone : Maybe Zone

||| The role nothing fills: what a deed with no agent, or no patient,
||| states about the end it does not have.
public export
noRole : DeedRole
noRole = MkDeedRole [] [] False Nothing

||| What a deed's label records beyond its name. The two roles are the
||| deed's own rule speaking; the four flags are what a statement may
||| write beside the deed.
public export
record DeedFacts where
  constructor MkDeedFacts
  ||| the label as a statement writes it
  deed : VerbLabel
  ||| who performs it
  deedAgent : DeedRole
  ||| who it is performed on
  deedPatient : DeedRole
  ||| whether the deed is aimed at a DEFENDER: [CR#506.3] lets an attack
  ||| name a player as well as a permanent, which is the one place a
  ||| deontic's other participant is not an object.
  deedDefends : Bool
  ||| whether the deed's other participant is a TARGETER -- a spell
  ||| [CR#115.1a] or an ability [CR#115.1c,115.1d] -- rather than a
  ||| participant of the deed's own kind.
  deedTargeted : Bool
  ||| which SORT of [CR#609.4] counterfactual may ride a permission of
  ||| this deed, and `Nothing` where none may. The rule's own bipartition
  ||| is "a player may do something 'as though' ... or a creature can do
  ||| something 'as though' ...", so the field records which deeds the
  ||| corpus writes one for; the SORT is [CR#609.4b]'s addition, which
  ||| gives the spend deed a mana matcher where the rest take a
  ||| description of an object.
  deedCounterfactual : Maybe PremiseSort
  ||| whether the deed spells the EFFECT-SCOPED rider (`CantBe`), which
  ||| is a different construction from a standing prohibition: the rider
  ||| denies one resolution's own consequence ("destroy it. It can't be
  ||| regenerated"), where the carrier states a continuous restriction.
  ||| Both read this one label vocabulary; only the flag differs.
  deedRides : Bool
  ||| whether a permission of this deed may carry the PLAY RIDER -- the
  ||| source zone, the per-window cap, the window, the exclusion and the
  ||| payment. [CR#601.3] is why the rider is the deed's and not the
  ||| carrier's: a player may begin to cast a spell only if a rule or
  ||| effect allows it, so the licence's own scope -- from where, how
  ||| often, when, and at what price -- is part of what an allowing
  ||| effect says, and only the two deeds an effect can allow this way
  ||| have anything to say it about. [CR#701.18a] gives playing a land
  ||| the same shape.
  deedPlays : Bool

||| The deed table, open by construction: a row is a name, two roles and
||| five flags, and adding one obliges nothing else.
public export
deedFacts : List DeedFacts
deedFacts =
  -- [CR#506.3]: "Only a creature can attack or block. Only a player, a
  -- planeswalker, or a battle can be attacked." The player half is
  -- `DefendingPlayer`'s, which is why the flag rides beside the types.
  --
  -- THE AGENT IS BARE-ADMITTING, and that is the deliberate reading of
  -- [CR#506.3] rather than a hole in it: the rule says which objects can
  -- perform the act, not how a sentence may DESCRIBE the one it is said
  -- of. The corpus settles it -- "Enchanted permanent can't attack,
  -- block, or crew Vehicles" (Bound in Gold, Intercessor's Arrest) and
  -- "This token can't block" name an object whose card type the noun
  -- never writes, and [CR#702.122d] states the first of those as a
  -- sentence an effect may make. What still refuses is a subject that
  -- WRITES a type the rule excludes ("enchanted land can't block",
  -- "enchanted planeswalker can't attack"), which is `deedTypeOk`'s
  -- half of the same row, and a subject off the battlefield, which is
  -- the zone's. The patient keeps its closed types: there the rule names
  -- what may be attacked or blocked and a description is checked against
  -- that list.
  [ MkDeedFacts "Attack"
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Planeswalker, Battle] False (Just Battlefield))
      True False (Just ObjectPremise) False False
  , MkDeedFacts "Block"
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Creature] False (Just Battlefield))
      False False (Just ObjectPremise) False False
  -- [CR#115.1] makes the targets "object(s) and/or player(s) the spell
  -- or ability will affect", and they are chosen wherever they are, so
  -- the patient's zone is unstated and its type is whatever the printed
  -- description says. The agent is the stack object
  -- [CR#115.1a,115.1c,115.1d], which is what `deedTargeted` says of it.
  , MkDeedFacts "Target"
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object, Player] [Creature, Artifact, Land, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True Nothing)
      False True (Just ObjectPremise) False False
  -- [CR#601.2] takes the spell from where it is and puts it on the
  -- stack; [CR#305.1] keeps a land off it -- "Since the land doesn't go
  -- on the stack, it is never a spell". The agent is the player who
  -- casts, in no zone [CR#400.1].
  , MkDeedFacts "Cast"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False (Just ObjectPremise) True True
  -- [CR#701.18a]: "To play a land means to put it onto the battlefield
  -- from the zone it's in" -- a special action, never on the stack, so
  -- the patient's zone is the one the clause names and not this rule's.
  -- The patient's TYPES are the land's and the castable ones together,
  -- because playing a CARD is defined as both: the glossary's own entry
  -- reads "to play a card is to play that card as a land or cast that
  -- card as a spell, whichever is appropriate", pointing at [CR#305.1]
  -- and [CR#601.2] for the two halves. So "you may play cards exiled
  -- with this Class" reaches every card either half admits, and the six
  -- types no half admits -- the ones [CR#311.2,312.2,313.2,314.2,315.3]
  -- and [CR#309.2c] each say outright can't be cast, none of them a land
  -- -- stay out, which the retired `castableTy Play` admitted on no rule
  -- at all.
  , MkDeedFacts "Play"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True Nothing)
      False False (Just ObjectPremise) True True
  -- [CR#701.6a] cancels a spell or ability and removes it from the
  -- stack, so both ends of the deed are stack objects.
  , MkDeedFacts "Counter"
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False Nothing True False
  -- [CR#707.10] puts a copy of a spell or ability onto the stack.
  , MkDeedFacts "Copy"
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False Nothing False False
  -- [CR#602.2] puts an ability on the stack and pays its costs; only
  -- the object's controller activates it. [CR#109.1] makes the ability
  -- an object with no card type, which is why the patient is bare with
  -- no types at all.
  , MkDeedFacts "Activate"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Ability] [] True Nothing)
      False False Nothing False False
  -- [CR#614.8] makes regeneration a destruction-replacement on a
  -- permanent, so the patient is on the battlefield.
  , MkDeedFacts "Regenerate"
      (MkDeedRole [] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing True False
  -- [CR#119.3] adjusts a PLAYER's life total; nothing is done to a
  -- second participant.
  , MkDeedFacts "GainLife"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False
  -- [CR#121.1] has a player put the top card of their library into
  -- their hand, so the deed's patient is that card, read where the rule
  -- finds it. The patient row exists for the COUNT CAP -- "can't draw
  -- more than one card each turn" counts cards, and a cap needs a role
  -- for what it counts.
  , MkDeedFacts "DrawCard"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [] True (Just Library))
      False False Nothing False False
  -- [CR#502.3] has the active player untap the PERMANENTS THEY CONTROL
  -- as a turn-based action, so both ends of the deed are named by that
  -- one sentence: a player at the agent, a battlefield permanent at the
  -- patient. The row exists for the count cap the untap step is the one
  -- deed to have printed nine times.
  -- [CR#701.21a]: "To sacrifice a permanent, its controller moves it
  -- from the battlefield directly to its owner's graveyard." Both ends
  -- come from that one sentence -- the controller at the agent, the
  -- battlefield permanent at the patient.
  , MkDeedFacts "Sacrifice"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing False False
  -- [CR#603.2] has the ability itself trigger -- "that ability
  -- automatically triggers" -- and nothing is done to a second
  -- participant. [CR#603.2a] says outright that triggering is neither
  -- casting nor activating, which is why the deed is its own label and
  -- not a reading of "Activate": "Ward abilities of those creatures
  -- don't trigger" is a statement no activation prohibition reaches.
  , MkDeedFacts "Trigger"
      (MkDeedRole [Ability] [] True Nothing) noRole
      False False Nothing False False
  , MkDeedFacts "Untap"
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing False False
  -- [CR#701.23a] looks at all cards in a zone and finds one; what a
  -- printed line writes after the verb is the zone, which is no `Kind`
  -- here, so the deed states only its agent.
  , MkDeedFacts "SearchLibrary"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False
  -- [CR#104.3] and [CR#104.2] name the ways a player loses and wins;
  -- the deed's agent is the player the outcome befalls and there is no
  -- second participant. Two labels rather than one signed row, because
  -- the corpus coordinates them under one subject ("players can't lose
  -- the game or win the game this turn") and the carrier's deed LIST is
  -- what that coordination is.
  , MkDeedFacts "LoseGame"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False
  , MkDeedFacts "WinGame"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False
  -- [CR#106.1]: "Players spend mana to pay costs." The agent is that
  -- player, in no zone [CR#400.1]. The patient is `noRole` and the
  -- omission is the rules': what is spent is MANA, and mana is a
  -- resource a pool holds rather than an object or a player, so no
  -- `Kind` here names it and none should -- the deed's other end is
  -- named by what the mana is spent ON, which is `SpendPurpose`'s
  -- business and rides the premise.
  -- The counterfactual sort is [CR#609.4b]'s: a permission of this deed
  -- says what already-made mana may COUNT AS while a cost is paid, which
  -- is a mana matcher and not a description of an object.
  , MkDeedFacts "Spend"
      (MkDeedRole [Player] [] True Nothing) noRole
      False False (Just ManaPremise) False False
  -- [CR#702.122b]: "A creature 'crews a Vehicle' when it's tapped to pay
  -- the cost to activate a Vehicle's crew ability." Both ends come from
  -- that one sentence -- a creature at the agent, the Vehicle at the
  -- patient, both on the battlefield, a Vehicle being an artifact
  -- [CR#702.122a]. [CR#702.122d] states the prohibition as a sentence an
  -- effect may make: "if an effect states that a creature 'can't crew
  -- Vehicles,' that creature can't be tapped to pay the crew cost".
  -- The counterfactual sort is the VALUE one, because what the printed
  -- permission counterfactualises is the crewing creature's POWER --
  -- the quantity the crew cost is paid in -- and not what the creature
  -- is.
  , MkDeedFacts "Crew"
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Artifact] True (Just Battlefield))
      False False (Just ValuePremise) False False
  -- [CR#702.171c]: "A creature 'saddles' a permanent as it's tapped to
  -- pay the cost to activate a permanent's saddle ability", and
  -- [CR#702.171b] leaves the patient at "only permanents can be or
  -- become saddled" -- every permanent type and no card type excluded,
  -- which is why the patient's list is the battlefield's own. Saddle's
  -- own cost is a total power too [CR#702.171a], so it reads the same
  -- premise sort.
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

||| The membership gate: typo-safety without a per-deed type, and the
||| whole of what exhaustiveness was traded for.
public export
knownDeed : VerbLabel -> Bool
knownDeed v = isJust (deedFactsFor v)

public export
KnownDeed : VerbLabel -> Type
KnownDeed v = So (knownDeed v)

||| One end's facts, or the empty role for a label with no row -- which
||| is the fail-closed answer, since the empty role admits nothing.
public export
deedRoleOf : VerbLabel -> Role -> DeedRole
deedRoleOf v Agent = maybe noRole deedAgent (deedFactsFor v)
deedRoleOf v Patient = maybe noRole deedPatient (deedFactsFor v)

||| Whether a referent of that KIND may fill the role at all. Asked
||| before the type, and the reason "spells can't be activated" and
||| "abilities can't be cast" stay refused now that no per-act subject
||| type indexes the gate.
public export
deedKindOk : VerbLabel -> Role -> Kind -> Bool
deedKindOk v r k = elem k (roleKinds (deedRoleOf v r))

||| Whether a participant of a stated card type may fill the role.
public export
deedTypeOk : VerbLabel -> Role -> CardType -> Bool
deedTypeOk v r t = elem t (roleTypes (deedRoleOf v r))

||| Whether a participant whose card type is unstated may fill it.
public export
deedBareOk : VerbLabel -> Role -> Bool
deedBareOk v r = roleBare (deedRoleOf v r)

||| Where the deed's rule has the role's participant be.
public export
deedZoneOf : VerbLabel -> Role -> Maybe Zone
deedZoneOf v r = roleZone (deedRoleOf v r)

public export
deedDefendsOk : VerbLabel -> Bool
deedDefendsOk v = maybe False deedDefends (deedFactsFor v)

public export
deedTargetedOk : VerbLabel -> Bool
deedTargetedOk v = maybe False deedTargeted (deedFactsFor v)

||| Which premise sort the deed admits, and `Nothing` where it admits
||| none -- the fail-closed answer for a label with no row.
public export
deedPremiseSort : VerbLabel -> Maybe PremiseSort
deedPremiseSort v = deedFactsFor v >>= deedCounterfactual

||| Whether a counterfactual may ride at all, at any sort.
public export
deedCounterfactualOk : VerbLabel -> Bool
deedCounterfactualOk v = isJust (deedPremiseSort v)

||| Whether the deed admits a counterfactual of THIS sort. A coordination
||| is asked of every deed in it, which is what keeps a premise from
||| riding a list one of whose deeds reads a different payload.
public export
deedPremiseOk : PremiseSort -> VerbLabel -> Bool
deedPremiseOk s v = deedPremiseSort v == Just s

public export
deedRidesOk : VerbLabel -> Bool
deedRidesOk v = maybe False deedRides (deedFactsFor v)

||| Whether a permission of the deed may carry the play rider. Asked of
||| every deed a coordination names, which is what keeps the rider off a
||| list one of whose deeds has nothing to say it about.
public export
deedPlaysOk : VerbLabel -> Bool
deedPlaysOk v = maybe False deedPlays (deedFactsFor v)

||| A statement's deeds: one label, or the several a single subject and
||| a single modality coordinate ("can't attack or block", "can't lose
||| the game or win the game this turn"). The coordination lives HERE,
||| at the carrier, rather than at each family that wants one: the deed,
||| the gate and the effect-scoped rider all read this list.
public export
Deeds : Type
Deeds = List VerbLabel

||| Every deed in the list is known. An empty list passes this and is
||| refused by `IsSucc` at the carrier instead, which is where the
||| emptiness is a statement about the SENTENCE rather than the labels.
public export
knownDeeds : Deeds -> Bool
knownDeeds ds = all knownDeed ds

public export
KnownDeeds : Deeds -> Type
KnownDeeds ds = So (knownDeeds ds)

||| Whether a list of labels names each deed once. `distinctClasses`'
||| content at the deed vocabulary, and asked at both seats that build a
||| list of them: a coordination that wrote one deed twice would forbid
||| the same act twice, and a per-conjunct complement written at one deed
||| twice would give that conjunct two objects.
public export
distinctDeeds : Deeds -> Bool
distinctDeeds [] = True
distinctDeeds (d :: ds) = not (elem d ds) && distinctDeeds ds

||| A coordination's shared zone: the zone every coordinated deed puts
||| the role's participant in, and `Nothing` where they disagree or
||| where none states one. `zoneFits` reads `Nothing` on the DESCRIPTION
||| side as "no zone stated", so a disagreeing coordination is not
||| constrained by zone at all -- what refuses those is the KIND and TYPE
||| gate, which `DeedParticipant` asks of EVERY deed in the list. No
||| printed line coordinates deeds whose rules name different zones.
public export
deedsZone : Deeds -> Role -> Maybe Zone
deedsZone [] r = Nothing
deedsZone (d :: ds) r =
  case deedsZone ds r of
    Nothing => if null ds then deedZoneOf d r else Nothing
    Just z => if deedZoneOf d r == Just z then Just z else Nothing

||| The participant gate, asked of every coordinated deed at once.
public export
data DeedParticipant : Deeds -> Role -> Kind -> Maybe CardType -> Type where
  ||| the participant whose card type the phrase states: every deed's
  ||| own rule must admit that kind and that type in that role.
  Participant : {auto 0 kk : So (all (\d => deedKindOk d r k) ds)} ->
                {auto 0 ok : So (all (\d => deedTypeOk d r t) ds)} ->
                DeedParticipant ds r k (Just t)
  ||| the participant whose card type the phrase does not state -- a
  ||| player, an ability [CR#109.1], or an object described by
  ||| something other than its type ("spells with the chosen name").
  ||| Refused wherever the deed's rule names the types outright FOR THAT
  ||| ROLE -- which after the agent-typing decision is every patient and
  ||| no agent: [CR#506.3] says which objects may attack or block, not
  ||| how a sentence may describe the one it is said of, and the printed
  ||| "enchanted permanent can't attack, block, or crew Vehicles" and
  ||| "this token can't block" name no card type at all. What the
  ||| decision costs is one refusal: a head that names SEVERAL types, one
  ||| of them excluded ("target creature or land can't block"), reads
  ||| `nounTy` as `Nothing` and is no longer refused, because a
  ||| coordinated head collapses to no type here and the disjuncts'
  ||| types are not asked for anywhere. `badCantDisjunctSubject` retired
  ||| with it; the typed refusals it stood beside (a land host, a
  ||| planeswalker attacker) are unaffected.
  BareParticipant : {auto 0 kk : So (all (\d => deedKindOk d r k) ds)} ->
                    {auto 0 ok : So (all (\d => deedBareOk d r) ds)} ->
                    DeedParticipant ds r k Nothing

public export
data StaticKind = PtDelta | KeywordGrant | DeedRestriction | TypeAddition
                | ControlGrant | Replacement | Prevention
                | Conditional | EntryRider
                | CostModification
                -- the statement that keeps mana in a pool past the
                -- emptying [CR#106.4] names as the end of each step and
                -- phase. Its own kind: it restricts no deed, grants
                -- nothing to an object, and replaces no event -- what it
                -- overrides is a rule of the game.
                | ManaPersistence
                | PtDefinition | BasePtSet | PtSwitch
                | TypeSet | TypeLoss | ColorSet | AbilityLoss | Coordination
                | CopyEffect
                | VisibilityRider
                | OutcomeImmunity
                | LandAllowance
                -- the block allowance: [CR#509.1a]'s blocker declaration
                -- raised, the land allowance's object-sorted sibling.
                | BlockAllowance
                -- the extra vote: [CR#701.38d]'s allowance, the land and
                -- block allowances' third sibling and their subject sort
                -- again. Its own kind because what it adds to is a
                -- PROCEDURE and not a permission -- the rule says the
                -- extra votes "all happen at the same time the player
                -- would otherwise have voted", which is a statement
                -- about when [CR#701.38a]'s turn-order pass reaches the
                -- voter, and neither allowance says anything of the
                -- kind.
                | VoteAllowance
                -- the recurring untap grant: an addition to
                -- [CR#502.3]'s turn-based action, which no other kind
                -- names.
                | UntapGrant
                -- the trigger multiplier [CR#603.2d]. Its own kind and
                -- NOT one of the three allowances, which the additional
                -- word made it look like: an allowance raises a limit on
                -- what a PLAYER may do, and this effect gives no one a
                -- choice -- the rule has the game "determine how many
                -- times it should trigger" and then trigger that many.
                -- Nor a `Replacement`: `interceptOk` says why at the
                -- name.
                | TriggerMultiplier
                | TurnSkip
                | LetterDefinition

||| Which WORD marks a static conditional: "as long as", "unless", or
||| "if". The word is carried and not derived, because the same
||| construction writes two of them one card apart -- Gwaihir the
||| Windlord's "this spell costs {2} less to cast AS LONG AS you've drawn
||| two or more cards this turn" beside Avenge's "this spell costs {2}
||| less to cast IF a player attacked you during their last turn".
|||
||| `IfSo` IS THE SAME CONSTRUCTION AS `AsLongAs`, settled on the two
||| rules that could have made it a one-time test and do not.
||| [CR#601.2f] determines a total cost once and then locks it in ("if
||| effects would change the total cost after this time, they have no
||| effect"), which is the lock-in the "if" reading would need; but the
||| lock-in there is the TOTAL COST's, not the ability's, and
||| [CR#611.3a] denies a static ability's continuous effect any of its
||| own -- it "isn't 'locked in'; it applies at any given moment to
||| whatever its text indicates". A cost-modification static is
||| therefore read exactly once, at [CR#601.2f]'s determination step,
||| whichever word marks it, and the two words say the same thing about
||| the same statement. So the third arm is a marking WORD and not a
||| row: 145 supported lines write it over a cost modification
||| (139 postposed, 6 leading, measured 2026-08-28), of which 48 write
||| the "if it targets ..." restrictor that is the described subject's
||| own predicate (Ghostfire Blade's idiom) and 97 write a game-state
||| condition this marking is for.
|||
||| MEASURED ZERO, not a refusal: no supported line conditions a
||| non-cost static on a GAME STATE with "if". The 6 that write "if"
||| beside another static kind describe the SUBJECT ("if it's white",
||| "if it has flying", "if it devoured a creature"), which is a
||| predicate and not a condition. `markingOk` gates nothing here for
||| that reason -- [CR#611.3a] would read such a line as a standing
||| condition, so the sentence is rules-meaningful and the zero is a
||| count.
||| -- spelling: "as long as [c]", "unless [c]", "if [c]".
public export
data CondMarking = AsLongAs | Unless | IfSo

public export
data PlayLimit = OnceEachYourTurn | OnceEachTurn

||| A stretch of time a static play permission is confined to. NOT
||| `PlayLimit`: a limit caps HOW OFTEN the permission may be used inside
||| whatever time it functions ("once during each of your turns"), while a
||| window says WHEN it functions at all and caps nothing -- Muldrotha's
||| "during each of your turns, you may play a land and cast a permanent
||| spell of each permanent type" permits repeatedly inside the window
||| where Danitha's "once during each of your turns" permits one. The two
||| axes are written in the same sentence position and are not the same
||| fact, which is why the "once" is not read off this vocabulary.
||| [CR#601.3] gives the permission its own scope -- a player may begin to
||| cast a spell only if a rule or effect allows it -- so an effect that
||| allows within a stretch of time allows only there.
||| Two arms:
||| * a library search [CR#701.23a], during which a card in that library
|||   may be cast by a permission that functions from the library
|||   [CR#113.6b] (Panglacial Wurm). No subrule of [CR#701.23] states that
|||   a player may cast a spell mid-search; the search action itself is
|||   what the window names.
||| * the controller's own turns, 2 supported lines (Muldrotha, the
|||   Gravetide and Coram, the Undertaker).
||| -- spelling: "while searching your library"; "during each of your
||| turns, ".
public export
data PlayWindow = WhileSearchingLibrary | DuringEachOfYourTurns

||| Which limit and window may be written together. "Once during each of
||| your turns" is `PlayLimit`'s own word for the composite, so writing
||| the limit and the turn window together would spell the same turn
||| phrase twice for one sentence. Every other pairing stands.
public export
playWindowOk : Maybe PlayLimit -> Maybe PlayWindow -> Bool
-- the no-window case first, so a permission carrying only a limit
-- reduces without the limit having to be known.
playWindowOk _ Nothing = True
playWindowOk (Just OnceEachYourTurn) (Just DuringEachOfYourTurns) = False
playWindowOk _ _ = True

public export
PlayWindowOk : Maybe PlayLimit -> Maybe PlayWindow -> Type
PlayWindowOk l w = So (playWindowOk l w)


public export
playableFrom : Maybe Zone -> Bool
playableFrom Nothing = True
-- the battlefield stands OPEN. [CR#601.2a] moves the card "from where it
-- is" and excludes no zone, and [CR#601.3] leaves which zones a spell may
-- be cast from to whatever rule or effect grants the permission -- the
-- rules close no zone against one. No printed line writes a battlefield
-- cast, and a count is no refusal. (The one reading that DOES refuse a
-- battlefield word does it on its own rule: `complementLocates` below.
-- `castComplementOk` used to be a second; it refused the battlefield on
-- no rule at all and retired for that reason -- see `playSourceOk`.)
playableFrom (Just Battlefield) = True
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

||| Whether a complement's own sort word LOCATES its object, or only says
||| what playing it will make it. Its own table, no longer `playableFrom`
||| read twice: the two agree everywhere but the battlefield, and each
||| cell here stands on the rule that gives the word its zone.
||| [CR#112.1] makes a spell a card ON the stack, so "spell" names what a
||| cast produced; [CR#110.1] makes a permanent a card on the battlefield
||| and [CR#305.1] puts a played land there, so "creature"/"land"/
||| "permanent" name what a play produced. A word for where the act ENDS
||| locates nothing, which is why Garruk's Horde may cast "creature
||| spells" from the top of a library [CR#701.5b] without the two zones
||| disagreeing.
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

||| Which zone a PLACEMENT's clause may name as the zone the card landed
||| in -- "put into your graveyard", "put into the command zone". A
||| different table from the move instruction's `DestOk`: this DESCRIBES
||| where a card landed rather than instructing a move, so its scope over
||| possessed zones is free where `DestOk`'s is gated [CR#400.3].
||| The battlefield is excluded because a permanent's arrival there is a
||| different construction with its own rule: [CR#603.6a] writes that
||| event as putting permanents "onto the battlefield" and gives it the
||| enters-the-battlefield ability, which is `Entry` and not this event.
||| The corpus agrees at the surface -- 1,070 lines write "onto the
||| battlefield" and none writes "into" it -- but the rule is what
||| refuses.
public export
placementDestOk : Zone -> Bool
placementDestOk Graveyard = True
placementDestOk Exile = True
placementDestOk Library = True
placementDestOk Hand = True
placementDestOk Battlefield = False
-- [CR#903.9a] and [CR#903.9b] both write the move as putting the card
-- into the command zone, and Myth Unbound's header watches it.
placementDestOk Command = True
placementDestOk Stack = False

||| Which zone a PLACEMENT's clause may name as the zone the card came
||| from -- "from the battlefield", "from your hand or library".
public export
placementOriginOk : Zone -> Bool
placementOriginOk Battlefield = True
placementOriginOk Graveyard = True
placementOriginOk Library = True
placementOriginOk Exile = True
-- nothing refuses a move out of the command zone -- Hellkite Courser
-- prints one -- so the source stays open. No header among the 48
-- supported command-zone lines watches one; a measured zero, not a
-- refusal.
placementOriginOk Command = True
placementOriginOk Hand = False
placementOriginOk Stack = False

||| Which zone an ENTRY's clause may name as the zone the permanent came
||| from -- "if it entered from your library", "enters from a graveyard",
||| "enters from anywhere other than your hand". [CR#400.7] makes a zone
||| change a move from one zone to ANOTHER, so the battlefield is the one
||| zone a permanent cannot enter it from. Every other zone is one an
||| object reaches the battlefield from: [CR#608.3] puts a resolving
||| permanent spell there off the stack, and the printed lines write the
||| graveyard, exile, the hand and a library.
public export
entryOriginOk : Zone -> Bool
entryOriginOk Battlefield = False
entryOriginOk Graveyard = True
entryOriginOk Library = True
entryOriginOk Hand = True
entryOriginOk Exile = True
entryOriginOk Command = True
entryOriginOk Stack = True

||| Which zones an event's clause may name as its ORIGIN -- the
||| "from [zone]" written beside the event, prospectively on a trigger
||| header and retrospectively by `Happened`/`EventCount`/`HappenedTo`.
||| One table keyed on the event, consulted from both seats.
||| A cast admits exactly `playableFrom`'s zones, and for the same reason:
||| [CR#601.2a] moves the card out of the zone it was in, so the origin
||| the clause names is a zone it could have been cast from. [CR#903.8] is
||| the family that reads it back -- "for each previous time the player
||| casting it has cast it from the command zone that game".
||| A placement and an entry each carry their own table above.
||| The rest have both ends fixed by their own rule and write neither --
||| [CR#700.4] makes "dies" MEAN a move from the battlefield to a
||| graveyard, so a death clause names neither end.
public export
lookbackOriginOk : EventName -> Zone -> Bool
lookbackOriginOk SpellCast z = playableFrom (Just z)
lookbackOriginOk Placement z = placementOriginOk z
lookbackOriginOk Entry z = entryOriginOk z
lookbackOriginOk _ _ = False

||| Whether the event's clause names an origin at all. "From anywhere"
||| names one without naming a zone [CR#400.1], so it cannot ask the
||| table above for a cell and asks it for a row instead.
public export
eventNamesOrigin : EventName -> Bool
eventNamesOrigin ev =
  lookbackOriginOk ev Battlefield || lookbackOriginOk ev Graveyard ||
  lookbackOriginOk ev Library || lookbackOriginOk ev Hand ||
  lookbackOriginOk ev Exile || lookbackOriginOk ev Command ||
  lookbackOriginOk ev Stack

||| Which zone an event's clause may name as its DESTINATION. Only a
||| placement writes one: every other event this vocabulary names fixes
||| where its object ended by naming the event -- an entry ends on the
||| battlefield [CR#603.6a], a death in a graveyard [CR#700.4].
public export
lookbackDestOk : EventName -> Zone -> Bool
lookbackDestOk Placement z = placementDestOk z
lookbackDestOk _ _ = False

||| Which zone an event's clause may name as the place the act was
||| PERFORMED IN -- "if you search your library this way", "each player
||| who searched their library this way". Neither an origin nor a
||| destination: nothing moves out of the searched zone or into it, and
||| the card the search finds is moved by the instructing clause
||| [CR#701.23e]. So it is the complement's fourth payload sort and not a
||| reading of `FromZones`.
||| Only a verbed act writes one, and which zones it may name is the
||| ACT's own business rather than the zone's: [CR#701.23a] admits every
||| zone a search may look in and [CR#701.24a] names the library a
||| shuffle randomizes, so the table reads `actLoci` and adds no zone
||| axis of its own.
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


||| only Hand, Graveyard, and Library are per-player zones; the rest are
||| shared [CR#400.1].
public export
data Possessable : Zone -> Type where
  HandIsOwned : Possessable Hand
  GraveyardIsOwned : Possessable Graveyard
  LibraryIsOwned : Possessable Library
