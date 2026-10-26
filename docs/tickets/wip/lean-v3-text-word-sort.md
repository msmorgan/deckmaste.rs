---
needs: []
---
# `replaceWord` takes its own `TextWordSort`, not `QualitySort`

Standard constraints apply. Decided by the owner in the 2026-10-25 triage follow-up (Q9); the
change was prototyped on a scratch copy and `lake build --wfail` was green. Paths are under
`lean/Semantics/` unless they start with `lean/` or `docs/`; find each site by name.

## The defect

`TextChange.replaceWord` (`Abilities.lean:641`) takes `List QualitySort`, so the type admits sorts
no Text Change replaces ("a card name", a type, a supertype) and a checker row
(`Refusal.textChangeSort`) refuses them afterwards. A Text Change replaces a color word, a land
type or a creature type, each used in its rules sense [CR#612.2,612.2a]; make the type say so. The
glossary's Text Change entry (`docs/contexts/game-model/CONTEXT.md`) already uses these terms, so
it needs no edit.

## The change

1. `Abilities.lean`, after `TextWord` (`:32-35`), add:
   ```lean
   /-- The kinds of word a Text Change replaces: a color word, a land type or a creature type, each
   used in its rules sense [CR#612.2,612.2a]. -/
   inductive TextWordSort where
     | color | landType | creatureType
     deriving DecidableEq, Repr
   ```
2. `Abilities.lean:641`: `| replaceWord (sorts : List QualitySort) (from_ : Option TextWord) (to :
   Option TextWord)` becomes `List TextWordSort`. Its docstring ("the kind one of `sorts`") stays.
3. `Templates.lean:1665` (`semantic_template replaceWord`): `(sorts : List QualitySort)` becomes
   `(sorts : List TextWordSort)`.
4. `Check/TextChanges.lean:11-16`: delete `QualitySort.replaceable` and put in its place
   ```lean
   /-- The quality sort a word of a Text Change's sort is of. -/
   def TextWordSort.quality : TextWordSort → QualitySort
     | .color => .color
     | .landType => .subtype .land
     | .creatureType => .subtype .creature
   ```
5. `Check/TextChanges.lean:28-34`, `TextChange.shape`: map the sorts first and drop the
   replaceable filter:
   ```lean
   def TextChange.shape : TextChange → List Refusal
     | .replaceWord sorts from_ to =>
       let qualities := sorts.map TextWordSort.quality
       (if sorts.isEmpty then [.nonEmpty] else []) ++
         TextWord.sortCheck qualities from_ ++ TextWord.sortCheck qualities to ++
         TextWord.sameSort from_ to
     | _ => []
   ```
6. `Check/Refusal.lean:134-136`: delete `Refusal.textChangeSort` with its docstring. Keep
   `textWordSort (sorts : List QualitySort) (found : QualitySort)` as it is.
7. `docs/decisions/semantics-v3.md:649` (the "Text-changing effects" bullet): say that
   `replaceWord` takes `TextWordSort`s (`color`, `landType`, `creatureType`) [CR#612.2,612.2a], so
   a card name or any other sort is unrepresentable.

No other use exists (`Cards/` and `Definitions/` have none; confirm with `grep -rn 'replaceWord'
lean/Semantics`).

## Pins (`Proofs/TextChanges.lean`)

Respelled, same claim:

| Pin | Respelling |
|---|---|
| `okReplaceLandTypeWord` (`:87`) | `replaceWord [.subtype .land]` becomes `replaceWord [.landType]` |
| `okReplaceCreatureTypeWord` (`:96`) | `[.subtype .creature]` becomes `[.creatureType]` |
| `okReplaceCreatureTypeWithVampire` (`:104`) | `[.subtype .creature]` becomes `[.creatureType]` |
| `okReplaceColorOrLandTypeWord` (`:126`) | `[.color, .subtype .land]` becomes `[.color, .landType]` |

Respelled, same expected refusal (the refusal rows are in `QualitySort` and do not change):

| Pin | Respelling | Still expects |
|---|---|---|
| `badReplaceCreatureTypeWithLandType` (`:111`) | `replaceWord [.subtype .creature] none (some (.subtype swamp))` becomes `replaceWord [.creatureType] none (some (.subtype swamp))` | `[.textWordSort [.subtype .creature] (.subtype .land)]` |
| `badReplaceWordSort` (`:118`) | `replaceWord [.color] none (some (.subtype swamp))` is unchanged (`.color` is a `TextWordSort` constructor too); check it elaborates | its current refusal |
| `badReplaceAcrossKinds` (`:130`) | `.replaceWord [.color, .subtype .land] …` becomes `.replaceWord [.color, .landType] …` | `[.kindMismatch (.quality .color) (.quality (.subtype .land))]` |

Retired: `badReplaceWordCardName` (`:134-137`, `.replaceWord [.cardName] none none` expecting
`[.textChangeSort .cardName]`). A card name is now unrepresentable as a `TextWordSort`, and no
card prints a Text Change over names, so there is nothing left to refuse. Delete it with its
docstring.

## Must stay green

`okCleaveDefinition` and every other pin in `Proofs/TextChanges.lean`; the Text Change checker
arm `Check/Phrase.lean:2003` (`| .replaceWord .. => (drs, tc.shape)`) is unchanged.

## Acceptance

- `./lean/scripts/build` (`--wfail`); `grep -rn 'textChangeSort\|replaceable' lean/Semantics
  lean/CONTRACTS.md docs/decisions/semantics-v3.md` is empty; `grep -rn 'replaceWord \[.subtype'
  lean/Semantics` is empty.
- `cargo xtask lean check`.
- The cite gates: `cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite check` 0
  stale, `jj --no-pager diff --git | cargo xtask cite audit --diff` read rule by rule.
