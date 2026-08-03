---
needs: []
---
Editorial sweep of the ~32.8k comment lines in `crates/` (≈21k doc, ≈11.8k
plain). The dominant genre is not slop narration but the dense defensive
mini-essay: real CR rulings wrapped in reviewer-directed argument, caps
emphasis, and diff-speak. Disposition is per-block triage — keep the
invariant compressed, delete what the code and types already say, relocate
genuine essays — for both doc and plain comments. Success is information-
shaped, not line-shaped: no ruling lost. Expected yield ~40–50% of block
mass; that is a prediction, not a quota (a quota rewards deleting the
hardest-to-recover content first).

## Assessment (2026-07-24)

- Concentration: 78 files with ≥100 comment lines hold 84% of the mass;
  the top 40 hold 66%. `deckmaste_engine` alone holds over half.
- 5,294 comment lines carry `[CR#…]` citations (1 in 6) — load-bearing,
  entangled with the cite lock.
- ~1,500 blocks of ≥6 lines hold ≈40% of the mass; ~738 lines use caps
  emphasis (THAT, NEVER, ONCE…).
- Classic slop is nearly absent: imperative narration ~72 lines,
  diff-speak ~205, commented-out code ~12, plan-speak ~5, TODOs 3.

## Addendum (2026-08-03)

External review re-confirmed the mini-essay as the sweep's primary target
and measured the work-log dialect the phases must also strip:
reviewer-directed scope negotiation ("a correctness rewrite this task
does not sign up for" — `deckmaste_engine/src/sba.rs:~103-130` is the
type specimen) and plan shorthand no outside reader can resolve — 89
`Task N` references, 55 `P0.Wn` references, and 233 "seam" mentions
across `crates/**/*.rs` (regenerate at claim time:
`rg -nE '\bTask [0-9]|P0\.W[0-9]' crates`). Under the Phase 0 rules these
compress to the bare invariant, or to a labeled `TODO`/`SEAM` naming a
ticket slug; the essay and the negotiation are deleted.

## Phase 0 — decision doc

Commit `docs/decisions/comments-state-what-code-cannot.md` with exactly the
approved text below, add its line to `docs/decisions/README.md`, and add a
one-line CLAUDE.md pointer (flag the CLAUDE.md delta for user review).

```markdown
# Comments state what code cannot

## Decision

A comment earns its lines by stating something unrecoverable from the code
and types: rules semantics with a `[CR#…]` citation, a cross-module
invariant, a rejected alternative and why, or a labeled seam. Everything
else is deleted rather than written.

Comments address the future maintainer, never the current reviewer. The
banned genres are correctness argument ("this is safe because", "never a
panic"), diff narration ("previously", "now uses", "no longer"), and
plan or ticket narration — except a labeled `TODO`/`SEAM` naming a ticket
slug. Voice is declarative, one fact per sentence, no capitalized
emphasis, no "we".

A plain `//` block over four lines is oversized: compress it, or promote
it — a module contract to the `//!` doc, contributor-critical design to
`docs/decisions/`. Tracked code never cites local memory.

Doc comments open with a one-sentence contract and document the
interface — inputs, outputs, panics, rules semantics — not an
implementation walkthrough. Implementation notes are `//` comments inside
the body, under the same rules.

Test comments state the scenario and the rule being proven, cited;
step-by-step narration only where a stack or priority walk is genuinely
hard to follow. Section banners (`// ---`) remain allowed.

## Rationale

The comment corpus teaches by example: every contributor and every agent
session that reads a defensive mini-essay learns to write the next one.
Review-time justification belongs in the review, invariants that need
proof belong in tests, and durable design belongs in decision docs —
a comment is the wrong container for all three, and the container
determines whether the content survives its author's context.

## Consequences

Reviewers flag genre and length violations rather than debating them;
this decision is the argument. Deleting or moving a cited comment keeps
the cite toolchain green (`cargo xtask cite check` and companions, per
CLAUDE.md). Legacy comments are brought under this rule by dedicated
sweep work, not opportunistic rewrites inside feature diffs; new and
edited code is held to it immediately.

## Tracked references

- [CLAUDE.md](../../CLAUDE.md) — CR citation format and toolchain
```

README index line:

```markdown
- [Comments state what code cannot](comments-state-what-code-cannot.md) —
  Comments carry citations, invariants, and seams; everything else is deleted.
```

## Phases 1–5 — heavy files, per crate

Order: `deckmaste_engine` → `deckmaste_plugin` → `deckmaste_migrations` →
`deckmaste_core` → remainder (`tui`, `macro_ron`, `macro_ron_derive`,
`xtask`, `noncanon`, `macro_ron_lsp`). Heavy file = ≥100 comment lines;
regenerate the list at claim time
(`rg -t rust -c '^\s*//' crates | awk -F: '$2>=100'`) rather than trusting
this ticket's snapshot.

Whole-file editorial passes, 2–4 files per agent. All agents in a round
share a byte-identical brief prefix (the decision doc's rules plus
disposition guidance) so the prompt cache absorbs the repetition; the
per-round variation is only the file list at the tail. Every brief carries
a standing crate-boundary line naming the crate's external consumers, so
agents judging "derivable / needed by whom" never burn rounds on
cross-crate audits. Diffs are comments-only; the orchestrator verifies
each round by diffstat and scope.

## Phase 6 — tail

One agent sweeps the mechanical patterns across the remaining ~169 files
(16% of mass): diff-speak, imperative narration, commented-out code,
plan-speak. Review each grep hit in place; no whole-file reading.

## Verification per round

Standard constraints apply. Deltas:

- The round diff must contain no non-comment code changes.
- `cargo test --doc -p <crate>` after doc-comment edits (doctests live in
  doc comments).
- Cite toolchain after every round even when no citation was deliberately
  edited — deleting a comment can orphan lock entries.
- Commit per round; the sweep must be resumable at any crate boundary.

## Budget

~2–3M tokens across all rounds, dominated by the heavy-file phases.
