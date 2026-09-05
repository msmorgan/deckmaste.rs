import Experimental.Words
import Experimental.Events

/-!
# Experimental.Phrase

The phrase grammar: zones, predicates, noun phrases, amounts, quantities, and conditions.
Port of `idris/src/Experimental/Phrase.idr`, syntax only.

The Idris block is inductive-recursive: constructor types carry `{auto 0 … : …}`
obligations computed by functions over the block itself, and every family is indexed by the
antecedent stack `bs` and by `Kind`. Neither index is here. What remains is the shape a
sentence is written in, one constructor per Idris constructor with the same fields, minus the
obligations. The kind of a phrase and the stack it introduces become functions over this
syntax when the checker returns.

Not ported (witnesses and checker data): `ChoiceInScope`, `ComplementWritten`, `RolesOk`,
`NonZeroQ`, `ComplementAnchor`, `LinkSource`, `PileMention`, `PaidSubject`, `DestOk`,
`MarkingOk`, `TokenPhrase`, `DamageRecipient`, `StatusHolder`, `NotAnAbility`, `DiscardOk`,
`Movable`.
-/

namespace Mtg

/-- A color, written or "the chosen color". -/
inductive ColorTerm where
  | lit (c : Color)
  | chosen (ref : ChoiceRef)
  deriving DecidableEq, Repr

mutual
  /-- Whose zone: "your graveyard", or a bare zone name. -/
  inductive ZoneScope where
    | bare
    | possessedBy (n : Noun)

  /-- Where in a library. -/
  inductive LibPlace where
    | oneEnd (pos : LibPos)
    | eitherEnd (chooser : Option Noun)
    | shuffled

  inductive ZoneExpr where
    | zoneAt (z : Zone) (scope : ZoneScope)
    | libraryAt (place : LibPlace) (order : Option Arrangement) (offset : Option Ordinal)
        (scope : ZoneScope)

  /-- Where a name comes from: printed, chosen, or "with the same name as …". -/
  inductive NameSource where
    | printed (name : String)
    | chosen
    | sameAs (n : Noun)

  /-- What a choice ranges over. The Idris indexes this by `ChoiceSort`. -/
  inductive ChoiceDomain where
    | nameOfCard (p : Predicate)
    | colorOtherThan (c : Color)
    | typeOtherThan (s : Subtype)
    | basicTypesOnly
    | nonbasicTypesOnly
    | numberAbove (n : Nat)
    | opponentsOnly
    | numberBetween (lo hi : Nat)

  inductive EventSource where
    | anywhere
    | zones (zs : List ZoneExpr)
    | anywhereBut (zs : List ZoneExpr)

  /-- What an event happened to, from where, into where. -/
  inductive EventComplement where
    | involving (what : Noun)
    | fromZones (src : EventSource) (what : Option EventComplement)
    | intoZone (to : ZoneExpr) (what : Option EventComplement)
    | atZone (z : ZoneExpr)

  /-- "… that died this turn": an event, a window, and what it involved. -/
  inductive LookbackClause where
    | mk (event : EventName) (window : Lookback) (complement : Option EventComplement)

  inductive Predicate where
    | hasType (t : CardType)
    | hasSubtype (s : Subtype)
    | anyPlayer
    | opponent
    | chosenPlayer (ref : ChoiceRef)
    | qualityNoun (q : QualitySort) (domain : Option ChoiceDomain)
    | counterKindOn (n : Noun)
    | ofChosen (ref : ChoiceRef) (q : QualitySort)
    | ofYourChoice (q : QualitySort) (domain : Option ChoiceDomain)
    | hasKeyword (k : KeywordTerm)
    | hasPossessor (axis : PossessorAxis) (n : Noun)
    | castBy (n : Noun) (rank : Option (Ordinal × RankPeriod))
    | castFrom (z : ZoneExpr)
    | wasCast
    | attacking
    | beingDeclaredAttacker
    | blocking
    | blocked
    | combatRel (r : CombatRelation) (m : Noun)
    | happenedTo (lb : LookbackClause)
    | colorIs (c : Color)
    | isColorless
    | colorCount (r : Comparator) (n : Nat)
    | hasSupertype (s : Supertype)
    | named (src : NameSource)
    | hasDesignation (d : Designation) (holder : Option Noun)
    | isAttached (w : AttachWord)
    | attachedBy (w : AttachWord) (by_ : Noun)
    | attachedTo (host : Noun)
    | permanent
    | isCard
    | isToken
    | isSpell
    | isEmblem
    | isCopyOfACard
    | isHistoric
    | isTransformed
    | hasStatus (v : Status)
    | hasCounters (kind : Option CounterKind)
    | compare (axes : List ProjAxis) (r : Comparator) (bound : Amount)
    | superlative (op : AggregateOp) (axis : ProjAxis) (domain : Predicate)
    /-- A vote stands only where a spell or ability instructed players to vote [CR#701.38a]. -/
    | withMostVotes
    | choseExtreme (op : AggregateOp)
    | compareOver (domain : Predicate) (measure : Amount) (r : Comparator) (bound : Amount)
    | inZone (z : ZoneExpr)
    | inPile (pile : Noun)
    | exiledWith (src : Noun)
    | and (ps : List Predicate)
    | or (ps : List Predicate)
    | not (p : Predicate)
    | other
    | notChosen
    | otherThan (n : Noun)
    | joined (l r : Predicate)
    | coinCameUp (face : CoinFace)
    | isSource
    | manaCostHas (sym : ManaSymbol)
    | abilityHead (cls : AbilityClass)
    | abilityOf (src : Noun)
    | activatedBy (who : Noun)
    | isManaAbility
    | targets (m : Noun) (extent : TargetExtent)

  inductive DetPhrase where
    | target (q : Quantity)
    | a (mode : ChoiceMode)
    /-- "each …": a group, resolution-time [CR#608.2] -/
    | each
    | all
    | the
    | count (q : Quantity) (mode : Option ChoiceMode)
    /-- the bare plural: a description, no determiner [CR#109.2] -/
    | bare

  inductive Noun where
    /-- the source, by self-name or "this spell" [CR#113.7] -/
    | this
    | asType (t : CardType) (n : Noun) (sub : Option Subtype)
    | asMarker (m : MarkerWord) (n : Noun)
    | resolvedPermanent (spell : Noun)
    | theGrantor (m : MarkerWord)
    | you
    | theDefendingPlayer
    | theAttackingPlayer
    | playerGroup (w : PlayerGroupWord)
    | described (d : DetPhrase) (p : Predicate)
    | eachOf (group : Noun)
    | both (l r : Noun)
    | eitherOf (l r : Noun)
    | librarySlice (pos : LibPos) (amount : Amount) (whose : Noun)
    | someOf (q : SliceCount) (descr : Option Predicate) (group : Noun)
    | namesAgree (agreement : NameAgreement) (group : Noun)
    | theRest (k : Kind) (pl : Plurality)
    | pileOf (q : SliceCount) (by_ : Option Noun)
    /-- A pronoun: what it reaches for, its number, and the window it resolves in. -/
    | pro (r : Reach) (pl : Plurality) (w : Window)
    | attachHost (w : AttachWord) (h : NounWord)
    | possessorOf (axis : PossessorAxis) (n : Noun)
    | designated (d : Designation) (whose : Noun)
    | oneEachOf (roles : List Predicate) (pool : Noun)

  inductive Amount where
    | lit (n : Nat)
    | playerStatOf (s : PlayerStat) (n : Noun)
    | statOf (s : Stat) (n : Noun)
    | countOf (group : Noun)
    | aggregate (op : AggregateOp) (axis : ProjAxis) (group : Noun)
    | countersOn (kind : CounterKind) (holder : Noun)
    | paid (facet : PaidFacet) (n : Noun)
    | eventTally (op : TallyOp) (who : Noun) (lb : LookbackClause)
    | timesOf (per : Amount) (a : Amount)
    | thatMuch
    | chosenNumber (ref : ChoiceRef)
    | votesFor (label : VoteLabel)
    | theOutcome (s : OutcomeSort)
    | coinsShowing (face : CoinFace)
    | greatestStoredMatch (n : Noun)
    | groupSize
    | theDifference
    | letter (l : Letter)
    | plus (a b : Amount)
    | minus (a b : Amount)
    | devotion (who : Noun) (c : ColorTerm) (d : Option ColorTerm)
    | half (rounding : RoundMode) (a : Amount)
    | differenceBetween (a b : Amount)
    | aggregateOver (op : AggregateOp) (domain : Predicate) (body : Amount)
    | distinctCount (axis : KindAxis) (domain : Noun)
    | upTo (bound : Amount)

  inductive Quantity where
    | range (lo hi : Option Nat)
    | upToOf (a : Amount)
    | exactlyOf (a : Amount)

  inductive SliceCount where
    | counted (q : Quantity)
    | whole
end

/-! `DecidableEq` does not derive for a nested mutual block; `BEq` and `Repr` do. -/
deriving instance Repr, BEq for ZoneScope, LibPlace, ZoneExpr, NameSource, ChoiceDomain,
  EventSource, EventComplement, LookbackClause, Predicate, DetPhrase, Noun, Amount, Quantity,
  SliceCount

inductive SearchScope where
  | oneZone (z : ZoneExpr)
  | someZones (whose : Option Noun) (zs : List Zone)
  deriving Repr, BEq

inductive FlipScope where
  | count (n : Amount)
  | per (each : Noun)
  deriving Repr, BEq

inductive IgnoredOutcomes where
  | extreme (e : RollExtreme)
  | allBut (e : RollExtreme)
  | chosen (chooser : Option Noun) (n : Amount)
  deriving Repr, BEq

inductive Ballot where
  | byLabel (options : List VoteLabel)
  | byCandidate (n : Noun)
  deriving Repr, BEq

inductive Condition where
  /-- "if there is a …". `exists` is a Lean keyword. -/
  | thereIs (n : Noun)
  | happened (who : Noun) (lb : LookbackClause)
  | gameIs (d : Designation)
  | noHolder (d : Designation)
  | matches (n : Noun) (p : Predicate)
  | compareAmt (subject : Amount) (r : Comparator) (bound : Amount)
  | dealtThisWay (p : Predicate)
  | choseThisWay (who : Noun) (p : Predicate)
  | preventedFromSource (p : Predicate)
  | flipCalled (who : Noun) (call : FlipCall)
  | flipFace (face : CoinFace)
  | voteLead (label : VoteLabel) (orTied : Bool)
  | anyResultIs (r : Comparator) (bound : Amount)
  | rolledDoubles
  | not (c : Condition)
  | and (cs : List Condition)
  | or (cs : List Condition)
  deriving Repr, BEq

inductive Exposed where
  | cards (n : Noun)
  | zone (z : ZoneExpr)
  | choice (sort : ChoiceSort)
  deriving Repr, BEq

inductive VisibleThing where
  | topOfLibrary
  | wholeHand
  | objects (n : Noun)
  deriving Repr, BEq

inductive DurationEnd where
  | startOf (part : TurnPart) (whose : Option Noun)
  | endOf (part : TurnPart) (whose : Option Noun)
  deriving Repr, BEq

end Mtg
