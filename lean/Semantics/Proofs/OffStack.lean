import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.OffStack

Off-Stack Statics (`semantics-v2-off-stack-statics`, `docs/decisions/semantics-v2.md` §7, ruling
2026-10-09): "as [event], do" (`StaticSpec.asEvent`) and "any time [condition] holds, do"
(`StaticSpec.whileHolds`), which carry an action that never uses the stack
[CR#604.1,603.7a,603.7g,614.1c]; the object a spell becomes (`NounPhrase.becomes`, [CR#400.7]);
and a deontic rule's one exempt source (`DeonticExcept`, [CR#702.145b,702.13b]). The keyword bodies are
read through their declarations (`Semantics.Definitions`). Each pin is closed by `decide`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.OffStack

/-! ## The nodes -/

/-- "As this permanent enters, you may draw a card": an action carried out as part of the
entering it replaces, which still happens [CR#614.1c]. -/
theorem okAsEnteringAlso :
    StaticSpec.check (DRS.ofList [])
      (replace (would thisPermanent enter) (also (may (draw 1)))) = [] := by
  decide

/-- "As it becomes night, transform this permanent": an action carried out as an event that is
no permanent's entering happens [CR#702.145b]. -/
theorem okAsEventBecomesNight :
    StaticSpec.check (DRS.ofList [])
      (.asEvent (gameBecomes night) (transform thisPermanent)) = [] := by
  decide

/-- "Any time you control ten or more permanents and you don't have the city's blessing, you get
the city's blessing for the rest of the game" [CR#702.131b]. -/
theorem okWhileHoldsCitysBlessing :
    StaticSpec.check (DRS.ofList [])
      (.whileHolds
        (.and
          [ .compareAmt (.countOf (.described .all (.and [isPermanent, .hasPossessor .controller .you])))
              .atLeast 10,
            .not (.matches .you (.hasDesignation (.named "citysBlessing") none)) ])
        (.gainDesignation .you (.named "citysBlessing") (some (.between none none)))) = [] := by
  decide

/-- "Return the permanent this spell becomes to its owner's hand" [CR#400.7,702.109a]. -/
theorem okBecomesThisSpell :
    Action.check (DRS.ofList []) (putInto (.becomes thisSpell) hand) = [] := by decide

/-- Only an object changes zones [CR#400.7]: what a player "becomes" is refused as a kind. -/
theorem badBecomesOfAPlayer :
    NounPhrase.checkAt none (DRS.ofList []) (.becomes .you) = [.kindMismatch .object .player] := by decide

/-- "This permanent can't transform except due to its daybound ability" [CR#702.145b]. -/
theorem okExceptByKeyword :
    StaticSpec.check (DRS.ofList [])
      (.cant .any (transform thisPermanent)
        { except := some (.byKeyword (.named "daybound")) }) = [] := by
  decide

/-- "This creature can't be blocked except by artifact creatures and/or black creatures"
[CR#702.36b]. -/
theorem okExceptByObjects :
    StaticSpec.check (DRS.ofList [])
      (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature, or [isArtifact, hasColor .black]])))) = [] := by
  decide

/-- An exception by objects describes objects: "except by opponents" is refused as a kind. -/
theorem badExceptByPlayers :
    StaticSpec.check (DRS.ofList [])
      (cant any (block thisCreature) (except := byObjects (bare isOpponent))) =
        [.kindMismatch .object .player] := by
  decide

/-! ## The keyword bodies -/

/-- Dash {1}{R} [CR#702.109a]. -/
theorem okDash : Ability.check (DRS.ofList []) (dash (.mana [generic 1, pip .red])) = [] := by decide

/-- Blitz {2}{R} [CR#702.152a]. -/
theorem okBlitz : Ability.check (DRS.ofList []) (blitz (.mana [generic 2, pip .red])) = [] := by decide

/-- Warp {3} [CR#702.185a]. -/
theorem okWarp : Ability.check (DRS.ofList []) (warp (.mana [generic 3])) = [] := by decide

/-- Rebound [CR#702.88a]. -/
theorem okRebound : Ability.check (DRS.ofList []) rebound = [] := by decide

/-- Ascend [CR#702.131a,702.131b]. -/
theorem okAscend : Ability.check (DRS.ofList []) ascend = [] := by decide

/-- Storied [CR#702.195a]. -/
theorem okStoried : Ability.check (DRS.ofList []) storied = [] := by decide

/-- Daybound [CR#702.145b]. -/
theorem okDaybound : Ability.check (DRS.ofList []) daybound = [] := by decide

/-- Nightbound [CR#702.145e]. -/
theorem okNightbound : Ability.check (DRS.ofList []) nightbound = [] := by decide

/-- Devour 1 [CR#702.82a]. -/
theorem okDevour : Ability.check (DRS.ofList []) (devour 1) = [] := by decide

/-- Devour artifact 2 [CR#702.82c]: devour's general form, labelled devour. Re-spelled from
`okDevourQualityExpansion`, which checked the same definition as a helper before the general form
was declared (`semantics-v2-devour-quality-keyword`). -/
theorem okDevourQuality :
    Ability.check (DRS.ofList []) (devourQuality isArtifact 2) = [] := by decide

/-- Riot [CR#702.136a]. -/
theorem okRiot : Ability.check (DRS.ofList []) riot = [] := by decide

/-- Tribute 3 [CR#702.104a]. -/
theorem okTribute : Ability.check (DRS.ofList []) (tribute 3) = [] := by decide

/-- Unleash [CR#702.98a]. -/
theorem okUnleash : Ability.check (DRS.ofList []) unleash = [] := by decide

/-- Fear, re-spelled onto its Deontic Exception [CR#702.36b]. -/
theorem okFear : Ability.check (DRS.ofList []) fear = [] := by decide

/-- Intimidate, re-spelled onto its Deontic Exception [CR#702.13b]. -/
theorem okIntimidate : Ability.check (DRS.ofList []) intimidate = [] := by decide

/-- Amplify 1 [CR#702.38a]. -/
theorem okAmplify : Ability.check (DRS.ofList []) (amplify 1) = [] := by decide

/-! ## The readbacks (`semantics-v2-linked-readback`) -/

/-- "When this creature enters, draw a card for each creature it devoured." -/
def drawForEachDevoured : Ability :=
  when (does thisCreature enter)
    (doForEach (each (and [isCreature, did it (devour which)])) (draw 1))

/-- Skullmulcher: "Devour 1 … When this creature enters, draw a card for each creature it
devoured." [CR#702.82b]: "it devoured" reads back the creatures devour's entry sacrificed, the
objects of a linked ability. -/
theorem okItDevouredCount :
    Ability.checkText (DRS.ofList []) [devour 1, drawForEachDevoured] = [] := by
  decide

/-- Voracious Dragon: "When this creature enters, it deals damage to any target equal to twice
the number of Goblins it devoured." [CR#702.82b] -/
theorem okGoblinsItDevoured :
    Ability.checkText (DRS.ofList [])
      [ devour 1,
        when (does thisCreature enter)
          (dealDamage (times 2 (countOf (and [hasSubtype goblin, did it (devour which)])))
            (target isAnyTarget)) ] = [] := by
  decide

/-- Without devour, it devoured nothing; and a creature another ability sacrificed was not
devoured [CR#702.82b]. -/
theorem badDevouredWithoutDevour :
    Ability.checkText (DRS.ofList []) [drawForEachDevoured] =
        [.participialDescription (.deed (.ofAbility (.named "devour"))) 0] ∧
      Ability.checkText (DRS.ofList [])
        [ static (.addedCost (.perform (sacrifice (a isCreature))) false .once),
          drawForEachDevoured ] =
        [.participialDescription (.deed (.ofAbility (.named "devour"))) 0] := by
  decide

/-- "If tribute wasn't paid" [CR#702.104b]: the tribute choice not taken. -/
theorem okTributeWasntPaid :
    Condition.check (DRS.ofList []) (.not (choiceWasTaken (.byKeyword (.named "tribute")) thisCreature)) = [] := by
  decide

end Semantics.Proofs.OffStack
