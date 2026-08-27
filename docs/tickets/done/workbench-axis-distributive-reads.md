---
needs: []
---
# The distributive axis read and the two derived axes

Routed from `workbench-distinct-kind-count` (close, 2026-08-26), which landed
`DistinctCount`/`KindAxis`. Its measured remainders:

1. **The distributive twin** — "for each basic land type among lands you
   control" (33 supported), "for each color among …" (11), ~52 lines total:
   same `KindAxis`, same `Noun` domain, but a `ForEachOf`-side iteration
   head, not an `Amount` row. Study `ForEachOf`'s element binder and the
   landed `DistinctCount` before minting; the axis vocabulary must be
   SHARED, not duplicated.
2. **Two derived axes, measured and not built:** the colour pair with a
   domain restriction (Niv-Mizzet, Guildpact) and the subtype complement
   (Subgoyf; unsupported corpus line). Land each or name it at its measured
   size with the rule that bounds it.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`KindAxis`), `Phrase.idr`, `Effect.idr`
(the iteration head), `Cards.idr` bench, `Proofs*.idr`. No Rust crate.

## Acceptance

- The distributive read lands over the shared axis vocabulary or ends in a
  written rule-backed verdict; witnesses benched per axis arm exercised.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

One `Effect` head, one `KindAxis` arm, one `Predicate`, four benches, four
pins; five files, +199 / −0. `idris/scripts/build` PASS (23/23, 0 errors, 0
warnings). No design artifact: the docstrings plus this section are the
record.

### The measurement that decided the shape

"For each [axis] among [domain]" is written **62 times in supported
text** (a coincidence of count with the `DistinctCount` round's amount
reading, which is a different family of words). Split by what the words
are DOING:

- **55 are scaling multipliers**, not iteration at all: "draw a card for
  each basic land type among lands you control" (28+2), "gets +1/+1 for
  each color among permanents you control", "this spell costs {1} less to
  cast for each card type among cards in your graveyard". These are
  `Times per (DistinctCount ax dom)` in an amount slot and were already
  expressible when `DistinctCount` landed. The ticket's "~52 lines needing
  a `ForEachOf`-side head" was a mis-read of the family.
- **7 are true distributive passes** — sentence-initial "For each [axis]
  among [domain], [body]" — and **all 7 read the axis value back in the
  body**: "add one mana of **that color**" (Bloom Tender, Faeburrow Elder,
  Tarnation Vista, Sunbird Effigy), "has landwalk **of that type**"
  (Magnigoth Treefolk), "put a card **of that type**" (Hurkyl, Master
  Wizard), "choose a creature **with that power**" (Celestial Judgment).

At 7 of 7 the bridge is settled: `Repeated (DistinctCount ax dom) body`
exports the count and the body's own delta and never a value on the axis,
so it cannot spell one of these lines. The pass must BIND the value.

### The design: a sibling row that binds, over the shared vocabulary

- **`ForEachKindOf ax dom q body`** (`Effect.idr`), beside `ForEachOf`,
  with the nine companion rows (`heldUntilOk`, `reflexEncloseUse`,
  `thisWayOutcomeOk`, `costActionOk`, `effEq`, `effIntro`, `preIntro`,
  `annIntro`, `deedDelta`). Same `KindAxis`, same one-slot `Noun` domain
  as `DistinctCount` — the axis vocabulary is shared, not duplicated — and
  ungated on plurality for `DistinctCount`'s reason [CR#205.2b,105.2].
- **The value is bound as a QUALITY, so no read is minted.** The body's
  context is `kindValueIntro q dom` = `qualityB q :: nomIntro dom`
  (`Phrase.idr`, beside `elemIntro`), which is exactly what
  `OfChosen`/`OfChosenColor` already take. Bloom Tender's "add one mana of
  that color" therefore benches with zero new read machinery — the
  strongest evidence the shape is the house's.
- **The sort rides beside the axis, gated to agree**
  (`kindAxisSort ax = Just q`), mirroring `Aggregate`'s
  `projScope ax = k`. Keeping the value under the existing
  `Quality : QualitySort -> Kind` is what keeps the cost at nine tables:
  the `Kind` semilattice's tables are n²/n³ (`Eq Kind`, `kindLte`,
  `kindLteTrans`), and a new `Kind` arm would have paid all of them.
- **`kindAxisSort`** (`Words.idr`, beside `KindAxis`) is total and returns
  `Maybe`. `ColorAxis → Color`, `SubtypeAxis Creature _ → CreatureType`,
  `ValueAxis _ → Number`; `Nothing` for card type, permanent type, the
  non-creature subtype hosts, counter kind and the pair axis. **`Nothing`
  is a vocabulary gap, never a refusal** — [CR#205.2a]'s card types,
  [CR#205.3e]'s subtypes, [CR#105.5]'s pairs and [CR#122.1]'s counter kinds
  are all nameable, and the corpus writes a CHOICE over three of them
  ("choose a card type" 10, "choose a land type" 5 / "choose a basic land
  type" 8, "choose a kind of counter" 2). `QualitySort` simply does not
  carry them yet; see the ledger for why this round did not mint them.
- **`ColorPairAxis`** on `KindAxis` [CR#105.5] — its own arm and not
  `ColorAxis` under a restriction, because its values are PAIRS: a
  white-and-blue permanent supplies one value here and two on `ColorAxis`.
- **`ExactlyColors n`** (`Phrase.idr`) with `colorCountOk` floors it at
  two and ceilings it at five: [CR#105.2a]'s "monocolored" and
  [CR#105.2c]'s "colorless" are the printed words below two and
  [CR#105.1] closes the colours at five. 9 supported lines ("exactly two
  colors" 5, "exactly three colors" 4).

### Bench (`Cards.idr`), oracle text verified against local card data

`bloomTenderMana` (the acceptance line, and verbatim Faeburrow Elder's
second), `tarnationVistaMana` (the pass's domain narrowed by a predicate),
`faeburrowElderPump` (the SCALING twin of the same words on the same card,
`Times` over `DistinctCount` — the 55-line reading benched beside the
7-line one), `nivMizzetGuildpactTrigger` (the pair axis under its
domain restriction). The `ColorAxis` arm — 4 of the 7 distributive lines —
is the arm exercised; every other arm's line is blocked outside this head
(ledger).

Pins (`ProofsG.idr`): `badRepeatedCarriesNoColor` (the round's argument
made a pin: the counted bridge leaves no colour for "of that color" to
read), `badAxisValueCrossing` ([CR#105.1] colours are not [CR#205.3e]
subtypes, so the sort cannot cross its axis), `badExactlyOneColor`
[CR#105.2a], `badExactlySixColors` [CR#105.1].

### Item 2 — the two derived axes

- **The colour pair with a domain restriction: LANDED.** Niv-Mizzet,
  Guildpact benches whole. `ColorPairAxis` [CR#105.5] plus `ExactlyColors`
  for "that are exactly two colors"; the restriction is a predicate on the
  domain `Noun`, which is where every restriction on these lines lives.
- **The subtype complement: NOT BUILT, and the verdict is measured, not
  deferred.** Subgoyf's "the number of different subtypes other than
  creature types among cards in all graveyards" is its ONLY corpus line
  and that face is **unsupported**, so the complement measures **zero
  supported lines**. Unsupported text never motivates structure — the same
  posture `SubtypeScope`'s `NonbasicOnly` already holds for Mojave Desert.

## Ledger

- **Magnigoth Treefolk** — "For each basic land type among lands you
  control, this creature has landwalk of that type." Blocked twice: the
  grammar has no landwalk at all, and `kindAxisSort (SubtypeAxis Land _)`
  is a gap.
- **Hurkyl, Master Wizard** — "For each card type among noncreature
  spells you've cast this turn, you may put a card of that type from among
  the revealed cards into your hand." Needs a card-type quality sort, plus
  "from among the revealed cards" and "put the rest on the bottom … in a
  random order".
- **Celestial Judgment** — "For each different power among creatures on
  the battlefield, choose a creature with that power." The head spells the
  pass (`ValueAxis Power → Number`), but the body's read is refused by the
  standing rule-backed `chosenQualityReadOk Number = False`: the
  numeric-characteristic read is the separate node that gate's docstring
  already names as unminted.
- **Niv-Mizzet Reborn — the DOMAINLESS distributive pass.** "For each
  color pair, choose a card that's exactly those colors from among them."
  It runs over all ten of [CR#105.5]'s pairs, not over the values present
  in some group, so `ForEachKindOf`'s mandatory domain slot does not
  reach it. Also needs the pair-valued read ("exactly those colors") and
  "from among them" / "the rest … in a random order".
- **Sunbird Effigy** — "{T}: For each color among the exiled cards used to
  craft this creature, add one mana of that color." The head fits; the
  domain needs craft machinery.
- **`QualitySort`'s three gaps** — a card type, a subtype at a host other
  than the creature type, a kind of counter. Each is motivated by the
  corpus's own CHOICE vocabulary independently of this head. Not minted
  here because the honest shape for the subtype one is
  `SubtypeQ : CardType -> QualitySort`, mirroring `SubtypeAxis`, and
  `sameQEq` would then need a `CardType` equality-reflection lemma —
  15 × 15 clauses — to serve zero benchable lines.
- **`ExactlyColors 3`** — "exactly three colors", 4 supported lines, no
  witness benched this round.
