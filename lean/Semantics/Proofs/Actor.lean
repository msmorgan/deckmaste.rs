import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Actor

The performer of an instruction and the handoff that names another one. `theActor` is whoever
performs the instruction it is part of: by default, the player addressed by the text
[CR#109.5], unless an enclosing `instruct` hands it to another player or to a permanent. Each pin is closed by `decide`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Actor

private def instructionResult (drs : DRS) (i : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs i).1
private def nounResult (drs : DRS) (n : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .obj true n).1


private def handoff (drs : DRS) (subject : NounPhrase) : DRS :=
  let (after, ids, _) := stepNounPhrase 64 drs .subj true subject
  let actorShape : ActorShape := {
    kind := subject.kindIn after .player
    isYou := subject.isYou
    selfDefined := false
    ascribable := false
    twoParties := false
    opponentOnly := subject.opponentOnly
    refIds := ids }
  after.push { det := .the, plur := .one, payload := .actor actorShape } .other false


private def performerIsYou (drs : DRS) : Bool :=
  ((actorShape? drs).map (·.isYou)).getD true

private def products (drs : DRS) : List Payload :=
  (drs.referents.filter (fun r => !r.isActorFrame && r.kind != .player)).map (·.payload)

/-- Kind, number and determiner of each binding, newest first. -/
private def shape (bs : DRS) : List (Kind × Plurality × Determiner) :=
  (bs.referents.filter (! ·.isActorFrame)).map fun b => (b.kind, b.plur, b.det)

/-- One player already named, for "they". -/
private def onePlayer : DRS := (DRS.ofList [{ det := .the, plur := .one, payload := .player false }])

/-! ## Cards -/

/-- Thoughtseize: "Target player reveals their hand. You choose a nonland card from it. That
player discards that card. You lose 2 life." The handoff leaves the target player published, so
"their hand" and the second handoff to "that player" read it. "A nonland card from it" says
`isCard` as the RON card's `cardIn` does: a token can sit in a hand until state-based actions
are checked [CR#111.7,704.5d]. -/
theorem okThoughtseize :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (target .anyPlayer) Actor.revealHand,
          choose (a (.and [.not isLand, .and [.isCard, .in_ (handOf they)]])),
          instruct they (Actor.discard (that .card)),
          loseLife (.lit 2) ]) = [] := by
  decide

/-- Burglar Rat: "When Burglar Rat enters, each opponent discards a card." The discard names no
zone: the deed's patient lives in a hand [CR#701.9a]. -/
theorem okBurglarRat : Action.check (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (Actor.discard (a .isCard))) = [] := by
  decide

/-- "Each opponent discards a card. Exile them.": what each opponent's body introduced is
published as a group, as `doForEach` publishes it. -/
theorem okDistributedDiscardReadsAsGroup :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (each (.isOpponentOf .you)) (Actor.discard (a .isCard)), exile them]) = [] := by
  decide

/-- "Each opponent discards a card. Exile it.": one card per opponent, so the singular read has
no antecedent (`okDistributedDiscardReadsAsGroup` is the plural read; compare
`Choice.badDistributedChoiceReadSingular`). -/
theorem badDistributedDiscardReadSingular :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (each (.isOpponentOf .you)) (Actor.discard (a .isCard)), exile it])
      = [.anaphor .bare .one 0] := by
  decide

/-- Hymn to Tourach: "Target player discards two cards at random." [CR#701.9b] -/
theorem okHymnToTourach :
    Action.check (DRS.ofList [])
      (instruct (target .anyPlayer) (Actor.discard (countedAtRandom (exactly 2) .isCard))) = [] := by
  decide

/-- "Amass Orcs 2" [CR#701.47a]: with no handoff the controller amasses. -/
theorem okAmassBare : Action.check (DRS.ofList []) (Actor.amass "Orc" 2) = [] := by decide

/-- Azog: "Destroy target creature. Its controller amasses Goblins 2." The performer is the
destroyed creature's controller; the reminder's "that creature" and "it" are the chosen Army. -/
theorem okAzogControllerAmasses :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature), instruct (controllerOf it) (Actor.amass "Goblin" 2)])
      = [] := by
  decide

/-- D′ selects the newly chosen Army after an earlier creature is destroyed. -/
theorem amassBareItAfterDestroyChecks :
    Action.check (DRS.ofList []) (.sequentially [destroy (target isCreature), amass "Goblin" 2])
      = [] := by
  decide

/-- The brief's amass body as the RON declaration expands it: bare "that creature" and "it"
(`Actor.amass` reads through `itPrior`). -/
private def literalAmass : Action :=
  .sequentially
    [ .doIf (.not (exists_ Actor.army))
        (create (.lit 1) (creatureToken 0 0 [.black] [creatureType "Orc", creatureType "Army"]))
        none,
      choose (a Actor.army),
      .putCounters (.lit 2) (.printed p1p1Counter) (that (.type .creature)) .one true,
      .doIf (.not (.matches it (.hasSubtype (creatureType "Orc"))))
        (.establish (Primitives.StaticSpec.qualityChange it .adds
          (.bundle { characteristics := { subtypes := [creatureType "Orc"] } } none)) none)
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
§7, ruling 2026-10-06). The `plugins_v2` loader writes the scope inside the action's `enact`
wrapper (`Action.ownScope`) and each parameter `inCaller`, read in the caller's view. -/

/-- The literal amass body inside the loader's wrapper. -/
private def scopedAmass : Action := .enact (.action "amass") (.ownScope none literalAmass) none none

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
  .enact (.action "amass") (.ownScope none (.sequentially
    [ .doIf (.not (exists_ Actor.army))
        (create (.lit 1) (creatureToken 0 0 [.black] [creatureType "Goblin", creatureType "Army"]))
        none,
      choose (a Actor.army),
      .putCounters (.inCaller amount) (.printed p1p1Counter) (that (.type .creature)) .one true,
      .doIf (.not (.matches it (.hasSubtype (creatureType "Goblin"))))
        (.establish (Primitives.StaticSpec.qualityChange it .adds
          (.bundle { characteristics := { subtypes := [creatureType "Goblin"] } } none)) none)
        none ])) none none

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
    ((Action.check caller (.enact (.action "destroy") (.ownScope none (destroy it)) none none)).contains
        (.anaphor .bare .one 0),
     Action.check caller (.enact (.action "destroy") (destroy it) none none)) = (true, []) := by
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
    Action.check (DRS.ofList []) (draw (.lit 1)) = Action.check (DRS.ofList []) (instruct .you (draw (.lit 1))) ∧
      (products (instructionResult (DRS.ofList []) (draw (.lit 1))) == products (instructionResult (DRS.ofList []) (instruct .you (draw (.lit 1)))))
        = true := by
  decide

/-- "Lose 2 life.": as handed to "you". -/
theorem actorLoseLifeIsYouLoseLife :
    Action.check (DRS.ofList []) (loseLife (.lit 2)) = Action.check (DRS.ofList []) (instruct .you (loseLife (.lit 2))) ∧
      (products (instructionResult (DRS.ofList []) (loseLife (.lit 2))) ==
        products (instructionResult (DRS.ofList []) (instruct .you (loseLife (.lit 2))))) = true := by
  decide

/-- "You may pay 1 life. If you do, draw a card." (`Choice.okMatchedPayer` with the actor) -/
theorem okActorMatchedPayer :
    Action.check (DRS.ofList [])
      (Primitives.Action.offer (.pay (payLife 1) .once .paid none none) none
        (some (draw (.lit 1)))) = [] := by
  decide

/-- "Pay 2 life: Draw a card." (`Keyword.okOwnPayerCost` with the actor) -/
theorem okActorOwnPayerCost :
    Ability.check (DRS.ofList []) (.activated (payLife 2) (draw (.lit 1)) none none none none) = [] := by
  decide

/-- "Target opponent loses 2 life: Draw a card.": a cost may hand its instruction to another
player, and a life payment inside the handoff is that player's, not the controller's. Who pays a
cost is not checked (`lean-checker-sweep-costs`); the cost's performer is still the handed
player, read in the context the cost's handoff builds. -/
theorem badHandedOffLifeCost :
    Ability.check (DRS.ofList [])
      (.activated (.perform (instruct (target (.isOpponentOf .you)) (loseLife (.lit 2)))) (draw (.lit 1))
        none none none none)
      = [] ∧
      performerIsYou (handoff (DRS.ofList []) (target (.isOpponentOf .you))) = false := by
  decide

/-! ## Costs: each handoff twin judged as its explicit-agent form

Every cost instruction on the bench or in the pins whose agent is not "you", written once with
the explicit agent and once handed off; both gave the same refusals. The explicit-agent spelling
is gone (`semantics-v2-drop-agent-fields`): each pin's second half now names the handoff the bench
writes for the same card, so it states that the bench's own spelling is the one judged. -/

private def drawOne : Action := draw (.lit 1)

/-- Wall of Shards: "Cumulative upkeep—An opponent gains 1 life." -/
theorem wallOfShardsHandoffTwin :
    Ability.check (DRS.ofList []) (cumulativeUpkeep (.perform (instruct anOpponent (gainLife (.lit 1))))) = [] ∧
      Ability.check (DRS.ofList []) (cumulativeUpkeep (.perform (instruct anOpponent (gainLife (.lit 1))))) =
        Ability.check (DRS.ofList []) (cumulativeUpkeep (.perform (instruct anOpponent (gainLife (.lit 1))))) := by
  decide

/-- Varchild's War-Riders: "Cumulative upkeep—Have an opponent create a 1/1 red Survivor creature
token." -/
theorem varchildsWarRidersHandoffTwin :
    Ability.check (DRS.ofList [])
        (cumulativeUpkeep (.perform (instruct anOpponent
          (create (.lit 1) (creatureToken 1 1 [.red] [creatureType "Survivor"]))))) = [] ∧
      Ability.check (DRS.ofList [])
          (cumulativeUpkeep (.perform (instruct anOpponent
            (create (.lit 1) (creatureToken 1 1 [.red] [creatureType "Survivor"]))))) =
        Ability.check (DRS.ofList [])
          (cumulativeUpkeep (.perform (instruct anOpponent (Action.create (.lit 1)
            (.written (creatureToken 1 1 [.red] [creatureType "Survivor"])) [])))) := by
  decide

private def controlsForest : Condition :=
  exists_ (.and [isLand, .hasSubtype (landType "Forest"), .hasPossessor .controller .you])

/-- Invigorate: "If you control a Forest, rather than pay this spell's mana cost, you may have an
opponent gain 3 life." -/
theorem invigorateHandoffTwin :
    Ability.check (DRS.ofList [])
        (.static (onlyWhile (.altCost .this (some (.perform (instruct anOpponent (gainLife (.lit 3))))))
          controlsForest)) = [] ∧
      Ability.check (DRS.ofList [])
          (.static (onlyWhile (.altCost .this (some (.perform (instruct anOpponent (gainLife (.lit 3))))))
            controlsForest)) =
        Ability.check (DRS.ofList [])
          (.static (onlyWhile (.altCost .this (some (.perform (instruct anOpponent (gainLife (.lit 3))))))
            controlsForest)) := by
  decide

private def blockersTheyControl : Amount :=
  times (.lit 1) (countOf (.and [isCreature, isBlocking, .hasPossessor .controller they]))

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
    Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (loseLife (.lit 2)))) drawOne
        none none none none) = [] ∧
      Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (loseLife (.lit 2)))) drawOne
          none none none none) =
        Ability.check (DRS.ofList []) (.activated (.perform (instruct anOpponent (loseLife (.lit 2)))) drawOne none none none
          none) := by
  decide

/-- "Cumulative upkeep—An opponent loses 1 life." (the pin
`Mana.badOpponentPaysYourCost`, retired with `keywordCostPaidByYou` by `lean-checker-sweep-costs`) -/
theorem opponentPaysYourCostHandoffTwin :
    Ability.check (DRS.ofList []) (keywordCosting (.named "cumulativeUpkeep") (.perform (instruct anOpponent (loseLife (.lit 1)))))
        = [] ∧
      Ability.check (DRS.ofList []) (keywordCosting (.named "cumulativeUpkeep") (.perform (instruct anOpponent (loseLife (.lit 1))))) =
        Ability.check (DRS.ofList []) (keywordCosting (.named "cumulativeUpkeep") (.perform (instruct anOpponent (loseLife (.lit 1))))) := by
  decide

/-- "You may pay an opponent's 1 life. If you do, draw a card." (the pin
`Choice.badMismatchedPayer`, retired with `payAgrees` by `lean-checker-sweep-costs`) -/
theorem mismatchedPayerHandoffTwin :
    Action.check (DRS.ofList [])
        (Primitives.Action.offer (.pay (.perform (instruct anOpponent (loseLife (.lit 1)))) .once .paid none none)
          none (some drawOne)) = [] ∧
      Action.check (DRS.ofList [])
          (Primitives.Action.offer (.pay (.perform (instruct anOpponent (loseLife (.lit 1)))) .once .paid none none)
            none (some drawOne)) =
        Action.check (DRS.ofList [])
          (Primitives.Action.offer (.pay (.perform (instruct anOpponent (loseLife (.lit 1)))) .once .paid none none)
            none (some drawOne)) := by
  decide

/-- "Pay 2 life: Draw a card." handed to "you" (`Keyword.okOwnPayerCost`). -/
theorem ownPayerCostHandoffTwin :
    Ability.check (DRS.ofList []) (.activated (.perform (instruct .you (loseLife (.lit 2)))) drawOne
        none none none none) = [] ∧
      Ability.check (DRS.ofList []) (.activated (.perform (instruct .you (loseLife (.lit 2)))) drawOne
          none none none none) =
        Ability.check (DRS.ofList []) (.activated (payLife 2) drawOne none none none none) := by
  decide

/-! ## A captured `theActor` keeps its handoff -/

private semantic_template capturedLifeLoss (who : NounPhrase) : Action :=
  .instruct who (.changeLife (.down (.lit 1)))

/-- "You may pay 1 life of <who>. If you do, draw a card.", the payer being "you". -/
private def youPayWith (c : Cost) : Action :=
  instruct .you (Primitives.Action.offer (.pay c .once .paid none none) none (some (draw (.lit 1))))

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
      Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (youPayWith (.perform (instruct they (loseLife (.lit 1))))))
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
      (.activated (.perform (instruct (target (.isOpponentOf .you)) (capturedLifeLoss theActor))) (draw (.lit 1))
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
        (Primitives.Action.offer (.pay (.mana [generic 2]) .once .paid none none) none
          (some (instruct .you (draw (.lit 1)))))) = [] := by
  decide

/-- The same sentence through `doUnless`, handed to each opponent, who decides. -/
theorem okEachOpponentMayPayElseYouDrawUnless :
    Action.check (DRS.ofList [])
      (instruct (each (.isOpponentOf .you)) (doUnless (instruct .you (draw (.lit 1))) (.mana [generic 2]))) = [] := by
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

/-- "You choose a creature they control.": `instruct you` publishes nothing, so "they" has no
antecedent (`Choice.badUnchooseredTheyControl`). -/
theorem handedToYouPossessorReadsExplicitPlayer :
    Action.check (DRS.ofList [])
      (instruct .you (choose (a (.and [isCreature, .hasPossessor .controller they]))))
      = [] := by
  decide

/-- "Target opponent may pay an opponent's 1 life. If they do, they draw a card.": inside the
handoff the payer is the opponent. Who pays a cost is not checked (`lean-checker-sweep-costs`). -/
theorem okHandedPayerIsTheTarget :
    Action.check (DRS.ofList [])
      (instruct (target (.isOpponentOf .you))
        (Primitives.Action.offer (.pay (.perform (instruct anOpponent (loseLife (.lit 1)))) .once .paid none none) none
          (some (draw (.lit 1))))) = [] := by
  decide

/-- Nested handoffs: the innermost one names the performer, so `instruct you` inside an opponent's
handoff makes the actor the controller again. Who pays a cost is not checked
(`lean-checker-sweep-costs`); the innermost handoff's actor is "you". -/
theorem badNestedHandoffInnermost :
    Action.check (DRS.ofList [])
      (instruct (target (.isOpponentOf .you))
        (instruct .you
          (Primitives.Action.offer (.pay (.perform (instruct anOpponent (loseLife (.lit 1)))) .once .paid none none) none
            (some (draw (.lit 1))))))
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
          (.abilityGrant (target isCreature)
            (.activated .tapSymbol
              (Primitives.Action.offer (.pay (.perform (instruct anOpponent (loseLife (.lit 1)))) .once .paid none none) none
                (some (draw (.lit 1))))
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

/-- "That player discards that card." as `enact (.action "discard") …` under the handoff, the
deed performed by the actor in context [CR#701.9a]. -/
theorem okDiscardWrapperCarriesActor :
    Action.check onePlayer
      (instruct they
        (.enact (.action "discard") (.move (a .isCard) ((.zone .hand (some theActor))) (some graveyard) .arrival none []) none none))
      = [] := by
  decide

/-- The discard deed admits a player performer at all: "Target player discards a card." handed to
that player. -/
theorem okDiscardWrapperWithAgent :
    Action.check (DRS.ofList [])
      (instruct (target .anyPlayer) (.enact (.action "discard") (.move (a .isCard) ((.zone .hand none)) (some graveyard) .arrival none []) none none))
      = [] := by
  decide

/-! ## What a handoff leaves bound -/

/-- A targeted performer stays published after its handoff, below what the body introduced. -/
theorem handedTargetStaysBound :
    shape (instructionResult (DRS.ofList []) (instruct (target .anyPlayer) (Actor.discard (a .isCard)))) =
      [(.object, .one, .a), (.player, .one, .target)] := by
  decide

/-- "That player": a pronoun performer introduces nothing; the body's mentions sit above the
player it read. -/
theorem handedPronounAddsOnlyTheBody :
    shape (instructionResult onePlayer (instruct they (Actor.discard (a .isCard)))) =
      [(.object, .one, .a), (.player, .one, .the)] := by
  decide

/-- "Each opponent": the members' mentions and the group, pluralized. -/
theorem handedGroupPublishesPlurals :
    shape (instructionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (Actor.discard (a .isCard)))) =
      [(.object, .many, .a), (.player, .many, .each)] := by
  decide

/-- `instruct you` publishes exactly what its body does with no handoff. -/
theorem handedToYouIsTransparent :
    (products (instructionResult (DRS.ofList []) (instruct .you (Actor.discard (a .isCard)))) ==
      products (instructionResult (DRS.ofList []) (Actor.discard (a .isCard)))) = true := by
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
        subtypes := [creatureType "Human", creatureType "Cleric"],
        text :=
          [ .keyword (.named "extort") []
              [ .triggered
                  (.does (.actor .you) (.procedure .cast (.described (.a .unmarked)
                      (.and [.in_ (.zone .stack none), .not (.abilityHead .anyOnStack)])) .itsOwnCost none none none))
                  [] none [] none none none
                  (.withContinuation .optional
                    (.pay (.mana [hybridPip .white .black]) .once .paid none none)
                    (some (.sequentially [loss, gainLife .thatMuch])) none) ] ],
        power := some (.lit 2), toughness := some (.lit 2) } }

/-- Extort handed to each opponent: "that much" reads the one total. -/
theorem okExtortHandedToEachOpponent :
    Card.check (syndicOfTithes (instruct (each (.isOpponentOf .you)) (loseLife (.lit 1)))) = [] := by
  decide

/-- The handoff twin of the explicit plural agent `changeLife (down 1) (each opponent)`, which is
gone (`semantics-v2-drop-agent-fields`); its replacement, the handoff of the bare life change, is
judged the same as the helper's spelling. -/
theorem extortHandoffTwin :
    Card.check (syndicOfTithes (instruct (each (.isOpponentOf .you)) (.changeLife (.down (.lit 1))))) = [] ∧
      Card.check (syndicOfTithes (instruct (each (.isOpponentOf .you)) (loseLife (.lit 1)))) =
        Card.check (syndicOfTithes (instruct (each (.isOpponentOf .you)) (.changeLife (.down (.lit 1))))) := by
  decide

/-- "Each opponent loses 1 life": the group and one `lifeLost` total, exactly what the explicit
plural agent published (its replacement is the handoff of the bare life change). -/
theorem handedGroupPublishesTotal :
    shape (instructionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (loseLife (.lit 1)))) =
        [(.outcome, .one, .the), (.player, .many, .each)] ∧
      (instructionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (loseLife (.lit 1))) ==
        instructionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (.changeLife (.down (.lit 1))))) = true := by
  decide

/-- "Each opponent discards a card. You gain that much life.": the body has no amount outcome,
so the group handoff publishes none and "that much" has nothing to read. -/
theorem badGroupHandoffNoOutcomeThatMuch :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (each (.isOpponentOf .you)) (Actor.discard (a .isCard)), gainLife .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Loses 1 life and discards a card." -/
private def lossAndDiscard : Action :=
  .sequentially [loseLife (.lit 1), Actor.discard (a .isCard)]

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
        [ instruct (each (.isOpponentOf .you)) (.sequentially [loseLife (.lit 1), loseLife (.lit 2)]),
          gainLife .thatMuch ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "For each opponent, that player loses 1 life. You gain that much life.": `doForEach` still
publishes its body's outcomes as a group; only the handoff carries the total. -/
theorem badForEachLossThatMuch :
    Action.check (DRS.ofList [])
      (.sequentially
        [ Primitives.Action.doForEach (each (.isOpponentOf .you)) (instruct they (loseLife (.lit 1))),
          gainLife .thatMuch ]) = [.quantOutcomeInScope 0] := by
  decide

/-! ## A permanent can be handed an instruction

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
    [ .expose .reveal (.cards (.slice .top (.lit 1) (libraryOf theActor))),
      .doIf (.matches (that .card) isLand) (.move (that .card) .anyLocation (some (.zone .hand none)) .arrival none [])
        (some (.sequentially
          [ .putCounters (.lit 1) (.printed p1p1Counter) perm .one true,
            .withContinuation .optional
              (.move (that .card) .anyLocation (some (.zone .graveyard none)) .arrival none []) none none ])) ])

/-- `explore` as the keyword-action loader wraps it: the deed, performed by the actor. -/
private def literalExplore (perm : NounPhrase := that .permanent) : Action :=
  .enact (.action "explore") (exploreBody perm) none none

/-- `endure N` as declared [CR#701.63a]: the enduring permanent's controller chooses between
the counters on "that permanent" and the Spirit token. -/
private def literalEndure (n : Nat) : Action :=
  .enact (.action "endure")
    (instruct (controllerOf theActor)
      (.chooseModes (.range (some 1) (some 1))
        [ (none, none, .putCounters (.lit n) (.printed p1p1Counter) (that .permanent) .one true),
          (none, none, create (.lit 1) (creatureToken n n [.white] [creatureType "Spirit"])) ])) none none

/-- `adapt N` as declared [CR#701.46a]: unchanged, still reading "this permanent". -/
private def literalAdapt (n : Nat) : Action :=
  .enact (.action "adapt")
    (.doIf (.not (.matches thisPermanent (.hasCounters (some p1p1Counter))))
      (.putCounters (.lit n) (.printed p1p1Counter) thisPermanent .one true) none) none none

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
    Action.check (DRS.ofList []) (instruct (target isCreature) (draw (.lit 1)))
        = [.kindMismatch .player .object] ∧
      Action.check (DRS.ofList []) (instruct (target isCreature) (instruct (controllerOf theActor) (draw (.lit 1))))
        = [] := by
  decide

/-- Explore handed to a player: the recorded performer is a player where the deed takes a
permanent, and "that permanent" has no antecedent. -/
theorem badExploreHandedToOpponent :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) literalExplore)
      = [.kindMismatch .object .player, .possessorKind .controller .player,
         .anaphor (.word .permanent) .one 0] := by
  decide

/-- A graveyard card is not a permanent [CR#110.1], so this permanent-controller handoff is refused. -/
theorem badHandoffToCardInGraveyard :
    Action.check (DRS.ofList []) (instruct (target (.and [.isCard, .in_ graveyard]))
        (instruct (controllerOf theActor) (draw (.lit 1))))
      = [.handoffPerformer] := by
  decide

/-- Printed "Adapt 2" on a permanent's own ability, with no handoff: the recorded actor is the
controller, and the deed is performed by the source permanent [CR#701.46a]. -/
theorem okBareAdaptDefaultsToThis : Action.check (DRS.ofList []) (literalAdapt 2) = [] := by decide

/-- A bare "explore" and "endure 1" default to the source permanent the same way: `theActor` in the
body is "this permanent", and "that permanent" reads it. -/
theorem okBareExploreAndEndureDefaultToThis :
    Action.check (DRS.ofList []) literalExplore = [] ∧ Action.check (DRS.ofList []) (literalEndure 1) = [] := by
  decide

/-- "Target opponent adapts 2": handed to another player, the actor is that player, not the
controller, so there is no default to the source permanent and the performer is refused. -/
theorem badAdaptHandedToOpponent :
    Action.check (DRS.ofList []) (instruct (target (.isOpponentOf .you)) (literalAdapt 2))
      = [.kindMismatch .object .player] := by
  decide

/-- "Whenever a creature explores": the event's performer is a permanent, as the deed's row
says; "whenever a player explores" is refused, while a player still bolsters. -/
theorem exploreEventTakesAPermanent :
    GameEvent.check (DRS.ofList []) (.does (.actor (a isCreature)) explore) = [] ∧
      GameEvent.check (DRS.ofList []) (.does (.actor (a .anyPlayer)) explore)
        = [.kindMismatch .object .player, .possessorKind .controller .player,
            .counterSourceScope] ∧
      GameEvent.check (DRS.ofList []) (.does (.actor (a .anyPlayer)) (bolster (.lit 1)))
        = [] := by
  decide

/-- The source and its controller remain accessible after the handoff, along with the drawn card. -/
theorem handedToThisPublishesSourceAndController :
    shape (instructionResult (DRS.ofList []) (instruct thisCreature (instruct (controllerOf theActor) (draw (.lit 1)))))
      = [(.object, .one, .the), (.player, .one, .the), (.object, .one, .the)] := by
  decide

theorem amassBareItSelectsChosenArmy :
    let stepsBeforeRead := match literalAmass with
      | .sequentially steps => Action.sequentially (steps.take 2)
      | body => body
    let after := instructionResult (DRS.ofList [])
      (.sequentially [destroy (target isCreature), stepsBeforeRead])
    let selected := resolveCandidates .bare .one (nextClause after)
    selected.map (·.id) = [after.nextId - 1] ∧
      selected.map (·.ty) = [[.creature]] := by decide

end Semantics.Proofs.Actor
