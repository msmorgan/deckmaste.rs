import Semantics.Check.AbilityRules

namespace Semantics.Proofs.RefusalReachability

private def emptyContext : DRS := DRS.ofList []

/-- Counting a domain is distinct from counting an already quantified selection. -/
theorem countRequiresACountableGroup :
    Amount.check emptyContext (.countOf (.described .bare (.hasType .creature))) = [] ∧
    Amount.check emptyContext (.countOf (.described .all (.hasType .creature))) =
      [.countableGroup] ∧
    Amount.check emptyContext (.countOf (.described (.a .unmarked) (.hasType .creature))) =
      [.plural, .countableGroup] := by decide

/-- An existential clause introduces a domain, rather than asserting a definite participant. -/
theorem existentialRequiresAnExistentialMention :
    Condition.check emptyContext (.exists_ (.described .bare .anyPlayer)) = [] ∧
    Condition.check emptyContext (.exists_ .you) = [.existentialMention] := by decide

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
    Amount.check emptyContext (.arith .times (.lit 0) (.lit 2)) = [.amtNonZero] ∧
    Amount.check emptyContext (.arith .times (.lit 1) (.lit 2)) = [] ∧
    Amount.check emptyContext (.arith .plus (.lit 0) (.lit 2)) = [] := by decide

/-- Naming payment of a keyword cost requires that keyword to have a cost. -/
theorem namedPaymentRequiresAPayableKeyword :
    (stepSpendPurposeWith (stepOps 64) emptyContext
      (.toPay (.ofKeyword (.named "flying")))).2 = [.costNameable] ∧
    (stepSpendPurposeWith (stepOps 64) emptyContext
      (.toPay (.ofKeyword (.named "ward")))).2 = [] := by decide

/-- A word-headed line wraps an ability, not another word-headed line. -/
theorem wordHeadsCannotBeNested :
    let keyword := Ability.keyword (.named "flying") [] []
    let head := ItalicWord.abilityWord "landfall"
    Ability.check emptyContext (.italicHead head keyword) = [] ∧
      Ability.check emptyContext (.italicHead head (.italicHead head keyword)) =
        [.notWordHeaded] := by decide

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
        (.hasType .creature)) (.draw (.exactlyOf (.lit 1))) none
    Action.check emptyContext (.ownScope none test) = [] ∧
      Condition.check emptyContext (.matches selectedCreature (.hasType .creature)) =
        [.testSubject] ∧
      Action.check emptyContext (.ownScope none
        (.doIf (.matches selectedCreature (.hasType .creature)) (.draw (.exactlyOf (.lit 1))) none)) =
        [.testSubject] := by decide

/-- Explicit counted selections and linked plural objects keep their written plurality. -/
theorem writtenPluralGroupsRemainCountable :
    Amount.check emptyContext
      (.countOf (.described (.count (.range (some 2) (some 2)) none) (.hasType .creature))) = [] ∧
    NounPhrase.plur (.paid (.objects (.byKeyword (.named "devour"))
      (.ofAbility (.named "devour")) .many) (.headed (.type .creature) .this)) = .many ∧
    Amount.check emptyContext (.countOf
      (.paid (.objects (.byKeyword (.named "devour")) (.ofAbility (.named "devour")) .many)
        (.headed (.type .creature) .this))) = [] := by decide

/-- "An opponent" and "a teammate" relate a player to the unwritten "you", which they read
without introducing; a written relatum is a player [CR#102.2,102.3]. -/
theorem opponentAndTeammateAreRelational :
    Predicate.checkAt .player emptyContext (.isOpponentOf .you) = [] ∧
    Predicate.checkAt .player emptyContext (.isTeammateOf .you) = [] ∧
    (stepPredicate 64 emptyContext (.isOpponentOf .you)).1.visibleReferents = [] ∧
    Predicate.checkAt .player emptyContext
      (.isOpponentOf (.described (.target (.exactlyOf (.lit 1))) (.hasType .creature))) =
        [.kindMismatch .player .object] := by decide

/-- A shift raises or lowers, never sets: "this spell costs {1} less" and "[−X]" stand, while a
cost shift, a damage shift, a loyalty symbol or a die-result shift that sets is refused (a cost
set instead is an alternative cost [CR#118.9]; a loyalty symbol puts on or removes counters
[CR#606.4]). -/
theorem shiftsNeverSet :
    StaticSpec.check emptyContext (.costShift .this (.generic (.down (.lit 1)) none)) = [] ∧
    StaticSpec.check emptyContext (.costShift .this (.generic (.set (.lit 1)) none)) =
      [.shiftSets] ∧
    StaticSpec.check emptyContext (.costShift .this (.bySymbols (.set (.lit [.simple (.generic 1)])) false)) =
      [.shiftSets] ∧
    (stepCost 64 emptyContext (.loyaltySymbol (.down (.letter .x)))).2 = [] ∧
    (stepCost 64 emptyContext (.loyaltySymbol (.set (.lit 2)))).2 = [.shiftSets] := by decide

/-- A Head Noun is an Object Class, Card Type or Subtype: "this Aura" names the source by its
subtype, read as an enchantment, while "this player" is no Head Noun [CR#109.1,205.2a,205.3]. -/
theorem headNounsNameObjects :
    NounPhrase.checkAt (some .object) emptyContext (.headed (.subtype (.of .enchantment "Aura")) .this)
      = [] ∧
    NounPhrase.checkAt (some .object) emptyContext (.headed .player .this) = [.headNoun] := by
  decide
end Semantics.Proofs.RefusalReachability
