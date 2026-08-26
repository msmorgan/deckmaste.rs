---
needs: []
---
# Randomness residues: roll replacements, dice follow-up reads, the planar die

Routed from `workbench-randomness-event-side` (close, 2026-08-26), which
landed the flip/roll event rows (`FlipEvent`, `RollsDice`), `TheTotal`,
`CoinsShowing`, and the two rule-backed verdicts. Six measured remainders,
one region:

1. **Roll modifiers, ignored rolls, rerolls** (4/8/2 lines) — the whole
   surface is REPLACEMENT-side ("If you would roll…, instead roll that many
   plus one and ignore the lowest roll"): needs an ignore instruction over
   rolls the same clause made and a superlative selection among them
   [CR#706.2,706.6]. Never pinnable — both rules make the family meaningful.
2. **The planar die** (6 lines) — instruction row, face vocabulary, special
   action [CR#901.3a,116.2i]. Its roll EVENT is already covered by
   `RollsDice` per [CR#706.7]; numerical reads are inapplicable to it by
   that rule's own second sentence.
3. **Result-conditioned roll headers** — "Whenever you roll a 4 or higher"
   (4 supported, 13 all).
4. **Noted/stored results** (3 supported, 5 all) [CR#706.8a..706.8c].
5. **Reads over a roll set** — "If any of those results was 10 or higher"
   (1), "rolled doubles" (1) [CR#706.5].
6. **The uncalled face read distributed over players** — "each player whose
   coin comes up tails" (3 lines) — `CoinsShowing`'s per-player twin.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`, `Phrase.idr`,
`Effect.idr` as each item's shape demands; `Cards.idr` bench; `Proofs*.idr`.
No Rust crate. Vocabulary only — no execution semantics, no randomness
engine (the parent round's fence carries over).

## Acceptance

- Each numbered item lands or ends in a written rule-backed verdict; no
  silent drops.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` 23/23 PASS on a clean
rebuild (`rm -rf build` first), 0 errors and 0 warnings. **Five pins added**,
none retired, none silently passing. Every count below was re-measured this
round with the corpus scripts (supported scope, `all=` where quoted), and every
witness's oracle text re-fetched with `card`. No execution semantics landed:
nothing resolves a flip, chooses a number, or models a randomness source.

Five of the six items landed in whole or in part; the two families the parent
round left open (the replacement side of the roll modifiers, the planar face
vocabulary) close in written verdicts below.

### Re-measured surface

| Ticket said | Measured | What it counts |
|---|---|---|
| 4 (ignore) | **7** | randomness "ignore" lines — **2 instruction-side**, 5 replacement-side |
| 8 (rerolls) | **1** (all=10) | the reroll verb proper: Centaur of Attention's stored-results reroll. The other supported "again" lines are [CR#706.3c]'s "You may roll again" and a repeated flip |
| 2 | **2** (all=6) | "increase or decrease the result" |
| 6 | **2** (all=13) | planar die — 1 instruction (Fractured Powerstone), 1 replacement |
| 4 (all=13) | **4** (all=13) | "Whenever you roll a ⟨n⟩" — confirmed |
| 3 (all=5) | **3** (all=4) | stored results — all three are Centaur of Attention |
| 1 + 1 | **1 + 1** | "if any of those results was" / "rolled doubles" — confirmed |
| 3 | **3** | "whose coin comes up ⟨face⟩" — confirmed |

The parent round's premise for item 1 does not survive re-measurement.
It read "the whole surface is REPLACEMENT-side"; two supported lines write
the ignore as a plain instruction in the same clause as the roll —
Berserker's Frenzy's "Roll two d20 and ignore the lower roll." and Iron
Mastiff's "…and ignore all but the highest roll." That half is what landed.

### 1 — Roll modifiers and ignored rolls: the instruction side landed

- **`IgnoreRolls : (which : IgnoredRolls) -> {auto 0 ok : countOutcomes
  RollResult bs = 1} -> Effect bs`**, over
  **`data IgnoredRolls = IgnoreExtreme RollExtreme | IgnoreAllBut RollExtreme`**
  and **`data RollExtreme = LowestRoll | HighestRoll`** in the word catalog.
  [CR#706.6] gives the instruction its whole meaning — an ignored roll "is
  considered to have never happened. No abilities trigger because of the
  ignored roll, and no effects apply to that roll" — and writes the
  superlative form itself, settling the tie it can leave. Its own clause
  rather than a slot on `RollDice`, on `ResultsTable`'s ground: [CR#706.3b]
  binds the roll and what is done with its results into one ABILITY without
  binding them into one node, and a roll may carry no ignore at all. Both
  arms are printed and neither spells the other: over more than two dice
  they keep different numbers of rolls. It mints nothing — the surviving
  rolls are the roll the clause already named, which is what a following
  `ResultsTable` or `TheResult` reads.
- **`ShiftResult : (amt : Amount bs) -> {auto 0 ok : countOutcomes RollResult
  bs = 1} -> Effect bs`** — "increase or decrease the result by 1".
  [CR#706.2] provides for it outright: "The instruction may include modifiers
  to the roll which add to or subtract from the natural result. Modifiers may
  also come from other sources." The direction is NOT a slot: every printed
  line writes the disjunction whole and leaves the choice to whoever applies
  it, so a direction word would name nothing the corpus says. Ungated above:
  Xenosquirrels' own ruling has the shift reach "a 0 or a 7 on a six-sided
  die", so no bound belongs here.
- **The replacement side is still unwritten, and still never pinnable.** The
  parent verdict stands — [CR#706.2] provides for modifiers and [CR#706.6]
  gives an ignored roll a defined meaning, so both are rules-meaningful and
  no pin may refuse the family. What refuses it now is exact and narrower
  than "a replacement over the roll event": `Intercepts (RollsDice …)` is
  writable, and its body is not. `eventIntro (RollsDice who _ _) = nomIntro
  who` announces the roller alone, so "instead roll THAT MANY dice plus one"
  has no antecedent for its count (`ThatMuch` presupposes a quantity
  mention), and `RollDice`'s `sides : Nat` is a literal with no anaphoric arm
  to spell "dice" of the kind the replaced roll named. Two slots, both on the
  roll's own announcement. Ledgered.
- **Rerolls are item 4's**, not this one's: the sole supported line for the
  verb is Centaur of Attention's, and [CR#706.8b] is the rule that defines it.

### 2 — The planar die: the instruction row landed, the faces closed

- **`RollPlanarDie : (who : Noun bs Player) -> Effect bs`**. Its own row and
  never a `RollDice` with a side count: [CR#901.3a] gives the planar die one
  Planeswalker face, one chaos face and four blanks, which is not the die
  [CR#706.1a] describes ("N equally likely outcomes, numbered from 1 to N").
  It announces nothing, because [CR#706.7] says any effect referring to a
  numerical result of a die roll "ignores the rolling of the planar die" — so
  `TheResult`, `TheTotal` and `ResultsTable` are inapplicable to it by rule
  and there is nothing for this row to mint. The parent round's event-side
  verdict is untouched: [CR#706.7]'s first sentence and [CR#901.9d] already
  give a planar roll to `RollsDice`.
- **The special action is retired as an item, not deferred.** [CR#116.2i] and
  [CR#901.9] grant rolling the planar die to the ACTIVE PLAYER from the
  Planechase rules themselves — "any time the active player has priority and
  the stack is empty, but only during a main phase of their turn" — and no
  card writes it. A grammar for card text has nothing to spell there.
  Fractured Powerstone's "{T}: Roll the planar die." is an ordinary activated
  ability, and that is the row above.
- **The face vocabulary stays out, with its rule.** [CR#901.3a]'s three faces
  get their meaning from [CR#901.9a..901.9c] — a blank face does nothing, the
  chaos symbol makes chaos ensue [CR#311.7], the Planeswalker symbol triggers
  the planeswalking ability [CR#901.8]. Those are game-rule consequences of a
  special action, and the card lines that read them ("Whenever you roll a
  blank on the planar die", "Whenever chaos ensues", "Whenever you roll the
  planar die") name Planechase's own EVENTS, not this row's roll: they need
  `EventName` rows and a face slot of their own, and no supported-scope line
  writes one. Two supported lines do write "chaos ensues" as an INSTRUCTION,
  which [CR#311.7] admits beside the die face ("if a resolving spell or
  ability says that chaos ensues"); that is the same Planechase family and is
  ledgered with it, not with the die.

### 3 — Result-conditioned roll headers: landed

**`RollsDice` gains a third slot**:
`RollsDice : (who : Noun bs Player) -> (many : DiceBatch) -> (res : Maybe
(Quantity bs)) -> {auto 0 rt : RollTest res} -> GameEvent bs`, gated by
**`data RollTest : Maybe (Quantity bs) -> Type`** (`AnyResult` / `ResultIn`),
which carries `RollRow`'s three gates (`NonZeroQ`, `WellFormedQ`,
`quantLiteral`) on `SpanOk`'s model.

The test IS the quantity vocabulary's literal range, not a vocabulary of its
own: [CR#706.3a] writes the same three forms for a results table's left
column — a single number, "N1–N2", "N+" — and the printed headers write all
three ("a 6", "a 1 or 2", "a 4 or higher"). It is not a third `DiceBatch`
arm: the batch is the determiner the header writes, and a tested header may
write one alongside the test ("a 4 or higher on a die") or write the range in
the determiner's place. `eventName` is unchanged — a tested roll is a roll —
so every name-keyed table answers about it as before.

Two supported header shapes remain outside the range vocabulary and are
ledgered: "Whenever you roll a die's highest natural result" (a test on the
die's own maximum, not a literal) and "Whenever you roll your third die each
turn" (`NthOccurrence` over this event, untried this round).

### 4 — Noted and stored results: landed whole

All three lines are Centaur of Attention's, the card [CR#706.8] is written for.

- **`StoreResults : (on : Noun bs Object) -> {auto 0 one : nounPlur on = OneOf}
  -> {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs`** —
  [CR#706.8a] notes "both the kind of die rolled and the result of that
  roll", so the instruction presupposes the roll and the holder is singular.
- **`RerollStored : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
  (whose : Noun (nomIntro who) Object) -> … -> Effect bs`** — [CR#706.8b]
  rolls one die of each noted kind and stores the new results in the old
  ones' place. NO roll gate: the rolls it repeats are the ones already noted
  on the holder, which is why the rule can define it with no roll in the
  sentence, and [CR#706.8c] links this ability to the one that stored them.
  It mints nothing either — the new results are stored, not left to read.
- **`GreatestStoredMatch : (n : Noun bs Object) -> {auto 0 one : nounPlur n =
  OneOf} -> Amount bs`** — "the greatest number of stored results on it of the
  same value". One row and never an `Aggregate`: [CR#706.8a] makes the domain
  noted numbers rather than a described set, so no `ProjAxis` names it and no
  predicate ranges over it. Ungated beyond the holder, because [CR#706.8c]'s
  linkage is what pairs the read with the storing.

### 5 — Reads over a roll set: both landed

- **`AnyResultIs : (r : Comparator) -> (bound : Amount bs) -> {auto 0 ok :
  countOutcomes RollResult bs = 1} -> Condition bs`** — "If any of those
  results was 10 or higher". [CR#706.2] gives each die its own result and
  stops there, so a clause that rolled several left several numbers behind;
  `TheResult` reads one and `TheTotal` sums them, and neither asks whether
  SOME one of them clears a bound. Written with a comparator and an amount
  rather than the header's striation range because it stands in a sentence
  ("was 10 or higher"), not in a determiner's place. Over a single die it
  agrees with the plain comparison on `TheResult` and is tolerated
  overgeneration, named here at its zero.
- **`RolledDoubles : {auto 0 ok : countOutcomes RollResult bs = 1} ->
  Condition bs`** — [CR#706.5] defines the phrase outright: "A player has
  rolled doubles if the result of each of those rolls is equal to the other."
  No subject slot, for `TheTotal`'s reason: the rule's "those rolls" is the
  roll the clause already named and the roller comes with it. Over a clause
  that rolled some other number of dice it says every result equals every
  other — tolerated overgeneration, named at its zero.

### 6 — The uncalled face read distributed over members: landed

**`CoinCameUp : {k : Kind} -> (face : CoinFace) -> {auto 0 fl : So
(coinFlipInScope bs)} -> {auto 0 rk : So (kindLte k (Object \/ Player))} ->
Predicate bs k`**.

A PREDICATE and not a condition: `FlipFace` reads the one coin a clause
flipped and takes no subject because [CR#705.2] gives the uncalled reading
none, while this narrows a described set by each member's OWN coin, which is
what a per-member flip leaves to read. Kind-polymorphic because both kinds
are printed — "each player whose coin comes up tails" and "each creature
whose coin comes up tails" — and [CR#705.1] makes a coin a physical
randomiser no kind of referent owns, so the gate is the coarse join
`DealtThisWay` already uses; a coin flipped for an ability is a category
error. No call rides here: [CR#705.2] says no player wins or loses a flip
read this way.

### Pins

Five, all in `ProofsG`, each naming the rule that makes its term meaningless;
none justified by a count, and each with a live positive on the bench.

| Pin | Refuses | Rule |
|---|---|---|
| `badIgnoreWithoutRoll` | "Ignore the lowest roll." with no roll | [CR#706.6] — an ignored roll is one that would otherwise have happened |
| `badStoreResultsWithoutRoll` | "Store those results on this creature." with no roll | [CR#706.8a] — storing notes the kind of die rolled and that roll's result |
| `badRolledDoublesWithoutRoll` | "If you rolled doubles, …" with no roll | [CR#706.5] — the phrase compares "each of those rolls" |
| `badCoinCameUpOnAbility` | "an ability whose coin comes up tails" | [CR#705.1,705.2] — a coin is flipped for a player or an object, never for an ability |
| `badZeroRollTest` | "Whenever you roll a 0, …" | [CR#706.1a] — a die is numbered from 1 to N, so a ceiling of zero covers no result |

`ShiftResult` and `AnyResultIs` carry the same roll presupposition as
`IgnoreRolls` and are not separately pinned: `badTotalWithoutRoll` and
`badIgnoreWithoutRoll` already stand for that gate on both sides of the
Amount/Effect line.

### Bench

| Witness | State |
|---|---|
| **Berserker's Frenzy**, its roll | **benches**, as `berserkersFrenzyRoll` — "Roll two d20 and ignore the lower roll" |
| **Iron Mastiff**, its ignore | benches over a bare roll, as `ironMastiffIgnore`; the card's own roll counts "each player being attacked", which no player description carries |
| **Xenosquirrels**, its modifier | benches over a roll of its own, as `xenosquirrelsShift`; the card's header word is "After", and [CR#603.1] writes a triggered ability as "[When/Whenever/At]", which is the whole of `TriggerWord` |
| **Atomwheel Acrobats**, first line | **benches whole**, as `atomwheelAcrobatsRoll` — the two-ended test, and "that many" reading the result back |
| **Monoxa, Midway Manager**, first line | **benches whole**, as `monoxaRollTrigger` — the one-ended test, and "the roll" read back as `TheResult` |
| **Fractured Powerstone**, second line | **benches whole**, as `fracturedPowerstonePlanarRoll` |
| **Centaur of Attention** | **benches whole**, as `centaurOfAttention` — all three lines |
| **Farideh, Devil's Chosen**, second sentence | **benches**, as `faridehResultRead`; the line's first sentence grants two keywords at once, a coordination of grants |
| **Celebr-8000**, its doubles clause | **benches**, as `celebr8000Doubles` |
| **Goblin Assassin**, second sentence | benches to "each player flips a coin. Each player whose coin comes up tails sacrifices a creature", as `goblinAssassinCoinTails`; "of their choice" is refused because the sentence mentions players twice and `TheirChoice` presupposes one chooser mention — the pre-existing chooser-mention gap |
| **Rakdos, the Showstopper** | blocked outside this round: "flip a coin for each creature" is a per-OBJECT flip, and `FlipCoins` takes a player subject |
| **Mana Clash** | blocked outside this round: "You and target opponent each flip a coin" is a coordination of two player nouns, which no mention shape carries |

### Ledger

| Item | State after this round | Exact missing piece |
|---|---|---|
| The replacement side of roll modifiers and ignored rolls (5 lines) | blocked, **owned by nobody** | the roll event must announce its die COUNT and its die KIND, and `RollDice`'s `sides` needs an anaphoric arm, before "instead roll that many dice plus one" can be written [CR#706.2,706.6] |
| Planechase events and the chaos instruction (10 planar-die event lines and 184 "whenever chaos ensues" lines, all-scope; 2 supported lines INSTRUCT chaos to ensue) | blocked, **owned by nobody** | `EventName` rows and a face slot for "whenever you roll the planar die" / "whenever you roll a blank" [CR#901.9a..901.9c], and the "chaos ensues" instruction [CR#311.7] the two supported lines write |
| "After you roll a die, …" (2 lines) | blocked, **owned by nobody** | a fourth trigger word; [CR#603.1] writes the ability as "[When/Whenever/At]" and the corpus writes "After" |
| "Whenever you roll a die's highest natural result" (1 line) | blocked, **owned by nobody** | a result test against the die's own maximum, which no literal range spells |
| "Whenever you roll your third die each turn" (1 line) | untried | `NthOccurrence` over `RollsDice` with the header's window — no vocabulary is missing, only a bench |
| A per-object coin flip ("flip a coin for each creature …") (1 line) | blocked, **owned by nobody** | `FlipCoins` takes a player subject; a flip made FOR a described object needs a second slot [CR#705.1] |
| A coordination of two player nouns ("You and target opponent each …") (1 line) | blocked, **owned by nobody** | the mention-shape round's, as before |
| Plural/target choosers, chooser-on-a-mention | unchanged | the mention-shape round the prior tickets ledgered |
