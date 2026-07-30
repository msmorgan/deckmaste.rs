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

## Completion notes

- Selected ordinary noun-phrase coordinations are normalized during lowering
  when a leading determiner is morphologically compatible with a later bare
  singular. The rewrite also reaches a coordination stranded outside the last
  PP object, preserves repeated-determiner groups, and extends Oxford lists.
  Invariant catalog plurals are checked through noun-declension metadata;
  `Equipment` and `Spacecraft` now carry their attested invariant declension.
- `CoordinatedNominalPhrase` now owns group-level complements, so a trailing
  relative scopes over the completed selection (`another target creature or
  artifact you control`) while parallel member relatives remain local.
- `this creature or equipped/enchanted creature` retains two complete noun
  phrases through a typed participial-verb exception. Other participles remain
  productive; no card text or surface suffix is inspected.
- Bracket provenance records the lowering edit itself, removes spans that would
  cross the recovered group, and emits no constituent for an unfinished Oxford
  prefix. The supported-corpus laminar-span gate passes across 55,983 ability
  records.
- The generalized grammar-feature/cost experiment was rejected after corpus
  comparison exposed unrelated parse selection changes. Mixed option-list
  versus common-head preferences remain with
  `english-coordination-structural-design` rather than being guessed here.
- The refined `bare-singular-conjunct` check reports 0 findings (from 984),
  excluding only structurally closed first nominals, fused/quantified heads,
  opacity, and typed Equip/Enchant attachment roles.
- All 31,685 supported faces round-trip with 0 mismatches and 0 render errors;
  nominal recovery remains 0. Workspace tests, workspace clippy (with existing
  warnings), and Wizards plugin generation pass.
