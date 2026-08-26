---
needs: []
---
# The distinct-kind count — an axis-keyed fold head

Routed from `workbench-counting-nouns-and-this-way` (which closed the other
two counting gaps): eight of the asymmetric definition's twelve lines
(Tarmogoyf among them) count distinct card TYPES among cards — "the number of
card types among cards in all graveyards". That counts distinct VALUES on a
characteristic axis, not members of a described set (`CountOf`) or of a named
mention (`CountOfGroup`), so neither existing head reaches it.

What it needs: an axis-keyed fold head over `Amount` — a domain (description
or mention) plus the axis whose distinct values are counted. Study the
existing `Aggregate`/`AggregateOf` op/axis split (`AggregateOp`, `ProjAxis`)
before minting new machinery: the natural shape may be a distinct-count op on
the existing axis vocabulary rather than a new head.

## Consumption boundary

`idris/src/Experimental/Phrase.idr` (the `Amount` head and its total-table
rows), evidence bench `idris/src/Experimental/Cards.idr`.
No Rust crate.

## Acceptance

- Tarmogoyf's definition line benches whole; the count is closed by design,
  not per-card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

One `Amount` head plus its own axis vocabulary; four files, +214 / −0.
`idris/scripts/build` PASS. No design artifact: the docstrings plus this
section are the record.

### The family, measured

The Amount reading — "the number of [axis] among [domain]" — is written **62
times in supported text**, over 31 distinct phrasings: basic land types among
lands you control 26, colours among permanents/mentions 16, card types 15
(the asymmetric definition's eight — Tarmogoyf, Barrowgoyf, Polygoyf,
Pyrogoyf, Tarmogoyf Nest, Consuming Blob, Nethergoyf, Nighthawk Scavenger —
plus Altar of the Goyf, Broodspinner, Deluge of Doom, Lucid Dreams, Loot),
permanent types 1, and one line each for
different mana values, different powers, different colour pairs, and different
kinds of counters. The DISTRIBUTIVE twin — "for each basic land type among
lands you control" (33), "for each color among …" (11) — is a separate ~52
lines and is NOT this head; see the ledger.

### The design: a head, not an op

`AggregateOp`'s three ops fold NUMBERS and `ProjAxis`'s two arms
(`CharAxis`, `PlayerStatAxis`) each name one number per referent. A distinct
count needs neither: [CR#205.2a]'s types and [CR#105.1]'s colours are not
numbers, and [CR#205.2b] gives one object several card types at once, so a
`DistinctOf` row on `AggregateOp` would be meaningless against every existing
axis and every existing op meaningless against every new one — payable only
with a cross-gate coupling op to axis at all three consumers (`Aggregate`,
`AggregateOf`, `Superlative`) plus `isExtremal`. The head costs one row and
gates nothing extra. `ProjAxis` again gained no rows.

- **`KindAxis`** (`Words.idr`), beside `ProjAxis`: `CardTypeAxis`
  [CR#205.2a], `PermanentTypeAxis` [CR#110.4], `ColorAxis` [CR#105.1],
  `SubtypeAxis host only` [CR#205.3c], `ValueAxis Characteristic`,
  `CounterKindAxis`. `ValueAxis` is why the distinct count is not an
  `AggregateOp` row at all: the same reading runs over a numeric
  characteristic ("different mana values") without folding it.
- **`SubtypeScope`** = `AnySubtype | BasicOnly | NonbasicOnly`, gated by
  `subtypeScopeOk` to the land type. [CR#305.6] says an object writing "basic
  land type" means one of five land subtypes and gives the term no other
  reading, so the split is the land type's alone. `NonbasicOnly` is the
  complement within [CR#205.3i]'s land types; its only corpus line (Mojave
  Desert) is unsupported.
- **`DistinctCount ax dom`** (`Phrase.idr`) with the five companion rows
  (`amtDelta`/`amtIntro`/`amtPlur`/`writtenBound`/`readAmount`); `boundEq`'s
  catch-all covers it.
- **ONE domain slot, a `Noun`** — no description/mention twin. The surface
  writes both with the same word: "among cards in all graveyards" is `AllOf`
  over a description, "among those creatures" a mention. The prior round's
  As-landed asked that a future round collapse `Aggregate`/`AggregateOver`
  rather than grow both; this one does not open a second pair.
- **Ungated on plurality.** A single object still has a set of types
  [CR#205.2b] and of colours [CR#105.2], and the corpus writes that reading
  with its own words — "the number of colors that spell is", 4 lines. The
  refusal would have had no rule.
- **"all graveyards" needed no machinery**: it is every player's own
  [CR#404.1], so `Macros.graveyardOf (PlayerGroup AllPlayers)`. The prior
  round's second double-block on five of the eight is discharged as a
  spelling, not a gap.

### Bench (`Cards.idr`), oracle text verified against local card data

`tarmogoyfDefinition` (the acceptance line, and verbatim Barrowgoyf's,
Polygoyf's, Pyrogoyf's and Tarmogoyf Nest's token's), `tarmogoyfBox`,
`consumingBlobDefinition` (also Nethergoyf's), `nighthawkScavengerDefinition`
(the offset count over the opponents' graveyards) — the asymmetric
definition's **eight card-type lines all now bench whole**. Beyond the
definition family: `lucidDreams`, `tribalFlames` (the basic-land-type axis
under `Domain`), `explosiveProdigyTrigger` (the colour axis under `Vivid`),
`korvoldCombatTrigger` (the permanent-type axis), `generalTazriPump` (the
domain as a MENTION, `Those`).

Pin (`ProofsG.idr`): `badBasicCreatureTypeAxis` — "the number of basic
creature types among creatures you control", refused on [CR#305.6] and not on
a count.

### Deviations and shortfalls

- **`generalTazriPump` writes "Ally creatures you control" with the creature
  card type in the description**; `Those (TypeW Creature)` needs the head type
  to match, and the subtype alone does not supply it.
- **`ValueAxis` and `CounterKindAxis` bench nothing.** Both of their corpus
  lines are blocked on frames outside this head — see the ledger. They are
  minted anyway: an axis vocabulary that could not say "different mana values"
  would draw its line at nothing in the rules.

## Ledger

- **`Gains` introduces no object binding a sibling clause can read** when the
  trigger already bound one. Perrie, the Pulverizer ("Whenever Perrie attacks,
  target creature you control gains trample and gets +X/+X until end of turn,
  where X is the number of different kinds of counters among permanents you
  control") is blocked either way round: `It` finds the attacker AND the
  target, so `countOnes Object` is 2. The `CounterKindAxis` witness.
- **`EventName` has no card-discard row**, so All-Seeing Arbiter's "Whenever
  you discard a card" trigger cannot be written. The `ValueAxis ManaValue`
  witness.
- **`searchLibraryFor` takes neither a quantity nor a destination.**
  Celebrate the Harvest ("Search your library for up to X basic land cards …
  Put those cards onto the battlefield tapped, then shuffle") needs both. The
  `ValueAxis Power` witness.
- **The distributive twin — "for each [axis] among [domain]"** — ~52 supported
  lines ("for each basic land type among lands you control" 33, "for each
  color among permanents you control" 11, "for each card type among …" ~7).
  Same axis, same domain, but an ITERATOR rather than an amount: a
  `ForEachOf`-side head, not a row here.
- **Two derived axes measured and not built**: the colour PAIR with a domain
  restriction (Niv-Mizzet, Guildpact — "different color pairs among permanents
  you control that are exactly two colors") and the subtype complement
  (Subgoyf, unsupported — "different subtypes other than creature types").
