import Experimental.Phrase

/-!
# Experimental.Triggers

Game events, durations, and trigger headers. Port of
`idris/src/Experimental/Triggers.idr`, syntax only.

Not ported (witnesses): `CrimeSubject`, `AltEvent`.

Renames: `NoDefender`/`NoPatient`/`NoPossessor` keep a word (`noDefender`, …) because a bare
`none` would shadow `Option.none` inside their own namespaces; `MkJoinedHeader` is a structure.
-/

namespace Mtg

/-- Whom an attack is declared against. -/
inductive AttackDefender where
  | noDefender
  | one (m : Noun)
  deriving Repr, BEq

/-- Whom damage is dealt to, when a trigger names it. -/
inductive DamagePatient where
  | noPatient
  | one (m : Noun)
  deriving Repr, BEq

inductive Door where
  | thisDoor
  | doorOf (state : Option LockState) (room : Noun)
  deriving Repr, BEq

/-- Which roll results a trigger watches. -/
inductive RollWatch where
  | anyResult
  | resultIn (q : Quantity)
  | highestNatural
  deriving Repr, BEq

/-- Whose turn part a "beginning of" header names. -/
inductive HeaderPossessor where
  | noPossessor
  | byPlayer (n : Noun)
  | byTurn (n : Noun)
  deriving Repr, BEq

mutual
  inductive Duration where
    | thisTurn
    | restOfGame
    | until (e : DurationEnd)
    | forAsLongAs (c : Condition)
    | untilEvent (ev : GameEvent)
    | duringNextTurnOf (who : Noun)

  inductive GameEvent where
    | dies (n : Noun)
    | leaves (n : Noun) (from_ : Option EventSource)
    | isDealtDamage (kind : DamageKind) (to : Noun)
    | draws (who : Noun)
    | losesGame (who : Noun)
    | enters (n : Noun) (from_ : Option EventSource)
    | attacks (n : Noun) (whom : AttackDefender)
    | attacksWith (who : Noun) (whom : AttackDefender) (attackers : Noun)
    | blocks (n : Noun) (what : Option Noun)
    | becomesBlocked (n : Noun) (by_ : Option Noun)
    | becomesAttached (n : Noun) (host : Noun)
    | becomesUnattached (n : Noun) (host : Noun)
    | dealsDamage (kind : DamageKind) (n : Noun) (to : DamagePatient)
    | beginningOf (q : PartQuant) (part : TurnPart) (whose : HeaderPossessor)
    | casts (who : Noun) (what : Noun) (from_ : Option ZoneExpr)
    | becomesTarget (n : Noun) (by_ : Noun)
    | statusEvent (n : Noun) (v : Status)
    | dayNightShift
    | stateHolds (c : Condition)
    | putInto (n : Noun) (to : ZoneExpr) (from_ : Option EventSource)
    | counterEvent (dir : CounterMove) (kind : Option CounterKind) (n : Noun)
        (batch : CounterBatch) (by_ : Option Noun) (byEffect : Bool)
    | tokensCreated (n : Noun) (byEffect : Bool) (by_ : Option Noun) (under : Option Noun)
    | chapterMark (ns : List ChapterNumber)
    | activates (who : Noun) (what : Noun)
    | statBecomes (n : Noun) (c : Stat) (v : Amount)
    | regenerates (n : Noun)
    | flipsCoin (who : Noun) (call : Option FlipCall)
    | rollsDice (who : Noun) (batch : DiceBatch) (die : RolledDie) (res : RollWatch)
    | paysCost (who : Option Noun) (out : PaymentOutcome) (whose : Noun) (kw : KeywordLabel)
    | paysLife (who : Noun)
    | lifeChanges (who : Noun) (dir : LifeMove)
    | verbedEvent (who : Option Noun) (v : VerbLabel) (what : Option Noun)
        (becomes : Option Predicate)
    /-- A mana ability with {T} in its cost resolving and producing mana [CR#106.12a]. -/
    | tappedForMana (who : Option Noun) (what : Noun)
    | unlocksDoor (who : Noun) (door : Door)
    | nthOccurrence (ord : Ordinal) (per : Option TurnPart) (ev : GameEvent)
    | triggers (what : Noun)
    | commitsCrime (who : Noun)
    | causes (by_ : Causing) (what : GameEvent)

  inductive Causing where
    | source (src : Noun)
    | event (ev : GameEvent)
    | anEffect
end

deriving instance Repr, BEq for Duration, GameEvent, Causing

inductive TriggerWindow where
  | during (p : TurnPart) (w : Option Noun)
  deriving Repr, BEq

inductive Timing where
  | asSorcery
  | asInstant
  | duringPart (p : TurnPart) (w : Option Noun)
  | beforePart (p : TurnPart) (w : Option Noun)
  deriving Repr, BEq

inductive UsageLimit where
  | oncePerTurn | oncePerGame | actionOncePerTurn
  deriving DecidableEq, Repr

/-- "while …": a condition or an event underway. -/
inductive Concurrent where
  | whileTrue (c : Condition)
  | whileDoing (ev : GameEvent)
  deriving Repr, BEq

/-- A second trigger header joined to the first with "or". -/
structure JoinedHeader where
  word : TriggerWord
  event : GameEvent
  alts : List GameEvent
  while_ : Option Concurrent
  window : Option TriggerWindow
  deriving Repr, BEq

end Mtg
