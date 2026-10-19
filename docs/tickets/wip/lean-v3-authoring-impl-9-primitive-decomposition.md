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

### 1. Fold Day/Night (`[CR#731.1]`) and Room Doors (`[CR#709.5c..709.5j]`) into Designations

- **Day and Night (`[CR#731.1,731.1a]`):**
  - Under `[CR#731.1]`, `day` and `night` are designations held by the game
    (`DesignationScope.heldByGame`), *"it becomes day/night"* is the game gaining
    that designation, and *"it is day/night"* tests that designation.
  - Retire `Action.setGameDesignation`, `GameEvent.gameBecomes`, and
    `Condition.gameIs` from `Abilities.lean`, expressing them through
    `gainDesignation`, `GameEvent.does`, and `hasDesignation` (using `day` and
    `night` from `Definitions/Designations.lean`), while keeping surface templates
    in `Templates.lean` (`setGameDesignation`, `gameBecomes`, `gameIs`) expanding
    into the unified designation primitives.
- **Room Doors & Unlocking (`[CR#709.5c,709.5f..709.5j]`):**
  - Under `[CR#709.5c,709.5f..709.5j]`, *"left half unlocked"* and *"right half
    unlocked"* are designations on a permanent (`leftHalfUnlocked` and
    `rightHalfUnlocked` in `Definitions/Designations.lean`), unlocking/locking a
    half is giving/removing the appropriate unlocked designation, and *"fully
    unlocks"* triggers when a permanent gains the remaining unlocked
    designation(s) so both are held.
  - Retire `Action.unlock`, `Action.fullyUnlock`, `CoreDeed.unlock`,
    `CoreDeed.fullyUnlock`, `Door`, `RoomHalf`, and `LockState` from `Words.lean`
    and `Abilities.lean`, re-expressing the surface helpers (`unlock`,
    `fullyUnlock`, `thisDoor`, `doorOf`, `isUnlocked`, `isLocked`) in
    `Templates.lean` / `Definitions/` over `leftHalfUnlocked` and
    `rightHalfUnlocked`.

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
     `Definitions.doubleCounters` via `putCounters` (matching `doubleCountersOfKind`
     and `proliferate`).
3. **`Action.crew` and `Action.saddle` (`[CR#702.122b,702.122d,702.171c]`):**
   - Retire the hardcoded `Action.crew` and `Action.saddle` constructors from
     `Abilities.lean` and express `crew` (`Action`) and `saddle` (`Action`) in
     `Definitions/` via `Action.enact (.ofAbility (.named "crew" / "saddle"))`.
4. **`Action.skipUntap` (`[CR#701.43a,502.3]`):**
   - Retire `Action.skipUntap` from `Abilities.lean` and express
     `doesntUntapDuringNextUntapStep` / `exert` (`[CR#701.43a]`) via `Action.establish`
     of `cant any (untap subject)` during the controller's next untap step (matching
     continuous `doesntUntap`).
5. **`Action.doAndForbid` (`Abilities.lean:999`):**
   - Retire `Action.doAndForbid` from `Abilities.lean` and express `doAndForbid` /
     `destroyCantRegenerate` in `Templates.lean` via `sequentially` +
     `establish (cant any ...) (some untilEndOfTurn)` using `firstMention` /
     `laterMention`.
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
