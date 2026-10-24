---
needs: []
---
# Stratum 7: long-tail PP and clause coordination

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Correlative, elliptical and adjunct-level coordination, and preposed or postposed adjuncts.

- **Dream Cache**: *[both on top of your library] or [both on the bottom of your library]*. Hypothesis: correlative *both* inside PP conjuncts of a disjunction.
- **Don't Blink**: *enter [from exile] or [after being cast from exile]*. Hypothesis: check *after* + Gerund as a coordinate PP.
- **Library of Leng**: *but you may put it on top of your library instead of into your graveyard*. Hypothesis: *instead of* + PP. Second triage: also mixed-mood Clause coordination with *but*; those schemas allow only the Coordinator class, not *but*.
- **Dance with Calamity, Lim-Dûl's Vault**: *As many times as you choose, you may …*. Hypothesis: a preposed equative frequency phrase.
- **Serum Powder**: *Any time you could mulligan and this card is in your hand*. Hypothesis: an adverbial relative coordinated with a gapless clause.

## Witness cards

- Dream Cache
- Don't Blink
- Library of Leng
- Dance with Calamity
- Lim-Dûl's Vault
- Serum Powder

## Estimated faces

6 faces.

## Hypotheses, not findings

Every diagnosis above comes from a triage of the unread faces and is a hypothesis. Before changing anything, confirm it against the code and a parse of the card, and drop or replace any diagnosis that does not hold. Measure each construction before and after (§4.4, principle 5), and add a positive judgment per witness asserting its intended analysis (`crates/deckmaste_english_v4/judgments/`).

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

## Review notes (hypotheses, verify first)

- Well scoped, with no engine changes expected.

## Landing

1. **Census delta** (`zqylmknn`, full corpus): faces read 32,770 → 32,777; total
   Readings (product) 337,354 → 337,514 and Surplus 141,594 → 141,705. Gained:
   Dream Cache 0 → 1, Library of Leng 0 → 6, Don't Blink 0 → 14, Serum Powder
   0 → 11, Dance with Calamity 0 → 42, Lim-Dûl's Vault 0 → 60, and (not a
   witness, now read through *but*-coordination of Clauses) Wolverine, Fierce
   Fighter 0 → 24. Trade Secrets 6 → 8, the equative frequency phrase beside
   its prior analysis. Lost faces: none.
2. **Judgment set** (`zqylmknn`): 3,566 enforced, 3 pending, including the new
   `long_tail_pp_coordination.txt`; all hold.
