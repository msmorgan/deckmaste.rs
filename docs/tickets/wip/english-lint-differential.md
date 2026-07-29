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
