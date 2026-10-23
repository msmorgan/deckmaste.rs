import Semantics.Definitions.Phrasings

/-!
# Semantics.Definitions.CardTypeRules

What each card type confers on the objects of that type: one `Definition.cardType` row per
declared card type (`Semantics.Definitions.CardTypes`). The value carries the type's own fields;
the conferrals live here, above `Semantics.Templates`, because a conferral holds abilities, which
hold card types. Each row writes the permission to cast or play a card of the type from its
owner's hand, and the creature's Combatant role.
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## Card types -/

/-- [CR#302.1] A creature card may be cast from its owner's hand as a sorcery; [CR#508.1a] a
creature is a Combatant, a quality and not an ability [CR#113.12]. -/
def creatureType : Definition :=
  .cardType .creature [castableFrom yourHand asSorcery, combatant]

/-- [CR#301.1] An artifact card may be cast from its owner's hand as a sorcery. -/
def artifactType : Definition := .cardType .artifact [castableFrom yourHand asSorcery]

/-- [CR#305.1] "A player who has priority may play a land card from their hand during a main
phase of their turn when the stack is empty." -/
def landType : Definition := .cardType .land [playableFrom yourHand asSorcery]

/-- [CR#303.1] An enchantment card may be cast from its owner's hand as a sorcery. -/
def enchantmentType : Definition := .cardType .enchantment [castableFrom yourHand asSorcery]

/-- [CR#304.1] "A player who has priority may cast an instant card from their hand." -/
def instantType : Definition := .cardType .instant [castableFrom yourHand asInstant]

/-- [CR#307.1] A sorcery card may be cast from its owner's hand as a sorcery. -/
def sorceryType : Definition := .cardType .sorcery [castableFrom yourHand asSorcery]

/-- [CR#306.1] A planeswalker card may be cast from its owner's hand as a sorcery. -/
def planeswalkerType : Definition := .cardType .planeswalker [castableFrom yourHand asSorcery]

/-- [CR#310.1] A battle card may be cast from its owner's hand as a sorcery. -/
def battleType : Definition := .cardType .battle [castableFrom yourHand asSorcery]

/-- [CR#308.1] A kindred card is cast and resolves by its other card type, so kindred confers
nothing of its own. -/
def kindredType : Definition := .cardType .kindred []

/-- [CR#309.2c] "Dungeon cards are not permanents. They can't be cast." -/
def dungeonType : Definition := .cardType .dungeon []

/-- Every card type's row, in the declared order (`cardTypes`). -/
def cardTypeDefinitions : List Definition :=
  [ creatureType, artifactType, landType, enchantmentType, instantType, sorceryType,
    planeswalkerType, battleType, kindredType, dungeonType ]

end Semantics.Definitions
