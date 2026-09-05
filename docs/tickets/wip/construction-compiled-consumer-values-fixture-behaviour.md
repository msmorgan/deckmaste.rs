---
needs: [construction-compiled-consumer-values]
---
Consumer-values fixture asserts values, not behaviour (compiled-consumer-values
landing review M1). The xtask emitter fixture's `NounLexeme` declares no
licence/relationality feature members, so the generated aggregate helpers
(`bare_locative_license`/`relationality` accessors over declared members) are
never emitted there; the fixture's assertions read the transported values but
nothing generated ACTS on them, so a regression in the aggregate helper
emission would pass the fixture. Extend the fixture grammar with a noun
lexeme declaring one member per licence/relationality feature and a
construction whose `require` reads the aggregate helper, then assert a
parse OUTCOME that flips with the declared value (mutation-verify: change the
member's value, the assertion must fail). ~20–40 lines. (Review L3, the
missing performance-advisory line in the landing-record rule, is already fixed
in CLAUDE.md.) Zero grammar change; `expand`
byte-identical. Standard constraints apply.

## Landing record

Measured after `kata refresh` on change `ltpuymsqmppz`, with 17,601 covered
lock identities.

- PROVE — report-mode coverage checked cleanly: 17,601 -> 17,601 selected and
  covered units; parse failures 15,040 -> 15,040; selected-uncovered,
  unresolved ties, internal failures, round-trip mismatches, ownership
  failures, and traversal failures remain 0. Lock delta: gained 0; lost 0.
  There are therefore no newly covered or no-longer-covered identities, and no
  selected analyses to name. No bless was needed.
- DISCLOSE — selection census is unchanged and not separately remeasured:
  this is a `deckmaste_construction` fixture-only change, with no
  `deckmaste_english_v2` grammar, vocabulary, selection rule, or corpus input
  change. The coverage result reports 20 permitted licensing checkers and 0
  forbidden checkers. The new fixture member satisfies both aggregate feature
  requirements; the default-valued member is rejected. Mutating the declared
  relationality member to `NonRelational` made that focused assertion fail;
  the declared value was restored before the green gates.
- REPORT — construction declarations: 387. Licensed vocabulary/lexicon
  homographs: `AttributiveAdjective::Untap` / keyword-action declaration
  `Untap`, and `TargetingMarker::Target` / `CommonNoun::Target`. Form
  literal/vocabulary overlaps: `additional`, `to`, `the`, `next`, `to`, `the`,
  `the`, `other`, `the`. Coverage used 8 workers and took 168.466477464 s at
  host load 23.90 / 21.80 / 12.23, with 208,210 ns/B thread CPU. Concurrent
  process count is unavailable in the sandbox. The 16.260 s quiet-host ceiling
  is advisory under this shared-host load.
- Gates — `cargo fmt --all`; strict
  `cargo clippy -p deckmaste_construction --all-targets -- -D warnings`;
  `cargo test -p deckmaste_construction` (40 compiled-consumer tests and 28
  trybuild fixtures); report-mode coverage; and
  `cargo xtask english_v2 roundtrip --require-clean --workers 8` all passed.
  Roundtrip: 17,601 accepted and clean; 0 mismatched.
- Assurance census: restored 0; re-spelled 0; ignored 0; added 1; removed 0.
- STOPs: none. Glossary gaps: none.

### Deviations and additions

- Added one fixture-only construction, root, feature-bearing lexeme member,
  and behaviour test to make generated aggregate helper values decide the
  checked parse outcome. No production grammar or emitter changed.
