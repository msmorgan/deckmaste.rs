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
  whenIf (does thisPermanent enter) onPermanentCard (draw (.lit 1))

/-- The spell half: "If this is an instant or sorcery, draw a card." -/
def spellHalf : Ability := .spell none (doIf onInstantOrSorcery (draw (.lit 1)))

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

end Semantics.Proofs.KeywordByCardClass
