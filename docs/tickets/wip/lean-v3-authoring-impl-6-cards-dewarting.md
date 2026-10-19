---
needs: [lean-v3-authoring-impl-5-definitions-dewarting]
---
# Dewart `lean/Semantics/Cards/*.lean` onto the cleaned `Definitions.lean` and `Templates.lean` surface

**Fold the remaining raw `Semantics.*` constructor escape hatches, `setStatus`
calls, parameterized `keyword*` wrappers, raw `.named "..."` counter/designation
literals, and `hasKeyword (.the (.named "..."))` boilerplate across all 15
`lean/Semantics/Cards/*.lean` modules.** Standard constraints apply.

## Scope

### 1. Eliminate the last `Semantics.*` constructor escape hatch (*Mirelurk Queen*)

- In `lean/Semantics/Cards/Counters.lean:914-925`, rewrite `mirelurkQueenTrigger`
  (*"Whenever one or more nonland cards are milled, ... Do this only once each
  turn"*, `[CR#701.17c]`) onto `millCards`:
  ```lean
  triggeredOnlyOnce
    (does any (millCards (counted (atLeast 1) (and [non isLand, isCard, in_ library]))))
    oncePerTurn
    ...
  ```
- Verify that zero `Semantics.Action.*`, `Semantics.NounPhrase.*`, or `Primitives.*`
  qualifications remain in `lean/Semantics/Cards/`.

### 2. Replace remaining `setStatus` calls with `phaseOut`, `phaseIn`, and `flip`

- **Phasing (`[CR#702.26]`):**
  - Replace the 6 remaining `setStatus .phasedOut` / `setStatus .phasedIn` calls in
    `Cards/Trigger.lean:56,60,70` (*Teferi's Imp*, *Shimmering Efreet*),
    `Cards/Cost.lean:92` (*Vodalian Illusionist*), and
    `Cards/Choice.lean:1464,1466` (*Oubliette*) with `phaseOut` and `phaseIn`.
- **Kamigawa flip cards (`[CR#710.1a]`):**
  - Replace the 5 `setStatus .flipped thisCreature` calls in
    `Cards/Faces.lean:38,166,203,225,239` (*Akki Lavarunner*, *Bushi Tenderfoot*,
    *Callow Jushi*, *Initiate of Blood*, *Jushi Apprentice*) with `flip thisCreature`
    (or `instruct thisCreature flip`).
- After this change, `setStatus` in `Cards/*.lean` is used only for `.tapped` /
  `.untapped` where no higher-level template applies (or zero sites if all tap/untap
  sites already use `tap` / `untap`).

### 3. Fold parameterized `keywordSubject` / `keywordCosting` / `keywordQuality` / `keywordNumber` calls (~95 sites)

- Replace all calls to `keywordSubject (.named "...")`, `keywordCosting (.named "...")`,
  `keywordQuality (.named "...")`, `keywordNumber (.named "...")`,
  `keywordQualityCosting (.named "...")`, and `keywordNumberCosting (.named "...")`
  in `lean/Semantics/Cards/*.lean` where a named `semantic_definition` exists in
  `Definitions.lean`:
  - `enchant` (41 Auras across `Cards/*.lean`)
  - `equip` (17 Equipment) and `equipQuality` (3 Equipment in `Cards/Keyword.lean`)
  - `protection` (13 sites) and `hexproofFrom` (*Knight of Grace* in `Cards/Deontic.lean`)
  - `ward`, `kicker`, `multikicker`, `cycling`, `typecycling`, `flashback`,
    `unearth`, `dredge`, `bushido`, `ninjutsu`, `miracle`, `warp`, `escalate`,
    `spectacle`, `affinity`, `saddle`, `crew`, `partnerWith`, `suspend`, `fortify`,
    `afterlife`, `echo`, `prowl`, `surge` (~25 sites across `Cards/*.lean`).

### 4. Fold raw `.named "..."` counter and designation literals and `hasKeyword (.the (.named "..."))`

- Replace raw `.named "timeCounter"`, `.named "ageCounter"`, `.named "poison"`,
  `.named "m1m1Counter"` in `Cards/*.lean` with `timeCounter`, `ageCounter`,
  `poison`, and `m1m1Counter`.
- Replace raw `.named "monarch"`, `.named "goaded"`, `.named "renowned"`,
  `.named "monstrous"`, `.named "ringBearer"`, `.named "commander"`, `.named "day"`,
  `.named "night"` in `Cards/*.lean` with their `Definitions.lean` identifiers.
- Replace all ~35 `hasKeyword (.the (.named "..."))` calls and `.the (.named "...")`
  `KeywordTerm` list items (*Odric, Lunarch Marshal*, *Soulflayer*, *Gabriel
  Angelfire*, *Urborg Scavengers*, *Cairn Wanderer*, etc.) in `Cards/*.lean` with
  bare keyword identifiers (`hasKeyword flying`, `[flying, firstStrike, ...]`).

### 5. Self-reference markers and active `cant` player subjects

- Replace `asMarker .token this_` (`Cards/Deontic.lean:771`) with `thisToken` and
  `asMarker .emblem this_` (`Cards/Damage.lean:1313`) with `thisEmblem`.
- In `Cards/Deontic.lean:72-197`, replace `allOf isPlayer` in active `cant`
  subjects with `eachPlayer` where Oracle text says *"Each player can't..."*
  (*Rule of Law*) and `bare isPlayer` where Oracle text says *"Players can't..."*
  (*Damping Field*, *Smoke*, *Winter Orb*, *Static Orb*, *Winter Moon*, *Everybody
  Lives!*, *Mornsong Aria*).

## Verification

- `rg 'Primitives\.|Semantics\.(Action|NounPhrase|Predicate|Condition)\.' lean/Semantics/Cards/ lean/Semantics/Definitions.lean`
  returns 0 matches.
- `rg '«this»' lean/Semantics/` returns 0 matches.
- `rg 'setStatus' lean/Semantics/Cards/` returns 0 matches.
- `./lean/scripts/build` (`lake build --wfail`) and `cargo xtask lean check` pass
  with zero warnings and zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check`
  0 stale.
