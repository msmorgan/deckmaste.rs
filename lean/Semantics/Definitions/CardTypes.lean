import Semantics.Words

/-!
# Semantics.Definitions.CardTypes

The card types [CR#205.2a]: each declaration's name is its `CardType`, a value carrying its own
fields. A leaf catalog over `Semantics.Words` alone, below the checker, which compares card types
by these values and reads their fields; it never matches on a card type. What a card type confers
on its objects is its `Definition.cardType` row (`Semantics.Definitions.CardTypeRules`), not a
field of the value.

The declarations live in `namespace Semantics`, as `CardType.creature` and the rest, so that
`.creature` names one wherever a `CardType` is expected. A card names these entries: `spelled`
refuses a raw `CardType` structure in a card.
-/

namespace Semantics

/-! ## Card types -/

/-- Creature: a permanent type [CR#110.4], whose objects alone have power and toughness
[CR#302.4]. -/
semantic_definition CardType.creature : CardType :=
  { name := "Creature", permanent := true, stats := [.power, .toughness] }

/-- Artifact: a permanent type [CR#110.4]. -/
semantic_definition CardType.artifact : CardType := { name := "Artifact", permanent := true }

/-- Land: a permanent type [CR#110.4]. -/
semantic_definition CardType.land : CardType := { name := "Land", permanent := true }

/-- Enchantment: a permanent type [CR#110.4]. -/
semantic_definition CardType.enchantment : CardType :=
  { name := "Enchantment", permanent := true }

/-- Instant: not a permanent type [CR#110.4]; an instant card is an instant or sorcery card
[CR#110.4a,304.1]. -/
semantic_definition CardType.instant : CardType :=
  { name := "Instant", permanent := false, instantOrSorcery := true }

/-- Sorcery: not a permanent type [CR#110.4]; a sorcery card is an instant or sorcery card
[CR#110.4a,307.1]. -/
semantic_definition CardType.sorcery : CardType :=
  { name := "Sorcery", permanent := false, instantOrSorcery := true }

/-- Planeswalker: a permanent type [CR#110.4], whose objects alone have loyalty [CR#306.5]. -/
semantic_definition CardType.planeswalker : CardType :=
  { name := "Planeswalker", permanent := true, stats := [.loyalty] }

/-- Battle: a permanent type [CR#110.4], whose objects have defense [CR#310.4]. -/
semantic_definition CardType.battle : CardType :=
  { name := "Battle", permanent := true, stats := [.defense] }

/-- Kindred: neither a permanent type nor an instant or sorcery type. A kindred card always has
another card type, and is cast and resolves by it [CR#308.1], so the other type settles both
whether it can be a permanent [CR#110.4] and its Card Class. -/
semantic_definition CardType.kindred : CardType := { name := "Kindred", permanent := false }

/-- Dungeon: a card type all the same, though a dungeon begins outside the game [CR#309.2] and is
never a permanent or a spell [CR#309.2c]. -/
semantic_definition CardType.dungeon : CardType := { name := "Dungeon", permanent := false }

/-- The declared card types in canonical order: the order the checker remembers a referent's
types in (`normalizeTypes`), where the order they are written in is immaterial. -/
def cardTypes : List CardType :=
  [ .creature, .artifact, .land, .enchantment, .instant, .sorcery, .planeswalker, .battle,
    .kindred, .dungeon ]

end Semantics
