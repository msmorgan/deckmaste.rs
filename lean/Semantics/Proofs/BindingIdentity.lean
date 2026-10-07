import Semantics.Macros

/-!
# Semantics.Proofs.BindingIdentity

A registry macro's referent parameter is bound at the body's first mention, and every later
mention refers back to that binding by its Binding Identity, not by a pronoun search
(`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06). The `plugins_v2` loader writes the
first mention `firstMention id argument` and each later one `laterMention id argument`; the
terms below are written the way it writes them. The identity is a name, never a stack position:
other bindings pushed between the mentions do not change what a later mention reads. Each pin
is closed by `decide`.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.BindingIdentity

/-- Kind, number and determiner of each binding, newest first. -/
private def shape (bs : Bindings) : List (Kind × Plurality × Determiner) :=
  bs.map fun b => (b.kind, b.plur, b.det)

/-- How many targets a context holds. -/
private def targets (bs : Bindings) : Nat := bs.countP (·.det == .target)

/-- "Tap <x>. Untap <x>.": a body that mentions its parameter twice. -/
private def tapThenUntap (x : NounPhrase) : Instruction :=
  .sequentially [tap (.firstMention "tapThenUntap.0" x), untap (.laterMention "tapThenUntap.0" x)]

/-- The same body with the argument copied into both mentions: two selections. -/
private def tapThenUntapCopied (x : NounPhrase) : Instruction :=
  .sequentially [tap x, untap x]

/-- Fight as the loader writes the `plugins_v2` body: each subject bound where the first damage
event first mentions it, and read back by identity everywhere after [CR#701.14a]. The rule that
a creature no longer on the battlefield deals no fight damage [CR#701.14b] is the rules' to
apply, not a guard in the text: a guard's condition may not introduce its subject (`testSubject`),
so it cannot carry a first mention. -/
private def fightByIdentity (l r : NounPhrase) : Instruction :=
  let left := NounPhrase.laterMention "fight.0" l
  let right := NounPhrase.laterMention "fight.1" r
  .enact (.action "Fight") (.simultaneously [
    .dealDamage (.firstMention "fight.0" l) (.statOf (.stat .power) left) (.firstMention "fight.1" r),
    .dealDamage right (.statOf (.stat .power) right) left])

/-! ## Two mentions, one binding -/

/-- "Tap target creature. Untap it.", spelled by identity, checks. -/
theorem okTwoMentionsByIdentity : Instruction.check [] (tapThenUntap (target creature)) = [] := by
  decide

/-- Both mentions are one binding: the body introduces one target. -/
theorem twoMentionsOneTarget : targets ((tapThenUntap (target creature)).intro []) = 1 := by
  decide

/-- Copying the argument into both mentions selects twice, which the identity spelling
does not. -/
theorem copiedArgumentTargetsTwice :
    targets ((tapThenUntapCopied (target creature)).intro []) = 2 := by
  decide

/-- The later mention reads the very binding the first mention introduced. -/
theorem laterMentionReadsTheFirstMention :
    let after := nomIntro [] (.firstMention "m.0" (target creature))
    (NounPhrase.result after (.laterMention "m.0" (target creature))).value == after.head? := by
  decide

/-- A binding pushed between the mentions does not move the read: the later mention still
reads the artifact the first mention targeted, not the creature targeted after it. -/
theorem interveningBindingDoesNotShiftTheRead :
    let after := nomIntro (nomIntro [] (.firstMention "m.0" (target artifact))) (target creature)
    ((NounPhrase.result after (.laterMention "m.0" (target artifact))).value.map (·.ty)) =
      some [.artifact] := by
  decide

/-- Two calls of the same macro each read their own first mention. -/
theorem eachCallReadsItsOwnBinding :
    Instruction.check []
      (.sequentially [tapThenUntap (target creature), tapThenUntap (target artifact)]) = [] := by
  decide

/-- A later mention with no first mention, whose phrase would select anew, is refused rather
than read as a second selection. -/
theorem badLaterMentionWithoutFirst :
    Instruction.check [] (untap (.laterMention "m.0" (target creature))) =
      [.bindingIdentity "m.0"] := by
  decide

/-- A phrase that introduces nothing ("this creature") records nothing; its later mentions read
the phrase again. -/
theorem selfDenotingArgumentIsReadAgain :
    Instruction.check [] (tapThenUntap thisCreature) = [] := by
  decide

/-- An existing reference ("it") is recorded where it resolves and read back by identity. -/
theorem existingReferenceIsRecorded :
    Instruction.check [⟨.the, .one, .object [.creature] (some .battlefield) none none none, []⟩]
      (tapThenUntap it) = [] := by
  decide

/-! ## Fight -/

/-- "Target creature fights target creature." by identity checks. -/
theorem okFightByIdentity :
    Instruction.check [] (fightByIdentity (target creature) (target creature)) = [] := by
  decide

/-- Each `target creature` argument is one targeting, though the body mentions it four times. -/
theorem fightTargetsEachArgumentOnce :
    targets ((fightByIdentity (target creature) (target creature)).intro []) = 2 := by
  decide

/-- "This creature fights target creature you don't control." -/
theorem okFightByIdentityThisCreature :
    Instruction.check [] (fightByIdentity thisCreature (target creatureYouDontControl)) = [] := by
  decide

/-- A group subject is refused by the singular power read and the per-member damage check, as
the bench's captured `fight` refuses it (`Damage.badFightGroup`). -/
theorem badFightByIdentityGroup :
    Instruction.check []
      (fightByIdentity (.described (.target (exactly 2)) creature) (target creature)) =
      [.singular, .perMember] := by
  decide

/-! ## A group handoff still distributes -/

/-- "Discard <x>, then exile <x>.": a two-mention body. -/
private def discardThenExile (x : NounPhrase) : Instruction :=
  .sequentially
    [ Actor.discard (.firstMention "discardThenExile.0" x),
      exile (.laterMention "discardThenExile.0" x) ]

/-- "Each opponent discards a card, then exiles it": the first mention is checked where it
stands, inside the handoff, so each opponent chooses their own card. -/
theorem okGroupHandoffByIdentity :
    Instruction.check [] (act (each .opponent) (discardThenExile (a .isCard))) = [] := by
  decide

/-- What the members' bodies introduced is published as a group, as for a one-mention body
(`Actor.handedGroupPublishesPlurals`): the choice is not hoisted out of the handoff. -/
theorem groupHandoffByIdentityPublishesPlurals :
    shape (Instruction.intro [] (act (each .opponent) (discardThenExile (a .isCard)))) =
      [(.object, .many, .a), (.player, .many, .each)] := by
  decide


end Semantics.Proofs.BindingIdentity
