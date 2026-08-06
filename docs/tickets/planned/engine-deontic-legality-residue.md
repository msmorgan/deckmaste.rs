---
needs: []
---
**Engine: deontic rows outside the Cast arm go unevaluated.**

`guard_deontic_seam` in `crates/deckmaste_engine/src/legal.rs` trips loudly for
any deontic row shape the legality evaluator doesn't yet interpret. It is a
*presence guard*, not a fizzle: a row exists in the derived view and would
change legality, so silently ignoring it would be wrong ([CR#101.2] — a
"can't" effect beats a permission; [CR#601.3] — a player may begin casting
only if no effect prohibits it).

`core-casting-restrictions` (done) narrowed the Cast arm. The residue is the
non-Cast deontic actions (Attach, Target, Play, …) and the non-`Gate`
polarities (May / Cant / Must) over them.

Fix: enumerate `DeonticAction` × polarity, decide which combinations the
corpus actually produces, and evaluate those; keep the guard loud for the
rest. See `docs/conformance.md` for the four-polarity model — full
constraint arbitration (maximize-without-violating) is a separate, later
concern.

Effort: **M**.
