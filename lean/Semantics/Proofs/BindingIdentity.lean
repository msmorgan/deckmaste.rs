import Semantics.Definitions
import Semantics.Proofs.Helpers

/-!
# Semantics.Proofs.BindingIdentity

A Definition's or Template's referent parameter is bound at the body's first mention, and every later
mention refers back to that binding by its Binding Identity, not by a pronoun search
(`docs/decisions/semantics-v3.md` §§1.2,1.4). The `plugins_v3` loader writes the
first mention `firstMention id argument` and each later one `laterMention id argument`; the
terms below are written the way it writes them. The identity is a name, never a stack position:
other bindings pushed between the mentions do not change what a later mention reads. Each pin
is closed by `decide`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.BindingIdentity

/-- How many targets a context holds. -/
private def targets (bs : DRS) : Nat := bs.referents.countP (·.det == .target)

/-- "Tap <x>. Untap <x>.": a body that mentions its parameter twice. -/
private def tapThenUntap (x : NounPhrase) : Action :=
  .sequentially [tap (.firstMention "tapThenUntap.0" x), untap (.laterMention "tapThenUntap.0" x)]

/-- The same body with the argument copied into both mentions: two selections. -/
private def tapThenUntapCopied (x : NounPhrase) : Action :=
  .sequentially [tap x, untap x]

/-- The first creature is handed the fight; the Definition names both participants before
checking its creature guards, then reads those same identities in the opposite damage directions
[CR#701.14a,701.14b]. -/
private def fightByIdentity (l r : NounPhrase) : Action := instruct l (fight r)

/-! ## Two mentions, one binding -/

/-- "Tap target creature. Untap it.", spelled by identity, checks. -/
theorem okTwoMentionsByIdentity : Action.check (DRS.ofList []) (tapThenUntap (target isCreature)) = [] := by
  decide

/-- Both mentions are one binding: the body introduces one target. -/
theorem twoMentionsOneTarget : targets (actionResult (DRS.ofList []) (tapThenUntap (target isCreature))) = 1 := by
  decide

/-- Copying the argument into both mentions selects twice, which the identity spelling
does not. -/
theorem copiedArgumentTargetsTwice :
    targets (actionResult (DRS.ofList []) (tapThenUntapCopied (target isCreature))) = 2 := by
  decide

/-- The later mention reads the very binding the first mention introduced. -/
theorem laterMentionReadsTheFirstMention :
    let after := nounResult (DRS.ofList []) (.firstMention "m.0" (target isCreature)) .obj true
    (stepNounPhrase 64 after .obj true (.laterMention "m.0" (target isCreature))).2.1 == [0] := by
  decide

/-- A binding pushed between the mentions does not move the read: the later mention still
reads the artifact the first mention targeted, not the creature targeted after it. -/
theorem interveningBindingDoesNotShiftTheRead :
    let after := nounResult (nounResult (DRS.ofList []) (.firstMention "m.0" (target isArtifact)) .obj true) (target isCreature) .obj true
    (identityBinding "m.0" after).map (·.ty) =
      some [.artifact] := by
  decide

/-- Two calls of the same Definition each read their own first mention. -/
theorem eachCallReadsItsOwnBinding :
    Action.check (DRS.ofList [])
      (.sequentially [tapThenUntap (target isCreature), tapThenUntap (target isArtifact)]) = [] := by
  decide

/-- A later mention with no first mention, whose phrase would select anew, is refused rather
than read as a second selection. -/
theorem badLaterMentionWithoutFirst :
    Action.check (DRS.ofList []) (untap (.laterMention "m.0" (target isCreature))) =
      [.bindingIdentity "m.0"] := by
  decide

/-- A phrase that introduces nothing ("this creature") records nothing; its later mentions read
the phrase again. -/
theorem selfDenotingArgumentIsReadAgain :
    Action.check (DRS.ofList []) (tapThenUntap thisCreature) = [] := by
  decide

/-- An existing reference ("it") is recorded where it resolves and read back by identity. -/
theorem existingReferenceIsRecorded :
    Action.check (DRS.ofList [{ det := .the, plur := .one, payload := .object [.creature] (some .battlefield) none none none }])
      (tapThenUntap it) = [] := by
  decide

/-! ## Fight -/

/-- "Target creature fights target creature." by identity checks: the first creature handed the
deed, `instruct(target(creature), fight(target(creature)))`. -/
theorem okFightByIdentity :
    Action.check (DRS.ofList []) (fightByIdentity (target isCreature) (target isCreature)) = [] := by
  decide

/-- Each `target creature` is one targeting: the handoff selects the first once, and the body
mentions the second three times. -/
theorem fightTargetsEachArgumentOnce :
    targets (actionResult (DRS.ofList []) (fightByIdentity (target isCreature) (target isCreature))) = 2 := by
  decide

/-- "This creature fights target creature you don't control." -/
theorem okFightByIdentityThisCreature :
    Action.check (DRS.ofList []) (fightByIdentity thisCreature (target (and [isCreature, youDontControl]))) = [] := by
  decide

/-- A group fighting creature is refused by the singular power read and the per-member damage
check, as the bench's captured `fight` refuses it (`Damage.badFightGroup`). Handed the deed,
the first creature is the performer, so the group is the parameter. -/
theorem badFightByIdentityGroup :
    Action.check (DRS.ofList [])
      (fightByIdentity (target isCreature) (.described (.target (exactly 2)) isCreature)) =
      [.perMember, .singular] := by
  decide

/-- A group handed the deed distributes it, each member performing it in turn and `actor` that
member, as every group handoff does: "two target creatures each fight target creature". -/
theorem okFightHandedToEachOfAGroup :
    Action.check (DRS.ofList [])
      (fightByIdentity (.described (.target (exactly 2)) isCreature) (target isCreature)) = [] := by
  decide

/-! ## A group handoff still distributes -/

/-- "Discard <x>, then exile <x>.": a two-mention body. -/
private def discardThenExile (x : NounPhrase) : Action :=
  .sequentially
    [ discard (.firstMention "discardThenExile.0" x),
      exile (.laterMention "discardThenExile.0" x) ]

/-- "Each opponent discards a card, then exiles it": the first mention is checked where it
stands, inside the handoff, so each opponent chooses their own card. -/
theorem groupHandoffRefusesPreMoveIdentity :
    Action.check (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (discardThenExile (a .isCard))) =
      [.bindingIdentity "discardThenExile.0"] := by
  decide

/-- What the members' bodies introduced is published as a group, as for a one-mention body
(`Actor.handedGroupPublishesPlurals`): the choice is not hoisted out of the handoff. -/
theorem groupHandoffByIdentityPublishesPlurals :
    shape (actionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (discardThenExile (a .isCard)))) =
      [(.object, .many, .a), (.player, .many, .each)] := by
  decide


/-! ## Plurality read off the amount

"Scry N" looks at a library slice and sorts it [CR#701.22a]. The slice is the body's own
mention, read back by identity, so the read's number is the slice's, which is the amount's:
scry 1 reads "that card", scry 2 "those cards" (ruling 2026-10-06). -/

/-- The `plugins_v3` helper `lookAndSortSlice` as the loader expands it: the
`Semantics.Templates` template. -/
private def lookAndSortSlice (slice : NounPhrase) (spill : Location) (spillRiders : List MoveRider) :
    Action :=
  Semantics.Templates.lookAndSortSlice slice spill spillRiders

/-- The template is the expansion this file spelled by hand before it was generated. -/
theorem lookAndSortSlice_is_the_hand_expansion (slice : NounPhrase) (spill : Location)
    (spillRiders : List MoveRider) :
    lookAndSortSlice slice spill spillRiders =
      .sequentially
        [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
          .move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) .anyLocation (some spill) .arrival
            spillRiders,
          .move (.theRest .object .many) .anyLocation (some onTop) .arrival [(.inOrder .anyOrder)] ] :=
  rfl

/-- The plurality of the slice's later mention in a look-and-sort declaration: what "put any
number of them" reads back. -/
private def sliceReadBack : Action → Option Plurality
  | .ownScope _ (.sequentially [_, .move (.someOf _ _ readBack@(.laterMention _ _)) _ _ _ _, _]) =>
      some readBack.plur
  | _ => none

/-- "Scry 1." -/
theorem okScryOne : Action.check (DRS.ofList []) (scry 1) = [] := by decide

/-- "Scry 2." -/
theorem okScryTwo : Action.check (DRS.ofList []) (scry 2) = [] := by decide

/-- Scry 1's read back is singular: "that card". -/
theorem scryOneReadsThatCard : sliceReadBack (scry 1) = some .one := by decide

/-- Scry 2's read back is plural: "those cards". -/
theorem scryTwoReadsThoseCards : sliceReadBack (scry 2) = some .many := by decide

/-- "Surveil 2." [CR#701.25a] -/
theorem okSurveilTwo : Action.check (DRS.ofList []) (surveil 2) = [] := by decide

/-- "Fateseal 2." [CR#701.29a]: an opponent's library. -/
theorem okFatesealTwo : Action.check (DRS.ofList []) (fateseal 2) = [] := by decide

/-! ## Regenerate

"Regenerate <permanent>" [CR#701.19a]: the next time it would be destroyed this turn, instead
remove all damage marked on it and its controller taps it; if it's an attacking or blocking
creature, remove it from combat. The `plugins_v3` declaration binds the permanent where the
replacement's event first mentions it, and the application (`regeneration`) binds
its own subject at its first mention, read back by identity in the other five. -/

private def regenerationApplicationBy := regeneration
private def regenerateBy := regenerate

/-- "Regenerate target creature." -/
theorem okRegenerateTarget : Action.check (DRS.ofList []) (regenerateBy (target isCreature)) = [] := by decide

/-- "Regenerate this creature." -/
theorem okRegenerateThisCreature : Action.check (DRS.ofList []) (regenerateBy thisCreature) = [] := by decide

/-- The replacement pattern selects its target once; the application keeps that identity. -/
theorem regenerateTargetsOnce :
    (match (regenerateBy (target isCreature)).scopeBody with
      | .establish (.replace event (.instead body) _ _ _) _ =>
        let context := (stepWouldClause 64 (DRS.ofList []) event).1
        targets (actionResult context body)
      | _ => 0) = 1 := by decide

theorem regenerationPatternDoesNotLeakIntoFollowingText :
    targets (actionResult (DRS.ofList []) (regenerateBy (target isCreature))) = 0 := by decide

/-- The application alone, as a static regeneration ability applies it [CR#701.19b]. -/
theorem okRegenerationApplication :
    Action.check (DRS.ofList []) (regenerationApplicationBy (target isCreature)) = [] := by decide
/-! ## Airbend, card by card

"Airbend [objects]" [CR#701.65a]: the performer exiles them, and for each card exiled this way,
for as long as it remains exiled, its owner may cast it by paying {2} rather than its mana cost.
The per-card spelling binds the objects where it exiles them and loops over each card exiled
this way, handing each card's permission to that card's own owner. The exile makes them new
objects [CR#400.7,400.7j], so the loop reads them back by anaphora, never by the parameter
(`docs/decisions/semantics-v3.md` §§0.1,1.3). The loop's singular referent remains
available through the handoff to its owner. -/

private def airbendByCard := airbend

theorem okAirbendByCardUpToTwo :
    Action.check (DRS.ofList [])
      (airbendByCard (.described (.target (upTo 2)) isCreature)) = [] := by decide

-- The distributive loop now outranks the outer singular referent under D′.
theorem okAirbendItOne :
    Action.check (DRS.ofList []) (airbendByCard (target isCreature)) = [] := by decide

end Semantics.Proofs.BindingIdentity
