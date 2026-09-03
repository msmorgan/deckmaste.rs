---
needs: [english-v2-retire-authenticator-tests]
---
Localize lock drift (normalization-freeze review M1/L1/L5). When the
normalization digest changes, `--check` prints two hashes over 32,641
texts and nothing else; add a per-unit report path (e.g. `coverage
--normalization-diff <old-lock>`) that lists the unit ids and card names
whose normalized text differs, so the required landing-record note can be
written from evidence. Also pin `context_onset` (the one remaining
code-derived unit input) into the digest recipe or a sibling digest, and
delete the two dead schema-migration paths that accreted. Zero identity
change; standard constraints apply.

## Landing record

The coverage lock is now schema 4. It retains one compact record per corpus
unit, keyed by a new domain-separated source-unit fingerprint that excludes
normalized text. Each record pins the current normalized unit identity, card
name, and `context_onset`. That stable source key lets `coverage --check`
automatically localize aggregate digest drift even when a normalized-text
change also changes the unit identity: diagnostics name the card and old/new
unit ids for text drift, or the card, unit id, and old/new onset for onset-only
drift.

The normalization digest recipe moved from v1 to v2 and now frames
`context_onset` beside every normalized text. Schema 3 remains as the single
exact migration path: `--bless` accepts it only when its source fingerprint,
covered vector, and legacy text-only normalization digest all match. The dead
schema-1 and schema-2 readers, migrations, and supporting types were deleted.

| gate | before | after |
| --- | ---: | ---: |
| normalized corpus units | 32,641 | 32,641 |
| selected and covered identities | 16,174 | 16,174 |
| ordinary parse failures | 16,467 | 16,467 |
| units with pinned `context_onset` | 0 | 32,641 |
| construction declarations | 378 | 378 |
| coverage-report schema | 4 | 5 |
| coverage-lock schema | 3 | 4 |
| corpus source fingerprint | `e85359d7…b3637dd` | `e85359d7…b3637dd` |
| normalization digest | `dfd48783…9486d749` | `f3a2fccd…935341c9` |

The covered vector and all corpus identities are unchanged. The lock SHA-256
moved only for the schema and digest-recipe migration, from
`aeba28e9d9309c147bda971e35365589e1e6cee8a726cd8bab5cf02bb323ff54`
to `cdd3c0ae4096c6bd538f7453ad46ef3679a38ee7a36d1ffe74f7fe0a634ffe04`.

Post-refresh positive gates: 413 xtask library tests passed with one documented
ignore, all 11 xtask CLI tests and the determinism test passed, and strict
all-target xtask Clippy passed. `cargo xtask english_v2 coverage --check`
passed against the current schema-4 lock with 32,641 total units, 16,174
selected and covered identities, and 16,467 ordinary parse failures. It
reported zero selected-uncovered units, unresolved ties, internal failures,
exception uses, round-trip mismatches, ownership failures, gaps, overlaps,
synthetic claims, or provenance-plan mismatches; its two pre-existing
literal/lexicon collisions are unchanged. The refreshed parser-performance
instrumentation remained wired and emitted its `PERFORMANCE` line plus the
expected advisory ceiling warning (18.583712101 seconds against 16.260).

### Deviations and additions

- Drift localization is emitted automatically by the existing `coverage
  --check` path instead of requiring a separate `--normalization-diff` flag.
  This keeps the evidence attached to every failing normalization check.
- Added two tests: exact schema-3-to-4 migration and strict validation of bad,
  duplicate, and unsorted per-unit records. Extended existing regressions to
  prove parse-failure-only text drift and onset-only drift both name the
  affected card and identities.
- Deleted three tests whose only subjects were the deliberately deleted
  schema-1 and schema-2 production paths. Re-spelled four existing tests for
  the schema-4 and onset-inclusive contracts. Assurance counts: restored 0;
  re-spelled 4; ignored with new blockers 0; added 2; removed 3.
- Added, changed, and deleted 0 constructions. No normalized identity or CR
  citation changed, and no STOP was taken.
- Retirement/re-coverage obligation: none; no covered identity was retired.
