---
needs: [english-v4-lexical-catalogs]
---
# Build the corpus complexity-stratum classifier, judgment harness, and `v3` test obligation audit

Partition the entire Vintage-legal Oracle card corpus—both the 93.8% currently
parsed by `deckmaste_english_v3` and the 6.2% unparsed residual—into the **8
Complexity Strata (Strata 0–7)** defined in
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6) and
[`docs/decisions/english-lexical-analysis.md`](../../decisions/english-lexical-analysis.md)
(§§1–2, 5), and audit `v3`'s test suite so no genuine behavioral obligation is
lost when `v3` is retired.

## Deliverables

1. **Syntactic Skeleton Profiler & Max-Stratum Assignment (in `deckmaste_english_v4`, CLI via `cargo xtask english-v4 strata`):**
   - Implement the corpus unit splitter, census runner, and syntactic-skeleton
     stratum profiler in `crates/deckmaste_english_v4` (keeping `crates/xtask` a
     thin CLI wrapper) using `deckmaste_lexical` + `deckmaste_lexical_source_v4`.
   - Abstract open-class catalog items (`CardName`, `Subtype`, `CounterKind`,
     `KeywordAction`, `KeywordAbility`, `AbilityWord`, `Number`, `Symbol`) to
     typed placeholders and classify each complete unit into **Strata 0–7** by
     the **Max-Stratum Composition Rule** ($\max_i \text{stratum}(c_i)$, where a
     unit belongs to the highest stratum required by any constituent inside it).
   - Account for 100% of the 6.2% unparsed `v3` residual so no unknown
     schema or production family remains hidden.
   - Report **read**, **known-wrong-only**, and **unread** counts separately per
     stratum, and verify both round-trip laws over the packed forest without
     blindly enumerating all $C_n$ attachment trees.
2. **Linguistic Judgment Set & `v3` Test Obligation Audit:**
   - Audit `crates/deckmaste_english_v3/tests/suite/` to extract all genuine
     positive and negative grammatical obligations (migrating them into the
     hand-written `v4` judgment set, one line per entry, no tree dumps) and
     classify the remaining wave tests that pin retired `v3` tree shapes,
     policing flags, or exact reading counts for retirement at cutover.
   - Support asserting constituent category, surface span, head identity,
     grammatical function (`Subject`, `Object`, `Complement`, `Adjunct`,
     `Modifier`), gap-filler binding, negative ungrammatical controls, and
     documented attachment-ambiguity witnesses (`[[Seedborn Muse]]`),
     adjudicated against the controlled Oracle English register
     (`docs/oracle-style-guide.md` and `docs/decisions/english-v4.md`, with
     CGEL used for standard terminology and constituent structure) by two
     independent judges.

Standard constraints apply.
