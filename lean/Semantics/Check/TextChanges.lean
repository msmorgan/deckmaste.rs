import Semantics.Abilities
import Semantics.Check.Refusal

namespace Semantics

def TextWord.sort : TextWord → QualitySort
  | .color _ => .color
  | .subtype (.of host _) => .subtype host
  | .subtype (.spell _) => .subtype .instant

/-- The quality sort a word of a Text Change's sort is of. -/
def TextWordSort.quality : TextWordSort → QualitySort
  | .color => .color
  | .landType => .subtype .land
  | .creatureType => .subtype .creature

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
    let qualities := sorts.map TextWordSort.quality
    (if sorts.isEmpty then [.nonEmpty] else []) ++
      TextWord.sortCheck qualities from_ ++ TextWord.sortCheck qualities to ++
      TextWord.sameSort from_ to
  | _ => []

end Semantics
