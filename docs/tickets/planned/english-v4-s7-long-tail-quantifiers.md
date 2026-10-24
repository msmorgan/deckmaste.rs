---
needs: []
---
# Stratum 7: long-tail quantifiers

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Quantifier, determiner and partitive gaps.

- **Academy Manufactor**: *create one of each*. Hypothesis: `NonNumericComplement`'s ExcludeTogether blocks fused *each* after *of*.
- **Avatar Aang**: *if you've done all four*. Hypothesis: `PredeterminerCount` requires an Overt head, and here the head is absent.
- **All Hallow's Eve**: *If there are no more scream counters*. Hypothesis: *no more* is not licensed without *than*.
- **Goblin Game**: *tied for fewest*. Hypothesis: a bare superlative fused head without *the*.
- **Hawkeye, Master Marksman**: *choose up to that many —*. Hypothesis: *up to* with a *that many* fused head.
- **Illusionary Mask**: *paid by [some amount of, or all of,] the mana …*. Hypothesis: a coordinated partitive prefix is not a right-nonce-constituent route.
- **Wolverine, Fierce Fighter**: *all other damage already dealt to him is healed*. Hypothesis: a pre-head adverb on a participial postmodifier, or *healed* is missing from the lexicon (unverified; check the lexicon first).
- **Soul Echo**: *for each 1 damage …, you remove …*. Hypothesis: `DistributiveQuantity` requires Count, and *each 1 damage* is a numeral determiner on a mass noun.

## Witness cards

- Academy Manufactor
- Avatar Aang
- All Hallow's Eve
- Goblin Game
- Hawkeye, Master Marksman
- Illusionary Mask
- Wolverine, Fierce Fighter
- Soul Echo

## Estimated faces

8 faces.

## Hypotheses, not findings

Every diagnosis above comes from a triage of the unread faces and is a hypothesis. Before changing anything, confirm it against the code and a parse of the card, and drop or replace any diagnosis that does not hold. Measure each construction before and after (§4.4, principle 5), and add a positive judgment per witness asserting its intended analysis (`crates/deckmaste_english_v4/judgments/`).

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
