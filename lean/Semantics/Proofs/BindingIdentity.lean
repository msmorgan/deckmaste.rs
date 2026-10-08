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

/-- Fight as the loader writes the `plugins_v2` body: the first creature is the performer, handed
the deed (`act(<first>, fight(<second>))`), and is `actor` inside it; the second, the parameter,
is bound where the first damage event first mentions it and read back by identity after
[CR#701.14a]. The rule that a creature no longer on the battlefield deals no fight damage
[CR#701.14b] is the rules' to apply, not a guard in the text: a guard's condition may not
introduce its subject (`testSubject`), so it cannot carry a first mention. -/
private def fightByIdentity (l r : NounPhrase) : Instruction :=
  let right := NounPhrase.laterMention "fight.0" r
  act l (.enact (.action "fight") (.simultaneously [
    .dealDamage actor (.statOf (.stat .power) actor) (.firstMention "fight.0" r),
    .dealDamage right (.statOf (.stat .power) right) actor]))

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

/-- "Target creature fights target creature." by identity checks: the first creature handed the
deed, `act(target(creature), fight(target(creature)))`. -/
theorem okFightByIdentity :
    Instruction.check [] (fightByIdentity (target creature) (target creature)) = [] := by
  decide

/-- Each `target creature` is one targeting: the handoff selects the first once, and the body
mentions the second three times. -/
theorem fightTargetsEachArgumentOnce :
    targets ((fightByIdentity (target creature) (target creature)).intro []) = 2 := by
  decide

/-- "This creature fights target creature you don't control." -/
theorem okFightByIdentityThisCreature :
    Instruction.check [] (fightByIdentity thisCreature (target creatureYouDontControl)) = [] := by
  decide

/-- A group fighting creature is refused by the singular power read and the per-member damage
check, as the bench's captured `fight` refuses it (`Damage.badFightGroup`). Handed the deed,
the first creature is the performer, so the group is the parameter. -/
theorem badFightByIdentityGroup :
    Instruction.check []
      (fightByIdentity (target creature) (.described (.target (exactly 2)) creature)) =
      [.perMember, .singular] := by
  decide

/-- A group handed the deed distributes it, each member performing it in turn and `actor` that
member, as every group handoff does: "two target creatures each fight target creature". -/
theorem okFightHandedToEachOfAGroup :
    Instruction.check []
      (fightByIdentity (.described (.target (exactly 2)) creature) (target creature)) = [] := by
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


/-! ## Plurality read off the amount

"Scry N" looks at a library slice and sorts it [CR#701.22a]. The slice is the body's own
mention, read back by identity, so the read's number is the slice's, which is the amount's:
scry 1 reads "that card", scry 2 "those cards" (ruling 2026-10-06). -/

/-- The `plugins_v2` helper `lookAndSortSlice` as the loader expands it. -/
private def lookAndSortSlice (slice : NounPhrase) (spill : ZoneExpr) (spillRiders : List TokenRider) :
    Instruction :=
  .sequentially
    [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
      .move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) .wherever spill
        spillRiders,
      .move (.theRest .object .many) .wherever onTop [.inOrder .anyOrder] ]

private def scryBy (amount : Amount) : Instruction :=
  .enact (.action "scry") (lookAndSortSlice (.slice .top amount (library .actor)) onBottom [.inOrder .anyOrder])

/-- "Scry 1." -/
theorem okScryOne : Instruction.check [] (scryBy (.lit 1)) = [] := by decide

/-- "Scry 2." -/
theorem okScryTwo : Instruction.check [] (scryBy (.lit 2)) = [] := by decide

/-- Scry 1's read back is singular: "that card". -/
theorem scryOneReadsThatCard :
    (NounPhrase.laterMention "lookAndSortSlice.slice" (.slice .top (.lit 1) (library .actor))).plur
      = .one := by decide

/-- Scry 2's read back is plural: "those cards". -/
theorem scryTwoReadsThoseCards :
    (NounPhrase.laterMention "lookAndSortSlice.slice" (.slice .top (.lit 2) (library .actor))).plur
      = .many := by decide

/-- "Surveil 2." [CR#701.25a] -/
theorem okSurveilTwo :
    Instruction.check []
      (.enact (.action "surveil") (lookAndSortSlice (.slice .top (.lit 2) (library .actor)) graveyard []))
      = [] := by decide

/-- "Fateseal 2." [CR#701.29a]: an opponent's library. -/
theorem okFatesealTwo :
    Instruction.check []
      (.enact (.action "fateseal")
        (lookAndSortSlice (.slice .top (.lit 2) (library (a .opponent))) onBottom
          [.inOrder .anyOrder]))
      = [] := by decide

/-! ## Connive

"Connive N" [CR#701.50a,701.50d]: the permanent's controller draws N cards, discards N cards,
and puts a +1/+1 counter on the permanent for each nonland card discarded this way. The
`plugins_v2` helper `discardThenForEachNonland` reads the discarded cards back by identity. -/

private def conniveDiscards (discarded : NounPhrase) : Instruction :=
  .sequentially
    [ Actor.discard (.firstMention "conniveDiscards.discarded" discarded),
      .doForEach (allFromAmong (.not land) (.laterMention "conniveDiscards.discarded" discarded))
        (.putCounters (.lit 1) (.printed p1p1Counter) (that .permanent)) ]

private def conniveBy (amount : Amount) : Instruction :=
  .enact (.action "connive") (act (controllerOf .actor)
    (.sequentially [draw amount, conniveDiscards (counted (.exactlyOf amount) .isCard)]))

/-- "Connive" (connive 1), performed by the source permanent [CR#701.50a]. -/
theorem okConniveOne : Instruction.check [] (conniveBy (.lit 1)) = [] := by decide

/-- "Target creature connives 2." [CR#701.50d]: the action is handed to the creature. -/
theorem okConniveTwoHandedToTarget :
    Instruction.check [] (act (target creature) (conniveBy (.lit 2))) = [] := by decide

/-- The discarded cards are read back with the discard's number: one card for connive 1,
several for connive 2. -/
theorem conniveReadsTheDiscardsNumber :
    ((NounPhrase.laterMention "d" (counted (.exactlyOf (.lit 1)) .isCard)).plur,
     (NounPhrase.laterMention "d" (counted (.exactlyOf (.lit 2)) .isCard)).plur) =
      (.one, .many) := by decide

/-! ## Regenerate

"Regenerate <permanent>" [CR#701.19a]: the next time it would be destroyed this turn, instead
remove all damage marked on it and its controller taps it; if it's an attacking or blocking
creature, remove it from combat. The `plugins_v2` declaration binds the permanent where the
replacement's event first mentions it, and the application (`regenerationApplication`) binds
its own subject at its first mention, read back by identity in the other five. -/

private def regenerationApplicationBy (subject : NounPhrase) : Instruction :=
  let s := NounPhrase.laterMention "regenerationApplication.subject" subject
  .enact (.action "regenerate") (.sequentially [
    .clearDamage (.firstMention "regenerationApplication.subject" subject),
    act (.possessorOf .controller s) (.enact (.action "tap") (.setStatus .tapped s)),
    .doIf (.or [.matches s (.inCombat .attackerOf none), .matches s (.inCombat .blockerOf none)])
      (.combat s (.participation .outsideCombat)) none])

private def regenerateBy (permanent : NounPhrase) : Instruction :=
  .establish (.replacement
    (.verbedEvent none (.action "destroy") (some (.firstMention "regenerate.0" permanent)) none none)
    [] none (regenerationApplicationBy (.laterMention "regenerate.0" permanent)) .nextTimeOnly none)
    (some .thisTurn)

/-- "Regenerate target creature." -/
theorem okRegenerateTarget : Instruction.check [] (regenerateBy (target creature)) = [] := by decide

/-- "Regenerate this creature." -/
theorem okRegenerateThisCreature : Instruction.check [] (regenerateBy thisCreature) = [] := by decide

/-- The target is mentioned seven times and targeted once. -/
theorem regenerateTargetsOnce :
    targets ((regenerateBy (target creature)).intro []) = 1 := by decide

/-- The application alone, as a static regeneration ability applies it [CR#701.19b]. -/
theorem okRegenerationApplication :
    Instruction.check [] (regenerationApplicationBy (target creature)) = [] := by decide
/-! ## Airbend, card by card

"Airbend [objects]" [CR#701.65a]: the performer exiles them, and for each card exiled this way,
for as long as it remains exiled, its owner may cast it by paying {2} rather than its mana cost.
The per-card spelling binds the objects where it exiles them and loops over each card exiled
this way, handing each card's permission to that card's own owner. The exile makes them new
objects [CR#400.7,400.7j], so the loop reads them back by anaphora, never by the parameter
(`docs/decisions/semantics-v2.md` §7, ruling 2026-10-08). The `plugins_v2` body keeps one
handoff to the owner of the whole group until the singular case below checks
(`semantics-v2-per-member-handoff`), whose later mentions are `it`. -/

/-- The permission one exiled card's owner is handed. -/
private def airbendPermission : Instruction :=
  act (ownerOf it)
    (.establish
      (mayPlayDeed (.action "cast") .actor it none
        (.play (some exileZone) none none false (.payingInstead (.mana [generic 2]))))
      (some (.forAsLongAs (.matches it (.inZone exileZone)))))

/-- The shipped airbend body as the loader writes it: the parameter is mentioned once, by the
exile, and the one handoff reads the exiled object back as `it`. -/
private def airbendByIt (objects : NounPhrase) : Instruction :=
  .enact (.action "airbend") (.ownScope 0 (.sequentially
    [ exile (.inCaller 0 objects), airbendPermission ]))

/-- "Airbend target creature.": "Exile it. While it's exiled, its owner may cast it …". -/
theorem okAirbendItOne :
    Instruction.check [] (airbendByIt (target creature)) = [] := by decide

/-- "Airbend up to two target creatures.": `it` agrees with one object only, so each of the
three is refused until the per-card loop replaces the one handoff. -/
theorem airbendItRefusesSeveral :
    Instruction.check [] (airbendByIt (.described (.target (upTo 2)) creature))
      = [.anaphor .bare .one 0, .anaphor .bare .one 0, .anaphor .bare .one 0] := by
  decide

/-- "each card exiled this way": a description, so its number is not fixed. -/
private def cardsExiledThisWay : NounPhrase :=
  allOf (.and [.isCard,
    happenedTo (.verbedEvent none (.action "exile") (some (.gap .object)) none none) .thisWay])

/-- The per-card airbend body as the loader would write it. -/
private def airbendByCard (objects : NounPhrase) : Instruction :=
  .enact (.action "airbend") (.ownScope 0 (.sequentially
    [ exile (.firstMention "airbend.0" (.inCaller 0 objects)),
      .doForEach cardsExiledThisWay airbendPermission ]))

/-- "Airbend up to two target creatures.": each exiled card's own owner is handed the
permission for that card, one handoff per card. -/
theorem okAirbendByCardUpToTwo :
    Instruction.check [] (airbendByCard (.described (.target (upTo 2)) creature)) = [] := by
  decide

/-- "Airbend target creature.": under the strict rule each "it" in the loop ties between the
loop's card and the one exiled creature, the same object. The nearest-clause rule of the
2026-10-07 ruling (`docs/decisions/semantics-v2.md` §7) resolves each to the loop's card;
`lean-drt-anaphora-refactor` flips this pin to `[]`. -/
theorem airbendByCardOneTiesUnderStrictRule :
    Instruction.check [] (airbendByCard (target creature))
      = [.anaphor .bare .one 2, .anaphor .bare .one 2, .anaphor .bare .one 2] := by
  decide

end Semantics.Proofs.BindingIdentity
