---
needs: []
---
# Write the turn schedule's remaining parts, allowances and procedures

The turn's own structure — what parts a turn has, how a turn already introduced
is read back, what a part may be skipped or quantified over, what an "additional"
allowance grants at each subject sort, how a procedure repeats over players, and
the whole-game protocols that sit on top — is one region. The pieces below all
touch the same rows (`TurnPart`, `ExtraTurn`, `Kind.TurnRef`, the allowance
siblings, `Effect.ForEachOf`/`Effect.Repeat`) and are claimed together.

## Added parts and the extra turn's deictic reads

`Effect.AdditionalPart` landed over the additional-phase frame's 24 sentences /
48 occurrences (part, anchor, follower and count as slots, the anchor's sort word
derived; AGGRAVATED ASSAULT, RELENTLESS ASSAULT and HELLKITE CHARGER benched
whole, Full Throttle's, Raphael's and Y'shtola's clauses as fragments), and the
turn mention landed beside it (`Kind.TurnRef` prepended by `ExtraTurn`, read by
`Owner.ThatTurns` gated by `TurnDeixis`; Final Fortune, Last Chance and Chance
for Glory whole — Warrior's Oath prints Last Chance's text word for word and is
not benched separately). The residues of both are the same region: what an added
part may be, and how a turn already introduced is read back.

### The "YOU GET" frame — 3

Obeka, Splitter of Seconds; Paradox Haze; The Ninth Doctor. [CR#500.10a] makes
this a difference of MEANING and not of wording — a "you get" addition on another
player's turn adds nothing — so it is a SECOND ROW and not a slot. Obeka's
variable count ("that many additional upkeep steps") rides with it, reading a
damage amount back.

### The added BEGINNING PHASE — 3

Cyclonus, Shadow of the Second Sun; Sphinx of the Second Sun. The one part the
corpus adds that `TurnPart` has no row for. SPHINX OF THE SECOND SUN is the only
whole card behind it and its header cell probed green
(`BeginningOf PostcombatMain (Just EachYours)`), so the row plus a `Sphinx`
subtype is the entire cost.

### The ORDINAL ANCHOR — 1

World at War's "After the second main phase this turn". Its second sentence also
reads the added combat back ("At the beginning of that combat"), which is the
deictic question asked at a PART rather than at a turn; Moraug, Fury of Akoum
writes the same read.

### The turn mention's second introducer and its non-possessive readers

- The SECOND INTRODUCER, 2 — "during that player's next turn" (Oracle en-Vec;
  Emrakul, the Promised End). This is GIDEON JURA's adverbial, prepending the
  same mention from a second site. Oracle en-Vec then reads it with the
  possessive already built, so both cards are one introducer away from the
  reader they already have; Emrakul additionally wants the fronted anchor below.
- "DURING THAT TURN", 2 — Alchemist's Gambit, Kang the Conqueror. A
  demonstrative on the PART itself rather than a possessive determiner before
  it, so `Owner` is the wrong slot and `Timing`/`Duration` is where it would sit.
- SAVOR THE MOMENT's "the untap step of that turn", 1 — the skip derives its
  possessive from the SUBJECT by design and has no slot for a turn. Giving it
  one is a second surface, not a cell.

### The extra turn's own residues

- EMRAKUL, THE PROMISED END's fronted anchor, 1 — "After that turn, that player
  takes an extra turn", the one sentence of 34 that does not write "after this
  one".
- The FOR-EACH multiplier, 3 — Ral Zarek, Expropriate, Sage of Hours. A scaling
  adverbial over a count of one, NOT a third count cell.
- The extra turn as a CLASS OF TURNS, 1 — Medomai the Ageless's "Medomai can't
  attack during extra turns", which reads extra turns rather than creating one
  and is a `TriggerWindow`/restriction question.

### Not this round's

"At this turn's next end of combat", 3 — Gaze of the Gorgon, Glyph of Doom,
Triton Tactics. These name the CURRENT turn with no introducer at all and are the
delayed trigger's endpoint question.

### Carriers still blocked, each checked

FURY OF THE HORDE (alternative cost), WAVES OF AGGRESSION (retrace), SEIZE THE
DAY (flashback — its two sentences write today), AKKI BATTLE SQUAD
(modified-permanent predicate), RAPHAEL, TAG TEAM TOUGH (menace, and the "for the
first time each turn" trigger rider), MORAUG and OVERPOWERING ATTACK
(attack-count reads), LAST NIGHT TOGETHER and ALL-OUT ASSAULT (their other
lines), ZARIEL (an emblem holding the clause), GREAT TRAIN HEIST (spree),
BALTHIER AND FRAN / LIGHTNING RUNNER / COMBAT CELEBRANT / BREATH OF FURY / BRUCE
BANNER (their own mechanics).

## Turn-part quantifiers, skips, and the recurring untap grant

The landed `SkipsNext`/`Skips` rows take a single `TurnPart` on a named turn.
Five measured surfaces sit outside that shape, and the untap cap's mirror — a
recurring GRANT over another player's step — wants the same quantified possessor.

### The WOULD-WORDED skip — 4 sentences

"If you would begin your draw step, you may skip that step instead" (Fasting),
"If you would begin your turn while this artifact is tapped, you may skip that
turn instead" (Time Vault), and Gerrard's Hourglass Pendant / Stranglehold's "If
a player would begin an extra turn, that player skips that turn instead" (2).

These want a BEGINS-A-STEP/TURN event for `Intercepts` to watch, which no
`GameEvent` row supplies; the last two additionally want the extra turn as a
DESCRIPTION rather than as a creation.

### The QUANTIFIED PART — 2

"Skips all combat phases of their next turn" (Empty City Ruse, False Peace) — a
quantifier over the parts of a named turn, and a second surface. Both are
one-line white sorceries, so this is two whole cards for one construction.

### The WINDOWED scheduled skip — 3

Elfhame Sanctuary's "your draw step this turn"; Moment of Silence's "their next
combat phase this turn", which writes the window AND "next"; Fatespinner's "each
instance of the chosen step or phase this turn".

### The VARIABLE skip count — 1

Ral Zarek, Guest Lecturer's "their next X turns", which `SkipCount`'s closed
table cannot carry — the land allowance's Nahiri situation exactly.

### Recurring untap grants — 12 lines

Seedborn Muse's family: "Untap [set] you control during each other player's untap
step". Grants where the cap denies. The plural player axis this family was once
said to be the sole consumer of is GONE — it was minted on the outcome gate's
evidence and `CapDomain` retired with it. What is left is the GRANT construction
itself plus ONE possessor cell: "each other player's untap step" writes a
quantifier possessor with an other-marking over it, and `Owner` has `EachPlayers`
with no other-marked row.

### Not this round's, recorded so it is not folded in

The DRAW-EVENT skip, 6 — Island Sanctuary, Living Conundrum, Notion Thief,
Obstinate Familiar, Plagiarize, Possessed Portal. This skips an EVENT and not a
turn part, so it belongs to the interception and is ledgered there; the landed
skip rows take a `TurnPart` and reach none of the six.

## The "additional" allowance at its remaining subject sorts

The additional LAND allowance landed and the surrounding surface was inventoried
at 669 supported sentences in six cells, every one of them a different
construction from the land allowance. This sub-area takes the allowance cells
that differ from it only in subject sort — block, time, card — plus the two land
forms fenced off from the landed row.

### The block allowance — 31 sentences, the object-sorted sibling

"This creature can block an additional creature each combat" is 13 lines by
itself, plus "each creature you control can block an additional creature each
combat" (2), activated "this turn" forms (5), and a quantity axis writing the
same four shapes ("an additional seven creatures", "an additional ninety-nine
creatures", "up to two additional creatures").

It has the SAME window covariance the land cell has — "each combat" on a static
line, "this turn" on a resolving one — so the derived-spelling argument
transfers whole. What does NOT transfer is the constructor: the subject is an
object, not a player, which is `PlayerCant`/`ObjectCant`'s situation exactly
(two rows, siblings), and the rule is [CR#509.1a]'s blocker declaration rather
than [CR#305.2]'s land count.

### The additional time — 46 sentences

"That ability triggers an additional time" (a trigger multiplier, ~15 lines over
a shared shape) and "while voting, you may vote an additional time" (3). Two
more allowances at two more subject sorts.

### The additional card — 31 sentences

A draw inside a draw-step trigger, where the word is an adverbial on an event
the turn already performs. RITES OF FLOURISHING is one line from whole behind
it, its other line being the each-player land allowance that already landed.

### The two land forms fenced from the landed row, with counts

- Nahiri's Lithoforming's "You may play X additional lands this turn" (1) —
  `Quantity` is Nat-based and the variable count is a magnitude.
- FASTBOND's "You may play any number of lands on each of your turns" (1) — the
  unbounded allowance, which drops the word "additional" and states the whole
  number rather than increasing it. `badAnyNumberOfAdditionalLands` is the pin.
  Fastbond's second line is a played-a-land trigger with an intervening if.

### Not this round's, recorded so the next inventory does not re-count them

- The ADDITIONAL COST cell, 328 sentences — already the cost machinery's
  territory.
- The ADDITIONAL PHASE OR STEP cell, recounted at 27 sentences over 51
  occurrences (not 44; three counting patterns were tried and none reproduces
  the earlier figure). It moved to the turn-schedule family as an EXISTENTIAL
  frame ("there is") over the turn schedule with [CR#500.10a]'s "you get" beside
  it, and its existential frame landed for 48 of the 51 occurrences, leaving
  four residues there.

Counts here are a prior session's measurements; re-measure before building.

## For-each-player elements and the loop's odd shapes

`Effect.ForEachOf` is typed at `Object`, so the same construction one kind over
— "For each opponent, …" read back as "that player" — is unwritable. Two
`Effect.Repeat` shapes sit in the same region and neither is the anaphoric loop
that landed: one refills a slot in a written process, one announces the process
before stating it.

Authority:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
— the element row's index is the lattice kind, so the generalisation is over
that index, not a second marked row beside the object one.

### The player element — 53 supported sentences over 50 cards

53 sentences front "For each opponent, …" / "For each player, …", and 32 of
them read the element back as "that player" — Blatant Thievery's "For each
opponent, gain control of target permanent that player controls", which is
[CR#608.2f]'s FIRST worked example, the rule that names this construction.
`ForEachOf` was typed at `Object` on its own corpus's evidence and this cell was
routed rather than folded in unmeasured.

Design pointer, one line: `elemIntro` generalises over `Kind` the way `bindFor`
does, minting `PlayerP` where it now mints `ObjectP`, with the Kind index on the
row.

Note the boundary before scoping: the wider "For each <fresh description>, …"
surface is 237 sentences over 220 cards, and most of it is the ordinary
multiplier `CountOf` already spells. This entry is the read-back subset at the
player sort and nothing else.

### The joined element — 2 cards

The same `elemIntro` generalisation, one kind further out. `ForEachOf`'s
domain is `Noun bs Object`, so a joined-kind group is a type error and not a
gate — there is nothing to pin, and the closure table's stated reason for the
cell ("no binding possible per `YouAnd`'s Kind-indexed `Binding` argument",
`docs/idris-workbench-closure-tables.md:745`) died with
`workbench-join-is-a-constructor`: a lattice kind indexes a binding, as
`bindFor`'s `PhJoin` row shows. Two lines are attested, not the table's one,
and both read the element back at the joined kind:

- Kaboom! — "Choose any number of target players or planeswalkers. For each of
  them, … deals damage … to that player or planeswalker, then …"
- Soulfire Eruption — "Choose any number of target creatures, planeswalkers,
  and/or players. For each of them, … deals damage … to that permanent or
  player."

Take it with the player element above: one `elemIntro` indexed over `Kind`
answers both, minting the payload the kind fixes (`joinHalfPayload` for a
join) instead of always an `ObjectP`. Routed here by
`workbench-joined-kind-binding`.

### The repeated schema — 6 cards

"Repeat this process FOR …", where the domain does not bound the repetition but
FILLS A HOLE in it once per item: Equipoise ("for artifacts and creatures", the
earlier sentence having said lands), Firemind's Foresight ("for instant cards
with mana values 2 and 1"), Invoke Despair, Kathril (ten keywords), Linessa,
and Protection Racket ("for each opponent in turn order"). What is missing is a
parameterized clause: nothing abstracts a slot out of a written process and
refills it.

PROTECTION RACKET DOES NOT COMPOSE WITH `ForEachOf` — checked, not assumed.
That row takes a `Noun bs Object`; this domain is the player element above; the
iteration is ORDERED ("in turn order"); and the process is stated forward
across the four sentences that follow. The rulebook writes this shape itself
([CR#701.44d], explore) — worth reading before scoping.

### The loop's forward announcement — 2 cards

"Repeat the following process X times" (Torment of Hailfire) and Protection
Racket's. A cataphor where the other 42 lines write an anaphor: it ANNOUNCES a
process and then states it, which is a different sentence and possibly a
body-carrying node after all, for these two alone. Torment of Hailfire is
additionally blocked on a disjunctive alternative payment ("unless that player
sacrifices a nonland permanent of their choice OR discards a card"), which is
also Remorseless Punishment's one gap and the largest single blocker in the
counted cell.

Context the two loop shapes must respect: `Effect.Repeat` landed as a LEAF and
not a container, because 20 of the 44 measured lines write it inside a
conditional consequent, where a body-carrying loop would have to restate the
gate as a termination phrase. These two cards are the only place that verdict
is up for re-examination.

Counts here are a prior session's measurements; re-measure before building.

## Game-procedure bundles, and game restart

Five protocol families must be designed as wholes — costing them row by row is
misleading, because each is a procedure whose parts only make sense together. A
sixth item, game restart, is NOT one of the bundles: it is a single queued row
in the same region and is listed here so it is not lost.

### The protocol bundles — each designed whole, never costed row by row

- **The draft.**
- **The secret-choice and voting pair.**
- **Pile partitions** — [CR#700.3a,700.3c,700.3d]. Note Death or Glory
  partitions the GRAVEYARD, so this is not the library gap.
- **Bidding.**
- **End-the-turn.**

A claimant may take one bundle per round; what is forbidden is taking a slice of
one and reporting the family as advanced.

### Game restart — MINT QUEUED, not a bundle

Declined once on the honest count ([CR#727.1], ONE corpus line) and that decline
is **overridden**: a small late round, one row. It does not need whole-protocol
design and should not be held hostage to the bundles above.

## From the v1 comparison (2026-08-24)

Two boundary notes from
[the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md)
(axes 18 and 21), so neither bundle above is sliced from outside.

- **Pile partitions.** The crate encodes piles with `Ident` names —
  `SeparatePiles { group, into: Arc<[Ident]>, by, note, then }` and
  `ChoosePile { from: PileSource, by, random, then }` (`effect.rs:410,425`) —
  which runs into the forward-anaphora binder contract. Whether a pile may be
  *named* is ruled on in
  [workbench-named-memory-channels](workbench-named-memory-channels.md); the
  partition procedure stays this bundle's. The two must land compatibly, and the
  bundle should not be opened before the naming ruling exists.
- **The secret-choice and voting pair** keeps Menacing Ogre's secret number and
  every vote. The coin flip, the die roll and the at-random selection mode are
  *not* part of it and are scheduled separately as
  [workbench-randomness-vocabulary](workbench-randomness-vocabulary.md); that
  ticket is fenced off this bundle explicitly.

## The closure grid this family owns

Ranked below the workbench's fifteen top flip risks — the grid's many zeros are
independent measurements, so a printing moves one cell and the closure survives —
but every cell this family moves is one of them. A widening names the cell it
moved and re-reads the grid rather than defaulting it, and a cell left closed
says whether a rule or a count is closing it. Rows, evidence and widening costs:
[the closure tables](../../idris-workbench-closure-tables.md) §2.3.

`spanUse`'s ~90-cell duration grid (`Experimental.idr:2509`): only four cells
are Unclaimed, the rest Unattested.

## Consumption boundary

`idris/src/Experimental.idr` (`Effect.AdditionalPart`, `TurnPart`, `ExtraTurn`,
`Kind.TurnRef`, `Owner.ThatTurns`, `TurnDeixis`, `Timing`/`Duration`,
`SkipsNext`, `Skips`, `SkipCount`, `Intercepts`, `Owner.EachPlayers`, the untap
grant's carrier, `PlayerCant`/`ObjectCant` and their allowance siblings,
`Quantity`, `Effect.ForEachOf`, `elemIntro`, `bindFor`, `Effect.Repeat`, and the
effect rows the protocols land on),
`idris/src/Experimental/Events.idr` (the beginning-of header cells, `GameEvent`
rows for a begun step/turn, the trigger multiplier and the draw-step adverbial),
`idris/src/Experimental/Words.idr` (part, possessive and window spelling, any
procedure vocabulary a bundle mints, and a word-side gate if the kind index needs
one), `idris/src/Experimental/Macros.idr` (the protocol spellings), the pin
modules `idris/src/Experimental/Proofs*.idr`
(`badAnyNumberOfAdditionalLands`), evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate. The engine-side restart verb
is separate work and is not this ticket's.

## Acceptance

- The "you get" frame lands as a row, not as a slot on the existing one.
- The beginning-phase row lands with Sphinx of the Second Sun whole.
- The second introducer prepends the SAME mention the built reader consumes —
  Oracle en-Vec benches with no new reader.
- The for-each multiplier is an adverbial, not a count cell; the class-of-turns
  read is not folded into the creation row.
- Empty City Ruse and False Peace bench whole off the quantified part.
- The other-marked quantifier possessor is one `Owner` cell, reused by the untap
  grant rather than minted privately for it.
- The variable skip count is not forced into `SkipCount`'s closed table.
- The draw-event six stay out of the turn-part rows.
- The block allowance's subject is an object row beside the player one, not a
  widened player row.
- The window covariance is spelled derived, as it is on the land cell.
- `badAnyNumberOfAdditionalLands` still refuses, and the variable-count form is
  admitted or refused explicitly rather than by `Quantity`'s Nat-ness alone.
- Rites of Flourishing benches whole.
- The element row carries a Kind index; the player sort is reached by that
  index rather than by a second row.
- The 237/220 wider surface stays out — the round admits the read-back subset
  and records the boundary.
- The schema round either mints the parameterized clause or records why the six
  cards do not share one; Protection Racket's non-composition with `ForEachOf`
  survives as a witness.
- The leaf-not-container verdict is either preserved or overturned explicitly,
  with the 20-of-44 conditional-consequent count answered.
- Whichever bundle a round takes lands as a whole procedure with its own
  witnesses, not as an isolated row.
- Pile partitions are graveyard-capable; the library search gap is not conflated
  with them.
- Game restart is minted regardless of its one-line count.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
