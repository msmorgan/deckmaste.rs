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
- **Goblin Game**: *tied for fewest*. Hypothesis: a bare superlative fused head without *the*. Second triage: also a central *then* (*… the fewest items then loses …*).
- **Hawkeye, Master Marksman**: *choose up to that many —*. Hypothesis: *up to* with a *that many* fused head. Second triage: also `• Word — Ability` modes.
- **Wolverine, Fierce Fighter**: *all other damage already dealt to him is healed*. Hypothesis: a pre-head adverb on a participial postmodifier, or *healed* is missing from the lexicon (unverified; check the lexicon first). Second triage: verb *heal* missing.
- **Frequency adverb plus interval** (*twice this turn*): in this ticket's quantifier and frequency scope, though no witness card is assigned. It takes the same analysis as *once each turn* (`english-lexical-analysis.md` §2, frequency interval): the temporal Noun Phrase after the final Adverb Phrase of a Verb Phrase is a second Adjunct of it, never also the adverb's interval. Confirm it against a parse, and add a judgment asserting one Reading.

## Witness cards

- Academy Manufactor
- Avatar Aang
- All Hallow's Eve
- Goblin Game
- Hawkeye, Master Marksman
- Wolverine, Fierce Fighter

## Estimated faces

6 faces.

Illusionary Mask and Soul Echo moved to `crates/deckmaste_english_v4/declined.txt` (pre-8th Edition, unique constructions); they are no longer witnesses here.

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

Census delta (measured on `qwlsoonxkxpl`, merged onto `long-tail-pp-coordination`): faces read 32,770 → 32,782, six of them this ticket's (Academy Manufactor, All Hallow's Eve, Avatar Aang, Goblin Game, Hawkeye, Master Marksman, Wolverine, Fierce Fighter: 0 → 1–24 Readings each; Wolverine's *but* Clause coordination is that ticket's); total Readings 337,354 → 337,038 and Surplus 141,594 → 141,660. The 17 units that fell all contain *twice* (frequency interval canonicalization). Lost faces: none.
Judgment set (`qwlsoonxkxpl`): 3,582 enforced and 3 pending, all enforced holding.
