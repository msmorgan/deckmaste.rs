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
