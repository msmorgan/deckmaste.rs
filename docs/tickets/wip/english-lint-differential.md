---
needs: [english-shape-rarity]
---
**Lint oracle-text substrings that parse to divergent subtrees across cards.**
A free correctness oracle extracted from the corpus itself: no ground truth and
no external table are needed, because the corpus is its own control. If `each
creature that player controls` yields one subtree on card A and a different one
on card B, at most one can be right — the divergence alone is the finding.

- Group supported faces by shared oracle substring; compare the subtrees the
  shared span lowers to; report substrings whose subtree set has more than one
  member.
- **Context guard required — this is the ticket's real work.** Some substrings
  legitimately parse differently by context (attachment genuinely governed by
  surrounding material, homographs, differing constituent boundaries at the
  span edges). Without a guard this lint reports legitimate variation as
  defects and forfeits the machine-only adjudication contract the
  `english-lint-*` set is built on.
- Soundness bar: report only divergences the guard proves cannot be
  context-licensed. If the guard cannot be made sound, downgrade this to an
  advisory report feeding `english-shape-rarity`'s worklist rather than a lint
  — do not ship a lint that needs triage.

Weakest soundness of the `english-lint-*` set; sequence it last.

Standard constraints apply.

## Completion

`divergent-parse` in `xtask/src/english/lint.rs` — **0 findings**, over 52,827
abilities paired with their source line and 3,297 texts shared by two or more
cards.

The context guard the ticket demanded turned out to be tractable rather than
impossible, because the context that can legitimately change a parse is exactly
what `parse_with_identity` consumes:

- abilities naming their own card are dropped (self-reference makes identical
  text legitimately parse differently), and
- `is_legendary` is part of the grouping key rather than something two cards in
  a group may differ on.

Pairing is conservative — an ability is compared only when a face's line count
matches its ability count, so a multi-line ability never mispairs.

The check therefore stays a lint rather than being downgraded to an advisory
feed. It reports a genuine proven invariant, and the run prints its own coverage
(`abilities paired`, `texts shared`) specifically so a silently no-op'd check
cannot be mistaken for a clean one — the failure mode this ticket warned about.
