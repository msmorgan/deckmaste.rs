import Semantics.Triggers

/-!
# Semantics.Abilities

Abilities and their parts: instructions, static specs, costs, tokens, and the
characteristics an object carries. Port of `idris/src/Experimental/Effect.idr`, syntax only,
under the name of its top type: an Ability is what generates Effects [CR#113.1,609.1], and
nothing here is an Effect.

`Characteristics` is [CR#109.3] as a structure, flat, and serves cards and tokens alike; a
token's power is an `Amount` ("X/X"), a card's is a literal, and `none` on a stat slot is a
slot a characteristic-defining ability fills (printed `*`). A `CharacteristicBundle` is a
characteristics set as an effect writes it, with the qualities an effect can add [CR#111.3].
-/

namespace Semantics

/-- What mana may be spent on. -/
inductive SpendPurpose where
  | toCast (predicate : Predicate)
  | toActivate (source : Option Predicate)
  | toPay (cost : CostNamed)
  deriving Repr, BEq

inductive ManaHeld where
  | thisMana
  | unspent (type : Option ColorOrColorless)
  deriving Repr, BEq

inductive AsThough where
  | of (predicate : Predicate)
  | mana (what : Option ColorOrColorless) (as_ : ManaMatch) (purpose : Option SpendPurpose)
  | greater (stat : Stat) (amount : Amount)
  deriving Repr, BEq

inductive Exchanged where
  | lifeTotals (parties : NounPhrase)
  | controlOf (left right : NounPhrase)
  | cardsAcross (left right : NounPhrase)
  | zones (left right : ZoneExpr)
  /-- Two numerical values [CR#701.12g]: life totals, powers or toughnesses, rolled results. -/
  | values (left right : Amount)
  /-- Two permanents' text boxes. -/
  | textBoxes (left right : NounPhrase)
  deriving Repr, BEq

inductive TokenQuality where
  | withEveryType (space : SubtypeSpace)
  | withQuality (quality : Predicate)
  deriving Repr, BEq

/-- Where a counter's kind comes from. -/
inductive CounterKindSource where
  | printed (kind : CounterKind)
  | chosen (menu : List CounterKind)
  | distinctChosen (menu : List CounterKind)
  | bound
  | those
  | own
  | sameAs (source : NounPhrase)
  deriving Repr, BEq

inductive QualityOp where
  | adds | sets | loses
  deriving DecidableEq, Repr

/-- Only present axes are written. A present empty list explicitly clears that axis. -/
structure TypeLineChanges where
  supertypes : Option (List Supertype) := none
  types : Option (List CardType) := none
  subtypes : Option (List Subtype) := none
  retained : Option CardType := none
  deriving Repr, BEq

inductive CharacteristicStat where
  | power | toughness | loyalty | defense
  deriving DecidableEq, Repr

inductive CostShift where
  | less (amount : Amount) (floor : Option Amount)
  | more (amount : Amount)
  | run (run : ManaCost) (rises : Bool) (coloredOnly : Bool)
  deriving Repr, BEq

inductive CountBound where
  | moreThan (amount : Amount)
  | additional (quantity : Quantity)
  deriving Repr, BEq

structure DeedComplement where
  deed : Deed
  counterpart : NounPhrase
  deriving Repr, BEq

/-- Whom a deontic rule's deed is done to. -/
inductive DeonticPatient where
  | noPatient
  | defendingPlayer (patient : NounPhrase)
  | counterpart (patient : NounPhrase)
  | targetedBy (patient : NounPhrase)
  | counterpartsAt (complements : List DeedComplement)
  deriving Repr, BEq

inductive DamageScope where
  | everywhere
  | toRecipient (recipient : NounPhrase)
  deriving Repr, BEq

inductive DamageAgent where
  | unattributed
  | dealtBy (source : NounPhrase)
  deriving Repr, BEq

inductive Unpreventable where
  | described (source : DamageAgent) (scope : DamageScope)
  | thatDamage
  deriving Repr, BEq

inductive PreventionBan where
  | noPreventionOnly | noRedirectEither
  deriving DecidableEq, Repr

inductive PreventCut where
  | all
  | some (amount : Amount)
  | shield (amount : Amount)
  | allBut (amount : Amount)
  | half (rounding : RoundMode)
  deriving Repr, BEq

inductive DamageScale where
  | multiplied (factor : ScaleFactor)
  | halved (rounding : RoundMode)
  | shifted (direction : ShiftDir) (amount : Amount)
  deriving Repr, BEq

inductive DividedVerb where
  | damage (source : NounPhrase)
  | counters (kind : CounterKind)
  deriving Repr, BEq

inductive ProducedMana where
  | runs (runs : List ProducedRun)
  | anyColor (freedom : ColorFreedom)
  | ofChosenColor (alternative : Option ProducedRun)
  | asPrintedCost (subject : NounPhrase)
  | producedByEvent (subject : NounPhrase)
  | couldProduce (subject : NounPhrase)
  | amongColorsOf (subject : NounPhrase)
  | amongWritten (colors : List Color)
  | lastNoted (subject : NounPhrase)
  deriving Repr, BEq

inductive SpentMode where
  | affectsIt | triggersThen
  deriving DecidableEq, Repr

/-- Optional payment is decided by its player; required payment tests whether it started
[CR#118.12]. -/
inductive ContinuationPolicy where
  | optional (agent : NounPhrase)
  | required
  deriving Repr, BEq

inductive DeckTrait where
  | aCharacteristic (predicate : Predicate)
  /-- "cards with even mana values" [CR#202.3] -/
  | manaValueParity (parity : Parity)
  /-- "more than one of the same mana symbol in its mana cost" -/
  | repeatedManaSymbol
  /-- "has an activated ability" -/
  | hasAbilityOf (class_ : AbilityClass)
  /-- "... and land cards" -/
  | anyOf (traits : List DeckTrait)
  deriving Repr, BEq

inductive DeckCondition where
  /-- "Each permanent card in your starting deck has mana value 2 or less." -/
  | everyCardIs (scope : Predicate) (trait : DeckTrait)
  /-- "No card in your starting deck has more than one of the same mana symbol in its mana
  cost." -/
  | noCardIs (scope : Predicate) (trait : DeckTrait)
  /-- "Each nonland card in your starting deck has a different name." -/
  | cardsDiffer (scope : Predicate) (axis : QualitySort)
  /-- "Each nonland card in your starting deck shares a card type." -/
  | cardsShare (scope : Predicate) (axis : QualitySort)
  /-- "at least twenty cards more than the minimum deck size", a minimum the format sets
  [CR#100.2a,100.2b]. -/
  | deckSizeOverMinimum (extra : Nat)
  deriving Repr, BEq

/-- Combat participation is independent of the set of blocking relations. -/
inductive CombatParticipation where
  | attacking (defender : Option NounPhrase)
  | blocked (value : Bool)
  | outsideCombat
  deriving Repr, BEq

inductive CombatUpdate where
  | participation (state : CombatParticipation)
  /-- Adding a blocker makes its attacker blocked; removing the relation leaves that
  blockedness unchanged. -/
  | blocking (move : AttachMove) (attacker : NounPhrase)
  deriving Repr, BEq

mutual
  inductive Repetition where
    | again
    | moreTimes (times : Amount)
    | anyNumber
    | untilCond (condition : Condition)
    | againExcludingChosen
    | fixed (times : Amount) (body : Instruction)

  /-- An object's characteristics: name, mana cost, color and color indicator, card type,
  subtype, supertype, rules text and abilities, power, toughness, loyalty, and defense
  [CR#109.3]. Every field defaults to absent. -/
  structure Characteristics where
    name : Option String := none
    cost : Option ManaCost := none
    /-- A color indicator, or a token's colors; empty means "from the mana cost". -/
    colors : List Color := []
    supertypes : List Supertype := []
    types : List CardType := []
    subtypes : List Subtype := []
    text : List Ability := []
    power : Option Amount := none
    toughness : Option Amount := none
    loyalty : Option Amount := none
    defense : Option Amount := none

  /-- A characteristics set as an effect writes it, with the token qualities ("with every
  creature type") an effect can add [CR#111.3]. -/
  structure CharacteristicBundle where
    characteristics : Characteristics
    qualities : List TokenQuality := []

  /-- Typed characteristic writes; the enclosing static or copy form retains its context. -/
  inductive CharacteristicEdit where
    | typeLine (op : QualityOp) (changes : TypeLineChanges)
    | name (op : QualityOp) (name : String)
    | manaCost (cost : Option ManaCost)
    | colors (op : QualityOp) (colors : ColorSpec)
    | stat (op : QualityOp) (axis : CharacteristicStat) (amount : Amount)
    | everyTypeOf (op : QualityOp) (space : SubtypeSpace)
    | chosenQuality (op : QualityOp) (quality : Predicate)
    | addedAbilities (abilities : List Ability)
    | removedAbilities (selection : AbilitySelection)

  inductive AbilitySelection where
    | specified (abilities : List AbilityLost)
    | family (class_ : AbilityClass)
    | allExcept (except : Option Predicate)

  inductive TokenSpec where
    /-- The creating spell or ability defines the token's characteristic values [CR#111.3] and
    sets its name and subtypes [CR#111.4]. -/
    | written (characteristics : CharacteristicBundle)
    | asThose
    | copyOf (source : NounPhrase) (exceptions : List CopyExcept)

  inductive StaticSpec where
    | withBindings (scope : Nat) (inputs : List CaptureInput) (body : StaticSpec)
    | inCaller (scope : Nat) (body : StaticSpec)
    | modification (subject : NounPhrase) (stat : Stat) (delta : Delta Amount)
    | ptDefinition (subject : NounPhrase) (slots : DefinedSlots) (amount : Amount)
    | ptSwitch (subject : NounPhrase)
    | costShift (subject : NounPhrase) (shift : CostShift)
    | altCost (subject : NounPhrase) (cost : Option Cost)
    | addedCost (cost : Cost) (offered : Bool)
    | letterDefinition (letter : Letter) (amount : Amount)
    | abilityGrant (subject : NounPhrase) (ability : Ability)
    | abilityGrantFrom (subject : NounPhrase) (classes : List AbilityClass) (source : NounPhrase)
        (except : Option Predicate)
    | deonticRule (subject : NounPhrase) (compulsion : Compulsion) (deeds : Deeds) (role : Role)
        (bound : Option CountBound) (patient : DeonticPatient) (asThough : Option AsThough)
        (rider : DeonticRider)
    | manaRetention (player : NounPhrase) (mana : ManaHeld)
    | partSkip (player : NounPhrase) (part : TurnPart)
    | characteristicChange (subject : NounPhrase) (edits : List CharacteristicEdit)
    | retention (spec : StaticSpec) (subject : NounPhrase)
    | copyChange (subject : NounPhrase) (source : NounPhrase) (exceptions : List CopyExcept)
    | controlGrant (player : NounPhrase) (subject : NounPhrase)
    | replacement (event : GameEvent) (alternatives : List GameEvent) (timing : Option Timing)
        (replacement : Instruction) (use : ReplUse) (limit : Option UsageLimit)
    | damageRule (kind : DamageKind) (source : DamageAgent) (scope : DamageScope) (op : DamageOp)
        (use : ReplUse)
    | preventionBan (kind : DamageKind) (damage : Unpreventable) (ban : PreventionBan)
    | conditional (spec : StaticSpec) (condition : Condition) (marking : CondMarking)
    | visibility (verb : ExposeVerb) (player : NounPhrase) (what : VisibleThing)
    | additionalTriggers (event : GameEvent) (quantity : Quantity)
    | entryRider (subject : NounPhrase) (rider : TokenRider)
    | choice (occasion : ChoiceOccasion) (subject : NounPhrase) (sort : ChoiceSort)
        (domain : Option ChoiceDomain) (disclosure : Disclosure)
    | conjunction (subject : Option NounPhrase) (parts : List StaticSpec)

  inductive Compulsion where
    | forbid
    | require
    | gatedBy (cost : Cost)
    | permit

  inductive PlayPayment where
    | itsOwnCost
    | withoutPaying
    | payingInstead (cost : Cost)

  inductive DeonticRider where
    | stateBased (cause : StateBasedCause)
    | noRider
    | play (from_ : Option ZoneExpr) (limit : Option PlayLimit) (timing : Option PlayTiming)
        (exclusive : Bool) (payment : PlayPayment)

  inductive DamageOp where
    | prevent (cut : PreventCut) (also : Option Instruction)
    | redirect (cut : PreventCut) (to : NounPhrase)
    | scale (scale : DamageScale)

  inductive TokenRider where
    | entersAs (status : Status)
    | entersAttacking (defender : Option NounPhrase)
    | entersTransformed
    | entersMelded (into : String)
    | withCounters (amount : Amount) (kind : CounterKindSource) (mark : EntryCounterMark)
    | under (controller : NounPhrase)
    | asCopyOf (optional : Bool) (source : NounPhrase) (exceptions : List CopyExcept)

  inductive Cost where
    | withBindings (scope : Nat) (inputs : List CaptureInput) (body : Cost)
    | inCaller (scope : Nat) (body : Cost)
    | mana (cost : ManaCost)
    | scaled (cost : Cost) (amount : Amount)
    | tapSymbol
    | untapSymbol
    | loyaltySymbol (loyalty : LoyaltyCost)
    /-- An instruction performed as a cost ("Sacrifice a creature:"). -/
    | perform (instruction : Instruction)
    | compound (costs : List Cost)
    | or (costs : List Cost)
    | itsManaCost

  inductive ManaRider where
    | spendOnly (purposes : List SpendPurpose)
    | spendNotOn (purposes : List SpendPurpose)
    | onSpent (mode : SpentMode) (only : Bool) (spell : NounPhrase) (says : Instruction)

  inductive CopyExcept where
    | edits (edits : List CharacteristicEdit)
    | ability (ability : Ability)
    | thisAbility
    | entersWithCounters (amount : Amount) (kind : CounterKind) (mark : EntryCounterMark)

  structure RollRow where
    results : Quantity
    instruction : Instruction

  inductive CreationSpec where
    | token (spec : TokenSpec) (riders : List TokenRider)
    | emblem (abilities : List Ability)

  inductive Instruction where
    /-- Bind ordered typed inputs for the body, without an implicit whole-body precondition. -/
    | withBindings (scope : Nat) (inputs : List CaptureInput) (body : Instruction)
    | inCaller (scope : Nat) (body : Instruction)
    | dealDamage (source : NounPhrase) (amount : Amount) (recipient : NounPhrase)
    | setStatus (status : Status) (subject : NounPhrase)
    | turnOver (subject : NounPhrase)
    | combat (subject : NounPhrase) (update : CombatUpdate)
    | attachment (move : AttachMove) (subject : NounPhrase) (host : Option NounPhrase)
    /-- Remove all damage marked on the permanent [CR#120.6]. -/
    | clearDamage (subject : NounPhrase)
    /-- "… can't be regenerated this turn": an instruction plus the deed it forbids. -/
    | doAndForbid (instruction : Instruction) (deed : Deed) (subject : NounPhrase)
    | gainDesignation (subject : NounPhrase) (designation : DesignationLabel)
        (conferral : Conferral) (duration : Option Duration)
    | unlock (door : Door)
    | setGameDesignation (designation : DesignationLabel)
    | conclude (verb : OutcomeVerb) (agent : NounPhrase := .you)
    | drawGame
    | restartGame
    | separateIntoPiles (group : NounPhrase) (piles : Nat) (faces : List PileFace)
        (agent : NounPhrase := .you)
    /-- The `when` rider is the printed "as you activate this ability": the announcement's
    timing, not a condition on what may be chosen. -/
    | choose (first : Option NounPhrase) (chosen : NounPhrase) (disclosure : Disclosure)
        (when : Option Concurrent) (agent : Option NounPhrase := none)
    | revealChoices (sort : HiddenSort)
    | vote (first : Option NounPhrase) (disclosure : Disclosure) (ballot : Ballot)
        (agent : NounPhrase := .you)
    | move (subject : NounPhrase) (to : ZoneExpr) (riders : List TokenRider)
    | copy (sort : CopySort) (subject : NounPhrase) (times : Amount) (exceptions : List CopyExcept)
        (agent : NounPhrase := .you)
    | chooseNewTargets (subject : NounPhrase)
    | copyTargets (copy : NounPhrase) (original : NounPhrase)
    | changeLife (delta : Delta Amount) (agent : NounPhrase := .you)
    | exchange (exchanged : Exchanged)
    | addMana (amount : Amount) (produced : ProducedMana) (riders : List ManaRider)
        (agent : NounPhrase := .you)
    | draw (amount : Amount) (agent : NounPhrase := .you)
    | expose (verb : ExposeVerb) (exposed : Exposed) (agent : NounPhrase := .you)
    | search (scope : SearchScope) (quantity : Quantity) (predicate : Predicate)
        (agent : NounPhrase := .you)
    | shuffle (agent : NounPhrase := .you)
    | flipCoins (count : FlipScope) (agent : NounPhrase := .you)
    | rollDice (count : Amount) (sides : DieSides) (agent : NounPhrase := .you)
    | applyResultsTable (rows : List RollRow)
    | ignoreOutcomes (which : IgnoredOutcomes)
    | shiftResult (direction : Option ShiftDir) (amount : Amount)
    | storeResults (on : NounPhrase)
    | rerollStored (quantity : Quantity) (whose : NounPhrase) (agent : NounPhrase := .you)
    | establish (spec : StaticSpec) (duration : Option Duration)
    | createObject (count : Amount) (spec : CreationSpec) (agent : NounPhrase := .you)
    | putCounters (amount : Amount) (kind : CounterKindSource) (on : NounPhrase)
    | distribute (verb : DividedVerb) (amount : Amount) (among : NounPhrase)
    | removeCounters (quantity : Option Quantity) (kind : Option CounterKindSource)
        (from_ : NounPhrase)
    | moveCounters (amount : Amount) (kind : Option CounterKindSource) (source : NounPhrase)
        (destination : NounPhrase)
    | doubleCounters (on : NounPhrase)
    /-- A named deed done: the deed's own facts row gates the agent, the patient and the zones
    the sentence may name, whichever of the three sources defines it. -/
    | enact (verb : Deed) (instruction : Instruction) (agent : Option NounPhrase := none)
    | pay (cost : Cost) (times : PayTimes) (agent : NounPhrase := .you)
    /-- Branch on the decision or start of payment, independently of resulting events. -/
    | withContinuation (policy : ContinuationPolicy) (body : Instruction)
        (ifDid : Option Instruction) (ifNot : Option Instruction)
    | doOnlyIf (instruction : Instruction) (condition : Condition) (otherwise : Option Instruction)
    | doIf (condition : Condition) (instruction : Instruction) (otherwise : Option Instruction)
    | doForEach (group : NounPhrase) (body : Instruction)
    | doForEachKind (axis : KindAxis) (domain : Option NounPhrase) (sort : QualitySort)
        (body : Instruction)
    | repeat_ (repetition : Repetition)
    | sequentially (steps : List Instruction)
    | simultaneously (steps : List Instruction)
    | chooseModes (quantity : Quantity) (modes : List (Option Cost × Instruction))
    | delay (event : GameEvent) (alternatives : List GameEvent) (duration : Option Duration)
        (body : Instruction)
    | replace (replaced : Instruction) (replacement : Instruction)
    | holdUntil (instruction : Instruction) (event : GameEvent)
    | triggerReflexively (body : Instruction) (trigger : Instruction)
    | triggerThisWay (body : Instruction) (event : GameEvent) (trigger : Instruction)
    | skipUntap (subject : NounPhrase) (steps : Amount)
    | skipPart (part : TurnPart) (count : Amount) (agent : NounPhrase := .you)
    | insertPart (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
        (followedBy : Option TurnPart) (agent : Option NounPhrase := none)

  inductive KeywordParam where
    | cost (cost : Cost)
    | quality (predicate : Predicate)
    | subject (predicate : Predicate)
    | number (amount : Amount)
    | deckCondition (condition : DeckCondition)

  inductive AbilityLost where
    | written (ability : Ability)
    | term (keyword : KeywordTerm)

  inductive Ability where
    | keyword (keyword : KeywordLabel) (params : List KeywordParam) (body : Option Ability)
    | activated (cost : Cost) (instruction : Instruction) (timing : Option Timing)
        (limit : Option UsageLimit) (guard : Option Condition) (activator : Option NounPhrase)
    | triggered (event : GameEvent) (alternatives : List GameEvent)
        (while_ : Option Concurrent) (joins : List JoinedHeader) (timing : Option Timing)
        (limit : Option UsageLimit) (intervening : Option Condition) (instruction : Instruction)
    | static (spec : StaticSpec)
    | spell (timing : Option Timing) (instruction : Instruction)
    /-- "You may begin the game with this on the battlefield" (the Leylines). -/
    | mayBeginOnBattlefield
    /-- "The same is true for first strike, double strike, …" (Odric). -/
    | alsoForKeywords (ability : Ability) (keywords : List KeywordTerm)
    /-- An italic head before an ability: an ability word or a flavor word. -/
    | italicHead (word : ItalicWord) (ability : Ability)
    /-- "that ability", read off an ability chosen earlier in the same text. -/
    | thatAbility (ref : ChoiceRef)
end

deriving instance Repr, BEq for Characteristics, CharacteristicBundle, CharacteristicEdit, AbilitySelection,
  TokenSpec, StaticSpec, Compulsion, PlayPayment, DeonticRider, DamageOp, TokenRider, Cost,
  ManaRider, CopyExcept, RollRow, Repetition, CreationSpec, Instruction, KeywordParam, AbilityLost, Ability

end Semantics

attribute [semantic_expression] Semantics.Instruction Semantics.StaticSpec Semantics.Cost Semantics.Ability Semantics.TokenSpec

classify_semantic_syntax

attribute [internal_expansion] Semantics.Instruction.enact Semantics.Instruction.withBindings
  Semantics.Instruction.inCaller Semantics.Cost.withBindings Semantics.Cost.inCaller
  Semantics.StaticSpec.withBindings Semantics.StaticSpec.inCaller
