import Semantics.Definitions.Actions

/-!
# Semantics.Definitions.CardTypeRules

What each card type confers on the objects of that type: one `RegistryDefinition.cardType` row per
declared card type (`Semantics.Definitions.CardTypes`). The value carries the type's own fields;
the conferrals live here, above `Semantics.Templates`, because a conferral holds abilities, which
hold card types. Each row writes the permission to cast or play a card of the type from its
owner's hand; the creature, planeswalker and battle rows add what their type rules give those
permanents in combat, the results of damage dealt to them, and their state-based actions.

A row is read with `this` bound to the object of the type, as a `RegistryDefinition`'s conferrals are, so
a rule the CR states of "a creature" writes `this`.
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## The results of damage

Each result is a rule, conferred as a property on what the rules give it to, so no effect that
removes abilities switches it off; an `asEvent` does it as the damage is dealt [CR#120.4c]. A
replacement of a result replaces this row's action, not the damage: wither and infect replace the
marking [CR#120.3d,702.80a,702.90b,702.90c]. -/

/-- The results of damage dealt to `this`, `result` reading the damage dealt as `thatMuch`. -/
semantic_template damageResult (result : Action) : Conferral :=
  .property (asEvent (does any (dealDamage anyAmount this_)) result)

/-! ## Creature -/

/-- [CR#302.5] "Creatures can attack and block." -/
def creatureCanAttack : Conferral := .property (can this_ attack)

/-- [CR#302.5] "Creatures can attack and block." -/
def creatureCanBlock : Conferral := .property (can this_ block)

/-- [CR#120.3e] "Damage dealt to a creature by a source with neither wither nor infect causes
that much damage to be marked on that creature." Wither and infect replace the marking
[CR#120.3d]. -/
def creatureDamageMarking : Conferral := damageResult (markDamage this_ thatMuch)

/-- [CR#704.5f] "If a creature has toughness 0 or less, it's put into its owner's graveyard." -/
def toughnessZero : Conferral :=
  .stateBased (.compareAmt (toughnessOf .this) .atMost 0) (putInto .this graveyard)

/-- [CR#704.5g] "If a creature has toughness greater than 0, it has damage marked on it, and the
total damage marked on it is greater than or equal to its toughness, that creature has been dealt
lethal damage and is destroyed." The marked damage is `statOf markedDamage` [CR#120.6]; "is
destroyed" is the destroy action, which regeneration can replace [CR#701.19a]. -/
def lethalDamage : Conferral :=
  .stateBased
    (.and [ .compareAmt (toughnessOf .this) .greater 0,
            .compareAmt (statOf .markedDamage .this) .greater 0,
            .compareAmt (statOf .markedDamage .this) .atLeast (toughnessOf .this) ])
    (destroy .this)

/-- [CR#302.6] "A creature's activated ability with the tap symbol or the untap symbol in its
activation cost can't be activated unless the creature has been under its controller's control
continuously since their most recent turn began." Haste is the exception [CR#702.10c]. -/
def summoningSicknessActivation : Conferral :=
  .property
    (onlyUnless
      (cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf this_,
        tapOrUntapInCost]))))
      (controlledSinceTheirTurnBegan this_))

/-- [CR#302.6] "A creature can't attack unless it has been under its controller's control
continuously since their most recent turn began." Haste is the exception [CR#702.10b]. -/
def summoningSicknessAttack : Conferral :=
  .property (onlyUnless (cant this_ attack) (controlledSinceTheirTurnBegan it))

/-- [CR#302.1] A creature card may be cast from its owner's hand as a sorcery; [CR#508.1a] a
creature is a Combatant, a quality and not an ability [CR#113.12]; it can attack and block
[CR#302.5], but neither attacks nor activates an ability with {T} or {Q} in its cost unless its
controller has controlled it continuously since their most recent turn began [CR#302.6]; damage
dealt to it is marked on it [CR#120.3e]; and the toughness and lethal-damage state-based actions
apply to it [CR#704.5f,704.5g]. -/
def creatureType : RegistryDefinition :=
  .cardType .creature
    [ castableFrom yourHand asSorcery, combatant, creatureCanAttack, creatureCanBlock,
      summoningSicknessAttack, summoningSicknessActivation, creatureDamageMarking, toughnessZero,
      lethalDamage ]

/-! ## Planeswalker -/

/-- [CR#306.5b] "A planeswalker has the intrinsic ability 'This permanent enters with a number
of loyalty counters on it equal to its printed loyalty number.'" A replacement effect
[CR#614.1c] whose count is the planeswalker's loyalty, `statOf (.stat .loyalty) .this`: off the
battlefield a planeswalker card's loyalty is its printed number [CR#306.5a], so the printed
number needs no projection of its own. When the entering object is read is the engine's
concern, not the term's (ADR §0.1 items 4 and 5; owner ruling 2026-10-22). -/
def planeswalkerLoyalty : Conferral :=
  .ability (.static
    (entersWithCounters .this (.statOf (.stat .loyalty) .this) (.named "loyaltyCounter")))

/-- [CR#306.6] "Planeswalkers can be attacked." -/
def planeswalkerCanBeAttacked : Conferral := .property (can any (attack this_))

/-- [CR#120.3c] "Damage dealt to a planeswalker causes that many loyalty counters to be removed
from that planeswalker." -/
def planeswalkerLoyaltyDamage : Conferral :=
  damageResult (loseCounters (some (.printed (.named "loyaltyCounter"))) (some .thatMuch) .this)

/-- [CR#704.5i] "If a planeswalker has loyalty 0, it's put into its owner's graveyard."
A planeswalker's loyalty on the battlefield is its loyalty-counter count [CR#306.5c,122.1e],
so the condition reads the counters and not the printed characteristic. -/
def loyaltyZero : Conferral :=
  .stateBased (.compareAmt (countersOn (.named "loyaltyCounter") .this) .eq 0)
    (putInto .this graveyard)

/-- [CR#306.1] A planeswalker card may be cast from its owner's hand as a sorcery; it enters with
loyalty [CR#306.5b], can be attacked [CR#306.6], loses loyalty to damage [CR#120.3c], and the
loyalty state-based action applies to it [CR#704.5i]. -/
def planeswalkerType : RegistryDefinition :=
  .cardType .planeswalker
    [ castableFrom yourHand asSorcery, planeswalkerLoyalty, planeswalkerCanBeAttacked,
      planeswalkerLoyaltyDamage, loyaltyZero ]

/-! ## Battle -/

/-- [CR#310.4b] "A battle has the intrinsic ability 'This permanent enters with a number of
defense counters on it equal to its printed defense number.'" The battle row of the same shape
[CR#614.1c], counting the battle's defense, which off the battlefield is its printed number
[CR#310.4a]. -/
def battleDefense : Conferral :=
  .ability (.static
    (entersWithCounters .this (.statOf (.stat .defense) .this) (.named "defenseCounter")))

/-- [CR#310.5] "Battles can be attacked." -/
def battleCanBeAttacked : Conferral := .property (can any (attack this_))

/-- [CR#120.3h] "Damage dealt to a battle causes that many defense counters to be removed from
that battle." -/
def battleDefenseDamage : Conferral :=
  damageResult (loseCounters (some (.printed (.named "defenseCounter"))) (some .thatMuch) .this)

/-- [CR#704.5w,310.8] "If a non-Siege battle has defense 0, it's put into its owner's
graveyard." A battle's defense on the battlefield is its defense-counter count [CR#310.4c], and
the Siege exclusion is a subtype read, not a seam. -/
def battleDefenseZero : Conferral :=
  .stateBased
    (.and [ .not (.matches .this (.hasSubtype siege)),
            .compareAmt (countersOn (.named "defenseCounter") .this) .eq 0 ])
    (putInto .this graveyard)

/-- [CR#310.9a] "As a battle enters the battlefield, its controller chooses a player to be its
protector. Which players may be chosen as its protector are determined by its battle type …
If it has no battle types, only its controller can be its protector." Siege is the one battle
type [CR#205.3q], and the Siege row makes its own choice among its controller's opponents
[CR#310.12a], so this row is the choice of a battle that is no Siege, and the only player it may
choose is its controller: a choice is of an indefinite, so it chooses "a player not other than
you". A rule and no ability of the battle [CR#113.12], so a `property`; the choice is made as
part of the battle's entering [CR#614.1c,614.12]. -/
def battleProtector : Conferral :=
  .property
    (onlyUnless (entersChoosingPlayer .this (a (.and [isPlayer, .not (.otherThan you)])))
      (.matches .this (.hasSubtype siege)))

/-- [CR#310.1] A battle card may be cast from its owner's hand as a sorcery; it enters with
defense [CR#310.4b], can be attacked [CR#310.5], loses defense to damage [CR#120.3h], the
non-Siege state-based action applies to it [CR#704.5w,310.8], and its controller chooses its
protector as it enters [CR#310.9a]. -/
def battleType : RegistryDefinition :=
  .cardType .battle
    [ castableFrom yourHand asSorcery, battleDefense, battleCanBeAttacked, battleDefenseDamage,
      battleDefenseZero, battleProtector ]

/-! ## The other card types -/

/-- [CR#301.1] An artifact card may be cast from its owner's hand as a sorcery. -/
def artifactType : RegistryDefinition := .cardType .artifact [castableFrom yourHand asSorcery]

/-- [CR#305.1] "A player who has priority may play a land card from their hand during a main
phase of their turn when the stack is empty." -/
def landType : RegistryDefinition := .cardType .land [playableFrom yourHand asSorcery]

/-- [CR#303.1] An enchantment card may be cast from its owner's hand as a sorcery. -/
def enchantmentType : RegistryDefinition := .cardType .enchantment [castableFrom yourHand asSorcery]

/-- [CR#304.1] "A player who has priority may cast an instant card from their hand." -/
def instantType : RegistryDefinition := .cardType .instant [castableFrom yourHand asInstant]

/-- [CR#307.1] A sorcery card may be cast from its owner's hand as a sorcery. -/
def sorceryType : RegistryDefinition := .cardType .sorcery [castableFrom yourHand asSorcery]

/-- [CR#308.1] A kindred card is cast and resolves by its other card type, so kindred confers
nothing of its own. -/
def kindredType : RegistryDefinition := .cardType .kindred []

/-- [CR#309.2c] "Dungeon cards are not permanents. They can't be cast." -/
def dungeonType : RegistryDefinition := .cardType .dungeon []

/-- Every card type's row, in the declared order (`cardTypes`). -/
def cardTypeDefinitions : List RegistryDefinition :=
  [ creatureType, artifactType, landType, enchantmentType, instantType, sorceryType,
    planeswalkerType, battleType, kindredType, dungeonType ]

end Semantics.Definitions
