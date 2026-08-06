---
needs: []
---
**Engine: the `Vote` decision is shaped but has no producer and no handler.**

`crates/deckmaste_engine/src/decide/pending/choice.rs` defines `Vote { player,
options }` and `strategy.rs` can answer one, but nothing ever constructs a
`PendingDecision::Vote`, and `Vote::resolve` is unbuilt.

[CR#701.38a]: players vote for one choice from a list of options; voting
starts with a *specified* player — not necessarily the active player — and
proceeds in turn order. The effect then depends on the tally. Building this
needs (a) a producer — the effect kind that opens a vote,
(b) a `ChoiceContinuation` that accumulates votes across players and resumes
once the last one is in, and (c) the tally semantics, including ties.

Note the separate parser-side ticket `english-voting-procedure`; this is the
engine half.

Effort: **M**.
