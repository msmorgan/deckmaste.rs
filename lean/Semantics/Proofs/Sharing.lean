import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Sharing

Pins of Sharing and Agreement (`docs/decisions/semantics-v2.md` §7, 2026-10-09): `shares` is at
least one value in common on an axis, never equality, and `agree` is a constraint on a plural
group, not a predicate on one member.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Sharing

/-- An object named both "Fire" and "Ice" shares a name with an object named "Fire" and with one
named "Ice", though it equals neither name list [CR#201.2a]. -/
theorem twoNamesShareEither :
    Agreement.holdsOn .cardName .share ["Fire", "Ice"] ["Fire"] = true ∧
      Agreement.holdsOn .cardName .share ["Fire", "Ice"] ["Ice"] = true ∧
      (["Fire", "Ice"] == ["Fire"]) = false := by
  decide

/-- An object with no name has the same name as no other object, a nameless one included
[CR#201.2a]. -/
theorem namelessSharesNoName :
    Agreement.holdsOn (α := String) .cardName .share [] [] = false ∧
      Agreement.holdsOn .cardName .share [] ["Fire"] = false := by
  decide

/-- "a different name": no name in common [CR#201.2c]. -/
theorem differIsNoValueInCommon :
    Agreement.holdsOn .cardName .differ ["Fire", "Ice"] ["Ice"] = false ∧
      Agreement.holdsOn .cardName .differ ["Fire", "Ice"] ["Bolt"] = true := by
  decide

/-- An object with no name does not have a different name than any other object, even one
that has a name [CR#201.2c]. -/
theorem namelessHasNoDifferentName :
    Agreement.holdsOn .cardName .differ [] ["Fire"] = false ∧
      Agreement.holdsOn (α := String) .cardName .differ [] [] = false := by
  decide

/-- The nameless rule is on the name axis only: a colorless object has no color in common
with a red one, having no color [CR#105.2c]. -/
theorem colorlessDiffersInColor :
    Agreement.holdsOn .color .differ [] [Color.red] = true := by
  decide

/-- Liliana's Contract: "if you control four or more Demons with different names". A group has
different names only if each member has at least one name and no two have a name in common
[CR#201.2b], so a nameless member, as a face-down permanent is [CR#708.2a], breaks the group. -/
theorem withDifferentNamesNeedsEveryName :
    Agreement.holdsAmong .cardName .differ
        [["Archfiend of Ifnir"], ["Lord of the Pit"], ["Demonlord Belzenlok"],
          ["Rune-Scarred Demon"]] = true ∧
      Agreement.holdsAmong .cardName .differ
        [["Archfiend of Ifnir"], ["Lord of the Pit"], ["Demonlord Belzenlok"], []] = false ∧
      Agreement.holdsAmong .cardName .differ
        [["Archfiend of Ifnir"], ["Lord of the Pit"], ["Lord of the Pit"],
          ["Rune-Scarred Demon"]] = false := by
  decide

/-- "creatures that share a color with it": both relata are objects. -/
theorem okSharesColorWithThisCreature :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (allOf (.and [isCreature, .shares .color .share thisCreature])) = [] := by
  decide

/-- "creatures that share a color with target player": the other relatum is not an object. -/
theorem badSharesWithAPlayer :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (allOf (.and [isCreature, .shares .color .share (target .anyPlayer)])) ≠ [] := by
  decide

/-- "two creatures that share a creature type": an Agreement over a plural group. -/
theorem okTwoCreaturesAgree :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.agree (.subtype .creature) .share (counted (exactly 2) isCreature)) = [] := by
  decide

/-- "target creature that shares a creature type": an Agreement over one object is refused as a
plurality error. -/
theorem badOneCreatureAgrees :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.agree (.subtype .creature) .share (target isCreature)) = [.plural] := by
  decide

/-- Conspire's additional cost: "tap two untapped creatures you control that each share a color
with it" [CR#702.78a]; the whole body is pinned in `Proofs/SpellCopies.lean`. -/
theorem okConspireCost :
    Ability.check (DRS.ofList [])
      (.static (.addedCost
        (.perform (tap (counted (exactly 2)
          (.and [isUntapped, isCreature, .hasPossessor .controller .you,
            .shares .color .share thisSpell]))))
        true .once)) = [] := by
  decide

end Semantics.Proofs.Sharing
