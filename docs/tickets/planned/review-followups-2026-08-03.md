---
needs: []
---
**Low-severity follow-ups from the 2026-08-03 external review** (batched,
same shape as `review-low-severity-followups`).

- **Stringly verb dispatch:** interned-`Ident`-to-string-literal compares
  in rules paths — `sba.rs:~646` / `replace_registry.rs:~362`
  ("Destroy"), `eval.rs:~348` / `step/act.rs:~69` / `step/mod.rs:~867`
  ("Draw"), `step/act.rs:~122` ("Mill"), `decide/mod.rs:~375` /
  `resolve/effect.rs:~1622` ("Discard"), `eval.rs:~1132-1140`
  ("DayNight"/"Day"/"Night"). A typo is a silent behavior change, not a
  compile error. Hoist the names to shared consts (or a preinterned verb
  table); 51 `as_str() == "…"` sites in `deckmaste_engine/src` to audit,
  some in test modules.
- **Drifted test-helper copies:** `fn force_onto_battlefield` is
  copy-pasted across 9 engine integration-test files and the copies have
  diverged (`tests/layers.rs`'s falls back to library search;
  `tests/combat.rs`'s is hand-only); `card()`, `two_player_with()`,
  `plugin()` similar. Consolidate into `test_support` (or a shared
  `tests/` include) carrying the union of behaviors.
- **`run_effect` long arms:** `resolve/effect.rs:389` opens a ~1,000-line
  match. Flat dispatch is right for an interpreter, but the `Each`
  (~139 lines), `May` (~89), and `With` (~76) arms are extractable into
  named helpers without scattering the dispatch — the
  `split-step-rs`/`split-trigger-rs` treatment. The english crate's
  longest functions (`scan` 521 lines, `NominalModifier::build` 387,
  `lower_composed_clause` 370) are candidates of the same kind.
- **Dead harness:** `tests/todo_test.fish` (with its fixture tree
  `tests/tickets/`) is referenced by nothing — no CI step, no script.
  Wire it into CI next to the other gates or document how it runs; a
  harness nothing runs is rot bait.
- **Nit:** `replace.rs:16-20` hoists `#[cfg(test)] use` imports to module
  top instead of inside the test module.
