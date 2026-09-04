Track the flavor-word nursery generator (flavor-word landing review M2).
The 630 `FlavorWord` stubs were produced by an untracked one-off; only the
census verifier exists (in tests/), so a newly printed flavor word must be
hand-added and repaired off a 630-element set-inequality diff. Add a
tracked xtask path (e.g. `cargo xtask english_v2 flavor-words --check /
--regenerate`) that derives the declaration set from the corpus census
with the SAME predicate the verifier uses (one shared implementation, not
two), writes stubs deterministically (sorted, byte-stable), and reports
the diff. Retire the hardcoded `assert_eq!(expected.len(), 630)` in favour
of check-mode equality against the generator. Zero grammar change;
standard constraints apply.

## Landing record

Measured on change `rpvxxqlt` with 16,702 covered lock identities. The
schema-4 lock remains 49,352 lines with SHA-256
`837688617caf0b0ff0c3e6551fcf2d58772e463f71040c1c42e6d51f2cb26446`.

| metric | before | after |
| --- | ---: | ---: |
| flavor-word declarations | 630 | 630 |
| selected and covered units | 16,702 | 16,702 |
| unique selections | 11,138 | 11,138 |
| specificity-resolved selections | 5,564 | 5,564 |
| construction declarations | 398 | 398 |
| literal/lexicon collisions | 59 | 59 |

- Generator result: `cargo xtask english_v2 flavor-words --check` reports
  that all 630 files are current. `--regenerate` is a byte-for-byte no-op on
  the checked-in nursery. Missing, changed, and stale files are classified in
  sorted order, and regeneration converges while leaving non-RON files alone.
- Shared authority: the Vintage-playable corpus predicate now lives in
  `deckmaste_data::flavor_words::census`; both the generator and the existing
  exact-set verifier call it. The verifier's hardcoded 630 count is removed.
- Coverage and selection state: the lock has +0/-0 identities and retains
  source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and normalization digest
  `f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
  Selected-uncovered units, unresolved ties, internal failures, exception
  resolutions and uses, round-trip mismatches, ownership failures, gaps,
  overlaps, synthetic claims, and provenance-plan mismatches are all zero.
- Positive artifacts after the required refresh: generator check; no-op
  regeneration; `cargo fmt --all -- --check`; strict all-target Clippy for
  `deckmaste_data`, `deckmaste_construction_core`, `deckmaste_english_v2`, and
  `xtask`; focused package tests; `cargo test --workspace`; `cargo xtask
  english_v2 ambiguity --require-resolved --json`; and `cargo xtask english_v2
  coverage --check --json` all exited zero. The corpus gates emitted only
  non-failing busy-host performance warnings.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added
  5 test functions; removed 0. The additions authenticate the shared
  three-position/Vintage predicate, stable declaration naming, deterministic
  diffing and convergence, and the CLI's exactly-one-action contract.
- STOPs: none.

### Deviations and additions

- The tracked command defaults to the repository's frozen AtomicCards,
  flavor-word catalog, ability-word supplement, and builtin-v2 flavor-word
  nursery, while allowing each path to be overridden for tests and review.
- Declaration naming reproduces the existing one-off's punctuation and
  numeric-prefix behavior. The two non-alphabetic spellings (`... Catch` and
  `10,000 Needles`) retain explicit consonant-onset metadata; all generated
  files remain byte-identical.
- No grammar, construction, coverage-lock, or generated-stub bytes changed.

## Erratum (flavor-generator landing review, 2026-09-03)

HIGH: `render_stub` emits `onset: Consonant` for every spelling the
recipe cannot classify — the inference the builtin-v2 ADR forbids
("normalization never guesses an onset"); it would write `∞`, `8-Bit`,
`11th Hour` as Consonant and revert any authored `Vowel` on --regenerate.
Latent (today's 630 bytes are correct) but a ruling contradiction
resolved without a STOP; "STOPs: none" is wrong. `--check` is exercised by
no test or gate. Routed to builtin-v2-flavor-word-generator-onsets.
