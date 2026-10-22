---
needs: []
---
# Retired (owner round-8 treatment plan): a coordination in a present-only Complement keeps preterite conjuncts

**Retired.** Left in `planned/` for the coordinator to close.

## Why

The ticket's only goal was to remove preterite conjunct Readings under
*whenever*, *while*, *until* and supplementary *where* at admission. Its
witness was *Whenever you cast a spell or put a counter on a creature, draw a
card*, with four Readings from present and preterite *cast* and *put*. Round 7
approved `resolve: [("Inflection", "Preterite")]` on the Coordination schemas
and folding `nonlast` across serial tails in `deckmaste_syntax`. Round 8
retires such work:

- Admission stops only Readings that no English grammar admits
  (`docs/decisions/english-v4.md` §4.4, item 1). A preterite conjunct under
  *whenever* is grammatical English. The present-only frame it would extend is
  the register's preference (`docs/oracle-style-guide.md` §1, "Voice and
  tense").
- The engine change exists to carry that suppression into coordinations,
  which §4.4, item 2, freezes.

## The census

Measured on `qynmxmro` with `cargo xtask english-v4 parse`:
*Whenever you cast a spell or put a counter on a creature, draw a card.* has
4 Derivations and a cheapest tier of 4 at `0/3/22`. The tier holds the
intended present/present Reading and the three others. They differ only in the
syncretic Word Forms of *cast* and *put*.

The intended Reading is in the cheapest tier. Choosing the tense by meaning is
`deckmaste_semantics_v3`'s job (`english-v4.md` §4.3). Agency Outfitter's
mixed-tense Clause coordination stays with
`english-v4-agency-outfitter-coordination-ranking`, which is a ranking ticket.
