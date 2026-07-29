---
needs: [english-lint-expressiveness]
---
**Stranded determiners in coordinations of complete noun phrases — 984
supported faces.** `english-ast-grouping` added `CoordinatedNominalPhrase` so
nominal material sharing one determiner has a home; these are the coordinations
that still route through `CoordinatedNounPhrase` with the determiner stranded
on the first conjunct.

Measured 2026-07-29 by `cargo xtask english lint --check bare-singular-conjunct`
(one finding per coordination, not per member): 984 faces where a determined
conjunct sits beside a determinerless singular sibling.

- **Accursed Duneyard** — `<<<target> <Shade>>, <Skeleton>>, … or <Zombie>>`.
  `Regenerate target Shade, Skeleton, … or Zombie` is ONE target of any of
  those types; `target` currently scopes over `Shade` alone.
- **Abuelo, Ancestral Echo** — `<another target creature> or <artifact you
  control>`; one selection, not a determined phrase coordinated with a bare noun.
- **Aang, Swift Savior** — `<up to one other target creature> or <spell>`.

The fix is NOT to distribute the determiner onto each member — that yields two
targets where the card has one. These belong under a shared determiner, which
is what `CoordinatedNominalPhrase` already models; the work is routing them
there.

Regression gate: `bare-singular-conjunct` to 0. Standard constraints apply.
