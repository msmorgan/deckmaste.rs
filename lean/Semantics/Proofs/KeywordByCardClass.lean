import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.KeywordByCardClass

A keyword that means one ability on a permanent and another on an instant or sorcery spell
writes one definition holding both halves. Gift's and haunt's halves carry no card-class guard:
each is scoped by its ability category and its event, an entering or dying trigger on the
permanent, and on the spell a spell ability or a trigger on its being put into a graveyard during
its resolution [CR#113.3a,113.6,603.4,608.2n,702.55a,702.174b] (ruling 2026-10-22). Ascend keeps
its guards until the card-class gate of `lean-v3-facts-from-declarations` decides them
[CR#113.6,702.131a,702.131b]. Each half is checked as a term on every card, with no class context;
against the card's class the keyword itself is checked, not the halves of its definition.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordByCardClass

/-- "if this is a permanent card" -/
def onPermanentCard : Condition := .matches .this isPermanentCard

/-- The halves of a keyword declaration's body. -/
def halves : Ability → List Ability
  | .keyword _ _ body => body
  | _ => []

/-- Gift's and haunt's unguarded halves, and ascend's guarded ones, are well-formed with no class
context. Gift's halves are checked as one text, since they read the player its cost chose
[CR#607.2d]; haunt's and ascend's each on its own. -/
theorem okEachHalfChecksClassBlind :
    Ability.checkText (DRS.ofList []) (halves (gift (draw 1))) = [] ∧
      Ability.checkAll (DRS.ofList []) (halves haunt) = [] ∧
      Ability.checkAll (DRS.ofList []) (halves ascend) = [] := by
  decide

/-- Gift, haunt and ascend, each holding both halves, on a permanent card. -/
theorem okBothHalvesOnPermanentCard :
    cardTextOk [.creature] [gift (draw 1)] = true ∧ cardTextOk [.creature] [haunt] = true ∧
      cardTextOk [.creature] [ascend] = true := by
  decide

/-- Gift, haunt and ascend, each holding both halves, on an instant card. -/
theorem okBothHalvesOnInstantCard :
    cardTextOk [.instant] [gift (draw 1)] = true ∧ cardTextOk [.instant] [haunt] = true ∧
      cardTextOk [.instant] [ascend] = true := by
  decide

/-- "Flying (As long as this is a permanent card, this creature can't be blocked …)": a class
guard written in a keyword's reminder is checked as any conditional static ability, as ascend's
static half still is. -/
theorem okFlyingGuardedToPermanentCard :
    Ability.check (DRS.ofList [])
      (.keyword (.named "flying") []
        [ match flying with
          | .keyword _ _ [.static se] => .static (.conditional se onPermanentCard .asLongAs)
          | ab => ab ])
      = [] := by
  decide

/-! ## No card-class gate on a keyword

A keyword ability is admitted on a card of either class (`classAbilityOk`). Which class the
printed corpus puts a keyword on is observation, not a rule: an ability that functions while its
spell is on the stack names no card class [CR#702.56a,702.61a,702.188a], and wither works from
any zone [CR#702.80c]. -/

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

/-- Keywords the retired card-class columns kept to one class, on a card of the other: a creature
card with flashback or fuse, an instant card with flying. -/
theorem okKeywordsOnTheOtherCardClass :
    cardTextOk [.creature] [flashback (mana [generic 1, pip .red])] = true ∧
      cardTextOk [.creature] [fuse] = true ∧
      cardTextOk [.instant] [flying] = true := by
  decide

end Semantics.Proofs.KeywordByCardClass
