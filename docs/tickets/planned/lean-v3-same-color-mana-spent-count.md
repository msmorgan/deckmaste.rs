---
needs: [lean-v3-colored-mana-spent-count]
---
# "At least three mana of the same color was spent to cast it"

Standard constraints apply. Split from `lean-v3-colored-mana-spent-count` (owner ruling
2026-10-24), which covers a stated type of mana through `LinkedFacet.manaSpent (type :
ManaTypeTerm)` [CR#106.1b]. Ruled by the owner 2026-10-25 (below).

## Problem

*Clockwork Servant* and *Henge Walker* print adamant conditions on "at least three mana of the same
color" (*Clockwork Servant*: "When this creature enters, if at least three mana of the same color
was spent to cast it, draw a card."; *Henge Walker*: "If at least three mana of the same color
was spent to cast this spell, this creature enters with a +1/+1 counter on it."). No color is stated, so `manaSpent` with a `ManaTypeTerm` cannot name it. The condition
quantifies over colors: some color has at least three mana spent of it. Adamant is an ability
word with no rules meaning of its own [CR#207.2c].

## Change

**Ruled 2026-10-25: a dedicated facet**, not a condition quantifying over colors: `ColorTerm`
(`lit`, `chosen`, `colorOf`) has no binder, so quantifying would need new binder machinery in
Core and `Check/`. Add a Linked Facet (alongside `LinkedFacet.colorsSpent` [CR#702.44a]) for the
greatest amount of mana of any one color spent to cast it, with a Template reading it for the
spell, so both cards compare it `.atLeast 3`. It ranges over the five colors [CR#105.1]; colorless
mana is a mana type but not a color [CR#106.1b] and does not count. It stays distinct from the
stated-type count of `lean-v3-colored-mana-spent-count`. Name the facet per `CONTEXT-MAP.md` and
the game-model glossary.

## Tests

- *Clockwork Servant* and *Henge Walker* spelled in full.
- A pin that a stated-color count does not satisfy the same-color form and vice versa.
