import Semantics.Cards
import Semantics.Check.Card

/-!
# Semantics.Proofs.CardFidelity

Pins on authored card trees whose printed text an earlier encoding dropped or misread
(`lean-v3-rules-and-card-fidelity` §§3–5): each states the corrected tree, or the part of it the
correction is about, and why the Oracle text and the rules want it.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.CardFidelity

/-- The text of a single-faced card. -/
def faceText : Card → List Ability
  | .singleFaced face => face.characteristics.text
  | _ => []

/-- The effect of a spell ability. -/
def spellEffect : Ability → Option Action
  | .spell _ effect => some effect
  | _ => none

/-- Memory Plunder: "You may cast target instant or sorcery card from an opponent's graveyard
without paying its mana cost." The cast is a one-shot optional instruction carried out during
resolution [CR#608.2g], not a durationless permission, which would last for the rest of the game
[CR#611.2a]. -/
theorem memoryPlunderCastsDuringResolution :
    ((faceText Cards.memoryPlunder.card).filterMap spellEffect) =
      [ may (cast (target (and [isInstantOrSorcery, isCard]))
          (from_ := graveyardOf anOpponent) (payment := withoutPaying)) ] := rfl

/-- Flawless Forgery and Arcane Proxy: "You may cast the copy without paying its mana cost," a
one-shot cast during resolution [CR#608.2g]. -/
theorem copiedCardIsCastDuringResolution :
    Cards.flawlessForgeryLine =
      sequentially
        [ exile (target (and [isInstantOrSorcery, in_ (graveyardOf anOpponent)])),
          copy .fromCardZone (that card) 1 [],
          may (cast (that .copy) (payment := withoutPaying)) ] := rfl

/-- Madness: the owner "may cast it by paying [cost] rather than paying its mana cost. If that
player doesn't, they put this card into their graveyard" [CR#702.35a]: a cast now, during the
trigger's resolution, or the graveyard. -/
theorem madnessCastsDuringTheTriggersResolution :
    (match madness (.mana [generic 1]) with
      | .keyword _ _ [_, .triggered _ _ _ _ _ _ _ (.instruct _ (.withContinuation _ arm _ _))] =>
        some arm
      | _ => none) =
      some (cast thisCard (from_ := exileZone)
        (payment := .payingInstead (.inCaller (.mana [generic 1])))) := rfl

/-- "That player discards a card" and "each player discards a card": `discard` moves a card from
its performer's own hand [CR#701.9a], so the instruction is handed to the player who discards,
not performed by the ability's controller. -/
theorem discardIsHandedToTheDiscardingPlayer :
    Cards.hypnoticSpecterDiscard = instruct they (discard (aAtRandom (cardIn hand))) ∧
      Cards.syphonMind =
        sequentially
          [ instruct (each isOtherPlayer) (discard (a (cardIn hand))),
            doForEach (each (and [isCard, was (discard which) thisWay])) (draw 1) ] ∧
      Cards.braidsFrightfulReturn =
        may (sacrifice (a isCreature)) (some (instruct eachOpponent (discard (a (cardIn hand))))) ∧
      (match Cards.immersturmSkullcairn with
        | .activated _ (.sequentially [_, discarding]) _ _ _ _ => some discarding
        | _ => none) = some (instruct (that .player) (discard (a (cardIn hand)))) :=
  ⟨rfl, rfl, rfl, rfl⟩

/-- "Counter target spell unless its controller pays {3}": the spell's controller is offered the
payment, and if they don't pay, the counterspell's controller counters it [CR#118.12a,608.2c],
handed back to `you`. -/
theorem unlessPaysCountersByTheCounterspellsController :
    (faceText Cards.manaLeak.card).filterMap spellEffect =
      [ instruct (controllerOf (target isSpell))
          (doUnless (instruct you (counter it)) [generic 3]) ] := rfl

/-- Killing Wave's "sacrifices it unless they pay X life" and Heat Wave's "unless their controller
pays 1 life for each blocking creature they control" pay life as a cost (`payLife`, [CR#119.4]),
not a life loss handed to the payer. -/
theorem lifePaymentsArePayLife :
    Cards.killingWave =
      doForEach (each isCreature)
        (instruct (controllerOf it) (doUnless (instruct they (sacrifice it)) (payLife (letter .x)))) :=
  rfl

/-- Candles' Glow prints "Splice onto Arcane {1}{W}" [CR#702.47a] after its spell ability. -/
theorem candlesGlowSplicesOntoArcane :
    (faceText Cards.candlesGlow.card).getLast? =
      some (Ability.splice (hasSubtype arcane) [generic 1, pip .white]) := rfl

/-- Giggling Skitterspike triggers "whenever this creature attacks, blocks, or becomes the target
of a spell": three trigger events [CR#603.2], the attack among them. -/
theorem gigglingSkitterspikeTriggersOnAttack :
    Cards.gigglingSkitterspikeTriggers =
      [does thisCreature attack, does thisCreature block, does (a isSpell) (target thisCreature)] :=
  rfl

/-- Cyclone: "sacrifice this enchantment unless you pay {G} for each wind counter on it. If you
pay, this enchantment deals damage equal to the number of wind counters on it to each creature and
each player": the payment's two outcomes [CR#118.12,118.12a], the damage when you pay. -/
theorem cycloneDealsDamageIfYouPay :
    (match Cards.cycloneUpkeepPayment with
      | .triggered _ _ _ _ _ _ _ (.sequentially [_, payment]) => some payment
      | _ => none) =
      some (may (pay (scaled [pip .green] (times 1 (countersOn windCounter thisEnchantment))) .once)
        (some (instruct thisEnchantment
          (dealDamage (countersOn windCounter thisEnchantment) (each (or [isCreature, isPlayer])))))
        (some (sacrifice thisEnchantment))) := rfl

/-- Purging Scythe: "deals 2 damage to the creature with the least toughness. If two or more
creatures are tied for least toughness, you choose one of them." The tie sentence modifies the
earlier text [CR#608.2c,609.5]: you choose among the least-toughness creatures first, and the
chosen one is dealt the damage. -/
theorem purgingScytheChoosesAmongTheLeastThenDeals :
    (match Cards.purgingScythe with
      | .triggered _ _ _ _ _ _ _ effect => some effect
      | _ => none) =
      some (sequentially
        [ choose (a (and [isCreature, superlative .min (.stat .toughness) isCreature])),
          instruct thisArtifact (dealDamage 2 (that creature)) ]) := rfl

/-! ## Printed move text: "under its owner's control" and "from your graveyard" -/

/-- The zone changes an action makes, in order: each move's written origin, its Observation
Point, and its riders. -/
def moves : Nat → Action → List (Location × ObservationPoint × List MoveRider)
  | 0, _ => []
  | _ + 1, .move _ from_ _ at_ riders => [(from_, at_, riders)]
  | n + 1, .sequentially steps => steps.flatMap (moves n)
  | n + 1, .inCaller body | n + 1, .ownScope _ body | n + 1, .instruct _ body
  | n + 1, .delay _ _ _ body | n + 1, .doForEachKind _ _ _ body => moves n body
  | n + 1, .withContinuation _ body _ _ => moves n body
  | n + 1, .triggerReflexively body _ reflexive => moves n body ++ moves n reflexive
  | _ + 1, _ => []

/-- The zone changes an ability's effect makes. -/
def abilityMoves : Ability → List (Location × ObservationPoint × List MoveRider)
  | .activated _ action .. | .spell _ action | .triggered _ _ _ _ _ _ _ action => moves 64 action
  | _ => []

/-- Turn to Mist, Voyager Staff and Graceful Reprieve return the card "under its owner's
control": with no such rider, an effect puts an object onto the battlefield under the control of
the player it instructs [CR#110.2a]. -/
theorem returnsUnderTheOwnersControl :
    (moves 64 Cards.turnToMist).getLast? =
        some (.anyLocation, .arrival, [(under (ownerOf it))]) ∧
      ((abilityMoves Cards.voyagerStaff).getLast?.map (·.2.2)) = some [(under (ownerOf it))] ∧
      moves 64 Cards.gracefulReprieve = [(.anyLocation, .arrival, [(under (ownerOf it))])] :=
  ⟨rfl, rfl, rfl⟩

/-- A move from a graveyard written with its printed origin is observed on departure, so a
"leaves a graveyard" trigger sees it [CR#603.6c,603.10a]; the target keeps its own
`in_ <graveyard>` for target legality [CR#115.1] (owner ruling 2026-10-24), and the checker
accepts the pair. -/
theorem graveyardOriginsAreDepartureObserved :
    abilityMoves Cards.hammerOfBogardan = [(yourGraveyard, .departure, [])] ∧
      moves 64 Cards.stormOfSouls = [(yourGraveyard, .departure, []), (.anyLocation, .arrival, [])] ∧
      (moves 64 Cards.bondOfRevival).head? = some (yourGraveyard, .departure, []) ∧
      (moves 64 Cards.everAfter).head? = some (yourGraveyard, .departure, []) ∧
      (moves 64 Cards.riseFromTheGrave).head? = some (graveyard, .departure, [(.under .you)]) ∧
      moves 64 Cards.hymnOfRebirth = [(graveyard, .departure, [(.under .you)])] ∧
      (moves 64 Cards.theLastRoninII).getLast? = some (yourGraveyard, .departure, []) ∧
      Action.check (DRS.ofList []) Cards.bondOfRevival = [] ∧
      Action.check (DRS.ofList []) Cards.hymnOfRebirth = [] := by
  refine ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, by decide, by decide⟩

/-- The same on the spelled cards: Eerie Ultimatum, Rogues' Gallery, Blood Spatter Analysis's
reflexive return, and Midnight Scavengers' front face, each "from your graveyard". -/
theorem spelledGraveyardOriginsAreDepartureObserved :
    (faceText Cards.eerieUltimatum.card).flatMap abilityMoves = [(yourGraveyard, .departure, [])] ∧
      (faceText Cards.roguesGallery.card).flatMap abilityMoves =
        [(yourGraveyard, .departure, [])] ∧
      ((faceText Cards.bloodSpatterAnalysis.card).flatMap abilityMoves).getLast? =
        some (yourGraveyard, .departure, []) ∧
      (match Cards.midnightScavengers.card with
        | .transforming front _ => front.characteristics.text.flatMap abilityMoves
        | _ => []) = [(yourGraveyard, .departure, [])] := ⟨rfl, rfl, rfl, rfl⟩

/-- A bare "return it to the battlefield", with no rider and no origin, stays observed on
arrival [CR#603.6a]. -/
theorem bareReturnToBattlefieldIsObservedOnArrival :
    moves 64 (returnToBattlefield (target isCreature)) = [(.anyLocation, .arrival, [])] := rfl

/-! ## Card witnesses that dropped or misread a clause -/

/-- The trigger event of a card's triggered abilities, in order. -/
def triggerEvents (card : Card) : List GameEvent :=
  (faceText card).filterMap fun
    | .triggered event .. => some event
    | _ => none

/-- Darksteel Garrison: "Whenever fortified land becomes tapped" fires on the change from untapped
to tapped [CR#603.2e], for mana or not (ruling 2007-05-01). -/
theorem darksteelGarrisonWatchesBecomingTapped :
    triggerEvents Cards.darksteelGarrison.card = [does (fortified land) (become tapped)] := rfl

/-- Lightmine Field deals damage "equal to the number of attacking creatures", counted as the
effect applies [CR#608.2h]: a creature removed from combat isn't attacking [CR#506.4]
(rulings 2010-06-15), so the amount is the attacking creatures, not the triggering group. -/
theorem lightmineFieldCountsAttackingCreaturesOnResolution :
    ((faceText Cards.lightmineField.card).filterMap fun
      | .triggered _ _ _ _ _ _ _ effect => some effect
      | _ => none) =
      [ instruct thisEnchantment
          (dealDamage (countOf isAttacking) (eachOf (those creature))) ] := rfl

/-- Temple Altisaur: "another Dinosaur you control", the bare "another" contrasting with the
source of the ability (ADR semantics-v3 §0.1). -/
theorem templeAltisaurProtectsAnotherDinosaur :
    (faceText Cards.templeAltisaur.card) =
      [ static (replace (would (a isSource)
          (dealDamage anyAmount (a (and [dinosaur, youControl, isOther]))))
          (instead (preventDamage (allBut 1)))) ] := rfl

/-- Aether Gust: "target spell or permanent that's red or green". -/
theorem aetherGustTargetsRedOrGreenSpellOrPermanent :
    (match Cards.aetherGustPlacement with
      | .sequentially (first :: _) => some first
      | _ => none) =
      some (choose (target (and [or [isSpell, isPermanent], or [hasColor .red, hasColor .green]]))) :=
  rfl

/-- Archfiend's Vessel: "exile it. If you do, create …" is followed in order [CR#608.2c]; a
reflexive trigger is only "when [a player] does" or "when … this way" [CR#603.12]. -/
theorem archfiendsVesselIfYouDoIsNoReflexiveTrigger :
    ((faceText Cards.archfiendsVessel.card).filterMap fun
      | .triggered _ _ _ _ _ _ _ (.withContinuation .required body (some _) none) => some body
      | _ => none) = [exile it] := rfl

/-- Memoricide: "Search … for any number of cards with that name and exile them. Then that player
shuffles." The exile is not dropped. -/
theorem memoricideExilesWhatItFinds :
    (match Cards.memoricideSearch with
      | .sequentially [_, _, exiling, _] => some exiling
      | _ => none) = some (exile them) := rfl

/-- Quakebringer's "This ability triggers only if …" is a trigger restriction, the `qualified`
guard, as Solved's [CR#702.169c]; the triggered ability itself has no intervening "if", which
would be rechecked as it resolves [CR#603.4], so the ability still deals its damage if
Quakebringer leaves in response. -/
theorem quakebringerRestrictsTriggeringOnly :
    (match Cards.quakebringerDamage with
      | .qualified (.triggered _ _ _ _ _ _ intervening _) _ _ guard _ =>
        some (intervening.isNone, guard.isSome)
      | _ => none) = some (true, true) := rfl

/-- Cult of the Waxing Moon: "Whenever a permanent you control transforms into a non-Human
creature": the transform deed, which converting also performs, with the characteristic the
permanent has immediately after [CR#701.27e]. -/
theorem cultOfTheWaxingMoonWatchesTransformingIntoANonHumanCreature :
    triggerEvents Cards.cultOfTheWaxingMoon.card =
      [ does any (transform (a (and [isPermanent, youControl]))
          (into := some (and [isCreature, not (hasSubtype human)]))) ] := rfl

/-- "The X with the most … or tied for the most" is membership in the tied set, not a singular
"the", since the game has no default for ties [CR#609.5,702.105a]: Padeem asks whether you control
an artifact with the greatest mana value, and dethrone, Seraphic Greatsword and Undercover Butler
watch an attack on a player with the most life. -/
theorem orTiedForTheMostIsMembership :
    (match Cards.padeemConsulOfInnovation with
      | .triggered _ _ _ _ _ _ intervening _ => intervening
      | _ => none) =
        some (exists_ (and [isArtifact, youControl,
          superlative .max (.stat .manaValue) (and [isArtifact, in_ battlefield])])) ∧
      (match dethrone with
        | .keyword _ _ [.triggered event ..] => some event
        | _ => none) =
        some (does thisCreature (.attack (some
          (a (and [.anyPlayer, .superlative .max (.playerStat .lifeTotal) .anyPlayer]))))) ∧
      triggerEvents Cards.seraphicGreatsword.card =
        [ does (equipped creature) (attack (some
            (a (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer])))) ] ∧
      triggerEvents Cards.undercoverButler.card =
        [ does thisCreature (attack (some
            (a (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer])))) ] :=
  ⟨rfl, rfl, rfl, rfl⟩

/-! ## Base power and toughness [CR#208.4] -/

/-- Curie, Emergent Intelligence draws "cards equal to its base power": its power ignoring effects
and counters that modify it [CR#208.4b]. -/
theorem curieDrawsItsBasePower :
    (match Cards.curieCombatDraw with
      | .triggered _ _ _ _ _ _ _ effect => some effect
      | _ => none) = some (draw (basePowerOf thisCreature)) := rfl

/-- Baird, Argivian Recruiter: "a creature with power greater than its base power", each creature
measured against its own base power [CR#208.4b]. -/
theorem bairdComparesPowerWithItsOwnBasePower :
    ((faceText Cards.bairdArgivianRecruiter.card).filterMap fun
      | .triggered _ _ _ _ _ _ intervening _ => intervening
      | _ => none) =
      [ exists_ (compareOver (and [isCreature, youControl])
          (minus (powerOf ownMember) (basePowerOf ownMember)) .greater 0) ] := rfl

/-- Vedalken Squirrel-Whacker: "you may exchange one result with this creature's base power or
base toughness", an exchange of values that sets each to the other [CR#701.12g,208.4]. -/
theorem squirrelWhackerExchangesWithBasePowerOrToughness :
    (match Cards.vedalkenSquirrelWhackerReroll with
      | .establish (.replace _ (.instead (.sequentially [_, exchanging])) ..) _ => some exchanging
      | _ => none) =
      some (may (chooseOne
        (exchange (values (theOutcome .rollResult) (basePowerOf thisCreature)))
        (exchange (values (theOutcome .rollResult) (baseToughnessOf thisCreature))))) := rfl

/-! ## Mana spent to turn a permanent face up [CR#116.2b] -/

/-- Overgrown Zealot: "Spend this mana only to turn permanents face up", the special action
[CR#116.2b] whatever its cost, morph, disguise, manifest or cloak. -/
theorem overgrownZealotSpendsToTurnFaceUp :
    (match faceText Cards.overgrownZealot.card with
      | [_, .activated _ (.addMana _ _ riders) ..] => riders
      | _ => []) = [spendOnly [toTurnFaceUp isPermanent]] := rfl

/-- Qarsi Deceiver: "Spend this mana only to cast a face-down creature spell, pay a mana cost to
turn a manifested creature face up, or pay a morph cost" [CR#701.40b,702.37e]. -/
theorem qarsiDeceiverSpendsOnManifestAndMorph :
    (match Cards.qarsiDeceiverMorphSpend with
      | .activated _ (.addMana _ _ riders) .. => riders
      | _ => []) =
      [ spendOnly
          [ toCast (and [isCreature, isFaceDown]),
            toTurnFaceUp (and [isCreature, was (manifest which) thisObject]),
            toPay (.ofKeyword morph) ] ] := rfl

end Semantics.Proofs.CardFidelity
