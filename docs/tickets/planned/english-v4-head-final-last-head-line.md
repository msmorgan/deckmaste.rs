---
needs: []
---
# `head_final` tracks the last head-line child; no coordinator special case

From `english-v4-child-name-control-and-category-fission-audit`, finding E.

## The defect

`Production::track_head_final` (`crates/deckmaste_syntax/src/step.rs`, about
line 692) leaves `State::head_final` unchanged across a coordinator leaf, but
only when `Production::coordinator` is set. That way the item before the last
conjunct records whether the first conjunct ended in its head.
`edge_key` (`reading.rs`, about line 287) reads it there for the Attachment
Class "a dependent after a coordination attaches to the whole"
(`docs/decisions/english-v4.md` §4.3).

`attachment_key` recognizes the coordination by the child names `first` and
`last`, not by `Production::coordinator`. Two authored Productions carry those
names and an ordinary `coordinator` leaf, but no `Production::coordinator`:
ComparativeBoundedAlternative and ComparativeBoundedAmount (`english.ron`,
about lines 867 and 874). For them the forest key reads `false` where the tree
key (`reading.rs`, about line 582) computes the first conjunct's real value. No
judgment trips it yet.

## Change

- `State::head_final` means "the last head-line child so far ends in its
  head". A head-line child sets it (a leaf to true). Every other child,
  punctuation and a coordinator leaf included, leaves it unchanged.
- `Summary::head_final` is that value when the Production's last
  non-punctuation child is in the head line, and false otherwise.
- Remove the coordinator special case, and update the doc comments in
  `step.rs` and `reading.rs` and the `deckmaste_syntax` README paragraph
  ("The chart carries the first conjunct's `head_final` across the
  coordinator …").
- Add a `deckmaste_syntax` test: an authored `first`/coordinator/`last`
  Production whose last conjunct ends in a dependent its first conjunct lacks
  has the same forest and tree key.

No field is added. The audit's probe of exactly this change (P1) left the
census, every Reading count, Surplus and the cheapest tier identical, and
`ranking_keys` and the judgment set held.

The forest key must equal the tree key (`ranking_keys`). That is engine
consistency, not a ranking gate.

## Acceptance (`docs/decisions/english-v4.md` §4.4, principles 4 and 5)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (principle 4). No Attachment Class, Production Cost
tweak or residual ticket is added to win a rank.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
