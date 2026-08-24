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

## As-landed

Bare coexistence collides — Idris 2 puts both data types' constructors in
the enclosing namespace, so a second `Define` is "already defined". The
arrangement is the telescopes' own: `data StaticEffect` (the smaller of the
two declarations) moved into `namespace Static` inside the same mutual
block, exactly as `SimEffects`/`StaticParts`/`CostSeq`/`AbilitySeq` sit in
`Sim`/`Coord`/`Paid`/`Text`. Nothing else moved: `StaticEffect` and its
other constructors still resolve unqualified everywhere, `Cards.idr` writes
bare `Define` in both positions, and the clause constructor keeps its
module-level name. Prose now says "the clause-level `Define`" for one and
`Static.Define` for the other; `StaticKind.LetterDefinition` is unchanged.

Touched: `Experimental.idr` (namespace wrap, constructor, `staticKind`,
`staticIntro`, both docstrings), `Cards.idr` (10 sites), `ProofsD.idr`
(2 sites). `Macros.idr`, `Events.idr` and `ProofsAnaphora.idr` never named
`DefinesLetter`.

The `Rider`-family unification stays deferred per the 2026-08-24 decision:
it waits on a second trailing-rider construction existing.
