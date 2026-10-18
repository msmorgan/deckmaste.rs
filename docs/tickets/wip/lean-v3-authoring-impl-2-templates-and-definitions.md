---
needs: [lean-v3-authoring-impl-1-core-ast-and-checker]
---
# Consolidate Primitives, Templates, and Definitions with the v3 authoring vocabulary

**Retire `lean/Semantics/Macros.lean` and `lean/Semantics/Macros/Declared.lean`,
consolidating into `lean/Semantics/Primitives.lean`,
`lean/Semantics/Templates.lean`, and `lean/Semantics/Definitions.lean` with the
complete `v3` authoring vocabulary and `Definition` bodies expressed through
`Templates.lean` helpers.** Implementation ticket 2 of
[`lean-v3-authoring-surface`](lean-v3-authoring-surface.md). Standard
constraints apply.

## Scope

1. **Module Consolidation (`lean-v3-authoring-surface.md` §6.1):**
   - Replace `lean/Semantics/Macros.lean`, `lean/Semantics/Macros/Primitives.lean`,
     `lean/Semantics/Macros/Declared.lean`,
     `lean/Semantics/Macros/CharacteristicInputs.lean`, and
     `lean/Semantics/Macros/Checks.lean` with:
     - `lean/Semantics/Primitives.lean` (`Semantics.Primitives` namespace for
       `declare_semantic_primitives` + `CharacteristicInputs`)
     - `lean/Semantics/Templates.lean` (authoritative `semantic_template`
       phrasing helpers)
     - `lean/Semantics/Definitions.lean` (authoritative `semantic_definition`
       rules-defined vocabulary: `[CR#701.1]` keyword actions, `[CR#702.1]`
       keyword abilities, and counters/tokens/designations)
     - `lean/Semantics/Proofs/DefinitionsAndTemplates.lean` (replacing
       `Macros/Checks.lean`)
2. **Land the `v3` Template Vocabulary in `Templates.lean` (`lean-v3-authoring-surface.md` §§2–5):**
   - **Deontic templates (§2.2):** `cant`, `can`, `must`, and `gatedBy` taking
     `(subj : Subject) (action : Action)` plus optional `(timing : Option Timing := none)`,
     `(asThough : Option AsThough := none)`, and
     `(except : Option DeonticExcept := none)`; delete `deonticActionFor` and all
     legacy `*Cant`, `*Be`, and `mayPlayDeed` wrappers.
   - **Verb and clause spine (§4):**
     - Retire all ~35 3p-singular `GameEvent` wrappers (`dies`, `enters`,
       `leaves`, `attacks`, `blocks`, `becomesBlocked`, `becomesTarget`, `casts`,
       `activates`, `discards`, `draws`, `losesGame`, etc.) in favor of
       `does <subj> <action>`.
     - Intransitive (`.actor`) moves (`die`, `enter`, `leave`) and status
       changes (`phaseOut`, `phaseIn` `[CR#702.26b]`, `become` `[CR#603.2e]`).
     - Transitive (`subject`) moves (`putInto`, `exile`, `destroy`, `sacrifice`,
       `discard`) and status changes (`tap`, `untap` `[CR#701.26a..701.26b]`,
       `turnFaceUp`, `turnFaceDown`).
     - Optional-kwarg `dealDamage`, `winGame`, `loseGame`, `flipCoins`, `rollDice`,
       `cast`, `play`, and `activate`.
   - **Uniform boolean-like `Predicate` templates (§5.2):**
     - `isCreature`, `isArtifact`, `isEnchantment`, `isLand`, `isInstant`,
       `isSorcery`, `isPlaneswalker`, `isBattle`, `isKindred`,
       `isInstantOrSorcery`, `isPermanentType`, `isPermanent`, `isPermanentCard`,
       `isSpell`, `isCard`, `isToken`, `isNontoken`, `isCopyOfACard`, `isSource`,
       `isEmblem`, `isManaAbility`, `isPlayer`, `isOpponentOf`, `isOpponent`,
       `isTeammateOf`, `isTeammate`, `isChosenPlayer`, `isOtherPlayer`,
       `isOutlaw`, `isHistoric`, `isAnyTarget`, `isAnyOtherTarget`.
     - `isTapped`, `isUntapped`, `isFaceDown`, `isFaceUp`, `isPhasedOut`,
       `isTransformed`, `isAttached`, `isAttacking`, `isBlocking`, `isBlocked`,
       `isUnblocked`, `isColorless`, `isMonocolored`, `isMulticolored`, `isOther`.
     - `controlledBy`, `youControl`, `youDontControl`, `opponentsControl`,
       `ownedBy`, `youOwn`, `opponentsOwn`, `in_`, `otherThan`, `non`,
       `hasKeyword`, `hasCounter`, `named`, `hasType`, `hasSubtype`,
       `hasSupertype`, `hasColor`, `hasPower`, `hasToughness`, `hasManaValue`.
     - Retire `creatureYouControl`, `creatureYouDontControl`,
       `creatureYourOpponentsControl`, `outlawYouControl`, `armyYouControl`,
       `otherCreature`, and `otherCreatureYouControl`.
   - **Nouns, statuses, hosts, `Location` helpers, and top-level combinators (§§3, 5.2–5.4):**
     - Bare `NounWord` / `CardType` (`creature`, `artifact`, `enchantment`,
       `land`, `planeswalker`, `battle`, `card`, `permanent`, `player`, `token`,
       `copy`), `Status` (`tapped`, `untapped`, `faceDown`, `faceUp`, `phasedIn`,
       `phasedOut`), `spell` (`Ability.spell`), `static` (`Ability.static`).
     - Attached hosts (`enchanted`, `equipped`, `fortified`) and quantified
       `targets`.
     - `Location` helpers (`topOf`, `bottomOf`, `fromTop`, `fromBottom`,
       `exiledWith`, `libraryOf`, `handOf`, `graveyardOf`, `yourLibrary`,
       `yourHand`, `yourGraveyard`).
     - Top-level combinators (`you`, `this`, `yourOpponents`, `and`, `or`, `not`,
       `sequentially`, `simultaneously`, `compound`, `conjunction`, `up`, `down`,
       `setTo`, `letter`, `thatMuch`, `anyAmount`, `mana`, `tapSymbol`,
       `untapSymbol`, `loyaltySymbol`, `perform`, `itsManaCost`, `establish`,
       `abilityGrant`, `qualityChange`, `ptDefinition`, `letterDefinition`,
       `instead`, `damageRule`, `costShift`, `altCost`, `addedCost`, `visibility`,
       `controlGrant`, `copyChange`, `matches`, `compareAmt`, `forAsLongAs`).
3. **Re-express `Definition` Bodies, Fix `menace`, & Update `Proofs/` and `lean/CONTRACTS.md` (`lean-v3-authoring-surface.md` §§2.4, 6.2–6.3):**
   - Change `DeonticExcept.byObjects (predicate : Predicate)` to
     `DeonticExcept.byObjects (objects : NounPhrase)` so `flying` (`[CR#702.9b]`),
     `fear` (`[CR#702.36b]`), `intimidate` (`[CR#702.13b]`), and `menace`
     (`[CR#702.111b]`: *"can't be blocked except by two or more creatures"*,
     `counted (atLeast 2) isCreature`, fixing the legacy `moreThan 1` error) all
     use `cant any (block thisCreature) (except := byObjects ...)`; retire the
     now-unused `CountBound` wrapper on `UsageLimit.mk (times : Amount) (per : Period)`
     (`[CR#602.5b]`).
   - Re-express all `Definition` bodies in `lean/Semantics/Definitions.lean`
     through `Templates.lean` helpers, reconciling the 15 signature drifts and 2
     body drifts (`amass` `[CR#701.47a]`, `bushido` `[CR#702.45a]`, `modular`
     `[CR#702.43a]`, `renown` `[CR#702.112a]`, `forEach`, `fateseal`
     `[CR#701.29a]`, `companion` `[CR#702.139a]`, `levelUp` `[CR#702.87a]`,
     `gainControl`, `creatureToken[Of]`, `create` `[CR#701.7]`,
     `lookAndSortSlice`).
   - Update `lean/Semantics/Proofs/*.lean` and `lean/CONTRACTS.md` to reflect
     `Primitives.lean`, `Templates.lean`, and `Definitions.lean` and retire
     remaining `v2` holdover references.

## Proof

- `lean/Semantics/Macros.lean` and `lean/Semantics/Macros/` no longer exist.
- `grep -rn "deonticActionFor\|CountBound" lean/Semantics/` returns zero matches.
- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
