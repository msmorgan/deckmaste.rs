---
needs: []
---
# Land the amount reads, the comparison's columns, the fold's axes and the quantity words

Everything numeric in one claimable unit: the `Amount` vocabulary's missing
reads, the comparison machinery's relation column and its two operand slots,
chapter sixty-two's fold (axes, mentioned complements, the element binder), the
quantity cell in both choice tables, and the ordinal occurrence word. They share
`readAmount`, `comparableBound`, `Comparator`, `CountOf`, `StatOf` and
`ProjAxis`, so a round that opens one has to answer the others anyway.

## The Amount vocabulary's measured reads

The X-rider's binder takes any amount that is not already written, so what its
1,118 cards still cannot define X as is exactly the list of reads the `Amount`
vocabulary lacks. The list is short, measured, and each member belongs to the row
it names rather than to the rider. This family takes the members that belong to no
other round, plus the two definition-side surfaces that read an amount back.

### The rider's blocked right sides — the inventory, with counts

- superlative **69** — the amount half and the modifier half both landed; what
  is left of them rides the count-axis catalog gap (a separate round: two of its
  cards fold cards in hand, an axis the catalog does not carry).
- the "amount of" magnitude read **58** — core's `EventSum`; the unnumbered form
  landed already and this is its numeric twin.
- devotion **17** — below.
- die results **12**.
- half **6**. "Half … rounded down/up" is now the **only** thing standing
  between Aspect of Wolf and the second letter, both of its halves being that
  phrase; with the static frame built, Aspect of Wolf is letter Y's nearest
  witness of the three.
- excess damage **5** — belongs to
  [workbench-damage-prevention-and-life](workbench-damage-prevention-and-life.md).

### The devotion read — 61 supported lines

24 are the strict comparison that gates a God's creature-ness ("as long as your
devotion to black is less than five"), 5 are a P/T definition (Anax, Callaphe,
Daxos, Renata, Tymaret — single-slot definitions whose printed box is already
spellable), and 32 are the read in an ordinary amount slot. The comparison frame
and the definition frame are both writable, so **the read is the whole of what is
missing, at three frames**. [CR#700.5] defines it as a count of *mana symbols*
among the mana costs of the permanents a player controls, which is not `CountOf`'s
shape — the domain is symbols inside costs, not a set of objects.

### The "difference between" surface — 11 lines, 6 supported

Jaws of Defeat, Lady Loki; the other 5 are dice Contraptions. **Not** the
directional row already landed: plain English "the difference between 2 and 5"
is three, so a directional floored `Minus` mis-models it whenever the stated
order puts the smaller first, and all six supported lines read symmetrically.
Wants an absolute difference of its own, or `Max` over the two directional ones.
Two of the six are blocked elsewhere, on the life exchange; the other four wait
on this surface alone.

A correction worth keeping: "Doran, Besieged by Time and Spry and Mighty" is
**two** cards, not three, and neither sets a base power and toughness — Doran
writes a ±pump under a rider ("it gets +X/+X until end of turn, where X is the
difference between its power and toughness") and Spry and Mighty writes the same
shape over a chosen pair. The difference *is* their blocker, not a neighbour of
it.

### The asymmetric definition — 12 supported lines

The whole Lhurgoyf family plus Souls of the Lost and Consuming Blob:
"Tarmogoyf's power is equal to the number of card types among cards in all
graveyards and its toughness is equal to that number plus 1". Two definitions in
one sentence, the second reading the first's *computed* value back: "that
number" is an anaphor over a sibling slot's amount, which is neither `ThatMuch`
(no clause did anything) nor `TheDifference` (no comparison held). The landed row
writes each half separately and the corpus never does.

Its printed box is the offset star at the toughness slot ("*/1+*"), which
`PrintedStat` already spells; what is missing is the anaphor and, for Cephalopod
Sentry, the subtracted star ("7-*"). Most of the family is double-blocked
besides: **eight of the twelve** count card types among cards rather than cards,
the domain-style distinct-kind count `CountOf` does not spell, and **five of
those eight** write the all-graveyards possessor as well.

## The comparison's relation column and its two operand slots

Four measured gaps sit in one region of the comparison machinery: what may be
written as a bound, what may be written on the left, which relations exist, and
whether either operand may announce a phrase. They share `readAmount`,
`comparableBound` and `Comparator`. The fifth entry is the anaphor that reads a
comparison back from the containers that type it.

### The summed bound — 12 supported lines, and a corrected measurement

`comparableBound`'s docstring listed the sum among the things that "appear as
bounds ZERO times". It is written twelve times: eleven in the description frame
("search your library for a creature card with mana value equal to 1 plus the
sacrificed creature's mana value" — Birthing Pod, Neoform, Prime Speaker
Vannifar, Oswald Fiddlebender, Enigmatic Incarnation, Iron Man Titan of
Innovation, Repurposing Bay, Synthesis Pod, Vivien on the Hunt, Pyre of Heroes,
In Search of Greatness) and once in the condition frame (Obscura Ascendancy).

All twelve stand at the **equality** and none at any other relation. Read the
other four relations against a sum before deciding whether `writtenBound` grows
or the sum gets its own admission — the earlier round left the refusal standing
rather than widen the bound column inside a round about the relation column.

Measured zeros that must survive: the **scaled product** and the **outcome
read**, both still zero, both still refused by `badScaledBound` and
`badScaledConditionBound`.

### X on a comparison's left — 4 supported lines, one pin already retired

Multiple Choice writes "If X is 1", "If X is 2", "If X is 3", "If X is 4 or
more" — `CompareAmt XVal` against a written bound, at the equality and at the
ranged relation. `readAmount` refuses `XVal` on the left, so none of the four is
writable. `badCompareXSubject` pinned the fourth as a measured zero and is gone.
`badCompareLiteralSubject` ("if 3 is 4 or greater") is untouched and still
correct.

**Measure `ReadAmount`'s other readers before widening it** — the test is shared,
and the reason it exists is that a numeral on the left states arithmetic rather
than a fact about the game, which is true of a literal and **not** true of an
announced X.

### The equality comparison — 4 supported lines

`Comparator` carries four relations where core has five, because "is equal to"
as a *comparison* is four supported lines, each blocked twice over: a die
result, unspent mana, and a sum. The sum is this family's summed-bound entry, so
the relation rides this round rather than being scheduled for itself.

### The announcing comparison operand

"power greater than target creature's power" (Fell the Mighty). The postnominal
frame threads its bound's announcement (`amtDelta`), but the **condition** frame
introduces nothing, so an announcing subject or bound written there is dropped
silently — a hole the comparison's subject has carried for many chapters and the
bound now shares. The fix is one bindingless test over amounts at both slots,
and **both slots have to be answered together**.

### The gap off the other licensors

The comparison anaphor reads a comparison in the containers that type the
leading order. Three lines want it under `Effect.If` (Balance of Power, Vraska's
[−9], Iymrith's second sentence) and wait on the fronted-conditional
orientation, which is
[workbench-anaphora-mentions-and-creation](workbench-anaphora-mentions-and-creation.md)'s
question, not this one. Three want it off a **choice** clause ("choose an
opponent who controls more lands than you …", Boreas Charger, Sandstone Oracle,
Slithermuse). Two want it off a **superlative** one (Tales of the Ancestors,
Scholarship Sponsor) and neither moved when the superlative landed: both fold
cards in hand, an axis this catalog does not carry.

## The fold: missing axes, mentioned complements, an element binder

Four measured gaps sit on chapter sixty-two's fold — the axis column, the
complement slot, and the per-element read. They share `ProjAxis`, `CountOf` and
`StatOf`. One of the four is a measurement that says NOT to widen the axis
column.

### The extremal modifier's missing AXES — 10 supported modifier lines

`ProjAxis` (today `CharAxis Characteristic | PlayerStatAxis PlayerStat`) does not
carry:

- votes, 4 — "each permanent with the most votes", the will-of-the-council
  family.
- the colour share, 3 — "shares a color with the most common color among all
  permanents", a fold over a QUALITY, which is a third sort beside the object and
  the player.
- cards in hand, 2 — Adamaro, Tales of the Ancestors; declined as a `PlayerStat`
  in chapter fifty-six on the measurement that the corpus counts cards rather
  than reading a hand size.
- a noted number, 1 — Menacing Ogre, chapter fifty-nine's cross-ability memory.

Each rides its own family; none is an extremal problem, so decide each against
its family rather than as an extremal row.

### The fold over a MENTION — 26 lines

The fold takes a DESCRIPTION (`CountOf`'s slot and core's shared `Countable`).
The corpus's other complement is a group already mentioned: "the total power of
the sacrificed creatures" (Soulblast, Corpse Cobble), "the greatest power among
them", "the total mana value of those cards", and the possessive "their total
toughness" (Dracoplasm, Sutured Ghoul) — 4 extremal, 15 sum, 7 possessive.
`ThoseVerbed`/`Them`/`Those` all exist as nouns, so this is a second slot shape
rather than new machinery, and it is what would finally pay `StatOf`'s own
comment (which has named Soulblast since the singular gate was written).

### The relativized per-element count — 10 supported cards

"the greatest number of creatures a player controls" — Investigator's Journal,
Jace's Archivist, Cavern-Hoard Dragon, Thought Sponge, Windfall's cycle. A fold
whose per-element read is a COUNT relativized to the member, spelled with a
relative clause instead of "among".

This is the general projection both prior arts have (core's `Projection { of, by
}` over a bound `It`; the legacy module's `Project`/`bindIt`) and it is what
chapter sixty-two's closed `ProjAxis` deliberately does not buy — the element
these lines bind sits INSIDE the domain's own description, so a general axis slot
alone lands none of them. The real ask is an element binder reaching into a
predicate. The ordinal cast wants the same shape; see
[workbench-event-zone-and-cast-provenance](workbench-event-zone-and-cast-provenance.md).

### The counter axis — 6 lines, and a warning against the row

"the number of counters among creatures you control". Four of the six write the
KIND-BLIND quantifier, which is its own family, and one names a kind outside the
catalog ("time counters"), so a `ProjAxis` row for `CountersOn` would land
exactly ONE line — Vault 12's rad counters, a Saga chapter. Do not mint the axis
row on that evidence: this rides the kind-blind counter work.

## The quantity in both choice tables

`choosable (CountedGroup _ _)` is False and quantity-blind; chapter seventy-six
set it that way on "choose one or more X" being zero, which it still is. Two
cards show the cell over-refusing, and the fix — reading the quantity — owes the
full quantity-by-attestation grid across BOTH choice tables before any cell moves.

- The over-refusals: "Choose up to one creature. Destroy the rest." (Duneblast)
  and "Choose any number of creatures. They block this turn if able."
  (Berserker's Frenzy).
- The "N or more" form is the unwritten one.
- Measured zero that must survive the round: "choose one or more X" is zero
  lines, and that is what set the blind cell. Keep it a zero or re-measure it
  explicitly — never let it pass silently.
- The grid runs across `choosable` and chapter seventy-seven's `agentChoosable`;
  answering one table alone leaves the other blind.
- `badChooseCountedGroup` is unaffected: its own line writes `atLeast 1`.

## The ordinal occurrence word — counters, casts and library positions

Three constructions want the same missing word — WHICH occurrence in a sequence,
not how many — and each was deferred separately for that reason. Taking them
apart mints the word three times.

### The ordinal counter event — 11 headers

"When the fourth plan counter is put on this enchantment": 11 supported headers
(7 plan, 1 hour, 1 +1/+1, plus siblings) that the landed `CounterEvent`
deliberately does not spell. `CounterBatch` is two determiners, "a" and "one or
more", and an ordinal is a third thing entirely — it names WHICH counter in a
sequence rather than how many. **Every one of the 11 writes `When` and not
`Whenever`**, which is what a once-per-object event looks like. The family is
Sagas' plan counters and Midnight Clock.

### The ordinal cast restriction — 8 + 3 headers

"Whenever you cast your FIRST spell during each opponent's turn" — Alela, Arena
Trickster, Blightwing Bandit, Dreamstalker Manticore, Mischievous Chimera,
Stinging Lionfish, Wavebreak Hippocamp and more; **8** headers write the
each-opponent's-turn form and Rashmi and Ragavan writes the each-of-your-turns
one. Plus "your second card" / "their second spell" (The Council of Four, Moon
Girl and Devil Dinosaur) and Geralf's "other than your first spell that turn".

The window landed and these did not: what they restrict is WHICH occurrence
counts, an ordinal over the event within the window.

### The library position, and the history identity read

`LibPos`'s comment already ledgers "second from the top" ([CR#401.7]) as the
same vocabulary at a third site — that ledger is the argument for doing them
together. And Once Upon a Time's "if this spell is the first spell you've cast
this game" is an ORDINAL IDENTITY read over the history, not a count and not an
occurrence: a fourth site for the word. Note that the rest of that card (look at
five, reveal, bottom in a random order) is unbuilt besides, so it lands no card
here — fold it in or refuse it explicitly.

## The closure grid this family owns

Ranked below the workbench's fifteen top flip risks — the grid's many zeros are
independent measurements, so a printing moves one cell and the closure survives —
but every cell this family moves is one of them. A widening names the cell it
moved and re-reads the grid rather than defaulting it, and a cell left closed
says whether a rule or a count is closing it. Rows, evidence and widening costs:
[the closure tables](../../idris-workbench-closure-tables.md) §2.3.

`pumpSignsOk`'s four disagreeing-zero cells (`Experimental.idr:2705`). It ranks
at the **top** of the grid list rather than the bottom, because the four zeros
are one canonicality fact rather than four independent ones — one printing can
take the whole closure, not one cell.

## Measured cell (2026-08-21)

- `comparableBound` admits a summed bound at all five comparators; the corpus attests it at equality only — of 36 "with <char> equal to …" lines, 15 take "the number of", 0 a summed bound; 24 "equal to the total" corpus-wide, 0 "greater/less than the total". Gate the summed bound to equality.

## From the v1 comparison (2026-08-24)

Two findings from
[the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md)
(axis 12, graded **WORSE**, and summary finding 2). Both are shape questions
this ticket's ledgers touch from the vocabulary side; neither is a row.

**Quantity bounds are literal-only, and widening them is not free.**

> Workbench: `data Quantity : Type where Range : Maybe Nat -> Maybe Nat ->
> Quantity` (`Words.idr:589`). Crate: `Quantity::Range(Option<Count>,
> Option<Count>)` (`quantity.rs:22`) — the bounds are full `Count`s. So "up to X
> target creatures", "choose up to that many", "target up to N creatures where N
> is …" have no v2 spelling. `Quantity` is threaded into `TargetGroup`,
> `CountedGroup`, `SomeOf`, `Modal` and `ChooseSpec`, so this is not a corner.

and

> the quantity one is a change to the `Quantity` type's shape and every gate over
> it (`NonZeroQ`, `WellFormedQ`, `quantPlur`, `modesFit`), which currently
> pattern-match on `Nat` literals and would have to become runtime-undecidable
> once bounds are `Amount`s. That interaction is not recorded anywhere.

This lands on the section above: the quantity-by-attestation grid owed across
`choosable` and `agentChoosable` is a grid over *literal* bounds today. Decide
whether an amount-valued bound is in scope before the grid is read, because the
four gates above are the cost, and record the verdict where `Quantity` is
defined either way.

**No division, halving, averaging or rounding.**

> Workbench `Amount` (`Experimental.idr:1458`) has `Lit`, `Times (per : Nat)`,
> `Plus`, `Minus`, `TheDifference`, and the readbacks — no rounding operator
> anywhere except `DamageScale::Halved` on a prevention shield
> (`Experimental.idr:2903`). Crate `Count` (`count.rs:167`) has
> `Half(RoundMode, ..)`, `Divide(RoundMode, .., ..)`, `Mod`, `Pow`, `Min`,
> `Max`, and `Aggregate(AverageOf(RoundMode), Projection)` (`count.rs:129`).
> `RoundMode` exists in `Words.idr:1538` and is used at exactly one site.
> "Equal to half your life total, rounded down" is unwritable.
>
> Also absent from `Amount` with crate counterparts: `Count::ManaAvailable`,
> `ManaAvailableKind`, `Damage(Reference)`, `TargetsOf(Reference)`, `TimesPaid`,
> `Noted`, `Allotment`.

The half already has a ledger entry above (6 lines, Aspect of Wolf). The rest of
the operator set does not — take `RoundMode`'s single existing site as the shape
to generalise rather than minting a second rounding vocabulary, and route
`TimesPaid` to
[workbench-cost-tags-and-paid-readbacks](workbench-cost-tags-and-paid-readbacks.md)
and `Noted` to
[workbench-named-memory-channels](workbench-named-memory-channels.md), which own
those channels.

## Consumption boundary

`idris/src/Experimental.idr` (the `Amount` reads, `CountOf`, `PrintedStat`, the
definition frame and its sibling-slot anaphor; `readAmount`/`ReadAmount`,
`comparableBound`, `writtenBound`, `CompareAmt`, `amtDelta`, the comparison
anaphor's licensor list; the fold, `StatOf`, `CountersOn`,
`ThoseVerbed`/`Them`/`Those`; `choosable`, `agentChoosable`, `CountedGroup`;
`CounterEvent`, `CounterBatch`, the cast-restriction header and its window,
`LibPos`), `idris/src/Experimental/Words.idr` (any new catalog row;
`Comparator`; `ProjAxis`, `projScope`, `sameProjAxis`; `LibPos`'s words and its
ledger comment), `idris/src/Experimental/Events.idr` (`CounterBatch`'s event
side, the header window), the pin modules `idris/src/Experimental/Proofs*.idr` —
including `idris/src/Experimental/ProofsB.idr`,
`idris/src/Experimental/ProofsD.idr`, `idris/src/Experimental/ProofsE.idr`
(`badChooseCountedGroup`) and `idris/src/Experimental/ProofsG.idr` — evidence
bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Each amount read lands as its own row; nothing is folded into the rider, which
  is not where any of these is blocked.
- The absolute difference is symmetric at all six supported lines, and the
  directional row is unchanged.
- The card-type-among-cards count and the all-graveyards possessor are recorded
  as still-blocking for the eight and the five, if this round does not take them.
- The sum's admission is decided against all five relations, not just the
  equality, and the decision is stated where the bound column is defined.
- The scaled product and the outcome read remain measured zeros with their pins
  intact; `badCompareLiteralSubject` still holds.
- The announcing subject and the announcing bound are answered in one change.
- Each new fold axis is justified by its own family, and the cards-in-hand axis
  does not silently reverse chapter fifty-six's `PlayerStat` refusal without
  stating the re-measurement.
- The mentioned complement lands as a second slot shape, with no new fold
  machinery, and `StatOf`'s comment is paid or updated.
- The element binder reaches into the domain's own predicate and the 10
  relativized cards land, or the shortfall is named card by card.
- `CountersOn` gets NO `ProjAxis` row on this round's 1-landing evidence.
- Both choice tables are answered in one change, with the quantity-by-attestation
  grid recorded where the cells are defined.
- Duneblast and Berserker's Frenzy are writable.
- `badChooseCountedGroup` still refuses on its `atLeast 1` line.
- One ordinal vocabulary serves the counter event, the cast restriction and
  `LibPos`; the `LibPos` ledger comment is discharged rather than left standing.
- `CounterBatch`'s two determiners are not extended into a third — the ordinal is
  its own thing.
- The counter event keeps `When`'s once-per-object reading for all 11 headers.
- The history identity read is either folded in or refused with its reason
  recorded; Once Upon a Time is not counted as a payoff.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
