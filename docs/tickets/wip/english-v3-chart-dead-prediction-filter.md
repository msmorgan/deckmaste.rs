---
needs: []
---
# Filter dead chart predictions

Remove dead dot-0 Earley items in `deckmaste_english_v3::chart::predict` without changing grammar, lexicon, admission, or either roundtrip law. Standard constraints apply.

## Decisions (CH1–CH5, orchestrator, 2026-10-11)

CH1 chooses diagnosis option B: skip a production only when its first runtime symbol is a Lexical category with no scanner occurrence at this position, or a Literal that fails the scanner's exact token comparison. The completed forest, all Reading sets, and their digests must be identical.

CH2 requires a conservative soundness argument in code and here. A dot-0 item with a failing first terminal has no outgoing scan edge, never becomes a nonterminal waiter, and cannot complete. Lexical occurrences have strictly positive length. Optional fields compile to helper nonterminals with absent empty productions; retain those helper predictions and all productions beginning with a nonterminal, including nullable ones. Retain empty productions and empty literals (if supplied through the public runtime API). Never follow nonterminals to compute FIRST; uncertainty means retaining an item.

The read-only diagnosis at default's `target/english-v3/chart-diagnosis/` measured 7,847 items / 6,815 dot-0 predictions / 6,417 never advanced on the Goblin Artillery fixture, and 12,251 / 10,672 / 9,982 on the artifact damage/gain fixture. B simulated about −45% (approximately 4,266 and 6,758 items). Follow-ups only: A, general left-corner/FIRST filtering, −62..64%, needs a soundness argument for feature-dependent nullables; C, selected-frame expansion, −31%; D, clitic-juncture category, −12%.

CH3: full censuses before/after, sequentially, `cargo xtask english-v3 --all --workers 6 --samples-per-face 0`; `cargo xtask english-v3 diff` must report 0 lost, gained, decreases, increases, and digest-only across all 32,828 faces. All nine inherited recipient fixtures must retain equal completed-node counts; report items and families before/after for each.

CH4: add three parser regressions comparing exact Reading sets by value and demonstrating fewer items (or equal forests with a test toggle). Keep every existing assertion, especially the 12,500-item and 13,000-family ceilings. Add a cheap `predicted_items` metric if feasible. Append the observed numbers and suggested relational guards to hygiene-3 §C: `items(text) <= 2.0 * items(text without the second cluster)`, or bounds on `items - predicted_items` and completed nodes; changing the guard remains an owner decision.

CH5: record census wall times with six workers, run the English v3 suite and clippy plus `cargo test -p xtask english_v3`, commit, refresh, and re-run the English suite if refresh brings crates changes. Stop before integrate for review. Evidence lives in `target/english-v3/english-v3-chart-dead-prediction-filter/`.

## Landing record

1. **Census delta**: pending before/after measurements.
2. **Witnesses**: pending nine-fixture measurements.
3. **Spot checks**: no new coverage intended; identity census must confirm this.
4. **Deviations and STOPs**: none so far; no grammar or lexicon change authorized.
5. **Tests**: pending; no existing assertion may be removed or weakened.
6. **Timing**: pending sequential full censuses, six workers each.
