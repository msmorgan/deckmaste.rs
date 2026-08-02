---
needs: [macro-author-surface, idris-mirror-authoring]
---
Strip the ~20 existing `default`-valued player/reference arguments from `idris/src/Core.idr`
grammar constructors, making every call site pass the reference explicitly.

**Rescope (2026-08-01):** the `{default You …}` player-agent half of this
ticket is subsumed by the action role reshape
(`core-pay-player-action`, spec
`docs/superpowers/specs/2026-08-01-action-role-reshape-design.md`) — deleting
`Action::By` and giving player verbs explicit agent slots strips those
defaults wholesale, and `MayPay`/`MustPay` (incl. their `{default You
actor}`) are deleted outright. Remaining scope here: the non-player defaults
(`{default This …}`, `{default [Library] …}`, `Countable.ManaSpent`'s
`default This`, and kin). Coordinate with the reshape (the Idris constructor
churn overlaps; land this after it or fold the leftovers into its plan).

**Second rescope (2026-08-01, PARTIALLY WITHDRAWN 2026-08-02):** sequenced
behind `macro-author-surface` (the `needs:`), but the subsumption claim is
withdrawn: identity-macro scaffolds mirror existing constructor DEFAULTS
(byte-identical canon re-parse is that program's invariant — decision §5),
so the card-RON churn of spelling elided arguments explicitly does NOT
happen as a side effect there. It returns to this ticket, priced here:
default removal = authoring-mirror constructor changes + emitter + canon/
builtin RON re-spelling + hand macro bodies that elide `This`/`[Library]`.
Re-read every path below against post-rename crate names at claim time
(the Idris mirror is the authoring mirror; the emitter lives in the
renamed plugin crate).

**Third rescope (2026-08-02):** under the authoring/spelling/lowering
program
(`docs/decisions/authoring-spelling-lowering.md`),
the RON-side defaults live in `deckmaste_authoring` and the Idris mirror
attaches to the authoring kernel — so this ticket's constructor/emitter
work targets the authoring mirror, not core. Sequencing unchanged (behind
`macro-author-surface`).

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
1. the emitter (`crates/deckmaste_plugin/src/idris_emit.rs`) so it always emits the argument;
2. every canon/builtin card RON that relied on the default (they must now spell the reference);
3. regenerate `plugins/wizards` (grammar changed) — `rm -rf plugins/wizards && cargo xtask generate plugins/wizards`.

## Gate

`cd idris && ./scripts/build` PASS; `cargo xtask idris-check plugins/canon` no regressions;
`cargo xtask fidelity` PASS; `cargo test -p deckmaste_core -p deckmaste_plugin` green;
`cargo clippy --all-targets -- -D warnings` clean. The blast radius (emitter + card RON + wizards
regen) is why this is its own ticket rather than folded into a feature.
