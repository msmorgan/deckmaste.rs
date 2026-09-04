---
needs: []
---
# Give corpus-text irregularities a ledger, not an inline patch

`docs/decisions/english-v2-rewrite.md` already decided the policy — guardrail
#3: "Corpus irregularities are quarantined as data — a known-uncovered list,
never grammar special cases" — but no artifact implements it. What exists
today (`normalize_oracle_text`, `crates/xtask/src/english_v2/corpus.rs:246`)
is uniform *structural* normalization only (quote-mark unification via
`deckmaste_data::academyruins::normalize_quotes`, roll-table dash spacing).
Every genuinely irregular oracle-text case turned up so far in the old (v1)
pipeline — Sphinx Summoner's stale-snapshot sentence break, Necratog's
lowercase-after-cost-colon typo (`crates/deckmaste_english/src/input.rs:290-309`)
— was absorbed by a *general* capitalization rule rather than a named patch,
which is the right answer for those two cases but means the "quarantine
irregularities as data" half of the guardrail has never actually been
exercised in code.

## The ask

Build the scaffold now, ahead of the first real irregularity, so the next
contributor who hits one has an obvious, reviewable home instead of an ad hoc
inline string edit or a second general-rule stretch that doesn't actually
generalize:

- A small data file (format at the executor's discretion; RON matches house
  convention elsewhere in the repo) listing patch entries — reason, the
  card/face name(s) it applies to, and exact find/replace text over the raw
  `AtomicCards` oracle text.
- Loaded and applied in `corpus.rs` before `normalize_oracle_text`, so a
  patched card's corpus unit carries the corrected text everywhere
  downstream.
- An empty ledger is the valid starting state. This ticket does not require
  seeding a real patch — none is currently known to be needed — only
  building the mechanism and wiring so the guardrail is honored the moment
  one is.

## Keep the two mechanisms distinct

This ledger is for "the source-of-truth text itself is wrong or needs
accommodating" (a patch, with a reason). It is **not** for "this text is
grammatical Oracle English but no construction parses it yet" — that case is
already the coverage lock's `SelectedUncovered` status
(`crates/xtask/src/english_v2/coverage.rs`), which exists and is unaffected
by this ticket. Document which of the two a given irregularity belongs to at
the point of use, so a future contributor doesn't reach for the wrong one.

## Consumption boundary

`crates/xtask/src/english_v2/corpus.rs` (wiring), a new small data file for
the ledger, `crates/xtask/src/english_v2/` tests. No Idris source.

## Acceptance

- A corpus-text patch is expressible as a data entry carrying a reason, never
  as an inline Rust string-literal edit.
- The ledger loads and applies before normalization; an empty ledger is a
  no-op (existing `english-v2-coverage.lock` output is unchanged).
- At least one test proves a patched entry's corpus text differs from the raw
  `AtomicCards` text and that the entry's reason is retrievable.
- A short doc-comment (or a paragraph in the corpus module) states the
  distinction from `SelectedUncovered` above.

Standard constraints apply.

## Landing record

Measured on change `wwtlulsk` with 16,771 covered lock identities.

- The xtask corpus loader now reads a named RON corpus-patch ledger. Each
  entry carries a reason, card or Card Face names, and exact find/replace
  text; matching raw `AtomicCards` text is patched exactly once before the
  existing structural normalization builds the downstream corpus unit. The
  checked-in ledger is empty, so this change introduces no real corpus patch.
- The corpus module documents that source-text irregularities belong in this
  ledger, while grammatical Oracle English that no Construction analyzes yet
  remains `CoverageStatus::SelectedUncovered`.
- Coverage before/after: 16,771 -> 16,771 selected and covered corpus units
  out of 32,641; 15,870 parse failures; 0 newly covered identities; 0 drops;
  0 selected-uncovered units, unresolved ties, internal failures,
  exception resolutions/uses, round-trip mismatches, ownership failures,
  traversal failures, gaps, overlaps, synthetic claims, or provenance-plan
  mismatches. Newly covered identities and selected analyses: none.
- Construction and selection censuses before/after: 397 -> 397 Construction
  declarations; 11,515 -> 11,515 unique and 5,256 -> 5,256
  specificity-resolved selections; 0 exception-resolved selections.
- Coverage lock: schema 4, +0/-0 identities, and byte-unchanged at 49,421
  lines / 8,306,882 bytes, with source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`,
  normalization digest
  `f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`,
  and SHA-256
  `2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.
- Performance advisory after the required refresh: `coverage --check`
  reported 24.013612974 seconds and 117,986 ns/B across 1,523,802 accepted
  bytes, above the 16.26-second quiet-host ceiling while host load was
  41.07/40.40/28.52; 1 concurrent
  `codex` process was present at measurement. This contention warning is
  reported and is not a STOP.
- Positive gate artifacts: `cargo fmt --all` exited 0; strict
  `cargo clippy -p xtask --all-targets -- -D warnings` exited 0;
  `cargo test -p xtask` reported `test result: ok. 423 passed; 0 failed; 1
  ignored`, `test result: ok. 12 passed; 0 failed`, and two integration
  `test result: ok. 1 passed; 0 failed` lines; `cargo xtask english_v2
  ambiguity --require-resolved` reported 0 unresolved ties and 0 internal
  failures; `cargo xtask english_v2 coverage --check` exited 0 with the
  coverage summary above. No citations changed, so the citation gate was not
  applicable.
- Assurance: restored 0; re-spelled 0; ignored with blockers 0; added 1;
  removed 0. The added corpus-level test proves a synthetic patch changes the
  raw snapshot text before quote normalization and preserves its reason.
- Deviations and additions: none; the one added test is required by the
  ticket. No grammar, Construction, `core_verbs.ron`, coverage-lock, citation,
  or Idris source changed. No scratch or probe tree was created.
- STOPs: none. Glossary gaps: none. Decisions wanted: none.
