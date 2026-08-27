---
needs: []
---
# Payment-event residues: passive headers, life payments, repeated offers

Routed from `workbench-payment-events-and-replacement-disjunction` (close,
2026-08-26), which landed `PaysCost` (polarity slot, `CostPayment`/
`CostNonpayment`) and benched Thought Lash whole plus Heart of Bogardan's and
Hibernation's End's headers. Three measured remainders of the 12-card
payment-header family:

1. **The passive payment header** — "…'s cumulative upkeep is paid"
   (Balduvian Fallen, Shah of Naar Isle): the subject is the COST, not a
   player; also `Echo` is not in `Keyword`.
2. **A life payment** (Font of Agonies): a non-keyword paid thing, and the
   tail's "that many" wants a magnitude the landed row does not announce.
3. **Repeated payment of an offered cost** — "pay this cost one or more
   times" (the 5 Adversaries, Tranquil Frillback): a cost anaphor plus a
   repetition count.

Scope fence carried over: cost-SIDE constructions (what a cost is, how it is
paid) belong to workbench-cost-and-payment-residues; this ticket owns only
what a trigger header may watch.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`; `Words.idr` for the
keyword catalog row; `Cards.idr` bench; `Proofs*.idr`. No Rust crate.

## Acceptance

- Each numbered item lands or ends in a written rule-backed verdict; the
  named carriers bench their headers or are named at their exact blocker.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Two of the three items landed; the third ends in a verdict that moves it off
the event seat entirely.

### The family, re-measured

The parent's 12 supported payment-header cards are confirmed: over the same
supported corpus, `\b(When|Whenever)\b[^,.]{0,80}\b(pays?|paid)\b` returns 13
lines — the 12 carriers plus Agent Maria Hill, the near-miss the parent already
excluded. Widening to the unsupported corpus adds two digital-only lines that
name a third and fourth kind of paid thing: Spare Changeling's "Whenever you pay
an additional cost while casting a spell" [CR#118.8] and Tax Taker's "Whenever
an opponent pays a tax", whose reminder text defines its own term. Neither is
in the supported corpus; noted here only so the next widening does not read them
as new.

One phrasing in the round brief did not survive measurement: the repeated-payment
tail is **"that many"**, not "for each time it was paid". The only "for each time
you paid" in the corpus is replicate's reminder text [CR#702.56a], a different
construction.

### Verdict 1: the passive payment header is a VOICE on `PaysCost`, not a
second row. Landed.

`PaysCost`'s payer became `Maybe (Noun bs Player)`, with the bearer scoped
through a new `payerIntro` on `mayCtx`'s shape. Signature:

```idris
PaysCost : (who : Maybe (Noun bs Player)) -> (out : PaymentOutcome) ->
           (whose : Noun (payerIntro who) Object) -> (kw : Keyword) ->
           {auto 0 kc : KeywordCost kw} ->
           {auto 0 one : nounPlur whose = OneOf} -> GameEvent bs
```

The grounding is that the passive omits a payer the rules have already fixed:
[CR#702.24a] and [CR#702.30a] both write "you" of the permanent's controller, so
the same event happened and nothing about it is left indeterminate by the
omission. A second row would have to name the same `EventName`, take the same
keyword-and-bearer cost naming and the same [CR#118.10] singular gate, and
differ only in whether one slot was written — which is what a slot is for. What
the omission does cost is the ANNOUNCEMENT, and that difference is now pinned
rather than asserted (`badPassivePayerReadback`).

- `eventName`/`eventIntro`/`eventAfter` are unchanged: they never read the
  payer slot, only the bearer's bindings, which sit above it either way.
- `eventSubjectPlur` splits: voiced, the payer's plurality; unvoiced, `OneOf`,
  because the surface subject is then "[whose]'s [keyword]", whose head is the
  bearer the [CR#118.10] gate already made singular.
- Both outcomes take either voice. A passive nonpayment ("…'s cumulative upkeep
  isn't paid") is unattested and refused by no rule — [CR#702.24a] fixes the
  payer whether or not the clause names them — so it is tolerated overgeneration,
  named at its zero.

`Echo` joined the `Keyword` catalog with `CostParam` [CR#702.30a], plus its rows
in `Eq Keyword`, `keywordCounterOk` (False — [CR#122.1b]'s closed list),
`keywordStackRegime` (Nothing) and `keywordCardOk` (permanent only: [CR#702.30a]
speaks of "this permanent" and [CR#110.4] denies an instant or sorcery card ever
being one). One enum row, as the brief directs; the open-label conversion stays
with workbench-keyword-labels.

### Verdict 2: a life payment is its own row, and the one payment that
announces a number. Landed.

```idris
PaysLife : (who : Noun bs Player) -> GameEvent bs
```

Its own row and not a `PaysCost` voice, because it names no cost at all: any
payment of life, towards whatever cost asked for it, is the event [CR#118.1].
That is also why its `EventName` had to be separate — `eventHasMagnitude` is
keyed on the name, and the two payments answer it differently:

- `eventHasMagnitude CostPayment = False` stands. The size of a keyword-named
  cost is the cost's own, and [CR#702.24a] refuses a partial payment outright.
- `eventHasMagnitude LifePayment = True`. [CR#118.3b] pays life by subtracting
  the indicated amount from a life total and [CR#119.4] reads that back as
  losing that much life, so this payment happens IN a number.

`eventAfter (PaysLife who) = outcomeB LifeLost :: nomIntro who` — the mint is
[CR#119.4]'s own life loss, not a second `OutcomeSort` naming the same number,
and `ThatMuch` reads it. `lookbackSubjectOk LifePayment Player = True`: a life
payment names its own paid thing, so a bare "paid life this turn" leaves nothing
unnamed, which is exactly where `CostPayment` fails — the participial lookback
carries no complement that could say WHICH cost.

`Blood` joined `CounterKind` as an ordinary marker [CR#122.1] so the carrier
benches whole.

### Verdict 3: "pay this cost one or more times" is not a watchable event at
all. Not minted, and the shape decided.

[CR#603.12] makes "When you pay this cost one or more times" a REFLEXIVE
triggered ability: it is checked against whether the enclosing resolution's own
action occurred, not against a game event, and its seat is `Reflexively`, which
takes an `Effect` and never a `GameEvent`. [CR#603.12a] names this exact
construction in the rules' own words — "if a resolving spell or ability includes
a choice to pay a cost multiple times and creates a triggered ability that
triggers when that payment is made, paying that cost one or more times causes the
reflexive triggered ability to trigger only once." So no `GameEvent` row is owed
here, the cost anaphor is not an event's slot, and widening `PaysCost` to carry
a repetition would be building the wrong seat.

All six carriers write one shape — "When this creature enters, you may pay
[cost] {any number of times | up to three times}. When you pay this cost one or
more times, …" (the five Adversaries and Tranquil Frillback). Its two blockers,
exactly:

1. **The enclosure, and it is cost-side.** `Reflexively`'s body would be
   `May (Just You) (<the payment, repeated>) Nothing Nothing`, and no term
   spells a repeated payment offer: `Pay` and `Cost` carry no repetition,
   `Repeated` takes a definite count `Amount` (neither "any number of times" nor
   "up to three times"), and `Repeat AnyNumber` is the standalone "repeat this
   process" sentence. Building it is a question about what a cost is and how it
   is paid — the scope fence puts it in
   `workbench-cost-and-payment-residues`, and this ticket declines it there.
2. **One seat row, which cannot be flipped without a witness.**
   `reflexEncloseUse (Repeated _ _) = EncNotOneAction` refuses the enclosure even
   once it exists, and [CR#603.12a] says otherwise for a repeated cost payment
   specifically. The row is the seat's and so is this ticket's, but flipping it
   wholesale would over-claim — the rule's exception is narrow to paying a cost,
   not to repetition in general — and a narrowed row has no whole-card witness
   until blocker 1 clears. Recorded, not built.

A third, smaller item rides along: the printed surface restates the action
("When you pay this cost one or more times") where `Reflexively` spells "When you
do". That is a spelling slot on the reflexive seat, and it follows the enclosure.

### Landed

- `Experimental/Words.idr` — `Echo` on `Keyword` and its four table rows;
  `Blood` on `CounterKind` with `counterScope` and `Eq`.
- `Experimental/Card.idr` — `keywordCardOk` for `Echo`.
- `Experimental/Events.idr` — `LifePayment` on `EventName`, with rows in
  `sameEventName`, `eventHasMagnitude`, `lookbackSubjectOk` and
  `lookbackComplementOk`.
- `Experimental/Triggers.idr` — `payerIntro`; `PaysCost`'s voice slot;
  `PaysLife` and its four table rows.
- `Experimental/Cards.idr` — `balduvianFallenHeader`, `shahOfNaarIsleHeader`
  (headers alone; blockers below), `fontOfAgoniesTrigger` (whole ability).
- `Experimental/ProofsG.idr` — `badBareEcho`, `badPassivePayerReadback`,
  `badKeywordCostPaymentThatMuch`, and the standing positive
  `youPaidLifeThisTurn`.

### Carrier status

- **Font of Agonies** — benches its whole ability, header and tail.
- **Balduvian Fallen** — benches its header. Its body does not: "it gets +1/+0
  until end of turn for each {B} or {R} spent this way" counts the mana that paid
  the cost, and no phrase names mana by what it was spent on. Card-level, and
  the mana-spent read is cost-side.
- **Shah of Naar Isle** — benches its header. Its body does not: "each opponent
  may draw up to three cards" wants a ceiling on the drawn count, and "up to
  [n]" is a `Quantity` over a described set with no `Amount` twin. Card-level.
- **The five Adversaries, Tranquil Frillback** — blocked at verdict 3's two
  blockers; nothing benched.

### Ledger

- **A repeated payment offer** — `workbench-cost-and-payment-residues`: a
  `May`-borne `Pay` repeated "any number of times" / "up to three times", with
  the `RepeatCount` mint the tail's "that many" reads. Verdict 3's blocker 1.
  [CR#702.56a] writes the same offer in the rules' own words — replicate means
  "you may pay [cost] any number of times" — so the construction has a rules
  statement to build against and is not read off the six cards alone.
- **`reflexEncloseUse` for a repeated cost payment** — [CR#603.12a] admits the
  reflexive trigger this row refuses; narrow it when the enclosure above exists
  to witness it. Verdict 3's blocker 2.
- **The reflexive seat's restating spelling** — "When you pay this cost one or
  more times" beside "When you do". Rides with the two above.
- **An amount ceiling** — "draw up to three cards": `UpToOf` is a `Quantity`
  over a described set, and no `Amount` spells a bounded number. Shah of Naar
  Isle's body.
- **Mana named by what it was spent on** — "for each {B} or {R} spent this way".
  Balduvian Fallen's body; cost-side.
- **An additional cost as a paid thing** — Spare Changeling, "Whenever you pay
  an additional cost while casting a spell" [CR#118.8]: a cost CLASS narrowed by
  a window rather than a named cost. Out of the supported corpus today; a live
  item only if the corpus widens.

### Gates

- `idris/scripts/build` — 23/23, 0 errors, 0 warnings.
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 17,081 citations, 0 stale; [CR#702.56a] blessed and
  read against its claim.
- `jj diff --git | cargo xtask cite audit --diff` — every site read against its
  rule.
