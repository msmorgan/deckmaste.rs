import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.KeywordByCardClass

A keyword that means one ability on a permanent and another on an instant or sorcery spell
writes one definition holding both halves, each guarded by the card's class with the ordinary
conditional forms (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and the
additional-cost keywords) [CR#702.174b]. The instant-or-sorcery half may be a spell ability
[CR#113.3a]. Each half is checked as a term on every card, with no class context; against the
card's class the keyword itself is checked, not the halves of its definition.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordByCardClass

/-- "if this is a permanent card" -/
def onPermanentCard : Condition := .matches .this isPermanentCard

/-- "if this is an instant or sorcery" -/
def onInstantOrSorcery : Condition := .matches .this isInstantOrSorcery

/-- The permanent half: "When this permanent enters, if this is a permanent card, draw a
card." -/
def permanentHalf : Ability :=
  when (does thisPermanent enter) (if_ := some onPermanentCard) (draw 1)

/-- The spell half: "If this is an instant or sorcery, draw a card." -/
def spellHalf : Ability := .spell none (doIf onInstantOrSorcery (draw 1))

/-- Both halves are each well-formed with no class context. -/
theorem okEachHalfChecksClassBlind : Ability.checkAll (DRS.ofList []) [permanentHalf, spellHalf] = [] := by
  decide

/-- The definition with both halves on a permanent card: gift appears on permanent cards. -/
theorem okBothHalvesOnPermanentCard :
    cardTextOk [.creature] [.keyword (.named "gift") [] [permanentHalf, spellHalf]] = true := by
  decide

/-- The definition with both halves on an instant card: gift appears on instant and sorcery
cards. -/
theorem okBothHalvesOnInstantCard :
    cardTextOk [.instant] [.keyword (.named "gift") [] [permanentHalf, spellHalf]] = true := by
  decide

/-- "Flying (As long as this is a permanent card, this creature can't be blocked …)": a class
guard written in a keyword's reminder is checked as any conditional static ability. -/
theorem okFlyingGuardedToPermanentCard :
    Ability.check (DRS.ofList [])
      (.keyword (.named "flying") []
        [ match flying with
          | .keyword _ _ [.static se] => .static (.conditional se onPermanentCard .asLongAs)
          | ab => ab ])
      = [] := by
  decide

/-! ## Card-class gates the CR doesn't impose

A keyword's card-class columns admit each class the printed corpus puts it on. An ability that
functions while its spell is on the stack names no card class [CR#702.56a,702.61a,702.188a], and
wither works from any zone [CR#702.80c]. -/

/-- Sulfur Elemental, a creature card with split second; a creature card with replicate. -/
theorem okSpellKeywordsOnPermanentCards :
    cardTextOk [.creature] [splitSecond] = true ∧
      cardTextOk [.creature] [replicate (mana [generic 1, pip .green])] = true := by
  decide

/-- An instant card with web-slinging; Puncture Blast, an instant card with wither. -/
theorem okPermanentKeywordsOnSpellCards :
    cardTextOk [.instant] [webSlinging (mana [generic 1, pip .red])] = true ∧
      cardTextOk [.instant] [wither] = true := by
  decide

end Semantics.Proofs.KeywordByCardClass
