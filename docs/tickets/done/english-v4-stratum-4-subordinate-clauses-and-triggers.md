---
needs: [english-v4-stratum-3-residual]
---
# Stratum 4: Subordinate clauses, triggers, and conditionals

Implement **Stratum 4** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§3, §4, §6.1):

- **Triggered Abilities (`[CR#603.1]`):** Clauses introduced by `When`,
  `Whenever`, or `At` (`At the beginning of ...`, `Whenever ~ attacks, ...`),
  including intervening-`if` clauses (`When/Whenever/At <event>, if <cond>,
  <effect>`).
- **Clausal Prepositions and Subordinators:** Conditional (`if`, `unless`, `as
  long as`), temporal/duration (`until`, `while`, `before`, `after`, `since`,
  `once`), causal/concessive/manner (`because`, `as`, `as though`, `as if`,
  `without`, `by`, `instead of`, `rather than`, `except`), and verbless clauses
  (`if able`, `if tapped`, `unless controller pays {2}`).
- **Content Clauses and Variable Supplements:** Declarative and subjunctive/mandative
  content clauses (`that ...`, `whether ...`) and supplementary variable
  definitions (`, where X is ...`).

## Gate

- 100% of corpus units in Strata 0–4 parse as **read** (0 `known-wrong-only`)
  and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s`
  and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Stratum 0–4 judgment suite passes with zero regressions.

Standard constraints apply.

Census delta (`pkqyzzvz` → `qsruvtzr`): faces read 9,931 → 18,780; total Readings 93,673 → 159,769; lost faces: none. Strata 0–4: 71,870 units read, 20 deferred to `english-v4-stratum-4-residual`, 0 known-wrong-only, 0 unread, 0 law failures.
Judgment set: seed and cumulative Strata 0–4 pass (1,948 enforced, 1,106 pending); tests and clippy pass for deckmaste_syntax, deckmaste_lexical_source_v4, deckmaste_english_v4 and xtask; citation checks pass.
