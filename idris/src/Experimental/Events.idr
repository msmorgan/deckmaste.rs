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
                | TypeSet | TypeLoss | ColorSet | AbilityLoss | Coordination
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
-- the battlefield stands OPEN. [CR#601.2a] moves the card "from where it
-- is" and excludes no zone, and [CR#601.3] leaves which zones a spell may
-- be cast from to whatever rule or effect grants the permission -- the
-- rules close no zone against one. No printed line writes a battlefield
-- cast, and a count is no refusal. (The two readings that DO refuse a
-- battlefield word do it on their own rules: `complementLocates` below,
-- and `castComplementOk`.)
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
