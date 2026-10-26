import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Attachments

private def nounZone (drs : DRS) (noun : NounPhrase) : Option Zone :=
  let result := stepNounPhrase 64 drs .other false noun
  result.2.1.head? >>= fun id => (result.1.findById? id).bind (·.zone)

private def nounTypes (drs : DRS) (noun : NounPhrase) : List CardType :=
  let result := stepNounPhrase 64 drs .other false noun
  ((result.2.1.head? >>= result.1.findById?).map (·.ty)).getD []

/-- The enchanted-player domain used by Curse of Exhaustion is also a valid host predicate. -/
theorem okEnchantedPlayer :
    Condition.check (DRS.ofList []) (.matches .you (.attachment .host (some .enchanted) none)) = [] := by decide

theorem okEnchantedPlayerWithModifierFirst :
    NounPhrase.checkAt (some .player) (DRS.ofList [])
      (a (.and [.attachment .host (some .enchanted) none, .isPlayer])) = [] := by decide

theorem okEnchantedPlayerHasNoZone :
    nounZone (DRS.ofList []) (a (.and [.isPlayer, .attachment .host (some .enchanted) none])) = none := by rfl

theorem badEquippedPlayer :
    Condition.check (DRS.ofList []) (.matches .you (.attachment .host (some .equipped) none)) =
      [.kindLte .player .object] := by decide

theorem badFortifiedPlayer :
    Predicate.checkAt .player (DRS.ofList []) (.attachment .host (some .fortified) none) =
      [.kindLte .player .object] := by decide

theorem okPlayerHostWithNoWord :
    Predicate.checkAt .player (DRS.ofList []) (.attachment .host none none) = [] := by decide

theorem badPlayerAsAttachment :
    Predicate.checkAt .player (DRS.ofList []) (.attachment .attachment none none) =
      [.kindMismatch .player .object] := by decide

theorem okAuraAttachedToPlayer :
    Condition.check (DRS.ofList []) (.matches thisAura (.attachment .attachment (some .enchanted) (some .you))) =
      [] := by decide

theorem badEquipmentAttachedToPlayer :
    Predicate.checkAt .object (DRS.ofList []) (.attachment .attachment (some .equipped) (some .you)) =
      [.kindLte .player .object] := by decide

theorem okCountedAuraAttachments :
    Condition.check (DRS.ofList []) (.matches thisCreature (.attachment .host (some .enchanted)
      (some (counted (orMore 2) (.hasSubtype (aura)))))) = [] := by decide

theorem badPlayerCounterpartOnHostSide :
    Predicate.checkAt .object (DRS.ofList []) (.attachment .host none (some .you)) =
      [.kindMismatch .object .player] := by decide

theorem okAttachedHostRetainsCreatureType :
    nounTypes (DRS.ofList []) (a (.and [isCreature, .attachment .host (some .equipped) none])) =
      [.creature] := by rfl

theorem okNestedCounterpartTargetRemainsAvailable :
    Action.check (DRS.ofList []) (.sequentially [
      exile (target (.and [isArtifact, .attachment .attachment none (some (target isCreature))])),
      .setStatus .tapped (that (.type .creature))]) = [] := by decide

/-- The explicit source mention remains available after the attached artifact is exiled, to a
later "this creature"; "that creature" never reads the Source, a self-referent
(`Determiner.self`; owner ruling 2026-10-24). -/
theorem sourceCounterpartRemainsAvailable :
    Action.check (DRS.ofList []) (.sequentially [
      exile (target (.and [isArtifact, .attachment .attachment none (some thisCreature)])),
      .setStatus .tapped thisCreature]) = [] ∧
    Action.check (DRS.ofList []) (.sequentially [
      exile (target (.and [isArtifact, .attachment .attachment none (some thisCreature)])),
      .setStatus .tapped (that (.type .creature))]) =
      [.anaphor (.word (.type .creature)) .one 0] := by decide

private def sourceCounterpartAfterExile : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList [])
    (exile (target (.and [isArtifact, .attachment .attachment none (some thisCreature)])))).1

/-- A later "this creature" re-mentions the Source's referent, on the battlefield as a creature
(`mentionThis`); no demonstrative selects it. -/
theorem sourceCounterpartReMentionSelectsTheSourceIdentity :
    let context := nextClause sourceCounterpartAfterExile
    let (after, ids, errors) := stepNounPhrase 64 context .obj true thisCreature
    errors = [] ∧
      ids = (stepNounPhrase 64 (DRS.ofList []) .other false thisCreature).2.1 ∧
      (ids.filterMap after.findById?).all
        (fun ref => ref.zone == some .battlefield && ref.ty.contains .creature) ∧
      resolveCandidates (.word (.type .creature)) .one context = [] := by decide

theorem isTransformedTemplateReadsWholeBackFace :
    isTransformed = Predicate.currentFace .back := by rfl

theorem frontAndBackAreDistinct :
    Predicate.currentFace .front ≠ Predicate.currentFace .back := by
  intro equal
  cases equal

theorem currentFaceIsNotFaceDownStatus :
    Predicate.currentFace .back ≠ Predicate.hasStatus .faceDown := by
  intro equal
  cases equal

theorem badCurrentFaceOnPlayer :
    Predicate.checkAt .player (DRS.ofList []) (.currentFace .back) = [.kindMismatch .player .object] := by decide

theorem currentFaceDoesNotSupplyTokenEvidence :
    Predicate.seedsToken (.currentFace .back) = false := by rfl

/-! ## The attach deed's host role (ruling 2026-10-10) -/

/-- "This Equipment may be attached to a creature" [CR#301.5,701.3a]. -/
theorem okEquipmentMayBeAttachedToACreature :
    StaticSpec.check (DRS.ofList []) (mayBeAttachedTo thisEquipment (allOf isCreature)) = [] := by decide

/-- An Aura may be attached to a player [CR#702.5d]: the host role admits either kind. -/
theorem okAuraMayBeAttachedToAPlayer :
    StaticSpec.check (DRS.ofList []) (mayBeAttachedTo thisAura (allOf .isPlayer)) = [] := by decide

theorem passiveAgentAndAttachmentHostAreDistinct :
    (.by (allOf isCreature) : Subject).patientRole = .subj ∧
      mayBeAttachedTo thisEquipment (allOf isCreature) =
        .can .any (attachTo thisEquipment (allOf isCreature))
          {} := by
  constructor <;> rfl

/-! ## The chapter axis (ruling 2026-10-10) -/

/-- "This Saga's final chapter number" folds one object's chapter numbers [CR#714.2d]. -/
theorem okFinalChapterNumber :
    Amount.check (DRS.ofList []) (finalChapterNumber thisSaga) = [] := by decide

/-- A single-valued axis still folds only a group. -/
theorem badGreatestPowerOfOne :
    Amount.check (DRS.ofList []) (.aggregate .max (.stat .power) thisCreature) = [.plural] := by decide

/-- "A chapter ability of this Saga whose chapter number is its lore counter count"
[CR#702.155a]. -/
theorem okChapterAbilityAtLoreCount :
    NounPhrase.checkAt none (DRS.ofList [])
      (allOf (.and [chapterAbilityOf thisSaga,
        .compare [.chapter] .eq (countersOn (.named "loreCounter") thisSaga)])) = [] := by decide

/-- Read ahead's body as its declaration writes it: chapter abilities of this Saga whose chapter
number is not its lore counter count can't trigger the turn it entered [CR#702.155a]. -/
theorem okReadAhead : Ability.check (DRS.ofList []) readAhead = [] := by decide

/-- Enchant's body as its declaration writes it, here over "enchant creature": the Aura spell's
target, the legal-host permission, and the entry clause [CR#702.5a,303.4a,303.4c,303.4f]. -/
theorem okEnchantCreature : Ability.check (DRS.ofList []) (enchant isCreature) = [] := by decide

/-- An Aura may enchant a player [CR#702.5d]: "enchant player". -/
theorem okEnchantPlayer : Ability.check (DRS.ofList []) (enchant .isPlayer) = [] := by decide

/-! ## An enchanted card is not on the battlefield

The battlefield default is for a description without "card", "spell", "source" or "scheme"
[CR#109.2]: "enchanted creature" is a permanent, "enchanted creature card" is a card wherever its
enchant ability puts it, as Animate Dead's and Spellweaver Volute's do ("enchant creature card in
a graveyard"; the card Volute enchants "won't be on the battlefield", ruling 2007-05-01). -/

/-- "enchanted creature" is on the battlefield; "enchanted creature card" is in no zone the
checker fixes. -/
theorem enchantedCardIsNotOnTheBattlefield :
    [nounZone (DRS.ofList []) (enchanted (.ofType .card .creature)),
      nounZone (DRS.ofList []) (enchanted creature)]
      = [none, some .battlefield] := by
  decide

/-- Animate Dead: "Return enchanted creature card to the battlefield under your control and attach
this Aura to it." -/
theorem okAnimateDeadReturn :
    Action.check (DRS.ofList [])
      (sequentially
        [ returnToBattlefield (enchanted (.ofType .card .creature)) [under you],
          attachTo thisAura it ]) = [] := by
  decide

/-- Spellweaver Volute: "exile the enchanted card". The card leaves a graveyard, not the
battlefield, so "the exiled instant", which names a permanent by what it was [CR#608.2h], does
not read it back, while "the exiled instant card" does. -/
theorem voluteExilesACardNotAPermanent :
    let exileEnchanted := exile (enchanted (.ofType .card .instant))
    Action.check (DRS.ofList [])
        (sequentially [exileEnchanted, returnTo (the (and [was (exile which), isInstant, isCard]))
          graveyard []]) = [] ∧
      Action.check (DRS.ofList [])
        (sequentially [exileEnchanted, returnTo (the (and [was (exile which), isInstant]))
          graveyard []]) = [.participialDescription (.deed (.action "exile")) 0] := by
  decide

/-! ## Protection's attach rows -/

/-- Protection from black, its five rows as its declaration writes them: can't be targeted by
black spells or abilities from black sources, can't be enchanted by black Auras, equipped by black
Equipment or fortified by black Fortifications, damage from black sources prevented, and can't be
blocked by black creatures [CR#702.16b..702.16f]. The attach rows name this permanent in the
attach deed's host role. -/
theorem okProtectionFromBlack :
    Ability.check (DRS.ofList []) (protection (.colorIs .black)) = [] := by decide

/-- "Black Auras can't be attached to this permanent": the attachment in the patient role, this
permanent in the host role [CR#701.3a,702.16c]. -/
theorem okCantBeAttachedTo :
    StaticSpec.check (DRS.ofList [])
      (cant any (attachTo (allOf (.and [isEnchantment, .colorIs .black])) thisPermanent))
      = [] := by decide

end Semantics.Proofs.Attachments
