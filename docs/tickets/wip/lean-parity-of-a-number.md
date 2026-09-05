---
needs: []
---
**Parity is a property of a number, not of mana.** `lean/Semantics/Abilities.lean`
declares `inductive ManaParity | even | odd`, used once, by
`DeckTrait.manaValueParity` for the companion trait "cards with even mana
values" ([CR#202.3] defines mana value as a number). Rename it `Parity`, move it
beside the other number vocabulary in `lean/Semantics/Words.lean` (next to
`NumberRegime`), and keep `manaValueParity (parity : Parity)`. No verdict
changes; re-spell the pins that name the old type.

Decisions already made: numbers are `Int` with a declared regime per slot
([CR#107.1,107.1b], ruling of 2026-09-05); a name never says where a value is read
from when the type is a property of the value itself.
