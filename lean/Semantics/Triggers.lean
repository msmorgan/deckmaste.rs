import Semantics.Phrase

/-!
# Semantics.Triggers

Game events, durations, and trigger headers. Port of `idris/src/Experimental/Triggers.idr`,
syntax only, reshaped: the combat events are one constructor over `CombatRelation`, the
attachment events one over `AttachMove`, a regeneration is the act event it is, and the
day/night shift is the game gaining a designation.
-/

namespace Semantics

/-- The mana type a "tapped for mana of …" trigger specifies [CR#106.12a]: colorless, or a
color written or chosen; the six types of [CR#106.1b]. -/
inductive ManaTypeTerm where
  | colorless
  | ofColor (color : ColorTerm)
  deriving Repr, BEq

inductive Door where
  | thisDoor
  | doorOf (state : Option LockState) (room : NounPhrase)
  deriving Repr, BEq

/-- Which roll results a trigger watches. -/
inductive RollWatch where
  | anyResult
  | resultIn (quantity : Quantity)
  | highestNatural
  deriving Repr, BEq

/-- Whose turn part a "beginning of" header names. -/
inductive HeaderPossessor where
  | noPossessor
  | byPlayer (player : NounPhrase)
  | byTurn (turn : NounPhrase)
  deriving Repr, BEq

/-- When an ability may be used or a trigger fires: "any time you could cast a sorcery",
"during your turn", "before attackers are declared". -/
inductive Timing where
  | asSorcery
  | asInstant
  | duringPart (part : TurnPart) (whose : Option NounPhrase)
  | beforePart (part : TurnPart) (whose : Option NounPhrase)
  deriving Repr, BEq

mutual
  inductive Duration where
    | thisTurn
    | restOfGame
    | until_ (end_ : DurationEnd)
    | forAsLongAs (condition : Condition)
    | untilEvent (event : GameEvent)
    | duringNextTurnOf (player : NounPhrase)

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
    | casts (player : NounPhrase) (spell : NounPhrase) (from_ : Option ZoneExpr)
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
        (becomes : Option Predicate)
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

deriving instance Repr, BEq for Duration, GameEvent, Causing

inductive UsageLimit where
  | oncePerTurn | oncePerGame | actionOncePerTurn
  deriving DecidableEq, Repr

/-- "while …": a condition or an event underway. -/
inductive Concurrent where
  | whileTrue (condition : Condition)
  | whileDoing (event : GameEvent)
  deriving Repr, BEq

/-- A second trigger header joined to the first with "or". -/
structure JoinedHeader where
  event : GameEvent
  alternatives : List GameEvent
  while_ : Option Concurrent
  timing : Option Timing
  deriving Repr, BEq

end Semantics

attribute [semantic_expression] Semantics.GameEvent Semantics.Duration Semantics.Timing Semantics.UsageLimit

classify_semantic_syntax
