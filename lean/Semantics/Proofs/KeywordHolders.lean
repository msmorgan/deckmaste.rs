import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.KeywordHolders

Keyword bodies written over the party the rules name. Hexproof, shroud and protection are a
"permanent or player"'s, so a player grant (*Leyline of Sanctity*, *Ivory Mask*, *Teferi's
Protection*) reads them over its holder [CR#702.11c,702.18a,702.16b,702.16c,702.16e]. Detain and
goad last until the next turn of the controller of the spell or ability, "your next turn", not
the source's controller's [CR#701.35a,701.15a,109.5,113.8]. Hideaway's look right follows whoever
controls the permanent [CR#702.75a,611.2c]. Converting is transforming [CR#701.28a]. Each pin is
closed by `decide` or `rfl`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordHolders

/-- The static clauses a keyword's body holds. -/
def staticClauses : Ability → List StaticSpec
  | .keyword _ _ body => body.filterMap fun | .static spec => some spec | _ => none
  | _ => []

/-- The patient of a forbidden targeting, if the clause is one. -/
def targetedPatient? : StaticSpec → Option NounPhrase
  | .cant _ (.target patient) _ => some patient
  | _ => none

/-- Hexproof and shroud forbid targeting their holder, whatever it is: "You can't be the target
of spells or abilities your opponents control" on a player [CR#702.11c,702.18a]. -/
theorem hexproofAndShroudReadTheirHolder :
    ((staticClauses hexproof).filterMap targetedPatient?,
      (staticClauses shroud).filterMap targetedPatient?) = ([this_], [this_]) := by
  rfl

/-- Protection's targeting clause reads its holder too [CR#702.16b]. -/
theorem protectionTargetingReadsItsHolder :
    (staticClauses (protection (hasColor .black))).filterMap targetedPatient? = [this_] := by
  rfl

/-- The host of a forbidden Aura attachment, if the clause is one. -/
def auraHost? : StaticSpec → Option NounPhrase
  | .cant _ (.ownScope _ (.attachment .attached _
      (some (.firstMention _ (.inCaller host))))) _ => some host
  | _ => none

/-- The recipient of a damage-prevention clause, if the clause is one. -/
def preventedRecipient? : StaticSpec → Option NounPhrase
  | .replace (.would _ (.dealDamage _ recipient _)) (.instead (.preventDamage ..)) .. =>
    some recipient
  | _ => none

/-- Protection's enchant and damage clauses read its holder: a "permanent or player" can't be
enchanted by Auras of the quality, and damage from sources of the quality to it is prevented
[CR#702.16c,702.16e]. Its Equipment and Fortification clause stays on the permanent
[CR#702.16d]. -/
theorem protectionEnchantAndDamageReadItsHolder :
    ((staticClauses (protection (hasColor .black))).filterMap auraHost?,
      (staticClauses (protection (hasColor .black))).filterMap preventedRecipient?)
      = ([this_, thisPermanent], [this_]) := by
  rfl

/-- "You have hexproof." (Leyline of Sanctity) still checks. The checker does not read which
party a keyword's body names, so this pin guards the grant's form only; the holder is pinned by
`hexproofAndShroudReadTheirHolder`. -/
theorem okPlayerHasHexproof :
    Ability.check (DRS.ofList []) (static (abilityGrant .you hexproof)) = [] := by decide

/-- "You have shroud." (Ivory Mask) still checks; see `okPlayerHasHexproof`. -/
theorem okPlayerHasShroud :
    Ability.check (DRS.ofList []) (static (abilityGrant .you shroud)) = [] := by decide

/-- "You have protection from everything." (Teferi's Protection), with a color standing in for
the quality, still checks; see `okPlayerHasHexproof`. -/
theorem okPlayerHasProtection :
    Ability.check (DRS.ofList []) (static (abilityGrant .you (protection (hasColor .black))))
      = [] := by
  decide

/-- The duration a keyword action's own scope establishes, if its deed's body is one. -/
def establishedDuration? : Action → Option Duration
  | .ownScope _ (.establish _ duration) => duration
  | .ownScope _ (.gainDesignation _ _ duration) => duration
  | _ => none

/-- Detain and goad last until "your next turn", the next turn of the controller of the spell or
ability that detains or goads [CR#701.35a,701.15a,109.5]. -/
theorem detainAndGoadLastUntilYourNextTurn :
    (establishedDuration? (detain (target isCreature)),
      establishedDuration? (goad (target isCreature)))
      = (some untilYourNextTurn, some untilYourNextTurn) := by
  rfl

/-- "Detain target creature." and "Goad target creature." still check. -/
theorem okDetainAndGoad :
    (Action.check (DRS.ofList []) (detain (target isCreature)),
      Action.check (DRS.ofList []) (goad (target isCreature))) = ([], []) := by
  decide

/-- Hideaway grants the look to you and to the permanent's controller [CR#702.75a,406.3]. -/
theorem okHideaway : Ability.check (DRS.ofList []) (hideaway 4) = [] := by decide

/-- Converting is transforming written with another verb: one tree, so a "transforms into"
trigger and "can't transform" reach it [CR#701.27e,701.28a,701.28f]. -/
theorem convertIsTransform (subject : NounPhrase) :
    (convert subject).namedDeed = (transform subject).namedDeed := rfl

end Semantics.Proofs.KeywordHolders
