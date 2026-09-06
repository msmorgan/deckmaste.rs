import Semantics.Words
import Semantics.Events

/-!
# Semantics.Phrase

The phrase grammar: zones, predicates, noun phrases, amounts, quantities, and conditions.
Port of `idris/src/Experimental/Phrase.idr`, syntax only, reshaped: no `bs` or `Kind`
indices (both are inferred by the checker), and no constructor whose meaning is a
composition of others (`permanent` is `inZone battlefield`, the combat predicates are one
constructor over `CombatRelation`, the two stat reads are one over `ProjAxis`).

A `NounPhrase` denotes one entity or a collection, by its number; CONTEXT.md's Reference and
Selection are what it denotes, not two spellings.
-/

namespace Semantics

/-- A color, written or "the chosen color". -/
inductive ColorTerm where
  | lit (color : Color)
  | chosen (ref : ChoiceRef)
  deriving DecidableEq, Repr

/-- The mana type a "tapped for mana of …" trigger specifies [CR#106.12a]: colorless, or a
color written or chosen; the six types of [CR#106.1b]. -/
inductive ManaTypeTerm where
  | colorless
  | ofColor (color : ColorTerm)
  deriving Repr, BEq

mutual
  /-- Whose zone: "your graveyard", or a bare zone name. -/
  inductive ZoneScope where
    | bare
    | possessedBy (possessor : NounPhrase)

  /-- Where in a library. -/
  inductive LibraryPlace where
    | oneEnd (end_ : LibraryEnd)
    | eitherEnd (chooser : Option NounPhrase)
    | shuffled

  inductive ZoneExpr where
    | zone (zone : Zone) (scope : ZoneScope)
    | library (place : LibraryPlace) (order : Option Arrangement) (offset : Option Ordinal)
        (scope : ZoneScope)

  /-- Where a name comes from: printed, chosen, or "with the same name as …". -/
  inductive NameSource where
    | printed (name : String)
    | chosen
    | sameAs (subject : NounPhrase)

  /-- What a choice ranges over. -/
  inductive ChoiceDomain where
    | nameOfCard (predicate : Predicate)
    | colorOtherThan (color : Color)
    | typeOtherThan (subtype : Subtype)
    | basicTypesOnly
    | nonbasicTypesOnly
    | number (quantity : Quantity)
    | players (predicate : Predicate)
    /-- "choose flying or trample": an ability chosen among named keywords. -/
    | abilitiesAmong (keywords : List KeywordTerm)

  inductive EventSource where
    | anywhere
    | zones (zones : List ZoneExpr)
    | anywhereBut (zones : List ZoneExpr)

  /-- A historical event whose gap denotes the entity being described. -/
  inductive LookbackClause where
    | mk (event : GameEvent) (lookback : Lookback)

  inductive Predicate where
    | hasType (type : CardType)
    | hasSubtype (subtype : Subtype)
    | hasSupertype (supertype : Supertype)
    | anyPlayer
    | opponent
    | chosenPlayer (ref : ChoiceRef)
    | qualityNoun (sort : QualitySort) (domain : Option ChoiceDomain)
    | counterKindOn (subject : NounPhrase)
    | ofChosen (ref : ChoiceRef) (sort : QualitySort)
    | ofYourChoice (sort : QualitySort) (domain : Option ChoiceDomain)
    | hasKeyword (keyword : KeywordTerm)
    | hasPossessor (axis : PossessorAxis) (possessor : NounPhrase)
    | castBy (caster : NounPhrase) (rank : Option (Ordinal × RankPeriod))
    | castFrom (zone : ZoneExpr)
    | wasCast
    | inCombat (relation : CombatRelation) (counterpart : Option NounPhrase)
    | happenedTo (lookback : LookbackClause)
    | colorIs (color : Color)
    | colorCount (comparator : Comparator) (count : Nat)
    | named (source : NameSource)
    | hasDesignation (designation : DesignationLabel) (holder : Option NounPhrase)
    | attachment (side : AttachmentSide) (word : Option AttachWord)
        (counterpart : Option NounPhrase)
    | isCard
    | isToken
    | isEmblem
    | isCopyOfACard
    /-- The whole single double-faced permanent has this side up [CR#701.27g]. -/
    | currentFace (side : CardFaceSide)
    | hasStatus (status : Status)
    | hasCounters (kind : Option CounterKind)
    | compare (axes : List ProjAxis) (comparator : Comparator) (bound : Amount)
    | superlative (op : AggregateOp) (axis : ProjAxis) (domain : Predicate)
    /-- A vote stands only where a spell or ability instructed players to vote [CR#701.38a]. -/
    | withMostVotes
    | choseExtreme (op : AggregateOp)
    | compareOver (domain : Predicate) (measure : Amount) (comparator : Comparator) (bound : Amount)
    | inZone (zone : ZoneExpr)
    | inPile (pile : NounPhrase)
    | exiledWith (source : NounPhrase)
    | and (conjuncts : List Predicate)
    /-- Disjuncts of different kinds join: "creature or player" denotes either. -/
    | or (disjuncts : List Predicate)
    | not (predicate : Predicate)
    | other
    | notChosen
    | otherThan (anchor : NounPhrase)
    | coinCameUp (face : CoinFace)
    | isSource
    | manaCostHas (symbol : ManaSymbol)
    | abilityHead (class_ : AbilityClass)
    | abilityOf (source : NounPhrase)
    | activatedBy (activator : NounPhrase)
    | isManaAbility
    | targets (subject : NounPhrase) (extent : TargetExtent)

  inductive DetPhrase where
    | target (quantity : Quantity)
    | a (mode : ChoiceMode)
    /-- "each …": a group, resolution-time [CR#608.2] -/
    | each
    | all
    | the
    | count (quantity : Quantity) (mode : Option ChoiceMode)
    /-- the bare plural: a description, no determiner [CR#109.2] -/
    | bare

  inductive NounPhrase where
    /-- The participant bound by the nearest lookback, viewed one entity at a time. -/
    | gap (kind : Kind)
    /-- the source, by self-name or "this spell" [CR#113.7] -/
    | this
    | asType (type : CardType) (subject : NounPhrase) (subtype : Option Subtype)
    | asMarker (marker : MarkerWord) (subject : NounPhrase)
    | resolvedPermanent (spell : NounPhrase)
    | theGrantor (marker : MarkerWord)
    | you
    | combatPlayer (role : CombatRole)
    | playerGroup (group : PlayerGroupWord)
    | described (determiner : DetPhrase) (predicate : Predicate)
    | eachOf (group : NounPhrase)
    | and (phrases : List NounPhrase)
    | or (phrases : List NounPhrase)
    | librarySlice (end_ : LibraryEnd) (amount : Amount) (whose : NounPhrase)
    | someOf (count : SliceCount) (description : Option Predicate) (group : NounPhrase)
    | namesAgree (agreement : NameAgreement) (group : NounPhrase)
    | theRest (kind : Kind) (plurality : Plurality)
    | pileOf (count : SliceCount) (by_ : Option NounPhrase)
    /-- A pronoun: what it reaches for, its number, and the window it resolves in. -/
    | pro (reach : Reach) (plurality : Plurality) (window : Window)
    | attachHost (word : AttachWord) (head : NounWord)
    | possessorOf (axis : PossessorAxis) (subject : NounPhrase)
    | designated (designation : DesignationLabel) (whose : NounPhrase)
    | oneEachOf (roles : List Predicate) (pool : NounPhrase)

  inductive Amount where
    | lit (value : Int)
    | statOf (axis : ProjAxis) (subject : NounPhrase)
    | countOf (group : NounPhrase)
    | aggregate (op : AggregateOp) (axis : ProjAxis) (group : NounPhrase)
    | paid (facet : PaidFacet) (subject : NounPhrase)
    | eventTally (op : TallyOp) (subject : NounPhrase) (lookback : LookbackClause)
    | thatMuch
    | chosenNumber (ref : ChoiceRef)
    | votesFor (label : VoteLabel)
    | theOutcome (sort : OutcomeSort)
    | coinsShowing (face : CoinFace)
    | greatestStoredMatch (subject : NounPhrase)
    | groupSize
    | theDifference
    | letter (letter : Letter)
    | arith (op : ArithOp) (left right : Amount)
    | devotion (player : NounPhrase) (color : ColorTerm) (second : Option ColorTerm)
    | half (rounding : RoundMode) (amount : Amount)
    | aggregateOver (op : AggregateOp) (domain : Predicate) (body : Amount)
    | distinctCount (axis : KindAxis) (domain : NounPhrase)
    | upTo (bound : Amount)

  inductive Quantity where
    | range (low high : Option Nat)
    | upToOf (amount : Amount)
    | exactlyOf (amount : Amount)

  inductive SliceCount where
    | counted (quantity : Quantity)
    | whole
  inductive Door where
    | thisDoor
    | doorOf (state : Option LockState) (room : NounPhrase)

  /-- Which roll results a trigger watches. -/
  inductive RollWatch where
    | anyResult
    | resultIn (quantity : Quantity)
    | highestNatural

  /-- Whose turn part a "beginning of" header names. -/
  inductive HeaderPossessor where
    | noPossessor
    | byPlayer (player : NounPhrase)
    | byTurn (turn : NounPhrase)


  inductive Condition where
    /-- "if there is a …" -/
    | exists_ (subject : NounPhrase)
    | happened (subject : NounPhrase) (lookback : LookbackClause)
    | gameIs (designation : DesignationLabel)
    | noHolder (designation : DesignationLabel)
    | matches (subject : NounPhrase) (predicate : Predicate)
    | compareAmt (subject : Amount) (comparator : Comparator) (bound : Amount)
    | dealtThisWay (predicate : Predicate)
    | choseThisWay (chooser : NounPhrase) (predicate : Predicate)
    | preventedFromSource (predicate : Predicate)
    | flipCalled (caller : NounPhrase) (call : FlipCall)
    | flipFace (face : CoinFace)
    | voteLead (label : VoteLabel) (orTied : Bool)
    | anyResultIs (comparator : Comparator) (bound : Amount)
    | rolledDoubles
    | not (condition : Condition)
    | and (conjuncts : List Condition)
    | or (disjuncts : List Condition)

  inductive GameEvent where
    | dies (subject : NounPhrase)
    | leaves (subject : NounPhrase) (from_ : Option EventSource)
    | isDealtDamage (kind : DamageKind) (subject : NounPhrase)
    | draws (player : NounPhrase)
    | losesGame (player : NounPhrase)
    | enters (subject : NounPhrase) (from_ : Option EventSource)
    /-- "attacks", "attacks you", "blocks", "becomes blocked by …": an object's combat event,
    with the counterpart the sentence names. -/
    | combat (relation : CombatRelation) (subject : NounPhrase) (counterpart : Option NounPhrase)
    /-- "you attack with one or more creatures". -/
    | attacksWith (player : NounPhrase) (defender : Option NounPhrase) (attackers : NounPhrase)
    | attachment (move : AttachMove) (subject : NounPhrase) (host : NounPhrase)
    | dealsDamage (kind : DamageKind) (source : NounPhrase) (patient : Option NounPhrase)
    | beginningOf (quantifier : PartQuant) (part : TurnPart) (whose : HeaderPossessor)
    | casts (player : NounPhrase) (spell : Option NounPhrase) (from_ : Option EventSource)
    | becomesTarget (subject : NounPhrase) (by_ : NounPhrase)
    | statusEvent (subject : NounPhrase) (status : Status)
    /-- The game gains a designation: "it becomes night" [CR#731.1]. -/
    | gameBecomes (designation : DesignationLabel)
    | stateHolds (condition : Condition)
    | putInto (subject : NounPhrase) (destination : ZoneExpr) (from_ : Option EventSource)
    | counterEvent (move : CounterMove) (kind : Option CounterKind) (subject : NounPhrase)
        (batch : CounterBatch) (by_ : Option NounPhrase) (byEffect : Bool)
    | tokensCreated (tokens : NounPhrase) (byEffect : Bool) (by_ : Option NounPhrase)
        (under : Option NounPhrase)
    | chapterMark (chapters : List ChapterNumber)
    | activates (player : NounPhrase) (ability : NounPhrase)
    | statBecomes (subject : NounPhrase) (stat : Stat) (value : Amount)
    | flipsCoin (player : NounPhrase) (call : Option FlipCall)
    /-- `sides = none` is "whenever you roll a die", any die. -/
    | rollsDice (player : NounPhrase) (batch : DiceBatch) (sides : Option Nat) (watch : RollWatch)
    | paysCost (player : Option NounPhrase) (outcome : PaymentOutcome) (whose : NounPhrase)
        (keyword : KeywordLabel)
    | paysLife (player : NounPhrase)
    | lifeChanges (player : NounPhrase) (move : LifeMove)
    | verbedEvent (agent : Option NounPhrase) (verb : Deed) (patient : Option NounPhrase)
        (becomes : Option Predicate) (locus : Option ZoneExpr)
    /-- A mana ability with {T} in its cost resolving and producing mana [CR#106.12a]. -/
    | tappedForMana (player : Option NounPhrase) (source : NounPhrase)
        (type : Option ManaTypeTerm)
    | unlocksDoor (player : NounPhrase) (door : Door)
    | nthOccurrence (ordinal : Ordinal) (per : Option TurnPart) (event : GameEvent)
    | triggers (ability : NounPhrase)
    | commitsCrime (player : NounPhrase)
    | causes (cause : Causing) (event : GameEvent)

  inductive Causing where
    | source (source : NounPhrase)
    | event (event : GameEvent)
    | anEffect
end

/-! `DecidableEq` does not derive for a nested mutual block; `BEq` and `Repr` do. -/
deriving instance Repr, BEq for ZoneScope, LibraryPlace, ZoneExpr, NameSource, ChoiceDomain,
  EventSource, LookbackClause, Predicate, DetPhrase, NounPhrase, Amount,
  Quantity, SliceCount, Door, RollWatch, HeaderPossessor, Condition, GameEvent, Causing

inductive SearchScope where
  | oneZone (zone : ZoneExpr)
  | someZones (whose : Option NounPhrase) (zones : List Zone)
  deriving Repr, BEq

inductive FlipScope where
  | count (amount : Amount)
  | per (each : NounPhrase)
  deriving Repr, BEq

inductive IgnoredOutcomes where
  | extreme (extreme : RollExtreme)
  | allBut (extreme : RollExtreme)
  | chosen (chooser : Option NounPhrase) (amount : Amount)
  deriving Repr, BEq

inductive Ballot where
  | byLabel (options : List VoteLabel)
  | byCandidate (candidates : NounPhrase)
  deriving Repr, BEq

inductive Exposed where
  | cards (cards : NounPhrase)
  | zone (zone : ZoneExpr)
  | choice (sort : ChoiceSort)
  deriving Repr, BEq

inductive VisibleThing where
  | topOfLibrary
  | wholeHand
  | objects (objects : NounPhrase)
  deriving Repr, BEq

inductive DurationEnd where
  | startOf (part : TurnPart) (whose : Option NounPhrase)
  | endOf (part : TurnPart) (whose : Option NounPhrase)
  deriving Repr, BEq

end Semantics

attribute [semantic_expression] Semantics.GameEvent Semantics.NounPhrase Semantics.Predicate
  Semantics.Amount Semantics.Quantity Semantics.ZoneExpr Semantics.Condition

classify_semantic_syntax

attribute [internal_expansion] Semantics.NounPhrase.pro Semantics.NounPhrase.gap

attribute [semantic_literal] Semantics.Amount.lit
