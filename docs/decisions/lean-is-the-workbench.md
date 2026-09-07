# Lean is the workbench

Decision: 2026-09-05.

## Decision

Lean succeeds Idris as the active semantics workbench. New syntax, checker
laws, macros, cards and proof pins belong in `lean/Semantics/`. The model
checks the card language's structural obligations; Rust remains the runtime
game engine. The separate `lean/English/` model is governed by
[English grammar design in Lean](english-lean-design-workbench.md).

Idris is reference only: reread its sentences, earlier obligations and
migration evidence, but do not extend its handwritten workbench. Shared
registry tooling may continue to regenerate the existing reference facts
module while that compatibility output remains supported. That maintenance
is not a second workbench-development lane.

## Checking and assurance

Lean syntax uses ordinary inductives. Checker functions compute attributes
and exact refusal lists; a `Spelled` card pairs its syntax with a proof of
`Card.check card = []`. Laws read structural and declared features. Open
labels are registry keys, and their checker facts are generated from those
declarations plus overlays for columns the declarations do not carry.

Inert vocabulary (ruling 2026-09-06, [Semantics v2 §16](semantics-v2.md)):
an enum no checker function reads is engine-facing vocabulary the syntax
carries opaquely. It owes no law, and its members are not twins to prune.

Pins are named theorems, normally `okX` / `badX`, proved by `decide`. A positive
pin fixes `check term = []`; its negative twin fixes the complete expected
refusal list. Retain the Idris names and card sentences as provenance when
re-spelling a migrated pin. A changed representation requires a corresponding
proof, not deletion or weakening of the existing assertion.

`lean/scripts/build` is the workbench gate, including the card bench and pin
suites, with warnings treated as failures. `cargo xtask facts check` separately
checks generated-table currency. These checks establish the behavior of the
workbench terms supplied to them.

## Card-gate succession

[Idris is a soundness gate](idris-is-a-soundness-gate.md) is superseded for
workbench choice and new modeling work. The executable card-validation boundary
moved with
[lean-card-soundness-gate](../tickets/done/lean-card-soundness-gate.md)
(2026-09-06): `cargo xtask lean-check` emits each expanded `plugins_v2/` card as
a Lean term and proves `Card.check = []` by `decide`, ratcheted per plugin. The
Rust-to-Idris emitter remains the legacy check for the v1 `plugins/canon`
corpus, which has no v2 counterpart until `plugins-v2-canon`; it retires with
that corpus. Validation examines the expanded semantic data on both sides.

Historical ADRs retain their original arguments and evidence paths. A dated
note points readers to the current Lean workbench without silently rewriting
a prior design decision.

## References

- [Lean workbench](../../lean/README.md)
- [Checker contracts](../../lean/CONTRACTS.md)
- [Keyword policy](../keyword-policy.md)
- [Conferrals come from registries](conferrals-come-from-registries.md)

## Appendix: migration record

The workbench began as a port of `idris/src/Experimental.idr` and its family.
The migration replaced constructor-index obligations with explicit attributes
and checking laws, preserving the named card sentences and pin outcomes.
This phrasebook records that migration; the Lean README describes current
authoring and checking.

### Idris → Lean phrasebook

| Idris | Lean |
| --- | --- |
| `%default total` | the default |
| `%unbound_implicits off` | `autoImplicit = false` (lakefile) |
| `import public` | plain `import` (Lean imports are transitive) |
| `public export` | the default |
| `data … where` | `inductive … where`, `deriving Repr, BEq` |
| `record … constructor MkFoo` | `structure Foo`, built with `⟨…⟩` or `{ … }` |
| `Maybe`/`Just`/`Nothing` | `Option`/`some`/`none` |
| `(a, b)` | `a × b` |
| `\|\|\|` docstring | `/-- … -/` |
| `k \/ k'` | `Kind.join k k'` |
| `Foo bs k` (indexed family) | `Foo` |
| `{auto 0 ok : So (f x)}` | a rule `refuse (f x) .reason` in `Check/*Rules` |
| `Unspellable T (\ok => term)` … `Oh impossible` | `theorem bad… : X.check … term = [.reason] := by decide` |
| a twin `ok… : T = term` | `theorem ok… : X.check … term = [] := by decide` |
| a bench card | `def c : Spelled := spelled <| .singleFaced { characteristics := { name := …, … } }` |
