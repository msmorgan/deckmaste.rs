---
needs: [english-v2-grammar-migration-design]
---
> **Migration routing (2026-09-05).** This unclaimed ticket waits on
> `english-v2-grammar-migration-design` under the
> [Lean design decision](../../decisions/english-lean-design-workbench.md).
> That design task must reconcile and repin this ticket before it becomes
> executable. The prior body below preserves examples, regression and
> re-coverage obligations, and proposed mechanisms; its old sequence,
> implementation prescriptions, and coverage-ratchet acceptance do not
> override the new design process or the current landing contract.

# Declare the type-line order in english_v2

Routed from `workbench-card-class-and-command-zone` (close, 2026-08-26),
superseding the follow-up left by `workbench-type-line-order-is-spelling`.
The measured order table now lives at
`crates/deckmaste_english_v2/docs/type-line-order.md` (12 attested
sequences; ranks 9–14 are [CR#300.1]'s stated alphabetical convention), but
the crate still builds no type line: nothing consumes the table. The consumer
is `english-v2-type-line-construction`, this ticket's `needs:` — when that
Construction lands, convert this document into the declaration form it
consumes.

2026-09-04: the letter contradicted its own Acceptance (the 2026-09-04 STOP
below) — was: "When the v2 spelling layer renders or parses a type line, this
document is its declaration source — wire it then; do not build a consumer
before one is needed." Acceptance demanded a consumed declaration with tests
while the letter forbade building the consumer; the `needs:` routing is the
recorded resolution.

## Consumption boundary

`crates/deckmaste_english_v2` only. No workbench files.

## Acceptance

- The type-line order is consumed from the declaration (or the doc is
  converted to the declaration form the crate uses), with tests against the
  attested sequences. The Linearization is total over the declared Card Type
  inventory: the 12 attested sequences are conformance evidence, never the
  admitted domain, and an unattested combination of declared Card Types must
  linearize rather than reject (rewrite ADR, "Terminal generation (stage-5
  Plan 03 ruling)": a recipe accepts its full structural domain, never an
  observed-corpus subset).

2026-09-04: attestation scope pinned per that ruling — the previous wording
left "tests against the attested sequences" as the only stated domain.

Standard constraints apply.

## Landing record (STOP, 2026-09-04)

STOP taken: the ticket cannot be implemented within its own consumption and
timing boundaries. The crate has no declaration-only form for Type Line Card
Type order. Its `constructions!` declaration language can declare a `vocab`,
but that declaration emits a parser terminal contribution and a renderer; it
therefore creates the speculative parse/render consumer that the ticket says
not to build before a Type Line Construction is needed. The crate's Card Type
words instead enter through the normalized open `Type` declaration inventory.
The governing builtin-v2 spelling/grammar decision requires that inventory to
be the single boundary seen by parser, renderer, diagnostics, and Construction
consumers, so duplicating its members as a closed vocabulary would also create
a second vocabulary authority.

The other available declaration source, `core_verbs.ron`, is specifically a
`CoreVerbDeclaration` inventory and cannot express Card Type order. Extending
the normalized open `Type` declaration schema to carry linearization order
would require changes outside `crates/deckmaste_english_v2` and to the builtin
declarations under `plugins/builtin_v2`, contrary to the ticket's consumption
boundary. A test-only declaration would not be production declaration content
and would leave Acceptance unsatisfied. No new declaration format, Type Line
Construction, parser, or renderer was invented to conceal this contradiction.

Decision wanted: either schedule the Type Line Construction that can consume
the order, or authorize and specify a declaration-only order field at the
normalized open `Type` boundary with the necessary wider consumption boundary.
Until then, the measured document remains the only non-speculative form.

Measured on change `rroqzmun` with a schema-4 coverage lock containing 16,771
covered identities (SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`).

| measure | before | after |
|---|---:|---:|
| selected and covered identities | 16,771 | 16,771 |
| coverage-lock identities | 16,771 | 16,771 (lock byte-unchanged, +/-0) |
| Construction declarations | 397 | 397 |
| selection census: unique | 11,515 | 11,515 |
| selection census: specificity-resolved | 5,256 | 5,256 |
| unresolved ties | 0 | 0 |

Newly covered identities: none. Coverage also reports zero selected-uncovered
units, internal failures, round-trip mismatches, ownership failures, traversal
failures, gaps, overlaps, synthetic claims, or provenance-plan mismatches.

Performance advisory: `coverage --check` completed in 44.200444463 seconds at
162,271 ns/B with 1 concurrent `codex` process at measurement. This exceeds
the 16.26-second quiet-host ceiling under load (1-minute load 29.26), so it is
reported as contention rather than a STOP.

Positive gates:

- `cargo fmt --all`: exit 0 (stable rustfmt reported the repository's existing
  nightly-only option warnings).
- `cargo clippy -p deckmaste_english_v2 --all-targets -- -D warnings`:
  `Finished dev profile`, exit 0.
- `cargo test -p deckmaste_english_v2`: 413 passed, 0 failed, 0 ignored across
  unit, integration, and doc tests; every suite reported `test result: ok`.
- `cargo xtask english_v2 coverage --check`: `summary
  {"total_units":32641,"selected_units":16771,"covered_units":16771,
  "selected_uncovered_units":0,"parse_failures":15870,
  "unresolved_ties":0,"internal_failures":0}`; exit 0 against the current
  lock.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check`: `checked 17921 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `jj diff --git | cargo xtask cite audit --diff`: `audited 1 citation site(s)
  - read each rule text against its claim`. The ticket's [CR#300.1] was read:
  its Card Type list supports the claimed relative order of ranks 9-14.

Lifecycle advisory: `kata refresh` was attempted twice from this feature
workspace after the safe STOP change was closed. Both attempts returned exit
69 before rewriting this stack because the sibling working-copy changes
`sulmlzws` (`english-v2-licensing-checker-count`) and `qtprrqyl`
(`english-v2-closed-class-single-owner`) are divergent. Those sibling-owned
changes were left untouched. Since refresh made no transition, the measured
tree did not become stale and no post-refresh re-measurement applies. This
feature was not integrated or dropped.

Assurance counts: 0 restored, 0 re-spelled, 0 ignored with blockers, 0 added,
0 removed.

Deviations and additions: the Landing record is the only change. No
Construction or test was added or deleted. `constructions.rs` and
`core_verbs.ron` are untouched. No coverage identity or selected analysis
changed.

Glossary gap: none. The record uses Oracle English `Construction`,
`Linearization`, and `Realization`, and Game Model `Type Line` and `Card Type`
with their owning-context meanings.
