---
needs: []
---
# Randomness tail: roll replacements' body, Planechase events, per-object flips

Routed from `workbench-randomness-residues` (close, 2026-08-26), which landed
the instruction-side ignores (`IgnoreRolls`/`ShiftResult`), `RollPlanarDie`,
result-conditioned headers, noted results, roll-set reads, and `CoinCameUp`.
Five measured remainders:

1. **The replacement side of roll modifiers/ignores** — `Intercepts
   (RollsDice …)` is writable but its body is not: `eventIntro` announces
   neither die count nor kind, so "instead roll that many dice plus one"
   has no antecedent and `RollDice`'s `sides : Nat` has no anaphoric arm.
   Never pinnable [CR#706.2,706.6].
2. **Planechase events + the "chaos ensues" instruction** — the planar die's
   faces get their meaning from [CR#901.9a..901.9c] as EVENTS (needing
   `EventName` rows); "chaos ensues" is a printed instruction [CR#311.7]
   (2 supported lines).
3. **"a die's highest natural result" test** (per [CR#706.2]'s natural
   result) — one measured line.
4. **"your third die each turn"** — untried; possibly composes from
   `NthOccurrence` + `RollsDice` already; probe before minting.
5. **The per-object coin flip** — "flip a coin for each creature" (Rakdos,
   the Showstopper): `FlipCoins` takes a player subject only.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`, `Phrase.idr`,
`Effect.idr`; `Cards.idr` bench; `Proofs*.idr`. No Rust crate. The parent
fence carries: watch/read vocabulary only, no randomness engine.

## Acceptance

- Each item lands or ends in a written rule-backed verdict; no silent drops.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` 23/23 PASS on a clean
rebuild (`rm -rf build` first), 0 errors and 0 warnings. **One pin added**,
none retired, none silently passing. Every count below was re-measured this
round against the local card corpus (supported scope, `all=` where quoted) and
every witness's oracle text re-fetched with `card`; every rule was re-read in
`data/rules/`. No execution semantics landed: nothing resolves a flip, rolls a
die, or models a randomness source.

Four of the five items landed; item 2 landed its instruction half and closes
its event half in the verdict below, which corrects a premise the parent round
left standing.

### Re-measured surface

| Ticket said | Measured | What it counts |
|---|---|---|
| 8 replacement lines | **6** (all=12) | randomness replacements over 7 cards — 5 "would roll", 1 "would flip" |
| 2 chaos instruction lines | **2** (6 cards) | "chaos ensues" as a printed instruction |
| — | **172** all-scope, **0** supported | "Whenever chaos ensues" — every one of them a Plane card |
| — | **6** all-scope, **0** supported | "Whenever you roll the planar die" / "a blank on the planar die" — 5 Plane cards and 1 unsupported sorcery |
| 1 | **1** | "a die's highest natural result" — Netherese Puzzle-Ward, confirmed |
| 1 | **1** | "your third die each turn" — Resolute Veggiesaur, confirmed |
| 1 | **2** | per-X flips — Rakdos, the Showstopper (object) and **Warp Vortex** (player), both supported |

### 1 — The replacement side: the two named slots landed

Both slots the ticket named are one mention, because [CR#706.1] has a rolling
instruction specify the kind of die and how many together.

- **`RollsDice`'s plural arm now announces the dice the roll called for.**
  `eventIntro (RollsDice who ManyDice _) = outcomeB DiceRolled :: nomIntro who`,
  over a new `OutcomeSort` row **`DiceRolled`** (`outcomeIsQuantity` True).
  On `CounterEvent`'s model: the singular determiner announces nothing, since
  "that many" needs a number the text wrote and "a die" wrote none. It is
  `eventIntro` and not `eventAfter` because [CR#614.6] keeps the replaced
  event from happening at all, so what stands there is the dice the
  instruction called for; `eventAfter` still announces the RESULT alone
  [CR#706.2], which is the one number a trigger's tail reads back.
- **`RollDice`'s `sides` is now `DieSides (amtIntro count)`**, over
  **`data DieSides : Bindings -> Type`** with `SidesOf : (n : Nat) ->
  {auto 0 nz : IsSucc n} -> DieSides bs` (carrying [CR#706.1a]'s positivity in
  the constructor, on `Ordinal`'s model) and `ThoseDice : {auto 0 ok :
  countOutcomes DiceRolled bs = 1} -> DieSides bs`. The anaphoric arm names no
  kind of its own and takes the one an announced roll carried — which is what
  [CR#706.3c] does in the rules' own voice, defining "Roll again" as using
  "the same kind of and number of dice originally called for".

**Benched**: `wyllExtraDie` — "If you would roll one or more dice, instead
roll that many dice plus one and ignore the lowest roll", whole
(`ifWouldInstead` over `youRollDice`, `RollDice You (Plus ThatMuch (Lit 1))
ThoseDice`, then the `IgnoreRolls` that landed last round). Barbarian Class's
level-1 line is the same sentence and Pixie Guide's is it under an ability
word, so 2 of the 6 measured lines and 3 of the 7 cards land here.

The four that do not are ledgered with exact pieces; none is refused and the
family stays never-pinnable [CR#706.2,706.6].

### 2 — Planechase: the instruction landed, the events close

- **`ChaosEnsues : Effect bs`** — no subject and no slot. [CR#311.7] admits it
  outright: a chaos ability triggers "if the chaos symbol is rolled on the
  planar die …, if a resolving spell or ability says that chaos ensues, or if
  a resolving spell or ability states that chaos ensues for a particular
  object". The middle clause is the one the corpus writes, and it gives the
  instruction no participant. It mints nothing — what ensues is a trigger on
  the plane card, not a phrase the sentence goes on to read. The
  object-scoped third clause is written by no supported line and is not a
  slot here. Benched as `missyChaosBranch`, Missy's second branch reduced to
  the conjunct it writes ("you draw a card and chaos ensues"); the villainous
  choice framing both branches is its own family, as is the vote the five
  `Path of the …` cards write around the same instruction.
- **The parent round's premise for the two planar-die headers does not
  survive.** It read that "Whenever you roll the planar die" and "Whenever you
  roll a blank on the planar die" "need `EventName` rows … of their own".
  They need no new event: [CR#706.7] and [CR#901.9d] make a planar roll
  trigger "any ability that triggers whenever a player rolls one or more
  dice", so those headers are `RollsDice` NARROWED by the die the roll
  named — the same missing slot four other lines want (see the ledger) —
  and the blank-face one adds a face-valued watch beside it. Nothing is
  landed for them: their whole printed surface is 5 Plane cards, none in
  supported scope, and the narrowing slot has no supported witness that its
  arrival alone would complete.
- **"Whenever chaos ensues" stays out, ledgered rather than refused.**
  [CR#311.7] names it as a thing that happens, so it is rules-meaningful and
  no pin may refuse it; it would be a real `EventName` row. Its entire
  printed surface is 172 lines and every one of them is on a Plane card, none
  in supported scope. `CardType` does carry `Plane`, so the grammar does not
  exclude these cards structurally — the deferral is scope, and it belongs to
  a plane-card round, not to this one.

### 3 — The die's highest natural result: landed

`RollsDice`'s test slot is now a data type of its own: **`data RollWatch :
Bindings -> Type`** with `AnyResult`, `ResultIn : (q : Quantity bs) -> … ->
RollWatch bs` (carrying `RollRow`'s three gates as before) and
**`HighestNatural`**, replacing the `res : Maybe (Quantity bs)` slot and its
erased `RollTest` companion. The fold was forced, not cosmetic: a third arm
under the old shape would have indexed `Nothing` alongside `AnyResult`, so two
distinct events would have differed only in an erased 0-multiplicity gate.

`HighestNatural` is the one test no literal range spells. [CR#706.2] takes the
natural result as the number on the top face BEFORE any modifier, so a
modified 20 is a result of 20 and not the natural one; and [CR#706.1a] numbers
each die from 1 to its own N, so the header's number differs with every die it
may watch. Benched as `netheresePuzzleWardIllumination`; the ability word is
not written.

### 4 — "Your third die each turn": composes, nothing minted

Probed before minting, as the ticket asked, and it already composes:
`NthOccurrence (Nth 3)` over `youRollADie` under `triggeredOnlyDuring`'s
window. Benched as `resoluteVeggiesaurThirdDie`. "Each turn" benches as the
every-player window (`DuringWindow Turn (Just EachPlayers)`), which is the
same span in the spelling `TriggerWindow` carries; Wavebreak Hippocamp's
"during each opponent's turn" is the same machinery with the other quantifier.
**No vocabulary was added for this item.**

### 5 — The per-object coin flip: landed, by widening the count slot

`FlipCoins`' count slot is now **`FlipScope (nomIntro who)`**:

    data FlipScope : Bindings -> Type where
      FlipCount : (n : Amount bs) -> FlipScope bs
      FlipPer : {k : Kind} -> (each : Noun bs k) ->
                {auto 0 pl : nounPlur each = ManyOf} ->
                {auto 0 rk : So (kindLte k (Object \/ Player))} -> FlipScope bs

with `flipScopeIntro` feeding `effIntro`/`preIntro`/`annIntro`.

The described set is a second slot on the instruction and **never a widened
subject**, which is the whole answer to the ticket's question. [CR#705.1]
leaves a coin a two-sided physical randomiser belonging to no referent, and
[CR#705.2] gives the flip to "the player who flips the coin" and to no one
else — so a flip made FOR a creature is still flipped BY the clause's subject.
Warp Vortex proves the distinction is load-bearing on the card: it flips "for
each opponent you have" and then reads "for each flip YOU win". Composition
under `ForEachOf` was probed and is not the shape: `effIntro (ForEachOf _ _) =
bs`, so the flip never escapes the loop and the following sentence's
`CoinCameUp` finds no coin — exactly the failure the parent round predicted.

Kind-polymorphic under the coarse join `CoinCameUp` already uses, because both
kinds are printed. The plurality gate is a canonicity gate and not a rules
refusal: one member is `FlipCount (Lit 1)` written out.

**Benched**: `rakdosShowstopperFlips` (Rakdos, the Showstopper's trigger body,
both sentences, the second narrowing by `CoinCameUp Tails`) and
`warpVortexFlips` (Warp Vortex's first sentence alone). Both supported lines
of the family now have a positive.

### Pins

One added, in `ProofsG`, naming the rule that makes its term meaningless, not
justified by a count, and with a live positive on the bench.

| Pin | Refuses | Rule |
|---|---|---|
| `badAnaphoricSidesWithoutRoll` | "Roll that many dice." with no roll announced | [CR#706.1] — a rolling instruction specifies what kind of die to roll, and the anaphoric arm specifies none of its own |

Its positive is `wyllExtraDie`, the same word written where the announcement
stands. `FlipPer`'s kind gate is not separately pinned: it is the same
[CR#705.1,705.2] line `badCoinCameUpOnAbility` already stands for, one slot
earlier on the same sentence.

### Bench

| Witness | State |
|---|---|
| **Wyll, Blade of Frontiers**, first line | **benches whole**, as `wyllExtraDie` — and with it Barbarian Class's level-1 line and Pixie Guide's ability-worded twin |
| **Missy**, end-step line | benches as the branch, `missyChaosBranch` — "you draw a card and chaos ensues"; the villainous choice is its own family |
| **Netherese Puzzle-Ward**, second line | **benches whole**, as `netheresePuzzleWardIllumination` (ability word not written) |
| **Resolute Veggiesaur**, second line | **benches whole**, as `resoluteVeggiesaurThirdDie`, with "each turn" as the every-player window |
| **Rakdos, the Showstopper**, trigger body | **benches whole**, as `rakdosShowstopperFlips` — both sentences |
| **Warp Vortex**, first sentence | benches, as `warpVortexFlips`; the remaining sentences count won and lost flips, which no reading of a flip gives [CR#705.2] |
| **Bamboozling Beeble** | blocked — see ledger |
| **Ichor Elixir** | blocked — see ledger |
| **Vedalken Squirrel-Whacker** | blocked — see ledger |
| **Krark's Thumb** | blocked — see ledger |

### Ledger

| Item | State after this round | Exact missing piece |
|---|---|---|
| The die the roll event NAMES ("one or more planar dice", "one or more six-sided dice", and the two planar-die headers) | blocked, **owned by nobody** | a die-kind slot on `RollsDice` beside `DiceBatch`; wanted by 4 lines (2 supported replacements, 2 unsupported Plane headers) and enough for none of them alone [CR#706.1,706.7,901.9d] |
| A counted-and-chosen ignore ("you choose one of those rolls to ignore", "ignore one") (3 lines, 1 of them a flip) | blocked, **owned by nobody** | a third `IgnoredRolls` arm carrying a count, and a chooser slot beside it; [CR#706.6] writes the chooser only as its tie-break |
| The planar roll's count ("instead roll that many planar dice plus one") (1 line) | blocked, **owned by nobody** | `RollPlanarDie` takes a subject and no count; it needs `RollDice`'s count slot, gated apart from [CR#706.1a]'s side count |
| A flip INSTRUCTION as an event ("If you would flip a coin, instead …") (1 line) | blocked, **owned by nobody** | `FlipEvent` is [CR#705.2]'s CALLED flip (win/lose); the flipping act itself has no `GameEvent` row, so no replacement can reach it |
| Exchanging a roll's result with a base characteristic (1 line) | blocked, **owned by nobody** | Vedalken Squirrel-Whacker's "you may exchange one result with this creature's base power or base toughness", and a fully anaphoric "roll them" |
| "Whenever chaos ensues" (172 lines, all-scope, all Plane cards) | blocked, **owned by nobody** | an `EventName` row and a `GameEvent` row; [CR#311.7] names the event, and the deferral is scope — a plane-card round's, not this one's |
| The planar die's blank-face read (1 line, all-scope) | blocked, **owned by nobody** | the die-kind slot above plus a face-valued `RollWatch` arm [CR#901.9a] |
| "After you roll a die, …" (2 lines) | unchanged | a fourth trigger word; [CR#603.1] writes the ability as "[When/Whenever/At]" |
| A coordination of two player nouns ("You and target opponent each …") (1 line) | unchanged | the mention-shape round's |
| Plural/target choosers, chooser-on-a-mention | unchanged | the mention-shape round's |
