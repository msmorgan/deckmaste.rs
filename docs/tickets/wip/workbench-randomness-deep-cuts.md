---
needs: []
---
# Randomness deep cuts: die kinds, chosen ignores, the flip instruction event

Routed from `workbench-randomness-tail` (close, 2026-08-26), the third
randomness round. Its supported unowned remainders:

1. **A die-kind slot on `RollsDice`** — 4 lines want the watched roll
   narrowed by die kind (the planar-header narrowing verdict from that
   round's item 2 rides this); completes none alone.
2. **The counted-and-chosen ignore arm** — "ignore any one of those results"
   style (3 lines): `IgnoreRolls` covers extremes (`IgnoreExtreme`/
   `IgnoreAllBut`), not a chooser-selected result.
3. **`RollPlanarDie`'s missing count** — the instruction takes no count;
   printed lines write one.
4. **The flip INSTRUCTION event** — `FlipEvent` is only [CR#705.2]'s called
   flip, so no replacement can reach the flipping act itself; Krark's Thumb
   ("If you would flip a coin, instead flip two coins and ignore one") is
   the carrier — verify its supported status and exact text from local data
   before building.

Named at zero in the parent close, not here: "Whenever chaos ensues" (172
lines, all on Plane cards, none supported — scope), the blank-face read,
exchange-a-result-with-base-P/T (unsupported carrier). Parent fence carries:
watch/read vocabulary only, no randomness engine.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`, `Effect.idr`,
`Words.idr`; `Cards.idr` bench; `Proofs*.idr`. No Rust crate.

## Acceptance

- Each item lands or ends in a written rule-backed verdict; no silent drops.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` 23/23 PASS on a clean
rebuild (`rm -rf build` first), 0 errors and 0 warnings. **Two pins added**,
none retired, none silently passing. Every count below was re-measured this
round against the local card corpus (supported scope, `all=` where quoted),
every witness's oracle text re-read from `data/derived/cards.jsonl`, and every
rule re-read in `data/rules/cr.txt`. No execution semantics landed: nothing
resolves a flip, rolls a die, or models a randomness source.

**All four items landed.** The parent's four blocked replacement lines all
bench as a consequence: with these four pieces in place, every one of the 6
supported randomness replacement lines now has a positive — 5 whole, and
Vedalken Squirrel-Whacker's benched to the clause its exchange blocks.

### Re-measured surface

| Ticket said | Measured | What it counts |
|---|---|---|
| 4 die-kind lines | **2** supported + **5** all-scope | the die a roll EVENT names: Ichor Elixir ("one or more planar dice") and Vedalken Squirrel-Whacker ("one or more six-sided dice"); the unsupported half is 4 "Whenever you roll the planar die" lines and 1 blank-face line, 2 header shapes, every one a Plane card |
| 3 chosen-ignore lines | **3** (all=7) | "ignore one" / "you choose one of those rolls to ignore" — Ichor Elixir, Krark's Thumb, Bamboozling Beeble; 1 of them a flip, as the ticket said |
| 1 planar count | **1** | "instead roll that many planar dice plus one" (Ichor Elixir); "Roll the planar die." is 1 supported line more (all=6) |
| Krark's Thumb, status unverified | **supported**, text confirmed | "If you would flip a coin, instead flip two coins and ignore one." The carrier exists, so item 4 mints rather than closing at zero |
| — | **6** (7 cards) | supported randomness replacement lines, the parent round's denominator — all 6 now positive |

### 1 — The die-kind slot: landed, and the planar headers with it

**`data RolledDie = AnyDie | SidedDie (n : Nat) {IsSucc n} | PlanarDie`**
(Events.idr, beside `DiceBatch`), a new third slot on
**`RollsDice : (who : Noun bs Player) -> (many : DiceBatch) -> (die :
RolledDie) -> (res : RollWatch bs) -> {auto 0 dw : So (watchFitsDie die
res)} -> GameEvent bs`**.

Spelling-only and Bindings-free, like `DiceBatch`: a header writes the kind
outright or not at all, and the anaphoric kind `DieSides` carries is the
replacement BODY's word, never the watched event's.

- **`AnyDie` is the kind left UNWRITTEN, not "some numbered die"** — which is
  the ticket's warning discharged. [CR#706.7] and [CR#901.9d] both say that
  rolling the planar die causes any ability that triggers whenever a player
  rolls one or more dice to trigger, so the unnarrowed header already watches
  the planar roll and narrowing is only what a WRITTEN kind does. Nothing
  about the new slot excludes a planar roll from `youRollDice`.
- **`PlanarDie` is a row beside `SidedDie` and never `SidedDie 6`**, even
  though [CR#901.3a] calls the planar die six-sided: [CR#706.1a] fixes a
  written kind as N equally likely outcomes NUMBERED from 1 to N, and the
  planar faces carry no numbers.
- **The planar arm takes no result test and announces none.** `watchFitsDie`
  refuses `ResultIn`/`HighestNatural` under `PlanarDie` ([CR#706.7]'s second
  sentence names the comparison of a result to a given number outright;
  [CR#901.9d] repeats it), and `eventAfter` mints `PlanarRolled` rather than
  `RollResult` there. `eventIntro` is unchanged: what the plural determiner
  announces is the COUNT of dice the instruction called for, which
  [CR#706.7] does not withhold, and it is exactly what "that many planar
  dice" reads.
- **The parent round's planar-header verdict resolves.** "Whenever you roll
  the planar die" is now writable outright as `RollsDice who OneDie PlanarDie
  AnyResult`; it is not benched because its whole printed surface is Plane
  cards outside supported scope. The deferral is scope alone now, not a
  missing slot. The blank-face line needs one thing more — a face-valued
  `RollWatch` arm [CR#901.9a] — and stays ledgered.

**Benched**: `ichorElixirPlanarDice` (whole) and
`vedalkenSquirrelWhackerReroll` (to its blocker), giving `PlanarDie` and
`SidedDie` a live positive each.

### 2 — The counted-and-chosen ignore: landed, over all three randomisers

`IgnoredRolls` becomes **`IgnoredOutcomes : Bindings -> Type`** (moved to
Phrase.idr beside `FlipScope`, which it now needs `Amount` and `Noun` for),
and `IgnoreRolls` becomes **`IgnoreOutcomes : (which : IgnoredOutcomes bs) ->
{auto 0 ok : So (ignorableFor which)} -> Effect bs`**. The third arm:

    IgnoreChosen : (chooser : Maybe (Noun bs Player)) -> (n : Amount bs) ->
                   {auto 0 ag : EventAgent chooser} -> IgnoredOutcomes bs

The rename is forced, not cosmetic: one of the three printed lines ignores a
COIN. [CR#706.6] defines the word over a roll and no rule makes it
meaningless over the other two randomisers this vocabulary has, so doctrine
admits them and the row's name has to stop saying "rolls".

- **The gate is asked per arm** (`ignorableFor`), not once for the row.
  `IgnoreChosen` takes any of the three — `ignorableInScope` is
  `countOutcomes RollResult bs == 1 || coinFlipInScope bs ||
  planarRollInScope bs`, the roll's own count-of-one beside two existence
  tests, since neither a coin nor a planar die leaves a number two mentions
  could confuse. The two superlative arms keep the roll's test alone:
  `RollExtreme` names the ends of an ORDER, [CR#706.2] makes a result a
  number, and [CR#705.1]'s two faces and [CR#901.3a]'s six are ranked by
  nothing.
- **The chooser is written only on the chosen arm.** [CR#706.6] hands the
  choice to the instructed player of its own accord, as the tie-break under
  "the lowest roll", so the superlative arms need no slot. The chosen arm
  gets one because a printed line puts the choice somewhere else: Bamboozling
  Beeble replaces TARGET player's roll and then has YOU choose. Bindingless
  on `EitherEnd`'s model — the chooser names a player the sentence already
  has and mints none, which is why the count needs no chaining through it.
- The count is an `Amount` on `FlipCount`'s model. [CR#706.6] states the word
  one roll at a time and bounds nothing; every printed line writes the
  literal one, so "ignore two" is overgeneration, named here at its zero.

**Benched**: all three lines — `ichorElixirPlanarDice`,
`krarksThumbExtraFlip`, `bamboozlingBeebleIgnore`.

### 3 — `RollPlanarDie`'s count: landed

**`RollPlanarDie : (who : Noun bs Player) -> (count : Amount (nomIntro who))
-> Effect bs`**, on `RollDice`'s model. A bare "Roll the planar die." is
`Lit 1`, as a bare flip is one coin.

The count is the whole of what the row takes from [CR#706.1]: there is no
kind slot beside it, because [CR#901.3a]'s die is not the numbered one
[CR#706.1a] describes and naming it IS naming the row — the ledger's "gated
apart from [CR#706.1a]'s side count", answered by the row's existence rather
than by a gate.

It now announces the roll, as a new `OutcomeSort` row **`PlanarRolled`**
whose `outcomeIsQuantity` is False. That is what lets "ignore one" name the
planar dice while `TheResult`, `TheTotal`, `ResultsTable` and `ThatMuch` stay
inapplicable to a planar roll by [CR#706.7] — the mention is there and
carries no value.

**Benched**: `fracturedPowerstonePlanarRoll` rewritten over the new slot
(`Lit 1`), and `ichorElixirPlanarDice` for the anaphoric count.

### 4 — The flip instruction event: landed; Krark's Thumb is supported

Verified first, as the ticket asked: **Krark's Thumb is `supported: true`** in
`data/derived/cards.jsonl`, text "If you would flip a coin, instead flip two
coins and ignore one." The item mints rather than closing at zero.

**`FlipsCoin : (who : Noun bs Player) -> GameEvent bs`**, over a new
`EventName` row **`CoinFlip`**. [CR#705.1] makes flipping a coin what an
effect instructs, and [CR#705.2] reads a winner off that flip only afterwards
and only where the flipper called it — so the act happens under both readings
of the coin, and it is the act a replacement reaches [CR#614.1], which
neither arm of `FlipEvent` can stand in for. The subject is a player for
[CR#705.2]'s reason, `FlipEvent`'s own.

- **It announces its subject and no coin.** [CR#614.6] keeps the replaced
  flip from happening at all, so no coin stands there to read, and the
  replacement writes its own count out ("instead flip two coins"). This is
  where it differs from `RollsDice`, whose plural determiner announces the
  dice because the body reads "that many".
- **`eventAfter` leaves the coin** (`CoinFlipped`), which following text
  reads by face or by call [CR#705.2].
- **No `DiceBatch` twin.** The corpus prints the singular determiner alone
  where the roll prints both, so a batch slot here would name nothing said.
- Table rows over the new name: `eventHasMagnitude` False ([CR#705.1] makes a
  coin a two-sided randomiser, so how many were flipped is a count and the
  flip happens in no amount); `lookbackSubjectOk` Player-only ([CR#705.2]
  gives the flip to "the player who flips the coin"); no complement.
  `interceptOk`, `spanEventOk`, `bareLookbackOk` keep their catch-alls — no
  rule refuses the act there.

**Benched**: `krarksThumbExtraFlip`, whole.

### Pins

Two, both in `ProofsG`, each naming the rule that makes its term meaningless,
neither justified by a count, both with a live positive on the bench.

| Pin | Refuses | Rule |
|---|---|---|
| `badPlanarResultTest` | "Whenever you roll a 4 or higher on the planar die" | [CR#706.7,901.9d] — every effect referring to a numerical result of a die roll, the comparison to a given number named outright, ignores the planar roll; [CR#901.3a] numbers none of its faces |
| `badExtremeOverFlips` | "instead flip two coins and ignore the lower one" | [CR#706.6] writes the superlative over rolls, whose results are numbers [CR#706.2]; [CR#705.1] gives a coin two faces and ranks neither |

Positives keeping them honest: `ichorElixirPlanarDice` (the same narrowing
where no test stands) and `atomwheelAcrobatsRoll` (the same test over a
numbered die) for the first; `krarksThumbExtraFlip` (the ignore that IS
printed over flips) and `berserkersFrenzyRoll` (the superlative where results
stand) for the second. `badIgnoreWithoutRoll` survives the gate change
unchanged in meaning — its proof is now `Oh` rather than `Refl`.

### Bench

| Witness | State |
|---|---|
| **Ichor Elixir**, first line | **benches whole**, as `ichorElixirPlanarDice` — items 1, 2 and 3 in one sentence |
| **Krark's Thumb** | **benches whole**, as `krarksThumbExtraFlip` |
| **Bamboozling Beeble**, second line | **benches whole**, as `bamboozlingBeebleIgnore` — the "next time … this turn" replacement over target player's roll, "they" reading the roller back, and the written chooser |
| **Vedalken Squirrel-Whacker**, second line | benches to "instead roll them", as `vedalkenSquirrelWhackerReroll`; the exchange with a base characteristic is not written — see ledger |
| **Fractured Powerstone**, second line | rewritten over the new count slot; unchanged in what it says |
| "Whenever you roll the planar die" (4 lines), "a blank on the planar die" (1) | writable / still short one arm; not benched — every line is a Plane card outside supported scope |

### Ledger

| Item | State after this round | Exact missing piece |
|---|---|---|
| The die the roll event NAMES (4 lines) | **landed** | — |
| A counted-and-chosen ignore (3 lines) | **landed** | — |
| The planar roll's count (1 line) | **landed** | — |
| A flip INSTRUCTION as an event (1 line) | **landed** | — |
| The planar die's blank-face read (1 line, all-scope) | blocked, **owned by nobody** | a face-valued `RollWatch` arm over [CR#901.3a]'s three faces; the die-kind half of the old entry has landed |
| "Whenever you roll the planar die" (4 lines, all-scope) | **writable**, unbenched | nothing structural — every line is a Plane card, so it is a plane-card round's scope call, not a grammar gap |
| Exchanging a roll's result with a base characteristic (1 line) | blocked, **owned by nobody** | Vedalken Squirrel-Whacker's "you may exchange one result with this creature's base power or base toughness" — an exchange between a roll's result and a settable characteristic — [CR#706.7] names the shape ("ones that exchange the results of that roll with another value") and refuses nothing |
| "Whenever chaos ensues" (172 lines, all-scope, all Plane cards) | unchanged | an `EventName` row and a `GameEvent` row [CR#311.7]; the deferral is scope, a plane-card round's |
| "After you roll a die, …" (2 lines) | unchanged | a fourth trigger word; [CR#603.1] writes the ability as "[When/Whenever/At]" |
| Noted and stored results, the difference between two results, plural/target choosers | unchanged | their own tickets |
