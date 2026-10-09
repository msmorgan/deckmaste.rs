import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Sharing

Pins of Sharing and Agreement (`docs/decisions/semantics-v2.md` §7, 2026-10-09): `shares` is at
least one value in common on an axis, never equality, and `agree` is a constraint on a plural
group, not a predicate on one member.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Sharing

/-- An object named both "Fire" and "Ice" shares a name with an object named "Fire" and with one
named "Ice", though it equals neither name list [CR#201.2a]. -/
theorem twoNamesShareEither :
    Agreement.holdsOn .share ["Fire", "Ice"] ["Fire"] = true ∧
      Agreement.holdsOn .share ["Fire", "Ice"] ["Ice"] = true ∧
      (["Fire", "Ice"] == ["Fire"]) = false := by
  decide

/-- An object with no name has the same name as no other object, a nameless one included
[CR#201.2a]. -/
theorem namelessSharesNoName :
    Agreement.holdsOn (α := String) .share [] [] = false ∧
      Agreement.holdsOn .share [] ["Fire"] = false := by
  decide

/-- "a different name": no name in common [CR#201.2c]. -/
theorem differIsNoValueInCommon :
    Agreement.holdsOn .differ ["Fire", "Ice"] ["Ice"] = false ∧
      Agreement.holdsOn .differ ["Fire", "Ice"] ["Bolt"] = true := by
  decide

/-- "creatures that share a color with it": both relata are objects. -/
theorem okSharesColorWithThisCreature :
    NounPhrase.check (some .object) []
      (allOf (.and [creature, .shares .color .share thisCreature])) = [] := by
  decide

/-- "creatures that share a color with target player": the other relatum is not an object. -/
theorem badSharesWithAPlayer :
    NounPhrase.check (some .object) []
      (allOf (.and [creature, .shares .color .share (target .anyPlayer)])) ≠ [] := by
  decide

/-- "two creatures that share a creature type": an Agreement over a plural group. -/
theorem okTwoCreaturesAgree :
    NounPhrase.check (some .object) []
      (.agree (.subtype .creature) .share (counted (exactly 2) creature)) = [] := by
  decide

/-- "target creature that shares a creature type": an Agreement over one object is refused as a
plurality error. -/
theorem badOneCreatureAgrees :
    NounPhrase.check (some .object) []
      (.agree (.subtype .creature) .share (target creature)) = [.plural] := by
  decide

/-- Conspire's additional cost: "tap two untapped creatures you control that each share a color
with it" [CR#702.78a]; its copy half waits on `semantics-v2-spell-copies`. -/
theorem okConspireCost :
    Ability.check []
      (.static (.addedCost
        (.perform (tap (counted (exactly 2)
          (.and [untapped, creature, .hasPossessor .controller .you,
            .shares .color .share thisSpell]))))
        true .once)) = [] := by
  decide

end Semantics.Proofs.Sharing
