import Semantics.Check.Events

namespace Semantics

def NounPhrase.isYouFuel (fuel : Nat) : NounPhrase → Bool :=
  match fuel with
  | 0 => fun _ => false
  | fuel + 1 => fun
      | .you => true
      | .inCaller body | .firstMention _ body | .laterMention _ body => body.isYouFuel fuel
      | _ => false
termination_by structural fuel

def NounPhrase.isYou := NounPhrase.isYouFuel 64

mutual
  /-- Whether a player phrase can only name opponents: "an opponent", "your opponents", "each
  opponent". -/
  def Predicate.opponentOnlyFuel (fuel : Nat) : Predicate → Bool :=
    match fuel with
    | 0 => fun _ => false
    | fuel + 1 => fun
        | .inCaller body | .bracketed body => body.opponentOnlyFuel fuel
        | .opponent => true
        | .and ps => Predicate.anyOpponentOnlyFuel fuel ps
        | _ => false
  termination_by structural fuel
  def Predicate.anyOpponentOnlyFuel (fuel : Nat) : List Predicate → Bool :=
    match fuel with
    | 0 => fun _ => false
    | fuel + 1 => fun
        | [] => false
        | p :: ps => p.opponentOnlyFuel fuel || Predicate.anyOpponentOnlyFuel fuel ps
  termination_by structural fuel
end

def Predicate.opponentOnly := Predicate.opponentOnlyFuel 64
def Predicate.anyOpponentOnly := Predicate.anyOpponentOnlyFuel 64

def NounPhrase.opponentOnlyFuel (fuel : Nat) : NounPhrase → Bool :=
  match fuel with
  | 0 => fun _ => false
  | fuel + 1 => fun
      | .laterMention _ body => body.opponentOnlyFuel fuel
      | .inCaller body | .firstMention _ body => body.opponentOnlyFuel fuel
      | .described _ p => p.opponentOnlyFuel fuel
      | .playerGroup .yourOpponents => true
      | .eachOf g => g.opponentOnlyFuel fuel
      | _ => false
termination_by structural fuel

def NounPhrase.opponentOnly := NounPhrase.opponentOnlyFuel 64

/-- The possessor a player phrase fixes, relative to the controller [CR#109.5], with `actor` read
in its handoff: "you" is the controller, and a phrase that can only name opponents is another
player. Any other player phrase fixes none. -/
def NounPhrase.possessorSide (bs : DRS) (who : NounPhrase) : Option Possessor :=
  match who with
  | .actor => match actorShape? bs with
    | none => some .you
    | some shape =>
      if shape.isYou then some .you else if shape.opponentOnly then some .other else none
  | _ => if who.isYou then some .you else if who.opponentOnly then some .other else none

/-- The possessor a zone expression names: the player whose hand, graveyard or library it is
[CR#400.1,400.3]. A shared zone has no possessor. -/
def ZoneExpr.seedPossessorFuel (fuel : Nat) (bs : DRS) : ZoneExpr → Option Possessor :=
  match fuel with
  | 0 => fun _ => none
  | fuel + 1 => fun
      | .inCaller body => body.seedPossessorFuel fuel (enterCaller bs)
      | .zone z (.possessedBy who) => if z.possessable then who.possessorSide bs else none
      | .place _ (.library (.possessedBy who)) | .place _ (.graveyard (.possessedBy who)) =>
        who.possessorSide bs
      | _ => none
termination_by structural fuel

def ZoneExpr.seedPossessor (bs : DRS) := ZoneExpr.seedPossessorFuel 64 bs

mutual
  /-- The possessor a description fixes for the object it describes: the possessor of the hand,
  graveyard or library it is in [CR#400.1,400.3], or its controller [CR#108.4]; "you don't control"
  fixes another player. A conjunction takes the first conjunct that fixes one, and the arms of
  a disjunction must agree. -/
  def Predicate.seedPossessorFuel (fuel : Nat) (bs : DRS) : Predicate → Option Possessor :=
    match fuel with
    | 0 => fun _ => none
    | fuel + 1 => fun
        | .inCaller body => body.seedPossessorFuel fuel (enterCaller bs)
        | .bracketed body => body.seedPossessorFuel fuel bs
        | .inZone z => z.seedPossessorFuel fuel bs
        | .hasPossessor .controller who => who.possessorSide bs
        | .not (.hasPossessor .controller who) =>
          if who.possessorSide bs == some .you then some .other else none
        | .and ps => Predicate.seedPossessorAllFuel fuel bs ps
        | .or ps => Predicate.seedPossessorJoinFuel fuel bs ps
        | _ => none
  termination_by structural fuel
  def Predicate.seedPossessorAllFuel (fuel : Nat) (bs : DRS) : List Predicate → Option Possessor :=
    match fuel with
    | 0 => fun _ => none
    | fuel + 1 => fun
        | [] => none
        | p :: ps =>
          match p.seedPossessorFuel fuel bs with
          | some o => some o
          | none => Predicate.seedPossessorAllFuel fuel bs ps
  termination_by structural fuel
  def Predicate.seedPossessorJoinFuel (fuel : Nat) (bs : DRS) : List Predicate → Option Possessor :=
    match fuel with
    | 0 => fun _ => none
    | fuel + 1 => fun
        | [] => none
        | [p] => p.seedPossessorFuel fuel bs
        | p :: ps =>
          match p.seedPossessorFuel fuel bs, Predicate.seedPossessorJoinFuel fuel bs ps with
          | some o, some q => if o == q then some o else none
          | _, _ => none
  termination_by structural fuel
end

def Predicate.seedPossessor (bs : DRS) := Predicate.seedPossessorFuel 64 bs
def Predicate.seedPossessorAll (bs : DRS) := Predicate.seedPossessorAllFuel 64 bs
def Predicate.seedPossessorJoin (bs : DRS) := Predicate.seedPossessorJoinFuel 64 bs

def NounPhrase.possessorFuel (fuel : Nat) (bs : DRS) : NounPhrase → Option Possessor :=
  match fuel with
  | 0 => fun _ => none
  | fuel + 1 => fun
      | .inCaller body => body.possessorFuel fuel (enterCaller bs)
      | .described _ p => p.seedPossessor bs
      | .eachOf g => NounPhrase.possessorFuel fuel bs g
      | .agree _ _ g => NounPhrase.possessorFuel fuel bs g
      | .someOf _ _ g => NounPhrase.possessorFuel fuel bs g
      | .oneEachOf _ pool => NounPhrase.possessorFuel fuel bs pool
      | .pro r pl => possessorOfReach r pl bs
      | .firstMention _ body => NounPhrase.possessorFuel fuel bs body
      | .laterMention id body =>
        match identityBinding id bs with
        | some b => b.possessor
        | none => NounPhrase.possessorFuel fuel bs body
      | _ => none
termination_by structural fuel

def NounPhrase.possessor (bs : DRS) := NounPhrase.possessorFuel 64 bs

def keptPossessor (p : Option Deed) (oldZn z : Option Zone) (ps : Option Possessor) :
    Option Possessor :=
  match p with
  | some (.core .gainControl) => none
  | _ =>
    if oldZn == z || (oldZn.elim false Zone.possessable && z.elim false Zone.possessable) then ps
    else none

def Action.enactPatientFuel (fuel : Nat) : Action → Option NounPhrase :=
  match fuel with
  | 0 => fun _ => none
  | fuel + 1 => fun
      | .inCaller body => (body.enactPatientFuel fuel).map .inCaller
      | .ownScope _ body | .bracketed body => body.enactPatientFuel fuel
      | .move n _ _ _ _ _ => some n
      | _ => none
termination_by structural fuel

def Action.enactPatient := Action.enactPatientFuel 64

def enactPatientPossessorOk (bs : DRS) (v : Deed) (perf : Option NounPhrase)
    (e : Action) : Bool :=
  !actOwnPatient v ||
    match e.enactPatient, perf with
    | some n, some who => !Possessor.differs (NounPhrase.possessor bs n) (who.possessorSide bs)
    | _, _ => true

end Semantics
