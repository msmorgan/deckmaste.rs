import Semantics.Cards.Deontic
import Semantics.Cards.Keyword

/-!
# Semantics.Proofs.Retention

"This effect doesn't remove …": `StaticSpec.retention`, an exemption from the one state-based
consequence of a clause that removes Attachments already on its host
[CR#702.16n,702.16p,303.4c,704.5m], and its two laws: the clause is one that removes Attachments
(`retentionClause`), and the subject names Attachments of that clause's host (`retentionSubject`).
Applying the exemption is the engine's; these pins are on the terms.
-/

open Semantics Semantics.Templates Semantics.Definitions Semantics.Cards

namespace Semantics.Proofs.Retention

private def emptyContext : DRS := DRS.ofList []

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

end Semantics.Proofs.Retention
