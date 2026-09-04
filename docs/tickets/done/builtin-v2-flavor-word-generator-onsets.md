Stop the flavor-word generator guessing onsets (flavor-generator review
H1/M1). `render_stub` writes `onset: Consonant` for any spelling the onset
recipe cannot classify — forbidden by the builtin-v2 spelling/grammar ADR
(authored, attested overrides only; never inferred). Replace with an
explicit authored `(surface, onset)` override table (today: "... Catch" and
"10,000 Needles", both Consonant, attested) and a HARD ERROR for any
unclassifiable spelling absent from it; `--check` verifies the override
table against the nursery too, and a hand-authored override is never
reverted. Add a test that runs `flavor-words --check` (and the
one-shared-predicate equality) so a census/predicate edit cannot silently
regenerate the nursery green; wire `--check` into the standard gate list.
Zero grammar/lock change. Standard constraints apply.

## Landing record

Measured on change `zkttrowo` with 16,702 covered lock identities. The
schema-4 lock remains 49,352 lines with SHA-256
`837688617caf0b0ff0c3e6551fcf2d58772e463f71040c1c42e6d51f2cb26446`.

| metric | refreshed parent | feature |
| --- | ---: | ---: |
| flavor-word declarations | 630 | 630 |
| selected and covered units | 16,702 | 16,702 |
| unique selections | 11,132 | 11,132 |
| specificity-resolved selections | 5,570 | 5,570 |
| construction declarations | 398 | 398 |
| literal/lexicon collisions | 59 | 59 |

- Generator result: `cargo xtask english_v2 flavor-words --check` accepts all
  630 files, and `--regenerate` is a byte-for-byte no-op. The closed authored
  override inventory contains `... Catch` and `10,000 Needles`, both with
  attested consonant onset. Missing, stale, or duplicate inventory entries
  fail before generation; an authored override remains explicit even if the
  bounded recipe later learns to classify its surface.
- Coverage and selection state: the feature changes no grammar inputs or
  generated declarations, so the refreshed-parent and feature measurements
  are identical. The lock retains source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and normalization digest
  `f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
  Selected-uncovered units, unresolved ties, internal failures, exception
  resolutions and uses, round-trip mismatches, ownership failures, gaps,
  overlaps, synthetic claims, and provenance-plan mismatches are all zero.
- Positive artifacts after the required refresh: direct generator check and
  no-op regeneration; the existing shared census-to-nursery equality test;
  `cargo fmt --all -- --check`; strict workspace all-target Clippy;
  `cargo test --workspace`; `cargo xtask english_v2 ambiguity
  --require-resolved`; and `cargo xtask english_v2 coverage --check` all
  exited zero. Corpus gates emitted only non-failing busy-host performance
  warnings.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added
  3 test functions; removed 0. The additions authenticate hard failure for an
  unreviewed unclassifiable surface, preservation of an authored override,
  and execution of the real checked-in nursery gate.
- STOPs: none in this corrective ticket. It removes the prior landing's
  ruling contradiction and makes recurrence a hard failure.

### Deviations and additions

- The standard CI fixture sequence now runs the flavor-word check immediately
  after deriving its corpus and catalog inputs.
- No flavor-word stub, grammar declaration, construction, or coverage-lock
  byte changed.

### Erratum (landing review, 2026-09-03)

- Performance advisory (measured by the reviewer, absent from the record):
  `coverage --check` elapsed 18.74s wall (ceiling 16.26s quiet-host; host load
  34–46 from concurrent executors, so the warning is load, not regression),
  accepted CPU 114.7 µs/B; `ambiguity --require-resolved` 42.9s, 148.7 µs/B.
- The `stale` and `duplicate` override-inventory branches in
  `crates/xtask/src/english_v2/flavor_words.rs` are unpinned (only `missing`
  is backstopped by a tested `ensure!`). Follow-up:
  `builtin-v2-flavor-word-override-inventory-closure`.
