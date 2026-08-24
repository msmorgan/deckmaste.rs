---
needs: []
---
# Build the falsifier: render bench terms back to printed Oracle text

**[design] — WHERE THIS LANDS NEEDS DISCUSSION BEFORE CLAIMING.** Every other
workbench ticket's consumption boundary is "the Idris workbench only". This one
cannot be, and the crate it belongs in is exactly the question. Claiming it
means settling that first.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axes 10 and 24 and summary finding 3 (2026-08-24). Deltas only. The premise this
attacks is `semantics-v2.md:§1` — "Constructions occupy their surface positions"
— which the comparison finds has no falsifier.

## From the v1 comparison (2026-08-24)

> **Adversarial check — the real limit.** The bench positives are hand-written
> Idris terms. Nothing checks that `Experimental/Cards.idr`'s 329 terms *say
> what the printed cards say*: there is no parser, no renderer, and no tie to
> the 7,294 authored `.ron` cards. The workbench proves the grammar internally
> consistent and CR-consistent; it does **not** prove it corpus-faithful.

and

> Workbench: 13 `-- spelling:` prose comments total (9 in `Experimental.idr`, 4
> in `Experimental/Macros.idr`) and no renderer, no lexicon, no witness. […]
> A renderer is the only artifact that can falsify that claim, and it does not
> exist. Every `-- spelling:` comment is an unverified assertion about what the
> constructor renders as; the 329 bench positives are never rendered and
> compared to printed Oracle text. The strongest claim the workbench currently
> supports is "the grammar is internally consistent and CR-consistent", not
> "the grammar is English".

## Scope

Design the **minimal** rendering path for bench terms — enough to take a
`Cards.idr` entry to a string and diff it against that card's printed Oracle
text. Not a general spelling layer, not a parser, not a round-trip. The
deliverable is the first honest measurement of how many of the 329 render
faithfully, plus the design that makes the measurement repeatable.

## Recorded obstacles, so they are not discovered mid-round

- **The ~200 proof arguments.** Roughly 200 `{auto 0 …}` obligations across
  `Experimental.idr` are erased at runtime and carry no surface. A renderer must
  read only the relevant arguments, and any renderer that needs a proof's
  *content* to choose a word has found a construction whose spelling is not
  determined by its surface arguments — which is itself a finding worth
  reporting rather than working around.
- **The `Expansion` collision.** The crate's macro round-trip works because
  every macroable type carries an `Expanded(Expansion<T>)` variant remembering
  the invocation (`macros.rs:120`,
  [macro-templates-are-bidirectional.md](../../decisions/macro-templates-are-bidirectional.md)),
  which is what lets the spelling layer re-render "destroy target creature"
  rather than the expanded move. The workbench's 241 `Macros.idr` functions
  erase at elaboration, and `Expansion<T>` carries a positional untyped argument
  list — precisely what the telescope forbids, since an argument whose type
  depends on the prefix cannot be stored positionally without its context. The
  comparison calls this "a real design collision between v2's binding law and
  the crate's macro round-trip, and nothing records it". A renderer either
  renders from the expanded term (and must then produce the macro's surface
  words from the expansion, not from a memory of the call) or the collision has
  to be solved first. Say which, up front.
- **The 13 `-- spelling:` comments** are the only existing statements of intent.
  Treat them as the first assertions to check, not as a specification.

## The boundary question this round settles

The renderer is not workbench-internal, and it must not land in a crate slated
for deletion. `deckmaste_english` and `deckmaste_spelling`'s splice-and-reparse
render/compile machinery both delete at cutover
([english-v2-rewrite.md](../../decisions/english-v2-rewrite.md)), and the
english_v2 stack is the *parser* rewrite — a different stack from the semantics
layer, which the comparison notes constrains nothing here except by naming
`deckmaste_english_v2` as the future producer of the trees semantics consumes.
The plausible homes are: an Idris-side renderer inside the workbench (no Rust,
no corpus tie without an export step), a small exporter plus a Rust checker, or
a new crate. Pick with the user; do not default.

## Consumption boundary

Undetermined by design — that is the round's first deliverable. Whatever is
chosen: `idris/src/Experimental/Cards.idr` (the 329 entries, read-only —
rendering must not require editing them), `idris/src/Experimental.idr` and
`idris/src/Experimental/Macros.idr` (the `-- spelling:` comments and the macro
layer), plus at most one Rust crate that is **not** on the cutover deletion
list. `plugins/wizards/cards/`'s 7,294 authored `.ron` cards are the corpus side
of any diff.

## Acceptance

- The home is settled and recorded before code lands; no deletion-slated crate
  is touched.
- A repeatable command renders bench terms and reports a pass/fail count against
  printed Oracle text; the first count is recorded whatever it is.
- Every mismatch is classified as one of: the term is wrong, the renderer is
  wrong, or the printed text has a surface the grammar deliberately normalises —
  and the third category is enumerated, not used as a catch-all.
- The `Expanded` collision is answered explicitly, one way, before the renderer
  is built on top of it.
- The 13 `-- spelling:` comments are checked against what actually renders and
  are corrected or discharged.
- No bench entry is edited to make the renderer pass.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
