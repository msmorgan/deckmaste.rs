---
needs: []
---
# kw-soulshift (+ Afterlife sibling)

Author the **Soulshift** and **Afterlife** keyword-ability macros under
`plugins/builtin/macros/keyword/`. Both are dies-triggered abilities that
expand entirely over existing engine primitives — no core enum additions.

## Soulshift N [CR#702.46a]

"When this permanent is put into a graveyard from the battlefield, you may
return target Spirit card with mana value N or less from your graveyard to your
hand." Pieces:

- `ThisDies` trigger ([CR#700.4]).
- `May(...)` ([CR#702.46a] "you may").
- `Targeted` with one target ([CR#115.1,601.2c]) whose filter conjoins
  `Subtype("Spirit")`, `InZone(Graveyard)`, `Owner(Ref(You))` ("your
  graveyard"), and `Stat(ManaValue, AtMost, Param(0))` — the printed N as a
  mana-value bound ([CR#202.3]). The `Target(Exactly(1), …)` spec is `TargetOne`
  INLINED (a macro body can't thread `Param(0)` into a nested macro's argument
  slot, the same constraint Modular inlines around).
- `Move(Target(0), Hand)` returns the chosen card to its owner's hand
  ([CR#400.7]).

The mana-value target clause is the pre-existing `Stat(ManaValue, …)`
characteristic filter — no engine gap.

## Afterlife N [CR#702.135a]

"When this permanent is put into a graveyard from the battlefield, create N 1/1
white and black Spirit creature tokens with flying." Pieces:

- `ThisDies` trigger ([CR#700.4]).
- `Create(Param(0), Token(…))` ([CR#111.1]) of an inline creature token: color
  White ∧ Black via color indicator ([CR#202.2e]), a Creature Spirit, 1/1, with
  `Keyword(Flying)`. The Spirit subtype is inlined as `Subtype(name: "Spirit",
  types: [Creature, Kindred])` (what the generated `CreatureType` macro expands
  to) because the concrete creature-subtype macros live under gitignored
  `plugins/wizards/`, not `plugins/builtin`.

The inline `Token` already carries every needed field
(color_indicator/types/subtypes/power/toughness/abilities) — no engine gap.

## Done

- Both macros authored; engine tests in
  `crates/deckmaste_cards/tests/keywords.rs` (two structural tests + rows in
  `every_builtin_keyword_macro_expands`).
- Graduation: 5363 → 5388 (+25 cards). Remaining Soulshift/Afterlife `.todo`
  cards are blocked on *other* unparsed abilities, not the keyword.
