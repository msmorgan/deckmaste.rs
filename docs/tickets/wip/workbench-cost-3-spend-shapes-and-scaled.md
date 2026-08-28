# cost-3: the cost-shaped spend purposes and the scaled payments

Sub-round 3 of [workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)
(the umbrella — authoritative). Runs after sub-round 1; may run parallel with
sub-round 2 (stay off `CostsToCast`/`costActionOk`/the gate tables — its
lane). BOUNDARY: workbench-mana-family-residues owns the as-though spend
permission and the mana-persistence region — this round touches
`SpendPurpose` shapes only. Owns: a COST as a thing a mana sentence can name
(the 9 spend restrictions naming a cost's contents/shape — Rosheen, Adarkar
Unicorn; the cost-arm/cast-arm disjunction; the special-action purpose); the
ZONE-QUALIFIED spend purpose (7 lines — the same third shape, never a
fourth); mana NAMED BY a cost ([CR#106.8..106.11]'s surface, 2 lines — its
own row, `ProducedRun` unwidened); the storage counters and Mana Batteries
(the ANY-NUMBER cost quantity + the for-each over what a COST removed; "an
additional" gets NO slot — 36 lines are discourse spelling); and the scaled
payment's three ledgered units taken TOGETHER (the colored scaled payment —
closes `ScaledMana` and `CostShift`'s symbol halves together; the {X}
per-unit product; the "plus an additional" compound), with the two
scaled-payment zeros pinned per acceptance.

Standard constraints apply; re-measure every count.

## As landed (2026-08-28)

Counts below are this round's re-measure over `data/derived/cards.jsonl`
filtered `select(.supported)`; where they differ from the umbrella's, the
umbrella's were wrong and the deviation is named.

### A cost as a nameable thing — landed

`SpendPurpose` gained ONE third arm, `ToPay : CostNamed -> SpendPurpose bs`,
with the object-taking `ToCast`/`ToActivate` cells untouched. The payload
`CostNamed` (`Words.idr`) has three arms and a `CostNameable` gate that reuses
`keywordCosts`, exactly as `paidCostNamed` does at the readback seat.

- **15 supported lines of 204 spend-restriction lines** carry a `ToPay` arm,
  not the umbrella's 9. Per arm (a line may write two): `Containing` 5 —
  Rosheen Meanderer, Rosheen Roaring Prophet, Nexos, Elementalist's Palette
  ({X}), Cultivator Drone ({C}); `OfKeyword` 4 — Adarkar Unicorn, Snowfall
  (cumulative upkeep), Unblinking Observer (disturb), Qarsi Deceiver (morph);
  `OfSpecialAction` 7 — Overgrown Zealot, Tin Street Gossip, Creeping Peeper,
  Qarsi Deceiver (turn face up), Niko Defies Destiny, Karfell Harbinger
  (foretell), Smoky Lounge, Creeping Peeper (unlock a door).
- **The umbrella's 9 was short by two families.** Cultivator Drone's "pay a
  cost that contains {C}" is the same `Containing` shape it credited only to
  the {X} lines, and the special-action arm is 7 lines rather than
  Overgrown Zealot alone.
- **The cost-arm/cast-arm disjunction needed nothing.** `SpendOnly` already
  takes a LIST of purposes; an arm of each kind in one list is that sentence.
- **`SpecialAction` is bounded by [CR#116.2], not by the corpus**: of its
  twelve special actions, four cost mana — turning a face-down creature face
  up [CR#116.2b], the companion [CR#116.2g], foretell [CR#116.2h] and the
  unlock cost [CR#116.2m]. The companion arm has no corpus line and is kept
  because the rule, not a count, is the bound.
- **Recorded, not landed.** Jegantha, the Wellspring (1 line) is NOT this
  type: "can't be spent to pay generic mana costs" restricts which PART of a
  cost the mana pays, and [CR#107.4b] makes a numerical symbol a component of
  a cost rather than a cost. Its own gap. Quinjet Technician and Sorcerer
  Class look like the special-action arm and are `ToActivate`'s
  ([CR#716.2c]). Unblinking Observer and Qarsi Deceiver cannot bench: the
  keyword catalog carries no Disturb and no Morph row, which is the keyword
  catalog's ledger and not this type's.
- Bench: `rosheenMeanderer`, `adarkarUnicorn`, `overgrownZealot` (whole
  cards).

### The zone-qualified spend purpose — WRONG PREMISE, no cell minted

The 7 lines write today, with no new shape and no fourth arm. Probed and
benched: `ToCast (And [spell, CastFrom (graveyardOf You)])` elaborates
against the landed cast-provenance predicate, and the two negative hand lines
are the same cell under `Not` (`predNegFree (CastFrom _) = True`), which is
what Mm'menon, the Right Hand writes positively as "from anywhere other than
your hand". [CR#601.2a] has the card on the stack before [CR#601.2h] takes
the payment, so the provenance is fixed when the restriction is tested. The
ticket's "the SAME third shape" is answered by "no shape at all".

Re-measured: 7 lines — Rootcoil Creeper, Lord of the Forsaken,
Interdimensional Web Watch, Altar of the Lost, Mm'menon the Right Hand,
Karolina Dean (negative), Vhal (negative). The umbrella's five-card positive
list was right once Mm'menon's "from anywhere other than" phrasing is caught.

Bench: `rootcoilCreeperGraveyardMana` (the ability).

### Mana named by a cost — landed, and the second line is a different gap

`ProducedMana.AsPrintedCost` takes the object whose printed cost names the
production; `ProducedRun` was NOT widened. The four rules the surface exists
for are cited where the row is: a printed cost is built from [CR#107.4]'s
symbols and four of those name no [CR#106.1b] type until the rules act
([CR#106.8..106.11]).

- 1 line lands: Elemental Resonance. Bench `elementalResonance` (whole card).
- Ice Cauldron is the family's second line and is NOT this row: its "last
  noted type and amount of mana" names a NOTE its own earlier ability took of
  mana that was SPENT, not any printed cost. Recorded; the note channel does
  not exist.
- Charmed Pendant ("for each colored mana symbol in the milled card's mana
  cost, add one mana of that color") is a third neighbour, a for-each over a
  cost's symbols; not measured into this row.

### The storage counters and the Mana Batteries — landed

Both purchases the ticket named:

- **The any-number cost quantity.** `RemoveCounters`' count is now a
  `Quantity`, not an `Amount`. 19 supported lines write "remove any number
  of" counters and 6 write "up to"; neither is a value read off the game, and
  a `Range` says both and says "a counter" too. `quantIntro` was added beside
  `quantDelta`. "Remove ALL counters" (25 lines) stays refused — a range
  names a number and "all" names whatever is there.
- **The for-each over what a cost removed.** New `OutcomeSort.CountersRemoved`
  (a quantity), exported by `effIntro`/`deedDelta` of `RemoveCounters`, read
  by a new sorted `Amount.RemovedThisWay` on `PreventedThisWay`'s model. The
  cost context reaches the effect because `activated` elaborates its effect at
  `publicOnly (costIntro cost)`, so the ACTIVATION-COST removal is what the
  production reads.
- **"An additional" got no slot**, as pinned — and the count is 16 lines, not
  36. All 16 are the second add of a chain or a production off another one, so
  the word marks discourse and the structure is the sequence.
- The family is **18 lines over 18 cards** (5 Mana Batteries, 11 storage
  lands, Hierophant Bio-Titan, Eventide's Shadow), not 36 lines over 18 cards.
- `CounterKind.Wind` was added (ordinary marker, [CR#122.1]) for the colored
  scaled bench. `Storage` was not: Black Mana Battery's `Charge` carries the
  family, and the 11 storage lands are recorded as waiting on that one data
  row.
- Bench: `blackManaBattery` (whole card) — an any-number `Charge` removal in
  the activation cost and `RemovedThisWay` in the production.

### The scaled payment's three units — two landed, one was not a gap

- **The COLORED scaled payment (3 lines).** New `ManaUnit`
  (`GenericUnit`/`RunUnit`) in `Words.idr`; `ScaledMana` now takes it.
  Cyclone {G}, Thelon's Curse {U}, Norn's Annex {W/P} — re-measured 3.
  Bench: `cycloneUpkeepPayment`.
- **The {X} PER-UNIT (4 lines).** New `Amount.TimesOf`, beside `Times` and not
  a widening of it: the numeral arm's `IsSucc` gate is the whole of what it
  buys, and a read per-unit states no size to gate. `forEachAmount` admits
  both. Collective Restraint, Sphere of Safety, War Cadence, War Tax —
  re-measured 4, all four attack/block gates, so the bench is the cost
  fragment `warTaxScaledPayment` and not a card.
- **The "PLUS AN ADDITIONAL" compound (3 lines) — WRONG PREMISE, nothing
  minted.** Probed: `Compound [Mana [{2}], ScaledMana GenericUnit (Times 2 …)]`
  elaborates today. A fixed base beside a scaled one IS the compound; "plus an
  additional" is the coordinator's WORDS, and spelling is english_v2's
  (`rendering-is-not-the-workbench`). Rune Snag, Spell Stutter, Concerted
  Defense — re-measured 3. Bench: `runeSnag` (whole card).

### The two measured zeros — recorded, not pinned

Per the ticket's fence and `measurements-live-in-pins`: a pin refuses
rules-impossibility only, and neither zero is refused by a rule. Both are
recorded in `ScaledMana`'s docstring.

- **No scaled ALTERNATIVE cost.** [CR#118.9] would allow one; the row reaches
  `AltCost` structurally. Liesa, Shroud of Dusk is the near miss and not the
  case: it DECLINES a scaled cost and substitutes a repeated life payment,
  which is the repetition channel's.
- **No scaled non-mana ADDITIONAL cost.** "as an additional cost … for each"
  returns nothing, and [CR#118.8] would allow it.

This deviates from the umbrella's acceptance line ("with a pin apiece rather
than a silence"), which the sub-ticket's own fence overrides.

### Deviation: `CostShift`'s symbol half

The ticket asked the colored scaled payment to close `ScaledMana`'s and
`CostShift`'s symbol halves TOGETHER. What landed: the shared vocabulary
(`ManaUnit`) is minted once and documented as the unit BOTH ledgers take, and
it is wired to `ScaledMana`. `CostShift`'s constructors were left untouched.
Reason: `CostShift`'s only consumer is `CostsToCast`, which the sub-round
boundary bans, its ~25 coloured-payload lines are the umbrella's
cost-statement section rather than this one's, and a positional slot would
have rewritten 41 `CostLess`/`CostMore` sites in `Cards.idr` — the exact lines
the parallel round edits. The half is one constructor slot away and takes the
landed type.

### Remainders

- `CostShift`'s coloured payload (~25 lines) — the sibling ledger, above.
- Disturb and Morph `keywordFacts` rows — Unblinking Observer and Qarsi
  Deceiver bench the moment they exist.
- `CounterKind.Storage` — the 11 storage lands.
- Jegantha's symbol-class spend restriction (1 line) — a restriction on a
  cost's PART, [CR#107.4b].
- Ice Cauldron's noted-mana readback (1 line) — no note channel.
- "Remove ALL counters" (25 lines) — `Quantity` names a number, not a
  totality.
- Cyclone's second sentence, Thelon's Curse's untap rider, Norn's Annex and
  the other three {X}-per-unit carriers — all wait on gaps outside this round
  (the payment readback, the defending-player noun).

### Gates

`idris/scripts/build` 23/23, EXIT=0, from a wiped `build/ttc`.
`cargo xtask cite check` 0 stale; `cite check --list-noncompliant` 0.
`cite bless` registered [CR#702.37e] and [CR#716.2c], both read against their
citing claims; `jj diff --git | cargo xtask cite audit --diff` audited every
site and moved one cite ([CR#601.2g] -> [CR#601.2h], the rule that actually
takes the payment). `Experimental/Cards.idr` binds no implicits in the new
benches; no `{default`.
