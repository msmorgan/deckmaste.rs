import Semantics.Macros

open Semantics.Macros

namespace Semantics.Proofs.CorpusRecency

-- Eriette introduces the Aura before the permanent it becomes attached to.
-- Mentioning the Aura again in the mana-value comparison preserves its identity.
private def erietteAura : Referent :=
  { id := 0, det := .a, plur := .one, clause := 1, role := .other,
    payload := .object [.enchantment] (some .battlefield) none none none none }
private def eriettePermanent : Referent :=
  { id := 1, det := .a, plur := .one, clause := 0, role := .obj,
    payload := .object [] (some .battlefield) none none none none }
private def erietteContext : DRS :=
  { referents := [erietteAura, eriettePermanent], nextId := 2, currentClause := 2 }

theorem erietteRawDemonstrativeSelectsAttachedPermanent :
    (resolveCandidates (.word .permanent) .one erietteContext).map (·.id) = [1] := by decide

private def erietteEvent : GameEvent :=
  .does .any (.attachment .attached
    (.described (.a .unmarked) (.and [
      .hasSubtype (.of .enchantment "Aura"), .hasPossessor .controller .you]))
    (some (.described (.a .unmarked) (.and [
      .inZone (.zone .battlefield .bare), .not (.hasType .land),
      .hasPossessor .controller (.described (.a .unmarked) .opponent),
      .compare [.stat .manaValue] .atMost
        (.statOf (.stat .manaValue) (.pro (.word (.type .enchantment)) .one))]))))

theorem erietteEventChecks :
    (stepGameEvent 64 (DRS.ofList []) erietteEvent).2 = [] := by decide

theorem erietteRawWordingSelectsItsAttachedPermanent :
    (resolveCandidates (.word .permanent) .one
      (nextClause (stepGameEvent 64 (DRS.ofList []) erietteEvent).1)).map (·.id) = [3] := by decide

private def selected (word : NounWord) (drs : DRS) : List (List BindingId) :=
  (resolveCandidates (.word word) .one (nextClause drs)).map (·.idents)

private def tahngarthHeader : GameEvent :=
  .does (.actor (.firstMention "tahngarth.opponent" anOpponent))
    (.attackWith (counted (atLeast 1) creature) none)
private def tahngarthAfter : DRS := (stepGameEvent 64 (DRS.ofList []) tahngarthHeader).1
private def tahngarthDefender : NounPhrase :=
  .firstMention "tahngarth.defender"
    (a (.and [.or [.anyPlayer, .hasType .planeswalker],
      .inCombat .attackedBy (some (that .player))]))

theorem tahngarthRawModifierSelectsAttackingOpponent :
    selected .player tahngarthAfter = [["tahngarth.opponent"]] := by decide

theorem tahngarthDefenderModifierChecks :
    NounPhrase.check (nextClause tahngarthAfter) tahngarthDefender = [] := by decide

theorem tahngarthRawFinalDemonstrativeSelectsDefender :
    selected .join (stepNounPhrase 64 (nextClause tahngarthAfter) .obj true tahngarthDefender).1 =
      [["tahngarth.defender"]] := by decide

private def bombshellPlayer : NounPhrase :=
  .firstMention "bombshell.controller"
    (a (.and [.anyPlayer, .otherThan (ownerOf thisCreature)]))

private def bombshellContext : DRS :=
  (stepCondition 64 (DRS.ofList [])
    (.matches thisCreature (.hasPossessor .controller bombshellPlayer))).1

theorem bronzeBombshellRawPlayerSelectsController :
    selected .player bombshellContext = [["bombshell.controller"]] := by decide

private def runeswordSource : NounPhrase :=
  .firstMention "runesword.source" (target creature)
private def runeswordBefore : DRS :=
  (stepNounPhrase 64 (DRS.ofList []) .obj true runeswordSource).1
private def runeswordVictim : NounPhrase :=
  .firstMention "runesword.victim" (a (.and [creature,
    .did ⟨.actor (.laterMention "runesword.source" (target creature)),
      .dealDamage .anyAmount .which .any, .duration soFarThisTurn none⟩]))
private def runeswordWould : WouldClause :=
  .would .any (.move runeswordVictim (.zones [battlefield]) (some graveyard) .departure none [])

theorem runeswordRawDemonstrativeSelectsDyingCreature :
    selected (.type .creature)
      (stepWouldClause 64 (nextClause runeswordBefore) runeswordWould).1 =
      [["runesword.victim"]] := by decide

private def ramSource : NounPhrase :=
  .firstMention "ram.source" (target creatureYouControl)
private def ramRecipient : NounPhrase :=
  .firstMention "ram.recipient" (target (.and [creature, .not (.hasPossessor .controller .you)]))
private def ramDamage : Instruction :=
  instruct ramSource (.dealDamage (powerOf it) ramRecipient .any)
private def ramAfterDamage : DRS :=
  (stepInstruction 64 .postEvent .obj (DRS.ofList []) ramDamage).1
private def ramAfterCondition : DRS :=
  (stepCondition 64 (nextClause ramAfterDamage)
    (.matches (.laterMention "ram.source" ramSource) (.hasKeyword (.the (.named "trample"))))).1

theorem ramThroughRawPossessorSelectsDamageRecipient :
    selected (.type .creature) ramAfterCondition = [["ram.recipient"]] := by decide

end Semantics.Proofs.CorpusRecency
