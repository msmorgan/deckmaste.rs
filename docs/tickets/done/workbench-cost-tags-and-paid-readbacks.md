---
needs: [workbench-named-memory-channels]
---
# Make a paid cost readable later: the cost tag and its four consumers

Kicker, multikicker, buyback, escalate-as-a-condition, "if its madness cost was
paid" — every card whose later clause asks *whether, or how many times, an
earlier optional cost was paid*. The alternative-cost row itself is
[workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)'s;
this ticket is only the readback channel that ticket does not own.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 19 (2026-08-24) — graded **MISSING** and "Not recorded anywhere as scope".
Scope split verified against the cost ticket: it owns the five populations the
alternative-cost static refuses, the free-cast family, the keyword cost catalog,
the cost-statement pieces, cumulative upkeep, the cost gate and the scaled
payment. It does **not** mention `CostTag`, `PaidCost`, `TimesPaid`,
`WasPaidWith`, `WasCastWith` or kicker anywhere — that is this ticket.

## From the v1 comparison (2026-08-24)

> Crate: `CostTag(Ident)` (`cost.rs:190`) with four consumers —
> `OptionalCost { components, tag, repeatable }` (`cost.rs:253`),
> `Condition::PaidCost(CostTag)` and `CastWith(CostTag)` (`condition.rs:52`),
> `Count::TimesPaid(CostTag)` (`count.rs:167`),
> `StatePredicate::WasPaidWith`/`WasCastWith` (`filter.rs:88`), plus
> `DeonticAction::Cast { cost: Option<AlternativeCost>, tag: Option<CostTag> }`
> (`deontic.rs:231`) and `AlternativeCost { Free, Components }`
> (`deontic.rs:27`). `StaticEffect::CostOption(OptionalCost)`
> (`continuous.rs:282`) is how a card declares one.
>
> Workbench: grep for `CostTag`, `WasPaidWith`, `TimesPaid`, `PaidCost`,
> `Kicker` across `Experimental.idr` and `Experimental/*.idr` returns **zero
> hits**. The only alternative-cost surface is `StaticEffect.AltCost : (c :
> Maybe (Cost bs)) -> {auto 0 ap : AltPayment c}` (`Experimental.idr:2668`) — a
> bare "you may pay this instead", untagged, so nothing downstream can read
> whether it was paid. Kicker, multikicker, buyback, escalate-as-a-condition,
> "if its madness cost was paid" — none have a v2 spelling.
> `ModalCostRider { Entwine(Cost), Escalate(Cost) }` (`ability.rs:158`) likewise
> has no counterpart.

## Why this needs the memory ruling first

`CostTag(Ident)` is a ninth `Ident`-keyed channel beside the eight in
[workbench-named-memory-channels](workbench-named-memory-channels.md), and it
runs into the same binder-contract clause 1 for the same reason: the tag is a
name minted by a cost and read by a clause that is not its child. Designing a
keyed cost tag before that ruling either pre-empts it or contradicts it. If the
ruling goes the source-anchored way, this channel is anchored to the *cost
component* that declared it and the tag is spelling; if it goes the carve-out
way, the tag is an ordinary keyed mint. Either way the shape falls out of the
ruling — hence the `needs:`.

Note that the printed English usually names the tag with the keyword itself
("if this spell was kicked", "for each time it was kicked"), which is the same
argument the memory ticket's option A rests on. Bring the measurement to that
ruling rather than re-litigating it here.

## What the round owes beyond the channel

- Whether the tag's declaration site is a static (`CostOption`'s shape) or a
  rider on the cost, and whether repeatability is a flag on the declaration or a
  property of the read (the crate puts `repeatable` on the declaration and the
  count on the read).
- Whether "cast with" and "paid with" are one relation or two — the crate has
  both a condition pair and a predicate pair.
- `ModalCostRider`'s entwine/escalate: whether a modal cost rider is this
  family or the modal clause's, decided once.
- Measure each surface before minting: an untagged optional cost with no later
  reader needs nothing from this round, and `AltCost` already writes it.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental.idr` (`AltCost`,
`AltPayment`, `Cost` and its components, the condition frame, the `Amount`
reads, the description frame's predicates, the modal clause's riders),
`idris/src/Experimental/Words.idr` (the tag's vocabulary and the keyword catalog
row it usually shares), `idris/src/Experimental/Events.idr` if the read is a
lookback, the pin modules `idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate. The alternative-cost row, the
free cast and the keyword cost columns stay with
[workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md);
this round must not widen them.

## Acceptance

- The tag's shape follows the memory-channel ruling rather than restating the
  question; no second keying convention is invented here.
- A kicked spell reads its own payment back and at least one kicker card and one
  multikicker card bench, or the shortfall is named card by card.
- "Cast with" and "paid with" are settled as one relation or two, with the
  reason recorded where the read is defined.
- The declaration site is decided once and the repeatable/count split is stated
  where both halves are defined.
- The cost ticket's landed rows are unchanged; this round adds a readback and
  does not re-open the alternative-cost static.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-counter-distributive-residues (close, 2026-08-26):** "If life was paid" (Nahiri, the Unforgiving) reads back a CAST-TIME payment; no condition row reaches the payment channel. It is a paid-readback, so it lands here.

- **Shape note (2026-08-26):** unblocked — `workbench-named-memory-channels` closed with the no-carve-out ruling: readbacks here are LABEL-SORTED ANCHORED READS on the spell/permanent (state, like counters), no tag namespace. See that done ticket's As-landed.

## As-landed (2026-08-26)

**No tag namespace, as the memory ruling dictates.** A paid optional cost is
STATE the object carries — [CR#707.2] copies "whether it was kicked" with the
spell's other casting choices, and [CR#702.152a] reads the same state off the
permanent the spell became — so the readback is an ANCHORED read on the object,
sorted by the cost's printed name. [CR#607.2i] is the defining rule and it
supplies the sort in its own last sentence: "Each of those abilities will
specify which cost it refers to." Nothing mints a name and nothing matches one.

### The measured family

Distinct oracle-text lines over 35,961 cards (`data/derived/cards.jsonl`):

- **254** lines write "kicked", in 250 distinct clause forms: **219** plain
  boolean ("if it/this spell/this creature was kicked"), **25** label-qualified
  ("kicked with its {1}{U} kicker"), **19** count reads ("the number of times …
  was kicked" 7, "for each time … was kicked" 12), **12** description filters
  ("whenever you cast a kicked spell"), **3** negated ("wasn't kicked").
- **38** lines write "[X] cost was paid". Label vocabulary, confirming it is
  OPEN and not an enum: additional 13, madness 4, prowl 4, sneak 4, surge 3,
  spectacle 2, emerge 1, freerunning 1, mayhem 1, and **5** naming the cost by
  its printed mana symbols instead of a keyword (the Mastery cycle, "If the
  {1}{B} cost was paid"). Two negated (Katara "unless", Ingenious Mastery "if
  that cost wasn't paid"); one turn-scoped (Karai, "if her sneak cost was paid
  this turn").
- **1** line writes the alternative-cost spelling of the same read — Full Bore,
  "if that creature was cast for its warp cost".
- **0** lines write "was cast with [a cost]". All 19 "cast with" lines are
  description filters over mana value, power, watermark or name, so the crate's
  `CastWith`/`WasCastWith` has no corpus payer as a payment relation.
- **buyback 16 / entwine 19 / escalate 7** lines are keyword declarations plus
  reminder text. **Zero readbacks.** The ticket's premise that escalate and
  buyback are readback consumers is not borne out.
- **7** "life was paid" lines: 6 are compleated reminder text ([CR#702.150a]
  puts the whole meaning inside the keyword), 1 is Verrak.

### What landed

`Experimental/Words.idr`
- `keywordCosts : KeywordLabel -> Bool` — one spelling of "this word's rule
  writes a cost"; `Events.idr`'s `KeywordCost` now reads it instead of
  restating it.
- `data PaidCostName = ByKeyword KeywordLabel | ByNthKeyword Ordinal
  KeywordLabel | TheAlternative`, with `Eq`, `paidCostNamed`/`PaidCostNamed`.
  The ordinal arm is [CR#702.33f]'s own definition of "with its [A] kicker" —
  A and B are the first and second kicker costs LISTED on the card, so the
  printed cost is spelling and the discriminator is the print order. That also
  keeps the sort layer-clean: `Cost` lives in `Effect.idr`, below `Phrase.idr`,
  so a predicate could not carry one.
- `keywordFacts` rows for `Kicker` and `Multikicker` (CostParam, AtCasting,
  permanent and spell cards) — [CR#702.33a]. The minimal declaration seat; the
  card writes `Kicker {4}{G}` through the existing `keywordCosting`, so no new
  declaration constructor was needed.

`Experimental/Phrase.idr`
- `PaidCost : (which : PaidCostName) -> Predicate bs Object` — the boolean read,
  a predicate and not a condition, exactly as counters are read: `Matches`
  lifts it to a condition, `Not` negates it, and a description uses it directly
  ("a kicked spell"). Zone ungated.
- `TimesPaid : PaidCostName -> Noun bs Object -> Amount bs` — the count read, on
  `CountersOn`'s model.
- Their rows in `seedZone`, `hasHead`, `predEq`, `predSays`, `predNegFree`;
  `amtDelta`, `amtIntro`, `amtPlur`, `writtenBound`, `readAmount`.

### The three questions the ticket asked, settled

- **One relation, not two.** The rules write the identical readback for an
  additional cost ([CR#702.27a] buyback) and for an alternative one
  ([CR#702.34a] flashback), and the corpus writes both as "[keyword] cost was
  paid". Which kind a keyword offers is that keyword's own fact, never the
  reading clause's, so `PaidCost`/`CastWith` collapse to one row. Recorded on
  `PaidCost`.
- **The declaration site is the keyword ability.** `keywordCosting "Kicker" c`
  already existed; the round added data rows, not a `CostOption` static. The
  un-keyworded additional cost is the one declaration still missing and is
  routed to the cost ticket.
- **Repeatability is a property of the READ, and not a flag anywhere.**
  [CR#702.33d] makes the number bigger than one by the cost the card declared
  ("two kicker costs or … multikicker"), so no `repeatable` flag exists; the
  count reads at every paid-cost name and reads 0 or 1 where one payment was
  offered — tolerated, as `GreatestStoredMatch` tolerates a permanent that
  stored nothing.
- **`ModalCostRider` is not this family.** Entwine and escalate have no
  readback in the corpus at all; routed to the cost ticket as cost declarations.

### Benched

- **Krosan Druid** (whole) — "Kicker {4}{G} / When this creature enters, if it
  was kicked, you gain 10 life." The cheapest whole card in the family.
- **Lightkeeper of Emeria** (whole) — the count read, with `Multikicker`
  declared and `Kicker` read ([CR#702.33c] makes them one cost).
- **Merfolk Falconer** (whole) — the read inside a DESCRIPTION, "whenever you
  cast a kicked spell".
- **Ertai's Trickery** (whole) — the read on a spell the clause just targeted,
  under `OnlyIf`; nothing ties the read to the text that carries it.
- **Stormscape Battlemage's first kicker trigger** (ability) — the ordinal arm,
  and [CR#607.2i]'s own worked example. The card does not write whole: its
  second trigger needs a regeneration ban.
- **Baleful Mastery's paid read** (ability) — the `TheAlternative` arm. The card
  does not write whole: "exile target creature or planeswalker" needs a joined
  target this round did not open.

Pins: `badPaidCostOnCostlessKeyword` (a word whose rule writes no cost names
nothing payable) and `badTimesPaidUnknownKeyword` (fail-closed on a misspelling)
in `ProofsG.idr`.

### The routed item: "If life was paid" (Nahiri, the Unforgiving)

**Not a readback the grammar owes.** Nahiri prints "Compleated" and nothing
else; [CR#702.150a] puts the entire replacement — "if the player who cast it
chose to pay life for any part of its cost represented by Phyrexian mana
symbols … minus two for each of those mana symbols" — INSIDE the keyword, and
the "If life was paid" sentence is reminder text. All six compleated lines are
the same. The card needs a `Compleated` row in `keywordFacts` and no readback
constructor; `PaidCost` correctly refuses it, because compleated names no cost.

The corpus's one genuine life-payment readback is **Verrak, Warped Sengir** —
"Whenever you activate an ability that isn't a mana ability, if life was paid to
activate it, you may pay that much life again." `PaidCost` does not reach it and
should not: it sorts by no cost NAME (any cost that asked for life, [CR#118.1])
and it is anchored to an ABILITY rather than an object, while `PaysLife` is an
event header rather than a state read. Routed to the cost ticket.

### v1 spot-check (obligation from the memory ruling)

Every shape the parent ticket quoted is real; only the line numbers had drifted.
`CostTag(pub Ident)` is `cost.rs:721` (quoted 190), `Condition::PaidCost`/
`CastWith` are `condition.rs:108,113` (quoted 52), `Count::TimesPaid` is
`count.rs:278` (quoted 167), `StatePredicate::WasPaidWith`/`WasCastWith` are
`filter.rs:126,131` (quoted 88). `filter.rs:130` already carries a comment
reading "Idris `WasCastWith`" — aspirational, and this round declines it on the
measurement above. The memory ruling's second obligation does not bind: no
"note"-family row landed here.

### Left for workbench-cost-and-payment-residues

The un-keyworded additional cost declaration and its fourth `PaidCostName` arm;
the keyword cost catalog rows (24 words named on that ticket); `ModalCostRider`;
Verrak; and Karai's turn-scoped "if her sneak cost was paid **this turn**"
(1 line) — `PaidCost` carries no window slot. All five are routed onto that
ticket.
