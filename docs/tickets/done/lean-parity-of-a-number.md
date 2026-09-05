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

## Landing record

Change `rpulxnmuqwvqylxqpnlzrtqupqulrtqp`; English lock `covered`: 20,254.

**PROVE:** Moved the even/odd type to Words beside `NumberRegime` as `Parity`;
`DeckTrait.manaValueParity` consumes it. No `ManaParity` references remain in
Lean. The existing even/odd companion examples needed no explicit spelling
changes. The shared `lean/scripts/build` gate passed all 60 jobs without
warnings. Assurance for this rename: 0 pins restored, re-spelled, ignored,
added, or removed; all prior verdict assertions retained.

**DISCLOSE:** No changed acceptance, lexical guards, or syntax alternatives.
Deviations and additions: README terminology clarification only. Parity uses
its ordinary numerical meaning; no new Game Model concept or unresolved STOP.

**REPORT:** English coverage and declaration data are untouched; English
selection, structural, inventory, and performance measurements are outside
this rename's scope. No CR citations changed.
