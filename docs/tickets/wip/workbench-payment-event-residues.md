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
