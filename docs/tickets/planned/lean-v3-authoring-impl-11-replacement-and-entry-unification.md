---
needs: [lean-v3-authoring-impl-10-definitions-and-cards-polish]
---
# Unify replacement and entry-modification phrasings (`replace`/`instead`, `entryRider`, `choice`, `asEvent`)

**Reshape static and self-replacement effects onto `replace (would ...) (instead ...)`,
retire `StaticSpec.entryRider` and `StaticSpec.choice` in favor of unified
`replace` / `asEvent` entry-modification phrasings (`[CR#614.1a,614.1c,614.1d,614.12,614.15]`),
and fix singular draw-replacement amounts (`[CR#121.2,702.52a]`).** Eleventh
implementation step of [`lean-v3-authoring-surface`](lean-v3-authoring-surface.md).
Standard constraints apply.

## Scope

### 1. `StaticSpec.instead` → `StaticSpec.replace` & `instead` at the Replacement Clause (`[CR#614.1a,614.15]`)

- Rename `StaticSpec.instead (event : WouldClause) (replacement : Action) ...` in
  `lean/Semantics/Abilities.lean` to **`StaticSpec.replace`**, dispatched alongside
  `Action.replace` (`[CR#614.15]`) via `expectedTypeDispatch` in
  `lean/Semantics/Templates.lean`.
- Define `semantic_template instead (action : Action) : Action := action` in
  `lean/Semantics/Templates.lean` so `"instead"` sits at the replacement action
  slot — including inside `may (instead ...)` (`dredge` `[CR#702.52a]`) and
  `doIf cond (instead ...)` in self-replacements (`tezzeretArtificeMaster`,
  `galvanicBlast`):
  - *"If you would draw a card, draw two cards instead"*:
    `replace (would you (draw (.lit 1))) (instead (draw (.lit 2)))`
  - *"If you would draw a card, you may instead mill N cards..."* (`dredge`):
    `replace (would you (draw (.lit 1))) (may (instead ...))`
  - *"Draw a card. If you control three or more artifacts, draw two cards instead"*:
    `replace (draw (.lit 1)) (doIf ... (instead (draw (.lit 2))))`
- Retire the duplicate `Templates.replacement` helper (`lean/Semantics/Templates.lean:1119`,
  Opus findings §5.4).
- Fix singular *"If you would draw a card"* replacements (`thoughtReflection` in
  `lean/Semantics/Cards/Static.lean:26`, `wordsOfWar` in `lean/Semantics/Cards/Damage.lean:114`,
  `wordsOfWorship` in `lean/Semantics/Cards/Damage.lean:1346`, and `dredge` in
  `lean/Semantics/Definitions/Keywords.lean:368`) from `draw anyAmount` to
  `draw (.lit 1)` (`[CR#121.2,702.52a]`).

### 2. Unify `[CR#614.1c,614.1d,614.12]` Entry-Modification Encodings

- Currently `lean/Semantics/` has four competing encodings for entry replacement
  and `"As ~ enters"` abilities:
  1. `StaticSpec.entryRider (subject : NounPhrase) (rider : MoveRider)` (`Abilities.lean:823`)
  2. `StaticSpec.choice (occasion : ChoiceOccasion) ...` (`Abilities.lean:824-825`)
  3. `StaticSpec.asEvent (event : GameEvent) (does : Action)` (`Abilities.lean:838`)
  4. `StaticSpec.instead (would any (putOntoBattlefield this_)) ...` (`Cards/Mana.lean:890,903,923`, `Cards/Keyword.lean:1692`)
- **Unification:**
  - Retire `StaticSpec.entryRider`: re-express `entersTapped`,
    `entersWithCounters`, `entersWithAdditionalCounters`, `entersWithFewerCounters`,
    `asCopyOf`, and `daybound`'s `.entersTransformed` through `replace` / `asEvent`
    over `enter (riders := [rider])` (`[CR#614.1c,614.1d,614.12]`).
  - Retire `StaticSpec.choice` and `ChoiceOccasion`: re-express `entersChoosing`
    and attachment-time choices through `asEvent` over `choose`.
  - Fix `devour` (`[CR#702.82a]`), `tribute` (`[CR#702.104a]`), `riot`
    (`[CR#702.136a]`), `unleash` (`[CR#702.98a]`), and `amplify` (`[CR#702.38a]`)
    in `lean/Semantics/Definitions/Keywords.lean` so they modify the entering
    event directly rather than dynamically calling `.establish (entryRider ...) none`
    inside `asEvent (does thisPermanent enter)`.
  - Align *Heart of Yavimaya*, *Mox Diamond*, *Sheltered Valley*, and *Deadpool,
    Trading Card* onto the unified entry-replacement phrasing.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
