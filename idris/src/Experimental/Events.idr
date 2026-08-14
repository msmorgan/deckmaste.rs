module Experimental.Events

import Experimental.Words

%default total

-- ===== Event patterns (the vocabulary the "would" clause and the
-- "until" clause share) =====

||| The events oracle names when it intercepts one, waits for one, or
||| TRIGGERS off one — the vocabulary four constructions read and no rule
||| enumerates. Closed and row-enumerated, so a new event is a totality
||| error on every table below before anything can be written with it.
|||
||| Which events are HERE is the corpus's answer, counted over the whole
||| supported corpus with the moods counted separately because they are
||| different constructions. What sits below the minted rows is ledgered
||| with its count rather than minted: the put-into-a-graveyard event, the
||| becomes-the-target event, the create event, and the life-change pair.
|||
||| The NAMES are the event class and not any construction's spelling,
||| which is chapter twenty-eight's correction: `GameEvent`'s rows carry
||| the finite verb phrase ("dies", "enters") and this key carries the
||| happening it names ("Death", "Entry"), because the round that gave the
||| vocabulary a third and fourth reader is the round the old
||| `Would`-prefixed spelling stopped being true of half of them.
|||
||| Core spells this space as ONE open filter type (`EventFilter`,
||| `deckmaste_core/src/event.rs`) shared by triggers, replacements,
||| durations, and condition lookbacks. The MERGE is taken here — the
||| delayed clause's own query type is gone and reads this vocabulary like
||| everyone else — and what stays apart from core is the table below,
||| which records that the four constructions still disagree about which
||| events they write.
public export
-- spelling: (construction-owned -- each row NAMES a happening whose verb
-- phrase, subject and mood the reading construction supplies: the
-- interception writes it after "would" ("would die", "would be
-- destroyed"), the held-until rider and the trigger header write it
-- finite ("leaves the battlefield", "dies"). Spelled only through
-- GameEvent)
data EventName = Death | Departure | Destruction | DamageTaken
               | CardDrawn | Entry | AttackDeclaration | BlockDeclaration
               | CombatDamage | PartBeginning | SpellCast | StatusChange
               | TurnedFaceUp | PhasingChange | BlockedDeclaration
               | LastCounterRemoval | LifeGain | LifeLoss | TimeShift
               -- a player LOSING the game, named apart from three
               -- neighbours it is not: `OutcomeVerb`'s `LoseGame`, which
               -- is the effect that states it; `LifeLoss`, which is life
               -- and not the game; and `Kind`'s `Outcome`. The word
               -- "loss" is the event's, and the game is what is lost.
               | GameLoss

||| Event-name equality, the closed-vocabulary comparison every other
||| closed word in this file already carries (`sameCounter`,
||| `sameStatusVal`, `sameKeyword`). Minted when the history read's
||| predicate surface needed to compare two phrases wholly: both of that
||| row's arguments are closed words, so `predEq` can answer honestly
||| there instead of conservatively.
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
sameEventName LifeGain LifeGain = True
sameEventName LifeGain _ = False
sameEventName LifeLoss LifeLoss = True
sameEventName LifeLoss _ = False
sameEventName TimeShift TimeShift = True
sameEventName TimeShift _ = False

||| The same for the history window.
public export
sameLookback : Lookback -> Lookback -> Bool
sameLookback ThisTurn ThisTurn = True
sameLookback ThisTurn _ = False
sameLookback ThisCombat ThisCombat = True
sameLookback ThisCombat _ = False
sameLookback LastTurn LastTurn = True
sameLookback LastTurn _ = False

||| WHICH constructions write a clause over a given event — `SpanUse`'s
||| shape asked of the event axis, and it tells the same two silences
||| apart. `EventUnattested` means no corpus line writes any clause over
||| the event in any mood; `EventUnclaimed` means the clause is real
||| oracle English and no construction HERE writes it.
|||
||| Four readers now, and the class names spell the SET each event is
||| written by. `Death` is the widest: the interception, nine hundred and
||| thirty trigger headers, and Graceful Reprieve's delayed clause, while
||| nothing anywhere ends a duration at a death. `Departure` never takes
||| the interception outside a granted quoted ability ("It gains 'If this
||| creature would leave the battlefield, exile it instead'"), a container
||| this grammar has no word for, but it takes the other three: [CR#610.3]
||| riders, trigger headers, and the delayed clause Portcullis and Stangg
||| write. `Destruction` is the one row NO construction claims: its
||| replacement is regeneration's four-part instruction ([CR#614.8] — tap,
||| remove from combat, heal), none of which this vocabulary writes, and
||| its trigger header is written zero times in the modern templating.
||| `DamageTaken` loses the interception for the redirection family's
||| reason ([CR#614.9] — "that damage is dealt to [other] instead", a
||| damage clause whose amount is the intercepted event's) and keeps the
||| trigger. The five new rows are triggers and nothing else, except the
||| turn-part beginning, which is also the delayed clause's ordinary word
||| — though [CR#603.7] says that word "won't usually begin" a delayed
||| trigger, and the corpus agrees at 412 to 1.
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
-- The GAME LOSS opens exactly two readers and the corpus is why. EIGHT
-- lines watch it as a trigger ("Whenever a player loses the game, put
-- five +1/+1 counters on this creature") and SEVEN replace it ("If you
-- would lose the game, instead …"), so trigger and intercept are both
-- attested and both land here. The other two are silences: no duration
-- ends at a loss, and no line writes the DELAYED form ("when a player
-- next loses the game this turn") -- which is `badDelayedGameLoss`, and
-- is not the same silence as the replacement's "next time", that being
-- the multiplicity of a replacement and not a delayed trigger.
eventUse GameLoss = InterceptedAndTriggered
eventUse Entry = TriggeredOnly
eventUse AttackDeclaration = TriggeredOnly
eventUse BlockDeclaration = TriggeredOnly
eventUse CombatDamage = TriggeredOnly
eventUse PartBeginning = TriggeredAndDelayed
-- The CAST event, chapter twenty-eight's biggest unminted family and the
-- first row that needed a whole zone under it. It is trigger-only, and
-- each of the other three readers is a measured silence rather than an
-- oversight: nothing INTERCEPTS a cast ([CR#614.1]'s replacements watch
-- events, and the cast is a player ACTION taken with priority -- the
-- nearest real family, "you may cast … without paying its mana cost", is
-- [CR#118.9]'s alternative cost and not a replacement of the casting);
-- no line ends a duration at one; and the DELAYED form is real ("When
-- you next cast a creature spell this turn, …") but not one line of it
-- is writable -- every body either grants an ability to an object on the
-- STACK, which `Gains` refuses by zone, or copies the spell (ledger).
eventUse SpellCast = TriggeredOnly
-- The STATUS transition, trigger-only like the object events beside
-- it: nothing intercepts a becomes-tapped, nothing holds a zone change
-- on one, and nothing delays on one — each a measured silence — while
-- the trigger headers are the family's whole corpus ([CR#603.2e]).
eventUse StatusChange = TriggeredOnly
-- The face-up TURNING, [CR#708.7]'s permission read as an event: the
-- ability that allowed the permanent to be face down "may also allow
-- the permanent's controller to turn it face up", and the headers watch
-- that turning happen. Trigger-only, and the other three readers are
-- measured silences: nothing replaces a turning, no [CR#610.3] rider
-- waits for one, and no delayed clause names one.
eventUse TurnedFaceUp = TriggeredOnly
-- Phasing, the same answer against the same three silences
-- ([CR#702.26b,702.26c] — the status changes, and the trigger headers
-- are the whole corpus of clauses written over it).
eventUse PhasingChange = TriggeredOnly
-- The BLOCKED declaration, [CR#509.1h]'s half of the turn-based action
-- read on the attacker where `BlockDeclaration` reads it on the blocker.
-- Trigger-only, and each other reader is a measured zero: nothing writes
-- "would become blocked" (the evasion abilities that keep a block from
-- happening are restrictions on the declaration, [CR#509.1b], not
-- replacements of an event), no duration ends at one, and no delayed
-- clause names one.
eventUse BlockedDeclaration = TriggeredOnly
-- The LAST counter's removal, [CR#702.62a] and [CR#702.63a] writing the
-- canonical forms into two keywords' expansions. Trigger-only, and the
-- other three readers are measured zeroes: nothing replaces the removal
-- of a last counter, no rider waits for one, no delayed clause names one.
eventUse LastCounterRemoval = TriggeredOnly
-- The two LIFE-CHANGE events, named apart from the OUTCOME words
-- `LifeGained`/`LifeLost` that the deed telescope already owns (the
-- nominalisation is Death's and Departure's). They are the first names
-- in this table
-- with NO PRODUCER: no `GameEvent` row makes one, the gains-life trigger
-- family being unbuilt. `EventUnclaimed` is exactly the cell for that and
-- it needed no invention -- the class means "real oracle English and no
-- construction HERE writes it", which is precisely the situation: the
-- headers are written in quantity and this vocabulary has no row for them
-- yet. The names exist because the LOOKBACK reads `EventName` directly
-- and does not go through `GameEvent` at all, so "if you gained life this
-- turn" is writable while "Whenever you gain life" is not.
eventUse LifeGain = EventUnclaimed
eventUse LifeLoss = EventUnclaimed
-- The day/night SHIFT, the game scope's own event ([CR#731.1a] naming the
-- two phrases). Trigger-only: nothing replaces it, no duration ends at it,
-- no delayed clause names it.
eventUse TimeShift = TriggeredOnly

||| May the would/instead clause intercept an event of this class? Full
||| rows in the answer axis, so a new `EventUse` declares every reader.
public export
admitsIntercept : EventUse -> Bool
admitsIntercept EventUnattested = False
admitsIntercept EventUnclaimed = False
admitsIntercept TriggeredOnly = False
admitsIntercept InterceptedAndTriggered = True
admitsIntercept InterceptedTriggeredAndDelayed = True
admitsIntercept HeldTriggeredAndDelayed = False
admitsIntercept TriggeredAndDelayed = False

||| May a [CR#610.3] zone-change rider wait for an event of this class?
public export
admitsHold : EventUse -> Bool
admitsHold EventUnattested = False
admitsHold EventUnclaimed = False
admitsHold TriggeredOnly = False
admitsHold InterceptedAndTriggered = False
admitsHold InterceptedTriggeredAndDelayed = False
admitsHold HeldTriggeredAndDelayed = True
admitsHold TriggeredAndDelayed = False

||| May a TRIGGERED ability's header name an event of this class
||| ([CR#603.1] — "[When/Whenever/At] [trigger condition or event],
||| [effect]")? The reader chapter twenty-eight added, and the widest of
||| the four: every event this vocabulary spells is written as a trigger
||| header except the destruction, whose modern templating is "dies".
public export
admitsTrigger : EventUse -> Bool
admitsTrigger EventUnattested = False
admitsTrigger EventUnclaimed = False
admitsTrigger TriggeredOnly = True
admitsTrigger InterceptedAndTriggered = True
admitsTrigger InterceptedTriggeredAndDelayed = True
admitsTrigger HeldTriggeredAndDelayed = True
admitsTrigger TriggeredAndDelayed = True

||| May a DELAYED clause wait for an event of this class ([CR#603.7])?
||| The narrowest reader: three events carry the whole family — the
||| turn-part beginning ("at the beginning of the next end step"), the
||| departure (Portcullis, Stangg, Mysterio), and the death (Graceful
||| Reprieve's "when target creature dies this turn").
public export
admitsDelay : EventUse -> Bool
admitsDelay EventUnattested = False
admitsDelay EventUnclaimed = False
admitsDelay TriggeredOnly = False
admitsDelay InterceptedAndTriggered = False
admitsDelay InterceptedTriggeredAndDelayed = True
admitsDelay HeldTriggeredAndDelayed = True
admitsDelay TriggeredAndDelayed = True

||| Which duration-adverbial class an event-ended "until" phrase falls in
||| — the event axis's own row of chapter seventeen's attestation table,
||| and the finding is that every row is a silence.
|||
||| `Departure` is `Unclaimed` and the composition is why: the lines that
||| write "until [object] leaves the battlefield" are overwhelmingly zone
||| changes ([CR#610.3] — a one-shot exile that schedules its own undo,
||| NOT a continuous effect) and phasings ([CR#610.4], the same shape),
||| leaving three genuine [CR#611.2a] continuous durations: two base-TYPE
||| settings and one becomes-a-copy, all constructions this grammar lacks,
||| so the cell names them and claims nothing. Every other event is
||| `Unattested`: no corpus line ends a duration at a death, a
||| destruction, a damage event, or a draw — the lines that appear to
||| write "until … dies" all cross a clause boundary ("until end of turn,
||| whenever another creature dies").
public export
eventSpan : EventName -> SpanUse
eventSpan Death = Unattested
eventSpan Departure = Unclaimed
eventSpan Destruction = Unattested
eventSpan DamageTaken = Unattested
eventSpan CardDrawn = Unattested
eventSpan GameLoss = Unattested
-- The four new OBJECT events are `Unattested` for the same reason the
-- older four are: no corpus line ends a duration at an entry, an attack,
-- a block, or a combat-damage event. The turn-part beginning is the one
-- row that is neither, and its silence is a DOUBLE-SPELLING refusal
-- rather than an absence: "until the beginning of your next upkeep" is
-- real English, and the adverbial that writes it is `DurationEnd`'s own
-- `StartOf` row. One phrase, one slot (`badUntilBeginningOfUpkeep`).
eventSpan Entry = Unattested
eventSpan AttackDeclaration = Unattested
eventSpan BlockDeclaration = Unattested
eventSpan CombatDamage = Unattested
-- Zero lines end a duration at a cast: the eleven "until … cast"
-- matches are all iterated-reveal repetitions ("until they cast a
-- spell" is written none), not adverbials.
eventSpan SpellCast = Unattested
eventSpan PartBeginning = Unclaimed
-- Measured: no corpus line ends a duration at a status transition. The
-- tapped-STATE family is [CR#611.2b]'s "for as long as … remains
-- tapped" condition, another axis entirely; the two "until …" lines
-- that mention the transition end at a TURN boundary and merely
-- contain a becomes-tapped trigger (chapter thirty-five).
eventSpan StatusChange = Unattested
-- Measured, and the face pair's two directions answer differently for
-- once. No line ends a duration at a turning face UP. Exactly one ends
-- at a turning face DOWN — Vesuvan Shapeshifter's "until this creature
-- is turned face down, it becomes a copy of that creature" — which is
-- the only clause of any kind written over the face-down direction and
-- is not this row's: the direction has no event constructor at all, its
-- trigger headers being zero.
eventSpan TurnedFaceUp = Unattested
-- Measured: no line ends a duration at a phasing. The nearest lines end
-- at a DEPARTURE while their effect is a phase-out ("target creature
-- phases out until this enchantment leaves the battlefield"), which is
-- `Departure`'s cell and not this one.
eventSpan PhasingChange = Unattested
-- Measured: no line ends a duration at a block declaration on either
-- side. The two "until … blocks" matches both cross a clause boundary
-- the way the death ones do ("Until end of turn, whenever a creature an
-- opponent controls blocks, draw a card" is a nested TRIGGER inside a
-- turn-bounded span), which is `Death`'s own reading.
eventSpan BlockedDeclaration = Unattested
-- Measured: no duration ends at a last counter's removal. The family's
-- whole corpus is trigger headers.
eventSpan LastCounterRemoval = Unattested
-- No duration ends at a life change; the "until you gain life" spelling is
-- written zero times.
eventSpan LifeGain = Unattested
eventSpan LifeLoss = Unattested
eventSpan TimeShift = Unattested

||| How many times an interception fires — [CR#614.3]'s two ways for a
||| replacement effect to end, "used up" against "duration expired", and
||| English marks the difference with the clause's own opening word.
|||
||| The word tracks the CARRIER, but the table is indexed by the EVENT,
||| which is the one axis it has. Measured exhaustively: "the next time
||| [someone] would [event] this turn" never occurs as a standing ability
||| in the supported corpus — those lines are one-shots spun up by an
||| activation cost or a trigger (the destruction ones being regeneration
||| and nothing else, [CR#614.8]) — while the conditional is what a
||| standing line writes, twenty-one cards printing "If you would draw a
||| card, … instead" with no duration word at all. So a two-dimensional
||| table cannot say what the corpus says: the carrier dimension is
||| LEDGERED, and what stays here is the event dimension with the standing
||| form open wherever a standing line writes one. The cell that reached
||| past its own measurement was the draw's `Repeatedly`, justified by
||| counting the duration-bounded "if you would draw a card THIS TURN"
||| (zero, correctly), which says nothing about the durationless line
||| (`thoughtReflection`). What the table still refuses is the one-shot
||| word where no carrier writes it (`badNextTimeWouldDie`).
public export
-- spelling: (construction-owned -- the clause's opening word: Repeatedly
-- writes "if <event>", NextTimeOnly writes "the next time <event>".
-- Consumed by StaticEffect.Intercepts, never spelled alone)
data ReplUse = Repeatedly | NextTimeOnly

public export
replUseOk : EventName -> ReplUse -> Bool
replUseOk Death Repeatedly = True
replUseOk Death NextTimeOnly = False
replUseOk Departure Repeatedly = True
replUseOk Departure NextTimeOnly = False
-- Both words, and the carrier is what picks between them: regeneration
-- writes the one-shot ([CR#614.8], twenty-five reminder lines) from a
-- cost prefix, and four supported cards print the standing "would be
-- destroyed, … instead" as a line. Neither cell is reachable through
-- this vocabulary — `Interceptable` shuts the event at `Unclaimed`,
-- [CR#614.8]'s four-part replacement being unwritable here — so the row
-- states what is measured rather than what one carrier happens to use.
replUseOk Destruction Repeatedly = True
replUseOk Destruction NextTimeOnly = True
replUseOk DamageTaken Repeatedly = True
replUseOk DamageTaken NextTimeOnly = True
-- The standing draw replacement is twenty-one printed cards — "If you
-- would draw a card, draw two cards instead" (Thought Reflection) and
-- its family — and not one of them writes a duration. The nine "the
-- next time you would draw a card this turn" lines are all one-shots.
replUseOk CardDrawn Repeatedly = True
replUseOk CardDrawn NextTimeOnly = True
-- Both words, both written, and the split is one card apart: five lines
-- write the standing "If you would lose the game, instead …" (Exquisite
-- Archangel, Lich's Mirror, The Golden Throne and two unsupported) and
-- two write the bounded "The next time you would lose the game this
-- turn, instead …" (Stunning Reversal, Nira). So neither cell is a
-- convenience and neither is pinnable.
replUseOk GameLoss Repeatedly = True
replUseOk GameLoss NextTimeOnly = True
-- The five trigger-only events answer NEITHER, and the table says so
-- rather than the interception's own gate saying it twice: an event no
-- would/instead clause writes has no multiplicity word to choose, so
-- both cells are `False` and the refusal a writer meets is
-- `Interceptable`'s.
replUseOk SpellCast Repeatedly = False
replUseOk SpellCast NextTimeOnly = False
replUseOk Entry Repeatedly = False
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
-- The life-change replacement IS real ("If you would gain life, …
-- instead") and is `EventUnclaimed`'s business rather than these cells':
-- with no producer there is no event term for a would/instead clause to
-- take, so the multiplicity word has nothing to choose and both cells
-- answer False. When the producer lands, these are the cells to
-- re-measure.
replUseOk LifeGain Repeatedly = False
replUseOk LifeGain NextTimeOnly = False
replUseOk LifeLoss Repeatedly = False
replUseOk LifeLoss NextTimeOnly = False
replUseOk TimeShift Repeatedly = False
replUseOk TimeShift NextTimeOnly = False

-- ===== The trigger header's opening word =====

||| The word a triggered ability opens with ([CR#603.1] — "[When/
||| Whenever/At] [trigger condition or event], [effect]"). No rule
||| distinguishes the three: [CR#113.3c] lists them together as words a
||| triggered ability "include(s) (and usually begin(s) with)", and
||| nothing assigns one to an event. So the word is a SLOT and the corpus
||| is the only evidence about which slot fillings are real.
|||
||| What the corpus assigns absolutely is `At`, and [CR#603.2b] is why:
||| it gives "at the beginning of" a phase or step its own clause, and
||| the corpus honors it without exception — every "At the beginning of"
||| line is a turn-part beginning and no object event takes the word.
|||
||| What the corpus does NOT assign is the When/Whenever split. The word
||| tracks REPEATABILITY: a once-per-object event with a fixed subject
||| takes "When", the same event over a DESCRIPTION takes "Whenever"
||| because the description ranges over many objects, and an event that
||| repeats for one object takes "Whenever" even with the fixed subject.
||| The residue is what keeps it out of the type: every counterexample is
||| an ability that destroys its own source, so the word is answering a
||| question about the EFFECT. A gate keyed on the event alone would have
||| to refuse real oracle either way, so it refuses only what [CR#603.2b]
||| settles.
public export
-- spelling: ["When", "Whenever", "At"] (the header's first word, followed
-- by the event clause, a comma, and the effect; the turn-part event
-- supplies "the beginning of" itself -- see GameEvent.BeginningOf),
-- kind: TODO(reason: header fragment -- not one of Nominal/Sentence/
-- Cost/KeywordLine/Ability)
data TriggerWord = When | Whenever | At

||| Which opening word an event's header may take. Full rows over
||| (event x word), so a new event declares its word before it can be
||| triggered on. Every object event answers `True` to both English
||| words and `False` to `At`; the turn-part beginning inverts it exactly
||| ([CR#603.2b]). The two events no trigger writes at all answer `False`
||| throughout, and the refusal a writer meets there is `Triggerable`'s.
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
-- Both words again, and the eight watching lines divide them the way
-- finding 247 said they divide everywhere: `Whenever` for the repeatable
-- class (five lines, "whenever a player loses the game") and `When` for
-- the singular occasion (two, both naming ONE player the card has
-- already fixed -- the enchanted player, the chosen player). `At` is
-- [CR#603.2b]'s and stays there (`badGameLossAtTrigger`).
triggerWordOk GameLoss When = True
triggerWordOk GameLoss Whenever = True
triggerWordOk GameLoss At = False
triggerWordOk Entry When = True
triggerWordOk Entry Whenever = True
triggerWordOk Entry At = False
-- Both header words, and the corpus divides them the way finding 172
-- said it would: "Whenever [someone] casts" at nine hundred forty-eight
-- against a hundred twenty-one "When", the casting being a repeatable
-- event whichever subject it takes. `At` is refused with every other
-- object event ([CR#603.2b] reserves it for phases and steps).
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
-- Both English words: the direct "Whenever … becomes tapped" mass
-- against the Aura family's "When enchanted … becomes tapped"
-- ([CR#603.1] assigns neither word; the guide's split tracks whether
-- the event is naturally singular in context, which the EFFECT decides
-- — an Aura's destroy ends the relationship — so the grammar declines
-- to encode it, tolerated over-generation). `At` is refused with every
-- other object event ([CR#603.2b]).
triggerWordOk StatusChange When = True
triggerWordOk StatusChange Whenever = True
triggerWordOk StatusChange At = False
-- Both English words, and this family writes the split the other way
-- round from the mass: the morph line "When this creature is turned
-- face up, …" is the largest cell by far, "Whenever a permanent you
-- control is turned face up, …" the description-subject one. The
-- grammar declines to encode the split here as everywhere ([CR#603.1]
-- assigns no word), and refuses `At` with every other object event
-- ([CR#603.2b]).
triggerWordOk TurnedFaceUp When = True
triggerWordOk TurnedFaceUp Whenever = True
triggerWordOk TurnedFaceUp At = False
-- Both again, though the corpus is nearly all `Whenever`: the one
-- `When` line ("When this creature phases out or leaves the
-- battlefield, …") writes a COORDINATED event this vocabulary has no
-- word for, so the cell is opened on the standing slot policy rather
-- than on a writable line.
triggerWordOk PhasingChange When = True
triggerWordOk PhasingChange Whenever = True
triggerWordOk PhasingChange At = False
-- Both words across the whole block family, 250 `Whenever` to 20 `When`,
-- and the split is the standing tolerated one: [CR#603.1] assigns no
-- word and the guide's repeatability test keys on the effect. `At` stays
-- [CR#603.2b]'s.
triggerWordOk BlockedDeclaration When = True
triggerWordOk BlockedDeclaration Whenever = True
triggerWordOk BlockedDeclaration At = False
-- Every explicit line writes `When` and none writes `Whenever`, which is
-- what a once-per-object event should look like -- but the grammar
-- declines to encode the split here as everywhere ([CR#603.1] assigns no
-- word), so the cell stays open as tolerated over-generation. `At` is
-- [CR#603.2b]'s.
triggerWordOk LastCounterRemoval When = True
triggerWordOk LastCounterRemoval Whenever = True
triggerWordOk LastCounterRemoval At = False
-- Answered from the English the corpus writes ("Whenever you gain life,
-- …", "When you gain life, …") rather than left blank, though no producer
-- can reach these cells yet: a table with full rows has to say something,
-- and saying what the corpus says is the answer that stays true when the
-- producer lands. `At` is [CR#603.2b]'s as always.
triggerWordOk LifeGain When = True
triggerWordOk LifeGain Whenever = True
triggerWordOk LifeGain At = False
triggerWordOk LifeLoss When = True
triggerWordOk LifeLoss Whenever = True
triggerWordOk LifeLoss At = False
-- Every one of the ten lines writes `Whenever`; the cell for `When`
-- stays open on the standing slot policy ([CR#603.1] assigns no word).
triggerWordOk TimeShift When = True
triggerWordOk TimeShift Whenever = True
triggerWordOk TimeShift At = False

||| WHICH events a history query names, and with a subject of WHICH
||| SORT — the lookback's attestation table, two-axis because one axis
||| could not say what the corpus says. `replUseOk`'s and `triggerWordOk`'s
||| shape at a third site.
|||
||| The second axis is forced by one family. Every other event name takes
||| a subject of a single sort in this position, but the ATTACK
||| declaration takes both: "if you attacked this turn" is the raid mass,
||| thirty-eight lines with a PLAYER subject, and "if this creature
||| attacked or blocked this turn" is six with an OBJECT one. A
||| single-valued `EventName -> Kind` table would have had to refuse one of
||| them, so the table is over the pair and `Happened` is indexed at the
||| kind it answers True for.
|||
||| Full rows over (name x the two phrasal kinds), each cell measured in
||| the CONDITION position and nowhere else. The Trues: death (the morbid
||| mass), departure (12), entry, damage taken (6), block declaration (4)
||| and attack declaration (6) on the object side; spell cast (21), card
||| drawn, life gained (37), life lost (18) and attack declaration (38) on
||| the player side. The zeroes are all queried and all real — no line
||| writes a destruction, a status change, a face turning, a phasing, a
||| becomes-blocked, a last-counter removal or a turn-part beginning as a
||| history read, and "if an upkeep began this turn" is not English.
|||
||| Two cells are refused for a reason SHARPER than a zero and the
||| distinction is worth keeping: combat damage is written twice in this
||| position and both lines carry a RECIPIENT complement ("if this
||| creature dealt combat damage to an opponent this turn") that a
||| one-noun query cannot spell, and the player side of the attack
||| declaration writes a complement too on the lines that are not bare
||| ("if you attacked with a Hero this turn"). The complement is a real
||| gap, ledgered, and admitting the cells on lines the row cannot finish
||| would have hidden it.
public export
lookbackSubjectOk : EventName -> Kind -> Bool
lookbackSubjectOk Death Object = True
lookbackSubjectOk Death Player = False
lookbackSubjectOk Departure Object = True
lookbackSubjectOk Departure Player = False
lookbackSubjectOk Destruction Object = False
lookbackSubjectOk Destruction Player = False
lookbackSubjectOk DamageTaken Object = True
-- RE-MEASURED when the second reader landed, and the only cell that
-- moved. Chapter forty-two measured every cell in the condition position
-- because that was the only reader there was, and found one line ("if you
-- haven't been dealt combat damage since your last turn") in a window this
-- vocabulary does not carry. The PREDICATE position writes the same event
-- over a player eight times -- "for each opponent who was dealt damage
-- this turn", "each player who was dealt combat damage this turn" -- so
-- the cell is True. A shared table's cells are a property of the QUERY
-- and not of a reader, which is why a new reader can only ever add Trues:
-- the union is what the table always meant, and the round that adds a
-- reader owes the re-measurement.
lookbackSubjectOk DamageTaken Player = True
lookbackSubjectOk CardDrawn Object = False
lookbackSubjectOk CardDrawn Player = True
-- The loss event's history cells are CLOSED, and this round measured
-- them rather than inheriting the queue's guess that a history read
-- might exist. Two corpus lines look backward at losses and neither is
-- this query: Hot Pursuit's "if two or more players have lost the game"
-- and Rampant Frogantua's "for each player who has lost the game" both
-- COUNT PLAYERS in a standing state, and neither names a window --
-- there is no "lost the game this turn" anywhere. This row asks whether
-- an event happened inside a `Lookback`, so a windowless count over
-- players is a different reader's business (the player-set count, which
-- this grammar has no term for). Recorded, closed, and pinned at the
-- player sort (`badGameLossLookback`).
lookbackSubjectOk GameLoss Object = False
lookbackSubjectOk GameLoss Player = False
lookbackSubjectOk Entry Object = True
lookbackSubjectOk Entry Player = False
lookbackSubjectOk AttackDeclaration Object = True
lookbackSubjectOk AttackDeclaration Player = True
lookbackSubjectOk BlockDeclaration Object = True
lookbackSubjectOk BlockDeclaration Player = False
lookbackSubjectOk CombatDamage Object = False
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
lookbackSubjectOk BlockedDeclaration Object = False
lookbackSubjectOk BlockedDeclaration Player = False
lookbackSubjectOk LastCounterRemoval Object = False
lookbackSubjectOk LastCounterRemoval Player = False
lookbackSubjectOk LifeGain Object = False
lookbackSubjectOk LifeGain Player = True
lookbackSubjectOk LifeLoss Object = False
lookbackSubjectOk LifeLoss Player = True
-- the shift has no SUBJECT of either sort -- the game is not a phrasal
-- kind -- so no history read can be written over it in either position.
lookbackSubjectOk TimeShift Object = False
lookbackSubjectOk TimeShift Player = False
-- the two non-phrasal kinds carry no subject at all: a quality names no
-- participant and an outcome is a deed's own mention.
lookbackSubjectOk _ (Quality _) = False
lookbackSubjectOk _ Outcome = False
lookbackSubjectOk _ Gap = False
lookbackSubjectOk _ (Letter _) = False

public export
data LookbackSubject : EventName -> Kind -> Type where
  MkLookbackSubject : {auto 0 ok : lookbackSubjectOk ev k = True} ->
                      LookbackSubject ev k

-- ===== Which turn-part beginnings a header names =====

||| The two silences again, asked of the (part x possession) grid the
||| turn-part trigger header writes. `PartUnattested` is a beginning no
||| corpus line names; `PartUnclaimed` is one it names with a possessor
||| this vocabulary has no word for.
public export
data PartUse = PartUnattested | PartUnclaimed | PartTriggered

||| WHICH turn-part beginnings a header names, over the SHARED endpoint
||| vocabulary chapter seventeen minted for durations and said out loud
||| was the trigger's too. Full rows over five parts and three
||| possessions — fifteen cells — so a new `TurnPart` or a new `Whose` is
||| a totality error here as well as on `spanUse`.
|||
||| `Upkeep` is the family's centre, possessed. `EndStep` writes both the
||| possessed form and the UNPOSSESSED one ("At the beginning of the end
||| step, destroy all Goblins"), the one cell where the bare part word is
||| the whole phrase. `Combat` writes the possessed form and extraposes
||| the possessive onto the turn ("At the beginning of combat on your
||| turn"), which is Brazen Cannonade's move in the duration table
||| appearing here as an ordinary spelling. `Turn` and `UntapStep` are
||| written ZERO times in every possession: the turn's own beginning is
||| not a trigger event English names — the upkeep is what it names
||| instead — and no line triggers at an untap step's beginning.
|||
||| The `PartUnclaimed` cells are the possessor gap chapter twenty-seven
||| measured from the other side: "at the beginning of each upkeep",
||| "each opponent's upkeep", "the upkeep of enchanted creature's
||| controller" and "each end step" are real headers whose possessor is a
||| QUANTIFIER or a nominal, where `Whose` is a two-word pronominal
||| vocabulary by construction. `ThatPlayers` stays a silence here too:
||| one line writes "at the beginning of that player's upkeep" and none
||| writes any other part.
public export
partUse : TurnPart -> Maybe Owner -> PartUse
partUse Turn Nothing = PartUnattested
partUse Turn (Just Yours) = PartUnattested
partUse Turn (Just ThatPlayers) = PartUnattested
partUse Turn (Just EachPlayers) = PartUnattested
partUse Turn (Just EachOpponents) = PartUnattested
partUse Turn (Just EachYours) = PartUnattested
partUse Turn (Just AnOpponents) = PartUnattested
-- RECLASSIFIED. This cell was `PartUnclaimed` -- "names it with a
-- possessor this vocabulary has no word for" -- and the words now exist,
-- so the honest answer is that NOTHING writes a possessor-less upkeep
-- header at all: the 36 "each upkeep" lines are `EachPlayers`' second
-- spelling, not this cell's (`badTriggerAtEachUpkeep` still refuses, the
-- pin unaffected by which silence it names).
partUse Upkeep Nothing = PartUnattested
partUse Upkeep (Just Yours) = PartTriggered
partUse Upkeep (Just ThatPlayers) = PartUnclaimed
partUse Upkeep (Just EachPlayers) = PartTriggered
partUse Upkeep (Just EachOpponents) = PartTriggered
partUse Upkeep (Just EachYours) = PartUnattested
partUse Upkeep (Just AnOpponents) = PartUnattested
-- The possessor-less end step is the DELAYED clause's and not the
-- header's: "at the beginning of the next end step" writes no possessor
-- because it names one occurrence rather than a class, and the header's
-- bare "each end step" is `EachPlayers`' second spelling. One cell, one
-- construction, and the round that gave the header its quantifiers is
-- what made the difference sayable.
partUse EndStep Nothing = PartTriggered
partUse EndStep (Just Yours) = PartTriggered
partUse EndStep (Just ThatPlayers) = PartUnattested
partUse EndStep (Just EachPlayers) = PartTriggered
partUse EndStep (Just EachOpponents) = PartTriggered
partUse EndStep (Just EachYours) = PartUnattested
partUse EndStep (Just AnOpponents) = PartUnattested
partUse Combat Nothing = PartUnattested
partUse Combat (Just Yours) = PartTriggered
partUse Combat (Just ThatPlayers) = PartUnattested
partUse Combat (Just EachPlayers) = PartTriggered
partUse Combat (Just EachOpponents) = PartUnattested
partUse Combat (Just EachYours) = PartUnattested
partUse Combat (Just AnOpponents) = PartUnattested
partUse UntapStep Nothing = PartUnattested
partUse UntapStep (Just Yours) = PartUnattested
partUse UntapStep (Just ThatPlayers) = PartUnattested
partUse UntapStep (Just EachPlayers) = PartUnattested
partUse UntapStep (Just EachOpponents) = PartUnattested
partUse UntapStep (Just EachYours) = PartUnattested
partUse UntapStep (Just AnOpponents) = PartUnattested
partUse EndOfCombat Nothing = PartTriggered
partUse EndOfCombat (Just Yours) = PartUnattested
partUse EndOfCombat (Just ThatPlayers) = PartUnattested
partUse EndOfCombat (Just EachPlayers) = PartUnattested
partUse EndOfCombat (Just EachOpponents) = PartUnattested
partUse EndOfCombat (Just EachYours) = PartUnattested
partUse EndOfCombat (Just AnOpponents) = PartUnattested
partUse FirstMain Nothing = PartUnattested
partUse FirstMain (Just Yours) = PartTriggered
partUse FirstMain (Just ThatPlayers) = PartUnattested
partUse FirstMain (Just EachPlayers) = PartTriggered
partUse FirstMain (Just EachOpponents) = PartUnattested
partUse FirstMain (Just EachYours) = PartUnattested
partUse FirstMain (Just AnOpponents) = PartUnattested
partUse PostcombatMain Nothing = PartUnattested
-- The SPELLING SPLIT, and at this site it is perfect: the possessor
-- `Yours` writes "your second main phase" (5 headers) and `EachYours`
-- writes "each of your postcombat main phases" (7), with both cross
-- cells at zero -- no header writes "your postcombat main phase" or
-- "each of your second main phases". [CR#505.1] names the two words for
-- one phase ("the first main phase (also known as the precombat main
-- phase)"), so this is finding 275's agreement a fourth time: one part
-- row, two spellings, chosen by the possessor. The scope matters and is
-- stated -- "your postcombat main phase" occurs 8 times ELSEWHERE, so
-- the perfection is the header reader's and not the language's.
partUse PostcombatMain (Just Yours) = PartTriggered
partUse PostcombatMain (Just ThatPlayers) = PartUnattested
partUse PostcombatMain (Just EachPlayers) = PartUnattested
partUse PostcombatMain (Just EachOpponents) = PartUnattested
partUse PostcombatMain (Just EachYours) = PartTriggered
partUse PostcombatMain (Just AnOpponents) = PartUnattested
partUse DrawStep Nothing = PartUnattested
partUse DrawStep (Just Yours) = PartTriggered
partUse DrawStep (Just ThatPlayers) = PartUnattested
partUse DrawStep (Just EachPlayers) = PartTriggered
partUse DrawStep (Just EachOpponents) = PartUnattested
partUse DrawStep (Just EachYours) = PartUnattested
partUse DrawStep (Just AnOpponents) = PartUnattested

||| The answer axis, full rows, so a new `PartUse` declares its reader.
public export
admitsPartTrigger : PartUse -> Bool
admitsPartTrigger PartUnattested = False
admitsPartTrigger PartUnclaimed = False
admitsPartTrigger PartTriggered = True

-- ===== Prevention shields (the damage class and the shield's size) =====

||| WHICH damage a prevention shield stops. [CR#615.1] makes the shield
||| watch "a damage event that would happen", and the only qualifier
||| oracle puts on the noun is the combat/noncombat split ([CR#510.2] —
||| combat damage is what the combat damage step deals, which is the whole
||| of what the adjective names). Closed at three because no fourth
||| qualifier is written: no line prevents "all trample damage" or "all
||| excess damage".
public export
-- spelling: ["damage", "combat damage", "noncombat damage"] (the bare noun
-- and its two adjectives; spelled only through StaticEffect.Prevents)
data DamageKind = AnyDamage | CombatOnly | NoncombatOnly

-- ===== Deontic restrictions (the one-shot "can't" vocabulary) =====

||| The deed a restriction denies — the verb alone. Core and the
||| predecessor grammar both mark WHICH PART the subject plays by which
||| SLOT carries the reference — `DeonticAction::Block { by, on }`
||| (`deckmaste_core/src/deontic.rs`), `Enact Block <agent> <patient>`
||| (`Semantics.idr`) — but a clause with ONE subject has no second slot
||| to put it in, so English marks it in the VOICE: "can't block" against
||| "can't be blocked". That is an axis of its own (`Role`), not a second
||| deed word, so this enum stops at the verbs. Closed and row-enumerated,
||| and every table over it is written out, so a new deed must declare
||| which types carry its grant, in which voice, before it can be written
||| at all. These lead the one-shot restrictions in the corpus; the rest
||| of the deontic surface is either the parked ability layer or a deed
||| with no clause of its own yet.
public export
-- spelling: ["attack", "block"] (the bare verb; `Role` inflects it into
-- the verb phrase after "can't" -- "block" against "be blocked" -- and the
-- PAIR is what Effect.Cant spells, never this enum alone)
data Deed = Attack | Block

||| Which part the restriction's SUBJECT plays in the deed: core's two
||| slots as one axis, since a one-subject clause has only its voice to
||| say it with. `Agent` is the active reading and core's `by` slot
||| ("can't block"), `Patient` the passive and core's `on` ("can't be
||| blocked"). The words are the predecessor grammar's own
||| (`Enact Block <agent> <patient>`, `Semantics.idr`).
public export
-- spelling: (construction-owned -- the VOICE of Deed's verb: Agent leaves
-- it bare, Patient makes it passive ("be blocked"). Consumed by
-- Effect.Cant, never spelled alone)
data Role = Agent | Patient

||| The three POLARITIES a deed clause can carry — the convergence
||| chapter forty-six named and deferred, executed here on its own stated
||| trigger (finding 331: "a GATE polarity with evidence would justify
||| refactoring both rows onto one compulsion axis").
|||
||| All three prior arts agree on the shape and disagree on the spine.
||| Core has `Deontic{May, Cant, Must, Gate}` over a `DeonticAction` whose
||| variants carry `by`/`on` predicates (`deckmaste_core/src/deontic.rs`);
||| the OLD module has `Constrain : Compulsion -> Deed` over an eight-kind
||| relation spine with a `Priced` sibling for gates (`Semantics.idr`);
||| this vocabulary keeps its own narrow `Deed x Role` grid and puts the
||| polarity on ONE axis over it. What is NOT taken from either is the
||| PERMISSION: core's `May` is the existential floor a granted row
||| widens, and this grammar has no line asking for it — "may attack" is
||| written zero times, attacking being permitted by default ([CR#506.3]).
||| Three rows, measured; a fourth would be symmetry.
|||
||| The GATE carries its cost on the VALUE rather than in a fourth field
||| on the row, which is `BoostCounter`'s arrangement and for its reason:
||| the payload exists exactly where the polarity needs it, so no other
||| polarity has to carry an empty slot and no gate can be written without
||| one.
public export
data CompTag = ForbidT | RequireT | GateT

||| Whether a polarity's cell writes a PATIENT, and the answer is
||| three-valued because the corpus is: the block restriction writes one
||| in fourteen lines and omits it in many more, the block requirement
||| writes one in all thirty-eight, and every other cell writes none.
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

||| WHICH cell writes a patient. Full rows over (polarity x deed x role),
||| every cell measured in its own family and none by symmetry.
public export
deonticPatientOk : CompTag -> Deed -> Role -> PatientNeed
-- the restriction: "can't block creatures with power 3 or greater" is
-- fourteen lines and the bare "can't block" many more, so the block
-- agent's cell is OPTIONAL. No restriction names an attack's defender in
-- this slot -- "can't attack you" writes a DEFENDING PLAYER, which is a
-- different participant and has no noun here (ledger).
deonticPatientOk ForbidT Attack Agent = PatientRefused
deonticPatientOk ForbidT Attack Patient = PatientRefused
deonticPatientOk ForbidT Block Agent = PatientOptional
deonticPatientOk ForbidT Block Patient = PatientRefused
-- the requirement: finding 334's measurement unchanged, required in one
-- cell and refused in three.
deonticPatientOk RequireT Attack Agent = PatientRefused
deonticPatientOk RequireT Attack Patient = PatientRefused
deonticPatientOk RequireT Block Agent = PatientRequired
deonticPatientOk RequireT Block Patient = PatientRefused
-- the gate: Hipparion writes one ("can't block creatures with power 3 or
-- greater unless you pay {1}") and the plural-subject lines write none,
-- so the same cell is optional here too. The attack gates write no
-- patient at all.
deonticPatientOk GateT Attack Agent = PatientRefused
deonticPatientOk GateT Attack Patient = PatientRefused
deonticPatientOk GateT Block Agent = PatientOptional
deonticPatientOk GateT Block Patient = PatientRefused

||| Which card types carry a deed's grant IN A GIVEN VOICE — the stand-in
||| for reading `May(Attack)`/`May(Block)` off the TypeDef declaration
||| (`plugins/builtin/macros/cardtype/Creature.ron`), and a DIFFERENT
||| table from `combatant`, which the fight chapter minted precisely
||| because fight keys on type membership and deals non-combat damage
||| ([CR#701.14b,701.14d]) where these deeds are combat proper.
||| [CR#506.3] — "Only a creature can attack or block" — answers the
||| active rows, and the passive of BLOCK too, because what a blocker
||| blocks is an attacking creature ([CR#509.1a]). The passive of ATTACK
||| is answered by the SECOND sentence of that same rule: only a player, a
||| planeswalker, or a battle can be attacked, and not one of those is a
||| card type this grammar spells. So that whole row is False — and False
||| for a reason worth writing down, because the PHRASE is real oracle
||| ("The Aetherspark can't be attacked" of a planeswalker, "you can't be
||| attacked except by creatures with flying" of a player). The row waits
||| on the planeswalker and battle card types and on the ledgered
||| player-subject restriction, not on a corpus witness.
public export
deedType : Deed -> Role -> CardType -> Bool
deedType Attack Agent Creature = True
deedType Attack Agent Artifact = False
deedType Attack Agent Land = False
deedType Attack Agent Enchantment = False
-- The two SPELL types answer False everywhere for a reason no count is
-- needed for: [CR#506.3] says "only a creature can attack or block",
-- and an instant or sorcery is never even a permanent ([CR#110.4]:
-- "instant and sorcery cards can't enter the battlefield and thus
-- can't be permanents").
deedType Attack Agent Instant = False
deedType Attack Agent Sorcery = False
deedType Attack Patient Creature = False
deedType Attack Patient Artifact = False
deedType Attack Patient Land = False
deedType Attack Patient Enchantment = False
deedType Attack Patient Instant = False
deedType Attack Patient Sorcery = False
deedType Block Agent Creature = True
deedType Block Agent Artifact = False
deedType Block Agent Land = False
deedType Block Agent Enchantment = False
deedType Block Agent Instant = False
deedType Block Agent Sorcery = False
deedType Block Patient Creature = True
deedType Block Patient Artifact = False
deedType Block Patient Land = False
deedType Block Patient Enchantment = False
deedType Block Patient Instant = False
deedType Block Patient Sorcery = False

||| The deed's demand on its subject's projected head, as a witness —
||| `FightParticipant`'s shape for the combat grants, reading the voice
||| along with the verb. There is no `Nothing` row: an UNTYPED head
||| cannot prove participation, so a disjunctive subject, which
||| honestly fixes no type (finding 50), is refused rather than waved
||| through on its silence.
public export
data DeedParticipant : Deed -> Role -> Maybe CardType -> Type where
  Participant : {auto 0 ok : deedType d r t = True} -> DeedParticipant d r (Just t)

||| Which static effect a `Continuously` clause establishes, as the span
||| tables' key — the axis `spanUse` classifies its adverbials against.
||| One row per `StaticEffect` constructor (`staticKind`), so a new static
||| row is a totality error on both tables and must declare which
||| durations it writes before it can be written at all.
|||
||| `Replacement` and `Prevention` carry no subject noun where the other
||| five do, and [CR#611.2c] is why: it cuts the continuous effects in two
||| — those that "modify the characteristics or change the controller of
||| any objects", whose affected set is fixed when the effect begins, and
||| those that do neither and therefore "modify the rules of the game" —
||| and the rule's own worked example is a prevention effect.
|||
||| `Conditional`, `PlayPermission` and `EntryRider` are not clause
||| constructions at all: they answer `False` to every cell of
||| `admitsSpan` and to `absentOk`, so no `Continuously` clause can
||| establish one, and the only construction that writes them is the
||| static ABILITY line (`staticAsAbility`) — a table with two readers
||| rather than one, the same move `eventUse` makes.
|||
||| The three layer-seven rows added with the layer words divide the same
||| way and none of them lands where its neighbour does. `PtDefinition` is
||| the [CR#208.2a] definition, a printed line and never a clause, so it
||| answers `False` everywhere `CostModification` does. `BasePtSet` is
||| both — 41 printed lines and 73 clauses — so it answers `True` to
||| `staticAsAbility` and to `absentOk` and joins two span cells.
||| `PtSwitch` is the mirror image of the definition: 25 lines, every one
||| of them a clause with "until end of turn" written on it, so it is the
||| second row after the control grant to answer `False` to
||| `staticAsAbility`.
public export
data StaticKind = PtDelta | KeywordGrant | DeedRestriction | TypeAddition
                | ControlGrant | Replacement | Prevention
                | Conditional | PlayPermission | EntryRider
                | CostModification
                | PtDefinition | BasePtSet | PtSwitch

||| The MARKING WORD a conditional static writes over its condition. "As
||| long as" and "unless" are not two constructions but one construction
||| with two spellings of the SAME negation: "unless [C]" holds the
||| statement true while C is false, which is "as long as not [C]" with
||| the "not" moved onto the marking word. No rule assigns either word —
||| [CR#611.3a] licenses the wrapper and says nothing about its surface —
||| so this is the corpus's fact, and the corpus is lopsided: the bare "as
||| long as" is the overwhelming majority and rarely negated, while
||| "unless" writes only the two shapes that need it, "can't … unless" and
||| "enters tapped unless".
public export
-- spelling: ["as long as", "unless"] (row order: AsLongAs/Unless; the
-- subordinator introducing the condition clause. Unless writes the
-- NEGATION it carries, so its condition's own words are the positive
-- ones -- "unless you control an artifact", never "unless you control no
-- artifact". Spelled only through StaticEffect.Conditionally)
data CondMarking = AsLongAs | Unless

||| The VERB a play permission writes. [CR#604.6] brackets the pair in
||| its own templates — "You may [cast/play] [this card] …" — so this is
||| a slot the rule itself declares, and chapter twenty-eight's reading of
||| it needed one correction: it recorded the verb as DERIVED from the
||| complement's kind, "cast" belonging to a spell and "play" to a card,
||| and [CR#701.5b] says the opposite in four words — "to cast a card is
||| to cast it as a spell". A card is what both verbs take.
|||
||| What separates them is the LAND, and the glossary derivation is still
||| the right one read the other way: "playing a card" means "playing that
||| card as a land or casting that card as a spell, whichever is
||| appropriate" ([CR#601.1a]), so "play" is the union and "cast" the half
||| of it that excludes lands — [CR#305.9] making that exclusion a rule,
||| "if an object is both a land and another card type, it can be played
||| only as a land. It can't be cast as a spell."
public export
-- spelling: ["play", "cast"] (row order: Play/Cast; the bare verb after
-- the modal, "you may play <what>" / "you may cast <what>". Spelled only
-- through StaticEffect.MayPlay)
data PlayVerb = Play | Cast

||| Which verb may take which complement. `Play` takes anything
||| ([CR#601.1a]'s union), `Cast` refuses a LAND-headed one outright
||| ([CR#305.9]) and passes a silent head for `zoneFits`' reason — a
||| phrase that names no type has not named a land (`badCastALand`).
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

||| The attestation table's other half: which classes of adverbial each
||| construction writes. Full rows in both directions. The shape is the
||| finding — the cross-turn span is the one class both GRANTS and the
||| RESTRICTION admit, and the two current-turn words divide those
||| families with nothing shared, which is why the detain family's
||| cross-turn span ("can't attack or block until your next turn") is the
||| one place a restriction and a grant write the same words. The type
||| addition is out of the class entirely.
|||
||| The `False` cells that rest on a MEASUREMENT rather than a rule. The
||| TYPE ADDITION takes "until end of turn" and never end of combat, and
||| Coward // Killer settles the current-turn split inside one sentence,
||| giving the restriction "this turn" and the addition "until end of
||| turn"; its cross-turn cell is closed because no line writes a NAKED
||| type addition across turns, the three that look like it being
||| otherwise explained (`badTypeAdditionAcrossTurns`). The CONTROL grant
||| is the widest of the five yet writes "this turn" ZERO times, so the
||| restriction's own current-turn word stays the restriction's alone.
||| Prevention writes ONE adverbial, "this turn", and nothing writes an
||| interception or a shield at an upkeep, at a combat endpoint, or under
||| a for-as-long-as condition.
public export
admitsSpan : StaticKind -> SpanUse -> Bool
admitsSpan PtDelta Unattested = False
admitsSpan PtDelta Unclaimed = False
admitsSpan PtDelta GrantsAndControl = True
admitsSpan PtDelta GrantsTypesControlReplacementPermissionSetAndSwitch = True
admitsSpan PtDelta KeywordGrantOnly = False
admitsSpan PtDelta RestrictionsShieldsPermissionsAndDelays = False
admitsSpan PtDelta GrantsRestrictionsReplacementAndBaseSet = True
admitsSpan PtDelta ControlGrantAndPermission = False
admitsSpan PtDelta GrantsRestrictionsControlAndPermission = True
admitsSpan PtDelta PermissionOnly = False
admitsSpan KeywordGrant Unattested = False
admitsSpan KeywordGrant Unclaimed = False
admitsSpan KeywordGrant GrantsAndControl = True
admitsSpan KeywordGrant GrantsTypesControlReplacementPermissionSetAndSwitch = True
admitsSpan KeywordGrant KeywordGrantOnly = True
admitsSpan KeywordGrant RestrictionsShieldsPermissionsAndDelays = False
admitsSpan KeywordGrant GrantsRestrictionsReplacementAndBaseSet = True
admitsSpan KeywordGrant ControlGrantAndPermission = False
admitsSpan KeywordGrant GrantsRestrictionsControlAndPermission = True
admitsSpan KeywordGrant PermissionOnly = False
admitsSpan DeedRestriction Unattested = False
admitsSpan DeedRestriction Unclaimed = False
admitsSpan DeedRestriction GrantsAndControl = False
admitsSpan DeedRestriction GrantsTypesControlReplacementPermissionSetAndSwitch = False
admitsSpan DeedRestriction KeywordGrantOnly = False
admitsSpan DeedRestriction RestrictionsShieldsPermissionsAndDelays = True
admitsSpan DeedRestriction GrantsRestrictionsReplacementAndBaseSet = True
admitsSpan DeedRestriction ControlGrantAndPermission = False
admitsSpan DeedRestriction GrantsRestrictionsControlAndPermission = True
admitsSpan DeedRestriction PermissionOnly = False
admitsSpan TypeAddition Unattested = False
admitsSpan TypeAddition Unclaimed = False
admitsSpan TypeAddition GrantsAndControl = False
admitsSpan TypeAddition GrantsTypesControlReplacementPermissionSetAndSwitch = True
admitsSpan TypeAddition KeywordGrantOnly = False
admitsSpan TypeAddition RestrictionsShieldsPermissionsAndDelays = False
admitsSpan TypeAddition GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan TypeAddition ControlGrantAndPermission = False
admitsSpan TypeAddition GrantsRestrictionsControlAndPermission = False
admitsSpan TypeAddition PermissionOnly = False
admitsSpan ControlGrant Unattested = False
admitsSpan ControlGrant Unclaimed = False
admitsSpan ControlGrant GrantsAndControl = True
admitsSpan ControlGrant GrantsTypesControlReplacementPermissionSetAndSwitch = True
admitsSpan ControlGrant KeywordGrantOnly = False
admitsSpan ControlGrant RestrictionsShieldsPermissionsAndDelays = False
admitsSpan ControlGrant GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan ControlGrant ControlGrantAndPermission = True
admitsSpan ControlGrant GrantsRestrictionsControlAndPermission = True
admitsSpan ControlGrant PermissionOnly = False
admitsSpan Replacement Unattested = False
admitsSpan Replacement Unclaimed = False
admitsSpan Replacement GrantsAndControl = False
admitsSpan Replacement GrantsTypesControlReplacementPermissionSetAndSwitch = True
admitsSpan Replacement KeywordGrantOnly = False
admitsSpan Replacement RestrictionsShieldsPermissionsAndDelays = True
admitsSpan Replacement GrantsRestrictionsReplacementAndBaseSet = True
admitsSpan Replacement ControlGrantAndPermission = False
admitsSpan Replacement GrantsRestrictionsControlAndPermission = False
admitsSpan Replacement PermissionOnly = False
admitsSpan Prevention Unattested = False
admitsSpan Prevention Unclaimed = False
admitsSpan Prevention GrantsAndControl = False
admitsSpan Prevention GrantsTypesControlReplacementPermissionSetAndSwitch = False
admitsSpan Prevention KeywordGrantOnly = False
admitsSpan Prevention RestrictionsShieldsPermissionsAndDelays = True
admitsSpan Prevention GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan Prevention ControlGrantAndPermission = False
admitsSpan Prevention GrantsRestrictionsControlAndPermission = False
admitsSpan Prevention PermissionOnly = False
admitsSpan Conditional Unattested = False
admitsSpan Conditional Unclaimed = False
admitsSpan Conditional GrantsAndControl = False
admitsSpan Conditional GrantsTypesControlReplacementPermissionSetAndSwitch = False
admitsSpan Conditional KeywordGrantOnly = False
admitsSpan Conditional RestrictionsShieldsPermissionsAndDelays = False
admitsSpan Conditional GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan Conditional ControlGrantAndPermission = False
admitsSpan Conditional GrantsRestrictionsControlAndPermission = False
admitsSpan Conditional PermissionOnly = False
admitsSpan PlayPermission Unattested = False
admitsSpan PlayPermission Unclaimed = False
admitsSpan PlayPermission GrantsAndControl = False
admitsSpan PlayPermission GrantsTypesControlReplacementPermissionSetAndSwitch = True
admitsSpan PlayPermission KeywordGrantOnly = False
admitsSpan PlayPermission RestrictionsShieldsPermissionsAndDelays = True
admitsSpan PlayPermission GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan PlayPermission ControlGrantAndPermission = True
admitsSpan PlayPermission GrantsRestrictionsControlAndPermission = True
admitsSpan PlayPermission PermissionOnly = True
admitsSpan EntryRider Unattested = False
admitsSpan EntryRider Unclaimed = False
admitsSpan EntryRider GrantsAndControl = False
admitsSpan EntryRider GrantsTypesControlReplacementPermissionSetAndSwitch = False
admitsSpan EntryRider KeywordGrantOnly = False
admitsSpan EntryRider RestrictionsShieldsPermissionsAndDelays = False
admitsSpan EntryRider GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan EntryRider ControlGrantAndPermission = False
admitsSpan EntryRider GrantsRestrictionsControlAndPermission = False
admitsSpan EntryRider PermissionOnly = False
-- The cost statement writes NO adverbial, and the row is all False on a
-- measurement rather than on a principle. Of six hundred and fifty-three
-- supported cost-modification lines, exactly ONE carries a duration --
-- "Spells with the chosen name cost {1} less to cast this turn" (Cheering
-- Fanatic), which is unwritable for its chosen-name subject anyway -- and
-- flipping its cell would rename a `SpanUse` row that currently tells the
-- truth about which constructions write "this turn". So the one line is
-- recorded rather than admitted (ledger), and the two "during your turn"
-- lines are not durations at all but a window on when the reduction
-- applies.
admitsSpan CostModification Unattested = False
admitsSpan CostModification Unclaimed = False
admitsSpan CostModification GrantsAndControl = False
admitsSpan CostModification GrantsTypesControlReplacementPermissionSetAndSwitch = False
admitsSpan CostModification KeywordGrantOnly = False
admitsSpan CostModification RestrictionsShieldsPermissionsAndDelays = False
admitsSpan CostModification GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan CostModification ControlGrantAndPermission = False
admitsSpan CostModification GrantsRestrictionsControlAndPermission = False
admitsSpan CostModification PermissionOnly = False
-- The [CR#208.2a] DEFINITION writes no adverbial either, and its row is
-- all False for a stronger reason than the cost statement's: a
-- characteristic-defining ability is printed on the card it affects
-- ([CR#604.3a]) and functions in every zone, so there is no resolution to
-- start a span and no battlefield stay to end one. The measurement agrees
-- exactly -- of 142 supported lines writing "power and toughness are each
-- equal to", not one carries a duration, and the three lines that put the
-- sentence beside "until end of turn" GRANT it in quotation marks to
-- something that becomes a creature (Chimeric Mass, Myth Realized,
-- Svogthos), which is the becomes family's clause and not this row's.
admitsSpan PtDefinition Unattested = False
admitsSpan PtDefinition Unclaimed = False
admitsSpan PtDefinition GrantsAndControl = False
admitsSpan PtDefinition GrantsTypesControlReplacementPermissionSetAndSwitch = False
admitsSpan PtDefinition KeywordGrantOnly = False
admitsSpan PtDefinition RestrictionsShieldsPermissionsAndDelays = False
admitsSpan PtDefinition GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan PtDefinition ControlGrantAndPermission = False
admitsSpan PtDefinition GrantsRestrictionsControlAndPermission = False
admitsSpan PtDefinition PermissionOnly = False
-- The base-P/T SETTING writes two adverbials and no third. "Until end of
-- turn" is 63 of its 114 supported lines and "until your next turn" three
-- (Mass Diminish, Will Kenrith, Rowan // Will), which are the two cells
-- this row's name was added to; every other endpoint is zero, including
-- the current-turn "this turn" that the restrictions own -- the one line
-- that looks like it hangs the adverbial on its other conjunct ("That
-- creature can't be blocked this turn and has base power and toughness
-- 3/1", Atomic Microsizer).
admitsSpan BasePtSet Unattested = False
admitsSpan BasePtSet Unclaimed = False
admitsSpan BasePtSet GrantsAndControl = False
admitsSpan BasePtSet GrantsTypesControlReplacementPermissionSetAndSwitch = True
admitsSpan BasePtSet KeywordGrantOnly = False
admitsSpan BasePtSet RestrictionsShieldsPermissionsAndDelays = False
admitsSpan BasePtSet GrantsRestrictionsReplacementAndBaseSet = True
admitsSpan BasePtSet ControlGrantAndPermission = False
admitsSpan BasePtSet GrantsRestrictionsControlAndPermission = False
admitsSpan BasePtSet PermissionOnly = False
-- The SWITCH writes exactly one adverbial: all 25 supported lines end
-- "until end of turn", and the row is the tightest in the table because
-- of it. [CR#613.4d] is silent about durations, as the layer rules are
-- about every construction's surface, so this is the corpus's fact alone.
admitsSpan PtSwitch Unattested = False
admitsSpan PtSwitch Unclaimed = False
admitsSpan PtSwitch GrantsAndControl = False
admitsSpan PtSwitch GrantsTypesControlReplacementPermissionSetAndSwitch = True
admitsSpan PtSwitch KeywordGrantOnly = False
admitsSpan PtSwitch RestrictionsShieldsPermissionsAndDelays = False
admitsSpan PtSwitch GrantsRestrictionsReplacementAndBaseSet = False
admitsSpan PtSwitch ControlGrantAndPermission = False
admitsSpan PtSwitch GrantsRestrictionsControlAndPermission = False
admitsSpan PtSwitch PermissionOnly = False

||| Whether a construction can write NO duration at all. A grant can: the
||| unwritten span is [CR#611.2a]'s end-of-game default, which the guide
||| permits where the effect is "intentionally indefinite under the
||| rules" (Through the Breach's bare "It gains haste."). A restriction
||| cannot — a durationless "can't" is the STATIC ability line
||| ("Enchanted creature can't attack", Pacifism), a different
||| construction and the parked ability layer's, so the whole clause is
||| unwritable here rather than the span being optional (`badStaticCant`).
|||
||| The type addition and the control grant answer `True`, and there the
||| unwritten span is the NORM rather than the exception: most "in
||| addition to its other types" lines and the bare "Gain control of
||| target creature." state no duration, with Memnarch and Phyrexian
||| Infiltrator printing [CR#611.2a]'s default out loud in reminder text —
||| "(This effect lasts indefinitely.)"
|||
||| Both SHIELD rows answer `False`, for the deed restriction's reason
||| exactly: a durationless interception or shield is the STATIC ABILITY
||| line and not a clause at all. [CR#603.6d] says so for the entry riders
||| in as many words and [CR#611.3] for the rest — a continuous effect
||| from a static ability carries no duration because it lasts while the
||| ability functions. The corpus divides on the same line: every one-shot
||| interception and every one-shot shield states a span, while the
||| durationless "Prevent all …" lines are static abilities to a line and
||| the standing interceptions are the permanent's own ability. So the
||| whole clause is unwritable here rather than the span being optional
||| (`badStandingIntercept`, `badStandingPrevention`).
public export
absentOk : StaticKind -> Bool
absentOk PtDelta = True
absentOk KeywordGrant = True
absentOk DeedRestriction = False
absentOk TypeAddition = True
absentOk ControlGrant = True
absentOk Replacement = False
absentOk Prevention = False
-- The three new rows answer `False` for the restriction's reason taken
-- one step further: they are not clause constructions at all. A
-- durationless "as long as" static, a durationless entry rider and a
-- durationless play permission are each the static ABILITY line and
-- nothing else, so there is no `Continuously` clause for the span to be
-- absent from (`badConditionalClause`, `badEntryRiderClause`,
-- `badStandingPermission`). The PERMISSION is the sharpest of the three
-- because it writes spans freely as a clause and states none at all only
-- when it is a card's own line ("You may cast this card from your
-- graveyard").
absentOk Conditional = False
absentOk PlayPermission = False
absentOk EntryRider = False
-- A fourth row with the same answer for the same reason: every cost
-- statement in the corpus is a static ability's own LINE ("Noncreature
-- spells cost {1} more to cast"), never a clause a resolution
-- establishes, so there is no `Continuously` for its span to be absent
-- from (`badCostClause`).
absentOk CostModification = False
-- The DEFINITION is the fifth, and the rule says it rather than the
-- corpus: a characteristic-defining ability is printed on the card it
-- affects ([CR#604.3a]) and functions everywhere ([CR#208.2a]), so it is
-- a line and there is no clause for its span to be absent from
-- (`badPtDefinitionClause`).
absentOk PtDefinition = False
-- The base-P/T setting answers `True`, and the unwritten span is the
-- ordinary [CR#611.2a] default the pump and the control grant already
-- take: six supported lines establish the setting with no duration at all
-- ("{2}{U}: This creature becomes blue and has base power and toughness
-- 5/4", Surge Engine; Tezzeret, Betrayer of Flesh; Tenth District Hero;
-- Aven Mimeomancer; Cycle of Life; Curious Colossus), and each of them
-- means the indefinite change the rule supplies.
absentOk BasePtSet = True
-- The SWITCH answers `False` on a total measurement: all 25 lines state
-- "until end of turn" and nothing writes a permanent switch, which makes
-- the durationless clause the one shape this family never takes
-- (`badStandingSwitch`).
absentOk PtSwitch = False

||| Whether English writes this static effect as a bare ability LINE —
||| the OTHER reader of the same axis. [CR#604.1] says what the line is
||| ("static abilities … are written as statements, and they're simply
||| true") and [CR#611.3b] says why it states no duration (the continuous
||| effect "applies at all times that the permanent generating it is on
||| the battlefield"), so the question is only which of these effects
||| English is willing to state.
|||
||| All but two rows answer yes, and both refusals say something about
||| ENGLISH rather than about the effect. The ability line
||| and the resolving clause differ in ASPECT, not in vocabulary: the
||| clause is an instruction and takes the inchoative verb, the line is a
||| statement and takes the stative one — "Target creature gains flying
||| until end of turn" against "Creatures you control have haste",
||| "becomes an Island in addition to its other types" against "are the
||| chosen type in addition to their other types". The stat delta and the
||| deed restriction write ONE word in both frames ("get", "can't")
||| because English has no separate inchoative for them, which is why the
||| split was invisible until a construction needed both.
|||
||| `ControlGrant` is the first row that says no, and the reason is that
||| the stative form of "gain control of" is a different VERB. English
||| writes the standing fact as "You control enchanted creature" and writes
||| "gains control of" as a durationless line zero times, so the line is a
||| construction this row does not spell rather than an inflection of it
||| (`badStaticGainsControl`, ledger).
|||
||| `PtSwitch` is the second, and it is the cleanest case in the table
||| because the verb has no stative form to try: the corpus writes
||| "switch" as an instruction 25 times out of 25 and a permanent's
||| standing "this creature's power and toughness are switched" zero times
||| (`badSwitchLine`). The DEFINITION is the opposite extreme and answers
||| yes for the same kind of reason the cost statement does — it is a
||| printed line and nothing else, [CR#604.3a] making "printed on the card
||| it affects" one of the five criteria that make an ability one at all.
public export
staticAsAbility : StaticKind -> Bool
staticAsAbility PtDelta = True
staticAsAbility KeywordGrant = True
staticAsAbility DeedRestriction = True
staticAsAbility TypeAddition = True
staticAsAbility ControlGrant = False
staticAsAbility Replacement = True
staticAsAbility Prevention = True
staticAsAbility Conditional = True
staticAsAbility PlayPermission = True
staticAsAbility EntryRider = True
staticAsAbility CostModification = True
staticAsAbility PtDefinition = True
staticAsAbility BasePtSet = True
staticAsAbility PtSwitch = False

||| Which zones a play permission names as its source ([CR#604.6] — a
||| static ability of this shape "appl(ies) while a card is in any zone
||| that you could cast or play it from"). Full rows over the five zones
||| and the silence, because the permission's own phrase is what places
||| its card: the impulse family exiles first and then permits, the
||| graveyard family names the zone in the permission ("You may cast this
||| card from your graveyard"), the library family names the top, and the
||| hand is the rules' own default, named per card type ([CR#302.1] for a
||| creature, [CR#305.1] for a land). The BATTLEFIELD is the one refusal
||| and it is a rules fact rather than a count: a permanent on the
||| battlefield has already been played (`badPlayFromBattlefield`).
||| Silence passes for `zoneFits`' reason — an untracked referent asserts
||| nothing about where it is.
public export
playableFrom : Maybe Zone -> Bool
playableFrom Nothing = True
playableFrom (Just Battlefield) = False
playableFrom (Just Graveyard) = True
playableFrom (Just Exile) = True
playableFrom (Just Hand) = True
playableFrom (Just Library) = True
-- The STACK is the second refusal and it is the battlefield's reason
-- exactly one step earlier: [CR#112.1] makes an object there a spell,
-- and a spell has already been cast. [CR#113.6d,113.6e] are the near
-- miss and worth naming — an ability that modifies what its own object
-- costs to cast, or restricts how it can be cast, "functions on the
-- stack" — but that is a cost or a restriction functioning there, not a
-- permission to cast something already on it (`badPlayFromStack`).
playableFrom (Just Stack) = False

public export
data PlayableFrom : Maybe Zone -> Type where
  MkPlayableFrom : {auto 0 ok : playableFrom z = True} -> PlayableFrom z

||| The verb's demand on its complement, as a witness named apart so a
||| pin says which of the permission's two questions refused.
public export
data CastableTy : PlayVerb -> Maybe CardType -> Type where
  MkCastableTy : {auto 0 ok : castableTy v ty = True} -> CastableTy v ty

||| Which adverbial a DELAYED trigger states as its duration
||| ([CR#603.7b] — it "will trigger only once … unless it has a stated
||| duration, such as 'this turn'"). The rule names the phrase and the
||| corpus writes no other. Full rows over `SpanUse`, so a new adverbial
||| class declares this reader too, and every class but the current-turn
||| one is `False` — the delayed clause writes no boundary endpoint and no
||| for-as-long-as ([CR#611.2b]'s adverbial belongs to a continuous
||| effect, and a delayed trigger is not one).
public export
admitsDelaySpan : SpanUse -> Bool
admitsDelaySpan Unattested = False
admitsDelaySpan Unclaimed = False
admitsDelaySpan GrantsAndControl = False
admitsDelaySpan GrantsTypesControlReplacementPermissionSetAndSwitch = False
admitsDelaySpan KeywordGrantOnly = False
admitsDelaySpan RestrictionsShieldsPermissionsAndDelays = True
admitsDelaySpan GrantsRestrictionsReplacementAndBaseSet = False
admitsDelaySpan ControlGrantAndPermission = False
admitsDelaySpan GrantsRestrictionsControlAndPermission = False
admitsDelaySpan PermissionOnly = False

||| How an indefinite phrase MARKS its choice method — the axis three
||| constructors used to spell three times. Choice method is surface data
||| (finding 11): no CR rule derives a chooser, so what the grammar
||| records is what the text writes, and the phrase itself is one
||| determiner throughout ("a …", article and all).
|||
||| Core keeps the same axis off the choice itself: `Binder::ChooseOne`
||| carries the filter and a separate `by` slot naming who chooses,
||| defaulting to the controller and OVERRIDDEN for the foreign chooser
||| ("that player sacrifices a creature of their choice",
||| [CR#608.2d,701.21a], `binder.rs`), while the chooserless form is a
||| different constructor entirely (`Selection::Random`, `selection.rs`).
||| `Unmarked` is core's elided `by`, `TheirChoice` its override,
||| `AtRandom` its random sibling.
|||
||| `TheirChoice` carries the pronoun's own obligation: "their" is a
||| POSSESSIVE, so it needs exactly one player antecedent — a singular
||| subject or one distributive group (`countChoosers`,
||| `badUnboundTheirChoice`). A NOMINAL chooser slot mirroring core's
||| `by: Reference` waits on the plural player read the distributive
||| antecedent would need (ledger).
public export
-- spelling: (construction-owned catalog -- each row is the ADVERBIAL an
-- indefinite phrase writes after its noun, and each is spelled through
-- its own macro: Unmarked = `a` (no adverbial), TheirChoice =
-- `aTheirChoice`'s "of their choice", AtRandom = `aAtRandom`'s "at
-- random". Never spelled alone -- see Experimental.Macros)
data ChoiceMode : Bindings -> Type where
  Unmarked : ChoiceMode bs
  TheirChoice : {auto 0 ch : countChoosers bs = 1} -> ChoiceMode bs
  AtRandom : ChoiceMode bs

-- ===== The grammar (mutual: types thread contexts through VALUES) =====

||| WHICH zones English writes a possessor for — the closed table behind
||| the owned zone phrase, and [CR#400.1] is the whole of it: "Each player
||| has their own library, hand, and graveyard. The other zones are shared
||| by all players." So "your hand" and "an opponent's graveyard" are
||| phrases and "your battlefield" is not, and the reason is a fact about
||| the ZONE rather than about the phrase that names it. A new shared zone
||| declares its absence by having no row to write.
public export
data Possessable : Zone -> Type where
  HandIsOwned : Possessable Hand
  GraveyardIsOwned : Possessable Graveyard
  LibraryIsOwned : Possessable Library
