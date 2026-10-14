import Semantics.Abilities
import Semantics.Check.Refusal

namespace Semantics

def TextWord.sort : TextWord → QualitySort
  | .color _ => .color
  | .basicLandType (.of host _) => .subtype host
  | .basicLandType (.spell _) => .subtype .instant

def QualitySort.replaceable : QualitySort → Bool
  | .color | .subtype .land => true
  | _ => false

def TextWord.sortCheck (sorts : List QualitySort) : Option TextWord → List Refusal
  | none => []
  | some w => if sorts.contains w.sort then []
    else [.kindMismatch (.quality (sorts.headD .color)) (.quality w.sort)]

def TextWord.sameSort : Option TextWord → Option TextWord → List Refusal
  | some a, some b => if a.sort == b.sort then []
    else [.kindMismatch (.quality a.sort) (.quality b.sort)]
  | _, _ => []

def TextChange.shape : TextChange → List Refusal
  | .replaceWord sorts from_ to =>
    (if sorts.isEmpty then [.nonEmpty] else []) ++
      (sorts.filter (!·.replaceable)).map
        (fun sort => .kindMismatch (.quality .color) (.quality sort)) ++
      TextWord.sortCheck sorts from_ ++ TextWord.sortCheck sorts to ++ TextWord.sameSort from_ to
  | _ => []

end Semantics
