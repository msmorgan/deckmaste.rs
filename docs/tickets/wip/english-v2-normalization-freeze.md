---
needs: [english-v2-retire-manifest-precondition]
---
Freeze normalization over the whole corpus (line-initial landing review
M1). The coverage lock hashes only covered units' normalized text; since
the fossil normalizer test was (correctly) retired, ~16,675 parse-failure
units' normalization is pinned by nothing — a normalization change that
alters only failing units' text is invisible until one of them starts
selecting under a changed identity. Add a corpus-wide normalization digest
to the lock header (hash over all 32,641 normalized texts in unit order),
checked by `--check` like the existing corpus fingerprint and updated only
by `--bless`; a normalization change therefore always surfaces as lock
drift to be blessed with a landing-record note. Also: rename the three
`*_mid_line_*` tests to what they now test, and decide line-final "()"
pass-through explicitly. Zero identity change; standard constraints apply.

## Landing record

The coverage lock is now schema 3 and carries a domain-separated,
length-framed SHA-256 digest over every normalized text in sorted corpus-unit
order. `coverage --check` rejects digest-only drift, including a change confined
to a parse-failure unit, and `coverage --bless` is the only path that updates the
digest. The one-time schema-2 migration is accepted only when the source
fingerprint and covered vector are both unchanged.

Line-final empty `()` groups explicitly pass through byte-exactly: they are not
reminder text. An empty group followed by non-whitespace text remains a
card-naming normalization error. The three former `*_mid_line_*` test names now
describe the actual followed-by-text rule.

| gate | before | after |
| --- | ---: | ---: |
| normalized corpus units | 32,641 | 32,641 |
| selected and covered identities | 15,966 | 15,966 |
| parse-failure units covered by the normalization digest | 0 | 16,675 |
| construction declarations | 378 | 378 |
| coverage-lock schema | 2 | 3 |
| corpus source fingerprint | `e85359d7…b3637dd` | `e85359d7…b3637dd` |
| normalization digest | absent | `dfd48783…9486d749` |

The covered vector is byte-for-byte unchanged. The lock SHA-256 moved only for
the schema/header migration, from
`b6a4a064882ecde7dca01b41345c805e6bfba7206fb6b58a63af807bcc91406e`
to `46d7754d9a4f56b68a63dca1bf4b311789eeca0c826a372be787b826514d33fd`.

Positive gates: 412 xtask library tests passed with one documented ignore, all
11 xtask CLI tests and the determinism test passed, strict all-target xtask
Clippy passed, `cargo xtask english_v2 coverage --check` passed, and the full
workspace test suite passed. The coverage check reports zero
selected-uncovered units, unresolved ties, internal failures, exception uses,
round-trip mismatches, ownership failures, gaps, overlaps, synthetic claims,
or provenance-plan mismatches; its two pre-existing literal/lexicon collisions
are unchanged.

### Deviations and additions

- Added three positive regressions: digest framing/order, explicit line-final
  empty-group pass-through, and exact schema-2-to-3 migration. Extended the
  schema-3 drift test to prove that a parse-failure-only text change fails
  `--check` without mutating the lock and is updated only by `--bless`.
- Renamed three tests without changing their inputs or assertions. Assurance
  counts: restored 0; re-spelled 0; ignored with new blockers 0; added 3;
  removed 0.
- Added, changed, and deleted 0 constructions. No normalized identity changed,
  no CR citation changed, and no STOP was taken.
- Retirement/re-coverage obligation: none; no covered identity was retired.
