---
needs: []
---
# Retired (owner round-8 treatment plan): *To solve —* takes a Condition, and a main clause is never preterite

**Retired and split.** Left in `planned/` for the coordinator to close. Its
catalog half is now `english-v4-case-to-solve-label`.

## Why

The ticket had two parts:

1. **Declare *To solve* as a keyword label over a Condition.** This is real
   coverage. Today every *To solve —* line reads only as a Flavor Word over a
   Sentence, which is a wrong analysis: *To solve* has rules meaning
   [CR#719.3a], and a Flavor Word has none [CR#207.2d]. It moves to
   `english-v4-case-to-solve-label`.
2. **Exclude `Inflection` `Preterite` on a matrix `Declarative`.** This is
   retired. Its only goal was to remove preterite Readings at admission (of
   *Creature spells you cast cost {1} less to cast.* and the like). A preterite
   main clause is grammatical English. The simple present is the register's
   preference (`docs/oracle-style-guide.md` §1, "Voice and tense"), not a
   grammaticality condition. Round 8 keeps such Readings and leaves them to
   ranking (`docs/decisions/english-v4.md` §4.4, items 1–2).

## The census

Measured on `qynmxmro` with `cargo xtask english-v4 parse`:
*Creature spells you cast cost {1} less to cast.* has 4 Derivations and a
cheapest tier of 4 at `0/2/21`. The tier holds the intended present/present
Reading and the three others. They differ only in the syncretic Word Forms
of *cast* and *cost*. The intended Reading is in the cheapest tier, and
choosing the tense by meaning is `deckmaste_semantics_v3`'s job
(`english-v4.md` §4.3).
