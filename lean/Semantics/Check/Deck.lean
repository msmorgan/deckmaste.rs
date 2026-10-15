import Semantics.Abilities

namespace Semantics

def ProjAxis.deck : ProjAxis → Bool
  | .stat .manaValue => true
  | _ => false

def Amount.deckBound (amount : Amount) (fuel : Nat := 32) : Bool :=
  match fuel, amount with
  | n + 1, .inCaller body => body.deckBound n
  | _ + 1, .lit _ => true
  | _, _ => false

def Predicate.deckReadable (predicate : Predicate) (fuel : Nat := 64) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body | n + 1, .bracketed body => body.deckReadable n
  | _ + 1, .isCard | _ + 1, .hasType _ | _ + 1, .hasSubtype _ => true
  | _ + 1, .compare axes _ bound => axes.all ProjAxis.deck && bound.deckBound
  | n + 1, .and predicates | n + 1, .or predicates => predicates.all (fun p => p.deckReadable n)
  | n + 1, .not body => body.deckReadable n
  | _, _ => false

def QualitySort.deckComparable : QualitySort → Bool
  | .number | .counterKind => false
  | _ => true

end Semantics
