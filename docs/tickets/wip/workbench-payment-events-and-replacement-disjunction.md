---
needs: []
---
# Row a payment event, and decide the replacement's event disjunction

The two STOP items `docs/tickets/done/workbench-event-algebra.md` reported
rather than acted on. Both are event-vocabulary questions the algebra round
measured and deliberately left unminted; neither has an owner.

## 1. No `GameEvent` row names a payment

Heart of Bogardan and Thought Lash both write:

> "When a player doesn't pay this enchantment's cumulative upkeep, …"

The event-algebra round found these while testing whether a general `NotEv`
negation operator was attested. Its verdict, quoted:

> They do not, however, buy `NotEv`: the negated thing is a **cost payment**,
> and this vocabulary rows no payment event at all, so a general negation
> operator over existing rows would spell neither line. Recorded as a
> vocabulary gap, not an operator one.

So the ask is the positive row first — an event naming a payment — and only then
whether its failure arm is a second row, a polarity on that row, or the general
operator the algebra round declined to mint. The two carriers are the evidence;
size the family before choosing a shape.

Neighbour, so nothing is built twice: cumulative upkeep's **cost-side**
constructions belong to
[workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)
(its "Cumulative upkeep's cost-side residues" section). This ticket owns only
the event side — what a trigger header may watch.

## 2. Illusionary Mask's three-way event disjunction under a replacement's `would`

The one attested non-header event disjunction:

> "If the creature that spell becomes as it resolves has not been turned face up
> and **would assign or deal damage, be dealt damage, or become tapped**,
> instead it's turned face up and assigns or deals damage, is dealt damage, or
> becomes tapped."

The algebra round retired its own "zero non-header attestation" claim on this
line and still minted no row, for two reasons that stand and must be answered
rather than re-derived:

- every non-header event reader goes through `eventName`, which a disjunction
  term would leave naming one of two events; and
- the line is a **three**-way disjunction, which the header's binary `AltEvent`
  could not spell even at a header.

So the decision is not "widen `AltEvent`". It is either an n-ary event
disjunction with an `eventName` story, or a ruling that this one line is spelled
some other way (or knowingly left unspellable, named at its zero). One card is
thin evidence for a general operator — say so if that is the answer.

## Consumption boundary

`idris/src/Experimental/Events.idr` (`GameEvent`, `EventName`, `eventName`,
`eventUse`, the complement tables), `idris/src/Experimental.idr` where the
trigger header and `Intercepts` read them, the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- A payment event row exists or its absence is argued from a rule; Heart of
  Bogardan and Thought Lash each bench their header or are named at their exact
  remaining blocker.
- The disjunction question has one written verdict covering the three-way arity
  and the `eventName` reader, not a widened binary.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

**Verdict 1: a payment event row landed, with polarity as a slot and not a
general negation.** `GameEvent.PaysCost` names a player, an outcome, the object
whose cost is meant, and the keyword that names that cost. The two outcomes
lift through `paymentEventName` to two `EventName`s, `CostPayment` and
`CostNonpayment`, on `flipEventName`'s model — [CR#118.1] makes paying a cost an
act a player carries out, [CR#118.12] reads a following "if [a player] doesn't"
as a check on whether that player chose to pay, and [CR#702.24a] writes exactly
that pair into cumulative upkeep; [CR#603.2] lets a header match a game event or
a game state, which is what admits the declined arm. The cost is NAMED (keyword
plus bearer), never spelled as a `Cost`: a header watches a payment, it does not
state one, which is the ticket's scope fence held in the type. [CR#118.10]
applies one payment to one cost, so the bearer is singular; its zone is ungated
because a keyword ability functions from the zones its own rule names
[CR#113.6b].

### The family, measured

Distinct supported oracle lines whose trigger clause watches a payment
(`\b(when|whenever) [^,]{0,70}\b(pays?|paid)\b[^,]{0,40},`): **12 cards.**

| shape | cards | landed? |
|---|---|---|
| active, keyword-named cost, paid | Hibernation's End | yes |
| active, keyword-named cost, NOT paid | Heart of Bogardan, Thought Lash | yes |
| passive, keyword-named cost, paid | Balduvian Fallen, Shah of Naar Isle | no — ledger |
| active, life, paid | Font of Agonies | no — ledger |
| active, cost anaphor + repetition count, paid | the five Adversaries, Tranquil Frillback | no — ledger |

Near-miss, deliberately excluded: Agent Maria Hill, "Whenever Agent Maria Hill
becomes tapped to pay a teamwork cost" — a `StatusEvent` narrowed by the tapping's
purpose, not a payment event.

So the negative is 2 of 12 and confined to cumulative upkeep, where [CR#702.24a]
supplies the decision that makes declining determinate. That is what buys a
polarity SLOT and refuses a general negation over events: `NotEv` would need a
second customer and has none — the only event-positional negations in the corpus
are these two lines, and they are one event's two arms. The algebra round's
verdict ("a vocabulary gap, not an operator one") is confirmed and closed.

Not taken, and not a duplicate: the large "you may pay {1}. When you do, …"
family is [CR#118.12]'s payment CHOICE read reflexively, whose seat is already
`Pay`'s `reflexEncloseUse = EncReflexive`. `PaysCost` watches a payment as an
event; that family checks a decision inside one clause.

### Landed

- `Experimental/Events.idr` — `CostPayment`/`CostNonpayment` on `EventName`;
  `PaymentOutcome` and `paymentEventName`; `KeywordCost` (the gate: a keyword
  whose parameter IS a cost). Rows added to every total table over `EventName`:
  `sameEventName`, `eventHasMagnitude` (both False — what is paid is the cost
  the clause names, whose size is the cost's own, and [CR#702.24a] refuses a
  partial payment outright), `lookbackSubjectOk` (all four False).
- `Experimental/Triggers.idr` — the `PaysCost` row and its four table rows
  (`eventName`, `eventIntro`, `eventAfter`, `eventSubjectPlur`). It announces
  the payer, which is what Thought Lash's tail reads back.
- `Experimental/Cards.idr` — `thoughtLashTrigger` (whole quoted ability: header
  plus "that player exiles all cards from their library"), `hibernationsEndTrigger`
  (whole quoted ability, the paid arm), `heartOfBogardanHeader` (the header
  alone; see below).
- `Experimental/ProofsG.idr` — `badPayCostlessKeyword` (a payment naming a
  keyword with no cost parameter; [CR#118.1] and [CR#702.9a]) and
  `badBarePaymentLookback` (a retrospective payment read; the participial
  lookback carries only a kind-to-kind complement and cannot name which cost).

### Carrier status

- **Thought Lash** — benches its whole trigger.
- **Heart of Bogardan** — benches its header (the same event term Thought Lash
  writes). Its BODY does not, and the blocker is exact: "deals X damage to
  target player or planeswalker and each creature that player or that
  planeswalker's controller controls" needs `splitOverPlaneswalker`, whose
  demonstrative requires `countWord PlayerW bs = 1`, and the header has already
  announced the non-payer as a second singular player mention. That is the
  demonstrative-uniqueness rule the `badIt`/`badThemAmbig` family names, and it
  is this CARD's blocker, not the row's.

**Verdict 2: the event disjunction is a SEAT slot, n-ary, and never a
`GameEvent` row. Shape decided; not minted, because its arms are not yet in the
vocabulary.**

Both of the algebra round's standing reasons are answered head-on:

- *The `eventName` reader.* `eventName` is total on `GameEvent`, so a
  disjunction ROW would have to name one of n events — the objection stands and
  is decisive against that shape. It does not reach a slot on the reader's
  seat, because no prospective seat needs one name for a coordination: the only
  name-keyed use on any of them is `Interceptable ev = So (interceptOk (eventName
  ev))`, and that distributes — every arm interceptable. `Delayed` and the
  header consult `eventName` not at all. That is the `eventName` story, and it
  is recorded on the `GameEvent` docstring so the next round need not re-derive
  it. The body reads the arms' whole-agreement discourse by the algebra round's
  own `sameBindings` rule, folded over the list instead of applied to a pair.
- *The arity.* Confirmed and generalised. The algebra round measured this as one
  three-way line; it is not. Re-measured over the same supported corpus, genuine
  event disjunctions with THREE arms are attested at three different seats:
  **Giggling Skitterspike** ("Whenever this creature attacks, blocks, or becomes
  the target of a spell"), **Trouble in Pairs** ("Whenever an opponent attacks
  you with two or more creatures, draws their second card each turn, or casts
  their second spell each turn") and **Syr Konrad, the Grim** ("Whenever another
  creature dies, or a creature card is put into a graveyard from anywhere other
  than the battlefield, or a creature card leaves your graveyard") at the
  trigger header; **Repeated Reverberation** ("When you next cast an instant
  spell, cast a sorcery spell, or activate a loyalty ability this turn") under a
  delayed trigger; **Illusionary Mask** under a replacement's `would`. The
  algebra round's M7 read 0 genuine because its regex could not separate
  "instant or sorcery spell" from a real coordination. So the binary `AltEvent`
  is short at its own seat, and `Delayed` and `Intercepts` carry no coordination
  slot at all.

**Why nothing is minted here.** Not thin evidence — five lines, three seats. The
refusal is the workbench's own: no line in the family composes today, because
the ARMS are missing from the vocabulary, and a slot with no whole-card witness
is a constructor with no witness. Verified: Giggling Skitterspike's third arm
("becomes the target of a spell") has no `GameEvent` row at all; Syr Konrad's
third arm ("a creature card leaves your graveyard") cannot be `Leaves`, whose
noun is gated to the battlefield. Vocabulary before the slot, then; the slot's
shape is settled in advance and written on `AltEvent` and on the `GameEvent`
docstring, and widening the binary to three is explicitly NOT the move.

### Ledger

- **The passive payment header** — Balduvian Fallen, Shah of Naar Isle. Two
  blockers: the event's subject is the cost rather than a player, which the row's
  subject slot is not; and `Echo` is not in `Keyword`.
- **A life payment** — Font of Agonies, "Whenever you pay life, put that many
  blood counters on this enchantment." The paid thing is not a keyword-named
  cost, and the tail's "that many" wants a magnitude this row does not announce.
- **A repeated payment of an offered cost** — the five Adversaries and Tranquil
  Frillback, "When you pay this cost one or more times". The paid thing is an
  anaphor to a cost the preceding clause offered, and "one or more times" is a
  repetition count; both are new machinery, adjacent to but distinct from the
  reflexive seat.
- **Heart of Bogardan's body** — the split read against a header that has
  already announced a player. Card-level; see above.
- **The n-ary event disjunction slot** — verdict 2's build, gated on its arms'
  vocabulary: a "becomes the target of" event, a `Leaves` whose zone is not
  fixed to the battlefield, and the coordination slot at `Delayed` and
  `Intercepts`.

### Gates

- `idris/scripts/build` — 23/23, 0 errors, 0 warnings.
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 16,850 citations, 0 stale; no rule needed blessing.
- `jj diff --git | cargo xtask cite audit --diff` — 13 sites, each read against
  its rule. Two were sharpened on that read: [CR#702.9a] now claims only what it
  says (flying stated whole as an evasion ability, naming no cost), and
  [CR#113.6b] is used as `Words.idr` already uses it (an ability functions from
  the zones its own rule names).
