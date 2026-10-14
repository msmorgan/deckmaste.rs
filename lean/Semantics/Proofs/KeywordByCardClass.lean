import Semantics.Macros.Declared
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.KeywordByCardClass

A keyword that means one ability on a permanent and another on an instant or sorcery spell
writes one definition holding both halves, each guarded by the card's class with the ordinary
conditional forms (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and the
additional-cost keywords) [CR#702.174b]. The instant-or-sorcery half may be a spell ability
[CR#113.3a]. Each half is checked as a term on every card, with no class context; against the
card's class only the half its guard names is checked.

The definition-fit pins hold a row written here, shaped as gift's, from before gift was bodied;
`Proofs/GiftVariants.lean` checks gift's variants against the registry's own row.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.KeywordByCardClass

/-- "if this is a permanent card" -/
def onPermanentCard : Condition := .matches .this permanentCard

/-- "if this is an instant or sorcery" -/
def onInstantOrSorcery : Condition := .matches .this instantOrSorcery

/-- The permanent half: "When this permanent enters, if this is a permanent card, draw a
card." -/
def permanentHalf : Ability :=
  triggeredIf (Declared.enters thisPermanent none) onPermanentCard (draw (.lit 1))

/-- The spell half: "If this is an instant or sorcery, draw a card." -/
def spellHalf : Ability := .spell none (doIf onInstantOrSorcery (draw (.lit 1)))

/-- A spell ability with no class guard: "Draw a card." -/
def unguardedSpell : Ability := .spell none (draw (.lit 1))

/-- A spell ability guarded by the other class: "If this is a permanent card, draw a card." -/
def permanentGuardedSpell : Ability := .spell none (doIf onPermanentCard (draw (.lit 1)))

/-- A keyword on both classes whose definition holds a triggered permanent half and a spell
half, as gift's [CR#702.174a,702.174b]. -/
def bothClassesRow : KeywordFacts :=
  { word := .named "gift", argumentSchemas := [[]], onInstantOrSorceryCard := true,
    definition := [⟨.triggered, none⟩, ⟨.spell, some .atResolution⟩] }

/-- The same definition on a keyword only permanents carry. -/
def permanentOnlyRow : KeywordFacts :=
  { bothClassesRow with onInstantOrSorceryCard := false }

/-- Both halves are each well-formed with no class context. -/
theorem okEachHalfChecksClassBlind : Ability.checkAll (DRS.ofList []) [permanentHalf, spellHalf] = [] := by
  decide

/-- Both halves fit a definition declaring a triggered and a spell ability. -/
theorem okBothHalvesFitDefinition :
    [permanentHalf, spellHalf].all bothClassesRow.bodyPartFits = true := by
  decide

/-- The definition with both halves on a permanent card: the triggered half applies and is
checked; the spell half never applies to a permanent and is not checked against it. -/
theorem okBothHalvesOnPermanentCard :
    cardTextOk [.creature] [.keyword (.named "gift") [] [permanentHalf, spellHalf]] = true := by
  decide

/-- The definition with both halves on an instant card. -/
theorem okBothHalvesOnInstantCard :
    cardTextOk [.instant] [.keyword (.named "gift") [] [permanentHalf, spellHalf]] = true := by
  decide

/-- The half the card's class names is checked against it: a spell ability guarded by the
permanent class is refused on a permanent card, and not checked on an instant. -/
theorem badPermanentGuardedSpellOnPermanentCard :
    (cardTextOk [.creature] [.keyword (.named "gift") [] [permanentGuardedSpell]],
      cardTextOk [.instant] [.keyword (.named "gift") [] [permanentGuardedSpell]])
      = (false, true) := by
  decide

/-- A spell ability outside a class guard is refused in a definition, as is one guarded by the
permanent class, where the keyword also appears on permanents [CR#113.3a]. -/
theorem badSpellOutsideClassGuard :
    (bothClassesRow.bodyPartFits unguardedSpell,
      bothClassesRow.bodyPartFits permanentGuardedSpell) = (false, false) := by
  decide

/-- A spell ability with no class guard is still refused in a registry definition: no row
declares one [CR#113.3a]. -/
theorem badUnguardedSpellInStormDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "storm") [] [unguardedSpell])
      = [.keywordBodyFits (.named "storm")] := by
  decide

/-- A guard on a class the keyword's card can never have is refused. -/
theorem badGuardOnClassKeywordNeverHas :
    (permanentOnlyRow.bodyPartFits spellHalf, permanentOnlyRow.bodyPartFits permanentHalf)
      = (false, true) := by
  decide

/-- "Flying (As long as this is an instant or sorcery, this creature can't be blocked …)": flying
is never on an instant or sorcery card, so a part guarded by that class is refused. -/
theorem badFlyingGuardedToInstantOrSorcery :
    Ability.check (DRS.ofList [])
      (.keyword (.named "flying") []
        [ match flyingExpansion with
          | .static se => .static (.conditional se onInstantOrSorcery .asLongAs)
          | ab => ab ])
      = [.keywordBodyFits (.named "flying")] := by
  decide

/-- "Flying (As long as this is a permanent card, this creature can't be blocked …)": a guard on
a class flying is carried on fits its definition. -/
theorem okFlyingGuardedToPermanentCard :
    Ability.check (DRS.ofList [])
      (.keyword (.named "flying") []
        [ match flyingExpansion with
          | .static se => .static (.conditional se onPermanentCard .asLongAs)
          | ab => ab ])
      = [] := by
  decide

end Semantics.Proofs.KeywordByCardClass
