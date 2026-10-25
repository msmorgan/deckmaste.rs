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

end Semantics.Proofs.CardFidelity
