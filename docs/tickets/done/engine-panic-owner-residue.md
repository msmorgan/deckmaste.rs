---
needs: []
---
**Engine: every production panic site now names an owner.**

`engine-todo-triage` (done) gave every production `todo!`/`unimplemented!` in
`crates/deckmaste_engine/src` a durable, mechanic-specific diagnostic and
appended `; owner: <slug>` wherever an existing ticket clearly owned the site.
It deliberately left the remainder unowned — an over-eager wrong owner is worse
than none, because it sends the next reader somewhere the work isn't. This
ticket scoped that remainder.

## Outcome

**64 of 64 sites owned** (was 51 of 63; the count rose by one because the
combined producer/search cost-binder arm in `activate.rs` split into two arms
with separate owners). 13 tickets minted — 11 in `planned/`, 2 in `maybe/`.

## The inventory in this ticket's original text was short

It listed three seams pointing at `done/` tickets. Joining every `owner:` slug
against the ticket folders instead found **five owners across sixteen sites**:
it missed `core-action-riders-cost-modes` (3 sites), `engine-resolve-effects`
(2), and `engine-target-distinctness` (1). It also listed
`engine-turn-modification` as a done ticket; it is in `planned/`, so that seam
had a perfectly good owner all along.

Where a mechanical query can answer a question, prefer it to a hand-written
list — the query is exhaustive and the prose list is one person's grep.

## The twelve unowned sites

| Site | Disposition | Owner |
|---|---|---|
| `activate.rs` producer cost binder | mint | `engine-produce-capture-binder` |
| `activate.rs` search cost binder | mint | `engine-library-search-primitive` |
| `resolve/effect.rs` `Produce` outside `With` | mint | `engine-produce-capture-binder` |
| `resolve/effect.rs` `Search`/`SearchOne` | mint | `engine-library-search-primitive` |
| `resolve/effect.rs` non-verb `Simultaneously` member | mint | `engine-simultaneous-non-verb-members` |
| `resolve/effect.rs` choice-bearing `Simultaneously` member | mint | `engine-simultaneous-choice-members` |
| `resolve/effect.rs` stage-3 effect catch-all | mint | `engine-piles` |
| `resolve/query.rs` `Selection::PilesOf` | mint | `engine-piles` |
| `condition.rs` `Crossed` channel | mint | `engine-crossed-channel-scope` |
| `decide/pending/cast.rs` `Must(Target)` vs trigger | mint | `engine-deed-agent-ability-kind` |
| `replace.rs` enters-replacement catch-all | mint | `engine-enters-replacement-compose` |
| `trigger.rs` snapshot filter catch-all | mint | `engine-snapshot-predicate-breadth` |
| `sba.rs` SBA effect kinds | residue | `docs/decisions/state-based-actions-are-data.md` |

## The sixteen done-ticket sites: five residue, eleven orphaned

Confirmed intentional residue, wording specific enough to recognise from the
panic: `RelatedBy` in both `target.rs` and `trigger.rs` (`engine-filter-breadth` says
it "stays a seam", for want of a linked-ability relation registry
[CR#607.1]); both `Unless`
cost seams in `decide/mod.rs` (`engine-resolve-effects` states its v1 boundary
verbatim); `Produce` over a non-`Move` action in `resolve/effect.rs`
(`engine-find-moved-object` describes exactly that cut).

Orphaned and reassigned:

- 4 frameless-carrier arms in `target.rs` → `engine-frameless-carrier-threading`.
  `engine-static-scope-carrier` says outright that `engine-filter-breadth`
  "added the `ControlledBy`/`Ref` match ARMS but did not thread the carrier",
  then fixed only the derived/continuous-effect path. `activate.rs` and
  `replace.rs` were never touched.
- 3 needing a full `Frame` rather than a watcher → `engine-candidate-frame-context`.
  Their code comments assert the shapes are "vanishingly rare" with no corpus
  check behind it, so that ticket audits reachability before building.
- 1 snapshot attachment relation in `trigger.rs` →
  `engine-snapshot-attachment-capture`. `engine-filter-breadth` promised
  `attached` "wires on refresh"; the follow-up `engine-attachment-references`
  scoped itself to the live matcher. Two tickets each expected the other to do
  it.
- 3 enter-rider sites → `engine-enter-rider-execution`.
  `core-action-riders-cost-modes` self-scopes as "the action-vocabulary half of
  the grammar reshape" and shipped no engine execution;
  `engine-find-moved-object` independently recorded that gap as why it could
  not use a real blink card.

## Findings worth carrying forward

**A supported canon card panics.** `Avarice Totem` (`supported: true`) activates
into a `Simultaneously` of two `Continuously` members, which the resolver
rejects. Only render and fidelity tests touch the card, so CI never activates
the ability. Its file comment shows the shape was chosen deliberately to get a
working Idris emit path, accepting the engine gap. Widening the match arm is
the wrong fix: a `Continuously` member mints a static row rather than events,
so it would pass the batch's emptiness check contributing nothing and silently
break the all-or-nothing guarantee [CR#701.12a].

**`Expanded` was dead code inside two catch-alls.** `lower()` erases it, and
27 sibling arms across 15 engine files already say
`unreachable!("provenance erased at lower")` — `runtime-prose-link`'s gate even
asserts "no `Expanded` match sites left in `deckmaste_engine`". These two were
missed because their `Expanded` case was never a named arm to grep for; it was
absorbed by the pre-existing catch-all. Both now carve it out explicitly, so
the seams report only genuinely unbuilt shapes.

**A rejected fold.** `core-do-or-die-divide-and-choose` names the stage-3 pile
seam in its blocker 2, which reads as ownership but is not: the same blocker
recommends "splitting into a `core:` emit/render/RON slice vs a separate
engine-resolution ticket", and the ticket separately carves the general N-pile
case out for Whims of the Fates. `engine-piles` is that separate ticket. Its
blocker 2 also records that `ChoiceContinuation::ArrangePiles` is scry-style
ordering within a known pile, not partition-and-choose — a reuse dead end worth
knowing before pickup.

**Two stale pointers fixed.** `engine-cost-modification` quoted
`todo!("P0.W2 residue…")`, a message retired repo-wide; it now names the two
functions and the owning ticket instead of quoting a volatile string, since
quoting is what went stale. `deckmaste_core`'s `Binder` doc pointed at
`resolve.rs`, gone since the module split.

## Residue

`sba.rs`'s catch-all is a design boundary, not unbuilt work, so its owner is a
tracked ADR rather than a ticket: `state-based-actions-are-data` scopes the
data-driven `SbaRule.then` path to unconditional effects because "choice cannot
be represented by an unconditional effect", and choice-bearing SBAs stay
imperative native Rust. Recorded in the `owner:` slot as the decision path.

Gates: workspace tests green, clippy clean, `cite check` 0 non-compliant and 0
stale, `cite bless` registered 8 new rules, the `cite audit --diff` read by
hand across 60 sites (two ticket-prose citations were hash-valid but
wrong-topic and were corrected), `scripts/todo check` clean.
