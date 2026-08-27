# Type-line order — the measured table for the declaration author

The order the card-type words print in on a type line. `deckmaste_english_v2`
builds no type line yet — there is no `TypeLine` construction in
`src/constructions.rs`, and the crate's only card-type contact is a
`kinds = [Type, Subtype]` field and the subtype lexemes. This file is where
the order lives until there is one, and it is the input a type-line
declaration is written from.

Why here and not in the semantics: a type line's meaning is a **set** of card
types. [CR#205.1] has the type line contain the card's card type(s) and states
no order, so the order is English's marking inside the meaning, declared at
this boundary. Authority:
`docs/decisions/kind-index-joins-union-marking-is-spelling.md` and the verdict
in `docs/tickets/done/workbench-type-line-order-is-spelling.md`, which retired
the workbench's order gate for a distinctness gate and left the table homeless.
It arrived here from `idris/src/Experimental/Words.idr`'s `typePrintOrder`,
which is deleted.

## Method

Measured over `data/mtgjson/AllPrintings.sqlite` on 2026-08-26, scope
`isFunny = 0` (the excluded printings are named below). The unit is the
distinct card name: every card's `type` string was cut at the em dash, the
supertypes dropped, and the remaining card-type words read as a sequence. Only
a line carrying two or more card types says anything about order — 1,667 such
cards, writing twelve distinct sequences.

## The table

| rank | type | attested |
|---|---|---|
| 0 | Kindred | yes |
| 1 | Enchantment | yes |
| 2 | Artifact | yes |
| 3 | Land | yes |
| 4 | Creature | yes |
| 5 | Planeswalker | yes |
| 6 | Battle | no |
| 7 | Instant | no (only after Kindred) |
| 8 | Sorcery | no (only after Kindred) |
| 9 | Conspiracy | no |
| 10 | Dungeon | no |
| 11 | Phenomenon | no |
| 12 | Plane | no |
| 13 | Scheme | no |
| 14 | Vanguard | no |

Ranks 0–5 are measured. Ranks 6–14 are a **convention**, not a measurement,
and no printing can decide them — see the two sections below. The convention
for 9–14 is [CR#300.1]'s own listing order, which is alphabetical.

## What the printings attest

Twelve ordered pairs, no contradictions among them, all consistent with ranks
0–5:

| cards | pair | example |
|---|---|---|
| 1271 | Artifact < Creature | Bottle Gnomes |
| 286 | Enchantment < Creature | Summon: Magus Sisters |
| 25 | Kindred < Sorcery | All Is Dust |
| 24 | Kindred < Instant | Wings of Velis Vel |
| 22 | Artifact < Land | Tree of Tales |
| 20 | Kindred < Enchantment | Rebellion of the Flamekin |
| 9 | Kindred < Artifact | Veteran's Armaments |
| 5 | Enchantment < Artifact | Whip of Erebos |
| 2 | Land < Creature | Dryad Arbor |
| 2 | Artifact < Planeswalker | The Aetherspark |
| 2 | Enchantment < Land | Urza's Saga |
| 1 | Land < Planeswalker | Wrenn and One |

Excluded by the scope, and each one a counterexample to a rank the table
keeps: `Instant Creature` (Lightning Colt, Visitor from Planet Q — Mystery
Booster playtest) against Creature < Instant, and `Artifact Enchantment`
(Greatest Show in the Multiverse — Unfinity) against Enchantment < Artifact.
Neither printing is vintage-legal.

## What no printing can attest

Battle never shares a type line with another card type, and Instant and
Sorcery are never printed beside anything but Kindred, so ranks 6, 7 and 8 are
fixed only relative to Kindred.

The six command-zone types are attested at **zero**: no printed card or token
carries a conspiracy, dungeon, phenomenon, plane, scheme or vanguard type
beside any other card type. Every printing of them is the type alone —
`Conspiracy`, `Dungeon`, `Dungeon — Undercity`, `Phenomenon`, `Plane`,
`Scheme`, `Ongoing Scheme` (`Ongoing` is a supertype [CR#205.4a]), `Vanguard`.
The workbench admits such a line — no rule refuses it, see
`typesCombinable` in `idris/src/Experimental/Card.idr` — so ranks 9–14 exist
to give the linearizer a total order over a case English has never had to
spell, and nothing but a new printing can promote them from convention to
measurement.
