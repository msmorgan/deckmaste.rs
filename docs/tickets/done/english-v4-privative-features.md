---
needs: []
---
# Privative one-valued features (R5.1)

From `english-v4-child-name-control-and-category-fission-audit`, Finding G
(the F list). The owner ruled R5.1 in round 9.

## Owner's ruling (round 9, R5.1)

"Approve: amend english-lexical-analysis §3 and the compile.rs check to allow
1-valued privative features (None = unmarked default; the marked value has
>=2 holders). Delete the unused VerbKind::Lexical and AdjectivalPassive::Derived
values."

## Evidence

VerbKind `Lexical` and AdjectivalPassive `Derived` have no holder, because
every reader excludes the marked value:
- VerbKind `["Lexical", "Auxiliary"]` (`english.ron:39`): `Auxiliary` is set
  by the lexicon build (`deckmaste_lexical_source_v4`, `build.rs`) and read by
  `Carry`/`Exclude` (`:631`, `:636`, `:702`, `:1213`) and `compile.rs`.
- AdjectivalPassive `["Derived", "Lexical"]` (`:37`): `Lexical` is set on
  declared adjectives (`deckmaste_lexical_source_v4`, `lib.rs`) and read by
  `compile.rs`.

The load report lists both unmarked values as zero-holder findings.

## Change

1. Amend `docs/decisions/english-lexical-analysis.md` §3. A feature may
   declare one value, a privative one. An unvalued feature is the unmarked
   default. The marked value is still a finding unless it has two or more
   holders. Keep "a class with one member, or a feature with one holder, is
   reported as a finding" for every other case.
2. Change the load check in `crates/deckmaste_syntax/src/compile.rs` to accept
   a one-valued feature whose value has two or more holders. That is the
   holder report (`report_holders`), and any check that requires two declared
   values. A one-valued feature whose value has fewer than two holders is
   still a finding.
3. Delete `Lexical` from VerbKind and `Derived` from AdjectivalPassive. Every
   reader keeps reading only the marked value.
4. Add a `deckmaste_syntax` test: a one-valued feature with two holders loads
   with no finding, and one with a single holder is reported.

`english-v4-holder-report-refinements` also changes `report_holders`. Whichever
ticket lands second rebases onto the other. Neither depends on the other's
meaning. `english-v4-engine-feature-names` owns `compile.rs` naming
AdjectivalPassive and VerbKind (line (a)). This ticket does not move those
names.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

Census delta (measured on mtykwkqy → zllmwulo): faces read 28,801 → 28,801; total Readings 245,825 → 245,825; Surplus 106,859 → 106,859; no unit changed; lost faces: none.
Judgment set (zllmwulo): 3,052 enforced and 206 pending, all holding.
