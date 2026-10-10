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
  deriving Repr, BEq

/-- A word a Text Change replaces where it is used in its rules sense: a color word or a basic
land type word [CR#612.2]. -/
inductive TextWord where
  | color (color : Color)
  | basicLandType (subtype : Subtype)
  deriving Repr, BEq

/-- A Text Change: a specified rewrite of the subject's rules text, applied in layer 3
[CR#612.1,613.1c]. Each row names the rewrite; the result is not checked again. -/
inductive TextChange where
  /-- Removes every `bracketed` term from the subject's rules text, the brackets with their
  contents; the rest of the text is unchanged (cleave [CR#702.148a]). A text with no
  brackets is left as it was. -/
  | removeBracketed
  /-- Replaces every `target` determiner in the subject's rules text with `each`, keeping its
  description; nothing else changes (overload [CR#702.96a,702.96b]). -/
  | targetToEach
  /-- Appends the rules text of `source` after the subject's own, which it neither modifies nor
  replaces (splice [CR#702.47a,702.47c,612.10]). -/
  | appendTextOf (source : NounPhrase)
  /-- Replaces every instance of the word `from_`, used in its rules sense, with the word `to`
  of the same kind, the kind one of `sorts`; `none` is a word of the performer's choice, made as
  the effect applies [CR#612.2] ("one color word with another or one basic land type with
  another"). -/
  | replaceWord (sorts : List QualitySort) (from_ : Option TextWord) (to : Option TextWord)
  /-- Exchanges the rules text of the subject and `other`: each one's rules text becomes the
  other's previous rules text [CR#612.5,701.12h]. -/
  | exchangeWith (other : NounPhrase)
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

/-- A mana cost a cost shift raises or lowers by: written ("{W}{B}"), or an object's mana cost
("the sacrificed permanent's mana cost" [CR#702.48a]). -/
inductive ManaCostTerm where
  | lit (cost : ManaCost)
  | manaCostOf (object : NounPhrase)
  deriving Repr, BEq

inductive CostShift where
  | less (amount : Amount) (floor : Option Amount)
  | more (amount : Amount)
  /-- Raise (`rises`) or lower the cost by the symbols of `cost`, each symbol of a type changing
  mana of that type. `coloredOnly` is "this effect reduces only the amount of colored mana you
  pay" (Edgewalker): a reduction that leaves generic mana alone; without it, colored mana
  in `cost` beyond what the total cost has reduces generic mana [CR#702.48c,118.7]. -/
  | bySymbols (cost : ManaCostTerm) (rises : Bool) (coloredOnly : Bool)
  deriving Repr, BEq

/-- The mana one act of a Mana Substitute pays for: one generic mana, or one mana of a color
[CR#702.51a,702.66a,702.126a,701.67a]. -/
inductive PipPaid where
  | generic
  | ofColor (color : ColorTerm)
  deriving Repr, BEq

structure DeedComplement where
  deed : Deed
  counterpart : NounPhrase
  deriving Repr, BEq

/-- A Deontic Exception: the one source a "can't" exempts. `byKeyword` names a keyword ability
of the subject ("can't transform except due to its daybound ability" [CR#702.145b]);
`byObjects` describes the objects that are exempt ("can't be blocked except by artifact
creatures and/or black creatures" [CR#702.36b]). -/
inductive Cause where
  | byKeyword (label : KeywordLabel)
  | byObjects (predicate : Predicate)
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

/-- Optional payment is decided by its performer, the actor in context; required payment tests
whether it started [CR#118.12]. -/
inductive ContinuationPolicy where
  | optional
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
  /-- "Each nonland card in your starting deck shares a card type." and "… has a different
  name.": an Agreement (`NounPhrase.agree`) over the starting deck's cards `group` describes. -/
  | groupAgrees (group : NounPhrase)
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

  /-- The one face-down spec. `none` is a card exiled face down, which has no characteristics
  and no status [CR#406.3a,110.5d]; `some c` is a face-down spell or permanent, `c` its listed
  characteristics and so its copiable values [CR#708.2,613.2b]. Turning it face up is
  `setStatus faceUp`, and the values revert [CR#708.8]. Nothing face down is a token. -/
  structure FaceDown where
    characteristics : Option Characteristics

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
    /-- The abilities printed below `keyword`'s line, other than that keyword's own: backup's
    "the non-backup abilities of this creature printed below this one". Only printed abilities
    are selected, never gained ones [CR#702.165a,702.165c]. -/
    | printedAfter (keyword : KeywordLabel)

  inductive TokenSpec where
    /-- The creating spell or ability defines the token's characteristic values [CR#111.3] and
    sets its name and subtypes [CR#111.4]. -/
    | written (characteristics : CharacteristicBundle)
    | asThose
    | copyOf (source : NounPhrase) (exceptions : List CopyExcept)
    /-- A double-faced token: its front face and its back face [CR#111.10i,701.53b]. -/
    | doubleFaced (front : CharacteristicBundle) (back : CharacteristicBundle)

  inductive StaticSpec where
    | withBindings (scope : Nat) (inputs : List CaptureInput) (body : StaticSpec)
    | inCaller (scope : Nat) (body : StaticSpec)
    | modification (subject : NounPhrase) (stat : Stat) (delta : Delta Amount)
    /-- "<subject> gets +P/+T" and "<subject> has base power and toughness P/T": one change
    to power and toughness together, its subject written once. Each delta is the one a
    `modification` of that stat would carry: a set applies in layer 7b, a change by an amount
    in layer 7c [CR#613.4b,613.4c]. -/
    | ptModification (subject : NounPhrase) (power : Delta Amount) (toughness : Delta Amount)
    | ptDefinition (subject : NounPhrase) (slots : DefinedSlots) (amount : Amount)
    | ptSwitch (subject : NounPhrase)
    | costShift (subject : NounPhrase) (shift : CostShift)
    | altCost (subject : NounPhrase) (cost : Option Cost)
    /-- An additional cost to cast the spell [CR#601.2f]: `offered` when the player may decline
    it ("you may pay"), and `times` how often it may be paid, as squad's and replicate's "any
    number of times" [CR#702.157a,702.56a]. -/
    | addedCost (cost : Cost) (offered : Bool) (times : PayTimes)
    | letterDefinition (letter : Letter) (amount : Amount)
    | abilityGrant (subject : NounPhrase) (ability : Ability)
    | abilityGrantFrom (subject : NounPhrase) (classes : List AbilityClass) (source : NounPhrase)
        (except : Option Predicate)
    /-- "<subject> has <selection> of <source>": the abilities of `source` the selection picks
    (backup's "it gains the non-backup abilities of this creature printed below this one"
    [CR#702.165a]). -/
    | abilityGrantSelected (subject : NounPhrase) (source : NounPhrase)
        (selection : AbilitySelection)
    | deonticRule (subject : NounPhrase) (compulsion : Compulsion) (deeds : Deeds) (role : Role)
        (bound : Option CountBound) (patient : DeonticPatient) (asThough : Option AsThough)
        (rider : DeonticRider) (except : Option Cause)
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
    | entryRider (subject : NounPhrase) (rider : EntryRider)
    | choice (occasion : ChoiceOccasion) (subject : NounPhrase) (sort : ChoiceSort)
        (domain : Option ChoiceDomain) (disclosure : Disclosure)
    | conjunction (subject : Option NounPhrase) (parts : List StaticSpec)
    /-- A Mana Substitute on a spell (convoke, delve, improvise): neither an additional nor an
    alternative cost, applied once the spell's total cost is locked
    [CR#702.51b,702.66b,702.126b]. -/
    | paySubstitute (subject : NounPhrase) (substitute : ManaSubstitute)
    /-- A special action: any time you have priority, within `when`, you may pay `cost` and do
    `does`, without using the stack [CR#116.1,116.2c]. Morph's turning face up, foretell's and
    plot's exiling are each one [CR#116.2b,116.2h,116.2k]. -/
    | specialAction (when : Option Timing) (cost : Cost) (does : Instruction)
    /-- An Off-Stack Static, "as [event], do": as `event` happens, `does` is carried out without
    using the stack [CR#604.1,614.1c]. A delayed trigger it creates has this static's object as
    its source [CR#603.7a,603.7g]. -/
    | asEvent (event : GameEvent) (does : Instruction)
    /-- An Off-Stack Static, "any time [condition] holds, do": whenever `condition` holds, `does`
    is carried out without using the stack [CR#604.1,702.131b]. -/
    | whileHolds (condition : Condition) (does : Instruction)
    /-- "<subject> is a Combatant": the creature-like combat rules apply to it. Its power deals
    combat and fight damage, marked damage is compared with its toughness, it may be declared an
    attacker or a blocker, and the summoning-sickness rule applies to it
    [CR#120.3e,302.6,508.1a,509.1a,701.14a]. The role is a quality of a permanent, not an ability
    [CR#113.12], and the creature type definition confers it; whether the Combatant may take a
    given combat action is a separate deontic question. -/
    | combatant (subject : NounPhrase)
    /-- A Mode Change: modifies how the modes of the spell or ability it is printed with are
    chosen [CR#700.2,702.42a,702.120a]. -/
    | modeChoice (change : ModeChange)
    /-- The spell gains a target, `description` saying what it may target, when its alternative
    cost is chosen: mutate's "targets a non-Human creature with the same owner as this spell"
    [CR#702.140a,601.2c]. -/
    | addsTarget (subject : NounPhrase) (description : Predicate)
    /-- A Text Change of the subject's rules text, applied in layer 3 [CR#612.1,613.1c]. -/
    | textChange (subject : NounPhrase) (change : TextChange)

  /-- How a Mode Change modifies its own spell's `chooseModes`. -/
  inductive ModeChange where
    /-- "You may choose all modes instead": for `cost` (entwine [CR#702.42a]), or when
    `condition` holds ("if COND, you may choose both instead"). -/
    | allInstead (cost : Option Cost) (condition : Option Condition)
    /-- "For each mode you choose beyond the first, you pay an additional [cost]" (escalate
    [CR#702.120a]). -/
    | extraModesCost (cost : Cost)
    /-- The number of modes chosen: spree's one or more, tiered's one [CR#702.172a,702.183a]. -/
    | setQuantity (quantity : Quantity)
    /-- "You may choose the same mode more than once" [CR#700.2d]. -/
    | repeatable
    /-- "Choose one that hasn't been chosen [within]". -/
    | notChosenBefore (within : Duration)

  inductive Compulsion where
    | forbid
    | require
    | gatedBy (cost : Cost)
    | permit

  inductive PlayPayment where
    | itsOwnCost
    | withoutPaying
    | payingInstead (cost : Cost)
    /-- Cast face down as `spec` says, paying `cost` rather than the mana cost: morph and
    disguise [CR#702.37a,702.168a,708.4]. -/
    | faceDown (spec : FaceDown) (cost : Cost)

  inductive DeonticRider where
    | stateBased (cause : StateBasedCause)
    | noRider
    | play (from_ : Option ZoneExpr) (limit : Option UsageLimit) (timing : Option Timing)
        (exclusive : Bool) (payment : PlayPayment)

  inductive DamageOp where
    | prevent (cut : PreventCut) (also : Option Instruction)
    | redirect (cut : PreventCut) (to : NounPhrase)
    | scale (scale : DamageScale)

  /-- How a permanent arrives on the battlefield: from a move, a token's creation, or a static
  entry rider. -/
  inductive EntryRider where
    | entersAs (status : Status)
    | entersAttacking (defender : Option NounPhrase)
    | entersTransformed
    | entersMelded (into : String)
    | withCounters (amount : Amount) (kind : CounterKindSource) (mark : EntryCounterMark)
    | under (controller : NounPhrase)
    | asCopyOf (optional : Bool) (source : NounPhrase) (exceptions : List CopyExcept)
    /-- Enters face down as `spec` lists, and with no other characteristics while it stays face
    down [CR#708.2,708.3]. -/
    | faceDown (spec : FaceDown)

  /-- How a moved card is placed in a zone other than the battlefield: its order and depth in
  a pile, its face, and the counters it is exiled with. -/
  inductive PilePlacement where
    /-- "In any order", "in a random order": how the objects moved to one place are ordered
    there [CR#401.4]. -/
    | inOrder (arrangement : Arrangement)
    /-- "Nth from the top": that many objects in from the place's end [CR#401.7]. -/
    | nthFromEnd (ordinal : Ordinal)
    /-- "On the top or bottom": the performer may choose the place's other end instead. -/
    | orOtherEnd
    /-- "Exile it face down": `none`, a card with no characteristics [CR#406.3,406.3a]. -/
    | faceDown (spec : FaceDown)
    /-- "Exile it with N time counters on it" [CR#702.62a]. -/
    | withCounters (amount : Amount) (kind : CounterKindSource) (mark : EntryCounterMark)

  /-- A move's rider, of the kind its destination allows: an entry rider onto the battlefield,
  a placement anywhere else. -/
  inductive MoveRider where
    | entry (rider : EntryRider)
    | placement (placement : PilePlacement)

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
    /-- Pay `cost`, `substitute` standing in only for that cost's own generic mana
    (waterbend [CR#701.67b]). -/
    | withSubstitute (cost : Cost) (substitute : ManaSubstitute)

  /-- "For each [generic | colored] mana in the cost, you may <action> rather than pay that
  mana" [CR#702.51a,702.66a,702.126a,701.67a]: the action first, then the mana each act pays,
  read back from the acted-on object by anaphora ("that creature's color"). -/
  structure ManaSubstitute where
    action : Instruction
    pays : List PipPaid

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
    | token (spec : TokenSpec) (riders : List EntryRider)
    /-- An emblem, named where the rules name it ("an emblem named The Ring" [CR#701.54c]). -/
    | emblem (name : Option String) (abilities : List Ability)
  inductive Instruction where
    /-- Bind ordered typed inputs for the body, without an implicit whole-body precondition. -/
    | withBindings (scope : Nat) (inputs : List CaptureInput) (body : Instruction)
    | inCaller (scope : Nat) (body : Instruction)
    /-- A keyword action body read in its own Reference Scope: it sees the performer the
    innermost handoff names, its parameters (each written `inCaller scope`, read in the caller's
    view) and what it introduces itself, never the calling text's other mentions
    (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06). The `plugins_v2` loader writes it
    inside a keyword action's `enact` wrapper. -/
    | ownScope (scope : Nat) (body : Instruction)
    /-- Rules text printed in square brackets, kept in place so a Text Change can find it
    [CR#702.148a]: transparent, it means what `instruction` means. -/
    | bracketed (instruction : Instruction)
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
        (duration : Option Duration)
    | unlock (door : Door)
    | setGameDesignation (designation : DesignationLabel)
    | conclude (verb : OutcomeVerb)
    | drawGame
    | restartGame
    | separateIntoPiles (group : NounPhrase) (piles : Nat) (faces : List PileFace)
    /-- The `when` rider is the printed "as you activate this ability": the announcement's
    timing, not a condition on what may be chosen. -/
    | choose (first : Option NounPhrase) (chosen : NounPhrase) (disclosure : Disclosure)
        (when : Option Timing)
    | revealChoices (sort : HiddenSort)
    | vote (first : Option NounPhrase) (disclosure : Disclosure) (ballot : Ballot)
    | move (subject : NounPhrase) (from_ : ZoneExpr) (to : ZoneExpr) (riders : List MoveRider)
    /-- Turn `subject` face down as `spec` lists: manifest's first step [CR#701.40a,708.2]. -/
    | turnFaceDown (subject : NounPhrase) (spec : FaceDown)
    | copy (sort : CopySort) (subject : NounPhrase) (times : Amount) (exceptions : List CopyExcept)
    | chooseNewTargets (subject : NounPhrase)
    | copyTargets (copy : NounPhrase) (original : NounPhrase)
    | changeLife (delta : Delta Amount)
    | exchange (exchanged : Exchanged)
    | addMana (amount : Amount) (produced : ProducedMana) (riders : List ManaRider)
    | draw (amount : Amount)
    | expose (verb : ExposeVerb) (exposed : Exposed)
    | search (scope : SearchScope) (quantity : Quantity) (predicate : Predicate)
    /-- Randomize the objects in a pile so no player knows their order [CR#701.24a]. -/
    | shuffle (pile : Pile)
    | flipCoins (count : FlipScope)
    | rollDice (count : Amount) (sides : DieSides)
    | applyResultsTable (rows : List RollRow)
    | ignoreOutcomes (which : IgnoredOutcomes)
    | shiftResult (direction : Option ShiftDir) (amount : Amount)
    | storeResults (on : NounPhrase)
    | rerollStored (quantity : Quantity) (whose : NounPhrase)
    | establish (spec : StaticSpec) (duration : Option Duration)
    | createObject (count : Amount) (spec : CreationSpec)
    | putCounters (amount : Amount) (kind : CounterKindSource) (on : NounPhrase)
    | distribute (verb : DividedVerb) (amount : Amount) (among : NounPhrase)
    | removeCounters (quantity : Option Quantity) (kind : Option CounterKindSource)
        (from_ : NounPhrase)
    | moveCounters (amount : Amount) (kind : Option CounterKindSource) (source : NounPhrase)
        (destination : NounPhrase)
    | doubleCounters (on : NounPhrase)
    /-- A named deed done by the performer in context: the deed's own facts row gates that
    performer, the patient and the zones the sentence may name, whichever of the three sources
    defines it. -/
    | enact (verb : Deed) (instruction : Instruction)
    | pay (cost : Cost) (times : PayTimes)
    /-- The handoff: `performer`, a player or a permanent, performs `body` ("target player
    discards a card", "target creature explores" [CR#701.44a]). Inside the body,
    `NounPhrase.actor` is that performer; with a distributive performer ("each opponent") each
    member performs the whole body. -/
    | act (performer : NounPhrase) (body : Instruction)
    /-- Branch on the decision or start of payment, independently of resulting events. -/
    | withContinuation (policy : ContinuationPolicy) (body : Instruction)
        (ifDid : Option Instruction) (ifNot : Option Instruction)
    | doOnlyIf (instruction : Instruction) (condition : Condition) (otherwise : Option Instruction)
    /-- "<instruction>. Do this only once each turn.": the instruction is limited, not the ability
    that carries it; the ability still triggers, and is done only within `limit` [CR#603.2h]. -/
    | limited (instruction : Instruction) (limit : UsageLimit)
    | doIf (condition : Condition) (instruction : Instruction) (otherwise : Option Instruction)
    | doForEach (group : NounPhrase) (body : Instruction)
    | doForEachKind (axis : KindAxis) (domain : Option NounPhrase) (sort : QualitySort)
        (body : Instruction)
    | repeat_ (repetition : Repetition)
    | sequentially (steps : List Instruction)
    | simultaneously (steps : List Instruction)
    /-- "Choose <quantity> —" over bulleted modes [CR#700.2]: each mode is its cost (spree's and
    tiered's priced bullets [CR#700.2h]), its weight (a pawprint mode's count of {P}
    [CR#700.2i]) and its instruction. -/
    | chooseModes (quantity : Quantity) (modes : List (Option Cost × Option Nat × Instruction))
    | delay (event : GameEvent) (alternatives : List GameEvent) (duration : Option Duration)
        (body : Instruction)
    | replace (replaced : Instruction) (replacement : Instruction)
    | holdUntil (instruction : Instruction) (event : GameEvent)
    | triggerReflexively (body : Instruction) (trigger : Instruction)
    | triggerThisWay (body : Instruction) (event : GameEvent) (trigger : Instruction)
    | skipUntap (subject : NounPhrase) (steps : Amount)
    | skipPart (part : TurnPart) (count : Amount)
    | addTurnPart (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
        (followedBy : Option TurnPart)
    /-- Carry out a rules procedure on `subject` now, during resolution: activate it, cast it,
    or play it [CR#701.2a,701.5a,701.18a], paying as `payment` says. `resulting` is what the
    spell the procedure produces must be ("if the resulting spell's mana value is N or less"
    [CR#701.57a]). -/
    | procedure (kind : Procedure) (subject : NounPhrase) (payment : PlayPayment)
        (resulting : Option Predicate)
    /-- Make `subject` stand in the Relation `label` with `counterpart`: "exile it haunting target
    creature", "pair this creature with another", "encoded on a creature"
    [CR#702.55b,702.95b,702.99b]. -/
    | relate (subject : NounPhrase) (label : RelationLabel) (counterpart : NounPhrase)
    /-- Merge `subject` with the permanent `onto`, placed at `position`: the permanent becomes
    a merged permanent represented by both [CR#730.2]. -/
    | merge (subject : NounPhrase) (onto : NounPhrase) (position : MergePosition)
  inductive KeywordParam where
    | cost (cost : Cost)
    /-- A description printed after a marker or bound to the keyword: "protection from red",
    "islandwalk" [CR#702.16a,702.14a]. A declaration writes it `Quality`. -/
    | quality (predicate : Predicate)
    /-- A description printed as a nominal: "Enchant creature you control", "Champion a Kithkin"
    [CR#702.5a,702.72a]. It is a description, not a referent, so a declaration writes it
    `Predicate`, never `Subject` (a `NounPhrase`). -/
    | subject (predicate : Predicate)
    | number (amount : Amount)
    | deckCondition (condition : DeckCondition)
    /-- The ability printed with the keyword ("Boast — [Cost]: [Effect]"), which the keyword's
    definition restricts (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-09). -/
    | ability (ability : Ability)

  inductive AbilityLost where
    | written (ability : Ability)
    | term (keyword : KeywordTerm)

  inductive Ability where
    /-- A keyword ability [CR#702.1]. `body` is the keyword's definition, held the way a text
    box holds rules text [CR#109.3]: empty where the card writes the keyword bare, and otherwise
    the abilities the keyword represents. -/
    | keyword (keyword : KeywordLabel) (params : List KeywordParam) (body : List Ability)
    | activated (cost : Cost) (instruction : Instruction) (timing : Option Timing)
        (limit : Option UsageLimit) (guard : Option Condition) (activator : Option NounPhrase)
    | triggered (event : GameEvent) (alternatives : List GameEvent)
        (while_ : Option Timing) (joins : List JoinedHeader) (timing : Option Timing)
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
    /-- `ability` with the restrictions a keyword or a granting effect adds to it: when it may be
    activated or trigger (`timing`), how often (`limit`), the condition it is subject to
    (`guard`) and the zone it functions from (`from_`) [CR#113.10a,113.6b]. The guard reads by
    the wrapped ability's category: "as long as" for a static ability, "this ability triggers
    only if" for a triggered one, "activate only if" for an activated one
    [CR#702.169b..702.169d]. -/
    | qualified (ability : Ability) (timing : Option Timing) (limit : Option UsageLimit)
        (guard : Option Condition) (from_ : Option Zone)
end

deriving instance Repr, BEq for Characteristics, CharacteristicBundle, FaceDown, CharacteristicEdit,
  AbilitySelection, TokenSpec, StaticSpec, ModeChange, Compulsion, PlayPayment, DeonticRider, DamageOp, EntryRider,
  PilePlacement, MoveRider, Cost,
  ManaSubstitute, ManaRider, CopyExcept, RollRow, Repetition, CreationSpec, Instruction, KeywordParam,
  AbilityLost, Ability

end Semantics

attribute [semantic_expression] Semantics.Instruction Semantics.StaticSpec Semantics.Cost Semantics.Ability Semantics.TokenSpec

classify_semantic_syntax

attribute [internal_expansion] Semantics.Instruction.enact Semantics.Instruction.withBindings
  Semantics.Instruction.inCaller Semantics.Instruction.ownScope Semantics.Cost.withBindings Semantics.Cost.inCaller
  Semantics.StaticSpec.withBindings Semantics.StaticSpec.inCaller
