---
needs: []
---
A 2026-07-16 adversarial audit of every `#[allow]`/`#[expect]` in the workspace
(each site's `reason =` claim checked against the code, enum sizes measured with
`-Zprint-type-sizes`) found a crisp rot pattern: **every stale or false
suppression was an `#[allow]`; every accurate one was an `#[expect]`** —
`#[expect]` errors (`unfulfilled_lint_expectations`) the moment its lint stops
firing, `#[allow]` lingers silently forever. This ticket clears the rot and
turns on the mechanical enforcement that makes the class unrepresentable.
(Enum-size root fixes are split out to `engine-event-size-boxing`; the
oversized-fn placeholders stay tracked in `refactor-oversized-fns`.)

## 1. Delete dead suppressions (fire on nothing today)

- `deckmaste_engine/tests/planeswalker_jace.rs` `#![allow(clippy::too_many_lines)]`
  — longest fn 63 raw lines vs threshold 150.
- `deckmaste_engine/tests/planeswalker_chandra.rs` same — longest 74.
- `deckmaste_engine/tests/activate.rs` same — longest 130. All three went dead
  when `clippy.toml` raised the threshold 100→150 and were never removed.
- `deckmaste_engine/src/layer.rs` `#[allow(clippy::match_same_arms)]` on
  `apply` — since the count-bearing/count-free split, `apply` has three
  distinct arms and the lint cannot fire; the comment describes stubs that now
  live in `apply_static`.
- `deckmaste_engine/src/cast.rs` `#[allow(clippy::unnecessary_wraps, ...)]` on
  `GameState::mana_cost` — never fires (clippy's `avoid-breaking-exported-api`
  default skips pub methods; the reason string itself concedes this). Keep the
  doc comment documenting the `Option` cast-legality seam; delete the attribute.

## 2. Fix instead of suppress

- **xtask `needless_pass_by_value` ×7** (`cite.rs` `dispatch`, and `run` in
  `extract.rs`, `generate.rs`, `graduate.rs`, `stubs.rs`, `resolve.rs`,
  `idris_check.rs`): every body only reads `args`; the sole dispatch is the
  `match` in `bin/cargo-xtask.rs`. Change the seven signatures to `&FooArgs`,
  the seven call sites to `run(&args)`, delete the seven allows.
- **`deckmaste_tui/src/driver.rs` `run_to_priority` / `submit`**, bare
  `#[allow(dead_code)]` ×2: all callers are inside `#[cfg(test)]` modules
  (`ui/zones.rs`, `ui/board.rs`, `ui/format.rs`, `interact.rs`), and `mod
  driver` is private so `pub` doesn't save them in non-test builds. Mark both
  `#[cfg(test)]`; drop the allows. (Their doc comments say "integration tests"
  — they are unit tests; fix the wording.)
- **`deckmaste_engine/src/cast.rs` `auto_pay`** `#[allow(dead_code, reason =
  "... canonical pure form for runners/tests")]`: the runners half is false —
  the fn sits in a private module and `lib.rs` never re-exports it, so no
  runner *can* call it; only its own unit tests do. Either `pub use
  cast::auto_pay` in `lib.rs` (if the pure subject-free form is genuinely
  wanted by runners — then `dead_code` stops firing) or drop `pub` and move it
  under `#[cfg(test)]`. Also fix the stale doc comment in `activate.rs` (near
  the `auto_pay_pending` call, ~line 1144) that still names `auto_pay`.
- **`deckmaste_engine/tests/stack.rs`** module-wide
  `#![allow(clippy::too_many_lines)]` grandfathers ~25 tests that don't need
  it. Replace with per-fn `#[expect(..., reason = ...)]` on the real offenders
  (only ~5 exceed 150 raw lines; after comment/blank stripping possibly just
  `occurrence_batch_and_apnap_ordering` at 292). That test also hand-rolls a
  `GameConfig` + seed-search block the named `*_game` builders elsewhere
  encapsulate — port it while there.
- **`deckmaste_engine/src/decide.rs`** `.expect()` on `mana_cost` (~line 1245,
  blocker mana-value read): panics the day a no-mana-cost face returns `None`
  — the exact future the `mana_cost` seam documents. Engine never panics on
  card data: replace with a graceful default (treat as mana value 0) and a
  comment.
- **`deckmaste_engine/src/layer.rs` `eval_divide`** (`cast_possible_truncation`
  expect, RoundUp arm): the "quotient bounded by a ≤ i32::MAX" argument is
  true only because the sole caller clamps both operands `.max(0)` — one new
  caller away from silently invalid. Add the `.max(0)` clamp (or a
  `debug_assert!`) inside `eval_divide` so the invariant is caller-proof.

## 3. Add missing `reason =` to structurally-justified bare allows

- `deckmaste_migrations/src/resolve.rs` `flying_only`
  (`unnecessary_wraps`): required by the `AbilityParser` fn-pointer table
  signature — say so.
- `deckmaste_engine/src/layer.rs` `apply_static`
  (`match_same_arms`): four identical `{}` stubs with per-arm comments and
  backing tickets (`engine-layers-misc`, `engine-layers-1-copy-facedown-text`).
  Staying `#[allow]` is deliberate — an `#[expect]` would churn every time one
  stub diverges while others remain — but the reason must say that.
- `deckmaste_tui/src/ui/mod.rs` `render` / `render_zone`
  (`too_many_arguments` ×2): convert to `#[expect]` + reason; optionally
  fold the shared params into a context struct (nicer, not required here).
- `deckmaste_core/src/ability.rs` `Ability` (`large_enum_variant`, bare):
  measured `Triggered` = 1352 B vs `Spell` = 592 — "balanced leaf" is not
  true. Point the reason at `engine-event-size-boxing`, which owns the
  investigation.

## 4. Enforcement

After 1–3, add to `[workspace.lints.clippy]` in the root `Cargo.toml`:

```toml
allow_attributes_without_reason = "warn"
```

so every future `#[allow]`/`#[expect]` must carry a `reason`. Stretch:
`allow_attributes = "warn"` (forces `#[expect]` wherever the lint can fire —
the stronger self-clearing guarantee). Before enabling the stretch, verify the
`#[allow(non_camel_case_types)]` that `macro_ron_derive` emits into generated
helper structs doesn't trip it at expansion sites; if it does, keep only the
`without_reason` form.

## Verification

`cargo clippy --workspace --all-targets` clean (converted `#[expect]`s
self-verify: any that stop firing error as `unfulfilled_lint_expectations`).
Full test suite once, per house rule.
