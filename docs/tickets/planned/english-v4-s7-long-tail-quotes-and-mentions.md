---
needs: []
---
# Stratum 7: long-tail quotes and mentions

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Quoted text used as a constituent, and mentioned words and phrases.

Quotes:
- **Brutal Suppression, Drought**: *cost an additional "Sacrifice a land" to activate*. Hypothesis: a quoted imperative acts as a count nominal after *an additional*.
- **Raging River**: *into a "left" pile and a "right" pile … choose "left" or "right."*. Hypothesis: quoted words as premodifiers and as mentioned objects, with a period inside the closing quote. Second triage: a period absorbed inside *"right."*, plus two comma-set front adjuncts.
- **Volrath's Shapeshifter**: *has the text "{2}: Discard a card."*. Hypothesis: an appositive quoted ability after the noun *text*.
- **Urza's Saga**: a single-quoted ability nested inside a double-quoted one. Hypothesis: nested quotation depth is unsupported.

Mentions:
- **Teferi's Realm**: *chooses [artifact, creature, land, or non-Aura enchantment]*. Hypothesis: a multiword mention in a coordination.
- **Sarpadian Empires, Vol. VII**: *choose [white Citizen, blue Camarid, …, or green Saproling]*. Hypothesis: each mention is a color adjective plus a subtype noun.
- **Artificial Evolution**: *The new creature type can't be [Wall]*. Hypothesis: a general predicative mention multiplied forests, so this needs a narrow, head-selected route. Measure.
- **Grim Captain's Call**: *then do the same for [Vampire, Dinosaur, and Merfolk]*. Hypothesis: a mention coordination as the complement of *for*.

## Witness cards

- Brutal Suppression
- Drought
- Raging River
- Volrath's Shapeshifter
- Urza's Saga
- Teferi's Realm
- Sarpadian Empires, Vol. VII
- Artificial Evolution
- Grim Captain's Call

## Estimated faces

9 faces.

## Hypotheses, not findings

Every diagnosis above comes from a triage of the unread faces and is a hypothesis. Before changing anything, confirm it against the code and a parse of the card, and drop or replace any diagnosis that does not hold. Measure each construction before and after (§4.4, principle 5), and add a positive judgment per witness asserting its intended analysis (`crates/deckmaste_english_v4/judgments/`).

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
