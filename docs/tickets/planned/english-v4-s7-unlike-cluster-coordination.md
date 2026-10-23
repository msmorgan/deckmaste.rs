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
