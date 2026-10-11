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

## Fixture measurements

Before: `swxynlzl`; after: `vlxnywvr` (before refresh). Each exact fixture has one Reading before and after. Prediction baselines come from the diagnosis replica, whose items/families/completed totals were checked against the real parser on every fixture; after predictions come from `Metrics::predicted_items` directly. Probes and the independent-value suite verify the Reading sets. No runtime test toggle exists; the three regressions use complete typed Reading-set equality.

| Fixture | Items before → after | Families before → after | Predictions before → after | Completed nodes before → after |
|---|---:|---:|---:|---:|
| Brothers of Fire | 7,988 → 4,813 | 8,202 → 5,027 | 6,870 → 3,695 | 194 → 194 |
| Fireslinger | 7,875 → 4,704 | 8,059 → 4,888 | 6,843 → 3,672 | 164 → 164 |
| Goblin Artillery | 7,875 → 4,704 | 8,059 → 4,888 | 6,843 → 3,672 | 164 → 164 |
| Orcish Artillery | 7,875 → 4,704 | 8,059 → 4,888 | 6,843 → 3,672 | 164 → 164 |
| Orcish Cannoneers | 7,875 → 4,704 | 8,059 → 4,888 | 6,843 → 3,672 | 164 → 164 |
| Psionic Entity | 7,869 → 4,698 | 8,051 → 4,880 | 6,843 → 3,672 | 163 → 163 |
| Reckless Embermage | 7,936 → 4,762 | 8,135 → 4,961 | 6,860 → 3,686 | 180 → 180 |
| Forge Devil | 9,290 → 5,738 | 9,505 → 5,953 | 8,135 → 4,583 | 179 → 179 |
| Artifact damage/gain (two lines) | 12,295 → 7,578 | 12,618 → 7,901 | 10,716 → 5,999 | 287 → 287 |

The maximum is 4,922 items and 5,099 families below the unchanged ceilings. Goblin Artillery saves 40.3% of items; the artifact fixture saves 38.4%. These are smaller than the diagnosis estimate: 438 and 820 additional dead dot-0 items, respectively, have a scannable lexical category but fail feature admission. CH1 retains them. Every measured item reduction equals its prediction reduction, and `items - predicted_items` is identical for each fixture.

Refresh incorporated `english-v3-independent-genitive-pronoun` (default completion `zkyoxmnx`), changing 12 crate paths. On `vlxnywvr` after refresh, the first seven fixtures each add one predicted item/family, Forge Devil adds two, and the artifact fixture adds three; every completed-node count in the table remains identical. The current maximum is 7,581 items / 7,904 families, leaving 4,919 / 5,096 below the unchanged ceilings. Current counters are in `refreshed-fixture-metrics.tsv`; the full census pair above predates this refresh.

## Landing record

1. **Census delta** (`swxynlzl` → `vlxnywvr`, before refresh): all 32,828 faces compared; coverage 30,530 → 30,530; Readings 717,212 → 717,212. Lost faces: none. Gained / decreases / increases / digest-only: 0 / 0 / 0 / 0; source changes: 0. All faces also retain equal completed nodes/families, completion work, and scan work. Full reports and `diff.json` are in the evidence directory named above.
2. **Witnesses** (`vlxnywvr`): all nine exact fixtures in the measurement table retain one complete Reading each, with identical probe Reading values and completed-node counts. Fireslinger, Goblin Artillery and Forge Devil additionally compare complete independently constructed Reading sets by value while asserting fewer items. Both roundtrip laws and the original nine-witness resource assertions pass.
3. **Spot checks** (`swxynlzl` → `vlxnywvr`): no newly covered faces. Ten unchanged checks, each one Reading: Brothers of Fire, Fireslinger, Goblin Artillery, Orcish Artillery, Orcish Cannoneers, Psionic Entity and Reckless Embermage retain activated abilities with the parallel selected recipient clusters; Forge Devil retains its preposed trigger Clause and those clusters; Char and Arc Trail retain the shared lexical head with complete parallel recipient tails. Their census counts/digests are identical and their independent structures pass the existing suite.
4. **Deviations and STOPs** (`vlxnywvr`): no grammar, lexicon, Construction, Frame or existing assertion changed by this feature; no STOPs or glossary gaps. Two additional runtime boundary tests, `prediction_skips_only_terminals_that_the_scanner_cannot_match` and `prediction_retains_empty_literals_and_nullable_first_nonterminals`, supplement the required three witness regressions to check CH2's soundness boundaries. The public `predicted_items` counter is measured by a probe under `target/`; xtask's ten-field chart map and its existing assertion stay unchanged. The smaller saving than the simulation is explained above; no feature-admission filter was added.
5. **Tests** (`vlxnywvr`): `cargo test -p deckmaste_english_v3` passes before refresh (1,102 passed, one pre-existing ignore) and after refresh (1,107 passed, the same ignore); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passes before and after refresh; `cargo test -p xtask english_v3` passes before and after refresh (56 passed each). Assurance: restored 0 / re-spelled 0 / newly ignored 0 / added 5 / removed 0; no removals. The inherited ignore is `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`, blocked by `english-v3-mana-symbol-verbless-subject`: MS4 needs a governed Mana Phrase Subject route keeping bare {X} and {2} outside ManaPhrase. Citation checks: 0 noncompliant, 0 stale, no new rule registrations.
6. **Timing**: after full census `vlxnywvr`: 4 min 14.72 s corpus wall, six workers, samples per face 0, no Reading limit. Before `swxynlzl`: 7 min 23.15 s with the same settings; measured reduction 42.5%. Census-start host load differed (21.21 → 4.61). Build and setup time are excluded from these corpus-wall figures; invocation timing is retained in `before.time` / `after.time`.
