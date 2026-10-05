import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Actor

The performer of an instruction and the handoff that names another one. `actor` is whoever
performs the instruction it is part of: the controller [CR#109.5] unless an enclosing `act`
hands the instruction to another player. Each pin is closed by `decide`.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Actor

/-- Kind, number and determiner of each binding, newest first. -/
private def shape (bs : Bindings) : List (Kind × Plurality × Determiner) :=
  bs.map fun b => (b.kind, b.plur, b.det)

/-- One player already named, for "they". -/
private def onePlayer : Bindings := [⟨.the, .one, .player false⟩]

/-! ## Cards -/

/-- Thoughtseize: "Target player reveals their hand. You choose a nonland card from it. That
player discards that card. You lose 2 life." The handoff leaves the target player published, so
"their hand" and the second handoff to "that player" read it. -/
theorem okThoughtseize :
    Instruction.check []
      (.sequentially
        [ act (target .anyPlayer) Actor.revealHand,
          choose (a (.and [.not land, .inZone (handOf they)])),
          act they (Actor.discard (that .card)),
          Actor.loseLife (.lit 2) ]) = [] := by
  decide

/-- Burglar Rat: "When Burglar Rat enters, each opponent discards a card." The discard names no
zone: the deed's patient lives in a hand [CR#701.9a]. -/
theorem okBurglarRat : Instruction.check [] (act (each .opponent) (Actor.discard (a .isCard))) = [] := by
  decide

/-- "Each opponent discards a card. Exile them.": what each opponent's body introduced is
published as a group, as `doForEach` publishes it. -/
theorem okDistributedDiscardReadsAsGroup :
    Instruction.check []
      (.sequentially [act (each .opponent) (Actor.discard (a .isCard)), exile them]) = [] := by
  decide

/-- "Each opponent discards a card. Exile it.": one card per opponent, so the singular read has
no antecedent (`okDistributedDiscardReadsAsGroup` is the plural read; compare
`Choice.badDistributedChoiceReadSingular`). -/
theorem badDistributedDiscardReadSingular :
    Instruction.check []
      (.sequentially [act (each .opponent) (Actor.discard (a .isCard)), exile it])
      = [.anaphor .bare .one 0] := by
  decide

/-- Hymn to Tourach: "Target player discards two cards at random." [CR#701.9b] -/
theorem okHymnToTourach :
    Instruction.check []
      (act (target .anyPlayer) (Actor.discard (countedAtRandom (exactly 2) .isCard))) = [] := by
  decide

/-- "Amass Orcs 2" [CR#701.47a]: with no handoff the controller amasses. -/
theorem okAmassBare : Instruction.check [] (Actor.amass "Orc" 2) = [] := by decide

/-- Azog: "Destroy target creature. Its controller amasses Goblins 2." The performer is the
destroyed creature's controller; the reminder's "that creature" and "it" are the chosen Army. -/
theorem okAzogControllerAmasses :
    Instruction.check []
      (.sequentially [destroy (target creature), act (controllerOf it) (Actor.amass "Goblin" 2)])
      = [] := by
  decide

/-- The same text through the explicit-agent `amass`, whose reminder reads a bare "it": the
destroyed card is a second antecedent. The refusal is anaphoric, not the handoff's
(`okAzogControllerAmasses` reads through the choice's own introduction). -/
theorem badAmassBareItAfterDestroy :
    Instruction.check [] (.sequentially [destroy (target creature), amass "Goblin" 2])
      = [.anaphor .bare .one 2, .anaphor .bare .one 2] := by
  decide

/-- The brief's amass body as the RON declaration expands it: bare "that creature" and "it"
(`Actor.amass` reads through `itPrior`). -/
private def literalAmass : Instruction :=
  .sequentially
    [ .doIf (.not (exists_ Actor.army))
        (Actor.create (.lit 1) (creatureToken 0 0 [.black] [creatureType "Orc", creatureType "Army"]))
        none,
      Actor.choose (a Actor.army),
      .putCounters (.lit 2) (.printed plusOnePlusOne) (that (.type .creature)),
      .doIf (itIsntA (.hasSubtype (creatureType "Orc")))
        (.establish (Primitives.StaticSpec.qualityChange it .adds
          (.bundle { characteristics := { subtypes := [creatureType "Orc"] } } none)) none)
        none ]

/-- "Amass Orcs 2" in the literal spelling checks on its own. -/
theorem okLiteralAmassBare : Instruction.check [] literalAmass = [] := by decide

/-- "Target opponent amasses Orcs 2" in the literal spelling: the opponent is a player, so no
second object is in view, and it checks. -/
theorem okLiteralAmassHandedOff : Instruction.check [] (act (target .opponent) literalAmass) = [] := by
  decide

/-! ## The actor outside a handoff is the controller -/

/-- "Draw a card.": the same refusals and the same published bindings as `agent := you`. -/
theorem actorDrawIsYouDraw :
    Instruction.check [] (Actor.draw (.lit 1)) = Instruction.check [] (draw (.lit 1) (agent := .you)) ∧
      (Instruction.intro [] (Actor.draw (.lit 1)) == Instruction.intro [] (draw (.lit 1) (agent := .you)))
        = true := by
  decide

/-- "Lose 2 life.": as `agent := you`. -/
theorem actorLoseLifeIsYouLoseLife :
    Instruction.check [] (Actor.loseLife (.lit 2)) = Instruction.check [] (loseLife (.lit 2) (agent := .you)) ∧
      (Instruction.intro [] (Actor.loseLife (.lit 2)) ==
        Instruction.intro [] (loseLife (.lit 2) (agent := .you))) = true := by
  decide

/-- "You may pay 1 life. If you do, draw a card." (`Choice.okMatchedPayer` with the actor) -/
theorem okActorMatchedPayer :
    Instruction.check []
      (Primitives.Instruction.offer (.pay (payLife actor 1) .once (agent := actor)) none
        (some (Actor.draw (.lit 1))) (agent := actor)) = [] := by
  decide

/-- "You pay an opponent's 1 life" (`Choice.badMismatchedPayer` with the actor): the payer is the
controller, so the payment must be theirs. -/
theorem badActorMismatchedPayer :
    Instruction.check []
      (Primitives.Instruction.offer (.pay (payLife anOpponent 1) .once (agent := actor)) none
        (some (Actor.draw (.lit 1))) (agent := actor))
      = [.payAgrees] := by
  decide

/-- "Pay 2 life: Draw a card." (`Keyword.okOwnPayerCost` with the actor) -/
theorem okActorOwnPayerCost :
    Ability.check [] (.activated (payLife actor 2) (Actor.draw (.lit 1)) none none none none) = [] := by
  decide

/-- "Target opponent loses 2 life: Draw a card.": a cost may hand its instruction to another
player, and a life payment inside the handoff is that player's, not the controller's. -/
theorem badHandedOffLifeCost :
    Ability.check []
      (.activated (.perform (act (target .opponent) (Actor.loseLife (.lit 2)))) (Actor.draw (.lit 1))
        none none none none)
      = [.costPaidByYou] := by
  decide

/-! ## Costs: each handoff twin judged as its explicit-agent form

Every cost instruction on the bench or in the pins whose agent is not "you", written once with
the explicit agent and once handed off; both give the same refusals. -/

private def drawOne : Instruction := draw (.lit 1) (agent := .you)

/-- Wall of Shards: "Cumulative upkeep—An opponent gains 1 life." -/
theorem wallOfShardsHandoffTwin :
    Ability.check [] (cumulativeUpkeep (.perform (act anOpponent (Actor.gainLife (.lit 1))))) = [] ∧
      Ability.check [] (cumulativeUpkeep (.perform (act anOpponent (Actor.gainLife (.lit 1))))) =
        Ability.check [] (cumulativeUpkeep (.perform (gainLife (.lit 1) (agent := anOpponent)))) := by
  decide

/-- Varchild's War-Riders: "Cumulative upkeep—Have an opponent create a 1/1 red Survivor creature
token." -/
theorem varchildsWarRidersHandoffTwin :
    Ability.check []
        (cumulativeUpkeep (.perform (act anOpponent
          (Actor.create (.lit 1) (creatureToken 1 1 [.red] [creatureType "Survivor"]))))) = [] ∧
      Ability.check []
          (cumulativeUpkeep (.perform (act anOpponent
            (Actor.create (.lit 1) (creatureToken 1 1 [.red] [creatureType "Survivor"]))))) =
        Ability.check []
          (cumulativeUpkeep (.perform (Primitives.Instruction.create (.lit 1)
            (.written (creatureToken 1 1 [.red] [creatureType "Survivor"])) [] (agent := anOpponent)))) := by
  decide

private def controlsForest : Condition :=
  exists_ (.and [land, .hasSubtype (landType "Forest"), .hasPossessor .controller .you])

/-- Invigorate: "If you control a Forest, rather than pay this spell's mana cost, you may have an
opponent gain 3 life." -/
theorem invigorateHandoffTwin :
    Ability.check []
        (.static (onlyWhile (.altCost .this (some (.perform (act anOpponent (Actor.gainLife (.lit 3))))))
          controlsForest)) = [] ∧
      Ability.check []
          (.static (onlyWhile (.altCost .this (some (.perform (act anOpponent (Actor.gainLife (.lit 3))))))
            controlsForest)) =
        Ability.check []
          (.static (onlyWhile (.altCost .this (some (.perform (gainLife (.lit 3) (agent := anOpponent)))))
            controlsForest)) := by
  decide

private def blockersTheyControl : Amount :=
  times (.lit 1) (countOf (.and [creature, blocking, .hasPossessor .controller they]))

/-- Heat Wave: "Nonblue creatures can't block creatures you control unless their controller pays 1
life for each blocking creature they control." -/
theorem heatWaveHandoffTwin :
    Ability.check []
        (.static (deontic (allOf (.and [creature, .not (.colorIs .blue)]))
          (.gatedBy (.perform (act they (Actor.loseLife blockersTheyControl)))) [.core .block] .agent
          (.counterpart (allOf creatureYouControl)))) = [] ∧
      Ability.check []
          (.static (deontic (allOf (.and [creature, .not (.colorIs .blue)]))
            (.gatedBy (.perform (act they (Actor.loseLife blockersTheyControl)))) [.core .block] .agent
            (.counterpart (allOf creatureYouControl)))) =
        Ability.check []
          (.static (deontic (allOf (.and [creature, .not (.colorIs .blue)]))
            (.gatedBy (.perform (loseLife blockersTheyControl (agent := they)))) [.core .block] .agent
            (.counterpart (allOf creatureYouControl)))) := by
  decide

/-- Killing Wave: "For each creature, its controller sacrifices it unless they pay X life." -/
theorem killingWaveHandoffTwin :
    Instruction.check []
        (Primitives.Instruction.doForEach (each creature)
          (doUnless (sacrifice it (agent := they)) (.perform (act they (Actor.loseLife (.letter .x))))
            (agent := controllerOf it))) = [] ∧
      Instruction.check []
          (Primitives.Instruction.doForEach (each creature)
            (doUnless (sacrifice it (agent := they)) (.perform (act they (Actor.loseLife (.letter .x))))
              (agent := controllerOf it))) =
        Instruction.check []
          (Primitives.Instruction.doForEach (each creature)
            (doUnless (sacrifice it (agent := they)) (.perform (loseLife (.letter .x) (agent := they)))
              (agent := controllerOf it))) := by
  decide

/-- "An opponent sacrifices a creature: Draw a card." (`Keyword.badForeignSacrificeCost`) -/
theorem foreignSacrificeCostHandoffTwin :
    Ability.check [] (.activated (.perform (act anOpponent (Actor.sacrifice (a creature)))) drawOne
        none none none none) = [.costPaidByYou] ∧
      Ability.check [] (.activated (.perform (act anOpponent (Actor.sacrifice (a creature)))) drawOne
          none none none none) =
        Ability.check [] (.activated (.perform (sacrifice (a creature) (agent := anOpponent))) drawOne
          none none none none) := by
  decide

/-- "An opponent pays 2 life: Draw a card." (`Keyword.badForeignPayerCost`) -/
theorem foreignPayerCostHandoffTwin :
    Ability.check [] (.activated (.perform (act anOpponent (Actor.loseLife (.lit 2)))) drawOne
        none none none none) = [.costPaidByYou] ∧
      Ability.check [] (.activated (.perform (act anOpponent (Actor.loseLife (.lit 2)))) drawOne
          none none none none) =
        Ability.check [] (.activated (payLife anOpponent 2) drawOne none none none none) := by
  decide

/-- "Cumulative upkeep—An opponent loses 1 life." (`Mana.badOpponentPaysYourCost`) -/
theorem opponentPaysYourCostHandoffTwin :
    Ability.check [] (keywordCosting "CumulativeUpkeep" (.perform (act anOpponent (Actor.loseLife (.lit 1)))))
        = [.keywordCostPaidByYou "CumulativeUpkeep"] ∧
      Ability.check [] (keywordCosting "CumulativeUpkeep" (.perform (act anOpponent (Actor.loseLife (.lit 1))))) =
        Ability.check [] (keywordCosting "CumulativeUpkeep" (.perform (loseLife (.lit 1) (agent := anOpponent)))) := by
  decide

/-- "You may pay an opponent's 1 life. If you do, draw a card." (`Choice.badMismatchedPayer`) -/
theorem mismatchedPayerHandoffTwin :
    Instruction.check []
        (Primitives.Instruction.offer (.pay (.perform (act anOpponent (Actor.loseLife (.lit 1)))) .once
          (agent := .you)) none (some drawOne) (agent := .you)) = [.payAgrees] ∧
      Instruction.check []
          (Primitives.Instruction.offer (.pay (.perform (act anOpponent (Actor.loseLife (.lit 1)))) .once
            (agent := .you)) none (some drawOne) (agent := .you)) =
        Instruction.check []
          (Primitives.Instruction.offer (.pay (payLife anOpponent 1) .once (agent := .you)) none
            (some drawOne) (agent := .you)) := by
  decide

/-- "Pay 2 life: Draw a card." handed to "you" (`Keyword.okOwnPayerCost`). -/
theorem ownPayerCostHandoffTwin :
    Ability.check [] (.activated (.perform (act .you (Actor.loseLife (.lit 2)))) drawOne
        none none none none) = [] ∧
      Ability.check [] (.activated (.perform (act .you (Actor.loseLife (.lit 2)))) drawOne
          none none none none) =
        Ability.check [] (.activated (payLife .you 2) drawOne none none none none) := by
  decide

/-! ## A captured `actor` keeps its handoff -/

private semantic_macro capturedLifeLoss (who : capture NounPhrase) : Instruction :=
  .changeLife (.down (.lit 1)) who

/-- "You may pay 1 life of <who>. If you do, draw a card.", the payer being "you". -/
private def youPayWith (c : Cost) : Instruction :=
  Primitives.Instruction.offer (.pay c .once (agent := .you)) none (some (Actor.draw (.lit 1)))
    (agent := .you)

/-- With no handoff, `actor` passed through a `capture` parameter is the controller, so the
controller pays their own life. -/
theorem okCapturedActorIsYou :
    Instruction.check [] (youPayWith (.perform (capturedLifeLoss actor))) = [] := by
  decide

/-- Inside "target opponent …", the same captured `actor` is the opponent: the controller paying
the opponent's life is refused, exactly as the uncaptured `Actor.loseLife` is. -/
theorem badCapturedActorIsTheHandedPlayer :
    Instruction.check [] (act (target .opponent) (youPayWith (.perform (capturedLifeLoss actor))))
        = [.payAgrees] ∧
      Instruction.check [] (act (target .opponent) (youPayWith (.perform (Actor.loseLife (.lit 1)))))
        = [.payAgrees] := by
  decide

/-! ## Handoffs -/

/-- "Each opponent may pay {2}. If they don't, you draw a card.": `act you` inside another
player's handoff. -/
theorem okEachOpponentMayPayElseYouDraw :
    Instruction.check []
      (act (each .opponent)
        (Primitives.Instruction.offer (.pay (.mana [generic 2]) .once (agent := actor)) none
          (some (act .you (Actor.draw (.lit 1)))) (agent := actor))) = [] := by
  decide

/-- The same sentence through `doUnless`, whose decider is the offer's agent. -/
theorem okEachOpponentMayPayElseYouDrawUnless :
    Instruction.check []
      (doUnless (act .you (Actor.draw (.lit 1))) (.mana [generic 2]) (agent := each .opponent)) = [] := by
  decide

/-- "Each player chooses a creature they control." through the handoff
(`Choice.okAgentScopedChoice` through the chooser slot). -/
theorem okHandedScopedChoice :
    Instruction.check []
      (act (each .anyPlayer) (Actor.choose (a (.and [creature, .hasPossessor .controller they]))))
      = [] := by
  decide

/-- "Choose a creature you control.", "you" being the performer. -/
theorem okActorControlsChoice : Instruction.check [] (Actor.choose (a (.and [creature, actorControls]))) = [] := by
  decide

/-- "You choose a creature they control.": `act you` publishes nothing, so "they" has no
antecedent (`Choice.badUnchooseredTheyControl`). -/
theorem badHandedToYouTheyControl :
    Instruction.check []
      (act .you (Actor.choose (a (.and [creature, .hasPossessor .controller they]))))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Target opponent may pay an opponent's 1 life. If they do, they draw a card.": inside the
handoff the payer is the opponent, so the payer rule reads them, as it reads any non-controller
payer (`badActorMismatchedPayer` is the same body with no handoff). -/
theorem okHandedPayerIsTheTarget :
    Instruction.check []
      (act (target .opponent)
        (Primitives.Instruction.offer (.pay (payLife anOpponent 1) .once (agent := actor)) none
          (some (Actor.draw (.lit 1))) (agent := actor))) = [] := by
  decide

/-- Nested handoffs: the innermost one names the performer, so `act you` inside an opponent's
handoff makes the actor the controller again. -/
theorem badNestedHandoffInnermost :
    Instruction.check []
      (act (target .opponent)
        (act .you
          (Primitives.Instruction.offer (.pay (payLife anOpponent 1) .once (agent := actor)) none
            (some (Actor.draw (.lit 1))) (agent := actor))))
      = [.payAgrees] := by
  decide

/-- "Target opponent … Until end of turn, target creature gains '{T}: You may pay an
opponent's 1 life. If you do, draw a card.'": a granted ability is performed by its own
controller, so the handoff around the grant does not reach the payer inside it
(`okHandedPayerIsTheTarget` is the same body handed off directly). -/
theorem badGrantedAbilityKeepsItsOwnPerformer :
    Instruction.check []
      (act (target .opponent)
        (.establish
          (.abilityGrant (target creature)
            (.activated .tapSymbol
              (Primitives.Instruction.offer (.pay (payLife anOpponent 1) .once (agent := actor)) none
                (some (Actor.draw (.lit 1))) (agent := actor))
              none none none none))
          (some untilEndOfTurn)))
      = [.payAgrees] := by
  decide

/-! ## Statics -/

/-- "You may cast spells from your graveyard.": with no handoff in a static ability the
permitted player is the controller, and the clause checks as the `you` spelling does. -/
theorem okStaticActorPermission :
    Ability.check [] (.static (Actor.mayCastFrom (allOf spell) (graveyardOf actor))) = [] ∧
      Ability.check [] (.static (Actor.mayCastFrom (allOf spell) (graveyardOf actor))) =
        Ability.check []
          (.static (mayPlayDeed (.action "Cast") .you (allOf spell) none
            (.play (some (graveyardOf .you)) none none false .itsOwnCost))) := by
  decide

/-- "Exile target creature. Its owner may cast it from exile.": the permission handed to the
exiled card's owner. -/
theorem okHandedCastPermission :
    Instruction.check []
      (.sequentially
        [ exile (target creature),
          act (ownerOf it) (.establish (Actor.mayCastFrom it exileZone) none) ]) = [] := by
  decide

/-! ## Named actions carry their performer -/

/-- "That player discards that card." as `enact (.action "Discard") … (some actor)` under the
handoff [CR#701.9a]. -/
theorem okDiscardWrapperCarriesActor :
    Instruction.check onePlayer
      (act they
        (.enact (.action "Discard") (.move (a .isCard) (.zone .hand (.possessedBy actor)) graveyard [])
          (some actor))) = [] := by
  decide

/-- The discard deed admits a player agent at all: the explicit-agent spelling. -/
theorem okDiscardWrapperWithAgent :
    Instruction.check []
      (.enact (.action "Discard") (.move (a .isCard) (.zone .hand .bare) graveyard [])
        (some (target .anyPlayer))) = [] := by
  decide

/-! ## What a handoff leaves bound -/

/-- A targeted performer stays published after its handoff, below what the body introduced. -/
theorem handedTargetStaysBound :
    shape (Instruction.intro [] (act (target .anyPlayer) (Actor.discard (a .isCard)))) =
      [(.object, .one, .a), (.player, .one, .target)] := by
  decide

/-- "That player": a pronoun performer introduces nothing; the body's mentions sit above the
player it read. -/
theorem handedPronounAddsOnlyTheBody :
    shape (Instruction.intro onePlayer (act they (Actor.discard (a .isCard)))) =
      [(.object, .one, .a), (.player, .one, .the)] := by
  decide

/-- "Each opponent": the members' mentions and the group, pluralized. -/
theorem handedGroupPublishesPlurals :
    shape (Instruction.intro [] (act (each .opponent) (Actor.discard (a .isCard)))) =
      [(.object, .many, .a), (.player, .many, .each)] := by
  decide

/-- `act you` publishes exactly what its body does with no handoff. -/
theorem handedToYouIsTransparent :
    (Instruction.intro [] (act .you (Actor.discard (a .isCard))) ==
      Instruction.intro [] (Actor.discard (a .isCard))) = true := by
  decide

end Semantics.Proofs.Actor
