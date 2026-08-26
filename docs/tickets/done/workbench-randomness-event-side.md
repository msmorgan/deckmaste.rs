---
needs: []
---
# The randomness family's event side: flip and roll as things that happen

`docs/tickets/done/workbench-randomness-vocabulary.md` wrote the random ACTS —
the coin flip, the die roll, the results table, the at-random selection mode —
and drew a hard boundary at the event side, ledgering seven items. They are one
region (an `EventName`/`GameEvent` row per random act, plus the reads over
several outcomes) and nothing owns them.

## The event rows

From that round's ledger, each with its own line there:

- **Chance Encounter** — "the event-side reader: an `EventName` and `GameEvent`
  row for winning/losing a coin flip (8 lines), which the ticket made optional."
- **"Whenever you roll one or more dice"** — "the same event-side axis, roll
  side."
- **Karplusan Minotaur** — its cost-side blocker was removed (`costActionOk
  (FlipCoins who _) = costNounOk who`), so "Cumulative upkeep—Flip a coin"
  composes as a cost; what is left is "the coin-flip trigger event (above) and
  cumulative upkeep". The cumulative-upkeep half is
  [workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)'s;
  the event is this ticket's.

## The reads over more than one outcome

- **"Flip five coins. … for each coin that comes up heads"** — "a count over
  flips — the `EventCount` axis, not this round's."
- **"roll two six-sided dice … If you rolled 7"** — writable as a roll; the
  total is not. "`RollResult` mints one singular result, per [CR#706.2]'s
  per-roll definition", so summing several rolls is a read the vocabulary does
  not have.

## Deliberately out, and to be re-decided rather than re-discovered

- **Roll modifiers and ignored rolls** — "roll that many dice plus one", "ignore
  the lowest roll"; [CR#706.2b,706.6]. Marked "deliberately out" by that round.
- **The planar die** — 6 lines, one of them the imperative "{T}: Roll the planar
  die." (Fractured Powerstone). "the die is not an N-sided die [CR#706.1a] and
  has its own faces — a Planechase axis, not this round's." Note the closure
  round's command-zone type widening has already landed the `Plane` type, so the
  Planechase argument is weaker than it was; re-read it.

Each of these last two is fine to close as "still out" — but with the rule
written down, not by silence.

## Not this ticket's

Execution semantics. That round landed none: "Nothing resolves a flip, chooses a
number, or models a randomness source; no decider, no visibility, no value kind
was imported." Keep it that way — this is vocabulary for what a text may *watch*
and *read*, not a randomness engine.

## Consumption boundary

`idris/src/Experimental/Events.idr` (`EventName`, `GameEvent`, `eventUse`, the
complement tables, `EventCount`), `idris/src/Experimental.idr` (`FlipCoins`,
`RollDice`, `TheResult`, `countOutcomes`/`countQuantOutcomes` and the trigger
header's readers), the pin modules `idris/src/Experimental/Proofs*.idr`, and the
evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Chance Encounter and Karplusan Minotaur's trigger each bench, or reduce to a
  named blocker outside the event rows.
- The multi-outcome reads land or are named at their measured size.
- The two "deliberately out" families end with a written rule-backed verdict.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` 23/23 PASS on a clean
rebuild (`rm -rf build` first), 0 errors and 0 warnings. **Five pins added**,
none retired, none silently passing — each names its rule and has a live
positive counterpart. Every count below was re-measured this round with the
corpus scripts (supported scope, `all=` where the wider scope is quoted), and
every witness's oracle text re-fetched with `card`. No execution semantics
landed: nothing resolves a flip, chooses a number, or models a randomness
source, and no decider, visibility or value kind was imported.

### Re-measured surface

| Ticket said | Measured | What it counts |
|---|---|---|
| 8 | **7** (all=8) | "coin flip" as an event ("Whenever you win a coin flip") |
| — | **7** | "Whenever you roll one or more dice" trigger lines (a further 3 supported lines are the replacement side) |
| — | **3** (all=6) | "Whenever you roll a die" — the singular determiner |
| — | **2** | "for each coin that comes up heads" / "the number of coins that came up heads" |
| — | **6** (all) | "the total of those results" |
| — | **6** (all) | "If you rolled ⟨n⟩" |
| 6 | **6** (all; 1 supported) | "roll the planar die" — 13 all-scope lines mention a planar die at all |
| — | 4 / 6 / 2 | "ignore …" / rerolls (all=8) / "increase or decrease the result" |

### The event rows

- **`FlipEvent : (who : Noun bs Player) -> (call : FlipCall) -> GameEvent bs`**,
  naming `FlipWin`/`FlipLoss` through `flipEventName`. [CR#705.2] makes
  winning or losing a flip a thing that happens — the flipper calls the coin,
  and the call either matches or does not — and gives it to that player
  alone ("no other players are involved"), so the subject is a player and the
  event names no second participant. The two arms are one row under a
  `FlipCall` slot on `counterEventName`'s model, so each arm can be answered
  about separately by a name-keyed table. It announces its subject and no
  flip: the win is what happened, and the face belongs to the effect that
  instructed the flip.
- **`RollsDice : (who : Noun bs Player) -> (many : DiceBatch) -> GameEvent bs`**.
  [CR#706.7] states this event in the rules' own words — "any ability that
  triggers whenever a player rolls one or more dice" — so the roll is
  watchable and not merely instructable. `eventAfter` mints `outcomeB
  RollResult`, because the printed bodies read it back ("put a number of
  charge counters on this artifact equal to the result") and [CR#706.2] gives
  the roll its result however the roll was called for. `DiceBatch` is
  spelling only, `OneDie` against `ManyDice`: the corpus writes both
  determiners and the roll leaves one result to read either way.
- Table rows over the three new names: `eventHasMagnitude` False for all
  three ([CR#705.2] gives a flip a face and, when called, a winner and
  nothing numeric; [CR#706.2] makes a roll's result a number the roll
  PRODUCED and read back off it, not an amount the rolling happened in, and
  how many dice were rolled is a count). `lookbackSubjectOk` Player-only for
  all three, on [CR#705.2] and [CR#706.1]; the roll's Player arm is attested
  ("At the beginning of each end step, if you rolled a die this turn"). No
  complement for any of the three. `interceptOk`, `spanEventOk` and
  `bareLookbackOk` keep their catch-alls: no rule refuses these events there.

### The reads over more than one outcome — both landed

- **`TheTotal : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs`** —
  "the total of those results", and on a comparison's left "If you rolled 7".
  [CR#706.2] defines a result per die and stops there, so the total is a
  SECOND read of the same roll, not a spelling of `TheResult`; no rule makes
  it undefined, and doctrine refuses only what a rule makes meaningless, so
  it lands rather than being named at its size. Gated on the roll exactly as
  `TheResult` is. A total over a single die equals that die's result and is
  tolerated overgeneration, named here at its zero.
- **`CoinsShowing : (face : CoinFace) -> Amount bs`**, gated on
  `coinFlipInScope`. The plural twin of the `FlipFace` condition: [CR#705.2]'s
  uncalled reading, the one that has a face and no winner, so it takes no
  subject for the same reason `FlipFace` does not. The gate is `FlipFace`'s
  existence test and not a count, because "Flip five coins" leaves one
  mention for any number of coins; a count over a single flip is 0 or 1 and
  is tolerated.
- Neither is `EventCount`'s axis, which the ticket guessed at. `EventCount`
  reads an event name, a subject and a `Lookback` window, and both printed
  reads carry no window at all — they name the flip or the roll the same text
  wrote a clause earlier. `TheResult` and `PreventedThisWay` are the idiom
  they belong to, and both are built as those are.

### The two deliberately-out families — written verdicts

**Roll modifiers and ignored rolls: still out of this round, and never
pinnable.** [CR#706.2] provides for modifiers outright — "The instruction may
include modifiers to the roll which add to or subtract from the natural
result. Modifiers may also come from other sources" — and [CR#706.6] gives an
ignored roll a defined meaning: "that roll is considered to have never
happened. No abilities trigger because of the ignored roll, and no effects
apply to that roll." Both are therefore rules-meaningful, so this family is
unwritten vocabulary and not overgeneration: no pin may ever refuse it, and
the honest close is a ledger line, not a refusal. It stays out here because
its whole surface sits on the REPLACEMENT side rather than the event side —
every corpus line is of the form "If you would roll one or more dice, instead
roll that many dice plus one and ignore the lowest roll" — so what it needs
is an ignore instruction over rolls the same clause made and a superlative
selection among them ("the lowest roll"), neither of which the roll event
this round lands supplies. [CR#706.2b] grounds nothing this layer would
write: it orders two effects that both modify one result, which is the
engine's business, not the sentence's.

**The planar die: still out, on a sharper rule — and half the old argument is
now retired.** Retired: the EVENT side needs no planar row. [CR#706.7] says
outright that "rolling the planar die will cause any ability that triggers
whenever a player rolls one or more dice to trigger" ([CR#901.9d] repeats
it), so this round's `RollsDice` already spells what a planar roll triggers,
and nothing further is owed there. Still out, with the rule that says why:
[CR#901.3a] gives the planar die six faces — one Planeswalker symbol, one
chaos symbol, the rest blank — so it is NOT the die [CR#706.1a] describes ("N
equally likely outcomes, numbered from 1 to N"), and `RollDice`'s `sides :
Nat` cannot spell it. What is missing is a separate instruction row plus a
face vocabulary the numbered-die rules do not supply, and the planar-specific
readers written over those faces. [CR#706.7]'s second sentence draws the same
line from the reader's side: an effect referring to a numerical result
"ignores the rolling of the planar die", so `TheResult`, `TheTotal` and
`ResultsTable` are inapplicable to it by rule. The landed `Plane` type does
not weaken this: [CR#901.3] makes the planar die a piece of game equipment
the game needs alongside each player's planar deck, not a card, and rolling
it is a special action [CR#116.2i,901.9] — having the card type in the
command zone supplies neither the die, its faces, nor the action.

### Pins

Five, all in `ProofsG`, each naming the rule that makes its term meaningless;
none justified by a count.

| Pin | Refuses | Rule |
|---|---|---|
| `badCoinsShowingWithoutFlip` | "Take an extra turn for each coin that comes up heads." with no flip | [CR#705.2] — the face belongs to a coin some effect flipped |
| `badTotalWithoutRoll` | "If you rolled 7, sacrifice this creature." with no roll | [CR#706.2] — a result is the number a die the text rolled came up on |
| `badCreatureWonFlip` | "creature that won a coin flip this turn" | [CR#705.2] — the flipper wins or loses, and no other player is involved |
| `badRollAsMagnitude` | "the amount of dice you rolled this turn" | [CR#706.2] — the result is a number the roll produced, not an amount it happened in |
| — | the two positives that keep the last two honest are `youWonAFlipThisTurn` and `youRolledADieThisTurn` | |

Positive counterparts on the bench: `ralZarekUltimate` for `CoinsShowing`,
`sparkFiendUpkeepRoll` for `TheTotal`, and the two lookback terms above for
the Player arms the last two pins leave standing.

### Bench

| Witness | State |
|---|---|
| **Chance Encounter** | **benches whole**, as `chanceEncounter` — needed one counter word, `Luck`, an ordinary marker [CR#122.1] on `Suspect`'s model |
| **Karplusan Minotaur**, win arm | **benches**, as `karplusanMinotaurWinFlip` |
| Karplusan Minotaur, lose arm | blocked outside the event rows: "any target of an opponent's choice" is a chooser on a target mention, the same mention-shape gap the prior round ledgered, and `ChoiceMode` has no opponent chooser |
| Karplusan Minotaur, cumulative upkeep | not this ticket's — `workbench-cost-and-payment-residues` |
| **Brazen Dwarf** | **benches whole**, as `brazenDwarf` — the plural roll trigger |
| **Vexing Puzzlebox**, first line | **benches**, as `vexingPuzzleboxCounters` — the body reading the result the event announced |
| **The Space Family Goblinson**, first line | **benches**, as `spaceFamilyGoblinsonRoll` — the singular determiner |
| **Ral Zarek** ultimate | **benches whole**, as `ralZarekUltimate` |
| **Spark Fiend**, upkeep roll | benches to "If you rolled 7, sacrifice this creature", as `sparkFiendUpkeepRoll`; the line's remaining clauses read a total the card NOTED on itself [CR#706.8], which is not written |

### Ledger

| Item | State after this round | Exact missing piece |
|---|---|---|
| Roll modifiers, ignored rolls, rerolls (4 / 8 / 2 lines) | blocked, **owned by nobody** | a replacement over the roll event whose body ignores rolls the clause made and selects among them superlatively [CR#706.2,706.6] |
| The planar die (6 lines, 1 imperative + 4 triggers + 1 face trigger) | blocked, **owned by nobody** | an instruction row, a face vocabulary ({PW}/{CHAOS}/blank) and the special action [CR#901.3a,116.2i]; the roll EVENT is now covered [CR#706.7] |
| "Whenever you roll a 4 or higher" (4 lines, all=13) | blocked, **owned by nobody** | a result-conditioned roll header — the event this round lands carries no result test |
| Noted and stored results (3 lines, all=5) | blocked, **owned by nobody** | noting a value on a permanent and reading it back [CR#706.8a..706.8c] |
| "If any of those results was 10 or higher" (1), "rolled doubles" (1) | blocked, **owned by nobody** | an existential over one clause's rolls, and the equality of two [CR#706.5] |
| "each player whose coin comes up tails" (3 lines) | blocked, **owned by nobody** | a per-player face read — `FlipFace` takes no subject because [CR#705.2] gives the CALLED reading none, and this is the uncalled reading distributed over players |
| "the difference between those results" (5 lines) | unchanged | owned by `workbench-amount-comparison-and-quantity`; `TheTotal` is the shape it reuses |
| Plural/target choosers ("at random", "of their choice", "of an opponent's choice") | unchanged | the mention-shape round the prior ticket ledgered |
