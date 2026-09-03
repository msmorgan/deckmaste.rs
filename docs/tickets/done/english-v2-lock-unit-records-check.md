Compare the lock's per-unit normalization records in `--check`
(lock-drift landing review M1). `--bless` writes 32,641 per-unit records
(schema 4) but the Check arm never compares them against the freshly
built `replacement.normalization_units`: a lock with the vector deleted,
or with 100 falsified card names and flipped onsets, passes exit 0 and
then mis-localizes real drift (zero lines, or 50 spurious "context onset
changed" lines). Add the equality check in the Check arm (the replacement
is constructed one line earlier); a mismatch fails naming `--bless` and
the first differing unit; add regressions for the deleted-vector and
falsified-row cases. Zero identity change; standard constraints apply.

## Landing record

Schema-4 `--check` now compares the lock's complete `normalization_units`
vector with the freshly constructed replacement after the aggregate digest
check. A mismatch reports the first differing vector index, the locked and
current records (including absence on either side), and directs the reviewer
to `--bless`.

| gate | before | after |
| --- | ---: | ---: |
| corpus units / normalization records | 32,641 | 32,641 |
| selected and covered identities | 16,174 | 16,174 |
| construction declarations | 398 | 398 |
| coverage lock | schema 4, 16,174 identities | current, byte-unchanged |

The production coverage gate reports zero selected-uncovered units,
unresolved ties, internal failures, exceptions, round-trip mismatches,
ownership failures, gaps, overlaps, synthetic claims, or provenance-plan
mismatches. The coverage lock remains byte-unchanged (SHA-256
`cdd3c0ae4096c6bd538f7453ad46ef3679a38ee7a36d1ffe74f7fe0a634ffe04`).

Positive gates: `cargo fmt --all -- --check`; all 427 non-ignored xtask unit,
binary, integration, and doc tests; strict all-target xtask Clippy; and
`cargo xtask english_v2 coverage --check`.

Assurance counts: restored 0; re-spelled 0; ignored with blockers 0; added 2;
removed 0.

### Deviations and additions

- Added the requested deleted-vector regression and one falsified-row
  regression that independently falsifies `card_name` and `context_onset`.
  No constructions, corpus fixtures, or tests were added or deleted beyond
  the ticket's letter. No STOP was taken.


## Erratum (unit-records landing review, 2026-09-02)

Construction count cell copied stale: 378 at parent, change, and tip (the
record says 398, a figure four landings old). Substantive claim
(unchanged) is true. Review LOWs unrouted: single named mismatch with no
count; both tests fail at index 0 on a one-unit fixture (first-differing
and baseline-longer paths pinned only by live probe); struct-dump message
vs the house tab-separated form.
