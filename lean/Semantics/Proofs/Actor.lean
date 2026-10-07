import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Actor

The performer of an instruction and the handoff that names another one. `actor` is whoever
performs the instruction it is part of: the controller [CR#109.5] unless an enclosing `act`
hands the instruction to another player or to a permanent. Each pin is closed by `decide`.
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
"their hand" and the second handoff to "that player" read it. "A nonland card from it" says
`isCard` as the RON card's `cardIn` does: a token can sit in a hand until state-based actions
are checked [CR#111.7,704.5d]. -/
theorem okThoughtseize :
    Instruction.check []
      (.sequentially
        [ act (target .anyPlayer) Actor.revealHand,
          choose (a (.and [.not land, .and [.isCard, .inZone (handOf they)]])),
          act they (Actor.discard (that .card)),
          loseLife (.lit 2) ]) = [] := by
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
        (create (.lit 1) (creatureToken 0 0 [.black] [creatureType "Orc", creatureType "Army"]))
        none,
      choose (a Actor.army),
      .putCounters (.lit 2) (.printed p1p1Counter) (that (.type .creature)),
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

/-- "Draw a card.": the same refusals and the same published bindings as handed to "you". -/
theorem actorDrawIsYouDraw :
    Instruction.check [] (draw (.lit 1)) = Instruction.check [] (act .you (draw (.lit 1))) ∧
      (Instruction.intro [] (draw (.lit 1)) == Instruction.intro [] (act .you (draw (.lit 1))))
        = true := by
  decide

/-- "Lose 2 life.": as handed to "you". -/
theorem actorLoseLifeIsYouLoseLife :
    Instruction.check [] (loseLife (.lit 2)) = Instruction.check [] (act .you (loseLife (.lit 2))) ∧
      (Instruction.intro [] (loseLife (.lit 2)) ==
        Instruction.intro [] (act .you (loseLife (.lit 2)))) = true := by
  decide

/-- "You may pay 1 life. If you do, draw a card." (`Choice.okMatchedPayer` with the actor) -/
theorem okActorMatchedPayer :
    Instruction.check []
      (Primitives.Instruction.offer (.pay (payLife 1) .once) none
        (some (draw (.lit 1)))) = [] := by
  decide

/-- "You pay an opponent's 1 life" (`Choice.badMismatchedPayer` with the actor): the payer is the
controller, so the payment must be theirs. -/
theorem badActorMismatchedPayer :
    Instruction.check []
      (Primitives.Instruction.offer (.pay (.perform (act anOpponent (loseLife (.lit 1)))) .once) none
        (some (draw (.lit 1))))
      = [.payAgrees] := by
  decide

/-- "Pay 2 life: Draw a card." (`Keyword.okOwnPayerCost` with the actor) -/
theorem okActorOwnPayerCost :
    Ability.check [] (.activated (payLife 2) (draw (.lit 1)) none none none none) = [] := by
  decide

/-- "Target opponent loses 2 life: Draw a card.": a cost may hand its instruction to another
player, and a life payment inside the handoff is that player's, not the controller's. -/
theorem badHandedOffLifeCost :
    Ability.check []
      (.activated (.perform (act (target .opponent) (loseLife (.lit 2)))) (draw (.lit 1))
        none none none none)
      = [.costPaidByYou] := by
  decide

/-! ## Costs: each handoff twin judged as its explicit-agent form

Every cost instruction on the bench or in the pins whose agent is not "you", written once with
the explicit agent and once handed off; both gave the same refusals. The explicit-agent spelling
is gone (`semantics-v2-drop-agent-fields`): each pin's second half now names the handoff the bench
writes for the same card, so it states that the bench's own spelling is the one judged. -/

private def drawOne : Instruction := draw (.lit 1)

/-- Wall of Shards: "Cumulative upkeep—An opponent gains 1 life." -/
theorem wallOfShardsHandoffTwin :
    Ability.check [] (cumulativeUpkeep (.perform (act anOpponent (gainLife (.lit 1))))) = [] ∧
      Ability.check [] (cumulativeUpkeep (.perform (act anOpponent (gainLife (.lit 1))))) =
        Ability.check [] (cumulativeUpkeep (.perform (act anOpponent (gainLife (.lit 1))))) := by
  decide

/-- Varchild's War-Riders: "Cumulative upkeep—Have an opponent create a 1/1 red Survivor creature
token." -/
theorem varchildsWarRidersHandoffTwin :
    Ability.check []
        (cumulativeUpkeep (.perform (act anOpponent
          (create (.lit 1) (creatureToken 1 1 [.red] [creatureType "Survivor"]))))) = [] ∧
      Ability.check []
          (cumulativeUpkeep (.perform (act anOpponent
            (create (.lit 1) (creatureToken 1 1 [.red] [creatureType "Survivor"]))))) =
        Ability.check []
          (cumulativeUpkeep (.perform (act anOpponent (Primitives.Instruction.create (.lit 1)
            (.written (creatureToken 1 1 [.red] [creatureType "Survivor"])) [])))) := by
  decide

private def controlsForest : Condition :=
  exists_ (.and [land, .hasSubtype (landType "Forest"), .hasPossessor .controller .you])

/-- Invigorate: "If you control a Forest, rather than pay this spell's mana cost, you may have an
opponent gain 3 life." -/
theorem invigorateHandoffTwin :
    Ability.check []
        (.static (onlyWhile (.altCost .this (some (.perform (act anOpponent (gainLife (.lit 3))))))
          controlsForest)) = [] ∧
      Ability.check []
          (.static (onlyWhile (.altCost .this (some (.perform (act anOpponent (gainLife (.lit 3))))))
            controlsForest)) =
        Ability.check []
          (.static (onlyWhile (.altCost .this (some (.perform (act anOpponent (gainLife (.lit 3))))))
            controlsForest)) := by
  decide

private def blockersTheyControl : Amount :=
  times (.lit 1) (countOf (.and [creature, blocking, .hasPossessor .controller they]))

/-- Heat Wave: "Nonblue creatures can't block creatures you control unless their controller pays 1
life for each blocking creature they control." -/
theorem heatWaveHandoffTwin :
    Ability.check []
        (.static (deontic (allOf (.and [creature, .not (.colorIs .blue)]))
          (.gatedBy (.perform (act they (loseLife blockersTheyControl)))) [.core .block] .agent
          (.counterpart (allOf creatureYouControl)))) = [] ∧
      Ability.check []
          (.static (deontic (allOf (.and [creature, .not (.colorIs .blue)]))
            (.gatedBy (.perform (act they (loseLife blockersTheyControl)))) [.core .block] .agent
            (.counterpart (allOf creatureYouControl)))) =
        Ability.check []
          (.static (deontic (allOf (.and [creature, .not (.colorIs .blue)]))
            (.gatedBy (.perform (act they (loseLife blockersTheyControl)))) [.core .block] .agent
            (.counterpart (allOf creatureYouControl)))) := by
  decide

/-- Killing Wave: "For each creature, its controller sacrifices it unless they pay X life." -/
theorem killingWaveHandoffTwin :
    Instruction.check []
        (Primitives.Instruction.doForEach (each creature)
          (act (controllerOf it)
            (doUnless (act they (sacrifice it)) (.perform (act they (loseLife (.letter .x))))))) = [] ∧
      Instruction.check []
          (Primitives.Instruction.doForEach (each creature)
            (act (controllerOf it)
              (doUnless (act they (sacrifice it)) (.perform (act they (loseLife (.letter .x))))))) =
        Instruction.check []
          (Primitives.Instruction.doForEach (each creature)
            (act (controllerOf it)
              (doUnless (act they (sacrifice it)) (.perform (act they (loseLife (.letter .x))))))) := by
  decide

/-- "An opponent sacrifices a creature: Draw a card." (`Keyword.badForeignSacrificeCost`) -/
theorem foreignSacrificeCostHandoffTwin :
    Ability.check [] (.activated (.perform (act anOpponent (sacrifice (a creature)))) drawOne
        none none none none) = [.costPaidByYou] ∧
      Ability.check [] (.activated (.perform (act anOpponent (sacrifice (a creature)))) drawOne
          none none none none) =
        Ability.check [] (.activated (.perform (act anOpponent (sacrifice (a creature)))) drawOne
          none none none none) := by
  decide

/-- "An opponent pays 2 life: Draw a card." (`Keyword.badForeignPayerCost`) -/
theorem foreignPayerCostHandoffTwin :
    Ability.check [] (.activated (.perform (act anOpponent (loseLife (.lit 2)))) drawOne
        none none none none) = [.costPaidByYou] ∧
      Ability.check [] (.activated (.perform (act anOpponent (loseLife (.lit 2)))) drawOne
          none none none none) =
        Ability.check [] (.activated (.perform (act anOpponent (loseLife (.lit 2)))) drawOne none none none
          none) := by
  decide

/-- "Cumulative upkeep—An opponent loses 1 life." (`Mana.badOpponentPaysYourCost`) -/
theorem opponentPaysYourCostHandoffTwin :
    Ability.check [] (keywordCosting "CumulativeUpkeep" (.perform (act anOpponent (loseLife (.lit 1)))))
        = [.keywordCostPaidByYou "CumulativeUpkeep"] ∧
      Ability.check [] (keywordCosting "CumulativeUpkeep" (.perform (act anOpponent (loseLife (.lit 1))))) =
        Ability.check [] (keywordCosting "CumulativeUpkeep" (.perform (act anOpponent (loseLife (.lit 1))))) := by
  decide

/-- "You may pay an opponent's 1 life. If you do, draw a card." (`Choice.badMismatchedPayer`) -/
theorem mismatchedPayerHandoffTwin :
    Instruction.check []
        (Primitives.Instruction.offer (.pay (.perform (act anOpponent (loseLife (.lit 1)))) .once)
          none (some drawOne)) = [.payAgrees] ∧
      Instruction.check []
          (Primitives.Instruction.offer (.pay (.perform (act anOpponent (loseLife (.lit 1)))) .once)
            none (some drawOne)) =
        Instruction.check []
          (Primitives.Instruction.offer (.pay (.perform (act anOpponent (loseLife (.lit 1)))) .once)
            none (some drawOne)) := by
  decide

/-- "Pay 2 life: Draw a card." handed to "you" (`Keyword.okOwnPayerCost`). -/
theorem ownPayerCostHandoffTwin :
    Ability.check [] (.activated (.perform (act .you (loseLife (.lit 2)))) drawOne
        none none none none) = [] ∧
      Ability.check [] (.activated (.perform (act .you (loseLife (.lit 2)))) drawOne
          none none none none) =
        Ability.check [] (.activated (payLife 2) drawOne none none none none) := by
  decide

/-! ## A captured `actor` keeps its handoff -/

private semantic_macro capturedLifeLoss (who : capture NounPhrase) : Instruction :=
  .act who (.changeLife (.down (.lit 1)))

/-- "You may pay 1 life of <who>. If you do, draw a card.", the payer being "you". -/
private def youPayWith (c : Cost) : Instruction :=
  act .you (Primitives.Instruction.offer (.pay c .once) none (some (draw (.lit 1))))

/-- With no handoff, `actor` passed through a `capture` parameter is the controller, so the
controller pays their own life. -/
theorem okCapturedActorIsYou :
    Instruction.check [] (youPayWith (.perform (capturedLifeLoss actor))) = [] := by
  decide

/-- Inside "target opponent …", "you may pay that player's 1 life": the controller paying the
opponent's life is refused, captured or not. The payer "you" is a handoff back to the controller,
inside which `actor` is the controller again, so the opponent is named "that player"
(`capturedActorIsTheHandedPlayer` captures `actor` itself inside the opponent's handoff). -/
theorem badCapturedActorIsTheHandedPlayer :
    Instruction.check [] (act (target .opponent) (youPayWith (.perform (capturedLifeLoss they))))
        = [.payAgrees] ∧
      Instruction.check [] (act (target .opponent) (youPayWith (.perform (act they (loseLife (.lit 1))))))
        = [.payAgrees] := by
  decide

/-- Inside "target opponent …", `actor` passed through a `capture` parameter is the opponent:
"Target opponent loses 1 life: Draw a card." through the captured performer is refused as the
uncaptured handoff is (`badHandedOffLifeCost`). -/
theorem capturedActorIsTheHandedPlayer :
    Ability.check []
      (.activated (.perform (act (target .opponent) (capturedLifeLoss actor))) (draw (.lit 1))
        none none none none)
      = [.costPaidByYou] := by
  decide

/-! ## Handoffs -/

/-- "Each opponent may pay {2}. If they don't, you draw a card.": `act you` inside another
player's handoff. -/
theorem okEachOpponentMayPayElseYouDraw :
    Instruction.check []
      (act (each .opponent)
        (Primitives.Instruction.offer (.pay (.mana [generic 2]) .once) none
          (some (act .you (draw (.lit 1)))))) = [] := by
  decide

/-- The same sentence through `doUnless`, handed to each opponent, who decides. -/
theorem okEachOpponentMayPayElseYouDrawUnless :
    Instruction.check []
      (act (each .opponent) (doUnless (act .you (draw (.lit 1))) (.mana [generic 2]))) = [] := by
  decide

/-- "Each player chooses a creature they control." through the handoff
(`Choice.okAgentScopedChoice` through the chooser slot). -/
theorem okHandedScopedChoice :
    Instruction.check []
      (act (each .anyPlayer) (choose (a (.and [creature, .hasPossessor .controller they]))))
      = [] := by
  decide

/-- "Choose a creature you control.", "you" being the performer. -/
theorem okActorControlsChoice : Instruction.check [] (choose (a (.and [creature, actorControls]))) = [] := by
  decide

/-- "You choose a creature they control.": `act you` publishes nothing, so "they" has no
antecedent (`Choice.badUnchooseredTheyControl`). -/
theorem badHandedToYouTheyControl :
    Instruction.check []
      (act .you (choose (a (.and [creature, .hasPossessor .controller they]))))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Target opponent may pay an opponent's 1 life. If they do, they draw a card.": inside the
handoff the payer is the opponent, so the payer rule reads them, as it reads any non-controller
payer (`badActorMismatchedPayer` is the same body with no handoff). -/
theorem okHandedPayerIsTheTarget :
    Instruction.check []
      (act (target .opponent)
        (Primitives.Instruction.offer (.pay (.perform (act anOpponent (loseLife (.lit 1)))) .once) none
          (some (draw (.lit 1))))) = [] := by
  decide

/-- Nested handoffs: the innermost one names the performer, so `act you` inside an opponent's
handoff makes the actor the controller again. -/
theorem badNestedHandoffInnermost :
    Instruction.check []
      (act (target .opponent)
        (act .you
          (Primitives.Instruction.offer (.pay (.perform (act anOpponent (loseLife (.lit 1)))) .once) none
            (some (draw (.lit 1))))))
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
              (Primitives.Instruction.offer (.pay (.perform (act anOpponent (loseLife (.lit 1)))) .once) none
                (some (draw (.lit 1))))
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

/-- "That player discards that card." as `enact (.action "Discard") …` under the handoff, the
deed performed by the actor in context [CR#701.9a]. -/
theorem okDiscardWrapperCarriesActor :
    Instruction.check onePlayer
      (act they
        (.enact (.action "Discard") (.move (a .isCard) (.zone .hand (.possessedBy actor)) graveyard [])))
      = [] := by
  decide

/-- The discard deed admits a player performer at all: "Target player discards a card." handed to
that player. -/
theorem okDiscardWrapperWithAgent :
    Instruction.check []
      (act (target .anyPlayer) (.enact (.action "Discard") (.move (a .isCard) (.zone .hand .bare) graveyard [])))
      = [] := by
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

/-! ## A group handoff publishes its amount outcomes as totals

What each member's body introduced is published as a group, but an amount outcome stays one
total, as the explicit plural agent leaves it: "each opponent loses 1 life and you gain life equal
to the total life lost this way" [CR#702.101a]. -/

/-- Syndic of Tithes, its extort body [CR#702.101a] written over `loss` for "each opponent loses
1 life", followed by "you gain that much life". -/
private def syndicOfTithes (loss : Instruction) : Card :=
  .singleFaced
    { characteristics :=
      { name := "Syndic of Tithes", cost := some [generic 1, pip .white], types := [.creature],
        subtypes := [creatureType "Human", creatureType "Cleric"],
        text :=
          [ .keyword "Extort" []
              [ .triggered
                  (.casts .you
                    (some (.described (.a .unmarked)
                      (.and [.inZone (.zone .stack .bare), .not (.abilityHead .anyOnStack)])))
                    none)
                  [] none [] none none none
                  (.withContinuation .optional
                    (.pay (.mana [hybridPip .white .black]) .once)
                    (some (.sequentially [loss, gainLife .thatMuch])) none) ] ],
        power := some (.lit 2), toughness := some (.lit 2) } }

/-- Extort handed to each opponent: "that much" reads the one total. -/
theorem okExtortHandedToEachOpponent :
    Card.check (syndicOfTithes (act (each .opponent) (loseLife (.lit 1)))) = [] := by
  decide

/-- The handoff twin of the explicit plural agent `changeLife (down 1) (each opponent)`, which is
gone (`semantics-v2-drop-agent-fields`); its replacement, the handoff of the bare life change, is
judged the same as the helper's spelling. -/
theorem extortHandoffTwin :
    Card.check (syndicOfTithes (act (each .opponent) (.changeLife (.down (.lit 1))))) = [] ∧
      Card.check (syndicOfTithes (act (each .opponent) (loseLife (.lit 1)))) =
        Card.check (syndicOfTithes (act (each .opponent) (.changeLife (.down (.lit 1))))) := by
  decide

/-- "Each opponent loses 1 life": the group and one `lifeLost` total, exactly what the explicit
plural agent published (its replacement is the handoff of the bare life change). -/
theorem handedGroupPublishesTotal :
    shape (Instruction.intro [] (act (each .opponent) (loseLife (.lit 1)))) =
        [(.outcome, .one, .the), (.player, .many, .each)] ∧
      (Instruction.intro [] (act (each .opponent) (loseLife (.lit 1))) ==
        Instruction.intro [] (act (each .opponent) (.changeLife (.down (.lit 1))))) = true := by
  decide

/-- "Each opponent discards a card. You gain that much life.": the body has no amount outcome,
so the group handoff publishes none and "that much" has nothing to read. -/
theorem badGroupHandoffNoOutcomeThatMuch :
    Instruction.check []
      (.sequentially [act (each .opponent) (Actor.discard (a .isCard)), gainLife .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Loses 1 life and discards a card." -/
private def lossAndDiscard : Instruction :=
  .sequentially [loseLife (.lit 1), Actor.discard (a .isCard)]

/-- "Each opponent loses 1 life and discards a card. You gain that much life.": "that much" reads
the life total; the discarded cards stay a group. -/
theorem okGroupHandoffSequenceReadsLifeTotal :
    Instruction.check []
        (.sequentially
          [ act (each .opponent) lossAndDiscard,
            gainLife .thatMuch ]) = [] ∧
      Instruction.check []
        (.sequentially
          [ act (each .opponent) lossAndDiscard,
            exile them ]) = [] := by
  decide

/-- "Each opponent loses 1 life, then loses 2 life. You gain that much life.": two totals, so
"that much" has no one antecedent. -/
theorem badGroupHandoffTwoTotalsThatMuch :
    Instruction.check []
      (.sequentially
        [ act (each .opponent) (.sequentially [loseLife (.lit 1), loseLife (.lit 2)]),
          gainLife .thatMuch ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "For each opponent, that player loses 1 life. You gain that much life.": `doForEach` still
publishes its body's outcomes as a group; only the handoff carries the total. -/
theorem badForEachLossThatMuch :
    Instruction.check []
      (.sequentially
        [ Primitives.Instruction.doForEach (each .opponent) (act they (loseLife (.lit 1))),
          gainLife .thatMuch ]) = [.quantOutcomeInScope 0] := by
  decide

/-! ## A permanent can be handed an instruction

The rules instruct a permanent itself to explore or endure [CR#701.44a,701.63a]; the handoff
takes a permanent as well as a player. Inside it `actor` is the permanent, and the steps the rule
gives "that permanent's controller" are handed on to `controllerOf actor`, inside which `actor`
is the controller and the permanent stays in view as "that permanent". An object-performed deed
written with no handoff to a permanent is performed by the source permanent. -/

/-- The `explore` declaration's body [CR#701.44a]: the exploring permanent's controller reveals
the top card of their library; a land goes to its owner's hand, otherwise "that permanent" gets
a +1/+1 counter and the card may go to the graveyard. `perm` is how the body names the exploring
permanent. -/
private def exploreBody (perm : NounPhrase) : Instruction :=
  act (controllerOf actor) (.sequentially
    [ .expose .reveal (.cards (.librarySlice .top (.lit 1) actor)),
      .doIf (.matches (that .card) land) (.move (that .card) .wherever (.zone .hand .bare) [])
        (some (.sequentially
          [ .putCounters (.lit 1) (.printed p1p1Counter) perm,
            .withContinuation .optional
              (.move (that .card) .wherever (.zone .graveyard .bare) []) none none ])) ])

/-- `explore` as the keyword-action loader wraps it: the deed, performed by the actor. -/
private def literalExplore (perm : NounPhrase := that .permanent) : Instruction :=
  .enact (.action "Explore") (exploreBody perm)

/-- `endure N` as declared [CR#701.63a]: the enduring permanent's controller chooses between
the counters on "that permanent" and the Spirit token. -/
private def literalEndure (n : Nat) : Instruction :=
  .enact (.action "Endure")
    (act (controllerOf actor)
      (.chooseModes (.range (some 1) (some 1))
        [ (none, .putCounters (.lit n) (.printed p1p1Counter) (that .permanent)),
          (none, create (.lit 1) (creatureToken n n [.white] [creatureType "Spirit"])) ]))

/-- `adapt N` as declared [CR#701.46a]: unchanged, still reading "this permanent". -/
private def literalAdapt (n : Nat) : Instruction :=
  .enact (.action "Adapt")
    (.doIf (.not (.matches thisPermanent (.hasCounters (some p1p1Counter))))
      (.putCounters (.lit n) (.printed p1p1Counter) thisPermanent) none)

/-- "Target creature explores.": handed to the creature, whose controller does the steps. -/
theorem okExploreHandedToTargetCreature :
    Instruction.check [] (act (target creature) literalExplore) = [] := by
  decide

/-- Deadeye Tracker's "This creature explores.": `this` introduces nothing, so the handoff puts
the permanent in view for "that permanent". -/
theorem okExploreHandedToThisCreature :
    Instruction.check [] (act thisCreature literalExplore) = [] := by
  decide

/-- The owner's spelling "that creature" reaches the permanent too, handed to a target creature
or to "this creature". -/
theorem okExploreReadsThatCreature :
    Instruction.check [] (act (target creature) (literalExplore (that (.type .creature)))) = [] ∧
      Instruction.check [] (act thisCreature (literalExplore (that (.type .creature)))) = [] := by
  decide

/-- "This creature endures 1." (the testing card Endure Handoff Probe) -/
theorem okEndureHandedToThisCreature :
    Instruction.check [] (act thisCreature (literalEndure 1)) = [] := by
  decide

/-- "Each creature you control explores.": a group of permanents, each exploring in turn. -/
theorem okExploreHandedToEachCreature :
    Instruction.check [] (act (each creature) literalExplore) = [] := by
  decide

/-- Inside a handoff to a permanent `actor` is that permanent: "target creature draws a card"
records a creature as the drawing player. Handed on to its controller, the draw checks. -/
theorem actorInPermanentHandoffIsThePermanent :
    Instruction.check [] (act (target creature) (draw (.lit 1)))
        = [.kindMismatch .player .object] ∧
      Instruction.check [] (act (target creature) (act (controllerOf actor) (draw (.lit 1))))
        = [] := by
  decide

/-- Explore handed to a player: the recorded performer is a player where the deed takes a
permanent, and "that permanent" has no antecedent. -/
theorem badExploreHandedToOpponent :
    Instruction.check [] (act (target .opponent) literalExplore)
      = [.kindMismatch .object .player, .possessorKind .controller .player,
         .anaphor (.word .permanent) .one 0, .enactAgentOk] := by
  decide

/-- A card in a graveyard is not a permanent, and is never handed an instruction [CR#110.1]. -/
theorem badHandoffToCardInGraveyard :
    Instruction.check [] (act (target (.and [.isCard, .inZone graveyard]))
        (act (controllerOf actor) (draw (.lit 1))))
      = [.handoffPerformer] := by
  decide

/-- Printed "Adapt 2" on a permanent's own ability, with no handoff: the recorded actor is the
controller, and the deed is performed by the source permanent [CR#701.46a]. -/
theorem okBareAdaptDefaultsToThis : Instruction.check [] (literalAdapt 2) = [] := by decide

/-- A bare "explore" and "endure 1" default to the source permanent the same way: `actor` in the
body is "this permanent", and "that permanent" reads it. -/
theorem okBareExploreAndEndureDefaultToThis :
    Instruction.check [] literalExplore = [] ∧ Instruction.check [] (literalEndure 1) = [] := by
  decide

/-- "Target opponent adapts 2": handed to another player, the actor is that player, not the
controller, so there is no default to the source permanent and the performer is refused. -/
theorem badAdaptHandedToOpponent :
    Instruction.check [] (act (target .opponent) (literalAdapt 2))
      = [.kindMismatch .object .player, .enactAgentOk] := by
  decide

/-- "Whenever a creature explores": the event's performer is a permanent, as the deed's row
says; "whenever a player explores" is refused, while a player still bolsters. -/
theorem exploreEventTakesAPermanent :
    GameEvent.check [] (.verbedEvent (some (a creature)) (.action "Explore") none none none) = [] ∧
      GameEvent.check [] (.verbedEvent (some (a .anyPlayer)) (.action "Explore") none none none)
        = [.kindMismatch .object .player] ∧
      GameEvent.check [] (.verbedEvent (some (a .anyPlayer)) (.action "Bolster") none none none)
        = [] := by
  decide

/-- A handoff to `this` publishes nothing, as a handoff to "you" publishes nothing: the
permanent is in view only inside the body. -/
theorem handedToThisPublishesOnlyTheBody :
    shape (Instruction.intro [] (act thisCreature (act (controllerOf actor) (draw (.lit 1)))))
      = [(.player, .one, .the)] := by
  decide

end Semantics.Proofs.Actor
