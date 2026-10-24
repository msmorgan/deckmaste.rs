---
needs: []
---
# Stratum 7: long-tail non-finite clauses

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Gerund, absolute and causative non-finite clauses.

- **Deification**: *would result in [all loyalty counters on it being removed]*. Hypothesis: a gerund clause with its own subject as a PP complement (CGEL Ch. 14 §4 on gerund-participial clauses).
- **Cosmos Charger**: *Foretelling cards from your hand costs {1} less and can be done …*. Hypothesis: a keyword action used as a transitive gerund subject.
- **Worms of the Earth**: *or have this enchantment deal 5 damage to that player*. Hypothesis: causative *have* + NP + bare infinitive.
- **Naked Singularity, Reality Twist**: *If tapped for mana, this land produces {R}.* Verified blocker: the verbless conditional *If tapped for mana,* does not read (*If tapped, …* does). The earlier trailing-*instead of*-PP diagnosis and the plural-land-subtype diagnosis were both tested and are wrong (*Plains*, *Islands* and *Swamps* already read).

## Witness cards

- Deification
- Cosmos Charger
- Worms of the Earth
- Naked Singularity
- Reality Twist

## Estimated faces

5 faces.

## Hypotheses, not findings

Every diagnosis above comes from a triage of the unread faces and is a hypothesis. Before changing anything, confirm it against the code and a parse of the card, and drop or replace any diagnosis that does not hold. Measure each construction before and after (§4.4, principle 5), and add a positive judgment per witness asserting its intended analysis (`crates/deckmaste_english_v4/judgments/`).

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

## Review notes (hypotheses, verify first)

- Naked Singularity / Reality Twist, *If tapped for mana,*: let `VerblessConditional`'s participle take a complement PP.
- Deification, Cosmos Charger and Worms of the Earth are plain grammar.
