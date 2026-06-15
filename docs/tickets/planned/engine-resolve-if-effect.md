---
needs: []
---
Interpret `Effect::If { condition, then, otherwise }` at resolution. Today
`GameState::run_effect` (`crates/deckmaste_engine/src/resolve.rs`) handles `Act`,
`Sequence`, `Continuously`, and `Expanded`, but an `Effect::If` node falls
through to `todo!("stage 3 does not interpret effect …")` (the choice seam) and
PANICS. Wire it: evaluate the branch via the existing `condition_holds(condition,
frame)`, then schedule a `RunEffect` for `then` (or `otherwise`, when present);
no-op when the condition is false and there is no `otherwise`.

Blocks the SPELL form of Ascend (CR 702.131a): the build-time fold (engine-
citys-blessing Task 7) emits
`Spell(effect: Sequence([If(<gate>, GetDesignation), <original effect>]))`, so
EVERY folded Ascend spell panics on resolution. The three e2e cases pinning the
grant-then-read sequencing live `#[ignore]`d in `resolve.rs`'s test module
(`ascend_spell_grants_then_reads_at_ten` / `_no_blessing_below_ten` /
`_no_high_water_mark`); the unignored `diag_setup_is_sound` proves the fixture is
correct (gate reads 10/9, `Draw(3)` lands three) — only the `Effect::If`
interpreter is missing. Un-ignore those three when this lands (they must pass as
written — the ten case draws 3 and holds the blessing).
