import Semantics.Definitions

open Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.CorpusRecency

-- Eriette introduces the Aura before the permanent it becomes attached to.
-- Mentioning the Aura again in the mana-value comparison preserves its identity.
private def erietteAura : Referent :=
  { id := 0, det := .a, plur := .one, clause := 1, role := .other,
    payload := .object [.enchantment] (some .battlefield) none none none }
private def eriettePermanent : Referent :=
  { id := 1, det := .a, plur := .one, clause := 0, role := .obj,
    payload := .object [] (some .battlefield) none none none }
private def erietteContext : DRS :=
  { referents := [erietteAura, eriettePermanent], nextId := 2, currentClause := 2 }

theorem erietteRawDemonstrativeSelectsAttachedPermanent :
    (resolveCandidates (.word .permanent) .one erietteContext).map (·.id) = [1] := by decide

private def erietteEvent : GameEvent :=
  .does .any (.attachment .attached
    (.described (.a .unmarked) (.and [
      .hasSubtype (.of .enchantment "Aura"), .hasPossessor .controller .you]))
    (some (.described (.a .unmarked) (.and [
      .in_ (.zone .battlefield none), .not (.hasType .land),
      .hasPossessor .controller (.described (.a .unmarked) (.isOpponentOf .you)),
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
    (.attackWith (counted (atLeast 1) isCreature) none)
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
  .firstMention "runesword.source" (target isCreature)
private def runeswordBefore : DRS :=
  (stepNounPhrase 64 (DRS.ofList []) .obj true runeswordSource).1
private def runeswordVictim : NounPhrase :=
  .firstMention "runesword.victim" (a (.and [isCreature,
    .did ⟨.actor (.laterMention "runesword.source" (target isCreature)),
      .dealDamage .anyAmount .which .any, some (.duration soFarThisTurn none)⟩]))
private def runeswordWould : WouldClause :=
  .would .any (.move runeswordVictim (battlefield) (some graveyard) .departure none [])

theorem runeswordRawDemonstrativeSelectsDyingCreature :
    selected (.type .creature)
      (stepWouldClause 64 (nextClause runeswordBefore) runeswordWould).1 =
      [["runesword.victim"]] := by decide

private def ramSource : NounPhrase :=
  .firstMention "ram.source" (target (and [isCreature, youControl]))
private def ramRecipient : NounPhrase :=
  .firstMention "ram.recipient" (target (.and [isCreature, .not (.hasPossessor .controller .you)]))
private def ramDamage : Action :=
  instruct ramSource (.dealDamage (powerOf it) ramRecipient .any)
private def ramAfterDamage : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList []) ramDamage).1
private def ramAfterCondition : DRS :=
  (stepCondition 64 (nextClause ramAfterDamage)
    (.matches (.laterMention "ram.source" ramSource) (.hasKeyword (.the (.named "trample"))))).1

theorem ramThroughRawPossessorSelectsDamageRecipient :
    selected (.type .creature) ramAfterCondition = [["ram.recipient"]] := by decide

private def opponentOrPermanent : NounPhrase :=
  .firstMention "damage.recipient" (a (.or [(.isOpponentOf .you),
    .and [isPermanent, .hasPossessor .controller anOpponent]]))
private def giselaPattern : WouldClause :=
  .would (.actor (a .isSource)) (.dealDamage .anyAmount opponentOrPermanent .any)
private def solphimPattern : WouldClause :=
  .would (.actor (a (.and [.isSource, .hasPossessor .controller .you])))
    (.dealDamage .anyAmount opponentOrPermanent .noncombatOnly)

theorem giselaRawJoinedDemonstrativeSelectsRecipient :
    selected .join (stepWouldClause 64 (DRS.ofList []) giselaPattern).1 =
      [["damage.recipient"]] := by decide

theorem solphimRawJoinedDemonstrativeSelectsRecipient :
    selected .join (stepWouldClause 64 (DRS.ofList []) solphimPattern).1 =
      [["damage.recipient"]] := by decide

private def mangaraSource : NounPhrase :=
  .firstMention "mangara.source" (a (.and [isCreature,
    .ofChosen .theLatestChoice .color]))
private def mangaraRecipient : NounPhrase :=
  .or [.you, .firstMention "mangara.recipient"
    (a (.and [isCreature, .colorIs .white, .hasPossessor .controller .you]))]
private def mangaraContext : DRS :=
  (stepGameEvent 64 (DRS.ofList [ChoiceSort.binding (.quality .color)])
    (.does (.actor mangaraSource) (.dealDamage .anyAmount mangaraRecipient .any))).1

theorem mangarasEquityRawDemonstrativeSelectsWrongCreature :
    selected (.type .creature) mangaraContext = [["mangara.recipient"]] := by decide

theorem mangarasEquityExplicitSelectorRestoresDamageSource :
    let (after, ids, errors) := stepNounPhrase 64 (nextClause mangaraContext) .obj true
      (.laterMention "mangara.source" mangaraSource)
    errors = [] ∧ (ids.filterMap after.findById?).map (·.idents) =
      [["mangara.source"]] := by decide

private def noeticUpkeep : GameEvent :=
  .beginningOf .upkeep (some (.firstMention "noetic.upkeepPlayer" (each .anyPlayer))) .current
private def noeticContext : DRS :=
  (stepLocationWith (stepOps 64)
    (nextClause (stepGameEvent 64 (DRS.ofList []) noeticUpkeep).1)
    (.zone .hand (some (.firstMention "noetic.owner" (ownerOf (each isCreature)))))).1

theorem noeticScalesRawDemonstrativeSelectsWrongPlayer :
    selected .player noeticContext = [["noetic.owner"]] := by decide

theorem noeticScalesExplicitSelectorRestoresUpkeepPlayer :
    let (after, ids, errors) := stepNounPhrase 64 (nextClause noeticContext) .other false
      (.laterMention "noetic.upkeepPlayer" (each .anyPlayer))
    errors = [] ∧ (ids.filterMap after.findById?).map (·.idents) =
      [["noetic.upkeepPlayer"]] := by decide

private def damagedVictim (attachment : AttachWord) : NounPhrase :=
  a (.and [isCreature,
    .did ⟨.actor (.attachHost attachment (.type .creature)),
      .dealDamage .anyAmount .which .any, some (.duration soFarThisTurn none)⟩])

private def scytheDeath : GameEvent :=
  does (damagedVictim .equipped) die
private def scytheAfterDeath : DRS :=
  (stepGameEvent 64 (DRS.ofList []) scytheDeath).1
private def scytheReturn : Action :=
  .move (that .card) .anyLocation (some battlefield) .arrival none []
private def scytheAfterReturn : DRS :=
  (stepAction 64 .postEvent .obj (nextClause scytheAfterDeath) scytheReturn).1

theorem scytheDeathAndReturnCheck :
    (stepGameEvent 64 (DRS.ofList []) scytheDeath).2 = [] ∧
      Action.check (nextClause scytheAfterDeath) scytheReturn = [] := by decide

theorem scytheRawDemonstrativeSelectsReturnedCreature :
    let selected := resolveCandidates (.word (.type .creature)) .one (nextClause scytheAfterReturn)
    selected.map (·.zone) = [some .battlefield] ∧
      selected.map (·.id) = [scytheAfterReturn.nextId - 1] := by decide

private def vampiricDeath : GameEvent :=
  does (damagedVictim .enchanted) die
private def vampiricAfterDeath : DRS :=
  (stepGameEvent 64 (DRS.ofList []) vampiricDeath).1

theorem vampiricDeathChecks :
    (stepGameEvent 64 (DRS.ofList []) vampiricDeath).2 = [] := by decide

theorem vampiricRawDemonstrativeSelectsEnchantedCreature :
    let refs := resolveCandidates (.word (.type .creature)) .one (nextClause vampiricAfterDeath)
    refs.length = 1 ∧ refs.all (fun r => r.attachmentHost && r.zone == some .battlefield) ∧
      refs.map (·.id) = (vampiricAfterDeath.referents.filter (·.attachmentHost)).map (·.id) := by decide

theorem vampiricDeadCreatureIsExcludedByZone :
    let dead := vampiricAfterDeath.referents.filter (fun r => r.zone == some .graveyard)
    dead.length = 1 ∧ dead.all (fun r => !r.attachmentHost) := by decide

theorem vampiricCountersUseEnchantedCreature :
    let (after, errors) := stepAction 64 .postEvent .obj (nextClause vampiricAfterDeath)
      (.putCounters (.lit 1) (.printed p1p1Counter) (that (.type .creature)) .many true)
    let patients := (after.pendingRoles.filter (fun p => p.2 == .obj)).map (·.1)
    errors = [] ∧ patients.length = 1 ∧
      patients = (vampiricAfterDeath.referents.filter (·.attachmentHost)).map (·.id) ∧
      (patients.filterMap after.findById?).all (fun r => r.zone == some .battlefield) := by decide

private def cycledContext : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList [])
    (.enact (.ofAbility (.named "cycling")) (discard (a .isCard)) none none)).1
private def cycloneFinalCard : NounPhrase :=
  a (.and [.isCard, .in_ yourLibrary,
    .shares .cardType .share
      (theVerbed (.ofAbility (.named "cycling")) .one (some .card) .attributive)])
private def cycloneExile : Action :=
  .move cycloneFinalCard (yourLibrary) (some exileZone) .arrival none []
private def cycloneAfter : DRS :=
  (stepAction 64 .postEvent .obj (nextClause cycledContext) cycloneExile).1

theorem unpredictableCycloneTerminalCardDescriptionChecks :
    Action.check (nextClause cycledContext) cycloneExile = [] := by decide

theorem unpredictableCycloneRawDemonstrativeSelectsNewlyExiledCard :
    let refs := resolveCandidates (.word .card) .one (nextClause cycloneAfter)
    refs.map (·.zone) = [some .exile] ∧
      refs.map (·.id) = [cycloneAfter.nextId - 1] := by decide

end Semantics.Proofs.CorpusRecency
