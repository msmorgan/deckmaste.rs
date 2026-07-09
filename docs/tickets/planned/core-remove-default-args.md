---
needs: []
---
Strip the ~20 existing `default`-valued player/reference arguments from `idris/src/Core.idr`
grammar constructors, making every call site pass the reference explicitly.

## Why

Implicit `{default You …}` / `{default This …}` / `{default [Library] …}` arguments on grammar
constructors complicate the Rust→Idris emit round-trip (`cargo xtask idris-check`): the emitter
must decide whether to emit the argument or rely on the default, and the two must agree exactly.
New constructors added since mid-2026 all take their player/reference argument as a **required
explicit positional** for this reason; this ticket brings the pre-existing constructors in line.

## Scope

The `default`-valued sites live on constructors including (verify the current set by grepping
`idris/src/Core.idr` for `default You`, `default This`, `default [Library]`, and similar):
`TopOfLibrary`, `Draw`, `Search`, `MayPay`, `Vote`, `DivideAndChoose`, and `Countable.ManaSpent`'s
`default This`, among others (~20 total).

For each: remove the `default`, make the argument a required positional, and update:
1. the emitter (`crates/deckmaste_cards/src/idris_emit.rs`) so it always emits the argument;
2. every canon/builtin card RON that relied on the default (they must now spell the reference);
3. regenerate `plugins/wizards` (grammar changed) — `rm -rf plugins/wizards && cargo xtask generate plugins/wizards`.

## Gate

`cd idris && ./scripts/build` PASS; `cargo xtask idris-check plugins/canon` no regressions;
`cargo xtask fidelity` PASS; `cargo test -p deckmaste_core -p deckmaste_cards` green;
`cargo clippy --all-targets -- -D warnings` clean. The blast radius (emitter + card RON + wizards
regen) is why this is its own ticket rather than folded into a feature.
