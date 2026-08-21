---
needs: [workbench-pins-refuse-rules-impossibility-only]
---
# The kind join is a constructor

Supersedes the shape landed by `workbench-join-shape-flat-or-pair` (ruling
2026-08-22). `Kind` gains the constructor `(\/) : Kind -> Kind -> Kind`
itself. `ObjectOrPlayer`, `joinable`, the `So (joinable a b)` gate, the
`joinableRefl/Sym` lemmas and the join pins (`badJoin*`) are deleted: a join
of any two kinds is writable, because the rules make "a spell or ability",
"a creature or player", "a permanent or player" all meaningful and the
semantics accepts anything rules-meaningful
(`docs/memory/rulings/measurements-live-in-pins.md`).

## The change

- `data Kind = … | (\/) Kind Kind`, `infixl 5 \/` kept.
- `Payload` gains `JoinP : Payload a -> Payload b -> Payload (a \/ b)`. A
  union mention carries what it knows about each half — "target creature or
  player" is `JoinP (ObjectP (Just Creature) …) PlayerP` — which is the pair
  the demonstrative echo reads back (33 of 33). `UnionP` is deleted and its
  readers (`bindingZone`/`bindingTy`, `bindFor`'s union branch) read `JoinP`.
- The join is a free term, so the laws move from the function to a **lattice
  order**, pinned (2026-08-22) as

  ```idris
  kindLte : Kind -> Kind -> Bool
  kindLte (a \/ b) y = kindLte a y && kindLte b y
  kindLte x (a \/ b) = kindLte x a || kindLte x b
  kindLte x y = x == y
  ```

  (left-join clause first; `&&`/`||` laziness is harmless at concrete kinds,
  which is all a `So` gate reduces). An inductive `KindLte` with
  `Same/JoinL/InL/InR` is the documented upgrade if a reader ever needs the
  witness (which half an anaphor resolved to); not minted now. Laws:
  `a ≤ a \/ b`, `a \/ b ≤ b \/ a`, `(a \/ b) \/ c ≤ a \/ (b \/ c)` and their
  converses, proved by case. Every reader that asks "is this binding of kind
  k" — the antecedent counting under `It`/`They`/`That` (`countOnes` and
  kin), `lookbackSubjectOk`, `Targetable` — asks `kindLte` instead, which also
  answers whether an `Object` anaphor may resolve to an `Object \/ Player`
  antecedent (it may). `Eq Kind` stays structural.
- Every exhaustive `Kind` match gains one `\/` clause, generically (recurse
  or combine), instead of one `ObjectOrPlayer` clause per attested pair.
- The `(\/)` docstring stays the decision record, rewritten: flat-vs-pair was a
  level confusion; the pair is `JoinP`; the join is syntax.
- `AnyTarget`, `KindJoin`, `YouAnd` and the `That` anaphor stay as they are;
  `workbench-union-family-macros` retires them over this shape.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Events.idr`, `idris/src/Experimental/Proofs*.idr`,
`idris/src/Experimental/Cards.idr`. No Rust crate is touched.

## Acceptance

- No `ObjectOrPlayer`, `joinable`, `UnionP` or `badJoin*` remains; `kindLte`
  and its laws exist; `JoinP` carries the pair and a witness reads it back.
- `idris/scripts/build` PASS; `Cards.idr` binds no implicits; cites 0/0.

Standard constraints apply.
