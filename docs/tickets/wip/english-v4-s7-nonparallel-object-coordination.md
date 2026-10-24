---
needs: []
---
# Stratum 7: a plain Object coordinated with Object + Adjunct sequences

Part of `english-v4-stratum-7-discourse-and-long-tail`. Moved here from
`english-v4-s7-unlike-cluster-coordination`, which found no clean analysis.
This ticket needs a design decision before any code is written.

## Witness

- Crabomination: *target opponent exiles [the top card of their library],
  [a card at random from their graveyard], and [a card at random from their
  hand]*. It has 0 Derivations today (the line is in `deferred.txt`).

## What is established

- *at random* is clause-level. It is a manner PP, and CGEL gives no NP-internal
  use of a manner PP. Adverbs never modify nouns (Ch. 6 §5.1, pp. 562–563), and
  manner adjuncts modify the verb together with its complements (Ch. 8 §2,
  pp. 672–673). *a card at random from your graveyard* does not stand as an NP
  outside Object position. The grammar treats it the same way: Gorex
  (`selection_idioms.txt:3`) reads *at random* as an Adjunct of *choose*, with
  the participle postposed past it (`PostposedParticipialModifier`), and
  `*NounPhrase a card at random` holds.
- So the later two coordinates are nonce constituents: an Object, *at random*,
  and a source PP. Each is two Adjuncts long, while a Cluster has at most one.
  The first coordinate is a bare Object.
- CGEL stars right nonce-constituent coordination when one coordinate has one
  element and the other has two (Ch. 15 §4.3, pp. 1341–1342, [17iia]: "the
  first contains just a single element … while the second contains two").
  Footnote 64 (p. 1342) grades some parallelism breaches as better than others
  (*?He left his daughter $20,000 and half that amount to each of his
  grandchildren*). The grammar already admits one Adjunct asymmetry: Clusters
  of two or more Slots, only some of which end in an Adjunct (Ransack,
  `complement_cluster_adjuncts.txt`).

## Rejected routes

- Three ordinary NPs, with NP-internal *at random* allowed only before a
  postmodifier PP. CGEL gives this no support. The restriction is there only to
  keep *a card at random* out (`selection_idioms.txt:23`). It would add an
  NP-internal Reading to every *a card at random from their hand* unit
  (Planeswalker's Favor and others), and that analysis is not English.
- A second Cluster mechanism (`ObjectCluster`): cut in the original ticket.

## Decision needed

Does the Oracle register admit a single-Object coordinate beside Object +
Adjunct coordinates? This would extend the existing Adjunct asymmetry to a
single Slot, and it goes against CGEL's reading of [17iia]. If it does, the work
is in the landed Cluster engine (`complement_clusters` in
`deckmaste_syntax/src/compile.rs`):

- a single-Slot Cluster without an Adjunct, admitted only when a sibling
  Cluster has one, so that an all-bare coordination still reads as NP
  coordination (one tree per analysis);
- a Cluster that ends in a sequence of Adjuncts.

If it does not, the owner names the analysis Crabomination should read through.
It was first printed after 8th Edition, so it cannot be declined
(`docs/decisions/english-v4.md` §4.4, Scope).
