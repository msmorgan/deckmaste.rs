---
needs:
  - lean-v3-authoring-1-deontic-cutover
  - lean-v3-authoring-2-verb-and-clause-spine
  - lean-v3-authoring-3-nominals-and-predicates
  - lean-v3-authoring-4-declared-bodies
design: true
---
# Refactor the Lean v3 authoring surface before mirroring into Rust and plugins_v3

**[design] Refactor the `lean/Semantics/` authoring surface so that authored cards and
keyword definitions read like Oracle English while expanding into the minimal
Core AST primitives, and retire all `v1`/`v2` module, macro, and alias
holdovers before `semantics-v3-dsl-and-plugins` mirrors the workbench into
`crates/deckmaste_semantics_v3` and `plugins_v3/`.** Umbrella epic for:
[1 deontic cutover](lean-v3-authoring-1-deontic-cutover.md),
[2 verb and clause spine](lean-v3-authoring-2-verb-and-clause-spine.md),
[3 nominals and predicates](lean-v3-authoring-3-nominals-and-predicates.md), and
[4 declared bodies and legacy name retirement](lean-v3-authoring-4-declared-bodies.md).
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
   - Bare numeral coercion (`1` vs `.lit 1`) and leading `.` enum prefixes are
     handled by `deckmaste_dsl_v3` in `semantics-v3-dsl-and-plugins`; Lean
     retains explicit `.lit` and `.` constructors where Lean's elaborator
     requires them.
4. **No `v1`/`v2` Holdover Names:**
   - Retire the `Semantics.Macros` hierarchy, `Declared.lean` split,
     `semantic_macro` command, `MacroParameters` / `MacroContext` / `MacroSplice`
     identifiers, empty `Phrase.lean` / `Triggers.lean` shims, `Binding` /
     `Bindings` aliases, and the `Semantic Macro` glossary entry in favor of
     `Primitives`, `Templates`, `Definitions`, `CallerScope`, `AuthoringContext`,
     `Referent`, and `DRS`.

## Sub-Tickets

1. [`lean-v3-authoring-1-deontic-cutover`](../wip/lean-v3-authoring-1-deontic-cutover.md):
   Retire the `deonticActionFor` `v2`-port shim and all `Deed`-taking deontic
   signatures in favor of `v3` `Action`-taking templates (`cant`, `can`, `must`,
   `gatedBy`).
2. [`lean-v3-authoring-2-verb-and-clause-spine`](../wip/lean-v3-authoring-2-verb-and-clause-spine.md):
   Introduce `Action` templates for verbs and observations (`die`, `enter`,
   `leave`, `putInto`, `phaseOut`, `phaseIn`, `turnFaceUp`, `loseGame`,
   `winGame`, `cast`, `attack`, `block`, `dealDamage`, etc.) so `does`,
   `would`, `did`, and `cant` share one verb vocabulary without raw 6-argument
   `Action.move` or `.setStatus` / `.procedure` / `.conclude` calls.
3. [`lean-v3-authoring-3-nominals-and-predicates`](lean-v3-authoring-3-nominals-and-predicates.md):
   Replace hardcoded compound noun macros and raw `Primitives.*` constructor
   chains in `Cards/*.lean` with composable noun-phrase, determiner, and
   predicate modifiers (`youControl`, `controlledBy`, `opponentsControl`,
   `youOwn`, `ownedBy`, `in_`, `withKeyword`, `otherThan`, `which`, etc.) and
   clean top-level combinator templates.
4. [`lean-v3-authoring-4-declared-bodies`](lean-v3-authoring-4-declared-bodies.md):
   Consolidate `Macros.lean` and `Macros/Declared.lean` into
   `lean/Semantics/Primitives.lean`, `lean/Semantics/Templates.lean`, and
   `lean/Semantics/Definitions.lean` (with `Definition` bodies re-expressed
   through `Template` helpers), retire all remaining `v1`/`v2` holdover module
   and identifier names, and amend `docs/contexts/game-model/CONTEXT.md`.

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

#### 2.4 Quantifiers on `Quantity` (Retiring `DeonticMods.bound`)
- Delete `DeonticMods.bound` (`Option CountBound`).
- Add `.moreThanOf (amount : Amount)` (`moreThan n`) and
  `.additional (quantity : Quantity)` (`additional q`) to `Quantity` so noun
  phrases carry their own quantifiers directly:
  - *"Each player can't cast more than one spell each turn"* (*Arcane Laboratory*):
    `cant (each anyPlayer) (cast (counted (moreThan 1) spell)) (timing := eachTurn)`
  - *"~ can't be blocked by more than one creature"* (*Alpha Authority*):
    `cant (by_ (counted (moreThan 1) creature)) (block enchantedCreature)`
  - *"You may play an additional land on each of your turns"* (*Exploration*):
    `can you (play (counted (additional (exactly 1)) land)) (timing := onEachOfYourTurns)`
  - *"~ can block an additional creature each combat"* (*Hundred-Handed One*):
    `can thisCreature (block (counted (additional (exactly 1)) creature)) (timing := eachCombat)`
  - *"You may vote an additional time"* (*Brago's Representative*):
    `can you (vote (times := additional (exactly 1)))`
- Replace `allPlayers` in active subjects where Oracle English says *"Each
  player"* (`each anyPlayer`) or *"Players"* (`bare anyPlayer`), and replace
  `cast (allOf spell)` with `cast (a spell)` / `cast (bare spell)`.

#### 2.5 `AsThough` Carries a `Condition` (Preserving Pronouns & Fixing Polarity)
- Replace `AsThough.of (predicate : Predicate)` with
  `AsThough.condition (cond : Condition)` (with coercion from `Condition` to
  `AsThough`) so the subject pronoun in *"as though **it** didn't have
  defender"*, *"as though **they** didn't have hexproof"*, and *"as though
  **they** had flash"* is preserved and checked by DRT (`D′`):
  - *Warmonger's Chariot*:
    `can thisCreature attack (asThough := matches it (not (hasKeyword defender)))`
  - *Glaring Spotlight*:
    `can (by_ (or [spellYouControl, abilityYouControl])) (target (allOf (opponentsControl creature))) (asThough := matches them (not (hasKeyword hexproof)))`
  - *Teferi, Mage of Zhalfir*:
    `can you (cast (a (and [creature, isCard]))) (asThough := matches them (hasKeyword flash))`
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
  - `whenever (does you (cast (a spell)))`
  - `whenever (does any (block thisCreature))` (*"becomes blocked"*)
  - `whenever (does (by_ (a spell)) (target thisCreature))` (*"becomes the target of a spell"*)

#### 4.2 Intransitive (`.actor`) vs. Transitive (`subject`) Zone Transitions
- `Action.move` remains the single Core AST zone-transition primitive.
- **Transitive move definitions/templates** (`putInto X loc`, `exile X`,
  `destroy X`, `sacrifice X`, `discard X`) take `X : NounPhrase` as their
  patient argument, leaving the enclosing clause's `Subject` (`you`,
  `target opponent`, `any`) as the performing agent:
  - *"Target opponent sacrifices a creature"*:
    `instruct (target opponent) (sacrifice (a creature))`
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
    `instead (would (a (opponentsControl creature)) die) ...`
  - *"a creature that died this turn"*:
    `a (and [creature, did which die thisTurn])`
  - *"Whenever a land you control enters from anywhere other than your hand"*:
    `whenever (does (a (youControl land)) (enter (from_ := not yourHand)))`
  - *"Creatures can't enter from graveyards"*:
    `cant (a creature) (enter (from_ := graveyard))`

#### 4.3 Status Changes (`setStatus`): Intransitive vs. Transitive
- **`phaseOut` and `phaseIn` are intransitive (`[CR#702.26b]`):**
  ```lean
  semantic_template phaseOut : Action := .setStatus .phasedOut .actor
  semantic_template phaseIn : Action := .setStatus .phasedIn .actor
  ```
  - *"Target creature phases out"*: `instruct (target creature) phaseOut`
  - *"Whenever ~ phases out"*: `whenever (does thisCreature phaseOut)`
- **`become (status : Status)` (`becomeTapped`, `becomeUntapped`) is intransitive (`[CR#603.2e]`):**
  ```lean
  semantic_template become (status : Status) : Action := .setStatus status .actor
  ```
  - *"Whenever a permanent becomes untapped"*:
    `whenever (does (a permanent) (become .untapped))`
  - Distinct from the transitive keyword actions `tap (subject : NounPhrase)`
    and `untap (subject : NounPhrase)` (`[CR#701.26a..701.26b]`), which wrap
    `.enact (.action "tap" / "untap")`.
- **`turnFaceUp` and `turnFaceDown` are transitive:**
  ```lean
  semantic_template turnFaceUp (subject : NounPhrase) : Action :=
    .setStatus .faceUp subject
  ```
  - *"Whenever a permanent you control is turned face up"*:
    `whenever (does any (turnFaceUp (a (youControl permanent))))`

#### 4.4 `dealDamage`, `winGame` / `loseGame`, `flipCoins`, and `rollDice` with Optional Kwargs
- **`dealDamage`:**
  ```lean
  semantic_template dealDamage (amount : Amount := any)
      (to : NounPhrase := any)
      (kind : DamageKind := .any) : Action :=
    .dealDamage amount to kind
  ```
  - Imperative: `instruct thisCreature (dealDamage 3 (to := target anyTarget))`
  - Active trigger: `whenever (does thisCreature (dealDamage (to := a anyPlayer) (kind := .combatOnly)))`
  - Passive trigger: `whenever (does any (dealDamage (to := thisCreature)))`
- **`winGame` and `loseGame` (`Action.conclude`):**
  ```lean
  semantic_template winGame : Action := .conclude .winGame
  semantic_template loseGame : Action := .conclude .loseGame
  ```
  - Used uniformly in `winGame`, `instruct (that .player) loseGame`,
    `would you loseGame`, `cant you loseGame`, `cant (each opponent) winGame`.
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

## Proof

- All four sub-tickets are integrated in `docs/tickets/done/`.
- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
