---
needs: [idris-tables-fixtures-v2]
---
**Macro-system v2: typed holes for every kind, binder contracts,
definition-time body checking, staged capture, one generic splice, and
bidirectional template codecs.** Six deltas to macro_ron; macros stay
declarative (no control flow, recursion is a load error) and never paper over
linkage, simultaneity, or rules-level quantification — those are core.

Related: [[macro-keyword-templates]], [[parse-params-via-macros]],
[[macro-bare-defaulted-invocations]].

## The six deltas

1. **Typed params for ALL kinds.** Every grammar kind a macro can take gets a
   registered param type + validator (roughly 19 of 26 kinds are missing
   validators today); `params: [Any]` slots are retyped. Declared types drive
   validation, codecs, and hole checking below.
2. **Binder-context annotations (the binder contract).** A param can declare
   the anaphora its body wraps around the hole: `Effect(binds: [It])` = "the
   body introduces an It-binder over this hole". DEFAULT = no anaphora beyond
   the call site: an argument that reads an anaphor the contract doesn't
   grant is an `E-MACRO-*` load error at the call site. The elaborator
   validates arguments against the contract — splice hygiene becomes checked,
   not conventional.
3. **Definition-time body checking.** After ALL plugins load (so
   cross-plugin macro references resolve — the load-order trap), expand each
   macro definition once with typed `Hole` placeholders and elaborate the
   result. An ill-formed body is a definition error at load, not a latent
   error awaiting the first caller.
4. **Caller-frame capture + `Quote`.** Params splice eagerly in the caller's
   frame; `Quote(Param(i))` is the stage marker that defers a param through a
   two-level meta-macro (a macro whose body invokes another macro with the
   outer macro's param). Unlocks the alternative-cost base macro and the
   parameterized keyword stubs (~190) that currently can't forward params.
5. **One generic list-splice.** `Splice(Param(i))` legal at any `Vec`
   position, replacing the two ad-hoc flatten mechanisms (the Cost-component
   flatten and the modification-bundle `Several` flatten) with one rule.
6. **Template codecs per param type, both directions.** Each param type's
   codec parses AND renders its slot; slots gain sign-aware (`${0:+}` renders
   "+1"/"-1") and plural-aware (`${n:card|cards}`) forms. Templates are part
   of the typed signature — the same declaration drives authoring-expansion,
   rendering, and parsing.

## Done

- All six deltas landed in `macro_ron`/`macro_ron_derive`/`deckmaste_plugin`
  with `E-MACRO-*` codes; each has a reject fixture (untyped param, contract
  violation, ill-formed definition body, unquoted meta-param, splice at a
  non-Vec position, codec round-trip failure).
- The two ad-hoc flatten paths are deleted in favor of `Splice`.
- Every existing macro definition passes definition-time checking unmodified
  or with mechanical fixes.

## Verification

- `cargo test -p macro_ron -p macro_ron_derive -p deckmaste_plugin` and
  `cargo test --workspace` green.
- `cargo xtask validate` clean; wizards regenerated and re-validated;
  `cargo xtask fidelity` still green on canon.
- `cargo xtask elaborate --lock` unchanged (pure macro-system work must not
  re-point any resolution) — a non-empty lock diff is a red flag, not a
  bless.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
