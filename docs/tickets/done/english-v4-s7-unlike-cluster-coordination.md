---
needs: []
---
# Stratum 7: A plain Object coordinated with Object + Adjunct Clusters

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read Crabomination: *target opponent exiles [the top card of their library], [a card at random from their graveyard], and [a card at random from their hand]*. The first coordinate is a bare Object. The others are Object + manner Adjunct sequences whose *at random* is clause-level, since it hosts only on predicates. This was split out of `english-v4-s7-selection-idioms`.

## Settled

- The landed Complement Clusters require parallel coordinates, and a single Object forms a Cluster only with its Adjunct (`complement_cluster_adjuncts.txt`). CGEL also marks right nonce-constituent coordination of non-parallel coordinates as ungrammatical (Ch. 15 §4.3, p. 1341, [17iia]). Find the analysis that CGEL supports for this shape before adding any route. Do not weaken the parallelism rule.
- A first attempt added a second mechanism for this: grammar-authored `ObjectCluster` Categories with an engine `sequence` flag and an `associative` coordination resolution. It was cut because it duplicates the landed Cluster engine. Do not reland it.

## Witness cards

- Crabomination

## Estimated faces

About 1 face.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

## Review notes (hypotheses, verify first)

- Crabomination: do not treat it as clusters. Read it as three ordinary NPs.
- Inside [NP a card at random from their graveyard], *at random* is NP-internal before a source postmodifier, as on Gorex (`selection_idioms.txt:3`). License NP-internal *at random* only when a postmodifier PP follows, so bare *a card at random* stays out (`selection_idioms.txt:23`).
- Claimed: today the sentence reads only through a wrong *from [NP their graveyard and a card]* attachment.

## Outcome

No clean analysis was found, so Crabomination moved to
`english-v4-s7-nonparallel-object-coordination`, which records the evidence and
the decision it needs. The review notes did not hold up on their witnesses:

- Crabomination has 0 Derivations, so no wrong *from [their graveyard and a
  card]* Reading exists to remove. That attachment is the only Reading of the
  two-coordinate sentence *exiles a card at random from their graveyard and a
  card at random from their hand*, which is not a corpus unit.
- Gorex gives no precedent for NP-internal *at random*. It reads *at random* as
  an Adjunct of *choose*, with the participle postposed past it.
- CGEL gives no NP-internal manner PP. NP-internal *at random* allowed only
  before a postmodifier would be an exclusion aimed at a single witness.
- Treating the coordinates as Clusters fails CGEL's parallelism requirement
  (Ch. 15 §4.3, p. 1341, [17iia]).

No grammar change.

1. **Census delta** (measured on kxxkzsru): faces read 32806 → 32806, total
   Readings 338087 → 338087; lost faces: none.
2. **Judgment set**: all judgment files hold (cargo test -p deckmaste_english_v4: 74 passed, 0 failed; tests/judgments.rs 6 passed).
