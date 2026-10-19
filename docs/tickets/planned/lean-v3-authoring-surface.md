---
needs: [lean-v3-authoring-impl-1-core-ast-and-checker, lean-v3-authoring-impl-2-templates-and-definitions, lean-v3-authoring-impl-3-cards-reauthoring, lean-v3-authoring-impl-4-checker-dewarting, lean-v3-authoring-impl-5-definitions-dewarting, lean-v3-authoring-impl-6-cards-dewarting, lean-v3-authoring-impl-7-checker-and-regime-retirement, lean-v3-authoring-impl-8-vocabulary-split-and-noun-heads, lean-v3-authoring-impl-9-primitive-decomposition, lean-v3-authoring-impl-10-definitions-and-cards-polish]
---
# Refactor the Lean v3 authoring surface before mirroring into Rust and plugins_v3

**Refactor the `lean/Semantics/` authoring surface so that authored cards and
keyword definitions read like Oracle English while expanding into the minimal
Core AST primitives, and retire all `v1`/`v2` module, macro, and alias
holdovers before `semantics-v3-dsl-and-plugins` mirrors the workbench into
`crates/deckmaste_semantics_v3` and `plugins_v3/`.** Umbrella epic for:
[1 Core AST and checker](lean-v3-authoring-impl-1-core-ast-and-checker.md),
[2 Templates and Definitions](lean-v3-authoring-impl-2-templates-and-definitions.md),
[3 Cards re-authoring](lean-v3-authoring-impl-3-cards-reauthoring.md),
[4 Checker dewarting](lean-v3-authoring-impl-4-checker-dewarting.md),
[5 Definitions dewarting](lean-v3-authoring-impl-5-definitions-dewarting.md),
[6 Cards dewarting](lean-v3-authoring-impl-6-cards-dewarting.md),
[7 Checker & regime retirement](lean-v3-authoring-impl-7-checker-and-regime-retirement.md),
[8 Vocabulary split & noun heads](lean-v3-authoring-impl-8-vocabulary-split-and-noun-heads.md),
[9 Primitive decomposition](lean-v3-authoring-impl-9-primitive-decomposition.md), and
[10 Definitions & cards polish](lean-v3-authoring-impl-10-definitions-and-cards-polish.md).
Standard constraints apply.

## Governing Principles

1. **Minimal Core AST Primitives (`Words.lean`, `Events.lean`, `Abilities.lean`):**
   - The Core AST remains the minimal set of orthogonal primitives so
     `deckmaste_lowering_v3` handles a small, closed constructor basis rather
     than dozens of overlapping surface forms.
2. **Surface-Faithful Authoring via `Template`s and `Definition`s:**
   - The surface of an authored card in `lean/Semantics/Cards/` or a keyword
     `Definition` must remain directly parallel to the English card text so
     spelling attachment and the card ingestion pipeline work cleanly.
   - Because `lean/Semantics/Authoring.lean` records `.call name args` on every
     `semantic_template` and `semantic_definition`, rich surface vocabulary is
     built as `Template`s and `Definition`s expanding into the minimal Core AST
     primitives.
3. **Literal and Enum Coercions Belong to the Rust `v3` DSL:**
   - Bare numeral coercion (`1` vs `.lit 1`), list coercions (`[...]` → `And` /
     `Sequentially` / `Compound` / `Conjunction`), and leading `.` enum prefixes
     are handled by `deckmaste_dsl_v3` in `semantics-v3-dsl-and-plugins`; Lean
     retains explicit `.lit` and top-level combinator templates (`and`,
     `sequentially`, `compound`, `conjunction`) so Lean's elaborator and kernel
     `by decide` never pay typeclass search or projection-unfolding overhead.
4. **No `v1`/`v2` Holdover Names:**
   - Retire the `Semantics.Macros` hierarchy, `Declared.lean` split,
     `semantic_macro` command, `MacroParameters` / `MacroContext` / `MacroSplice`
     identifiers, empty `Phrase.lean` / `Triggers.lean` shims, `Binding` /
     `Bindings` aliases, and the `Semantic Macro` glossary entry in favor of
     `Primitives`, `Templates`, `Definitions`, `CallerScope`, `AuthoringContext`,
     `Referent`, and `DRS`.

## Implementation Tickets

1. [`lean-v3-authoring-impl-1-core-ast-and-checker`](lean-v3-authoring-impl-1-core-ast-and-checker.md):
   Land all Core AST and checker changes in one pass (`Instruction` → `Action`,
   unified `Location` algebra + `Predicate.in_` + flattened `MoveRider`,
   `StaticSpec.can` + simplified `DeonticMods` + `Quantity` bounds +
   `AsThough.condition`, relational `Predicate.isOpponentOf` / `isTeammateOf`
   retiring `PlayerGroupWord`, and `CallerScope` / `Authoring` / `Payload.difference`
   renames + `CONTEXT.md` amendment).
2. [`lean-v3-authoring-impl-2-templates-and-definitions`](lean-v3-authoring-impl-2-templates-and-definitions.md):
   Consolidate `Macros.lean` and `Macros/Declared.lean` into
   `lean/Semantics/Primitives.lean`, `lean/Semantics/Templates.lean`, and
   `lean/Semantics/Definitions.lean`, landing the complete `v3` template
   vocabulary (4 deontics, intransitive/transitive verb spine, uniform
   boolean-like `Predicate`s, attached hosts, `Location` helpers, and top-level
   combinators), re-expressing all `Definition` bodies through `Templates.lean`
   helpers, and updating `lean/Semantics/Proofs/`.
3. [`lean-v3-authoring-impl-3-cards-reauthoring`](lean-v3-authoring-impl-3-cards-reauthoring.md):
   Re-author all 15 `lean/Semantics/Cards/*.lean` modules onto the new
   `Templates.lean` and `Definitions.lean` surface so authored cards read like
   Oracle English and contain zero `Primitives.*` qualifications.
4. [`lean-v3-authoring-impl-4-checker-dewarting`](lean-v3-authoring-impl-4-checker-dewarting.md):
   Retire the remaining non-anaphora game-rule checks in `lean/Semantics/Check/`
   (`ActFacts` role tables, `Check/Possessor.lean`, counter-holder/memory checks,
   `statHeadTysOk`, `contradictionFree`, zone checks in `handoffPerformer` and
   `designationHolder`, `settableValue`, `permanentSpellType`, `ascriptionOk`,
   and `riderKind`) and update `Proofs/` and `lean/CONTRACTS.md`.
5. [`lean-v3-authoring-impl-5-definitions-dewarting`](lean-v3-authoring-impl-5-definitions-dewarting.md):
   Dewart `Templates.lean` and `Definitions.lean` (`«this»` → `this_`,
   `thisCard`/`thisToken`/`thisEmblem`, `mill` without `whose` + `millCards`,
   `flip`, consolidated `double*`/`triple*`, deleting the 8 duplicate
   `*Expansion` definitions, converting the 6 `gift*` phrasings to
   `semantic_template`s over `gift`, folding unfolded keyword bodies, and
   dispatching bare keyword definitions in `hasKeyword`).
6. [`lean-v3-authoring-impl-6-cards-dewarting`](lean-v3-authoring-impl-6-cards-dewarting.md):
   Sweep `lean/Semantics/Cards/*.lean` onto the dewarted surface (`mirelurkQueenTrigger`
   onto `millCards`, remaining `setStatus` calls onto `phaseOut`/`phaseIn`/`flip`,
   ~95 parameterized `keyword*` calls onto `Definitions.lean` definitions, raw
   counter/designation `.named "..."` literals, `hasKeyword` bare keywords, and
   `eachPlayer`/`bare isPlayer` in active `cant` subjects).
7. [`lean-v3-authoring-impl-7-checker-and-regime-retirement`](lean-v3-authoring-impl-7-checker-and-regime-retirement.md):
   Retire `KeywordFacts` body/regime checking (`keywordBodyFits`, `StackRegime`,
   `ComponentFacts`, `ClassGuard`), `CardFace.choices` (`jointChoices`), the
   remaining phrase-level game-rule refusals (`deonticPatientOk`, `deckReadable`,
   `deckComparable`, `qualifiedLimit`, `counterBatchOk`, `selfDefinedOk`,
   `manaSymbolOk`, `colorBoundOk`, `colorCountOk`, `rolesOk`, `subtypeType`,
   `twoParties`, `kindAmountOk`, `verbBecomesOk`), and dead `Check/` fields, while
   fixing `changeLife (.set _)` outcome publication (`[CR#119.5]`).
8. [`lean-v3-authoring-impl-8-vocabulary-split-and-noun-heads`](lean-v3-authoring-impl-8-vocabulary-split-and-noun-heads.md):
   Split `Definitions.lean` into leaf catalog modules (`Counters`, `Designations`,
   `Relations`, `Subtypes`) imported by `Templates.lean` and composite definition
   modules (`Actions`, `Keywords`) importing `Templates.lean`; collapse
   `NounPhrase.asType` and `NounPhrase.asMarker` (retiring `MarkerWord` and the
   dead `Option Subtype` field); and seed `Predicate.did` (`which`) from its
   enclosing predicate so `(asMarker .permanent (relative .object))` disappears
   from `Cards/`.
9. [`lean-v3-authoring-impl-9-primitive-decomposition`](lean-v3-authoring-impl-9-primitive-decomposition.md):
   Decompose CR-defined and overlapping Core AST primitives: fold Day/Night
   (`[CR#731.1]`) and Room doors (`[CR#709.5c..709.5j]`) into designations;
   decompose `moveCounters` (`[CR#122.5]`), `doubleCounters` (`[CR#701.10e]`),
   `crew`/`saddle` (`[CR#702.122b,702.171c]`), `skipUntap` (`[CR#701.43a]`),
   `doAndForbid`, `drawGame` (`[CR#104.4]`), `triggerReflexively`/`triggerThisWay`
   (`[CR#603.12]`), and `GameEvent.triggers` (`[CR#603.2]`); and unify
   `StaticSpec.modification` / `ptModification` / `abilityGrant*` into
   `CharacteristicEdit` inside `StaticSpec.characteristicChange`.
10. [`lean-v3-authoring-impl-10-definitions-and-cards-polish`](lean-v3-authoring-impl-10-definitions-and-cards-polish.md):
    Fold the remaining ~546 raw dot-constructors and 10 `*Expansion` helpers in
    `Definitions/`, add missing `:= none` defaults on `Option` parameters,
    eliminate the 27 remaining `.named "..."` literals and card-local
    `firstMention`/`laterMention` workarounds in `Cards/*.lean`, and fold raw
    constructors in `Proofs/*.lean`.

---

## Detailed Design Specification

### 1. Core AST Rename: `Instruction` → `Action`

Because the Unified Action-Clause Spine embeds the same verb-phrase AST inside
imperative instructions, `instruct`, `Cost.perform`, `GameEvent.does`,
`WouldClause.would`, `StaticSpec.cant`/`.can`/`.must`/`.gatedBy`, and
`DidClause.did` — including unaccusative observations like `die`, `enter`, and
`leave` that are never imperative instructions — rename `Instruction` to
**`Action`** across `lean/Semantics/`.

---

### 2. Stage 1: Deontic & Gated Static Clauses

#### 2.1 `StaticSpec.may` → `StaticSpec.can`
- Rename `StaticSpec.may` to **`StaticSpec.can`** (`[CR#101.2]`), reserving
  `may` exclusively for optional one-shot actions
  (`Action.withContinuation .optional`, `[CR#603.5]`).

#### 2.2 Four Deontic Templates Only (`cant`, `can`, `must`, `gatedBy`) — No Fused `*Be` Variants
- Voice is carried by `Subject`, never by fused `*Be` template names:
  - **Active (`Subject.actor np`):** written as bare `np` via
    `Coe NounPhrase Subject`.
  - **Unspecified-agent passive (`Subject.any`):** written as `any`.
  - **Explicit-agent passive (`Subject.by np`):** written as `by_ np` in Lean
    (`by(np)` in the Rust `v3` DSL, matching `exists_` / `exists(...)`).
- Expose `DeonticMods` fields as optional keyword arguments directly on `cant`,
  `can`, `must`, and `gatedBy`:
  ```lean
  semantic_template cant (subj : Subject) (action : Action)
      (timing : Option Timing := none)
      (asThough : Option AsThough := none)
      (except : Option DeonticExcept := none) : StaticSpec :=
    .cant subj action { timing, asThough, except }

  semantic_template can (subj : Subject) (action : Action)
      (timing : Option Timing := none)
      (asThough : Option AsThough := none)
      (except : Option DeonticExcept := none) : StaticSpec :=
    .can subj action { timing, asThough, except }

  semantic_template must (subj : Subject) (action : Action)
      (timing : Option Timing := none)
      (asThough : Option AsThough := none)
      (except : Option DeonticExcept := none) : StaticSpec :=
    .must subj action { timing, asThough, except }

  semantic_template gatedBy (cost : Cost) (subj : Subject) (action : Action)
      (timing : Option Timing := none)
      (asThough : Option AsThough := none)
      (except : Option DeonticExcept := none) : StaticSpec :=
    .gatedBy cost subj action { timing, asThough, except }
  ```
- Delete `deonticActionFor` and all legacy wrapper templates (`objectCant`,
  `playerCant`, `cantMoreThan`, `cantDoTo`, `canDoAsThough`, `mayPlayDeed`,
  `mayPlayAdditionalLands`, `mayBlockAdditional`, `mayVoteAdditional`,
  `forbidAttack`, `forbidBlock`, `forbidBeingBlocked`, `cantBeBlockedBy`,
  `cantBeBlockedExceptBy`, `cantExcept`).

#### 2.3 Retire `DeonticRider` (`stateBased | noRider | play`)
- Delete `DeonticRider` completely:
  - **`from_` and `payment`** live on `cast`, `play`, and `activate`
    (`Action.procedure`), which already carry `(payment : PlayPayment)` and
    `(from_ : Option Location)` and are already stepped by `Check/Phrase.lean`:
    ```lean
    semantic_definition cast (subject : NounPhrase)
        (from_ : Option Location := none)
        (payment : PlayPayment := .itsOwnCost)
        (resulting : Option Predicate := none) : Action
    ```
  - **`timing : Option Timing := none`** moves directly onto `DeonticMods`
    (covering rate/window adjuncts such as `eachTurn`, `onEachOfYourTurns`,
    `eachCombat`, `duringTheirUntapSteps`, `whileVoting`). Note:
    *"attacks each combat if able"* (`must thisCreature attack`) requires no
    timing override.
  - **`exclusive : Bool`** is deleted: *"You may cast ~ from your graveyard,
    but not from anywhere else"* (*Haakon, Stromgald Scourge*) and *"Cast this
    spell only from your graveyard"* (Aftermath `[CR#702.127a]`) are expressed
    as a `can` + `cant` pair:
    ```lean
    can you (cast this (from_ := yourGraveyard))
    cant you (cast this (from_ := not yourGraveyard))
    ```
  - **`stateBased` (owner ruling 2026-10-18):** the cause rides the prohibited action, not the
    rule: *"can't lose the game for having 0 or less life"* (*Phyrexian Unlife*, `[CR#704.5a]`)
    is `cant you (loseGame (cause := nonpositiveLife))`, over `Action.conclude (verb) (cause)`.

#### 2.4 Quantifiers on `Quantity` (Retiring `DeonticMods.bound`)
- Delete `DeonticMods.bound` (`Option CountBound`).
- Add `.moreThanOf (amount : Amount)` (`moreThan n`) and
  `.additional (quantity : Quantity)` (`additional q`) to `Quantity` so noun
  phrases carry their own quantifiers directly:
  - *"Each player can't cast more than one spell each turn"* (*Arcane Laboratory*):
    `cant (each isPlayer) (cast (counted (moreThan 1) isSpell)) (timing := eachTurn)`
  - *"~ can't be blocked by more than one creature"* (*Alpha Authority*):
    `cant (by_ (counted (moreThan 1) isCreature)) (block (enchanted creature))`
  - *"You may play an additional land on each of your turns"* (*Exploration*):
    `can you (play (counted (additional (exactly 1)) isLand)) (timing := onEachOfYourTurns)`
  - *"~ can block an additional creature each combat"* (*Hundred-Handed One*):
    `can thisCreature (block (counted (additional (exactly 1)) isCreature)) (timing := eachCombat)`
  - *"You may vote an additional time"* (*Brago's Representative*):
    `can you (vote (times := additional (exactly 1)))`
  - *"Once during each of your turns, you may cast …"* (owner ruling 2026-10-18): the count on
    the noun phrase, the rate as timing, by analogy with *Exploration*:
    `can you (cast (counted (exactly 1) …)) (timing := onEachOfYourTurns)`; there is no usage
    limit on a deontic.
- **`DeonticExcept.byObjects (objects : NounPhrase)` and Menace (`[CR#702.111b]`):**
  - Replace `DeonticExcept.byObjects (predicate : Predicate)` with
    `DeonticExcept.byObjects (objects : NounPhrase)` so quantified and bare-plural evasion
    exceptions share one representation:
    - `flying` (`[CR#702.9b]`): `cant any (block thisCreature) (except := byObjects (bare (and [isCreature, or [hasKeyword flying, hasKeyword reach]])))`
    - `fear` (`[CR#702.36b]`): `cant any (block thisCreature) (except := byObjects (bare (and [isCreature, or [isArtifact, isBlack]])))`
    - `intimidate` (`[CR#702.13b]`): `cant any (block thisCreature) (except := byObjects (bare (and [isCreature, or [isArtifact, shares .color .share thisCreature]])))`
    - `menace` (`[CR#702.111b]`): `cant any (block thisCreature) (except := byObjects (counted (atLeast 2) isCreature))` (fixing the legacy `moreThan 1` definition in `Declared.lean`).
  - With `DeonticMods.bound` retired, `CountBound.atLeast` and `CountBound.additional` have zero
    remaining callers; simplify `UsageLimit.mk (times : Amount) (per : Period)` (`[CR#602.5b]`)
    and retire `CountBound`.
- Replace `allPlayers` in active subjects where Oracle English says *"Each
  player"* (`each isPlayer`) or *"Players"* (`bare isPlayer`), and replace
  `cast (allOf isSpell)` with `cast (a isSpell)` / `cast (bare isSpell)`.

#### 2.5 `AsThough` Carries a `Condition` (Preserving Pronouns & Fixing Polarity)
- Replace `AsThough.of (predicate : Predicate)` with
  `AsThough.condition (cond : Condition)` (with coercion from `Condition` to
  `AsThough`) so the subject pronoun in *"as though **it** didn't have
  defender"*, *"as though **they** didn't have hexproof"*, and *"as though
  **they** had flash"* is preserved and checked by DRT (`D′`):
  - *Warmonger's Chariot*:
    `can thisCreature attack (asThough := matches it (not (hasKeyword defender)))`
  - *Glaring Spotlight*:
    `can (by_ (allOf (and [or [isSpell, isAbilityOnStack], youControl]))) (target (allOf (and [isCreature, opponentsControl]))) (asThough := matches them (not (hasKeyword hexproof)))`
  - *Teferi, Mage of Zhalfir*:
    `can you (cast (a (and [isCreature, isCard]))) (asThough := matches them (hasKeyword flash))`
- Fix the inverted `asThough` polarity on *Warmonger's Chariot* and *Glaring
  Spotlight* (which previously encoded positive `hasKeyword` instead of
  `not (hasKeyword ...)`).

---

### 3. Cross-Cutting: Unified `Location` Algebra

#### 3.1 Replace `ZoneExpr`, `Pile`, `EventSource`, `SearchScope`, and `ZoneScope` with `Location`
- Delete `ZoneExpr`, `Pile`, `EventSource`, `SearchScope`, and `ZoneScope`.
- Introduce a single orthogonal **`Location`** inductive in `Abilities.lean`:
  ```lean
  inductive Location where
    | inCaller (body : Location)
    | zone (zone : Zone) (whose : Option NounPhrase := none)
    | fromEnd (end_ : PileEnd) (ordinal : Ordinal := .nth 1) (loc : Location)
    | exiledWith (source : NounPhrase)
    | pile (pile : NounPhrase)
    | anyLocation
    | not (loc : Location)
    | or (locs : List Location)
  ```
  with `instance : AnyValue Location := ⟨.anyLocation⟩` (`any : Location`) and
  templates:
  - `topOf loc` := `.fromEnd .top (.nth 1) loc`
  - `bottomOf loc` := `.fromEnd .bottom (.nth 1) loc`
  - `fromTop n loc` := `.fromEnd .top (.nth n) loc`
  - `fromBottom n loc` := `.fromEnd .bottom (.nth n) loc`

#### 3.2 Collapse Location Predicates and `MoveRider` Placement Hacks
- Replace the three separate `Predicate` constructors (`Predicate.inZone`,
  `Predicate.inPile`, `Predicate.exiledWith`) with a single
  **`Predicate.in_ (loc : Location)`** (or `atLocation`).
- Delete `PilePlacement.nthFromEnd` and `PilePlacement.orOtherEnd`:
  - *"into its owner's library seventh from the top"* (*Approach of the Second
    Sun*, `[CR#401.7]`): `putInto this (fromTop 7 (libraryOf (ownerOf this)))`
  - *"on the top or bottom of your library"*:
    `or [topOf yourLibrary, bottomOf yourLibrary]`
- Flatten `EntryRider` and the remaining `PilePlacement` constructors
  (`inOrder`, `faceDown`, `withCounters`) into a single `MoveRider` enum.
- `Location.positiveZones? : Location → Option (List Location)` drives both LKI
  (`historicalMoveFrom := some .battlefield`) and `putInto` departure-vs-arrival
  classification (`[CR#603.6c,603.10a]`).

---

### 4. Stage 2: Verb and Clause Spine

#### 4.1 Retire 3p-Singular `GameEvent` Wrappers in Favor of `does <subj> <action>`
- Retire all ~35 `v2`-port 3rd-person singular `GameEvent` wrapper templates in
  `Primitives.lean`, `Macros.lean`, and `Declared.lean` (`dies`, `enters`,
  `leaves`, `leavesZone`, `attacks`, `attacksPlayer`, `attacksWith`, `blocks`,
  `becomesBlocked`, `becomesTarget`, `casts`, `activates`, `discards`, `draws`,
  `losesGame`, `commitsCrime`, `flipsCoin`, `regenerates`, `dealsDamage`,
  `dealsCombatDamage`, `isDealtDamage`, `tappedForMana`, `unlocksDoor`,
  `statusEvent`, `zoneChange`, `verbedEvent`, etc.).
- Every event trigger uses `does <subj> <action>` with the exact same `Action`
  verb templates used in `would`, `cant`/`can`/`must`/`gatedBy`, `did`, and
  `instruct`:
  - `when (does thisCreature die)`
  - `whenever (does thisCreature attack)`
  - `whenever (does you (cast (a isSpell)))`
  - `whenever (does any (block thisCreature))` (*"becomes blocked"*)
  - `whenever (does (by_ (a isSpell)) (target thisCreature))` (*"becomes the target of a spell"*)

#### 4.2 Intransitive (`.actor`) vs. Transitive (`subject`) Zone Transitions
- `Action.move` remains the single Core AST zone-transition primitive.
- **Transitive move definitions/templates** (`putInto X loc`, `exile X`,
  `destroy X`, `sacrifice X`, `discard X`) take `X : NounPhrase` as their
  patient argument, leaving the enclosing clause's `Subject` (`you`,
  `target isOpponent`, `any`) as the performing agent:
  - *"Target opponent sacrifices a creature"*:
    `instruct (target isOpponent) (sacrifice (a isCreature))`
  - *"Whenever a card is put into your graveyard from anywhere other than the
    battlefield"*:
    `whenever (does any (putInto (a isCard) yourGraveyard (from_ := not battlefield)))`
- **Intransitive (unaccusative) move templates** (`die`, `enter`, `leave`) take
  **no `subject` argument** and pass **`NounPhrase.actor`** as `move`'s
  `subject` (matching `explore` `[CR#701.44a]` and `connive` `[CR#701.50a]`), so
  the moving object sits in the active `Subject` slot of `does`, `would`,
  `cant`, `did`, and `instruct`:
  ```lean
  semantic_template die : Action :=
    .move .actor battlefield (some graveyard) .departure none []

  semantic_template enter (from_ : Option Location := none)
      (riders : List MoveRider := []) : Action :=
    .move .actor (from_.getD any) (some battlefield) .arrival none riders

  semantic_template leave (from_ : Location := battlefield) : Action :=
    .move .actor from_ none .departure none []
  ```
  - *"When this creature dies"*: `when (does thisCreature die)`
  - *"If a creature an opponent controls would die"*:
    `instead (would (a (and [isCreature, opponentsControl])) die) ...`
  - *"a creature that died this turn"*:
    `a (and [isCreature, did which die thisTurn])`
  - *"Whenever a land you control enters from anywhere other than your hand"*:
    `whenever (does (a (and [isLand, youControl])) (enter (from_ := not yourHand)))`
  - *"Creatures can't enter from graveyards"*:
    `cant (a isCreature) (enter (from_ := graveyard))`

#### 4.3 Status Changes (`setStatus`): Intransitive vs. Transitive
- **`phaseOut` and `phaseIn` are intransitive (`[CR#702.26b]`):**
  ```lean
  semantic_template phaseOut : Action := .setStatus .phasedOut .actor
  semantic_template phaseIn : Action := .setStatus .phasedIn .actor
  ```
  - *"Target creature phases out"*: `instruct (target isCreature) phaseOut`
  - *"Whenever ~ phases out"*: `whenever (does thisCreature phaseOut)`
- **`become (status : Status)` (`becomeTapped`, `becomeUntapped`) is intransitive (`[CR#603.2e]`):**
  ```lean
  semantic_template become (status : Status) : Action := .setStatus status .actor
  ```
  - *"Whenever a permanent becomes untapped"*:
    `whenever (does (a isPermanent) (become untapped))`
  - Distinct from the transitive keyword actions `tap (subject : NounPhrase)`
    and `untap (subject : NounPhrase)` (`[CR#701.26a..701.26b]`), which wrap
    `.enact (.action "tap" / "untap")`.
- **`turnFaceUp` and `turnFaceDown` are transitive:**
  ```lean
  semantic_template turnFaceUp (subject : NounPhrase) : Action :=
    .setStatus .faceUp subject
  ```
  - *"Whenever a permanent you control is turned face up"*:
    `whenever (does any (turnFaceUp (a (and [isPermanent, youControl]))))`

#### 4.4 `dealDamage`, `winGame` / `loseGame`, `flipCoins`, and `rollDice` with Optional Kwargs
- **`dealDamage`:**
  ```lean
  semantic_template dealDamage (amount : Amount := any)
      (to : NounPhrase := any)
      (kind : DamageKind := .any) : Action :=
    .dealDamage amount to kind
  ```
  - Imperative: `instruct thisCreature (dealDamage 3 (to := target isAnyTarget))`
  - Active trigger: `whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly)))`
  - Passive trigger: `whenever (does any (dealDamage (to := thisCreature)))`
- **`winGame` and `loseGame` (`Action.conclude`):**
  ```lean
  semantic_template winGame : Action := .conclude .winGame
  semantic_template loseGame : Action := .conclude .loseGame
  ```
  - Used uniformly in `winGame`, `instruct (that player) loseGame`,
    `would you loseGame`, `cant you loseGame`, `cant (each isOpponent) winGame`.
- **`flipCoins`:**
  ```lean
  semantic_template flipCoins (count : Amount := any)
      (watch : Option FlipWatch := none)
      (caller : Option NounPhrase := none) : Action :=
    .flipCoins (.count count) watch caller
  ```
  - Imperative: `flipCoins 5`
  - Replacement: `would you (flipCoins 1)`
  - Trigger (*"Whenever you win a coin flip"*):
    `whenever (does you (flipCoins (watch := .wins)))`

---

### 5. Stage 3: Composable Nominals, Predicates, and Top-Level Combinators (`lean-v3-authoring-3-nominals-and-predicates`)

#### 5.1 List and Wrapper Coercions in the Rust DSL vs. Explicit Top-Level Templates in Lean
- **In the Rust `v3` DSL (`deckmaste_dsl_v3` / `plugins_v3`):**
  - List literals coerce by default in each domain slot at the syntax/serde layer:
    - `[p1, p2, ...] : Predicate` → `Predicate::And(vec![p1, p2, ...])`
    - `[a1, a2, ...] : Action` → `Action::Sequentially(vec![a1, a2, ...])`
    - `[c1, c2, ...] : Cost` → `Cost::Compound(vec![c1, c2, ...])`
    - `[s1, s2, ...] : StaticSpec` → `StaticSpec::Conjunction(None, vec![s1, s2, ...])`
  - Wrapper and literal coercions (`1` → `Amount::Lit(1)`, `.x` → `Amount::Letter(X)`, `you` → `Subject::Actor(You)`, `p1p1Counter` → `CounterKindSource::Printed(p1p1Counter)`) are likewise desugared by the Rust DSL parser with zero Lean elaboration cost.
- **In Lean (`lean/Semantics/Templates.lean`, `Definitions.lean`, `Cards/*.lean`):**
  - To avoid Lean typeclass resolution (`CoeHTCT` → `CoeTC` → `CoeB` → `Coe` / `OfNat`), loss of top-down expected-type propagation inside list literals, and 4-layer typeclass projection unfolding in `Authoring.inspect` and kernel `by decide`, Lean uses direct top-level `semantic_template`s (`and [...]`, `or [...]`, `not ...`, `sequentially [...]`, `simultaneously [...]`, `compound [...]`, `conjunction [...]`).

#### 5.2 Uniform Boolean-Like (`is*` / `has*` / Participial / Prepositional) Naming for All `Predicate`s
- **Adopt boolean-like naming across all `Predicate` constructors and templates:**
  - **Types, kinds, and roles (`is*`):**
    - `isCreature`, `isArtifact`, `isEnchantment`, `isLand`, `isInstant`, `isSorcery`, `isPlaneswalker`, `isBattle`, `isKindred`, `isInstantOrSorcery`, `isPermanentType`
    - `isPermanent`, `isPermanentCard`, `isSpell`, `isCard`, `isToken`, `isNontoken`, `isCopyOfACard`, `isSource`, `isEmblem`, `isManaAbility`
    - `isPlayer` (replacing `anyPlayer`), `isChosenPlayer`, `isOtherPlayer`, `isOutlaw`, `isHistoric`, `isAnyTarget`, `isAnyOtherTarget`
    - **Relational opponent and teammate predicates (`[CR#102.2..102.3]`):**
      - Replace nullary `Predicate.opponent` in the Core AST with relational `Predicate.isOpponentOf (who : NounPhrase)` and `Predicate.isTeammateOf (who : NounPhrase)`.
      - `isOpponent : Predicate := isOpponentOf you` (and inside actor-relative `Definition`s such as `fateseal` `[CR#701.29a]`, `isOpponentOf .actor`).
      - `isTeammate : Predicate := isTeammateOf you`.
      - Retire `PlayerGroupWord` (`allPlayers | yourOpponents | yourTeam`) and `NounPhrase.playerGroup` from the Core AST: `yourOpponents` is `allOf isOpponent`, `allPlayers` is `allOf isPlayer`, and `yourTeam` is `or [you, a isTeammate]`.
  - **States and characteristics (`is*`):**
    - `isTapped`, `isUntapped`, `isFaceDown`, `isFaceUp`, `isPhasedOut`, `isTransformed`, `isAttached`
    - `isAttacking`, `isBlocking`, `isBlocked`, `isUnblocked`
    - `isColorless`, `isMonocolored`, `isMulticolored`
    - `isOther` (for `"another ..."` anchored on a prior mention)
  - **Properties, possessors, locations, and relations (`has*` / participial / prepositional):**
    - **Possessors (`Predicate.hasPossessor`):**
      - `controlledBy (who : NounPhrase) : Predicate := .hasPossessor .controller who`
      - `youControl : Predicate := controlledBy you`
      - `youDontControl : Predicate := not (controlledBy you)`
      - `opponentsControl : Predicate := controlledBy yourOpponents`
      - `ownedBy (who : NounPhrase) : Predicate := .hasPossessor .owner who`
      - `youOwn : Predicate := ownedBy you`
      - `opponentsOwn : Predicate := ownedBy yourOpponents`
    - **Locations (via unified `Location` from §3):**
      - `in_ (loc : Location) : Predicate := .in_ loc`
      - Examples: `and [isCard, in_ yourGraveyard]`, `and [isCreature, isCard, in_ yourHand]`, `and [isCard, in_ (exiledWith thisArtifact)]`.
    - **Identity and Negation:**
      - `otherThan (np : NounPhrase) : Predicate := .otherThan np`
      - `non (p : Predicate) : Predicate := .not p` (e.g., `non isLand`, `non isCreature`, `non isToken`)
    - **Characteristics and Stat Comparisons:**
      - `hasKeyword (kw : KeywordTerm) : Predicate := .hasKeyword kw`
      - `hasCounter (kind : CounterKindSource) (bound : CountBound := .atLeast (.lit 1)) : Predicate := .hasCounters kind bound`
      - `named (name : NameSource) : Predicate := .named name`
      - `hasType (t : CardType) : Predicate`, `hasSubtype (s : Subtype) : Predicate`, `hasSupertype (s : Supertype) : Predicate`, `hasColor (c : ColorTerm) : Predicate`
      - `hasPower (cmp : Comparator) (amt : Amount) : Predicate := .compare [.stat .power] cmp amt`
      - `hasToughness (cmp : Comparator) (amt : Amount) : Predicate := .compare [.stat .toughness] cmp amt`
      - `hasManaValue (cmp : Comparator) (amt : Amount) : Predicate := .compare [.stat .manaValue] cmp amt`
- **Delete ad-hoc compound predicate macros:**
  - Retire `creatureYouControl`, `creatureYouDontControl`, `creatureYourOpponentsControl`, `outlawYouControl`, `armyYouControl`, `otherCreature`, and `otherCreatureYouControl`.
- **Unlocks bare nouns and statuses for non-predicate positions:**
  - Because `isCreature`, `isCard`, `isPermanent`, `isPlayer`, `isSpell`, and `isTapped` carry the `is` prefix:
    - `spell (body : Action) (sub : Option Subtype := none) : Ability := .spell sub body` keeps the natural name `spell` without colliding with `isSpell : Predicate` (e.g., `spell (counter (target isSpell))`).
    - `creature`, `artifact`, `enchantment`, `land`, `planeswalker`, `battle`, `card`, `permanent`, `player`, `token`, `copy` are free for `NounWord` / `CardType` (`that creature`, `that card`, `those cards`, `enchanted creature`, `equipped creature`, `fortified land`).
    - `tapped`, `untapped`, `faceDown`, `faceUp`, `phasedIn`, `phasedOut` are free for `Status` (`become tapped`, `become untapped`, `entersAs tapped`).

#### 5.3 Attached Hosts (`enchanted`, `equipped`, `fortified`) & Quantified `targets`
- **Attached Hosts (`NounPhrase.attachHost` — 126 occurrences in `Cards/`):**
  ```lean
  semantic_template enchanted (w : NounWord) : NounPhrase := .attachHost .enchanted w
  semantic_template equipped (w : NounWord) : NounPhrase := .attachHost .equipped w
  semantic_template fortified (w : NounWord) : NounPhrase := .attachHost .fortified w
  ```
  - Examples: `enchanted creature`, `enchanted permanent`, `enchanted player`, `equipped creature`, `fortified land`.
- **Quantified `targets` (`DetPhrase.target` — 35 occurrences in `Cards/`):**
  ```lean
  semantic_template targets (q : Quantity) (p : Predicate) : NounPhrase :=
    .described (.target q) p
  ```
  - Examples: `targets (upTo 2) isCreature`, `targets (oneThrough 3) isAnyTarget`, `targets anyNumber isAnyTarget`.

#### 5.4 Top-Level Templates Eliminating All Remaining `Primitives.*` Qualifications
- **Expose top-level `semantic_template`s for all remaining `Primitives.*` constructors used in `Cards/` and `Definitions.lean`:**
  - **Abilities:** `static (spec : StaticSpec) : Ability := .static spec`, `spell (body : Action) (timing : Option Timing := none) : Ability := .spell timing body`.
  - **Nouns & Subjects:** `you`, `this`, `theActor` (`NounPhrase.actor`), `yourOpponents`, `eachPlayer`, `eachOpponent`, `actor (np : NounPhrase) : Subject`, `by_ (np : NounPhrase) : Subject`, `any : Subject / NounPhrase / Amount / Location`.
  - **Combinators:** `and`, `or`, `not` (`Predicate` / `Condition` / `NounPhrase` / `Location` — dispatched via single-method typeclasses or top-down expected-type resolution rather than `export` overloads so nested combinators elaborate in $O(1)$ per node without bottom-up $3^d$ branching), `sequentially`, `simultaneously`, `compound`, `conjunction`.
  - **Deltas & Amounts:** `up`, `down`, `setTo`, `letter (l : Letter) : Amount`, `thatMuch : Amount`, `anyAmount : Amount`, `Amount.moreThan (bound : Amount)` (matching `Amount.upTo`, for *"can't draw more than one card each turn"* on *Spirit of the Labyrinth*).
  - **Costs, Payments & Ballots:** `mana`, `tapSymbol`, `untapSymbol`, `loyaltySymbol`, `perform`, `itsManaCost`, `anyCost`, `itsOwnCost : PlayPayment`, `withoutPaying : PlayPayment`, `payingInstead (cost : Cost) : PlayPayment`, `anyBallot : Ballot`.
  - **Continuous & Static Specs:** `establish`, `abilityGrant`, `qualityChange`, `ptDefinition`, `letterDefinition`, `instead`, `damageRule`, `costShift`, `altCost`, `addedCost`, `visibility`, `controlGrant`, `copyChange`.
  - **Conditions, Timings & Durations:** `matches_ (np : NounPhrase) (p : Predicate) : Condition`, `compareAmt`, `forAsLongAs`, `duringYourUntapStep`, `onEachOfTheirTurns`.

---

### 6. Stage 4: Consolidating `Templates` and `Definitions` and Retiring `v1`/`v2` Holdover Names (`lean-v3-authoring-4-declared-bodies`)

#### 6.1 Module Consolidation (`Primitives.lean`, `Templates.lean`, `Definitions.lean`)
- Retire `lean/Semantics/Macros.lean`, `lean/Semantics/Macros/Primitives.lean`, `lean/Semantics/Macros/Declared.lean`, `lean/Semantics/Macros/CharacteristicInputs.lean`, and `lean/Semantics/Macros/Checks.lean`, replacing them with:
  - **`lean/Semantics/Primitives.lean`**: `Semantics.Primitives` namespace for `declare_semantic_primitives` + `CharacteristicInputs`.
  - **`lean/Semantics/Templates.lean`**: Single authoritative module of `semantic_template` phrasing helpers.
  - **`lean/Semantics/Definitions.lean`**: Single authoritative module of `semantic_definition` rules-defined vocabulary (`[CR#701.1]` keyword actions, `[CR#702.1]` keyword abilities, and counters/tokens/designations), with bodies expressed through `Templates.lean` helpers instead of single-line raw constructor dumps.
  - **`lean/Semantics/Proofs/DefinitionsAndTemplates.lean`**: Replaces `Macros/Checks.lean`.

#### 6.2 Reconciling the `Macros.lean` vs. `Declared.lean` Signature and Body Drifts
- **Generalize `Nat`/`String` parameters to `Amount`/`Subtype`:**
  - `amass (subtype : Subtype) (count : Amount)` (`[CR#701.47a]`)
  - `bushido (count : Amount)` (`[CR#702.45a]`), `modular (count : Amount)` (`[CR#702.43a]`), `renown (count : Amount)` (`[CR#702.112a]`)
  - `forEach (per : Amount) (p : Predicate) : Amount`
- **Rules-accurate signatures and bodies:**
  - `fateseal (amount : Amount)` (`[CR#701.29a]` — looks at the top `amount` cards of an opponent's library (`a (isOpponentOf .actor)`), retiring the extra `whose` parameter in `Macros.lean`).
  - `companion (condition : DeckCondition)` (`[CR#702.139a]` — uses `DeckCondition` from `Macros.lean`, not `Condition` from `Declared.lean`).
  - `levelUp (cost : Cost)` (`[CR#702.87a]` — uses the full sorcery-speed activated-ability expansion body from `Declared.lean`, not the empty-body stub in `Macros.lean`).
  - `gainControl (subject : NounPhrase) (duration : Option Duration := none) (controller : NounPhrase := .actor)` — defaults `controller` to `.actor` so both `"gain control of target creature"` (`you`) and `"target player gains control of..."` (`instruct`) work uniformly.
  - `creatureToken` / `creatureTokenOf` — include `(abilities : List Ability := [])` from `Declared.lean`.
  - `create (count : Amount) (token : TokenSpec) (riders : List MoveRider := [])` — unified `Definition` (`[CR#701.7]`), with convenience templates `createToken`, `createTappedToken`, `createTappedAttacking`, `createTokenCopy`.
  - `lookAndSortSlice` — single helper backing `scry`, `surveil`, and `fateseal`, retiring `lookAndSort`, `lookAndSortInto`, and `lookedCards`.

#### 6.3 Retiring `v1`/`v2` Holdover Names & Amending the Glossary and `lean/CONTRACTS.md`
- **`Authoring.lean` & `CallerScope.lean`:**
  - Rename `lean/Semantics/MacroParameters.lean` (`MacroExpansion`, `MacroSplice`) and `lean/Semantics/Proofs/MacroParameters.lean` → `lean/Semantics/CallerScope.lean` (`CallerScope`, `CallerSplice`) and `lean/Semantics/Proofs/CallerScope.lean`.
  - In `lean/Semantics/Authoring.lean`, retire raw `semantic_macro` and `macroAttribute` so only `semantic_definition` and `semantic_template` exist; rename `MacroContext` → `AuthoringContext`, `contextualMacroAttribute` → `contextualAuthoringAttribute`, `macro_splice_type%` → `caller_splice_type%`, and `Form.onlyMacros` / `Forms.onlyMacros` → `Form.onlyAuthored` / `Forms.onlyAuthored`.
- **Retire `v2` shims and holdover identifiers:**
  - Delete empty 12-line shims `lean/Semantics/Phrase.lean` and `lean/Semantics/Triggers.lean`; fold the 1-function `Check/Triggers.lean`, `Check/EventContext.lean`, and `Check/PhraseRules.lean` micro-files into `Check/Phrase.lean`.
  - Delete `abbrev Binding := Referent` and `abbrev Bindings := DRS` in `lean/Semantics/Check/Words.lean`.
  - Rename `Payload.gap`, `Referent.isGap`, `gapB`, `countGaps`, and `Refusal.gapInScope` → `Payload.difference`, `Referent.isDifference`, `differenceB`, `countDifferences`, and `Refusal.differenceInScope`.
- **Amend Game Model Glossary (`docs/contexts/game-model/CONTEXT.md`) & `lean/CONTRACTS.md`:**
  - Replace the legacy **Semantic Macro** entry with **Definition** and **Template** (`[CR#701.1,702.1]`), listing `_Avoid_: Semantic Macro, macro`.
  - Rename **Instruction** to **Action** in `docs/contexts/game-model/CONTEXT.md` (with `_Avoid_: Instruction`) and sweep remaining `instruction` parameter/doc uses and stale pre-DRT (`Bindings`, `Window`, `capture`) references in `lean/CONTRACTS.md`.

---

## Proof

- All four sub-tickets are integrated in `docs/tickets/done/`.
- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
