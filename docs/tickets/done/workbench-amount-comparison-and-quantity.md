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

## Re-audit (2026-08-24)

The sections above were written against an older tree. Read against the tree as
claimed, these named artifacts do not exist:

| named | status |
|---|---|
| `comparableBound` | **never existed** in this tree (0 hits under `idris/src`). The bound column has no gate at all; `CompareAmt` takes any `Amount`. |
| `badScaledBound`, `badScaledConditionBound` | **never existed** (0 hits). The "measured zeros that must survive" have no pins to survive in. |
| `badChooseCountedGroup` | **never existed** (0 hits). Nothing to keep refusing. |
| `pumpSignsOk` (claimed at `Experimental.idr:2705`) | **never existed**; that line holds `DefinesPt`. |
| `sameProjAxis` | **never existed**; an `Eq ProjAxis` instance stands in its place. |
| `writtenBound` | exists, **no consumer** (signature + rows only). Residue of a removed bound gate; left untouched, conductor decides deletion. |
| `StatOf`'s "Soulblast" comment | **never existed** in this tree (0 hits for `Soulblast` under `idris/src` at the round's baseline). Nothing to pay. |
| the `LibPos` ledger comment holding "second from the top" beside the counter and cast sites | **never existed** (the only baseline "second from the top" is a witness docstring quoting a card). Nothing to discharge. |

Already landed contrary to this ticket's premises, before the round opened:

- `Comparator` already carries **five** relations (`AtLeast | AtMost | Greater |
  Less | Eq`) — the "equality comparison" entry was already discharged.
- `choosable (CountedGroup _ _) = True` already (`Experimental.idr:1872`), so the
  choice cell was **not** over-refusing.
- `condDelta (CompareAmt subj _ bound)` already threads both operands' deltas
  beside `gapB` — the "announcing comparison operand" hole was already closed in
  the condition frame.
- Balance of Power (`balanceOfPower`), Vraska's ultimate
  (`vraskaBetrayalsStingUltimate`) and Iymrith's margin read (`iymrithGapDraw`)
  were already benched.

Where the ticket and the tree disagreed, the tree won.

Two of this ticket's own instructions are also superseded by
`docs/memory/rulings/measurements-live-in-pins.md` (2026-08-22), which postdates
the "Measured cell (2026-08-21)" block above:

- "Gate the summed bound to equality" — **not done**. A count never refuses; the
  bound slot stays open at all five relations and the attestation shape is
  recorded here, not gated. The verdict is stated in `CompareAmt`'s docstring.
- "the fix — reading the quantity — owes the full quantity-by-attestation grid
  across BOTH choice tables" — **not done**. Both choice tables stay
  quantity-blind; no cell moved, and the grid is not owed because no cell was
  read.

## Acceptance

- Each amount read lands as its own row; nothing is folded into the rider, which
  is not where any of these is blocked.
- The absolute difference is symmetric, and the directional `Minus` row is
  unchanged.
- The card-type-among-cards count and the all-graveyards possessor are recorded
  as still-blocking for the eight and the five (Ledger below).
- The bound slot's admission is decided against all five relations, and the
  decision is stated where the bound column is defined (`CompareAmt`'s
  docstring, with a pointer comment at `Compare`).
- The scaled product and the outcome read stay unpinned and unrefused — the
  doctrine forbids minting a corpus-zero pin for either; `badCompareLiteralSubject`
  still holds.
- The announcing subject and the announcing bound are answered in one change:
  the left side opens (`readAmount (LetterVal _) = True`) and negation stops
  dropping announcements (`dropGaps`).
- No `ProjAxis` row is added; each candidate axis is recorded against its own
  family in the Ledger, and the cards-in-hand refusal is not reversed.
- The mentioned complement lands as a second slot shape, with no new fold
  machinery. (`StatOf` carries no Soulblast comment in this tree — see the
  Re-audit table — so that clause is void, not satisfied.)
- The element binder reaches into the domain's own predicate; the relativized
  cards that do not land are named card by card in the Ledger.
- `CountersOn` gets NO `ProjAxis` row.
- Both choice tables stay quantity-blind; the counted-choice line that is
  writable is benched, and Duneblast's blocker is ledgered.
- One ordinal vocabulary serves the counter event, the cast restriction and
  `LibPos`. (The `LibPos` "ledger comment" named here does not exist in the
  tree — see the Re-audit table — so there is nothing to discharge.)
- `CounterBatch`'s two determiners are not extended into a third — the ordinal is
  its own thing.
- The counter event keeps `When`'s once-per-object reading; the trigger word
  stays the header's own slot and the ordinal wrapper does not touch it.
- The history identity read is refused with its reason recorded; Once Upon a
  Time is not counted as a payoff.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Design artifact: `docs/memory/scratch/amount-quantity-design.md`. Two ordered
sub-rounds, `idris/scripts/build` 19/19 after each. No experiment-log chapter
was written: the decision record is the docstrings plus this section.

### Sub-round A — the amount vocabulary and the comparison's columns

`jj diff --stat` at the A/B cut: 6 files, +404 / −5.

- **`Amount` gains six rows** (`Experimental.idr`), each with its
  `amtDelta`/`amtIntro`/`amtPlur`/`writtenBound`/`readAmount` companions:
  `Devotion` (the mana-symbol count [CR#700.5], gated to one possessor only —
  the colour pair is ungated, see the deviations), `Half` (a `RoundMode` slot, generalising
  the one prior `DamageScale.Halved` site), `DifferenceBetween` (the symmetric
  margin; the directional `Minus` is untouched), `EventSum` (`EventCount`'s
  numeric twin, gated by the new `eventHasMagnitude`), `AggregateOf` (the fold
  over a group MENTION — a second slot shape, no new fold machinery), and
  `AggregateOver` (the element binder: the domain binds one member with
  `bindFor TheD OneOf` for the body to read back as `It`/`They`).
- **`eventHasMagnitude`** (`Events.idr`) answers all 28 `EventName`s: damage and
  life gain/loss carry a number, everything else is a transition or an act.
- **The comparison's left side opens**: `readAmount (LetterVal _) = True`, on the
  ticket's own distinction — an announced X reads the value its announcement
  fixed [CR#107.3a], where a bare numeral states arithmetic. `ReadAmount`'s only
  consumer is `CompareAmt`'s `rd` gate (whole-tree grep: 1 site), the ticket's
  stated precondition for widening it. `badCompareLiteralSubject` still refuses.
- **The bound column's verdict is recorded, not gated**: a docstring on
  `CompareAmt` and a pointer comment at `Compare`.
- **Negation stops dropping announcements**: `condDelta (NotCond c) =
  dropGaps (condDelta c)` — a target written inside "unless [comparison]" is
  announced at casting like any other [CR#601.2c], and only the `Gap` binding
  goes, because a comparison that did not hold leaves no margin.
- **The asymmetric definition is a telescope, not a constructor**: `NamedNumber`
  joins `OutcomeSort` with `outcomeIsQuantity NamedNumber = True`, and
  `staticIntro (DefinesPt …)` introduces `outcomeB NamedNumber`, so a sibling
  slot reads it back as `ThatMuch` ("that number").
- **`PrintedMinusStar`** joins `PrintedStat` (and `starred`).

Bench (`Cards.idr`), all oracle text verified against MTGJSON before landing:
`karametrasAcolyte`, `anaxPowerDefinition`, `grayMerchantDrain`,
`devotionCondition` (Erebos, God of the Dead), `aspectOfWolf`, `jawsOfDefeat`,
`defilingDaemogothDrain`, `skullsporeNexusTrigger`,
`greatestCreaturesAPlayerControls`, `greatestArtifactsAnOpponentControls`,
`lhurgoyfDefinition`, `shapeshifterBox`, `multipleChoiceFirstArm`,
`multipleChoiceFourthGate`, `fellTheMighty`, `birthingPodSearch`.

Pins (`ProofsG.idr`): `badPluralDevotion`, `badDeathSum`,
`badSingularAggregateOf`, `badAggregateOfWrongSort` — each naming the rule that
makes its term meaningless, none justified by a count.

### Sub-round B — the ordinal word and the quantity's bindings index

- **One ordinal vocabulary**: `LibOrdinal` becomes `Ordinal` (same `Nth`, same
  [CR#401.7] refusal) with `LibOrdinal` kept as a type alias, so every existing
  use site and `badZerothFromTop` compile unchanged. (No `LibPos` ledger comment
  existed to discharge; see the Re-audit table.)
- **`NthOccurrence`** wraps any `GameEvent` (`eventName`/`eventIntro`/
  `eventAfter`/`eventSubjectPlur` read through it; those four are the only total
  matches over `GameEvent` in the tree). `CounterBatch` keeps its two
  determiners. The trigger word stays the header's own slot.
- **`Plan` and `Hour` counter kinds** join the catalog with their `counterScope`
  and `Eq` rows.
- **`Quantity` moves into `Experimental.idr`'s mutual block and gains a
  `Bindings` index and `UpToOf`** — the ceiling that is a written amount. The
  four literal gates (`NonZeroQ`, `WellFormedQ`, `quantPlur`, `modesFit`) answer
  the new arm whole-constructor and stay decidable; that is the recorded answer
  to the v1/v2 comparison's "would have to become runtime-undecidable" cost.
  `quantDelta` threads the bound's mentions at `TargetGroup`, `CountedGroup`,
  `SomeOf` and — because "choose up to X, where X is …" is attested (Bumi, The
  Ruinous Wrecking Crew, and the modal-spell rider) — at `Modal`'s
  `effIntro`/`preIntro`/`annIntro`, which previously dropped the headcount's
  letter. Two constructs are gated literal (`quantLiteral`): the results table,
  because [CR#706.3a]'s own three forms are numbers, and the land allowance,
  because no printed line writes "up to [amt] additional lands" and the
  statement introduces no mention of its own — that widening waits on a printed
  line.
- **The counter-header count** the ordinal serves: all 11 printed headers write
  `When`, not `Whenever`. That is a corpus measurement, so it lives here and not
  in `NthOccurrence`'s docstring; the trigger word stays the header's own slot
  and the ordinal wrapper does not constrain it.

Bench: `wavebreakHippocamp`, `midnightClockHeader`, `politicalTriumphHeader`,
`runThePlayCounters`, `berserkersFrenzyLowRoll`.
Pin: `badAmountRollRow`.

Must-not-regress, all still refusing (a pin that stopped refusing would fail the
`impossible` clause and break the build): `badCompareLiteralSubject`,
`badZeroGroup`, `badDescendingRange`, `badModalOneMode`, `badModalOverreach`,
`badZerothFromTop` (now pinning the shared `Ordinal`). Every literal-quantity
witness typechecks unchanged.

### Deviations from the design artifact

- **`devotionColorsOk` and `badSameColorDevotion` were built, then deleted**
  (conductor ruling, 2026-08-24). [CR#700.5]'s pair reading computes over the
  symbols that are "[color 1], [color 2], or both colors", which is well defined
  when the two names coincide — it is then the single-colour count. So "devotion
  to black and black" is CR-*meaningful*, the pin was a measured zero dressed as
  a presupposition, and
  `docs/memory/rulings/measurements-live-in-pins.md` forbids it. The pair slot is
  ungated and the ruling is recorded in one line on the `Devotion` row.
- **[CR#119.3] and [CR#120.1] were the wrong cites** for `eventHasMagnitude` and
  were replaced after reading the rule text: [CR#119.3] says only that a life
  total is "adjusted accordingly", and [CR#120.1] says only who can be dealt
  damage.
  Landed as [CR#120.8] (a 0-damage deal is no damage event), [CR#119.9] (a
  0-life gain is no life gain event) and [CR#119.2] (a damaged player loses
  "that much" life).
- **`condDelta (NotCond c)` has a second consequence the design did not name.**
  `condDelta (Matches (AsType t This _) _)` also survives negation now, which
  broke two macros. `unlessSo` was retyped to take its static effect at
  `condIntro (NotCond c)`; `monstrosity`'s gate moved from `Matches thisCreature`
  to `Matches This`, which is what [CR#701.37a] actually writes ("If this
  permanent isn't monstrous"). Both are improvements, not workarounds.
- **Doran, Besieged by Time is not the `DifferenceBetween` witness.** Its real
  header is "Whenever a creature you control attacks or blocks", and
  `triggeredOr`'s body is typed at `bs` rather than at the event's after-context,
  so the rider's "it" has no antecedent. Jaws of Defeat — "Whenever a creature
  you control enters, target opponent loses life equal to the difference between
  that creature's power and its toughness" — carries the same surface with a
  single-event header and is the witness instead.
- **Soulblast is not the `AggregateOf` witness.** Its sacrifice is an additional
  cost ("As an additional cost to cast this spell, sacrifice all creatures you
  control"), and the additional-cost frame is unbuilt. The Skullspore Nexus —
  "Whenever one or more nontoken creatures you control die, create a green Fungus
  Dinosaur creature token with base power and toughness each equal to the total
  power of those creatures" — is the witness. It required one catalog row,
  `Fungus`, with its `subtypeType` and `Eq` companions.
- **Death Denied writes "Return X target creature cards", not "up to X".** The
  `UpToOf` witness is Run the Play's first clause instead.
- **The design's `badAggregateOfWrongSort` complement was unelaborable** in the
  empty context (`thoseVerbed` needs a prior plural sacrifice mention), so the
  pin reads "the greatest life total among all creatures" over `AllOf` instead.
  The sort mismatch it pins is unchanged.
- **Cephalopod Sentry is */5, not "7-*".** The subtracted-star card is
  **Shapeshifter** (*/7-*, "its toughness is equal to 7 minus that number"), and
  that is the box the witness spells.
- **The Skullspore Nexus witness reads `Those CardW`, not `Those (TypeW
  Creature)`**, because `eventAfter (Dies …)` moves the mention to the graveyard.
  The card-side spelling of a death's own mention is a standing gap, noted at the
  witness; it is not this row's.

### Ledger

Named, not built, with the exact blocker:

- **Duneblast — "Choose up to one creature. Destroy the rest." (oracle text
  verified)** — NOT landed. `TheRest`'s presupposition is a partitioned group,
  and a choice mints none: neither `countGroups` nor `countParts` sees a choice
  clause. Landing it needs either a new `TheRest` licensor over the world's
  creatures or a different row, which is a ruling this round did not have.
  Berserker's Frenzy's counted choice IS landed (`berserkersFrenzyLowRoll`),
  which is the half of the ticket's pair that the tree already admitted.
- **Boreas Charger / Sandstone Oracle / Slithermuse** — deferred whole. "Choose
  an opponent who controls more lands than you … the difference" needs a
  predicate-level member-relative comparison AND a gap-licensing choice frame.
  Neither exists; building half of it lands none of the three.
- **Once Upon a Time's history identity read** ("if this spell is the first spell
  you've cast this game") — neither built nor pinned. No rule makes it
  meaningless, so a pin would be a doctrine violation; no buildable card pays it
  (the rest of that card — look at five, reveal, bottom in a random order — is
  unbuilt besides). Recorded as a ledger, not a refusal.
- **`AggregateOver`'s ten relativized cards, card by card.** The binder itself is
  landed and benched twice at the phrase level (Investigator's Journal's count,
  Cavern-Hoard Dragon's cost rider). Whole cards still blocked:
  - *Investigator's Journal* — additionally needs a `Suspect` counter kind
    ("enters with a number of suspect counters on it equal to …"), outside this
    ticket's counter scope.
  - *Thought Sponge* — "the greatest number of cards an opponent has drawn this
    turn" needs an `EventCount` INSIDE the binder body, relativized to the bound
    member; the body admits an `Amount` but no per-member event subject exists.
  - *Windfall / Jace's Archivist cycle* — "cards a player discarded this way"
    inside the binder: same per-member event-mention gap.
  - *Cavern-Hoard Dragon* — the phrase is landed; the whole card needs the
    cost-reduction rider's `{X} less` frame.
- **The definition-side double-blocks stand.** Eight of the asymmetric twelve
  (incl. Tarmogoyf) count card TYPES among cards — a distinct-kind count
  `CountOf` does not spell — and five of those eight also write the
  all-graveyards possessor. The Lhurgoyf witness spells its zone as the bare
  graveyard zone with a spelling note; the possessor question is untouched.
- **`ProjAxis` gained no rows**, as the ticket required. Each candidate rides its
  own family: votes → the will-of-the-council family; the colour share → a
  quality-domain fold (a third sort beside object and player) that nothing here
  builds; the noted number → named memory channels; cards in hand → chapter
  fifty-six's `PlayerStat` refusal, NOT reversed. `CountersOn` gets no row.
- **`writtenBound` has no consumer** (signature + rows only, whole-tree grep).
  Likely residue of a removed bound gate. Left untouched; conductor decides
  deletion.
- **`Aggregate` is now a special case of `AggregateOver`.** `Aggregate op ax p`
  folds `ax` over the members of `p`; `AggregateOver op p body` folds `body` over
  the same members with one bound. Writing the axis read as the body recovers
  the older row exactly, so the vocabulary now has two spellings for one meaning.
  Nothing here dedups them — every existing `Aggregate` witness would have to be
  re-spelled and the extremal modifier's own consumers re-read — but a future
  round should collapse the pair rather than grow both.
- **Three devotion cards stay unspellable**, per the shortfall-naming rule:
  *Nykthos, Shrine to Nyx* and *Nyx Lotus* both write "Choose a color. Add an
  amount of mana of that color equal to your devotion to that color" — the
  colour slot is an anaphor over a chosen quality, and `Devotion`'s slot is a
  literal `Chroma.Color`; a chosen-colour read is the quality-domain work, not
  this row's. *Altar of the Pantheon* writes "Your devotion to each color and
  each combination of colors is increased by one" — a modification of the
  devotion count across every colour combination at once, which is neither a
  read nor a slot this vocabulary has.
- **The scaled product and the outcome read as bounds** stay unpinned. Both are
  CR-meaningful, so under `docs/memory/rulings/measurements-live-in-pins.md` no
  pin may be minted for either; the ticket's `badScaledBound` /
  `badScaledConditionBound` never existed to begin with.
- **`EventSum`'s witness is Defiling Daemogoth** ("At the beginning of your end
  step, each opponent loses X life, where X is the amount of life you gained this
  turn", verified) — the family (51 distinct supported oracle lines across 50
  supported cards; 57 cards at `--all` scope) is no longer pin-backed only.
