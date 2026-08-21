---
needs: []
---
# Reflect the workbench's Bool gates through `Data.So`

The workbench's gates are proof-by-reflection over `Bool` predicates, but the
reflection is hand-rolled, in two forms:

- bare: `{auto 0 ok : chosenQualityReadOk q = True}`;
- wrapped: `data ZoneFits : Maybe Zone -> Maybe Zone -> Type where
  MkZoneFits : {auto 0 ok : zoneFits subj desc = True} -> ZoneFits subj desc`,
  a one-constructor type whose only job is to name `zoneFits … = True` so call
  sites can write `{auto 0 zn : ZoneFits …}`.

Both are `So` written out longhand. `Data.So` is base's reflection type — `So b`
is inhabited by `Oh` exactly when `b` reduces to `True` — with the splitting and
`choose` helpers the wrappers re-derive. Semantics, erasure, and elaboration
behaviour are identical: one constructor, no search, instant success or failure.

## The change

In `idris/src/Experimental.idr` and `idris/src/Experimental/Words.idr`:

- Every bare gate `{auto 0 x : f … = True}` becomes `{auto 0 x : So (f …)}`.
- Every wrapper `data G : … -> Type where MkG : {auto 0 ok : g … = True} -> G …`
  becomes the synonym `G : … -> Type; G a b = So (g a b)`, keeping the name
  every signature already uses and deleting the `Mk…` constructor. A wrapper
  that carries anything besides the proof, or that is pattern-matched on, stays
  a `data` and is listed in the report.
- Hand-rolled `&&`/`||` splitting lemmas over `= True` are replaced by
  `Data.So`'s, or converted to `So` where no library lemma fits.
- Keep `= True` only where a proof genuinely `rewrite`s with the equation;
  convert at the boundary with `soToEq`/`eqToSo` and list each kept site.

This is mechanical. Sequence it **under** any open design ticket that adds
gates to these files — `workbench-join-shape-flat-or-pair` is the first such —
so the new gates are written on `So` from the start rather than migrated.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`, and the
evidence bench `idris/src/Experimental/Cards.idr` only where a `Mk…`
constructor was named explicitly. No Rust crate is touched.

## Acceptance

- No `= True}` gate remains in the two files except the listed `rewrite`
  sites; no one-constructor proof wrapper remains.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing: every
  refusal that failed to elaborate before fails to elaborate after.

Standard constraints apply.
