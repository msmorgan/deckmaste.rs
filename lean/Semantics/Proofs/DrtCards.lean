import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.DrtCards

private def feralContest : Action := .sequentially [
  .putCounters 1 (.printed p1p1Counter) (target (and [isCreature, youControl])),
  .establish
    (.must (.actor (target (.and [isCreature, .other]))) (.block it) {})
    (some thisTurn)]

private def monstrousStep : Action := .sequentially [
  get (target isCreature) (.up 7) (.up 7) (some untilEndOfTurn),
  .establish
    (.must (.actor (.described (.target (upTo 1)) (.and [isCreature, .other])))
      (.block it) {}) (some thisTurn)]

private def cloudAttach : Ability :=
  when (.does .any (putOntoBattlefield thisCreature))
    (attachTo
      (.described (.target (upTo 1))
        (.and [.hasSubtype (equipment), .hasPossessor .controller .you])) it)

theorem feralContestUsesOrdinaryPronoun :
    Action.check (DRS.ofList []) feralContest = [] := by decide

theorem monstrousStepUsesOrdinaryPronoun :
    Action.check (DRS.ofList []) monstrousStep = [] := by decide

theorem cloudUsesOrdinaryPronoun :
    Ability.check (DRS.ofList []) cloudAttach = [] := by decide

theorem feralContestBlocksFirstCreature :
    (stepAction 64 .postEvent .obj (DRS.ofList []) feralContest).1.resolvedIds = [1] := by
  decide

theorem monstrousStepBlocksFirstCreature :
    (stepAction 64 .postEvent .obj (DRS.ofList []) monstrousStep).1.resolvedIds = [0] := by
  decide

theorem airbendSingleTargetUsesPerCardLoop :
    Action.check (DRS.ofList []) (airbend (target isCreature)) = [] := by decide

theorem airbendMultipleTargetsUsePerCardLoop :
    Action.check (DRS.ofList [])
      (airbend (.described (.target (upTo 2)) isCreature)) = [] := by decide

theorem selfCounterProhibitionIsValidInstantText :
    cardTextOk [.instant] [.static (cant any (counter thisSpell))] = true := by decide

theorem otherSpellCounterProhibitionIsNotInstantStaticText :
    cardTextOk [.instant] [.static (cant any (counter (allOf isSpell)))] = false := by decide

theorem battlefieldProhibitionIsNotInstantStaticText :
    cardTextOk [.instant] [.static (cant any (untap thisCreature))] = false := by decide

theorem counterAlternativesUseTheirOwnHolderKinds :
    Predicate.checkAt (.join .object .player) (DRS.ofList [])
      (.or [.and [.in_ battlefield, .hasCounters none],
        .compare [.anyCounter .player] .atLeast 1]) = [] := by decide

theorem mixedCounterAxesStillRejectOneHolder :
    axesAt .object [.anyCounter .object, .anyCounter .player] = false ∧
      axesAt .player [.anyCounter .object, .anyCounter .player] = false := by decide

theorem anyDieResultReadsOneRollBatch :
    Condition.check (DRS.ofList [outcomeB .diceRolled])
      (.anyResultIs .atLeast 10) = [] := by decide

theorem anyDieResultRequiresARoll :
    Condition.check (DRS.ofList []) (.anyResultIs .atLeast 10) =
      [.outcomeInScope .rollResult 0] := by decide

theorem anyDieResultRejectsAmbiguousBatches :
    Condition.check (DRS.ofList [outcomeB .diceRolled, outcomeB .diceRolled])
      (.anyResultIs .atLeast 10) = [.outcomeInScope .rollResult 2] := by decide

theorem distributedLifeLossKeepsOneScalarLetter :
    let out := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (instruct (each (.isOpponentOf .you)) (loseLife (.letter .x)))).1
    countLetter .x out = 1 ∧ anyOpenLetter .x out = true := by decide

theorem alternativeConditionsCannotBorrowOtherBranchParticipants :
    Condition.check (DRS.ofList [])
      (.or [.did ⟨.any, destroy (a isCreature), some (.duration soFarThisTurn none)⟩,
        .matches it isCreature]) =
      [.anaphor .bare .one 0] := by decide

theorem powerDefinitionPublishesItsNumber :
    let out := (stepStaticSpec 64 (DRS.ofList [])
      (ptDefinition thisCreature .powerAlone 2)).1
    countOutcomes .namedNumber out = 1 := by decide

theorem grantedKeywordPublishesItsScalarParameter :
    let out := (stepStaticSpec 64 (DRS.ofList [])
      (Templates.abilityGrant thisCreature (keywordNumber (.named "bushido") (.letter .x)))).1
    anyOpenLetter .x out = true ∧ countLetter .x out = 1 := by decide

/-- The same through a keyword Definition, whose parameter arrives in its caller's scope:
"Fumiko has bushido X" publishes X for the sibling "where X is …". -/
theorem grantedKeywordDefinitionPublishesItsScalarParameter :
    let out := (stepStaticSpec 64 (DRS.ofList [])
      (Templates.abilityGrant thisCreature (bushido (.letter .x)))).1
    anyOpenLetter .x out = true ∧ countLetter .x out = 1 := by decide

theorem typedPlayerViewOwnsAHandWithoutChangingTheJoinedReferent :
    let joinedRef : Referent := {
      id := 17, det := .target, plur := .one,
      payload := .join (.object [.planeswalker] (some .battlefield) none none none)
        (.player false) }
    let initial := nextClause (DRS.ofList [joinedRef])
    let out := stepZoneOwnerWith (stepOps 12) initial (some they)
    out.2 = [] ∧ (out.1.findById? 17).map (·.payload) =
      (initial.findById? 17).map (·.payload) := by decide

theorem distributedDamagePublishesItsRecipientGroup :
    let out := (stepAction 12 .postEvent .obj (DRS.ofList [])
      (.dealDamage 1 (each (.isOpponentOf .you)))).1
    (resolveCandidates (.word .player) .many (nextClause out)).map (·.id) = [0] := by decide

/-- "Target creature gets +2/+2 if its power is 2. …": the condition reads the target, chosen as
the spell is cast, and the target is the one later antecedent. -/
theorem conditionalModificationKeepsItsAnnouncedTarget :
    let out := stepAction 12 .postEvent .obj (DRS.ofList [])
      (doOnlyIf (get (target isCreature) (.up 2) (.up 2) none)
        (.compareAmt (.statOf (.stat .power) it) .eq 2) none)
    out.2 = [] ∧ (resolveCandidates .bare .one (nextClause out.1)).map (·.id) = [0] := by decide

/-- "Deal X damage to target creature if X is 2. Tap it.": the condition brings X into scope,
so the branch, stepped again, introduces one referent fewer than it announced; its ids stay
clear of the condition's, no two referents share one, and "it" reads the target. -/
theorem conditionalBranchIdsNeverCollideWithTheCondition :
    let line := doOnlyIf (dealDamage (letter .x) (target isCreature))
      (compareAmt (letter .x) .eq 2) none
    let out := (stepAction 12 .postEvent .obj (DRS.ofList []) line).1
    let ids := out.referents.map (·.id)
    Action.check (DRS.ofList []) (.sequentially [line, .setStatus .tapped it]) = [] ∧
      ids.eraseDups.length = ids.length ∧
      (resolveCandidates .bare .one (nextClause out)).map (·.det) = [.target] := by decide

/-- "If you control a creature, destroy a creature. Otherwise, tap it.": the other branch reads
what the branch announces (its targets, the amounts it names), never a participant the branch
would choose as it resolves, so "it" finds nothing [CR#601.2c,608.2c]. -/
theorem badOtherwiseReadsIndefiniteThenArm :
    Action.check (DRS.ofList [])
      (.doIf (exists_ (and [isCreature, youControl])) (destroy (a isCreature))
        (some (.setStatus .tapped it))) = [.anaphor .bare .one 0] := by decide

/-- "A creature gets +2/+2 if its power is 2.": a creature the branch chooses as it resolves is
not chosen when the condition is checked, so the condition cannot read it; only the branch's
targets are [CR#601.2c]. -/
theorem conditionReadsOnlyTheBranchsTargets :
    let out := stepAction 12 .postEvent .obj (DRS.ofList [])
      (doOnlyIf (get (a isCreature) (.up 2) (.up 2) none)
        (.compareAmt (.statOf (.stat .power) it) .eq 2) none)
    out.2 = [.anaphor .bare .one 0] := by decide

theorem prohibitionsReadTheirCallerScopedSpellParameter :
    let announced := nextClause ((stepNounPhrase 12 (DRS.ofList []) .subj true (a isSpell)).1)
    let out := stepStaticSpec 16 announced (cant any (counter (that .spell)))
    out.2 = [] ∧ (out.1.findById? 0).map (·.zone) = some (some .stack) := by decide

theorem prohibitionsReadTheirCallerScopedPermanentParameter :
    let announced := nextClause ((stepNounPhrase 12 (DRS.ofList []) .subj true (a isCreature)).1)
    ([destroy, sacrifice, regeneration].map fun verb =>
      (stepStaticSpec 16 announced (cant any (verb (that (.type .creature))))).2) =
      [[], [], []] := by decide

theorem prohibitionsStillRefuseMissingCallerParameters :
    StaticSpec.check (DRS.ofList []) (cant any (counter (that .spell))) =
      [.anaphor (.word .spell) .one 0] := by decide

theorem iterationCanReadDepartedPermanentsWithoutChangingTheirZone :
    let dead := (stepAction 16 .postEvent .obj (DRS.ofList [])
      (destroy (allOf isCreature))).1
    let out := stepAction 16 .postEvent .obj (nextClause dead)
      (.doForEach (those .permanent) (instruct (controllerOf (that .permanent)) (draw 1)))
    out.2 = [] ∧ (out.1.visibleReferents.filter (fun ref => ref.kind == .object)).all
      (fun ref => ref.zone == some .graveyard) = true := by decide +kernel

theorem lookingBackAtAnEntryKeepsTheCurrentReferent :
    let initial := { nextClause ((stepGameEvent 16 (DRS.ofList [])
      (does thisCreature enter)).1) with
      triggerEvents := (does thisCreature enter).triggerEvents }
    let original := initial.referents.head?.map (·.id)
    let out := stepCondition 16 initial
      (.did ⟨.any, .move it (graveyardOf .you) (some battlefield) .arrival [],
        some .triggering⟩)
    out.2 = [] ∧ (original.bind out.1.findById?).map (·.zone) = some (some .battlefield) ∧
      out.1.historicalReferentIds = [] := by decide +kernel

theorem aNewHistoricalDeathStillPublishesItsMovedReferent :
    let out := stepCondition 16 (DRS.ofList [])
      (.did ⟨.any, destroy (a isCreature), some (.duration soFarThisTurn none)⟩)
    out.2 = [] ∧ out.1.visibleReferents.any
      (fun ref => ref.zone == some .graveyard && ref.id == 1) = true := by decide +kernel

private def createdCreature : Action :=
  .createObject 1
    (.token (.written { characteristics := { types := [.creature] } }) []) none

private def creatureCreationLoop : Action :=
  .doForEach (.described .all (.hasType .creature)) createdCreature

theorem aCreationLoopPublishesAFreshGroupRatherThanItsLocalIdentity :
    let out := stepAction 12 .postEvent .obj (DRS.ofList []) creatureCreationLoop
    let groups := out.1.visibleReferents.filter (·.createdResult)
    out.2 = [] ∧ groups.map (·.id) = [3] ∧ groups.map (·.plur) = [.many] ∧
      groups.all (fun ref => ref.idents.isEmpty && ref.payload.size.isNone) = true ∧
      (out.1.findById? 1).isNone = true ∧ (out.1.findById? 2).isNone = true ∧
      NounPhrase.checkAt (some .object) (nextClause out.1) (those .token) = [] := by decide +kernel

theorem aTokenMentionIsNotACreationResult :
    let mention := Action.putCounters 1 (.printed p1p1Counter)
      (.described (.a .unmarked) (.isToken))
    let out := stepAction 12 .postEvent .obj (DRS.ofList [])
      (.doForEach (.described .all (.hasType .creature)) mention)
    out.2 = [] ∧ out.1.visibleReferents.any (·.createdResult) = false ∧
      (resolveCandidates (.word .token) .many (nextClause out.1)).isEmpty = true := by decide +kernel

theorem prospectiveCreationDoesNotPublishALoopResult :
    let out := stepAction 12 .preEvent .obj (DRS.ofList []) creatureCreationLoop
    out.2 = [] ∧ out.1.visibleReferents.any (·.createdResult) = false ∧
      (resolveCandidates (.word .token) .many (nextClause out.1)).isEmpty = true := by decide +kernel

theorem nestedCreationLoopsRetainOnlyTheirFreshAggregate :
    let out := stepAction 16 .postEvent .obj (DRS.ofList [])
      (.doForEach (.described .all (.hasType .land)) creatureCreationLoop)
    let groups := out.1.visibleReferents.filter (·.createdResult)
    out.2 = [] ∧ groups.length = 1 ∧ groups.all (fun ref => ref.plur == .many) = true ∧
      (resolveCandidates (.word .token) .one (nextClause out.1)).isEmpty = true ∧
      NounPhrase.checkAt (some .object) (nextClause out.1) (those .token) = [] := by decide +kernel

theorem creationResultsKeepTheirActualDestinationAfterAMove :
    let moved := Action.sequentially [createdCreature,
      .move (.pro .bare .one) (battlefield) (some exileZone) .departure []]
    let out := stepAction 16 .postEvent .obj (DRS.ofList [])
      (.doForEach (.described .all (.hasType .creature)) moved)
    let groups := out.1.visibleReferents.filter (·.createdResult)
    out.2 = [] ∧ groups.length = 1 ∧
      groups.all (fun ref => ref.zone == some .exile) = true ∧
      (stepAction 8 .postEvent .obj (nextClause out.1)
        (.setStatus .tapped (those .token))).2 = [.anaphor (.word .token) .many 0] := by decide +kernel

theorem movingAnOuterCreationInsideALoopDoesNotCreateAPluralResult :
    let initial := nextClause ((stepAction 8 .postEvent .obj (DRS.ofList []) createdCreature).1)
    let out := stepAction 12 .postEvent .obj initial
      (.doForEach (.described .all (.hasType .creature))
        (.move (.pro (.word .token) .one) (battlefield)
          (some exileZone) .departure []))
    let created := out.1.visibleReferents.filter (·.createdResult)
    out.2 = [] ∧ created.map (·.id) = [3] ∧ created.map (·.plur) = [.one] ∧
      created.all (fun ref => ref.zone == some .exile) = true ∧
      (out.1.findById? 0).isNone = true ∧ (out.1.findById? 2).isNone = true ∧
      (resolveCandidates (.word .token) .many (nextClause out.1)).isEmpty = true := by decide +kernel

theorem countersOnItReadsTheStatSubjectInsideTheSmallClause :
    let initial := (stepNounPhrase 16 (DRS.ofList []) .subj true
      (target isCreature)).1
    let result := stepAmount 16 initial (countersOn p1p1Counter it)
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧
      (initial.findById? 0).map (·.role) = some .subj ∧
      (initial.pendingRoles.contains (0, .subj)) = true := by decide +kernel

private def weepingAngelRecipientContext : DRS :=
  (stepWouldClauseWith (stepOps 16) (DRS.ofList [])
    (.would (.actor thisCreature)
      (.dealDamage .anyAmount (.firstMention "weeping.recipient" (a isCreature)) (some .combat)))).1

theorem weepingAngelNestedPossessorDoesNotMakeItsCreatureAClauseMate :
    let initial := nextClause weepingAngelRecipientContext
    let owner := stepNounPhrase 16 initial .subj true (ownerOf (that (.type .creature)))
    let patient := stepNounPhrase 16 owner.1 .obj true
      (.laterMention "weeping.recipient" it)
    owner.2.2 = [] ∧ patient.2.2 = [] ∧
      owner.2.1 = [4] ∧ patient.2.1 = [2] ∧
      (owner.1.pendingRoles.contains (2, .subj)) = false ∧
      (owner.1.pendingRoles.contains (4, .subj)) = true := by decide +kernel

theorem weepingAngelOwnerCanShuffleItsCreatureIntoTheirLibrary :
    Action.check (nextClause weepingAngelRecipientContext)
      (instruct (ownerOf (that (.type .creature)))
        (shuffleInto (.laterMention "weeping.recipient" it))) = [] := by decide +kernel

theorem weepingAngelRawPronounCurrentlySelectsTheDamageSource :
    let owner := stepNounPhrase 16 (nextClause weepingAngelRecipientContext)
      .subj true (ownerOf (that (.type .creature)))
    let raw := stepNounPhrase 16 owner.1 .obj true it
    owner.2.2 = [] ∧ raw.2.2 = [] ∧ raw.2.1 = [0] ∧
      (identityBinding "weeping.recipient" raw.1).map (·.id) = some 2 := by decide +kernel

theorem weepingAngelDesignatedRecipientMovesAndTheSourceDoesNot :
    let shuffleAction := shuffleInto (.laterMention "weeping.recipient" it)
    let movePrefix := match shuffleAction with
      | .ownScope name (.sequentially (first :: _)) => Action.ownScope name first
      | _ => .sequentially []
    let initial := nextClause weepingAngelRecipientContext
    let moved := stepAction 12 .postEvent .obj initial
      (instruct (ownerOf (that (.type .creature))) movePrefix)
    let out := stepAction 12 .postEvent .obj initial
      (instruct (ownerOf (that (.type .creature))) shuffleAction)
    moved.2 = [] ∧ out.2 = [] ∧
      (moved.1.visibleReferents.filter
        (fun ref => ref.ty.contains .creature && ref.zone == some .library)).length = 1 ∧
      (out.1.findById? 0).map (·.zone) = some (some .battlefield) ∧
      (out.1.findById? 2).isNone = true := by decide +kernel

end Semantics.Proofs.DrtCards
