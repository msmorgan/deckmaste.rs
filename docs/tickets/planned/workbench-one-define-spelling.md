---
needs: []
---
# One `Define` spelling across both clause categories

`Effect.Define` and `StaticEffect.DefinesLetter` are the same construction
(", where X is …") in two clause categories; the shared content is the
`defineLetter` re-marking and the `anyOpenLetter` gate, and the rest of their
rows are per-category. Give the bench ONE spelling by the house
constructor-overloading idiom (the five telescopes already overload
`Nil`/`(::)` by expected type): rename `DefinesLetter` → `Define`,
namespacing as needed so the two constructors coexist, exactly as the
telescope namespaces do. Every slot's expected type is `Effect` or
`StaticEffect`, never both, so resolution is deterministic.

No structural change: constructors, gates, rows and proofs keep their
shapes; only the name and its ~22 touch points (2 static rows, bench sites,
docstrings, `ProofsAnaphora` mentions) move. Decision record 2026-08-24:
the `Rider`-family unification is deferred until a second trailing-rider
construction exists.

## Consumption boundary

`idris/src/Experimental.idr`, `Events.idr`, `Cards.idr`, `Proofs*.idr`,
`Macros.idr` if it names `DefinesLetter`. No Rust crate.

## Acceptance

- `grep -rn 'DefinesLetter' idris/src` → empty; the bench writes `Define`
  in both positions; `idris/scripts/build` PASS, no witness lost, no pin
  silently passing.

Standard constraints apply.
