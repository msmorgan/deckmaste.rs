---
needs: [lean-v3-authoring-impl-4-checker-dewarting]
---
# Dewart Lean v3 `Templates.lean` and `Definitions.lean`

**Eliminate the remaining identifier inconsistencies, redundant parameters,
duplicate `*Expansion` definitions, and unfolded constructor trees in
`lean/Semantics/Templates.lean` and `lean/Semantics/Definitions.lean`.**
Standard constraints apply.

## Scope

### 1. Self-reference templates (`this_`, `thisCard`, `thisToken`, `thisEmblem`)

- In `lean/Semantics/Templates.lean`:
  - Rename `«this»` (`Templates.lean:81`, the only French-quoted identifier in
    `lean/Semantics/`) to **`this_`** so it follows the trailing-underscore
    convention used by every other Lean keyword collision (`by_`, `exists_`, `in_`,
    `repeat_`, `at_`, `matches_`, `splice_`, `seal_`, `class_`). Update all call
    sites across `Templates.lean`, `Definitions.lean`, `Proofs/*.lean`, and
    `Cards/*.lean`.
  - Add the missing self-reference marker templates alongside `thisPermanent`,
    `thisSpell`, and `thisAbility`:
    ```lean
    semantic_template thisCard : NounPhrase := .asMarker .card .this
    semantic_template thisToken : NounPhrase := .asMarker .token .this
    semantic_template thisEmblem : NounPhrase := .asMarker .emblem .this
    ```

### 2. Fix `mill` (`[CR#701.17a,701.17c]`), add `flip` (`[CR#710.1a]`), and consolidate `double*` / `triple*` (`[CR#701.10a..701.11b]`)

- **`mill` (`[CR#701.17a]`):**
  - Under `[CR#701.17a]`, a player mills from their own library (*"For a player to
    mill a number of cards, that player puts that many cards from the top of their
    library into their graveyard"*). Drop the redundant `(whose : NounPhrase)`
    parameter from `mill` in `Definitions.lean` so it reads `libraryOf .actor`
    (matching `scry`, `surveil`, `discard`, and `shuffle`):
    ```lean
    semantic_definition mill (amount : Amount) : Action :=
      .ownScope (some (.action "mill"))
        (.enact (.action "mill")
          (.move (.slice .top (.inCaller amount) (libraryOf .actor))
            (.zone .library none) (some graveyard) .arrival none [])
          none none)
    ```
  - Update all call sites of `mill` in `Definitions.lean` (including folding
    `dredge`'s inlined `mill` into `mill (.inCaller amount)`), `Cards/*.lean`, and
    `Proofs/*.lean`.
- **Passive card-patient `millCards` (`[CR#701.17c]`):**
  - Add `millCards (cards : NounPhrase) : Action` (or expected-type / argument
    dispatch on `mill` between `Amount` and `NounPhrase`) for event triggers whose
    grammatical subject is a card description (*"Whenever one or more nonland cards
    are milled"*, *Mirelurk Queen*, `[CR#701.17c]`):
    ```lean
    semantic_definition millCards (cards : NounPhrase) : Action :=
      .ownScope (some (.action "mill"))
        (.enact (.action "mill")
          (.move (.inCaller (.firstMention "millCards.cards" cards))
            library (some graveyard) .arrival none [])
          none none)
    ```
- **`flip` (`[CR#710.1a]`):**
  - Add `flip (subject : NounPhrase := .actor) : Action` in `Definitions.lean` for
    Kamigawa flip cards.
- **Consolidate `double*` and `triple*` (`[CR#701.10a..701.11b]`):**
  - Move `doublePowerAndToughness`, `doubleToughness`, `triplePower`, and
    `tripleToughness` from `Templates.lean` into `Definitions.lean` as
    `semantic_definition`s alongside `doubleCounters`, `doublePower`,
    `triplePowerAndToughness`, and `doubleLifeTotal`.

### 3. Deduplicate and fold `Definitions.lean`

1. **Delete the 8 duplicate `*Expansion` definitions:**
   - Delete `flashExpansion`, `sneakExpansion`, `morphExpansion`,
     `megamorphExpansion`, `disguiseExpansion`, `foretellExpansion`,
     `plotExpansion`, and `hideawayExpansion` (`Definitions.lean:814-875`), which
     duplicate `flash`, `sneak`, `morph`, `megamorph`, `disguise`, `foretell`,
     `plot`, and `hideaway`.
   - Point `Proofs/Turn.lean` (`okFlashExpansion`, `okSneakExpansion`) and
     `Proofs/FaceDown.lean` (`okMorph`, `okMegamorph`, `okDisguise`, `okForetell`,
     `okPlot`, `okHideaway`) at the canonical keyword definitions.
2. **Reorder leaf catalogs above `Keyword abilities` in `Definitions.lean`:**
   - Move `Subtypes` (`creatureType`, `artifactType`, `enchantmentType`, etc.),
     `Tokens` (`foodToken`, `treasureToken`, `clueToken`, `bloodToken`, `junkToken`,
     `landerToken`, `mapToken`, `powerstoneToken`, `goldToken`, `shardToken`), and
     `Templates over Definitions` (`createToken`, `createTappedToken`, `exileUntil`,
     `attachTo`, etc.) above the `Abilities` (`Keyword abilities` and `Keyword actions`)
     section so keyword definitions can call them directly.
   - Add `day` and `night` to the `Designations` section (`[CR#731.1]`).
   - Add `hexproofFrom (quality : Predicate) : Ability` alongside `hexproof`
     (`[CR#702.11d]`), and fix `equipQuality` (`Definitions.lean:1464`) to use
     `.named "equip"` instead of `.named "equipQuality"`.
3. **Convert the 6 licensed `[CR#702.174a]` `gift*` phrasings to `semantic_template`s over `gift` (`Definitions.lean:1632-1806`):**
   - Because `[CR#702.174a]`'s templating is non-normative (`"Gift a card"`, `"Gift a tapped Fish"`, etc.), bare `gift` is the `semantic_definition` while each licensed phrasing is its own **`semantic_template`** calling `gift (...)` (matching `plainscycling` .. `wizardcycling` over `typecycling`):
     ```lean
     semantic_template giftACard : Ability := gift (draw (.lit 1))
     semantic_template giftAFood : Ability := gift (createToken foodToken)
     semantic_template giftATappedFish : Ability := gift (createTappedToken (creatureToken 1 1 [.blue] [fish]))
     semantic_template giftATreasure : Ability := gift (createToken treasureToken)
     semantic_template giftAnExtraTurn : Ability := gift (extraTurn (.lit 1))
     semantic_template giftAnOctopus : Ability := gift (createToken (creatureToken 8 8 [.blue] [octopus]))
     ```
4. **Fold unfolded definitions and templates inside `Definitions.lean`:**
   - Across `Definitions.lean`, fold inlined constructor trees back into calls to
     `sacrifice`, `exile`, `tap`, `untap`, `destroy`, `discard`, `transform`,
     `attachTo`, `cast`, `play`, `activate`, `mill`, `die`, `enter`, `leave`,
     `createToken`, `createTappedToken`, `thisPermanent`, `thisSpell`, `thisCard`,
     and named subtypes/tokens/counters/designations.
   - Replace `allOf isPlayer` in `splitSecond` (`Definitions.lean:2491`) with
     `bare isPlayer` (*"Players can't cast spells or activate abilities..."*).
   - Clean up the triple-nested `enact` + dummy `.clearDamage` hack in `umbraArmor`
     (use `destroy (enchanted permanent)`), `mayhem` (use `discard this_`),
     `madness`, and `theRingTemptsYou`.

### 4. Support bare keyword identifiers in `hasKeyword` and `KeywordTerm` lists

- In `Templates.lean` and `Definitions.lean`, extend `expectedTypeDispatch` (or add
  `KeywordTerm` overloads/instances) for evergreen and common keyword abilities
  (`flying`, `firstStrike`, `doubleStrike`, `deathtouch`, `defender`, `flash`,
  `haste`, `hexproof`, `indestructible`, `lifelink`, `menace`, `reach`, `trample`,
  `vigilance`, `ward`, `shroud`, `fear`, `intimidate`, `prowess`, `changeling`,
  `convoke`, `delve`, `devoid`, `exalted`, `cascade`, `flanking`, `horsemanship`,
  `infect`, `ingest`, `livingWeapon`, `melee`, `mentor`, `myriad`, `persist`,
  `phasing`, `provoke`, `rebound`, `retrace`, `riot`, `shadow`, `skulk`,
  `splitSecond`, `storm`, `sunburst`, `training`, `umbraArmor`, `undying`,
  `unleash`, `wither`) so `hasKeyword flying`, `hasKeyword defender`, and
  `KeywordTerm` lists (`[flying, firstStrike, trample]`) elaborate without
  `.the (.named "...")` boilerplate.
- Fold `hasKeyword (.the (.named "..."))` sites in `Definitions.lean` (`flying`,
  `backup`, etc.) onto bare keyword identifiers.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) and `cargo xtask lean check` pass
  with zero warnings and zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check`
  0 stale.
