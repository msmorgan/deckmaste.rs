import Semantics.Abilities
import Semantics.Check.Refusal

namespace Semantics

def TextWord.sort : TextWord → QualitySort
  | .color _ => .color
  | .subtype (.of host _) => .subtype host
  | .subtype (.spell _) => .subtype .instant

/-- The sorts of word a Text Change replaces: color words, land types and creature types
[CR#612.2,612.2a]. -/
def QualitySort.replaceable : QualitySort → Bool
  | .color => true
  | .subtype host => host == .land || host == .creature
  | _ => false

def TextWord.sortCheck (sorts : List QualitySort) : Option TextWord → List Refusal
  | none => []
  | some w => if sorts.contains w.sort then []
    else [.textWordSort sorts w.sort]

def TextWord.sameSort : Option TextWord → Option TextWord → List Refusal
  | some a, some b => if a.sort == b.sort then []
    else [.kindMismatch (.quality a.sort) (.quality b.sort)]
  | _, _ => []

def TextChange.shape : TextChange → List Refusal
  | .replaceWord sorts from_ to =>
    (if sorts.isEmpty then [.nonEmpty] else []) ++
      (sorts.filter (!·.replaceable)).map
        (fun sort => .textChangeSort sort) ++
      TextWord.sortCheck sorts from_ ++ TextWord.sortCheck sorts to ++ TextWord.sameSort from_ to
  | _ => []

end Semantics
