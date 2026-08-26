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
