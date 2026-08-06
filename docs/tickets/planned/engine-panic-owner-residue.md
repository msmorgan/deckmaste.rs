---
needs: []
---
**Engine: twelve production panic sites still have no owning ticket.**

`engine-todo-triage` (done) gave every production `todo!`/`unimplemented!` in
`crates/deckmaste_engine/src` a durable, mechanic-specific diagnostic, and
appended `; owner: <slug>` wherever an existing ticket clearly owned the site.
It deliberately did **not** mint tickets for the remainder: an over-eager wrong
owner is worse than none, because it sends the next reader somewhere the work
isn't. These are the sites left for a human to scope.

Each needs a judgement about whether it deserves its own ticket, folds into an
existing one, or is a permanent documented residue like
`engine-cost-modification-residue`.

| Site | Unbuilt | Note |
|---|---|---|
| `decide/pending/cast.rs` | a `Must(Target)` row matching a placing trigger's source | the `by` filter can't distinguish a spell from a triggered ability as the targeting agent; `engine-retarget-corners` is a near neighbour but doesn't cover it |
| `replace.rs` | enters-replacement fold catch-all beyond self-count and `If` | needs the list of `OneShotEffect` shapes an `AsEnters` fold must support |
| `resolve/query.rs` | `Selection::PilesOf` — labelled pile groups at runtime | the code names `engine-piles`; no such ticket exists. `core-do-or-die-divide-and-choose` is narrower |
| `condition.rs` | `Condition::Crossed` with no before/after channel in the frame | which non-chapter-trigger context evaluates `Crossed`, and how the frame carries the channel there |
| `activate.rs`, `resolve/effect.rs` ×2 | `Binder::Produce` / `Search` / `SearchOne` unwired at runtime | `parse-tutor-search` (done) covered only the parser side; the engine needs a produce-and-capture and a library-search primitive |
| `sba.rs` | an SBA's own effect supports only `Act`/`Sequentially` | which choice-bearing effects an SBA can carry, and how a decision surfaces from an SBA sweep |
| `resolve/effect.rs` ×2 | `Simultaneously` members restricted to pure verbs | extending the exchange-family construct to non-verb and choice-bearing members |
| `resolve/effect.rs` | stage-3 effect interpreter catch-all ("the choice seam") | spans many `OneShotEffect` variants — scope per-variant before ticketing |
| `trigger.rs` | stage-3 snapshot filter catch-all | same, for `Predicate` variants unevaluated against an LKI snapshot |

Also verify three assignments that point a live seam at a **done** ticket —
`engine-filter-breadth`, `engine-turn-modification`, `engine-find-moved-object`.
Either the done ticket documents the residue as intentional (as
`engine-durations-grants` and `engine-cost-modification` do), or the seam is
orphaned and needs a fresh owner, the way
`engine-granted-prevention-rows` was minted after `engine-prevention` closed
without unblocking its arm.

Effort: **S** as bookkeeping; the implementations it spawns are separate.
