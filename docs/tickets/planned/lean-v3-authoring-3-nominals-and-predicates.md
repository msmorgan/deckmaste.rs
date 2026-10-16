---
needs: [lean-v3-authoring-2-verb-and-clause-spine]
---
# Introduce composable nominal and predicate modifiers and eliminate Primitives.* clutter in Cards

**Replace ad-hoc compound noun macros (`creatureYouControl`, `outlawYouControl`,
`armyYouControl`, etc.) and raw `Primitives.*` constructor chains in
`lean/Semantics/Macros.lean` and `lean/Semantics/Cards/*.lean` with composable
noun-phrase, determiner, and predicate modifiers (`youControl`, `controlledBy`,
`opponentsControl`, `youOwn`, `ownedBy`, `cardIn`, `withKeyword`, `otherThan`,
`which`, etc.) and clean top-level combinator templates.** Sub-ticket 3 of
[`lean-v3-authoring-surface`](lean-v3-authoring-surface.md). Standard
constraints apply.

## Context & Problem

Currently in `lean/Semantics/Macros.lean` and `lean/Semantics/Cards/*.lean`:
- Possessor and zone qualification is split between a handful of hardcoded
  compound macros (`creatureYouControl`, `outlawYouControl`, `armyYouControl`,
  `nonlandPermanentYouControl`) and verbose raw constructor chains whenever a
  card combines another type or modifier:
  `a (Primitives.Predicate.and [nonToken, creature, Primitives.Predicate.hasPossessor .controller opponent])`
  or `a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan this, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])`.
- Common structural combinators (`Primitives.Ability.static`,
  `Primitives.Ability.spell`, `Primitives.Instruction.sequentially`,
  `Primitives.Instruction.simultaneously`, `Primitives.NounPhrase.you`,
  `Primitives.Predicate.and`, `Primitives.Predicate.or`,
  `Primitives.Predicate.not`, `Primitives.Delta.up`, `Primitives.Delta.down`,
  `Primitives.CountBound.atLeast`, `.asMarker .permanent (relative .object)`)
  lack clean top-level templates or collide with namespaces, resulting in over
  550 `Primitives.*` qualifications across `lean/Semantics/Cards/*.lean`.

## Scope

1. **Design and land composable predicate, determiner, and noun-phrase templates:**
   - Possessor / controller / owner modifiers: `controlledBy`, `youControl`,
     `opponentsControl`, `dontControl`, `ownedBy`, `youOwn`, `opponentsOwn`.
   - Zone-qualified card descriptions: `cardIn`, `inZone`, `inYourHand`,
     `inYourGraveyard`, `inYourLibrary`, `onBattlefield`.
   - Characteristic and identity modifiers: `withKeyword`, `withCounter`,
     `otherThan`, `another`, `non` (`nonToken`, `nonLand`, `nonCreature`, etc.),
     and clean relative-clause `which` helpers.
2. **Expose clean top-level templates for core combinators:**
   - Provide ergonomic top-level `semantic_template` definitions for `static`,
     `spell`, `sequentially`, `simultaneously`, `you`, `this`, `and`, `or`,
     `not`, `up`, `down`, `setTo`, `atLeast`, `atMost`, `exactly`,
     `becomesPermanent`, etc.
3. **Re-author `lean/Semantics/Cards/*.lean`:**
   - Rewrite card definitions across all 14 `lean/Semantics/Cards/*.lean` files
     to use the composable nominal, predicate, and combinator templates so that
     authored cards read like Oracle English and contain zero `Primitives.*`
     qualifications.

## Proof

- `grep -rn "Primitives\." lean/Semantics/Cards/` returns zero matches.
- All 14 `lean/Semantics/Cards/*.lean` modules and `lean/Semantics/Proofs/` pass
  `./lean/scripts/build` (`lake build --wfail`) with zero warnings and zero
  `sorry`s.
