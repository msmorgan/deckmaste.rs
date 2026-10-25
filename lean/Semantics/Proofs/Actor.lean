import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Actor

The performer of an action and the handoff that names another one. `theActor` is whoever
performs the action it is part of: by default, the player addressed by the text
[CR#109.5], unless an enclosing `instruct` hands it to another player or to a permanent. Each pin is closed by `decide`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Actor

private def actionResult (drs : DRS) (i : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs i).1
private def nounResult (drs : DRS) (n : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .obj true n).1

private def handoff (drs : DRS) (subject : NounPhrase) : DRS :=
  let (after, ids, _) := stepNounPhrase 64 drs .subj true subject
  let actorShape : ActorShape := {
    kind := subject.kindIn after .player
    isYou := subject.isYou
    ascribable := false
    refIds := ids }
  after.pushPerformer actorShape

private def performerIsYou (drs : DRS) : Bool :=
  ((actorShape? drs).map (·.isYou)).getD true

private def products (drs : DRS) : List Payload :=
  (drs.referents.filter (fun r => r.kind != .player)).map (·.payload)

/-- Kind, number and determiner of each binding, newest first. -/
private def shape (bs : DRS) : List (Kind × Plurality × Determiner) :=
  bs.referents.map fun b => (b.kind, b.plur, b.det)

/-- One player already named, for "they". -/
private def onePlayer : DRS := (DRS.ofList [{ det := .the, plur := .one, payload := .player false }])

/-! ## Cards -/

/-- Thoughtseize: "Target player reveals their hand. You choose a nonland card from it. That
player discards that card. You lose 2 life." The handoff leaves the target player published, so
"their hand" and the second handoff to "that player" read it. "A nonland card from it" says
`isCard`: a token can sit in a hand until state-based actions
are checked [CR#111.7,704.5d]. -/
theorem okThoughtseize :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (target .anyPlayer) revealHand,
          choose (a (.and [.not isLand, .and [.isCard, .in_ (handOf they)]])),
          instruct they (discard (that .card)),
          loseLife 2 ]) = [] := by
  decide

/-- Burglar Rat: "When Burglar Rat enters, each opponent discards a card." The discard names no
zone: the deed's patient lives in a hand [CR#701.9a]. -/
theorem okBurglarRat : Action.check (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (discard (a .isCard))) = [] := by
  decide

/-- "Each opponent discards a card. Exile them.": what each opponent's body introduced is
published as a group, as `doForEach` publishes it. -/
theorem okDistributedDiscardReadsAsGroup :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (each (.isOpponentOf .you)) (discard (a .isCard)), exile them]) = [] := by
  decide

/-- "Each opponent discards a card. Exile it.": one card per opponent, so the singular read has
no antecedent (`okDistributedDiscardReadsAsGroup` is the plural read; compare
`Choice.badDistributedChoiceReadSingular`). -/
theorem badDistributedDiscardReadSingular :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (each (.isOpponentOf .you)) (discard (a .isCard)), exile it])
      = [.anaphor .bare .one 0] := by
  decide

/-- Hymn to Tourach: "Target player discards two cards at random." [CR#701.9b] -/
theorem okHymnToTourach :
    Action.check (DRS.ofList [])
      (instruct (target .anyPlayer) (discard (countedAtRandom (exactly 2) .isCard))) = [] := by
  decide

/-- "Amass Orcs 2" [CR#701.47a]: with no handoff the controller amasses. -/
theorem okAmassBare : Action.check (DRS.ofList []) (amass (orc) 2) = [] := by decide

/-- Azog: "Destroy target creature. Its controller amasses Goblins 2." The performer is the
destroyed creature's controller; the reminder's "that creature" and "it" are the chosen Army. -/
theorem okAzogControllerAmasses :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature), instruct (controllerOf it) (amass (goblin) 2)])
      = [] := by
  decide

/-- D′ selects the newly chosen Army after an earlier creature is destroyed. -/
theorem amassBareItAfterDestroyChecks :
    Action.check (DRS.ofList []) (.sequentially [destroy (target isCreature), amass (goblin) 2])
      = [] := by
  decide

/-- A literal amass body, written with bare "that creature" and "it". -/
private def literalAmass : Action :=
  .sequentially
    [ .doIf (.not (exists_ army))
        (create 1 (.written (creatureToken 0 0 [.black] [orc, Subtype.army])))
        none,
      choose (a army),
      .putCounters 2 (.printed p1p1Counter) (that (.type .creature)),
      .doIf (.not (.matches it (.hasSubtype (orc))))
        (.establish (qualityChange it .adds
          (.bundle { characteristics := { subtypes := [orc] } } none)) none)
        none ]

/-- "Amass Orcs 2" in the literal spelling checks on its own. -/
theorem okLiteralAmassBare : Action.check (DRS.ofList []) literalAmass = [] := by decide

/-- "Target opponent amasses Orcs 2" in the literal spelling: the opponent is a player, so no
second object is in view, and it checks. -/
theorem okLiteralAmassHandedOff : Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) literalAmass) = [] := by
  decide

/-! ## A keyword body's own reference scope

A keyword action body is read in its own Reference Scope: it sees the performer, its parameters
and what it introduces, never the calling text's other mentions (`docs/decisions/semantics-v2.md`
§7, ruling 2026-10-06). A keyword action's Definition writes the scope (`Action.ownScope`) with the action's
deed as its header and each parameter `inCaller`, read in the caller's view. -/

/-- The literal amass body in its own scope, under the amass deed. -/
private def scopedAmass : Action := .ownScope (some (.action "amass")) literalAmass

/-- "Amass Orcs 2" in its scope checks on its own, as the bare body does. -/
theorem okScopedAmass : Action.check (DRS.ofList []) scopedAmass = [] := by decide

/-- "Target opponent amasses Orcs 2" in its scope checks, as the bare body does. -/
theorem okScopedAmassHandedOff : Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) scopedAmass) = [] := by
  decide

/-- After destroying a creature, literal amass selects its own Army: the destroyed
creature is outside the battlefield domain of the Army read. -/
theorem literalAmassAfterDestroyReadsItsArmy :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature), instruct (controllerOf it) literalAmass])
      = [] := by
  decide

/-- Azog, Moria's Ruin, with the body in its own scope: "Destroy target creature. Its
controller amasses Goblins 2." The destroyed creature is the caller's mention, masked from the
body, so "that creature" and "it" are the chosen Army alone. -/
theorem okScopedAmassAfterDestroy :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature), instruct (controllerOf it) scopedAmass]) = [] := by
  decide

/-- The amass body with its amount a parameter, as the loader writes it: read in the caller's
view (`inCaller`). -/
private def scopedAmassOf (amount : Amount) : Action :=
  .ownScope (some (.action "amass")) (.sequentially
    [ .doIf (.not (exists_ army))
        (create 1 (.written (creatureToken 0 0 [.black] [goblin, Subtype.army])))
        none,
      choose (a army),
      .putCounters (.inCaller amount) (.printed p1p1Counter) (that (.type .creature)),
      .doIf (.not (.matches it (.hasSubtype (goblin))))
        (.establish (qualityChange it .adds
          (.bundle { characteristics := { subtypes := [goblin] } } none)) none)
        none ])

/-- Azog's shape with X: "Its controller amasses Goblins X, where X is its power." The amount
is a parameter, so it reads the destroyed creature in the caller's view while the body's own
"that creature" and "it" read the Army. -/
theorem okAzogScopedAmassX :
    Action.check (DRS.ofList [])
      (.sequentially
        [ destroy (target isCreature),
          instruct (controllerOf it) (scopedAmassOf (.statOf (.stat .power) it)) ])
      = [] := by
  decide

/-- A body's own "it" does not see the calling text's creature, which the same words written
outside the scope read. -/
theorem scopedBodyCannotReadTheCaller :
    let caller : DRS :=
      (DRS.ofList [{ det := .target, plur := .one, payload := .object [.creature] (some .battlefield) none none none }])
    ((Action.check caller (.ownScope none (destroy it))).contains
        (.anaphor .bare .one 0),
     Action.check caller (destroy it)) = (true, []) := by
  decide

/-- A keyword ability's body is read in its own scope too: the card's mentions are not in view
of its abilities (`Ability.check` reads the body from an empty context), so a body that reads
"it" finds nothing, while the same ability written on the card reads the card's mention. -/
theorem keywordAbilityBodyCannotReadItsCard :
    let card : DRS := (DRS.ofList [{ det := .the, plur := .one, payload := .object [.creature] (some .battlefield) none none none }])
    ((Ability.check card (.keyword (.named "flying") [] [Primitives.Ability.spell none (destroy it)])).contains
        (.anaphor .bare .one 0),
     Ability.check card (Primitives.Ability.spell none (destroy it))) = (true, []) := by
  decide

/-! ## The actor outside a handoff is the controller -/

/-- "Draw a card.": the same refusals and the same published bindings as handed to "you". -/
theorem actorDrawIsYouDraw :
    Action.check (DRS.ofList []) (draw 1) = Action.check (DRS.ofList []) (instruct .you (draw 1)) ∧
      (products (actionResult (DRS.ofList []) (draw 1)) == products (actionResult (DRS.ofList []) (instruct .you (draw 1))))
        = true := by
  decide

/-- "Lose 2 life.": as handed to "you". -/
theorem actorLoseLifeIsYouLoseLife :
    Action.check (DRS.ofList []) (loseLife 2) = Action.check (DRS.ofList []) (instruct .you (loseLife 2)) ∧
      (products (actionResult (DRS.ofList []) (loseLife 2)) ==
        products (actionResult (DRS.ofList []) (instruct .you (loseLife 2)))) = true := by
  decide

/-- "You may pay 1 life. If you do, draw a card." (`Choice.okMatchedPayer` with the actor) -/
theorem okActorMatchedPayer :
    Action.check (DRS.ofList [])
      (may (payLife 1) none
        (some (draw 1))) = [] := by
  decide

/-- "Pay 2 life: Draw a card." (`Keyword.okOwnPayerCost` with the actor) -/
theorem okActorOwnPayerCost :
    Ability.check (DRS.ofList []) (.activated (payLife 2) (draw 1) none none none none) = [] := by
  decide

/-- "Target opponent loses 2 life: Draw a card.": a cost may hand its action to another
player, and a life payment inside the handoff is that player's, not the controller's. Who pays a
cost is not checked (`lean-checker-sweep-costs`); the cost's performer is still the handed
player, read in the context the cost's handoff builds. -/
theorem badHandedOffLifeCost :
    Ability.check (DRS.ofList [])
      (.activated (.perform (instruct (target (.isOpponentOf .you)) (loseLife 2))) (draw 1)
        none none none none)
      = [] ∧
      performerIsYou (handoff (DRS.ofList []) (target (.isOpponentOf .you))) = false := by
  decide

/-! ## Costs: each handoff twin judged as its explicit-agent form

Every cost action on the bench or in the pins whose agent is not "you", written once with
the explicit agent and once handed off; both gave the same refusals. The explicit-agent spelling
is gone (`semantics-v2-drop-agent-fields`): each pin's second half now names the handoff the bench
writes for the same card, so it states that the bench's own spelling is the one judged. -/

private def drawOne : Action := draw 1

/-- Wall of Shards: "Cumulative upkeep—An opponent gains 1 life." -/
theorem wallOfShardsHandoffTwin :
    Ability.check (DRS.ofList []) (cumulativeUpkeep (.perform (instruct anOpponent (gainLife 1)))) = [] ∧
      Ability.check (DRS.ofList []) (cumulativeUpkeep (.perform (instruct anOpponent (gainLife 1)))) =
        Ability.check (DRS.ofList []) (cumulativeUpkeep (.perform (instruct anOpponent (gainLife 1)))) := by
  decide

/-- Varchild's War-Riders: "Cumulative upkeep—Have an opponent create a 1/1 red Survivor creature
token." -/
theorem varchildsWarRidersHandoffTwin :
    Ability.check (DRS.ofList [])
        (cumulativeUpkeep (.perform (instruct anOpponent
          (create 1 (.written (creatureToken 1 1 [.red] [survivor])))))) = [] ∧
      Ability.check (DRS.ofList [])
          (cumulativeUpkeep (.perform (instruct anOpponent
            (create 1 (.written (creatureToken 1 1 [.red] [survivor])))))) =
        Ability.check (DRS.ofList [])
          (cumulativeUpkeep (.perform (instruct anOpponent (create 1
            (.written (creatureToken 1 1 [.red] [survivor])) [])))) := by
  decide

private def controlsForest : Condition :=
  exists_ (.and [isLand, .hasSubtype (forest), .hasPossessor .controller .you])

/-- Invigorate: "If you control a Forest, rather than pay this spell's mana cost, you may have an
opponent gain 3 life." -/
theorem invigorateHandoffTwin :
    Ability.check (DRS.ofList [])
        (.static (onlyWhile (.altCost .this (some (.perform (instruct anOpponent (gainLife 3)))))
          controlsForest)) = [] ∧
      Ability.check (DRS.ofList [])
          (.static (onlyWhile (.altCost .this (some (.perform (instruct anOpponent (gainLife 3)))))
            controlsForest)) =
        Ability.check (DRS.ofList [])
          (.static (onlyWhile (.altCost .this (some (.perform (instruct anOpponent (gainLife 3)))))
            controlsForest)) := by
  decide

private def blockersTheyControl : Amount :=
  times 1 (countOf (.and [isCreature, isBlocking, .hasPossessor .controller they]))

/-- Heat Wave: "Nonblue creatures can't block creatures you control unless their controller pays 1
life for each blocking creature they control." -/
theorem heatWaveHandoffTwin :
    Ability.check (DRS.ofList [])
        (.static (.gatedBy (.perform (instruct they (loseLife blockersTheyControl)))
            (.actor (allOf (.and [isCreature, .not (.colorIs .blue)])))
            (.block (allOf (and [isCreature, youControl]))) {})) = [] ∧
      Ability.check (DRS.ofList [])
          (.static (.gatedBy (.perform (instruct they (loseLife blockersTheyControl)))
            (.actor (allOf (.and [isCreature, .not (.colorIs .blue)])))
            (.block (allOf (and [isCreature, youControl]))) {})) =
        Ability.check (DRS.ofList [])
          (.static (.gatedBy (.perform (instruct they (loseLife blockersTheyControl)))
            (.actor (allOf (.and [isCreature, .not (.colorIs .blue)])))
            (.block (allOf (and [isCreature, youControl]))) {})) := by
  decide

/-- Killing Wave: "For each creature, its controller sacrifices it unless they pay X life." -/
theorem killingWaveHandoffTwin :
    Action.check (DRS.ofList [])
        (Primitives.Action.doForEach (each isCreature)
          (instruct (controllerOf it)
            (doUnless (instruct they (sacrifice it)) (.perform (instruct they (loseLife (.letter .x))))))) = [] ∧
      Action.check (DRS.ofList [])
          (Primitives.Action.doForEach (each isCreature)
            (instruct (controllerOf it)
              (doUnless (instruct they (sacrifice it)) (.perform (instruct they (loseLife (.letter .x))))))) =
        Action.check (DRS.ofList [])
          (Primitives.Action.doForEach (each isCreature)
            (instruct (controllerOf it)
              (doUnless (instruct they (sacrifice it)) (.perform (instruct they (loseLife (.letter .x))))))) := by
  decide

/-- "An opponent sacrifices a creature: Draw a card." (the pin
`Keyword.badForeignSacrificeCost`, retired with `costPaidByYou` by `lean-checker-sweep-costs`) -/
theorem foreignSacrificeCostHandoffTwin :
    Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (sacrifice (a isCreature)))) drawOne
        none none none none) = [] ∧
      Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (sacrifice (a isCreature)))) drawOne
          none none none none) =
        Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (sacrifice (a isCreature)))) drawOne
          none none none none) := by
  decide

/-- "An opponent pays 2 life: Draw a card." (the pin `Keyword.badForeignPayerCost`,
retired with `costPaidByYou` by `lean-checker-sweep-costs`) -/
theorem foreignPayerCostHandoffTwin :
    Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (loseLife 2))) drawOne
        none none none none) = [] ∧
      Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (loseLife 2))) drawOne
          none none none none) =
        Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (loseLife 2))) drawOne none none none
          none) := by
  decide

/-- "Cumulative upkeep—An opponent loses 1 life." (the pin
`Mana.badOpponentPaysYourCost`, retired with `keywordCostPaidByYou` by `lean-checker-sweep-costs`) -/
theorem opponentPaysYourCostHandoffTwin :
    Ability.check (DRS.ofList []) (keywordCosting (.named "cumulativeUpkeep") (.perform (instruct anOpponent (loseLife 1))))
        = [] ∧
      Ability.check (DRS.ofList []) (keywordCosting (.named "cumulativeUpkeep") (.perform (instruct anOpponent (loseLife 1)))) =
        Ability.check (DRS.ofList []) (keywordCosting (.named "cumulativeUpkeep") (.perform (instruct anOpponent (loseLife 1)))) := by
  decide

/-- "You may pay an opponent's 1 life. If you do, draw a card." (the pin
`Choice.badMismatchedPayer`, retired with `payAgrees` by `lean-checker-sweep-costs`) -/
theorem mismatchedPayerHandoffTwin :
    Action.check (DRS.ofList [])
        (may (.pay (.perform (instruct anOpponent (loseLife 1))) .once .paid none none)
          none (some drawOne)) = [] ∧
      Action.check (DRS.ofList [])
          (may (.pay (.perform (instruct anOpponent (loseLife 1))) .once .paid none none)
            none (some drawOne)) =
        Action.check (DRS.ofList [])
          (may (.pay (.perform (instruct anOpponent (loseLife 1))) .once .paid none none)
            none (some drawOne)) := by
  decide

/-- "Pay 2 life: Draw a card." handed to "you" (`Keyword.okOwnPayerCost`). -/
theorem ownPayerCostHandoffTwin :
    Ability.check (DRS.ofList []) (.activated (.perform (instruct .you (loseLife 2))) drawOne
        none none none none) = [] ∧
      Ability.check (DRS.ofList []) (.activated (.perform (instruct .you (loseLife 2))) drawOne
          none none none none) =
        Ability.check (DRS.ofList []) (.activated (payLife 2) drawOne none none none none) := by
  decide

/-! ## A captured `theActor` keeps its handoff -/

private semantic_template capturedLifeLoss (who : NounPhrase) : Action :=
  .instruct who (.changeLife (.down 1))

/-- "You may pay 1 life of <who>. If you do, draw a card.", the payer being "you". -/
private def youPayWith (c : Cost) : Action :=
  instruct .you (may (.pay c .once .paid none none) none (some (draw 1)))

/-- With no handoff, `theActor` passed through a `capture` parameter is the controller, so the
controller pays their own life. -/
theorem okCapturedActorIsYou :
    Action.check (DRS.ofList []) (youPayWith (.perform (capturedLifeLoss theActor))) = [] := by
  decide

/-- Inside "target opponent …", "you may pay that player's 1 life", captured or not. The payer
"you" is a handoff back to the controller, inside which `theActor` is the controller again, so the
opponent is named "that player" (`capturedActorIsTheHandedPlayer` captures `theActor` itself inside
the opponent's handoff). Who pays a cost is not checked (`lean-checker-sweep-costs`); the actor
inside the inner handoff, past the capture, is still "you". -/
theorem badCapturedActorIsTheHandedPlayer :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (youPayWith (.perform (capturedLifeLoss they))))
        = [] ∧
      Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (youPayWith (.perform (instruct they (loseLife 1)))))
        = [] ∧
      performerIsYou
        (handoff (handoff (DRS.ofList []) (target (.isOpponentOf .you))) .you) =
        true := by
  decide

/-- Inside "target opponent …", `theActor` passed through a `capture` parameter is the opponent:
"Target opponent loses 1 life: Draw a card." through the captured performer is admitted as the
uncaptured handoff is (`badHandedOffLifeCost`), and the captured `theActor` still reads as the
handed player, not "you". -/
theorem capturedActorIsTheHandedPlayer :
    Ability.check (DRS.ofList [])
      (.activated (.perform (instruct (target (.isOpponentOf .you)) (capturedLifeLoss theActor))) (draw 1)
        none none none none)
      = [] ∧
      performerIsYou (handoff (DRS.ofList []) (target (.isOpponentOf .you))) =
        false := by
  decide

/-! ## Handoffs -/

/-- "Each opponent may pay {2}. If they don't, you draw a card.": `instruct you` inside another
player's handoff. -/
theorem okEachOpponentMayPayElseYouDraw :
    Action.check (DRS.ofList [])
      (instruct (each (.isOpponentOf .you))
        (may (.pay (.mana [generic 2]) .once .paid none none) none
          (some (instruct .you (draw 1))))) = [] := by
  decide

/-- The same sentence through `doUnless`, handed to each opponent, who decides. -/
theorem okEachOpponentMayPayElseYouDrawUnless :
    Action.check (DRS.ofList [])
      (instruct (each (.isOpponentOf .you)) (doUnless (instruct .you (draw 1)) (.mana [generic 2]))) = [] := by
  decide

/-- "Each player chooses a creature they control." through the handoff
(`Choice.okAgentScopedChoice` through the chooser slot). -/
theorem okHandedScopedChoice :
    Action.check (DRS.ofList [])
      (instruct (each .anyPlayer) (choose (a (.and [isCreature, .hasPossessor .controller they]))))
      = [] := by
  decide

/-- "Choose a creature you control.", "you" being the performer. -/
theorem okActorControlsChoice : Action.check (DRS.ofList []) (choose (a (.and [isCreature, actorControls]))) = [] := by
  decide

/-- "You choose a creature they control.": "they" never reads "you", a self-referent even as
the handoff's performer (`Determiner.self`, `performerInView`; owner ruling 2026-10-24), so it has
no antecedent (`Choice.badUnchooseredTheyControl`); "you control" is how the text says it. -/
theorem handedToYouThirdPersonDoesNotReadYou :
    Action.check (DRS.ofList [])
        (instruct .you (choose (a (.and [isCreature, .hasPossessor .controller they]))))
        = [.anaphor (.word .player) .one 0] ∧
      Action.check (DRS.ofList [])
        (instruct .you (choose (a (.and [isCreature, .hasPossessor .controller .you])))) = [] := by
  decide

/-- "Target opponent may pay an opponent's 1 life. If they do, they draw a card.": inside the
handoff the payer is the opponent. Who pays a cost is not checked (`lean-checker-sweep-costs`). -/
theorem okHandedPayerIsTheTarget :
    Action.check (DRS.ofList [])
      (instruct (target (.isOpponentOf .you))
        (may (.pay (.perform (instruct anOpponent (loseLife 1))) .once .paid none none) none
          (some (draw 1)))) = [] := by
  decide

/-- Nested handoffs: the innermost one names the performer, so `instruct you` inside an opponent's
handoff makes the actor the controller again. Who pays a cost is not checked
(`lean-checker-sweep-costs`); the innermost handoff's actor is "you". -/
theorem badNestedHandoffInnermost :
    Action.check (DRS.ofList [])
      (instruct (target (.isOpponentOf .you))
        (instruct .you
          (may (.pay (.perform (instruct anOpponent (loseLife 1))) .once .paid none none) none
            (some (draw 1)))))
      = [] ∧
      performerIsYou (handoff (handoff (DRS.ofList []) (target (.isOpponentOf .you))) .you) = true ∧
      performerIsYou (handoff (DRS.ofList []) (target (.isOpponentOf .you))) = false := by
  decide

/-- "Target opponent … Until end of turn, target creature gains '{T}: You may pay an
opponent's 1 life. If you do, draw a card.'": a granted ability is performed by its own
controller, so the handoff around the grant does not reach the payer inside it
(`okHandedPayerIsTheTarget` is the same body handed off directly). Who pays a cost is not checked
(`lean-checker-sweep-costs`); the granted ability's actor is its own controller, "you". -/
theorem badGrantedAbilityKeepsItsOwnPerformer :
    Action.check (DRS.ofList [])
      (instruct (target (.isOpponentOf .you))
        (.establish
          (Templates.abilityGrant (target isCreature)
            (.activated .tapSymbol
              (may (.pay (.perform (instruct anOpponent (loseLife 1))) .once .paid none none) none
                (some (draw 1)))
              none none none none))
          (some untilEndOfTurn)))
      = [] ∧
      performerIsYou (DRS.ofList []) = true := by
  decide

/-! ## Statics -/

/-- "You may cast spells from your graveyard.": with no handoff in a static ability the
permitted player is the controller, and the clause checks as the `you` spelling does. -/
theorem okStaticActorPermission :
    Ability.check (DRS.ofList []) (.static (can theActor (cast (allOf isSpell) (from_ := some (graveyardOf theActor))))) = [] ∧
      Ability.check (DRS.ofList []) (.static (can theActor (cast (allOf isSpell) (from_ := some (graveyardOf theActor))))) =
        Ability.check (DRS.ofList [])
          (.static (can you (cast (allOf isSpell) (from_ := some (graveyardOf .you))))) := by
  decide

/-- "Exile target creature. Its owner may cast it from exile.": the permission handed to the
exiled card's owner. -/
theorem okHandedCastPermission :
    Action.check (DRS.ofList [])
      (.sequentially
        [ exile (target isCreature),
          instruct (ownerOf it) (.establish (can theActor (cast it (from_ := some exileZone))) none) ]) = [] := by
  decide

/-! ## Named actions carry their performer -/

/-- "That player discards that card." as `ownScope (some (.action "discard")) …` under the handoff, the
deed performed by the actor in context [CR#701.9a]. -/
theorem okDiscardWrapperCarriesActor :
    Action.check onePlayer
      (instruct they
        (.ownScope (some (.action "discard")) (.move (a .isCard) ((.zone .hand (some theActor))) (some graveyard) .arrival [])))
      = [] := by
  decide

/-- The discard deed admits a player performer at all: "Target player discards a card." handed to
that player. -/
theorem okDiscardWrapperWithAgent :
    Action.check (DRS.ofList [])
      (instruct (target .anyPlayer) (.ownScope (some (.action "discard")) (.move (a .isCard) ((.zone .hand none)) (some graveyard) .arrival [])))
      = [] := by
  decide

/-! ## What a handoff leaves bound -/

/-- A targeted performer stays published after its handoff, below what the body introduced. -/
theorem handedTargetStaysBound :
    shape (actionResult (DRS.ofList []) (instruct (target .anyPlayer) (discard (a .isCard)))) =
      [(.object, .one, .a), (.player, .one, .target)] := by
  decide

/-- "That player": a pronoun performer introduces nothing; the body's mentions sit above the
player it read. -/
theorem handedPronounAddsOnlyTheBody :
    shape (actionResult onePlayer (instruct they (discard (a .isCard)))) =
      [(.object, .one, .a), (.player, .one, .the)] := by
  decide

/-- "Each opponent": the members' mentions and the group, pluralized. -/
theorem handedGroupPublishesPlurals :
    shape (actionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (discard (a .isCard)))) =
      [(.object, .many, .a), (.player, .many, .each)] := by
  decide

/-- `instruct you` publishes exactly what its body does with no handoff. -/
theorem handedToYouIsTransparent :
    (products (actionResult (DRS.ofList []) (instruct .you (discard (a .isCard)))) ==
      products (actionResult (DRS.ofList []) (discard (a .isCard)))) = true := by
  decide

/-! ## A group handoff publishes its amount outcomes as totals

What each member's body introduced is published as a group, but an amount outcome stays one
total, as the explicit plural agent leaves it: "each opponent loses 1 life and you gain life equal
to the total life lost this way" [CR#702.101a]. -/

/-- Syndic of Tithes, its extort body [CR#702.101a] written over `loss` for "each opponent loses
1 life", followed by "you gain that much life". -/
private def syndicOfTithes (loss : Action) : Card :=
  .singleFaced
    { characteristics :=
      { name := "Syndic of Tithes", cost := some [generic 1, pip .white], types := [.creature],
        subtypes := [human, cleric],
        text :=
          [ .keyword (.named "extort") []
              [ whenever
                  (.does (.actor .you) (.procedure .cast (.described (.a .unmarked)
                      (.and [.in_ (.zone .stack none), .not (.abilityHead .anyOnStack)])) .itsOwnCost none none none))
                  (.withContinuation .optional
                    (.pay (.mana [hybridPip .white .black]) .once .paid none none)
                    (some (.sequentially [loss, gainLife .thatMuch])) none) ] ],
        power := some 2, toughness := some 2 } }

/-- Extort handed to each opponent: "that much" reads the one total. -/
theorem okExtortHandedToEachOpponent :
    Card.check (syndicOfTithes (instruct (each (.isOpponentOf .you)) (loseLife 1))) = [] := by
  decide

/-- The handoff twin of the explicit plural agent `changeLife (down 1) (each opponent)`, which is
gone (`semantics-v2-drop-agent-fields`); its replacement, the handoff of the bare life change, is
judged the same as the helper's spelling. -/
theorem extortHandoffTwin :
    Card.check (syndicOfTithes (instruct (each (.isOpponentOf .you)) (.changeLife (.down 1)))) = [] ∧
      Card.check (syndicOfTithes (instruct (each (.isOpponentOf .you)) (loseLife 1))) =
        Card.check (syndicOfTithes (instruct (each (.isOpponentOf .you)) (.changeLife (.down 1)))) := by
  decide

/-- "Each opponent loses 1 life": the group and one `lifeLost` total, exactly what the explicit
plural agent published (its replacement is the handoff of the bare life change). -/
theorem handedGroupPublishesTotal :
    shape (actionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (loseLife 1))) =
        [(.outcome, .one, .the), (.player, .many, .each)] ∧
      (actionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (loseLife 1)) ==
        actionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (.changeLife (.down 1)))) = true := by
  decide

/-- "Each opponent discards a card. You gain that much life.": the body has no amount outcome,
so the group handoff publishes none and "that much" has nothing to read. -/
theorem badGroupHandoffNoOutcomeThatMuch :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (each (.isOpponentOf .you)) (discard (a .isCard)), gainLife .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Loses 1 life and discards a card." -/
private def lossAndDiscard : Action :=
  .sequentially [loseLife 1, discard (a .isCard)]

/-- "Each opponent loses 1 life and discards a card. You gain that much life.": "that much" reads
the life total; the discarded cards stay a group. -/
theorem okGroupHandoffSequenceReadsLifeTotal :
    Action.check (DRS.ofList [])
        (.sequentially
          [ instruct (each (.isOpponentOf .you)) lossAndDiscard,
            gainLife .thatMuch ]) = [] ∧
      Action.check (DRS.ofList [])
        (.sequentially
          [ instruct (each (.isOpponentOf .you)) lossAndDiscard,
            exile them ]) = [] := by
  decide

/-- "Each opponent loses 1 life, then loses 2 life. You gain that much life.": two totals, so
"that much" has no one antecedent. -/
theorem badGroupHandoffTwoTotalsThatMuch :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (each (.isOpponentOf .you)) (.sequentially [loseLife 1, loseLife 2]),
          gainLife .thatMuch ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "For each opponent, that player loses 1 life. You gain that much life.": `doForEach` still
publishes its body's outcomes as a group; only the handoff carries the total. -/
theorem badForEachLossThatMuch :
    Action.check (DRS.ofList [])
      (.sequentially
        [ Primitives.Action.doForEach (each (.isOpponentOf .you)) (instruct they (loseLife 1)),
          gainLife .thatMuch ]) = [.quantOutcomeInScope 0] := by
  decide

/-! ## A permanent can be handed an action

The rules instruct a permanent itself to explore or endure [CR#701.44a,701.63a]; the handoff
takes a permanent as well as a player. Inside it `theActor` is the permanent, and the steps the
rule gives "that permanent's controller" are handed on to `controllerOf theActor`, inside which
`theActor` is the controller and the permanent stays in view as "that permanent". An
object-performed deed
written with no handoff to a permanent is performed by the source permanent. -/

/-- The `explore` declaration's body [CR#701.44a]: the exploring permanent's controller reveals
the top card of their library; a land goes to its owner's hand, otherwise "that permanent" gets
a +1/+1 counter and the card may go to the graveyard. `perm` is how the body names the exploring
permanent. -/
private def exploreBody (perm : NounPhrase) : Action :=
  instruct (controllerOf theActor) (.sequentially
    [ .expose .reveal (.cards (.slice .top 1 (libraryOf theActor))),
      .doIf (.matches (that .card) isLand) (.move (that .card) .anyLocation (some (.zone .hand none)) .arrival [])
        (some (.sequentially
          [ .putCounters 1 (.printed p1p1Counter) perm,
            .withContinuation .optional
              (.move (that .card) .anyLocation (some (.zone .graveyard none)) .arrival []) none none ])) ])

/-- `explore` as the keyword-action loader wraps it: the deed, performed by the actor. -/
private def literalExplore (perm : NounPhrase := that .permanent) : Action :=
  .ownScope (some explore) (exploreBody perm)

/-- `endure N` as defined [CR#701.63a]: the enduring permanent's controller chooses between
the counters on "that permanent" and the Spirit token. -/
private def literalEndure (n : Nat) : Action :=
  .ownScope (some endure) (instruct (controllerOf theActor)
      (.chooseModes (.range (some 1) (some 1))
        [ (none, none, .putCounters (.lit n) (.printed p1p1Counter) (that .permanent)),
          (none, none, create 1 (.written (creatureToken n n [.white] [spirit]))) ]))

/-- "Target creature explores.": handed to the creature, whose controller does the steps. -/
theorem okExploreHandedToTargetCreature :
    Action.check (DRS.ofList []) (instruct (target isCreature) literalExplore) = [] := by
  decide

/-- Deadeye Tracker's "This creature explores.": `this` introduces nothing, so the handoff puts
the permanent in view for "that permanent". -/
theorem okExploreHandedToThisCreature :
    Action.check (DRS.ofList []) (instruct thisCreature literalExplore) = [] := by
  decide

/-- The owner's spelling "that creature" reaches the permanent too, handed to a target creature
or to "this creature". -/
theorem okExploreReadsThatCreature :
    Action.check (DRS.ofList []) (instruct (target isCreature) (literalExplore (that (.type .creature)))) = [] ∧
      Action.check (DRS.ofList []) (instruct thisCreature (literalExplore (that (.type .creature)))) = [] := by
  decide

/-- "This creature endures 1." (the testing card Endure Handoff Probe) -/
theorem okEndureHandedToThisCreature :
    Action.check (DRS.ofList []) (instruct thisCreature (literalEndure 1)) = [] := by
  decide

/-- "Each creature you control explores.": a group of permanents, each exploring in turn. -/
theorem okExploreHandedToEachCreature :
    Action.check (DRS.ofList []) (instruct (each isCreature) literalExplore) = [] := by
  decide

/-- Inside a handoff to a permanent `theActor` is that permanent: "target creature draws a card"
records a creature as the drawing player. Handed on to its controller, the draw checks. -/
theorem actorInPermanentHandoffIsThePermanent :
    Action.check (DRS.ofList []) (instruct (target isCreature) (draw 1))
        = [.kindMismatch .player .object] ∧
      Action.check (DRS.ofList []) (instruct (target isCreature) (instruct (controllerOf theActor) (draw 1)))
        = [] := by
  decide

/-- Explore handed to a player: the body asks for its performer's controller, which a player
does not have, and "that permanent" has no antecedent. -/
theorem badExploreHandedToOpponent :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) literalExplore)
      = [.possessorKind .controller .player, .anaphor (.word .permanent) .one 0] := by
  decide

/-- Printed "Adapt 2" on a permanent's own ability, with no handoff: the recorded actor is the
controller, and the deed is performed by the source permanent [CR#701.46a]. -/
theorem okBareAdaptDefaultsToThis : Action.check (DRS.ofList []) (adapt 2) = [] := by decide

/-- A bare "explore" and "endure 1" default to the source permanent the same way: `theActor` in the
body is "this permanent", and "that permanent" reads it. -/
theorem okBareExploreAndEndureDefaultToThis :
    Action.check (DRS.ofList []) literalExplore = [] ∧ Action.check (DRS.ofList []) (literalEndure 1) = [] := by
  decide

/-! ### A player performer is refused by the deed's own body

No table says which deeds a permanent performs. A keyword action a permanent performs reads its
performer in its Definition body, so a player handed one is refused by ordinary reference
resolution and kind agreement in that body: explore and endure ask for the performer's
controller [CR#701.44a,701.63a], adapt reads the counters on it [CR#701.46a], monstrosity
reads its designation [CR#701.37a], and fight reads its power [CR#701.14a]. -/

/-- "Target opponent adapts 2": handed to another player, the actor is that player, not the
controller, so there is no default to the source permanent, and the body's counter read of
the performer is refused. -/
theorem badAdaptHandedToOpponent :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (adapt 2))
      = [.kindMismatch .object .player] := by
  decide

/-- "Target creature adapts 2" and "This creature adapts 2": the body reads the handed
permanent. -/
theorem okAdaptHandedToAPermanent :
    Action.check (DRS.ofList []) (instruct (target isCreature) (adapt 2)) = [] ∧
      Action.check (DRS.ofList []) (instruct thisCreature (adapt 2)) = [] := by
  decide

/-- "Target opponent explores", with the authored `explore`: the body asks for the player's
controller. -/
theorem badAuthoredExploreHandedToOpponent :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) explore)
      = [.possessorKind .controller .player] := by
  decide

/-- "Target opponent endures 1": the body asks for the player's controller. -/
theorem badEndureHandedToOpponent :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (endure 1))
      = [.possessorKind .controller .player, .anaphor (.word .permanent) .one 0] := by
  decide

/-- "Target opponent monstrosities 2": the body reads the player's designation, and "monstrous"
is a designation an object holds. Bare, or handed a creature, it checks. -/
theorem badMonstrosityHandedToOpponent :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (monstrosity 2))
      = [.kindMismatch .object .player, .designationScope (.named "monstrous" (.heldBy .object))] ∧
      Action.check (DRS.ofList []) (monstrosity 2) = [] ∧
      Action.check (DRS.ofList []) (instruct (target isCreature) (monstrosity 2)) = [] := by
  decide

/-- "Target opponent fights target creature": the body reads the player's power for the damage
it deals and as the creature's recipient. -/
theorem badFightHandedToOpponent :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (fight (target isCreature)))
      = [.kindMismatch .object .player, .kindMismatch .object .player] := by
  decide

/-- "Players can't explore" and "Players can't adapt": the prohibited deed's body is checked
with the players as its performer, and refused the same way. -/
theorem badPlayersCantExploreOrAdapt :
    StaticSpec.check (DRS.ofList []) (.cant (.actor (allOf .anyPlayer)) explore {})
        = [.possessorKind .controller .player] ∧
      StaticSpec.check (DRS.ofList []) (.cant (.actor (allOf .anyPlayer)) (adapt 1) {})
        = [.kindMismatch .object .player] ∧
      StaticSpec.check (DRS.ofList []) (.cant (.actor (allOf isCreature)) explore {}) = [] := by
  decide

/-- "Whenever a creature explores": the event's performer is a permanent; "whenever a player
explores" is refused by the body's controller read, while a player still bolsters. -/
theorem exploreEventTakesAPermanent :
    GameEvent.check (DRS.ofList []) (.does (.actor (a isCreature)) explore) = [] ∧
      GameEvent.check (DRS.ofList []) (.does (.actor (a .anyPlayer)) explore)
        = [.possessorKind .controller .player] ∧
      GameEvent.check (DRS.ofList []) (.does (.actor (a .anyPlayer)) (bolster 1))
        = [] := by
  decide

/-- The source and its controller remain accessible after the handoff, along with the drawn card.
Out of the handoff the Source is a self-referent again (`Determiner.self`, `performerOutOfView`;
owner ruling 2026-10-24). -/
theorem handedToThisPublishesSourceAndController :
    shape (actionResult (DRS.ofList []) (instruct thisCreature (instruct (controllerOf theActor) (draw 1))))
      = [(.object, .one, .the), (.player, .one, .the), (.object, .one, .self)] := by
  decide

theorem amassBareItSelectsChosenArmy :
    let stepsBeforeRead := match literalAmass with
      | .sequentially steps => Action.sequentially (steps.take 2)
      | body => body
    let after := actionResult (DRS.ofList [])
      (.sequentially [destroy (target isCreature), stepsBeforeRead])
    let selected := resolveCandidates .bare .one (nextClause after)
    selected.map (·.id) = [after.nextId - 1] ∧
      selected.map (·.ty) = [[.creature]] := by decide

/-! ## The Source and "you" are self-referents (owner ruling 2026-10-24) -/

/-- "Tap this creature. Untap that creature.": "that creature" never reads the Source, a
self-referent (`Determiner.self`); "Untap this creature" re-mentions it, one referent
(`mentionThis`). -/
theorem thisThenThatCreatureIsRefused :
    let untapping := fun (noun : NounPhrase) => Action.sequentially
      [.setStatus .tapped thisCreature, .setStatus .untapped noun]
    Action.check (DRS.ofList []) (untapping (that (.type .creature))) =
        [.anaphor (.word (.type .creature)) .one 0] ∧
      Action.check (DRS.ofList []) (untapping thisCreature) = [] ∧
      ((stepAction 64 .postEvent .obj (DRS.ofList []) (untapping thisCreature)).1.referents.filter
        (·.kind == .object)).map (·.id) = [0] := by
  decide

/-- "You gain 1 life. You lose 1 life.": the two "you"s are one referent (`mentionYou`). -/
theorem twoYousAreOneReferent :
    ((stepAction 64 .postEvent .obj (DRS.ofList [])
        (.sequentially [instruct .you (gainLife 1), instruct .you (loseLife 1)])).1.referents.filter
      (·.kind == .player)).map (fun ref => (ref.id, ref.det)) = [(0, .self)] := by
  decide

/-- *Terror of the Peaks*: "Whenever another creature you control enters, this creature deals
damage equal to that creature's power to any target." In the handoff's own clause the Source
stays the self-referent no noun word reads (owner ruling 2026-10-24), so "that creature" reads
the creature that entered: with no other creature in the header it finds none. -/
theorem terrorOfThePeaksThatCreatureIsTheEnteringCreature :
    Ability.check (DRS.ofList [])
        (whenever (does (a (and [isCreature, isOther, youControl])) enter)
          (Action.instruct thisCreature
            (.dealDamage (powerOf (that (.type .creature))) (target isAnyTarget) none))) = [] ∧
      Ability.check (DRS.ofList [])
        (whenever (does thisCreature enter)
          (Action.instruct thisCreature
            (.dealDamage (powerOf (that (.type .creature))) (target isAnyTarget) none))) =
        [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- *Rampaging Ferocidon*: "Whenever another creature enters, this creature deals 1 damage to
that creature's controller." As in *Terror of the Peaks*, "that creature" in the handoff's own
clause reads the entering creature, never the Source. -/
theorem rampagingFerocidonThatCreatureIsTheEnteringCreature :
    Ability.check (DRS.ofList [])
        (whenever (does (a (and [isCreature, isOther])) enter)
          (Action.instruct thisCreature
            (.dealDamage 1 (controllerOf (that (.type .creature))) none))) = [] ∧
      Ability.check (DRS.ofList [])
        (whenever (does thisCreature enter)
          (Action.instruct thisCreature
            (.dealDamage 1 (controllerOf (that (.type .creature))) none))) =
        [.anaphor (.word (.type .creature)) .one 0] := by
  decide

end Semantics.Proofs.Actor
