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
        (.hasType .creature)) (.draw (.lit 1)) none
    Action.check emptyContext (.ownScope none test) = [] ∧
      Condition.check emptyContext (.matches selectedCreature (.hasType .creature)) =
        [.testSubject] ∧
      Action.check emptyContext (.ownScope none
        (.doIf (.matches selectedCreature (.hasType .creature)) (.draw (.lit 1)) none)) =
        [.testSubject] := by decide

/-- Explicit counted selections and linked plural objects keep their written plurality. -/
theorem writtenPluralGroupsRemainCountable :
    Amount.check emptyContext
      (.countOf (.described (.count (.range (some 2) (some 2)) none) (.hasType .creature))) = [] ∧
    NounPhrase.plur (.paid (.objects (.byKeyword (.named "devour"))
      (.ofAbility (.named "devour")) .many) (.asType .creature .this none)) = .many ∧
    Amount.check emptyContext (.countOf
      (.paid (.objects (.byKeyword (.named "devour")) (.ofAbility (.named "devour")) .many)
        (.asType .creature .this none))) = [] := by decide

end Semantics.Proofs.RefusalReachability
