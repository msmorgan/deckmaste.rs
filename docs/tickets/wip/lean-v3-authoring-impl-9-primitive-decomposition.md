---
needs: [lean-v3-authoring-impl-8-vocabulary-split-and-noun-heads]
---
# Decompose CR-defined and duplicate Core AST primitives in `lean/Semantics/`

**Factor CR-defined composite primitives and overlapping constructors in
`lean/Semantics/Words.lean` and `lean/Semantics/Abilities.lean` into the smaller
orthogonal Core AST basis while keeping surface authoring templates and
definitions intact in `Templates.lean` and `Definitions/`.** Standard
constraints apply.

## Scope

### 1. Fold Day/Night (`[CR#731.1]`) into Designations

- **Day and Night (`[CR#731.1,731.1a]`):**
  - Under `[CR#731.1]`, `day` and `night` are designations held by the game
    (`DesignationScope.heldByGame`), *"it becomes day/night"* is the game gaining
    that designation, and *"it is day/night"* tests that designation.
  - Retire `Action.setGameDesignation`, `GameEvent.gameBecomes`, and
    `Condition.gameIs` from `Abilities.lean`, expressing them through
    `gainDesignation`, `GameEvent.does`, and `hasDesignation` (using `day` and
    `night` from `Definitions/Designations.lean`), while keeping surface templates
    in `Templates.lean` (`setGameDesignation`, `gameBecomes`, `gameIs`) expanding
    into the unified designation primitives. The game is the holder through a game noun phrase
    (`NounPhrase.theGame`), admitted only as the holder of a `heldByGame` designation.
- **Room Doors & Unlocking (`[CR#709.5c,709.5f..709.5j]`):** kept: "this door" is relative to
  the half it is printed on, "a locked door" is a choice of half, and "fully unlocks" needs an
  event no designation primitive expresses, user ruling 2026-10-19.

### 2. Decompose CR-defined and compound `Action` and `GameEvent` primitives

1. **`Action.moveCounters` (`[CR#122.5]`):**
   - Under `[CR#122.5]`, moving a counter removes that counter from the source
     object and puts it onto the second object. Retire `Action.moveCounters` from
     `Abilities.lean` and define `moveCounters` in `Templates.lean` as a compound
     action over `removeCounters` and `putCounters` (using `CounterKindSource.sameAs`).
2. **`Action.doubleCounters` (`[CR#701.10e]`):**
   - Under `[CR#701.10e]`, doubling the number of a kind of counters on a player or
     permanent gives that player or permanent as many of those counters as it
     already has. Retire `Action.doubleCounters` from `Abilities.lean` and express
     `Definitions.doubleCounters` via `putCounters` under `doForEachKind .counterKind`, its
     amount a bound-kind counter projection (`ProjAxis.boundCounter`).
3. **`Action.crew` and `Action.saddle` (`[CR#702.122b,702.122d,702.171c]`):** kept: the
   constructors carry the object-actor check and the deontic head gate `enact` lacks, user ruling
   2026-10-19.
4. **`Action.skipUntap` (`[CR#701.43a,502.3]`):** kept: a duration cannot count "next two untap
   steps" (Telekinesis), user ruling 2026-10-19.
5. **`Action.doAndForbid` (`Abilities.lean:999`):**
   - Retire `Action.doAndForbid` from `Abilities.lean` and express `doAndForbid` in
     `Templates.lean` over the forbidden `Action` via `sequentially` +
     `establish (cant any ...) (some untilEndOfTurn)`, with `destroyCantRegenerate` in
     `Definitions/Actions.lean` as `doAndForbid (destroy X) (regeneration it)`.
6. **`Action.drawGame` (`[CR#104.4]`):**
   - Add `.drawGame` to `OutcomeVerb` (`Words.lean:525`) and retire the separate
     `Action.drawGame` constructor in favor of `Action.conclude .drawGame none`.
7. **`Action.triggerReflexively` and `Action.triggerThisWay` (`[CR#603.12]`):**
   - Unify `Action.triggerReflexively` and `Action.triggerThisWay` (`Abilities.lean:1089-1090`)
     into a single `Action.triggerReflexively (body : Action) (event : Option GameEvent) (trigger : Action)`
     constructor (where `event = none` is *"when you do"* and `event = some ev` is
     *"when [ev] this way"*), keeping both `Templates.triggerReflexively` and
     `Templates.triggerThisWay` helpers.
8. **`GameEvent.triggers` (`[CR#603.2]`):**
   - Retire `GameEvent.triggers (ability : NounPhrase) (of_ : Option NounPhrase) (by_ : Option NounPhrase)`
     (`Abilities.lean:463`) and express `Templates.triggers` via `GameEvent.does` +
     `Action.trigger` (already in `Action.eventHeadOk`), `Predicate.abilityOf` for
     `of_`, and `GameEvent.causes` for `by_`.

### 3. Unify `StaticSpec` continuous modifications into `CharacteristicEdit`

- In `lean/Semantics/Abilities.lean`, `CharacteristicEdit` (inside
  `StaticSpec.characteristicChange`) already carries `.stat`, `.addedAbilities`, and
  `.removedAbilities`, while `StaticSpec` also carries separate `.modification`,
  `.ptModification`, `.abilityGrant`, `.abilityGrantFrom`, and
  `.abilityGrantSelected` constructors.
- Unify `StaticSpec.modification`, `StaticSpec.ptModification`,
  `StaticSpec.abilityGrant`, `StaticSpec.abilityGrantFrom`, and
  `StaticSpec.abilityGrantSelected` into `CharacteristicEdit` inside
  `StaticSpec.characteristicChange`, keeping all `Templates.lean` surface helpers
  (`modification`, `getsPt`, `hasBasePt`, `abilityGrant`, `abilityGrantFrom`,
  `abilityGrantSelected`, `has`, `is`, `gains`, `loses`) unchanged so card and
  keyword authoring syntax is preserved.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) and `cargo xtask lean check` pass
  with zero warnings and zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check`
  0 stale.
