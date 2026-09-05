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

inductive Repetition where
  | again
  | moreTimes (times : Amount)
  | anyNumber
  | untilCond (condition : Condition)
  | againExcludingChosen
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

mutual
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

  inductive QualityPayload where
    | bundle (characteristics : CharacteristicBundle) (retained : Option CardType)
    | everyTypeOf (space : SubtypeSpace)
    | chosenQuality (quality : Predicate)
    | colored (colors : ColorSpec)

  inductive TokenSpec where
    /-- The creating spell or ability defines the token's characteristic values [CR#111.3] and
    sets its name and subtypes [CR#111.4]. -/
    | written (characteristics : CharacteristicBundle)
    | asThose
    | copyOf (source : NounPhrase) (exceptions : List CopyExcept)

  inductive StaticSpec where
    | modify (subject : NounPhrase) (stat : Stat) (delta : Delta Amount)
    | definesPt (subject : NounPhrase) (slots : DefinedSlots) (amount : Amount)
    | switchesPt (subject : NounPhrase)
    | costs (subject : NounPhrase) (shift : CostShift)
    | altCost (subject : NounPhrase) (cost : Option Cost)
    | addedCost (cost : Cost) (offered : Bool)
    | definesLetter (letter : Letter) (amount : Amount)
    | gains (subject : NounPhrase) (ability : Ability)
    | gainsAbilitiesOf (subject : NounPhrase) (classes : List AbilityClass) (source : NounPhrase)
        (except : Option Predicate)
    | deontic (subject : NounPhrase) (compulsion : Compulsion) (deeds : Deeds) (role : Role)
        (bound : Option CountBound) (patient : DeonticPatient) (asThough : Option AsThough)
        (rider : DeonticRider)
    | keepsUnspentMana (player : NounPhrase) (mana : ManaHeld)
    | skips (player : NounPhrase) (part : TurnPart)
    | becomes (subject : NounPhrase) (op : QualityOp) (payload : QualityPayload)
    | alsoOffBattlefield (spec : StaticSpec)
    | doesntRemove (spec : StaticSpec) (subject : NounPhrase)
    | becomesCopy (subject : NounPhrase) (source : NounPhrase) (exceptions : List CopyExcept)
    | losesAllAbilities (subject : NounPhrase) (except : Option Predicate)
    | losesAbilities (subject : NounPhrase) (abilities : List AbilityLost)
    | gainsControl (player : NounPhrase) (subject : NounPhrase)
    | intercepts (event : GameEvent) (alternatives : List GameEvent) (timing : Option Timing)
        (replacement : Instruction) (use : ReplUse) (limit : Option UsageLimit)
    | damageRule (kind : DamageKind) (source : DamageAgent) (scope : DamageScope) (op : DamageOp)
        (use : ReplUse)
    | cantPrevent (kind : DamageKind) (damage : Unpreventable) (ban : PreventionBan)
    | conditionally (spec : StaticSpec) (condition : Condition) (marking : CondMarking)
    | onlyDuring (part : TurnPart) (whose : Option NounPhrase) (spec : StaticSpec)
    | visibility (verb : ExposeVerb) (player : NounPhrase) (what : VisibleThing)
    | triggersAdditionally (event : GameEvent) (quantity : Quantity)
    | entersRider (subject : NounPhrase) (rider : TokenRider)
    | entersChoice (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain)
        (disclosure : Disclosure)
    | attachChoice (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain)
    | andAlso (subject : Option NounPhrase) (parts : List StaticSpec)

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
    | mana (cost : ManaCost)
    | scaled (cost : Cost) (amount : Amount)
    | tapSymbol
    | untapSymbol
    | loyaltySymbol (loyalty : LoyaltyCost)
    /-- An instruction performed as a cost ("Sacrifice a creature:"). -/
    | perform (instruction : Instruction)
    | compound (costs : List Cost)
    | either (left right : Cost)
    | itsManaCost

  inductive ManaRider where
    | spendOnly (purposes : List SpendPurpose)
    | spendNotOn (purposes : List SpendPurpose)
    | onSpent (mode : SpentMode) (only : Bool) (spell : NounPhrase) (says : Instruction)

  inductive CopyExcept where
    | types (types : List CardType) (subtypes : List Subtype)
    | name (name : String)
    /-- [CR#707.9d] "In addition" retains only type characteristics. -/
    | chars (characteristics : Characteristics) (typesAdded : Bool)
    | ability (ability : Ability)
    | thisAbility
    | pt (power toughness : Amount)
    | nonlegendary
    | color (color : Color)
    | entersWithCounters (amount : Amount) (kind : CounterKind) (mark : EntryCounterMark)

  structure RollRow where
    results : Quantity
    instruction : Instruction

  inductive Instruction where
    | dealDamage (source : NounPhrase) (amount : Amount) (recipient : NounPhrase)
    | fights (left right : NounPhrase)
    | setStatus (status : Status) (subject : NounPhrase)
    | turnOver (subject : NounPhrase)
    | removeFromCombat (subject : NounPhrase)
    | attachTo (subject : NounPhrase) (host : NounPhrase)
    | unattach (subject : NounPhrase)
    | becomesBlocking (subject : NounPhrase) (blocked : NounPhrase)
    | stopsBlocking (subject : NounPhrase) (blocked : NounPhrase)
    | becomesAttacking (subject : NounPhrase) (defender : Option NounPhrase)
    | regenerate (subject : NounPhrase)
    /-- "… can't be regenerated this turn": an instruction plus the deed it forbids. -/
    | cantBe (instruction : Instruction) (deed : Deed) (subject : NounPhrase)
    | gainsDesignation (subject : NounPhrase) (designation : DesignationLabel)
        (conferral : Conferral) (duration : Option Duration)
    | unlock (door : Door)
    | gameBecomes (designation : DesignationLabel)
    | concludes (verb : OutcomeVerb) (player : NounPhrase)
    | gameDrawn
    | restartsGame
    | separateIntoPiles (player : NounPhrase) (group : NounPhrase) (piles : Nat)
        (faces : List PileFace)
    /-- The `when` rider is the printed "as you activate this ability": the announcement's
    timing, not a condition on what may be chosen. -/
    | choose (first : Option NounPhrase) (by_ : Option NounPhrase) (chosen : NounPhrase)
        (disclosure : Disclosure) (when : Option Concurrent)
    | choicesRevealed (sort : HiddenSort)
    | vote (first : Option NounPhrase) (voters : NounPhrase) (disclosure : Disclosure)
        (ballot : Ballot)
    | move (subject : NounPhrase) (to : ZoneExpr) (riders : List TokenRider)
    | counterSpell (spell : NounPhrase)
    | copy (sort : CopySort) (agent : NounPhrase) (subject : NounPhrase) (times : Amount)
        (exceptions : List CopyExcept)
    | chooseNewTargets (subject : NounPhrase)
    | copyTargets (copy : NounPhrase) (original : NounPhrase)
    | changeLife (player : NounPhrase) (delta : Delta Amount)
    | exchange (exchanged : Exchanged)
    | addMana (player : NounPhrase) (amount : Amount) (produced : ProducedMana)
        (riders : List ManaRider)
    | draw (player : NounPhrase) (amount : Amount)
    | expose (verb : ExposeVerb) (player : NounPhrase) (exposed : Exposed)
    | search (player : NounPhrase) (scope : SearchScope) (quantity : Quantity)
        (predicate : Predicate)
    | shuffle (whose : NounPhrase)
    | flipCoins (player : NounPhrase) (count : FlipScope)
    | rollDice (player : NounPhrase) (count : Amount) (sides : DieSides)
    | resultsTable (rows : List RollRow)
    | ignoreOutcomes (which : IgnoredOutcomes)
    | shiftResult (direction : Option ShiftDir) (amount : Amount)
    | storeResults (on : NounPhrase)
    | rerollStored (player : NounPhrase) (quantity : Quantity) (whose : NounPhrase)
    | continuously (spec : StaticSpec) (duration : Option Duration)
    | create (agent : NounPhrase) (count : Amount) (token : TokenSpec) (riders : List TokenRider)
    | getsEmblem (player : NounPhrase) (abilities : List Ability)
    | putCounters (amount : Amount) (kind : CounterKindSource) (on : NounPhrase)
    | distribute (verb : DividedVerb) (amount : Amount) (among : NounPhrase)
    | removeCounters (quantity : Option Quantity) (kind : Option CounterKindSource)
        (from_ : NounPhrase)
    | moveCounters (amount : Amount) (kind : Option CounterKindSource) (source : NounPhrase)
        (destination : NounPhrase)
    | doubleCounters (on : NounPhrase)
    | losesCounters (player : NounPhrase) (kind : Option CounterKindSource) (amount : Option Amount)
    /-- A named deed done: the deed's own facts row gates the agent, the patient and the zones
    the sentence may name, whichever of the three sources defines it. -/
    | enact (agent : Option NounPhrase) (verb : Deed) (instruction : Instruction)
    | controllerSacrifices (subject : NounPhrase)
    | pay (player : NounPhrase) (cost : Cost) (times : PayTimes)
    | may (player : NounPhrase) (body : Instruction) (ifDid : Option Instruction)
        (ifNot : Option Instruction)
    | ifDone (body : Instruction) (ifDid : Option Instruction) (ifNot : Option Instruction)
    | onlyIf (instruction : Instruction) (condition : Condition) (otherwise : Option Instruction)
    | if_ (condition : Condition) (instruction : Instruction) (otherwise : Option Instruction)
    | define (letter : Letter) (amount : Amount)
    | forEachOf (group : NounPhrase) (body : Instruction)
    | forEachKindOf (axis : KindAxis) (domain : Option NounPhrase) (sort : QualitySort)
        (body : Instruction)
    | repeat_ (repetition : Repetition)
    | repeated (times : Amount) (body : Instruction)
    | sequentially (steps : List Instruction)
    | simultaneously (steps : List Instruction)
    | modal (quantity : Quantity) (modes : List (Option Cost × Instruction))
    | delayed (event : GameEvent) (alternatives : List GameEvent) (duration : Option Duration)
        (body : Instruction)
    | insteadOf (replaced : Instruction) (replacement : Instruction)
    | heldUntil (instruction : Instruction) (event : GameEvent)
    | reflexively (body : Instruction) (trigger : Instruction)
    | thisWay (body : Instruction) (event : GameEvent) (trigger : Instruction)
    | doesntUntapNext (subject : NounPhrase) (steps : Amount)
    | skipsNext (player : NounPhrase) (part : TurnPart) (count : Amount)
    | extraTurn (player : NounPhrase) (count : Amount)
    | additionalPart (player : Option NounPhrase) (part : TurnPart) (anchor : Option TurnPart)
        (count : Amount) (followedBy : Option TurnPart)

  inductive KeywordParam where
    | cost (cost : Cost)
    | quality (predicate : Predicate)
    | subject (predicate : Predicate)
    | number (amount : Amount)
    | qualityCost (predicate : Predicate) (cost : Cost)
    | numberCost (amount : Amount) (cost : Cost)
    | deckCondition (condition : DeckCondition)

  inductive AbilityLost where
    | written (ability : Ability)
    | term (keyword : KeywordTerm)

  inductive Ability where
    | keyword (keyword : KeywordLabel) (param : Option KeywordParam) (body : Option Ability)
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

deriving instance Repr, BEq for Characteristics, CharacteristicBundle, QualityPayload,
  TokenSpec, StaticSpec, Compulsion, PlayPayment, DeonticRider, DamageOp, TokenRider, Cost,
  ManaRider, CopyExcept, RollRow, Instruction, KeywordParam, AbilityLost, Ability

end Semantics
