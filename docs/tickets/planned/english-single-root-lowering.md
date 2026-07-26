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
