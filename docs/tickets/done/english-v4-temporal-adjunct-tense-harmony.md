---
needs: []
---
# Retired (owner round-8 treatment plan): tense of a *when* or *as* clause follows the clause it modifies

**Retired.** Left in `planned/` for the coordinator to close.

## Why

The ticket's only goal was to remove preterite Readings of a temporal *when* or
*as* Clause at admission (*When you cast this spell, …* with preterite *cast*).
Round 7 approved doing that with a `TemporalClause` complement kind and a
generic Valence `agree` rule in `compile.rs`. Round 8 retires such work:

- Admission stops only Readings that no English grammar admits
  (`docs/decisions/english-v4.md` §4.4, item 1). A preterite *when* Clause is
  grammatical English. The ticket's own witnesses show it is also genuine
  Oracle text (Soulgorger Orgg's *the life you lost when it entered*).
  `docs/oracle-style-guide.md` §1, "Voice and tense", prefers the present, but
  that is a register preference, not a grammaticality condition.
- The new complement kind and `agree` rule would be engine and grammar
  machinery whose purpose is to suppress a Reading, which §4.4, item 2,
  freezes.

## The census

Measured on `qynmxmro` with `cargo xtask english-v4 parse`:

- *When you cast this spell, draw a card.*: 2 Derivations, cheapest tier 2
  at `0/0/15`. The tier holds the intended present *cast* and the preterite.
  They differ only in that syncretic Word Form.
- *As this creature enters, choose a color.*: 1 Derivation.
- Soulgorger Orgg's *… the life you lost when it entered.*: 6 Derivations,
  cheapest tier 1 at `0/3/30`.

The intended Reading is in the cheapest tier everywhere measured. The
remaining tie is between Readings that differ only in a syncretic Word Form,
which each Reading keeps. Choosing the tense by meaning is
`deckmaste_semantics_v3`'s job (`english-v4.md` §4.3, "Lexical and domain
plausibility are not part of ranking"). No judgment asserts tense here.
