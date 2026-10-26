import Semantics.Definitions
import Semantics.Check.Rules

/-!
# Semantics.Proofs.Rules

The rules-table bench and its pins. Not a port of an Idris family — Idris never modelled the
tables — so the sentences here are the Comprehensive Rules' own, and the bench items are the
rows `plugins/builtin/rules` and `plugins/builtin/tokens` actually write.

Each `ok…` fixes the empty refusal list for a real row; each `bad…` fixes the complete list a
malformed one earns. `distinctPredefinedTokens` is asserted over the predefined-token catalog.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Rules


/-! ## State-based actions -/

/-- A scope binds objects or players; a color is neither. -/
theorem badSbaScopeKind :
    SbaRule.check
      { scope := .qualityNoun .color none,
        when := .compareAmt (toughnessOf .this) .atMost 0,
        then_ := putInto .this graveyard }
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-! ## Conferrals -/

/-- A conferral scopes what can have an ability — an object, or a player granted one
[CR#113.1a,113.1b] — and a color is neither. -/
theorem badConferralScopeKind :
    ConferralRule.check
      { scope := .qualityNoun .color none, confer := .ability (keyword (.named "flying")) }
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-- [CR#702.170d] "A plotted card's owner may cast it from exile without paying its mana cost
during their main phase while the stack is empty during any turn after the turn in which it became
plotted. … A plotted card may be cast this way even if it doesn't have the plot ability while in
exile." A rule for every plotted card, not part of plot's text [CR#702.170a]; "you" on a card in
exile is its owner [CR#109.5]. -/
def plottedCardCast : ConferralRule :=
  { scope := .and [.isCard, hasDesignation plotted, in_ exileZone],
    confer :=
      .ability (static
        (onlyUnless
          (can you (cast this_ (from_ := exileZone) (payment := withoutPaying))
            (timing := asSorcery))
          (did any (gainDesignation this_ plotted) soFarThisTurn))) }
theorem okPlottedCardCast : ConferralRule.check plottedCardCast = [] := by decide

/-- [CR#702.143d] "That card's owner may look at that card as long as it remains in exile": the
look right of every foretold card, whatever made it foretold. Its cast right, "for any foretell
cost it has", needs a cost term naming a keyword's cost, which the grammar does not have. -/
def foretoldCardLook : ConferralRule :=
  { scope := .and [.isCard, hasDesignation foretold, in_ exileZone],
    confer := .ability (static (visibility .lookAt you (objects this_))) }
theorem okForetoldCardLook : ConferralRule.check foretoldCardLook = [] := by decide

/-! ## The results of damage

Each result is a rule, conferred as a property on what the rules give it to, so no effect that
removes abilities switches it off; an `asEvent` does it as the damage is dealt [CR#120.4c]. A
replacement of a result replaces this row's action, not the damage: wither and infect replace the
marking and the life loss [CR#120.3d,702.80a,702.90b,702.90c]. Lifelink, an ability, is the
keyword's own body [CR#702.15b]. -/

/-- [CR#120.3a] "Damage dealt to a player by a source without infect causes that player to lose
that much life." In a row scoped to players, "you" is the player. Infect replaces the loss
(`infect`) [CR#702.90b]. -/
def playerDamageLifeLoss : ConferralRule :=
  { scope := isPlayer,
    confer := .property (asEvent (does any (dealDamage anyAmount you)) (loseLife thatMuch)) }
theorem okPlayerDamageLifeLoss : ConferralRule.check playerDamageLifeLoss = [] := by decide

/-- [CR#120.3g,702.164c] "Combat damage dealt to a player by a creature with toxic causes that
creature's controller to give the player a number of poison counters equal to that creature's
total toxic value, in addition to the damage's other results." One row for the creature,
reading the sum of its toxic abilities' N values [CR#702.164b], not one per toxic ability: the
keyword is bodiless, so combat damage from a creature with toxic 2 and toxic 1 gives three poison
counters once. -/
def toxicPoison : ConferralRule :=
  { scope := .and [isCreature, .hasKeyword (.the (.named "toxic"))],
    confer := .property
      (asEvent (does this_ (dealDamage anyAmount (a isPlayer) (some .combat)))
        (putCounters (totalToxicValue this_) poison (that player))) }
theorem okToxicPoison : ConferralRule.check toxicPoison = [] := by decide

/-- A creature with two toxic abilities has one result: each ability is the bare keyword, with
no body of its own, and the one row reads the total toxic value [CR#702.164b,702.164c]. -/
theorem toxicIsOneResultReadingTheTotal :
    (match toxic 2 with
      | .keyword (.named "toxic") [.number _] body => body.isEmpty
      | _ => false) = true := rfl

/-- The results of damage to a planeswalker are no ability of it: they are a property the rules
confer, so an effect that makes a permanent lose all abilities leaves them [CR#120.3c,113.12]. -/
theorem planeswalkerDamageResultIsNotAnAbility :
    (match planeswalkerLoyaltyDamage with
      | .property _ => true
      | _ => false) = true := rfl

/-! ## The rules that read a keyword

A keyword whose rule the CR states as a rule of the game, not as an ability of its object, holds
no body (`lean/CONTRACTS.md`, "Keyword abilities with no body"); its rule is a row here, as
toxic's is. -/

/-- [CR#702.2b,704.5h] "If a creature has toughness greater than 0, and it's been dealt damage by
a source with deathtouch since the last time state-based actions were checked, that creature is
destroyed." The source is read as it dealt the damage, from any zone [CR#702.2d]; "is destroyed"
is the destroy action, which regeneration can replace. -/
def deathtouchDestruction : SbaRule :=
  { scope := isCreature,
    when :=
      .and
        [ .compareAmt (toughnessOf .this) .greater 0,
          did (by_ (a (and [isSource, hasKeyword deathtouch]))) (dealDamage anyAmount .this)
            (within (.between (some .stateBasedActionsChecked) (some .now))) ],
    then_ := destroy .this }
theorem okDeathtouchDestruction : SbaRule.check deathtouchDestruction = [] := by decide

/-- [CR#703.4a,502.1,702.26a] "Immediately after the untap step begins, all phased-in permanents
with phasing that the active player controls phase out, and all phased-out permanents that the
active player controlled when they phased out phase in." One row, so each permanent does one or
the other: one phased out this way is not phased in again by the same action. The active player
is read as the controller whose turn it is [CR#102.1]. The phase-in half reads the phased-out
permanent's present controller, which phasing does not change [CR#702.26d]; it differs from the
one it phased out under only if a control effect ended while it was phased out [CR#702.26f], and
reading that one needs a term for the active player (`lean-v3-active-player-term`). An Aura,
Equipment or Fortification that phased out indirectly, with the permanent it is attached to,
doesn't phase in by itself but with that permanent [CR#702.26g], so the phase-in half skips a
phased-out permanent attached to a phased-out one. That reads "indirectly" by the attachment: an
attachment that phased out directly and whose host later phased out too is skipped as well,
which the rule would phase in (`lean-v3-phasing-direct-phase-out`). That every permanent changes
at once is the engine's. -/
def phasingTurnBased : ConferralRule :=
  { scope := isPermanent,
    confer :=
      .turnBased .untapStep
        (doIf (.duringPart .turn (some (controllerOf .this)))
          (doIf (is .this isPhasedOut)
            (doIf
              (not (is .this (.attachment .attachment none (some (a (and [isPermanent,
                isPhasedOut]))))))
              (.setStatus .phasedIn .this) none)
            (some (doIf (is .this (hasKeyword phasing)) (.setStatus .phasedOut .this) none)))
          none) }
theorem okPhasingTurnBased : ConferralRule.check phasingTurnBased = [] := by decide

/-! ## Registry definitions -/

/-- [CR#122.1] "A counter is a marker placed on an object or player." A charge counter is a
marker and nothing else: the CR gives it no behavior of its own, so it confers nothing. -/
def chargeCounterDefinition : Definition := .counter chargeCounter []
theorem okChargeCounter : Definition.check chargeCounterDefinition = [] := by decide

/-- [CR#122.1b] "A keyword counter on a permanent … causes that object to gain that keyword."
The counter is not an ability, so its conferral is the ability-free `property` flavor; but its
bearer gains the keyword in layer 6 [CR#613.1f], and that gained ability can be removed by
another effect [CR#113.12], applied in timestamp order [CR#613.7,613.7c]. -/
def flyingCounterDefinition : Definition :=
  .counter flyingCounter [.property (Templates.abilityGrant .this (keyword (.named "flying")))]
theorem okFlyingCounter : Definition.check flyingCounterDefinition = [] := by decide

/-- A keyword counter's conferral written with `grants` is the one `flyingCounterDefinition`
writes, the keyword gained by its bearer [CR#122.1b,613.1f]. -/
theorem grantsIsTheKeywordConferral :
    flyingCounterDefinition = .counter flyingCounter [grants (.named "flying")] := rfl

/-- [CR#122.1a] "A +X/+Y counter on a creature … adds X to that object's power and Y to that
object's toughness." The definition states what ONE counter confers, "+1/+1" in layer 7c
[CR#613.4c], and the model applies it once per counter held, so the conferral reads no count.

The conferral is about its bearer wherever the bearer is: a +1/+1 counter also counts "on a
creature card in a zone other than the battlefield" [CR#122.1a], and the checker asserts no zone
for the bearer of a stat change, a definition's or a card's (`lean-checker-sweep-zone`). -/
def p1p1CounterDefinition : Definition :=
  .counter (.named "p1p1Counter") [boost (.up 1) (.up 1)]
theorem p1p1CounterDefinitionZone :
    Definition.check p1p1CounterDefinition = [] := by decide

/-- [CR#122.1a]: the minus counter's twin of `p1p1CounterDefinition`, subtracting 1 from each
stat. -/
def m1m1CounterDefinition : Definition :=
  .counter (.named "m1m1Counter") [boost (.down 1) (.down 1)]
theorem m1m1CounterDefinitionZone :
    Definition.check m1m1CounterDefinition = [] := by decide

/-- The retired spelling of `p1p1Counter`'s conferral, which multiplied each stat by the count of
its own kind on the bearer, in two single-stat changes. Nothing is refused for reading its own
count or for not reading it, because multiplicity is the model's reading of a counter definition
and the checker computes none. -/
def ownCount : Amount := .statOf (.counter (.named "p1p1Counter")) .this
def p1p1CounterCountMultiplied : Definition :=
  .counter (.named "p1p1Counter")
    [ .property (Templates.modification .this .power (.up ownCount)),
      .property (Templates.modification .this .toughness (.up ownCount)) ]
theorem p1p1CounterCountMultipliedTwin :
    Definition.check p1p1CounterCountMultiplied = [] := by decide

/-- [CR#122.1f] "If a player has ten or more poison counters, that player loses the game." A
poison counter is a player's. -/
theorem okPoisonCounter : Definition.check (.counter (.named "poison" .player) []) = [] := by
  decide

/-- A counter is placed on an object or a player [CR#122.1] and on nothing else; a color is
neither. -/
theorem badCounterHolder :
    Definition.check (.counter (.named "chargeCounter" (.quality .color)) [])
      = [.definitionHolder (.quality .color)] := by
  decide

/-- Counter names distinguish kinds [CR#122.1]. This registry uses nonempty names as lookup
keys, so it refuses an unnamed row. -/
theorem badNamelessCounter :
    Definition.check (.counter (.named "") []) = [.definitionNamed] := by decide

/-- Subtypes are type-line vocabulary [CR#205.3a,205.3b]. This catalog row supplies the word
without an intrinsic conferral. -/
def gargoyle : Definition := .subtype Semantics.Definitions.gargoyle []
theorem okGargoyle : Definition.check gargoyle = [] := by decide

/-- Subtypes are written as words [CR#205.3b]. The registry requires a nonempty word as its
lookup key. -/
theorem badNamelessSubtype :
    Definition.check (.subtype (.of .creature "") []) = [.definitionNamed] := by decide

/-! ## The card types' rows

Each card type's declared `Definition.cardType` row (`Semantics.Definitions.CardTypeRules`) checks,
and each rule a row confers is pinned on the row: it is one of the row's conferrals, and it checks.
-/

/-- The conferrals a registry definition carries. -/
def confersOf : Definition → List Conferral
  | .counter _ confers | .subtype _ confers | .cardType _ confers | .designation _ confers =>
    confers
  | .relation _ => []

/-- [CR#302.1,508.1a,302.5,302.6,120.3e,704.5f,704.5g] A creature card may be cast as a
sorcery, a creature is a Combatant, it can attack and block, the summoning-sickness rule applies
to it, damage is marked on it, and the toughness and lethal-damage state-based actions apply to
it. -/
theorem okCreatureType : Definition.check creatureType = [] := by decide

/-- [CR#302.5] "Creatures can attack and block." -/
theorem okCreatureCanAttack :
    creatureCanAttack ∈ confersOf creatureType ∧ creatureCanAttack.check = [] :=
  ⟨by simp [confersOf, creatureType], by decide⟩
theorem okCreatureCanBlock :
    creatureCanBlock ∈ confersOf creatureType ∧ creatureCanBlock.check = [] :=
  ⟨by simp [confersOf, creatureType], by decide⟩

/-- [CR#302.6] "A creature can't attack unless it has been under its controller's control
continuously since their most recent turn began." -/
theorem okSummoningSicknessAttack :
    summoningSicknessAttack ∈ confersOf creatureType ∧ summoningSicknessAttack.check = [] :=
  ⟨by simp [confersOf, creatureType], by decide⟩

/-- [CR#302.6] "A creature's activated ability with the tap symbol or the untap symbol in its
activation cost can't be activated unless the creature has been under its controller's control
continuously since their most recent turn began." -/
theorem okSummoningSicknessActivation :
    summoningSicknessActivation ∈ confersOf creatureType ∧
      summoningSicknessActivation.check = [] :=
  ⟨by simp [confersOf, creatureType], by decide⟩

/-- [CR#302.6] The summoning-sickness reading's "their" reads the player its condition names, the
creature's controller: with no player named before it, "their most recent turn" reads nothing. -/
theorem badHeldSinceTheirTurnWithNoPlayer :
    Condition.check (DRS.ofList [])
      (heldSince (.matches this_ isCreature) (.beginningOf .turn none (.mostRecent they))) =
      [.anaphor (.word .player) .one 0] := by decide

/-- [CR#120.3e] Damage dealt to a creature by a source with neither wither nor infect is marked
on it. -/
theorem okCreatureDamageMarking :
    creatureDamageMarking ∈ confersOf creatureType ∧ creatureDamageMarking.check = [] :=
  ⟨by simp [confersOf, creatureType], by decide⟩

/-- [CR#704.5f] "If a creature has toughness 0 or less, it's put into its owner's graveyard." -/
theorem okToughnessZero :
    toughnessZero ∈ confersOf creatureType ∧ toughnessZero.check = [] :=
  ⟨by simp [confersOf, creatureType], by decide⟩

/-- [CR#704.5g] A creature with toughness greater than 0 and damage marked on it greater than or
equal to its toughness is destroyed. -/
theorem okLethalDamage :
    lethalDamage ∈ confersOf creatureType ∧ lethalDamage.check = [] :=
  ⟨by simp [confersOf, creatureType], by decide⟩

/-- [CR#306.1,306.5b,306.6,120.3c,704.5i] A planeswalker card may be cast as a sorcery; a
planeswalker enters with loyalty, can be attacked, loses loyalty to damage, and the loyalty
state-based action applies to it. -/
theorem okPlaneswalkerType : Definition.check planeswalkerType = [] := by decide

/-- [CR#306.5b] A planeswalker enters with loyalty counters equal to its printed loyalty, which
is the loyalty the row reads, the card not yet being on the battlefield [CR#306.5a]. -/
theorem okPlaneswalkerLoyalty :
    planeswalkerLoyalty ∈ confersOf planeswalkerType ∧ planeswalkerLoyalty.check = [] :=
  ⟨by simp [confersOf, planeswalkerType], by decide⟩

/-- [CR#306.6] "Planeswalkers can be attacked." -/
theorem okPlaneswalkerCanBeAttacked :
    planeswalkerCanBeAttacked ∈ confersOf planeswalkerType ∧
      planeswalkerCanBeAttacked.check = [] :=
  ⟨by simp [confersOf, planeswalkerType], by decide⟩

/-- [CR#120.3c] Damage dealt to a planeswalker removes that many loyalty counters from it. -/
theorem okPlaneswalkerLoyaltyDamage :
    planeswalkerLoyaltyDamage ∈ confersOf planeswalkerType ∧
      planeswalkerLoyaltyDamage.check = [] :=
  ⟨by simp [confersOf, planeswalkerType], by decide⟩

/-- [CR#704.5i] "If a planeswalker has loyalty 0, it's put into its owner's graveyard." -/
theorem okLoyaltyZero :
    loyaltyZero ∈ confersOf planeswalkerType ∧ loyaltyZero.check = [] :=
  ⟨by simp [confersOf, planeswalkerType], by decide⟩

/-- [CR#310.1,310.4b,310.5,120.3h,704.5w,310.9a] A battle card may be cast as a sorcery; a battle
enters with defense, can be attacked, loses defense to damage, the non-Siege state-based action
applies to it, and its controller chooses its protector as it enters. -/
theorem okBattleType : Definition.check battleType = [] := by decide

/-- [CR#310.4b] A battle enters with defense counters equal to its printed defense, which is the
defense the row reads, the card not yet being on the battlefield [CR#310.4a]. -/
theorem okBattleDefense :
    battleDefense ∈ confersOf battleType ∧ battleDefense.check = [] :=
  ⟨by simp [confersOf, battleType], by decide⟩

/-- [CR#310.5] "Battles can be attacked." -/
theorem okBattleCanBeAttacked :
    battleCanBeAttacked ∈ confersOf battleType ∧ battleCanBeAttacked.check = [] :=
  ⟨by simp [confersOf, battleType], by decide⟩

/-- [CR#120.3h] Damage dealt to a battle removes that many defense counters from it. -/
theorem okBattleDefenseDamage :
    battleDefenseDamage ∈ confersOf battleType ∧ battleDefenseDamage.check = [] :=
  ⟨by simp [confersOf, battleType], by decide⟩

/-- [CR#704.5w,310.8] "If a non-Siege battle has defense 0, it's put into its owner's
graveyard." Its defense on the battlefield is its defense-counter count [CR#310.4c]. -/
theorem okBattleDefenseZero :
    battleDefenseZero ∈ confersOf battleType ∧ battleDefenseZero.check = [] :=
  ⟨by simp [confersOf, battleType], by decide⟩

/-- [CR#310.9a,205.3q] As a battle with no battle type enters, its controller chooses its
protector, and the only player it may choose is itself. -/
theorem okBattleProtector :
    battleProtector ∈ confersOf battleType ∧ battleProtector.check = [] :=
  ⟨by simp [confersOf, battleType], by decide⟩

/-- [CR#301.1] An artifact card may be cast as a sorcery. -/
theorem okArtifactType : Definition.check artifactType = [] := by decide

/-- [CR#305.1] A land card may be played as a sorcery. -/
theorem okLandType : Definition.check landType = [] := by decide

/-- [CR#303.1] An enchantment card may be cast as a sorcery. -/
theorem okEnchantmentType : Definition.check enchantmentType = [] := by decide

/-- [CR#304.1] An instant card may be cast whenever its owner has priority. -/
theorem okInstantType : Definition.check instantType = [] := by decide

/-- [CR#307.1] A sorcery card may be cast as a sorcery. -/
theorem okSorceryType : Definition.check sorceryType = [] := by decide

/-- [CR#308.1] Kindred confers nothing of its own. -/
theorem okKindredType : Definition.check kindredType = [] := by decide

/-- [CR#309.2c] "Dungeon cards are not permanents. They can't be cast." -/
theorem okDungeonType : Definition.check dungeonType = [] := by decide

/-- The rows are the declared card types, one each, in the declared order. -/
theorem cardTypeRowsAreDeclared :
    cardTypeDefinitions.map (fun
      | .cardType type _ => some type
      | _ => none) = cardTypes.map some := by decide

/-- A card type is written as a word [CR#205.2a]; the registry refuses a nameless one. -/
theorem badNamelessCardType :
    Definition.check (.cardType { name := "", permanent := true } []) = [.definitionNamed] := by
  decide

/-! ## The rules-defined subtypes (ruling 2026-10-10, `semantics-v2-subtype-rules`) -/

/-- "This Aura/Equipment/Fortification is attached": the attachment relation the engine's
attach-permission model keeps [CR#704.5m,704.5n]. -/
def isAttached (subject : NounPhrase) : Condition :=
  .matches subject (.attachment .attachment none none)

/-- [CR#301.5,704.5n] An Equipment may be attached to a creature, and one attached to no legal
host becomes unattached. -/
def equipmentDefinition : Definition :=
  .subtype Semantics.Definitions.equipment
    [ .property (mayBeAttachedTo thisEquipment (allOf isCreature)),
      .stateBased (.not (isAttached thisEquipment)) (.attachment .unattached thisEquipment none) ]
theorem okEquipmentDefinition : Definition.check equipmentDefinition = [] := by decide

/-- [CR#301.6,704.5n] The Land twin of Equipment. -/
def fortificationDefinition : Definition :=
  .subtype Semantics.Definitions.fortification
    [ .property (mayBeAttachedTo thisFortification (allOf isLand)),
      .stateBased (.not (isAttached thisFortification))
        (.attachment .unattached thisFortification none) ]
theorem okFortificationDefinition : Definition.check fortificationDefinition = [] := by decide

/-- [CR#704.5m] An Aura that is not attached is put into its owner's graveyard. -/
def auraDefinition : Definition :=
  .subtype Semantics.Definitions.aura
    [ .stateBased (.not (isAttached thisAura))
        (.move thisAura (.zone .battlefield none) (some graveyard) .arrival []) ]
theorem okAuraDefinition : Definition.check auraDefinition = [] := by decide

/-- [CR#714.3a,714.3c,714.4] The declared Saga row (`Semantics.Definitions.SubtypeRules`): its
lore counter on entering, the lore increment and the sacrifice rule. -/
theorem okSagaDefinition : Definition.check sagaDefinition = [] := by decide

/-- [CR#310.12a,310.12b,704.5v] The declared Siege row (`Semantics.Definitions.SubtypeRules`): the
opponent protector, the last-counter trigger and the Siege state-based action. -/
theorem okSiegeDefinition : Definition.check siegeDefinition = [] := by decide

/-- [CR#310.12a] As a Siege enters, its controller chooses its protector among their
opponents. -/
theorem okSiegeProtector :
    siegeProtector ∈ confersOf siegeDefinition ∧ siegeProtector.check = [] :=
  ⟨by simp [confersOf, siegeDefinition], by decide⟩

/-- [CR#310.12b] When the last defense counter is removed from a Siege, it is exiled, and then
its controller may cast it transformed without paying its mana cost. -/
theorem okSiegeLastCounter :
    siegeLastCounter ∈ confersOf siegeDefinition ∧ siegeLastCounter.check = [] :=
  ⟨by simp [confersOf, siegeDefinition], by decide⟩

/-- [CR#704.5v] A Siege with defense 0 that is the source of no ability that has triggered but
not yet left the stack is put into its owner's graveyard. -/
theorem okSiegeDefenseZero :
    siegeDefenseZero ∈ confersOf siegeDefinition ∧ siegeDefenseZero.check = [] :=
  ⟨by simp [confersOf, siegeDefinition], by decide⟩

/-- The lore increment is a turn-based action [CR#714.3c], a game action and no effect
[CR#703.1,609.1]: the `turnBased` conferral that encloses it says so, and the put itself carries
no cause. Doubling Season's "if an effect would put" is `causedBy .anEffect` [CR#614.16], so it
doubles a Saga's entry counter, put by a replacement effect [CR#714.3a], and not the increment.
The checker has no matcher, so this pins the row's spelling; the matching is the engine's. -/
theorem sagaLoreIncrementIsTurnBased :
    ∃ parts, sagaDefinition = .subtype Semantics.Definitions.saga parts ∧
      parts[1]? = some (.turnBased .firstMain (doIf sagaHasChapterAbilities sagaLoreIncrement)) :=
  ⟨_, rfl, rfl⟩

/-- The triggered-chapter guard reads the Saga's chapter abilities that have triggered and not yet
left the stack, waiting ones included [CR#714.4,704.5s,113.7]. -/
theorem okSagaHasTriggeredChapter :
    Condition.check (DRS.ofList []) sagaHasTriggeredChapter = [] := by
  decide

/-- "An ability that has triggered but not yet left the stack" is an ability, so it binds an
ability referent alone, as the Siege's guard writes it [CR#704.5v,310.7]. -/
theorem triggeredNotLeftStackSeedsAnAbility : triggeredNotLeftStack.seedsAbility = true := rfl

/-- The Siege's guard, "it isn't the source of an ability that has triggered but not yet left the
stack", over the row's own subject, checks
[CR#704.5v,310.7]. -/
theorem okSiegeGuard : Condition.check (DRS.ofList []) siegeHasNoWaitingTrigger = [] := by decide

/-! ## The designations' rows

A designation's row carries its inherent triggered abilities [CR#725.2,726.2], read with no
source, and controlled by the player who held the designation when each triggered
(`Definition.controllingDesignation?`), an exception to [CR#113.8]. Each ability is pinned on its
row: it is one of the row's conferrals, and it checks in the row's context. -/

/-- [CR#725.1,725.2] "The monarch is a designation a player can have. There is no monarch in a
game until an effect instructs a player to become the monarch"; its two inherent triggered
abilities check. -/
theorem okMonarchDefinition : Definition.check monarchDefinition = [] := by decide

/-- [CR#725.2] "At the beginning of the monarch's end step, that player draws a card." -/
theorem okMonarchDraw :
    monarchDraw ∈ confersOf monarchDefinition ∧
      monarchDraw.checkIn monarchDefinition.conferralContext = [] :=
  ⟨by simp [confersOf, monarchDefinition], by decide⟩

/-- [CR#725.2] "Whenever a creature deals combat damage to the monarch, its controller becomes
the monarch." -/
theorem okMonarchSteal :
    monarchSteal ∈ confersOf monarchDefinition ∧
      monarchSteal.checkIn monarchDefinition.conferralContext = [] :=
  ⟨by simp [confersOf, monarchDefinition], by decide⟩

/-- [CR#726.1,726.2] The one of the initiative's inherent triggered
abilities the grammar can write checks; the two that venture into Undercity [CR#701.49d] wait on
a venture action (`lean-v3-venture-dungeon`). -/
theorem okInitiativeDefinition : Definition.check initiativeDefinition = [] := by decide

/-- [CR#726.2] "Whenever one or more creatures a player controls deal combat damage to the player
who has the initiative, the controller of those creatures takes the initiative." -/
theorem okInitiativeSteal :
    initiativeSteal ∈ confersOf initiativeDefinition ∧
      initiativeSteal.checkIn initiativeDefinition.conferralContext = [] :=
  ⟨by simp [confersOf, initiativeDefinition], by decide⟩

/-- [CR#725.2,726.2] The designations' abilities are controlled by the player who held the
designation when they triggered; a card type's are its objects' own, controlled as their source
is [CR#113.8]. -/
theorem designationControllerReading :
    monarchDefinition.controllingDesignation? = some monarch ∧
      initiativeDefinition.controllingDesignation? = some initiative ∧
      creatureType.controllingDesignation? = none := by decide

/-- [CR#725.2] The monarch's abilities have no source, so a trigger on its row that names "this"
names nothing, and is refused. -/
theorem badSourceInDesignationAbility :
    Definition.check
      (.designation monarch
        [.ability (at_ (beginningOfPossessed .endStep theMonarch) (sacrifice this_))])
      = [.noSource] := by decide

/-- The same text on a card names the card [CR#113.7]: the refusal is the designation's having no
source. -/
theorem okSourceInPrintedAbility :
    Ability.check (DRS.ofList [])
      (at_ (beginningOfPossessed .endStep theMonarch) (sacrifice this_)) = [] := by decide

/-- An ability a sourceless ability grants has a source of its own, the object it is granted to:
"this creature" in it names that creature. -/
theorem okSourceInGrantedAbility :
    Definition.check
      (.designation monarch
        [.ability
          (at_ (beginningOfPossessed .endStep theMonarch)
            (gain (target isCreature)
              (at_ (beginningOfYour .upkeep) (sacrifice thisCreature)) untilEndOfTurn))])
      = [] := by decide

/-- The rows are declared designations, each once. -/
theorem designationRowsAreDeclared :
    designationDefinitions.map Definition.controllingDesignation?
      = [some initiative, some monarch] := by decide

/-- [CR#709.5c] "'Left half unlocked' and 'right half unlocked' are designations that a
permanent on the battlefield can have." -/
def leftHalfUnlocked : Definition :=
  .designation (.named "leftHalfUnlocked" (.heldBy .object)) []
theorem okLeftHalfUnlocked : Definition.check leftHalfUnlocked = [] := by decide

/-- Goaded is one named designation [CR#701.15b]. This registry uses nonempty designation
names as lookup keys. -/
theorem badNamelessDesignation :
    Definition.check (.designation (.named "" (.heldBy .player)) [])
      = [.definitionNamed] := by
  decide

/-! ## The predefined-token catalog -/

/-- [CR#111.10a] "A Treasure token is a colorless Treasure artifact token with '{T}, Sacrifice
this token: Add one mana of any color.'" -/
def treasure : PredefinedToken :=
  { name := "Treasure",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.treasure],
            text :=
              [ activated (.compound [.tapSymbol, .perform (sacrifice .this)])
                  (.addMana 1 (.anyColor .sameColor) []) ] } } }
theorem okTreasure : PredefinedToken.check treasure = [] := by decide

/-- [CR#111.10b] "A Food token is a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" -/
def food : PredefinedToken :=
  { name := "Food",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.food],
            text :=
              [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice .this)])
                  (gainLife 3) ] } } }
theorem okFood : PredefinedToken.check food = [] := by decide

/-- [CR#111.10c] "A Gold token is a colorless Gold artifact token with 'Sacrifice this token:
Add one mana of any color.'" -/
def gold : PredefinedToken :=
  { name := "Gold",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.gold],
            text :=
              [ activated (.perform (sacrifice .this))
                  (.addMana 1 (.anyColor .sameColor) []) ] } } }
theorem okGold : PredefinedToken.check gold = [] := by decide

/-- [CR#111.10f] "A Clue token is a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" -/
def clue : PredefinedToken :=
  { name := "Clue",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.clue],
            text :=
              [ activated (.compound [.mana [generic 2], .perform (sacrifice .this)])
                  (draw 1) ] } } }
theorem okClue : PredefinedToken.check clue = [] := by decide

/-- [CR#111.10g] "A Blood token is a colorless Blood artifact token with '{1}, {T}, Discard a
card, Sacrifice this token: Draw a card.'" -/
def blood : PredefinedToken :=
  { name := "Blood",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.blood],
            text :=
              [ activated
                  (.compound
                    [ .mana [generic 1], .tapSymbol, .perform (discard (a (.and [.isCard, .in_ yourHand]))),
                      .perform (sacrifice .this) ])
                  (draw 1) ] } } }
theorem okBlood : PredefinedToken.check blood = [] := by decide

/-- [CR#111.10h] "A Powerstone token is a colorless Powerstone artifact token with '{T}: Add
{C}. This mana can't be spent to cast a nonartifact spell.'" [CR#106.6] -/
def powerstone : PredefinedToken :=
  { name := "Powerstone",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.powerstone],
            text :=
              [ activated .tapSymbol
                  (.addMana 1 (.runs [[.colorless]])
                    [.spendNotOn [.toCast (.and [.not isArtifact, isSpell])]]) ] } } }
theorem okPowerstone : PredefinedToken.check powerstone = [] := by decide

/-- [CR#111.10w] "A Vibranium token is a colorless Vibranium artifact token with indestructible
and '{T}: Add {C}. This mana can't be spent to cast a nonartifact spell.'" -/
def vibranium : PredefinedToken :=
  { name := "Vibranium",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.vibranium],
            text :=
              [ keyword (.named "indestructible"),
                activated .tapSymbol
                  (.addMana 1 (.runs [[.colorless]])
                    [.spendNotOn [.toCast (.and [.not isArtifact, isSpell])]]) ] } } }
theorem okVibranium : PredefinedToken.check vibranium = [] := by decide

/-- The seven predefined tokens the builtin plugin carries. -/
def builtinPredefinedTokens : List PredefinedToken :=
  [blood, clue, food, gold, powerstone, treasure, vibranium]

theorem builtinPredefinedTokensCheck :
    builtinPredefinedTokens.flatMap PredefinedToken.check = [] := by decide

theorem builtinPredefinedTokensDistinct :
    distinctPredefinedTokens builtinPredefinedTokens = true := by decide

/-- An effect creates a predefined token by name [CR#111.10], so an unnamed entry is
unreachable. -/
theorem badUnnamedPredefinedToken :
    PredefinedToken.check { treasure with name := "" } = [.tokenNamed] := by decide

/-! ## The builtin tables -/

/-- The predicate-scoped rules tables the builtin plugin carries. What a card type or a subtype
confers, its results of damage and state-based actions among them, is on that type's declared
`Definition` row (`Semantics.Definitions.CardTypeRules`, `Semantics.Definitions.SubtypeRules`),
not here. The deathtouch lethal-damage row [CR#704.5h] is not written yet. -/
def builtinTables : RulesTables :=
  { conferral := [plottedCardCast, foretoldCardLook, playerDamageLifeLoss, toxicPoison] }

theorem okBuiltinTables : RulesTables.check builtinTables = [] := by decide

end Semantics.Proofs.Rules
