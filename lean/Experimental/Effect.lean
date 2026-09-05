import Experimental.Triggers

/-!
# Experimental.Effect

Instructions, static specifications, costs, tokens, and abilities. Port of
`idris/src/Experimental/Effect.idr`, syntax only.

The Idris `Instructions n bs`, `SimInstructions n bs`, `StaticParts n bs`, `CostSeq n bs`
and `AbilitySeq bs` telescopes are plain lists here. Dependent pairs that existed only for
the index (`pt : Maybe (p : Amount bs ** Amount (amtIntro p))`) are products.

Not ported (checker machinery and witnesses): `InstrProfile`, `DivTag`, `DividedTakes`,
`CtrlOverrideOk`, `EncloseUse`, `EachStackOk`, `DeckReadable`, `DeckComparable`,
`AddedPayment`, and the four telescopes above.

Renames: `Cost.Do` → `Cost.action`, `Instruction.If` → `ifThen`, `Instruction.Repeat` →
`repeatProcess` (keywords); a field named `from` is `from_`, `by` is `by_`, `as` is `as_`,
`while` is `while_`.
-/

namespace Mtg

/-- What mana may be spent on. -/
inductive SpendPurpose where
  | toCast (p : Predicate)
  | toActivate (src : Option Predicate)
  | toPay (c : CostNamed)
  deriving Repr, BEq

inductive ManaHeld where
  | thisMana
  | unspent (ty : Option ColorOrColorless)
  deriving Repr, BEq

inductive AsThough where
  | of (p : Predicate)
  | mana (what : Option ColorOrColorless) (as_ : ManaMatch) (purpose : Option SpendPurpose)
  | greater (stat : Stat) (amt : Amount)
  deriving Repr, BEq

inductive Exchanged where
  | lifeTotals (parties : Noun)
  | controlOf (a b : Noun)
  | cardsAcross (a b : Noun)
  | zones (a b : ZoneExpr)
  deriving Repr, BEq

inductive TokenQuality where
  | withEveryType (space : TypeSpace)
  | withQuality (q : Predicate)
  deriving Repr, BEq

/-- Where a counter's kind comes from. -/
inductive CounterKindSource where
  | printed (kind : CounterKind)
  | chosen (menu : List CounterKind)
  | distinctChosen (menu : List CounterKind)
  | bound
  | those
  | own
  | sameAs (src : Noun)
  deriving Repr, BEq

inductive QualityOp where
  | adds | sets | loses
  deriving DecidableEq, Repr

inductive CostShift where
  | less (amt : Amount) (floor : Option Amount)
  | more (amt : Amount)
  | run (run : ManaCost) (rises : Bool) (coloredOnly : Bool)
  deriving Repr, BEq

inductive CountBound where
  | moreThan (k : Amount)
  | additional (q : Quantity)
  deriving Repr, BEq

structure DeedComplement where
  deed : VerbLabel
  m : Noun
  deriving Repr, BEq

/-- Whom a deontic rule's deed is done to. The Idris indexes this by `Deeds` and `Role`. -/
inductive DeonticPatient where
  | noPatient
  | defendingPlayer (m : Noun)
  | counterpart (m : Noun)
  | targetedBy (m : Noun)
  | counterpartsAt (cs : List DeedComplement)
  deriving Repr, BEq

inductive DamageScope where
  | everywhere
  | toRecipient (n : Noun)
  deriving Repr, BEq

inductive DamageAgent where
  | unattributed
  | dealtBy (n : Noun)
  deriving Repr, BEq

inductive Unpreventable where
  | described (src : DamageAgent) (scope : DamageScope)
  | thatDamage
  deriving Repr, BEq

inductive PreventionBan where
  | noPreventionOnly | noRedirectEither
  deriving DecidableEq, Repr

inductive PreventCut where
  | all
  | some (amt : Amount)
  | shield (amt : Amount)
  | allBut (amt : Amount)
  | half (rounding : RoundMode)
  deriving Repr, BEq

inductive DamageScale where
  | multiplied (f : ScaleFactor)
  | halved (rounding : RoundMode)
  | shifted (dir : ShiftDir) (amt : Amount)
  deriving Repr, BEq

inductive DividedVerb where
  | damage (src : Noun)
  | counters (kind : CounterKind)
  deriving Repr, BEq

inductive ProducedMana where
  | runs (rs : List ProducedRun)
  | anyColor (freedom : ColorFreedom)
  | ofChosenColor (alt : Option ProducedRun)
  | asPrintedCost (n : Noun)
  | producedByEvent (n : Noun)
  | couldProduce (n : Noun)
  | amongColorsOf (n : Noun)
  | amongWritten (cs : List Color)
  | lastNoted (n : Noun)
  deriving Repr, BEq

inductive SpentMode where
  | affectsIt | triggersThen
  deriving DecidableEq, Repr

inductive Repetition where
  | again
  | moreTimes (n : Amount)
  | anyNumber
  | untilCond (c : Condition)
  | againExcludingChosen
  deriving Repr, BEq

inductive ManaParity where
  | even | odd
  deriving DecidableEq, Repr

inductive DeckTrait where
  | aCharacteristic (p : Predicate)
  /-- "cards with even mana values" [CR#202.3] -/
  | manaValueParity (parity : ManaParity)
  /-- "more than one of the same mana symbol in its mana cost" -/
  | repeatedManaSymbol
  /-- "has an activated ability" -/
  | hasAbilityOf (cls : AbilityClass)
  /-- "... and land cards" -/
  | anyOf (ts : List DeckTrait)
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
  structure TokenChars where
    pt : Option (Amount × Amount)
    colors : List Color
    typeLine : TypeLine
    abilities : List AbilityAt
    name : Option String
    quals : List TokenQuality

  inductive QualityPayload where
    | bundle (t : TokenChars) (ret : Option CardType)
    | everyTypeOf (space : TypeSpace)
    | chosenQuality (q : Predicate)
    | colored (cs : ColorSpec)

  inductive TokenSpec where
    /-- The creating spell or ability defines the token's characteristic values [CR#111.3] and
    sets its name and subtypes [CR#111.4]. -/
    | written (t : TokenChars)
    | asThose
    | copyOf (src : Noun) (exc : List CopyExcept)

  inductive StaticSpec where
    | modify (n : Noun) (what : Stat) (d : Delta Amount)
    | definesPt (n : Noun) (slots : DefinedSlots) (amt : Amount)
    | switchesPt (n : Noun)
    | costs (n : Noun) (shift : CostShift)
    | altCost (n : Noun) (c : Option Cost)
    | addedCost (c : Cost) (offered : Bool)
    | definesLetter (l : Letter) (amt : Amount)
    | gains (n : Noun) (ab : AbilityAt)
    | gainsAbilitiesOf (n : Noun) (cls : List AbilityClass) (src : Noun)
        (except : Option Predicate)
    | deontic (n : Noun) (c : Compulsion) (deeds : Deeds) (role : Role)
        (bound : Option CountBound) (patient : DeonticPatient) (asThough : Option AsThough)
        (rider : DeonticRider)
    | keepsUnspentMana (who : Noun) (what : ManaHeld)
    | skips (who : Noun) (part : TurnPart)
    | becomes (n : Noun) (op : QualityOp) (q : QualityPayload)
    | alsoOffBattlefield (se : StaticSpec)
    | doesntRemove (se : StaticSpec) (n : Noun)
    | becomesCopy (n : Noun) (src : Noun) (exc : List CopyExcept)
    | losesAllAbilities (n : Noun) (except : Option Predicate)
    | losesAbilities (n : Noun) (abl : List AbilityLost)
    | gainsControl (who : Noun) (what : Noun)
    | intercepts (ev : GameEvent) (alts : List GameEvent) (window : Option TriggerWindow)
        (repl : Instruction) (use : ReplUse) (limit : Option UsageLimit)
    | damageRule (kind : DamageKind) (src : DamageAgent) (scope : DamageScope) (op : DamageOp)
        (use : ReplUse)
    | cantPrevent (kind : DamageKind) (what : Unpreventable) (ban : PreventionBan)
    | conditionally (se : StaticSpec) (c : Condition) (marking : CondMarking)
    | onlyDuring (p : TurnPart) (w : Option Noun) (se : StaticSpec)
    /-- "doesn't lose the game for having 0 or less life" [CR#704.5a] -/
    | noLossFromZeroLife (who : Noun)
    | visibility (v : ExposeVerb) (who : Noun) (what : VisibleThing)
    | triggersAdditionally (ev : GameEvent) (q : Quantity)
    | entersRider (n : Noun) (rider : TokenRider)
    | entersChoice (n : Noun) (q : ChoiceSort) (domain : Option ChoiceDomain)
        (disclosure : Disclosure)
    | attachChoice (n : Noun) (q : ChoiceSort) (domain : Option ChoiceDomain)
    | andAlso (subject : Option Noun) (parts : List StaticSpec)

  inductive Compulsion where
    | forbid
    | require
    | gatedBy (c : Cost)
    | permit

  inductive PlayPayment where
    | itsOwnCost
    | withoutPaying
    | payingInstead (c : Cost)

  inductive DeonticRider where
    | noRider
    | play (from_ : Option ZoneExpr) (limit : Option PlayLimit) (window : Option PlayWindow)
        (exclusive : Bool) (payment : PlayPayment)

  inductive DamageOp where
    | prevent (cut : PreventCut) (also : Option Instruction)
    | redirect (cut : PreventCut) (to : Noun)
    | scale (sc : DamageScale)

  inductive TokenRider where
    | entersAs (v : Status)
    | entersAttacking (whom : AttackDefender)
    | entersTransformed
    | entersMelded (into : String)
    | withCounters (amt : Amount) (kind : CounterKindSource) (mark : EntryCounterMark)
    | under (who : Noun)
    | asCopyOf (optional : Bool) (src : Noun) (exc : List CopyExcept)

  inductive Cost where
    | mana (c : ManaCost)
    | scaled (c : Cost) (amt : Amount)
    | tapSymbol
    | untapSymbol
    | loyaltySymbol (s : LoyaltyCost)
    /-- Idris `Do`: an instruction as a cost. -/
    | action (e : Instruction)
    | compound (cs : List Cost)
    | either (l r : Cost)
    | itsManaCost

  inductive ManaRider where
    | spendOnly (ps : List SpendPurpose)
    | spendNotOn (ps : List SpendPurpose)
    | onSpent (mode : SpentMode) (only : Bool) (what : Noun) (says : Instruction)

  inductive CopyExcept where
    | types (added : TypeLine)
    | name (nm : String)
    /-- [CR#707.9d] "In addition" retains only type characteristics. -/
    | chars (t : TokenChars) (typesAdded : Bool)
    | ability (ab : AbilityAt)
    | thisAbility
    | pt (power toughness : Amount)
    | nonlegendary
    | color (c : Color)
    | entersWithCounters (amt : Amount) (kind : CounterKind) (mark : EntryCounterMark)

  structure RollRow where
    results : Quantity
    instruction : Instruction

  inductive Instruction where
    | dealDamage (src : Noun) (amt : Amount) (to : Noun)
    | fights (a b : Noun)
    | setStatus (v : Status) (n : Noun)
    | turnOver (what : Noun)
    | removeFromCombat (n : Noun)
    | attachTo (what : Noun) (host : Noun)
    | unattach (what : Noun)
    | becomesBlocking (n : Noun) (what : Noun)
    | stopsBlocking (n : Noun) (what : Noun)
    | becomesAttacking (n : Noun) (whom : AttackDefender)
    | regenerate (n : Noun)
    | cantBe (e : Instruction) (deed : VerbLabel) (what : Noun)
    | gainsDesignation (n : Noun) (d : Designation) (w : GivingWarrant) (span : Option Duration)
    | unlock (door : Door)
    | gameBecomes (d : Designation)
    | concludes (v : OutcomeVerb) (who : Noun)
    | gameDrawn
    | restartsGame
    | separateIntoPiles (who : Noun) (group : Noun) (piles : Nat) (faces : List PileFace)
    | choose (first : Option Noun) (by_ : Option Noun) (n : Noun) (disclosure : Disclosure)
    | choicesRevealed (s : HiddenSort)
    | vote (first : Option Noun) (voters : Noun) (disclosure : Disclosure) (ballot : Ballot)
    | move (what : Noun) (to : ZoneExpr) (riders : List TokenRider)
    | counterSpell (what : Noun)
    | copy (src : CopySort) (agent : Noun) (what : Noun) (times : Amount)
        (exc : List CopyExcept)
    | chooseNewTargets (what : Noun)
    | copyTargets (copy : Noun) (whom : Noun)
    | changeLife (who : Noun) (d : Delta Amount)
    | exchange (what : Exchanged)
    | addMana (who : Noun) (amt : Amount) (prod : ProducedMana) (riders : List ManaRider)
    | draw (who : Noun) (amt : Amount)
    | expose (v : ExposeVerb) (who : Noun) (what : Exposed)
    | search (who : Noun) (scope : SearchScope) (q : Quantity) (p : Predicate)
    | shuffle (whose : Noun)
    | flipCoins (who : Noun) (count : FlipScope)
    | rollDice (who : Noun) (count : Amount) (sides : DieSides)
    | resultsTable (rows : List RollRow)
    | ignoreOutcomes (which : IgnoredOutcomes)
    | shiftResult (dir : Option ShiftDir) (amt : Amount)
    | rollPlanarDie (who : Noun) (count : Amount)
    | chaosEnsues (what : Option Noun)
    | storeResults (on : Noun)
    | rerollStored (who : Noun) (q : Quantity) (whose : Noun)
    | continuously (se : StaticSpec) (span : Option Duration)
    | create (agent : Noun) (count : Amount) (spec : TokenSpec) (riders : List TokenRider)
    | getsEmblem (who : Noun) (abl : List AbilityAt)
    | putCounters (amt : Amount) (kind : CounterKindSource) (on : Noun)
    | distribute (v : DividedVerb) (amt : Amount) (among : Noun)
    | removeCounters (q : Option Quantity) (kind : Option CounterKindSource) (from_ : Noun)
    | moveCounters (amt : Amount) (kind : Option CounterKindSource) (src dst : Noun)
    | doubleCounters (on : Noun)
    | losesCounters (who : Noun) (kind : Option CounterKindSource) (amt : Option Amount)
    | enact (subject : Option Noun) (v : VerbLabel) (e : Instruction)
    | controllerSacrifices (n : Noun)
    | pay (who : Noun) (c : Cost) (times : PayTimes)
    | may (offer : Noun) (body : Instruction) (ifDid : Option Instruction)
        (ifNot : Option Instruction)
    | ifDone (body : Instruction) (ifDid : Option Instruction) (ifNot : Option Instruction)
    | onlyIf (e : Instruction) (c : Condition) (otherwise : Option Instruction)
    /-- Idris `If`: `if` is a keyword. -/
    | ifThen (c : Condition) (e : Instruction) (otherwise : Option Instruction)
    | define (l : Letter) (amt : Amount)
    | forEachOf (group : Noun) (body : Instruction)
    | forEachKindOf (axis : KindAxis) (domain : Option Noun) (q : QualitySort)
        (body : Instruction)
    /-- Idris `Repeat`: `repeat` is a tactic keyword. -/
    | repeatProcess (rep : Repetition)
    | repeated (n : Amount) (body : Instruction)
    | sequentially (es : List Instruction)
    | simultaneously (es : List Instruction)
    | modal (q : Quantity) (modes : List (Option Cost × Instruction))
    | delayed (ev : GameEvent) (alts : List GameEvent) (span : Option Duration)
        (body : Instruction)
    | insteadOf (replaced : Instruction) (repl : Instruction)
    | heldUntil (e : Instruction) (ev : GameEvent)
    | reflexively (body : Instruction) (trig : Instruction)
    | thisWay (body : Instruction) (ev : GameEvent) (trig : Instruction)
    | doesntUntapNext (n : Noun) (steps : Amount)
    | skipsNext (who : Noun) (part : TurnPart) (count : Amount)
    | extraTurn (who : Noun) (count : Amount)
    | additionalPart (who : Option Noun) (part : TurnPart) (anchor : Option TurnPart)
        (count : Amount) (followedBy : Option TurnPart)

  inductive KeywordParam where
    | cost (c : Cost)
    | quality (p : Predicate)
    | subject (p : Predicate)
    | number (amt : Amount)
    | qualityCost (p : Predicate) (cost : Cost)
    | numberCost (amt : Amount) (cost : Cost)
    | deckCondition (dc : DeckCondition)

  inductive AbilityLost where
    | written (ab : AbilityAt)
    | term (t : KeywordTerm)

  inductive AbilityAt where
    | keyword (k : KeywordLabel) (param : Option KeywordParam) (body : Option AbilityAt)
    | activated (cost : Cost) (instr : Instruction) (window : Option Timing)
        (limit : Option UsageLimit) (guard : Option Condition) (activator : Option Noun)
    | triggered (word : TriggerWord) (ev : GameEvent) (alts : List GameEvent)
        (while_ : Option Concurrent) (joins : List JoinedHeader) (window : Option TriggerWindow)
        (limit : Option UsageLimit) (intervening : Option Condition) (instr : Instruction)
    | static (se : StaticSpec)
    | spell (window : Option Timing) (instr : Instruction)
    | mayBeginOnBattlefield
    | alsoForKeywords (ab : AbilityAt) (ks : List KeywordTerm)
    | italicHead (word : ItalicWord) (ab : AbilityAt)
end

deriving instance Repr, BEq for TokenChars, QualityPayload, TokenSpec, StaticSpec, Compulsion,
  PlayPayment, DeonticRider, DamageOp, TokenRider, Cost, ManaRider, CopyExcept, RollRow,
  Instruction, KeywordParam, AbilityLost, AbilityAt

/-- Idris `AbilitySeq bs`: a card's text, one ability per line. -/
abbrev AbilitySeq := List AbilityAt

end Mtg
