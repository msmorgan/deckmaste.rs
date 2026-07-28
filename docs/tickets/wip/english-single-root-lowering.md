---
needs: []
---
**`parse_nonterminal_with_profile` selects one best root and fails outright
when lowering declines — the next-best root is never tried.** Root-caused
2026-07-25 (round impclause, Stage-0 experiment): `grammar/mod.rs`
(`parse_nonterminal_with_profile`) picks a single `best_root` and maps a
lowering failure to `ParseNonterminalError::Lowering` with no retry.
Confirmed witness: `copy that spell` under `Nonterminal::Clause` returns
`Err(Lowering)` — the chart licenses the imperative fine (`Sentence` reduces
ONLY through `Clause`, so licensing was never the issue), but with
noun/verb-homograph heads (`copy`, `return`, `tap`, `destroy`, `sacrifice`)
the `Clause` goal is genuinely ambiguous and a non-lowerable root wins the
tiebreak; the extra `Sentence → Clause` layer changes which root wins, so
the same span succeeds under `Nonterminal::Sentence`. Two staged
workarounds now exist that a proper fix (try the next-best root when
lowering declines, or make lowering-viability part of root selection) would
let us DELETE: `parse_cost_component` (grammar/ability.rs, pre-existing)
and `parse_trigger_effect` (round impclause). Latent exposed consumer with
no corpus witness: `parse_dash_appositive`'s matrix. CAUTION: this is a
core-parser change with whole-corpus blast radius — needs its own round
with corpus-wide roundtrip and selection-stat gates on a broad control set;
a changed root selection is by definition a selection-stat change, so the
gate design must distinguish "previously-failing span now succeeds" (the
win) from "previously-succeeding span selects differently" (a stop).

## Completion

`ParseForest::best_root_matching` now computes the packed choices once, ranks
complete roots by the unchanged `(ParseCost, stable NodeId)` ordering, and
allows the grammar boundary to choose the first root that lowers. The original
winner is therefore returned unchanged whenever it already lowers; only a
previous `Lowering` result can advance to another root.

`copy that spell` now lowers directly as an imperative `Clause`. The causal
test runs the former single-root algorithm beside the new one: it proves the
old path returns `Lowering`, then proves the fallback produces an imperative.
A broad successful-control test compares root, root rule, cost, tied
alternatives, chart and forest statistics, and syntax against the former
algorithm, pinning the no-reordering half of the contract.

The redundant `Clause` → `Sentence` retries in activation-cost components and
trigger effects are deleted. Existing trigger and cost tests cover `copy`,
`return`, `tap`, `destroy`, and `sacrifice` imperative heads through the single
`Clause` path. The full supported-corpus recovery census is byte-for-byte
unchanged at 3,477 structural spans / 64,284 source tokens, as expected: the
removed local retries had already hidden the core defect from corpus totals.
