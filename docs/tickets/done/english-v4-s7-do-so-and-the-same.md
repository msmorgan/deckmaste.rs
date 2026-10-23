---
needs: []
---
# Stratum 7: *Do so*, *do the same* and *the same is true for*

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read the pro-forms:
- *do so*, as in *All creatures able to block this creature do so* (about 30 faces);
- *does the same* and *Do the same for ...*;
- *The same is true for ...* (25 faces, profiled into Stratum 7 by Stratum 6).

The serial keyword list after *for* reuses the Stratum 6 coordination.

## Grounding

- CGEL, Ch. 17 §7.5, pp. 1529–1532 (*do so*).
- CGEL, Ch. 17 §7.6, p. 1532 (*do it*, *do that*).
- CGEL, Ch. 17 §1.4, p. 1461 (pro-forms).

## Witness cards

- Nessian Boar
- Breaker of Armies
- Taunting Challenge
- Escaped Shapeshifter
- Encroaching Mycosynth

## Estimated faces

About 59 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

No. Grammar and lexicon.

## Pending judgments this slice makes hold

None are pending against this slice. Add a positive judgment for each witness (`crates/deckmaste_english_v4/judgments/`) asserting its intended analysis.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

1. **Census delta** (`qqqsurtz`): faces read 28,802 → 28,862; total Readings 250,544 → 250,768; Surplus 109,750 → 109,870; 62 units changed; lost faces: none.
2. **Judgment set** (`qqqsurtz`): `cargo test -p deckmaste_english_v4 -p deckmaste_lexical_source_v4` passes (100 tests); all 26 entries in `do_so_and_able_to.txt` and `anaphoric_same.txt` independently hold through `cargo xtask english-v4 parse --limit 0`, including Stratum 7 entries.
