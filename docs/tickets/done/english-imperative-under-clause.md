---
needs: []
---
**Bare-imperative trigger effects — DONE (round impclause, 2026-07-25),
with the original premise corrected.** This ticket's original headline
claim ("bare `IndependentClause::Imperative` does not reduce under
`Nonterminal::Clause`") was FALSE: `N::Sentence` is headed by exactly two
rules, both through `N::Clause`, so anything reducing under Sentence
reduces under Clause by construction, and the imperative features/lowering
arms exist and work. The real mechanism (Stage-0-confirmed by experiment):
single-root lowering — `parse_nonterminal_with_profile` picks one best
root and returns `Err(Lowering)` when it fails to lower, never trying the
next-best; with noun/verb-homograph imperative heads (`copy`, `return`,
`tap`, `destroy`) the `Clause` goal selects a non-lowerable root, while
the `Sentence` goal's extra layer selects a lowerable one. Fixed at the
one witnessed call site by `parse_trigger_effect` staging Clause →
Sentence (the `parse_cost_component` idiom); clause −177/−2843. The core
defect is ticketed separately as english-single-root-lowering. Residue
after this round: 28 multi/coordinated-effect groups (midtrigger §2.3
discount) and the 332 content-blocked coordinated-EVENT groups
(`recovery-harness/out/midtrigger-initial.txt`) — event-side gaps, now
the largest remaining trigger diagnosis.
