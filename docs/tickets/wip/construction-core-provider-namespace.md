Close the provider-helper namespace hole for all features
(compiler-onset landing review M1). The generated-name reservation for
latent provider helpers is guarded by `feature == Feature::Onset`, so the
other nine provider features keep the exact defect the new test pins: a
latent `NominalForm` provider plus an authored `nominal_form_for_source`
element generates with NO error where `onset_for_source` is now rejected.
Reserve for every provider feature (drive the reservation from the
feature list, not a match arm); extend the namespace test over all ten.
Also: five of six new latent helpers have no caller and are hidden by the
generated modules' blanket `#![allow(dead_code)]` — emit helpers only for
features some construction reads, or pin the latent-emission contract in
an english_v2 test (today reverting the arm passes the whole suite).
Zero grammar change; standard constraints apply.

## Landing record (2026-09-03)

The generated value namespace now reserves the future helper name for all ten
provider features from the same feature inventory: modifier license,
determiner number, fused-head license, preposition-complement kind,
locative-temporal license, nominal form, nominal license, onset,
preposition attachment, and relationality. The namespace regression exercises
every feature and rejects the authored helper-name collision before emission.

Onset helpers are no longer emitted merely because a category provides onset.
The renderer inventories the categories whose onset is actually consumed by a
preceding declaration determinative, including determinatives nested in a
category or structural field. The existing pawprint measure-unit regression
therefore retains `onset_for_measure_unit`, while the five unread English v2
onset helpers are not emitted.

| gate | before | after |
| --- | ---: | ---: |
| corpus units | 32,641 | 32,641 |
| selected and covered identities | 16,174 | 16,174 |
| unresolved ties / internal failures | 0 / 0 | 0 / 0 |
| construction declarations | 378 | 378 |
| coverage lock | current, 48,824 lines | current, unchanged |

The coverage lock remains byte-unchanged (SHA-256
`cdd3c0ae4096c6bd538f7453ad46ef3679a38ee7a36d1ffe74f7fe0a634ffe04`).
The coverage gate reports zero selected-uncovered units, exception uses,
round-trip mismatches, ownership failures, gaps, overlaps, synthetic claims,
or provenance-plan mismatches. The ambiguity gate reports 10,487 unique and
5,687 specificity-resolved selections, with zero unresolved ties or internal
failures. Both gates retained 1,433,177 accepted bytes. The ambiguity gate
emitted the existing common-path wall-clock warning against the 16.260-second
ceiling and completed successfully.

Positive gates: all `deckmaste_construction_core` and
`deckmaste_english_v2` test targets; strict all-target, all-feature Clippy for
both crates; `cargo xtask english_v2 coverage --check`; `cargo xtask
english_v2 ambiguity --require-resolved`; and `jj fix` with no changes. The
namespace regression was also proved non-vacuous: restoring the onset-only
reservation made its first non-onset case (`ModifierLicense`) fail, and the
completed reservation restored it to green.

### Deviations and additions

- Added one compiler regression that rejects emission of an unread onset
  provider helper. Extended the existing namespace regression with the other
  nine provider features.
- Added the renderer-side following-onset source inventory required to retain
  helpers with real generated callers while removing unread helpers.
- Assurance counts: restored 0 tests; re-spelled 0 tests; ignored 0 tests with
  new blockers; added 1 test; removed 0 tests.
- No constructions were added or deleted. No CR citations changed. No STOP was
  taken.
