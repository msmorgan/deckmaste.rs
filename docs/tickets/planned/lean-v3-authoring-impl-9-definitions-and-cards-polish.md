---
needs: [lean-v3-authoring-impl-8-primitive-decomposition]
---
# Polish `Definitions.lean`, `Templates.lean`, `Cards/*.lean`, and `Proofs/*.lean`

**Fold the remaining ~546 raw dot-constructors and 10 `*Expansion` helpers in
`lean/Semantics/Definitions.lean`, add missing `:= none` defaults on `Option`
parameters, eliminate the remaining 27 raw `.named "..."` literals and AST
plumbing in `lean/Semantics/Cards/*.lean`, and fold raw constructors in
`lean/Semantics/Proofs/*.lean`.** Standard constraints apply.

## Scope

### 1. Fold raw dot-constructors and `*Expansion` helpers in `Definitions.lean`

- Across `lean/Semantics/Definitions.lean`:
  - Replace the ~546 raw dot-constructors where `Templates.lean` helpers exist:
    - `.static` (133 sites) → `static`
    - `.and` (110 sites), `.not` (34 sites), `.or` (14 sites) → `and`, `not` / `non`, `or`
    - `.sequentially` (53 sites) → `sequentially`
    - `.matches` (47 sites) → `matches_`
    - `.does` (37 sites) → `does`
    - `.doIf` (20 sites) → `doIf`
    - `.instruct` (19 sites) → `instruct`
    - `.spell none` (6 sites) → `spell`
    - `.abilityGrant` (6 sites) → `abilityGrant`
    - `.entryRider` (2 sites) → `entryRider`
  - Inline the 10 remaining `*Expansion` helper definitions (`levelUpExpansion`,
    `renownExpansion`, `stormExpansion`, `cascadeExpansion`, `rippleExpansion`,
    `cumulativeUpkeepExpansion`, `flyingExpansion`, `bushidoExpansion`,
    `modularExpansion`, `cyclingExpansion` at `Definitions.lean:1388-1496`) directly
    into their owning keyword `semantic_definition`s.

### 2. Add missing `:= none` defaults on `Option` parameters

- Keep explicit `(some ...)` in Lean (per Governing Principle 3, `Option` wrapping
  is handled by the Rust `v3` DSL), and add `:= none` defaults where missing:
  - In `lean/Semantics/Templates.lean`:
    - `gain (subject : NounPhrase) (ability : Ability) (duration : Option Duration := none) : Action`
    - `gainHaste (subject : NounPhrase) (duration : Option Duration := none) : Action`
    - `become (subject : NounPhrase) (added : CharacteristicBundle) (duration : Option Duration := none) : Action`
    - `becomeColor (subject : NounPhrase) (colors : ColorSpec) (duration : Option Duration := none) : Action`
    - `allAbilityLoss (subject : NounPhrase) (except : Option Predicate := none) : StaticSpec`
  - In `lean/Semantics/Definitions.lean`:
    - `doublePower (target : NounPhrase) (duration : Option Duration := none) : Action` (and align its body with `doubleToughness` / `doublePowerAndToughness`)
    - `triplePowerAndToughness (target : NounPhrase) (duration : Option Duration := none) : Action` (and align its body with `triplePower` / `tripleToughness`)
- Remove trailing `none` arguments at call sites of `gain`, `gainHaste`, `become`,
  `becomeColor`, and `allAbilityLoss` across `Cards/*.lean` and `Definitions.lean`.

### 3. Eliminate the remaining 27 raw `.named "..."` literals and AST plumbing in `Cards/*.lean`

1. **`KeywordLabel` dispatch / constants for keyword references:**
   - Extend `expectedTypeDispatch` (or export `KeywordLabel` definitions/coercions)
     so positions expecting `KeywordLabel` or `KeywordTerm` (`keywordCostPaid`,
     `abilityHead`, `toPay (.ofKeyword ...)`, `pay anyCost ...`, `byKeyword`,
     `byNthKeyword`, `KeywordTerm.anyIn`, `hasKeyword`) accept named keyword
     identifiers (`kicker`, `surge`, `cumulativeUpkeep`, `crew`, `cycling`,
     `exhaust`, `powerUp`, `equip`, `morph`, `disguise`, `disturb`, `prowl`, `echo`,
     `sneak`, `spectacle`, `protection`, `landwalk`, `ward`, `bandsWithOther`,
     `craft`, `emerge`, `rampage`) without raw `.named "..."` strings.
   - Fold all 27 remaining `.named "..."` occurrences in `lean/Semantics/Cards/*.lean`
     (including `hasKeyword (.the (.named "emerge"))` at `Cards/Trigger.lean:804`).
2. **Retire card-local `firstMention` / `laterMention` templates in `Cards/Counters.lean`:**
   - Fix DRS anaphor resolution across `putCounters` (or provide a reusable template
     in `Templates.lean`) so *Thranduil's Company* (`Cards/Counters.lean:1003-1015`)
     and *Stunning Shot* (`Cards/Counters.lean:1027-1041`) resolve `it` without
     card-private `companyRecipient` / `stunningShotRecipient` templates or raw
     `.described` / `.hasPossessor`.
3. **Clean up remaining card-level warts:**
   - Replace the 9 remaining `allOf isPlayer` sites in `Cards/Keyword.lean:628,1209`,
     `Cards/Static.lean:720,756,760,767`, `Cards/Deontic.lean:381,387`, and
     `Cards/Description.lean:368` with `bare isPlayer` or `eachPlayer` to match
     Oracle text.
   - Replace `[.of .battle "Siege"]` in `Cards/Faces.lean:44` with `[siege]`.
   - Add `semantic_template anOpponentControls : Predicate := controlledBy anOpponent`
     and `thatToken` / `thoseTokens` in `Templates.lean` and fold call sites in
     `Cards/*.lean`.
   - Fold raw `.keyword` / `.spell` / `.static` constructor calls in
     `Proofs/KeywordArguments.lean`, `Proofs/Faces.lean`, and
     `Proofs/CallerScope.lean` onto `Templates.lean` and `Definitions.lean`.

## Verification

- `rg '\.named "' lean/Semantics/Cards/` returns 0 matches.
- `rg '\.(static|sequentially|matches|does|doIf|instruct|entryRider)\b' lean/Semantics/Definitions.lean`
  returns 0 matches.
- `rg 'Expansion\b' lean/Semantics/Definitions.lean` returns 0 matches.
- `./lean/scripts/build` (`lake build --wfail`) and `cargo xtask lean check` pass
  with zero warnings and zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check`
  0 stale.
