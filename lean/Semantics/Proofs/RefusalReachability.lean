import Semantics.Check.AbilityRules
import Semantics.Definitions.Subtypes
import Semantics.Templates
import Semantics.Definitions.Phrasings

namespace Semantics.Proofs.RefusalReachability

private def emptyContext : DRS := DRS.ofList []

/-- "The number of …" counts a group: a plural universal ("the number of creatures"), not one
indefinite object. -/
theorem countRequiresACountableGroup :
    Amount.check emptyContext (.countOf (.described .all (.hasType .creature))) = [] ∧
    Amount.check emptyContext (.countOf (.described (.a .unmarked) (.hasType .creature))) =
      [.plural, .countableGroup] := by decide

/-- An existential clause introduces a plain or counted indefinite ("if there is a player", "if
two or more creatures are …"), rather than asserting a definite participant. -/
theorem existentialRequiresAnExistentialMention :
    Condition.check emptyContext (.exists_ (.described (.a .unmarked) .anyPlayer)) = [] ∧
    Condition.check emptyContext
      (.exists_ (.described (.count (.range (some 2) none) none) (.hasType .creature))) = [] ∧
    Condition.check emptyContext (.exists_ .you) = [.existentialMention] := by decide

open Semantics.Templates Semantics.Definitions in
/-- The domain "the number of …" counts and "the greatest … among …" ranges over is closed after
the measure (`semantics-v3.md` §1.3): no later pronoun reads it back. In *Tears of Rage*, *Mishra,
Claimed by Gix* and *Highcliff Felidar* the reference after the measure reads something else. -/
theorem measuredDomainIsNotAnAntecedent :
    Action.check emptyContext
        (.sequentially [gainLife (countOf isCreature), .setStatus .tapped them]) =
      [.anaphor .bare .many 0] ∧
    Action.check emptyContext
        (.sequentially [gainLife (aggregate .max (.stat .power) isCreature),
          .setStatus .tapped them]) =
      [.anaphor .bare .many 0] := by decide

open Semantics.Templates Semantics.Definitions in
/-- A target announced inside a measured domain survives its closure, being chosen as the spell or
ability is put on the stack [CR#601.2c,115.1]: *Tibalt, the Fiend-Blooded*'s "deals damage equal
to the number of cards in target player's hand to that player", and the same through "the
greatest power among creatures target opponent controls". -/
theorem measureKeepsItsAnnouncedTargets :
    Action.check emptyContext
        (instruct this_ (dealDamage (countOf (cardIn (handOf (target isPlayer)))) (that player))) =
      [] ∧
    Action.check emptyContext
        (.sequentially
          [ gainLife (aggregate .max (.stat .power)
              (and [isCreature, .hasPossessor .controller (target isOpponent)])),
            instruct (that player) (draw 1) ]) = [] := by decide

open Semantics.Templates Semantics.Definitions in
/-- An existential's domain is read in a self-replacement's replacement and closed after it, as
after a `doIf`: "You gain 1 life. If there are two or more creature cards in your graveyard, you
gain 2 life instead. Choose one of them." refuses "them". -/
theorem replacementConditionClosesItsExistentialDomain :
    Action.check emptyContext
        (.sequentially
          [ replace (gainLife 1)
              (doIf (thereAre (counted (orMore 2) (and [isCreature, cardIn yourGraveyard])))
                (instead (gainLife 2))),
            choose (someOf (exactly 1) them) ]) = [.anaphor .bare .many 0] := by decide

open Semantics.Templates Semantics.Definitions in
/-- *Tears of Rage*: "Attacking creatures you control get +X/+0 until end of turn, where X is the
number of attacking creatures. Sacrifice those creatures …": "those creatures" reads the
creatures that got +X/+0 (the subject, referent 1), not the attacking creatures counted. -/
theorem tearsOfRageThoseCreaturesReadTheSubject :
    let pumped := get (bare (and [isCreature, isAttacking, youControl])) (up (letter .x)) (up 0)
      untilEndOfTurn
    let out := (stepAction 512 .postEvent .obj emptyContext
      (.sequentially [pumped, define .x (countOf (and [isCreature, isAttacking]))])).1
    (out.findById? 1).map (·.det) = some .all ∧
      (resolveCandidates (.word (.type .creature)) .many (nextClause out)).map (·.id) = [1] := by
  decide

/-- A written multi-zone search names at least two different zones. -/
theorem searchRequiresDistinctMultipleZones :
    (stepSearchLocationWith (stepOps 64) emptyContext
      (.or [.zone .hand none, .zone .library none])).2 = [] ∧
    (stepSearchLocationWith (stepOps 64) emptyContext
      (.or [])).2 = [.atLeastTwoZones] ∧
    (stepSearchLocationWith (stepOps 64) emptyContext
      (.or [.zone .library none])).2 = [.atLeastTwoZones] ∧
    (stepSearchLocationWith (stepOps 64) emptyContext
      (.or [.zone .library none, .zone .library none])).2 = [.atLeastTwoZones] := by decide

/-- A multiplier used to express “for each” cannot be literally zero. -/
theorem zeroMultiplierIsRefused :
    Amount.check emptyContext (.arith .times 0 2) = [.amtNonZero] ∧
    Amount.check emptyContext (.arith .times 1 2) = [] ∧
    Amount.check emptyContext (.arith .plus 0 2) = [] := by decide

/-- A word-headed line wraps an ability, not another word-headed line. -/
theorem wordHeadsCannotBeNested :
    let keyword := Ability.keyword (.named "flying") [] []
    let head := ItalicWord.abilityWord (.named "landfall")
    Ability.check emptyContext (.italicHead head keyword) = [] ∧
      Ability.check emptyContext (.italicHead head (.italicHead head keyword)) =
        [.notWordHeaded] := by decide

/-- An ability word is one the CR lists, as it spells it [CR#207.2c]: "landfall" is, a misspelt
or capitalised word is not. -/
theorem abilityWordIsOneTheCrLists :
    let keyword := Ability.keyword (.named "flying") [] []
    Ability.check emptyContext (.italicHead (.abilityWord (.named "landfall")) keyword) = [] ∧
      Ability.check emptyContext (.italicHead (.abilityWord (.named "Landfal")) keyword) =
        [.knownAbilityWord (.named "Landfal")] := by decide

/-- “In that pile” requires both pile kind and a written pile mention. -/
theorem inPileRequiresAPileMention :
    let context := DRS.ofList [{
      det := .the, plur := .one, payload := .pile (some .exile) none none }]
    Predicate.checkAt .object context (.in_ (.pile (.pro (.word .pile) .one))) = [] ∧
      Predicate.checkAt .object emptyContext (.in_ (.pile .you)) =
        [.kindMismatch .pile .player, .pileMention] := by decide

private def selectedCreature : NounPhrase :=
  .described (.target (.range (some 1) (some 1))) (.hasType .creature)

/-- A lazily evaluated caller argument is already supplied to its definition's test. -/
theorem callerOwnedTestSubjectsDoNotPermitFreshLocalTestSelections :
    let test := Action.doIf
      (.matches (.firstMention "tested.parameter" (.inCaller selectedCreature))
        (.hasType .creature)) (.draw (.exactlyOf 1)) none
    Action.check emptyContext (.ownScope none test) = [] ∧
      Condition.check emptyContext (.matches selectedCreature (.hasType .creature)) =
        [.testSubject] ∧
      Action.check emptyContext (.ownScope none
        (.doIf (.matches selectedCreature (.hasType .creature)) (.draw (.exactlyOf 1)) none)) =
        [.testSubject] := by decide

/-- A property test's subject is definite: "if a creature is tapped" is written as an existential.
Only a coordinated subject may hold an indefinite conjunct ("Titania and a land named Argoth"),
read existentially (owner ruling 2026-10-22). -/
theorem onlyACoordinatedTestSubjectHoldsAnIndefinite :
    Condition.check emptyContext
        (.matches (.described (.a .unmarked) (.hasType .creature)) (.hasType .artifact)) =
      [.testSubject] ∧
    Condition.check emptyContext
        (.matches (.and [.headed (.type .creature) .this,
            .described (.a .unmarked) (.hasType .land)]) (.hasType .artifact)) = [] := by
  decide

/-- "the creatures put into a graveyard": a definite plural Participial Description. -/
private def thePutCreatures : NounPhrase :=
  .described (.the .many)
    (.and [ .did { subj := .any
                   action := .move .which (.zone .battlefield none) (some (.zone .graveyard none))
                     .departure []
                   within := none },
            .hasType .creature ])

/-- Two creatures moved from the battlefield to a graveyard. -/
private def putCreaturesContext : DRS := DRS.ofList [
  { det := .count, plur := .many,
    payload := .object [.creature] (some .graveyard)
      (some (moveStamp none (some .battlefield) (some .graveyard))) none none }]

/-- Explicit counted selections and definite plural readbacks keep their written plurality. -/
theorem writtenPluralGroupsRemainCountable :
    Amount.check emptyContext
      (.countOf (.described (.count (.range (some 2) (some 2)) none) (.hasType .creature))) = [] ∧
    NounPhrase.plur thePutCreatures = .many ∧
    Amount.check putCreaturesContext (.countOf thePutCreatures) = [] := by decide

/-- "An opponent" and "a teammate" relate a player to the unwritten "you", which they read
without introducing; a written relatum is a player [CR#102.2,102.3]. -/
theorem opponentAndTeammateAreRelational :
    Predicate.checkAt .player emptyContext (.isOpponentOf .you) = [] ∧
    Predicate.checkAt .player emptyContext (.isTeammateOf .you) = [] ∧
    (stepPredicate 64 emptyContext (.isOpponentOf .you)).1.visibleReferents = [] ∧
    Predicate.checkAt .player emptyContext
      (.isOpponentOf (.described (.target (.exactlyOf 1)) (.hasType .creature))) =
        [.kindMismatch .player .object] := by decide

/-- A shift raises or lowers, never sets: "this spell costs {1} less" and "[−X]" stand, while a
cost shift, a damage shift, a loyalty symbol or a die-result shift that sets is refused (a cost
set instead is an alternative cost [CR#118.9]; a loyalty symbol puts on or removes counters
[CR#606.4]). -/
theorem shiftsNeverSet :
    StaticSpec.check emptyContext (.costShift .this (.generic (.down 1) none)) = [] ∧
    StaticSpec.check emptyContext (.costShift .this (.generic (.set 1) none)) =
      [.shiftSets] ∧
    StaticSpec.check emptyContext (.costShift .this (.bySymbols (.set (.lit [.simple (.generic 1)])) false)) =
      [.shiftSets] ∧
    (stepCost 64 emptyContext (.loyaltySymbol (.down (.letter .x)))).2 = [] ∧
    (stepCost 64 emptyContext (.loyaltySymbol (.set 2))).2 = [.shiftSets] := by decide

/-- A Head Noun is an Object Class, Card Type or Subtype: "this Aura" names the source by its
subtype, read as an enchantment, while "this player" is no Head Noun [CR#109.1,205.2a,205.3]. -/
theorem headNounsNameObjects :
    NounPhrase.checkAt (some .object) emptyContext (.headed (.subtype Definitions.aura) .this)
      = [] ∧
    NounPhrase.checkAt (some .object) emptyContext (.headed .player .this) = [.headNoun] := by
  decide
end Semantics.Proofs.RefusalReachability
