import Semantics.Cards.Deontic
import Semantics.Cards.Keyword
import Semantics.Proofs.Helpers

/-!
# Semantics.Proofs.Retention

"This effect doesn't remove …": `StaticSpec.retention`, an exemption from the one state-based
consequence of a clause that removes Attachments already on its host
[CR#702.16n,702.16p,303.4c,704.5m], and its two laws: the clause is one that removes Attachments
(`retentionClause`), and the subject names Attachments of that clause's host (`retentionSubject`).
Applying the exemption is the engine's; these pins are on the terms.

The last section pins "protection from each …" (`Predicate.eachKind`), one protection ability per
value [CR#702.16h,702.16i], and the exemption that reaches each of them (*Spectra Ward*, *Pledge
of Loyalty*).
-/

open Semantics Semantics.Templates Semantics.Definitions Semantics.Cards

namespace Semantics.Proofs.Retention

/-- "Enchanted creature has protection from black." -/
private def protectionFromBlack : StaticSpec :=
  abilityGrant (enchanted creature) (protection (hasColor .black))

/-- The first `retention` a single-faced card's text writes. -/
private def retentionOf? (card : Spelled) : Option StaticSpec :=
  match card.card with
  | .singleFaced face => face.characteristics.text.findSome? fun
    | .static (.retention spec subject) => some (.retention spec subject)
    | _ => none
  | _ => none

/-- The shapes the laws admit: "this Aura" on the enchanted creature (*Black Ward*), "this
Equipment" on the equipped creature, "Auras" (*Spectra Ward*'s sentence [CR#702.16n]), and
Auras attached to the host read back by "it" (*Benevolent Blessing* [CR#702.16p]). -/
theorem retentionAdmitsItsShapes :
    StaticSpec.check emptyContext (retention protectionFromBlack thisAura) = [] ∧
    StaticSpec.check emptyContext
      (retention (abilityGrant (equipped creature) (protection (hasColor .black))) thisEquipment)
      = [] ∧
    StaticSpec.check emptyContext (retention protectionFromBlack (allOf aura)) = [] ∧
    StaticSpec.check emptyContext
      (retention protectionFromBlack (allOf (and [aura, attachedTo it]))) = [] := by decide

/-- The 3a law (owner ruling 2026-10-22): a retention whose clause removes no Attachment is
refused (`retentionClause`), as "gets +1/+1" and "has flying" remove none; one whose subject is
no Attachment of the clause's host is refused (`retentionSubject`): "this creature", "creatures",
"Auras attached to target creature" (a fresh host, not the clause's), and "this Equipment" for
an enchanted creature. -/
theorem retentionLawRefusals :
    StaticSpec.check emptyContext (retention (getsPt (enchanted creature) (up 1) (up 1)) thisAura)
      = [.retentionClause] ∧
    StaticSpec.check emptyContext (retention (abilityGrant (enchanted creature) flying) thisAura)
      = [.retentionClause] ∧
    StaticSpec.check emptyContext (retention protectionFromBlack thisCreature)
      = [.retentionSubject] ∧
    StaticSpec.check emptyContext (retention protectionFromBlack (allOf isCreature))
      = [.retentionSubject] ∧
    StaticSpec.check emptyContext
      (retention protectionFromBlack (allOf (and [aura, attachedTo (target isCreature)])))
      = [.retentionSubject] ∧
    StaticSpec.check emptyContext (retention protectionFromBlack thisEquipment)
      = [.retentionSubject] := by decide

/-- "If the creature has other instances of protection from the same quality, those instances
affect Auras as normal" [CR#702.16n]. The exemption belongs to the clause it wraps: a second
grant of protection from black, written as its own clause, carries none and removes Attachments
in its own right. Which Aura goes to the graveyard is the engine's to apply. -/
theorem exemptionDoesNotCoverOtherProtection :
    retentionOf? blackWard = some (retention protectionFromBlack thisAura) ∧
    StaticSpec.check emptyContext
      (conjunction [retention protectionFromBlack thisAura, protectionFromBlack]) = [] ∧
    protectionFromBlack.removesAttachments = true :=
  ⟨rfl, by decide, by decide⟩

/-- "Already attached to" is a point in time [CR#702.16p] (owner rulings 2026-10-22, 3c): the
Attachments attached when the protection effect starts to apply are exempt, and other permanents
with the quality can't become attached afterwards. *Benevolent Blessing*'s subject is a
description of the host's Attachments ("attached to it"), told apart from *Black Ward*'s "this
Aura" by the `attachedTo` it carries; the engine reads it once, as the effect starts. An Aura
attached after the effect began is not exempt: the exemption leaves its clause whole, and that
clause forbids such an Aura to become attached at all. -/
theorem alreadyAttachedIsReadWhenTheEffectStarts :
    (match retentionOf? benevolentBlessing with
     | some (.retention spec (.described _ p)) =>
       spec.removesAttachments &&
         (match spec.attachmentHost?, p.attachmentHosts with
          | some (.attachHost .enchanted _), [.pro .bare .one] => true
          | _, _ => false)
     | _ => false) = true ∧
    (match retentionOf? blackWard with
     | some (.retention _ (.headed (.subtype _) .this)) => true
     | _ => false) = true := by decide

/-- The exemption's referent reads the effect's own host, not an earlier mention: after
"artifacts your opponents control" (referent 0), *Guardian Beast*'s "those artifacts" reads the
artifacts the effect governs, its clauses' shared subject. -/
theorem exemptionReadsTheEffectsOwnHost :
    let before := (stepNounPhrase 64 emptyContext .other false
      (allOf (and [isArtifact, opponentsControl]))).1
    let governed := (stepNounPhrase 64 before .subj true
      (bare (and [non isCreature, isArtifact, youControl]))).2.1
    let out := (stepStaticSpec 64 before guardianBeastProtection).1
    (resolveCandidates (.word (.type .artifact)) .many out).map (·.id) = governed ∧
      governed ≠ [0] := by
  decide

/-! ## "Protection from each …": one ability per value -/

/-- The first `retention` a single-faced card's static abilities write, a member of a conjunction
included. -/
private def memberRetentionOf? (card : Spelled) : Option StaticSpec :=
  match card.card with
  | .singleFaced face => face.characteristics.text.findSome? fun
    | .static (.retention spec subject) => some (.retention spec subject)
    | .static (.conjunction _ parts) => parts.findSome? fun
      | .retention spec subject => some (.retention spec subject)
      | _ => none
    | _ => none
  | _ => none

/-- The clause a retention exempts from removing Attachments. -/
private def exemptClause? : Option StaticSpec → Option StaticSpec
  | some (.retention spec _) => some spec
  | _ => none

/-- The abilities a grant adds. -/
private def grantedAbilities : StaticSpec → List Ability
  | .characteristicChange _ edits => edits.flatMap fun
    | .addedAbilities abilities => abilities
    | _ => []
  | _ => []

/-- The five colors [CR#105.1], each "protection from" it. -/
private def protectionFromEachOfFiveColors : List (KeywordLabel × Predicate) :=
  [Color.white, .blue, .black, .red, .green].map fun color => (.named "protection", hasColor color)

/-- "Protection from each color" is five protection abilities, each from a different color
[CR#702.16h,105.1]. -/
theorem protectionFromEachColorIsFiveAbilities :
    (protection (eachKind .color)).keywordInstances? = some protectionFromEachOfFiveColors ∧
    [Color.white, .blue, .black, .red, .green].eraseDups.length = 5 ∧
    Ability.check emptyContext (protection (eachKind .color)) = [] :=
  ⟨rfl, by decide, by decide⟩

/-- *Spectra Ward*: "Enchanted creature gets +2/+2 and has protection from each color. This effect
doesn't remove Auras." The exemption is the grant's conjunction member, beside `getsPt`; it
wraps the grant as a whole, the one protection ability whose quality is "each color", so it
reaches each of the five abilities that grant is ("this effect" [CR#702.16h,702.16n]). -/
theorem spectraWardExemptsAurasFromEachColor :
    memberRetentionOf? spectraWard =
      some (retention (abilityGrant (enchanted creature) (protection (eachKind .color)))
        (allOf aura)) ∧
    (exemptClause? (memberRetentionOf? spectraWard)).map
        (fun spec => (grantedAbilities spec).flatMap (·.keywordInstances?.getD [])) =
      some protectionFromEachOfFiveColors ∧
    ((exemptClause? (memberRetentionOf? spectraWard)).map (·.removesAttachments)).getD false
      = true :=
  ⟨rfl, rfl, by decide⟩

/-- *Pledge of Loyalty*: "Enchanted creature has protection from each color among permanents you
control. This effect doesn't remove this Aura." The set of colors is a description of the
permanents you control, not a list fixed when the effect begins: a static ability's effect
applies at each moment to whatever its text indicates [CR#611.3a,702.16i], so the bench names no
fixed instances and the engine reads the colors as it applies. The exemption wraps the whole
grant. -/
theorem pledgeOfLoyaltyReadsItsColorsContinuously :
    memberRetentionOf? pledgeOfLoyalty =
      some (retention
        (abilityGrant (enchanted creature)
          (protection (eachKind .color (some (allOf (and [isPermanent, youControl]))))))
        thisAura) ∧
    (match exemptClause? (memberRetentionOf? pledgeOfLoyalty) with
     | some spec =>
       spec.removesAttachments &&
         (grantedAbilities spec).all (fun ability =>
           ability.keywordInstances?.isNone &&
             match ability with
             | .keyword _ [.quality q] _ =>
               match q.eachKindOf? with
               | some (.color, some (.described .all _)) => true
               | _ => false
             | _ => false)
     | none => false) = true :=
  ⟨rfl, by decide⟩

/-- "Each <value>" stands only as the whole quality of a keyword (`eachKindPlacement`): not
conjoined into one ("protection from each color creature"), nor as an object description ("each
color" as the creatures a grant reaches). Its axis may be open, with a domain or without, as
"each possible quality the listed characteristic could have" is any characteristic's
[CR#702.16h,702.16i]: "protection from each mana value among artifacts you control". -/
theorem eachKindLaws :
    (Ability.check emptyContext (protection (and [eachKind .color, isCreature]))).eraseDups =
      [.eachKindPlacement] ∧
    StaticSpec.check emptyContext (abilityGrant (allOf (eachKind .color)) flying)
      = [.eachKindPlacement] ∧
    Ability.check emptyContext
      (protection (eachKind (.value .manaValue) (some (allOf (and [isArtifact, youControl])))))
      = [] ∧
    Ability.check emptyContext (protection (eachKind (.subtype .creature .any))) = [] := by
  decide

end Semantics.Proofs.Retention
