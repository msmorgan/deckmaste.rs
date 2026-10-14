import Semantics.Events

/-!
# Semantics.Abilities

Minimal core action-clause and DRT spine (`docs/decisions/semantics-v3.md` §1, §3):
phrases (`ZoneExpr`, `Predicate`, `NounPhrase`, `Amount`, `Quantity`, `Condition`, `GameEvent`,
`DidClause`, `LookbackSpan`, `Subject`), instructions (`Instruction`), static specifications
(`StaticSpec`, `WouldClause`, `DeonticMods`, `DamageResult`), costs (`Cost`), and abilities
(`Ability`).
-/

namespace Semantics

inductive ObservationPoint where
  | arrival | departure
  deriving DecidableEq, Repr

inductive ContinuationPolicy where
  | optional
  | required
  deriving DecidableEq, Repr

inductive NameSource where
  | printed (name : String)
  | chosen
  deriving DecidableEq, Repr

open scoped Semantics.Authoring

mutual
  -- ## Phrase Grammar

  inductive ZoneScope where
    | bare
    | possessedBy (possessor : NounPhrase)

  inductive Pile where
    | library (whose : ZoneScope)
    | graveyard (whose : ZoneScope)
    | that
    | exiledWith (source : NounPhrase)

  inductive ZoneExpr where
    | inCaller (body : ZoneExpr)
    | zone (zone : Zone) (scope : ZoneScope)
    | place (end_ : LibraryEnd) (pile : Pile)
    | wherever
    | anyZone

  inductive EventSource where
    | anywhere
    | zones (zones : List ZoneExpr)
    | anywhereBut (zones : List ZoneExpr)

  inductive Subject where
    | actor (np : NounPhrase)
    | any
    | by (np : NounPhrase)

  inductive LookbackSpan where
    | duration (d : Duration) (timing : Option Timing)
    | thisWay
    | triggering

  structure DidClause where
    subj : Subject
    action : Instruction
    within : LookbackSpan

  inductive TurnRef where
    | current
    | previous
    | next (player : Option NounPhrase)
    | each (player : Option NounPhrase)
    | later
    | that

  inductive Timing where
    | inPart (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef)
    | relativeToEvent (side : EventSide) (event : GameEvent)
    | whileDoing (event : GameEvent)
    | onlyIf (condition : Condition)
    | and (conjuncts : List Timing)
    | or (disjuncts : List Timing)
    | not (timing : Timing)
    | anyTiming

  inductive Duration where
    | between (start : Option GameEvent) (end_ : Option GameEvent)
    | forAsLongAs (condition : Condition)

  inductive LinkedFacet where
    | colorsSpent
    | manaValueSpent
    | timesPaid (which : LinkSource)
    | readback (which : LinkSource) (within : Option Duration)
    | chosenPlayer (which : LinkSource)
    | objects (which : LinkSource) (deed : Deed) (plurality : Plurality)
    | taken (which : LinkSource)

  inductive Predicate where
    | inCaller (body : Predicate)
    | hasType (type : CardType)
    | hasSubtype (subtype : Subtype)
    | hasSupertype (supertype : Supertype)
    | anyPlayer
    | opponent
    | hasPossessor (axis : PossessorAxis) (possessor : NounPhrase)
    | did (clause : DidClause)
    | colorIs (color : Color)
    | named (source : NameSource)
    | isCard
    | isToken
    | hasStatus (status : Status)
    | hasCounters (kind : Option CounterKind)
    | compare (axes : List ProjAxis) (comparator : Comparator) (bound : Amount)
    | compareOver (domain : Predicate) (measure : Amount) (comparator : Comparator) (bound : Amount)
    | inZone (zone : ZoneExpr)
    | inCombat (relation : CombatRelation) (counterpart : Option NounPhrase)
    | hasDesignation (designation : DesignationLabel) (holder : Option NounPhrase)
    | related (label : RelationLabel) (counterpart : Option NounPhrase)
    | attachment (side : AttachmentSide) (word : Option AttachWord)
    | isEmblem
    | currentFace (side : CardFaceSide)
    | exiledWith (source : NounPhrase)
    | otherThan (anchor : NounPhrase)
    | isSource
    | and (conjuncts : List Predicate)
    | or (disjuncts : List Predicate)
    | not (predicate : Predicate)
    | bracketed (predicate : Predicate)
    | other

  inductive DetPhrase where
    | target (quantity : Quantity)
    | a (mode : ChoiceMode)
    | each
    | all
    | the
    | count (quantity : Quantity) (mode : Option ChoiceMode)
    | bare

  inductive NounPhrase where
    | inCaller (body : NounPhrase)
    | which
    | this
    | you
    | actor
    | playerGroup (group : PlayerGroupWord)
    | described (determiner : DetPhrase) (predicate : Predicate)
    | eachOf (group : NounPhrase)
    | and (phrases : List NounPhrase)
    | or (phrases : List NounPhrase)
    | slice (end_ : LibraryEnd) (amount : Amount) (pile : Pile)
    | someOf (count : SliceCount) (description : Option Predicate) (group : NounPhrase)
    | theRest (kind : Kind) (plurality : Plurality)
    | pro (reach : Reach) (plurality : Plurality)
    | refl (plurality : Plurality)
    | firstMention (id : BindingId) (phrase : NounPhrase)
    | laterMention (id : BindingId) (phrase : NounPhrase)
    | possessorOf (axis : PossessorAxis) (subject : NounPhrase)
    | paid (facet : LinkedFacet) (subject : NounPhrase)
    | asType (type : CardType) (subject : NounPhrase) (subtype : Option Subtype)
    | asMarker (marker : MarkerWord) (subject : NounPhrase)
    | resolvedPermanent (spell : NounPhrase)
    | becomes (subject : NounPhrase)
    | theGrantor (marker : MarkerWord)
    | combatPlayer (role : CombatRole)
    | attachHost (word : AttachWord) (head : NounWord)
    | designated (designation : DesignationLabel) (whose : NounPhrase)
    | counterpart (label : RelationLabel) (subject : NounPhrase)
    | any

  inductive Amount where
    | inCaller (body : Amount)
    | lit (value : Int)
    | statOf (axis : ProjAxis) (subject : NounPhrase)
    | countOf (group : NounPhrase)
    | tally (clause : DidClause) (mode : TallyMode)
    | thatMuch
    | theOutcome (sort : OutcomeSort)
    | letter (letter : Letter)
    | arith (op : ArithOp) (left right : Amount)
    | upTo (bound : Amount)
    | anyAmount

  inductive Quantity where
    | inCaller (body : Quantity)
    | range (low high : Option Nat)
    | upToOf (amount : Amount)
    | exactlyOf (amount : Amount)

  inductive SliceCount where
    | counted (quantity : Quantity)
    | whole

  inductive Condition where
    | inCaller (body : Condition)
    | duringPart (part : TurnPart) (whose : Option NounPhrase)
    | exists_ (subject : NounPhrase)
    | did (clause : DidClause)
    | matches (subject : NounPhrase) (predicate : Predicate)
    | compareAmt (subject : Amount) (comparator : Comparator) (bound : Amount)
    | gameIs (designation : DesignationLabel)
    | noHolder (designation : DesignationLabel)
    | hasPriority (player : NounPhrase)
    | not (condition : Condition)
    | and (conjuncts : List Condition)
    | or (disjuncts : List Condition)

  inductive GameEvent where
    | inCaller (body : GameEvent)
    | does (subj : Subject) (action : Instruction)
    | beginningOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef)
    | endOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef)
    | gameBecomes (designation : DesignationLabel)
    | chapterMark (chapters : List ChapterNumber)
    | stateBasedActionsChecked
    | now
    | stateHolds (condition : Condition)

  inductive Exposed where
    | cards (cards : NounPhrase)
    | zone (zone : ZoneExpr)
    | choice (sort : ChoiceSort)

  inductive CountBound where
    | atLeast (amount : Amount)
    | moreThan (amount : Amount)
    | additional (quantity : Quantity)

  inductive UsageLimit where
    | mk (times : CountBound) (per : Period)

  structure JoinedHeader where
    event : GameEvent
    alternatives : List GameEvent
    while_ : Option Timing
    timing : Option Timing

  -- ## Instructions, Static Specs, Costs, and Abilities

  inductive CombatParticipation where
    | attacking (defender : Option NounPhrase)
    | blocked (value : Bool)
    | outsideCombat

  inductive CombatUpdate where
    | participation (state : CombatParticipation)
    | blocking (move : AttachMove) (attacker : NounPhrase)

  inductive Door where
    | thisDoor
    | doorOf (state : Option LockState) (room : NounPhrase)

  inductive CounterKindSource where
    | printed (kind : CounterKind)
    | bound
    | those
    | own
    | sameAs (source : NounPhrase)
    | anyKind

  structure Characteristics where
    name : Option String := none
    cost : Option ManaCost := none
    colors : List Color := []
    supertypes : List Supertype := []
    types : List CardType := []
    subtypes : List Subtype := []
    text : List Ability := []
    power : Option Amount := none
    toughness : Option Amount := none
    loyalty : Option Amount := none
    defense : Option Amount := none

  structure CharacteristicBundle where
    characteristics : Characteristics

  structure FaceDown where
    characteristics : Option Characteristics

  inductive TokenSpec where
    | written (characteristics : CharacteristicBundle)
    | asThose

  inductive WouldClause where
    | would (subj : Subject) (action : Instruction)
    | wouldAny (clauses : List WouldClause)

  inductive PlayPayment where
    | itsOwnCost
    | withoutPaying
    | payingInstead (cost : Cost)
    | faceDown (spec : FaceDown) (cost : Cost)

  inductive DeonticRider where
    | stateBased (cause : StateBasedCause)
    | noRider
    | play (from_ : Option ZoneExpr) (limit : Option UsageLimit) (timing : Option Timing)
        (exclusive : Bool) (payment : PlayPayment)

  structure DeonticMods where
    bound : Option CountBound := none
    rider : DeonticRider

  structure DamageResult where
    kind : DamageKind
    source : Subject
    recipient : Predicate
    mode : ResultMode
    does : Instruction

  inductive StaticSpec where
    | inCaller (body : StaticSpec)
    | modification (subject : NounPhrase) (stat : Stat) (delta : Delta Amount)
    | ptModification (subject : NounPhrase) (power : Delta Amount) (toughness : Delta Amount)
    | letterDefinition (letter : Letter) (amount : Amount)
    | abilityGrant (subject : NounPhrase) (ability : Ability)
    | controlGrant (player : NounPhrase) (subject : NounPhrase)
    | entryRider (subject : NounPhrase) (rider : EntryRider)
    | combatant (subject : NounPhrase)
    | cant (subj : Subject) (action : Instruction) (mods : DeonticMods)
    | must (subj : Subject) (action : Instruction) (mods : DeonticMods)
    | may (subj : Subject) (action : Instruction) (mods : DeonticMods)
    | gatedBy (cost : Cost) (subj : Subject) (action : Instruction) (mods : DeonticMods)
    | instead (event : WouldClause) (replacement : Instruction) (timing : Option Timing)
    | damageResult (rule : DamageResult)
    | conditional (spec : StaticSpec) (condition : Condition) (marking : CondMarking)
    | conjunction (subject : Option NounPhrase) (parts : List StaticSpec)
    | specialAction (when : Option Timing) (cost : Cost) (does : Instruction)
    | asEvent (event : GameEvent) (does : Instruction)
    | whileHolds (condition : Condition) (does : Instruction)

  inductive EntryRider where
    | entersAs (status : Status)
    | entersAttacking (defender : Option NounPhrase)
    | entersTransformed
    | entersMelded (into : String)
    | withCounters (amount : Amount) (kind : CounterKindSource) (mark : EntryCounterMark)
    | under (controller : NounPhrase)
    | faceDown (spec : FaceDown)

  inductive Cost where
    | inCaller (body : Cost)
    | mana (cost : ManaCost)
    | scaled (cost : Cost) (amount : Amount)
    | tapSymbol
    | untapSymbol
    | loyaltySymbol (loyalty : LoyaltyCost)
    | perform (instruction : Instruction)
    | compound (costs : List Cost)
    | or (costs : List Cost)
    | itsManaCost

  inductive CreationSpec where
    | token (spec : TokenSpec) (riders : List EntryRider)
    | emblem (name : Option String) (abilities : List Ability)

  inductive Instruction where
    | inCaller (body : Instruction)
    | ownScope (deed : Option Deed) (body : Instruction)
    | bracketed (instruction : Instruction)
    | dealDamage (amount : Amount) (recipient : NounPhrase) (kind : DamageKind)
    | markDamage (subject : NounPhrase) (amount : Amount)
    | clearDamage (subject : NounPhrase)
    | setStatus (status : Status) (subject : NounPhrase)
    | turnOver (subject : NounPhrase)
    | turnFaceDown (subject : NounPhrase) (spec : FaceDown)
    | combat (subject : NounPhrase) (update : CombatUpdate)
    | attack (defender : Option NounPhrase)
    | attackUnblocked (defender : Option NounPhrase)
    | attackWith (attackers : NounPhrase) (defender : Option NounPhrase)
    | block (attacker : NounPhrase)
    | attachment (move : AttachMove) (subject : NounPhrase) (host : Option NounPhrase)
    | changeControl (subject : NounPhrase)
    | gainDesignation (subject : NounPhrase) (designation : DesignationLabel)
    | unlock (door : Door)
    | setGameDesignation (designation : DesignationLabel)
    | commitCrime
    | conclude (verb : OutcomeVerb)
    | drawGame
    | restartGame
    | relate (subject : NounPhrase) (label : RelationLabel) (counterpart : NounPhrase)
    | choose (first : Option NounPhrase) (chosen : NounPhrase) (disclosure : Disclosure)
    | move (subject : NounPhrase) (from_ : EventSource) (to : Option ZoneExpr)
        (riders : List EntryRider) (byEffect : Bool) (departed : ObservationPoint)
    | changeLife (delta : Delta Amount)
    | draw (amount : Amount)
    | expose (verb : ExposeVerb) (exposed : Exposed)
    | establish (spec : StaticSpec) (duration : Option Duration)
    | createObject (count : Amount) (spec : CreationSpec) (byEffect : Bool)
    | putCounters (amount : Amount) (kind : CounterKindSource) (on : NounPhrase)
    | removeCounters (quantity : Option Quantity) (kind : Option CounterKindSource)
        (from_ : NounPhrase) (batch : CounterBatch)
    | enact (verb : Deed) (instruction : Instruction) (becomes : Option Predicate)
        (locus : Option ZoneExpr)
    | pay (cost : Cost) (times : PayTimes) (outcome : PaymentOutcome)
        (whose : Option NounPhrase) (keyword : Option KeywordLabel)
    | instruct (subj : NounPhrase) (action : Instruction)
    | withContinuation (policy : ContinuationPolicy) (body : Instruction)
        (ifDid : Option Instruction) (ifNot : Option Instruction)
    | doIf (condition : Condition) (instruction : Instruction) (otherwise : Option Instruction)
    | doForEach (group : NounPhrase) (body : Instruction)
    | sequentially (steps : List Instruction)
    | simultaneously (steps : List Instruction)
    | procedure (kind : Procedure) (subject : NounPhrase) (payment : PlayPayment)
        (resulting : Option Predicate) (from_ : Option EventSource) (of_ : Option NounPhrase)

  inductive KeywordParam where
    | cost (cost : Cost)
    | quality (predicate : Predicate)
    | subject (predicate : Predicate)
    | number (amount : Amount)
    | ability (ability : Ability)

  inductive Ability where
    | keyword (keyword : KeywordLabel) (params : List KeywordParam) (body : List Ability)
    | activated (cost : Cost) (instruction : Instruction) (timing : Option Timing)
        (limit : Option UsageLimit) (guard : Option Condition) (activator : Option NounPhrase)
    | triggered (event : GameEvent) (alternatives : List GameEvent)
        (while_ : Option Timing) (joins : List JoinedHeader) (timing : Option Timing)
        (limit : Option UsageLimit) (intervening : Option Condition) (instruction : Instruction)
    | static (spec : StaticSpec)
    | spell (timing : Option Timing) (instruction : Instruction)
    | qualified (ability : Ability) (timing : Option Timing) (limit : Option UsageLimit)
        (guard : Option Condition) (from_ : Option Zone)
end

instance : AnyValue Subject where
  any := .any

instance : Coe NounPhrase Subject where
  coe := .actor

instance : AnyValue NounPhrase where
  any := .any

instance : AnyValue Amount where
  any := .anyAmount

instance : AnyValue CounterKindSource where
  any := .anyKind

instance : AnyValue ZoneExpr where
  any := .anyZone

instance : AnyValue Timing where
  any := .anyTiming

abbrev did := DidClause.mk
abbrev would := WouldClause.would
abbrev wouldAny := WouldClause.wouldAny

end Semantics

attribute [semantic_expression] Semantics.GameEvent Semantics.NounPhrase Semantics.Predicate
  Semantics.Amount Semantics.Quantity Semantics.ZoneExpr Semantics.Condition Semantics.Duration
  Semantics.Timing Semantics.Subject Semantics.LookbackSpan Semantics.WouldClause
  Semantics.Instruction Semantics.StaticSpec Semantics.Cost Semantics.Ability Semantics.TokenSpec

classify_semantic_syntax

attribute [internal_expansion] Semantics.NounPhrase.pro Semantics.NounPhrase.which
  Semantics.NounPhrase.firstMention Semantics.NounPhrase.laterMention
  Semantics.Quantity.inCaller Semantics.ZoneExpr.inCaller Semantics.Condition.inCaller
  Semantics.NounPhrase.inCaller Semantics.GameEvent.inCaller Semantics.Amount.inCaller
  Semantics.Predicate.inCaller Semantics.Instruction.enact Semantics.Instruction.inCaller
  Semantics.Instruction.ownScope Semantics.Cost.inCaller Semantics.StaticSpec.inCaller

attribute [semantic_literal] Semantics.Amount.lit
