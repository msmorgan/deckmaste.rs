---
needs: [ci-cite-gate, engine-todo-triage]
---
**Land the reviewer-facing slice of the comment-discipline program without
making the entire repository sweep a publication blocker.** The cold-reader
failures are concentrated: unresolved `Task N`/`P0.Wn` shorthand, the
reviewer-directed correctness essay in `deckmaste_engine/src/sba.rs`, and the
large engine files a ten-minute reviewer is actually directed toward.

Scope:

1. Commit `docs/decisions/comments-state-what-code-cannot.md` with the exact
   approved text below, index it from `docs/decisions/README.md`, and add a
   one-line CLAUDE.md pointer (flag that CLAUDE.md delta for user review).
2. Eliminate unresolved work-log vocabulary across `crates/`: regenerate with
   `rg -n -e '\bTask [0-9]' -e 'P0\.W[0-9]' crates`, then replace every hit
   with the durable invariant or a labeled `TODO`/`SEAM` naming a real ticket
   slug. The final command must return no hits.
3. Perform comment-only whole-file passes over `sba.rs`, `layer.rs`, and
   `resolve/effect.rs`, plus the two remaining engine files with the highest
   comment-line count at claim time. Preserve CR citations and invariants;
   remove reviewer negotiation, diff narration, caps emphasis, and prose that
   merely restates the code.
4. Run doctests for touched crates, the citation toolchain, and a diff check
   proving no non-comment code changed.

The complete 32k-line editorial program remains
`comment-discipline-sweep`; this ticket is the bounded presentation slice and
its prerequisite.

Approved decision text:

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

Decision-index line:

```markdown
- [Comments state what code cannot](comments-state-what-code-cannot.md) —
  Comments carry citations, invariants, and seams; everything else is deleted.
```
