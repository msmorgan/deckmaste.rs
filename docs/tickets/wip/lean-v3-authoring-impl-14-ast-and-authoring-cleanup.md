---
needs: [lean-v3-authoring-impl-13-replacement-also]
---
# Finish Core AST & authoring unification (`CharacteristicEdit`, vocabulary overlaps, layering, escape hatches)

**Complete the remaining Core AST, `Templates`, `Definitions`, and `Cards`
unifications identified in [`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md)
(§§4.1–4.14, §5.4).** Fourteenth implementation step of
[`lean-v3-authoring-surface`](lean-v3-authoring-surface.md). Standard
constraints apply.

## Scope

### 1. Finish `CharacteristicEdit` Unification (Opus §4.1)

- Fold `StaticSpec.ptDefinition` and `StaticSpec.ptSwitch` (`[CR#613.4a,613.4d]`)
  into `CharacteristicEdit` under `StaticSpec.characteristicChange`. A defined
  power or toughness is a `stat .sets` edit; whether it is a
  characteristic-defining ability (layer 7a) is read from where it stands (the
  object's own printed, unconditional static ability setting its own stat,
  `[CR#604.3a]`), as a self "is all colors" already was, not stored. The
  switch is `CharacteristicEdit.ptSwitch`. `ptDefinition`/`ptSwitch` remain as
  templates, `DefinedSlots` as `ptDefinition`'s input only.
- Collapse the triple encoding of `"every type of"`, `"chosen quality"`, and
  color setting across `TokenQuality`, `QualityPayload`, and `CharacteristicEdit`,
  and deduplicate `retained` between `TypeLineChanges` and `QualityPayload.bundle`.
  `TokenQuality` becomes `WrittenQuality` (`everyTypeOf`, `chosen`), the one
  encoding a token's bundle and `CharacteristicEdit.quality op q` share;
  `QualityPayload` is `bundle | quality | colored`, the op-less inputs of the
  edits it expands to. `TypeLineChanges.retained` is retired: "It's still a
  land" keeps every prior type `[CR#205.1b]`, so a retained bundle's type line
  is an `adds`, whatever op writes the rest; the named type is wording only.
- Fix `Characteristics.valueEdits` (`lean/Semantics/Primitives.lean:16-56`) where
  `name` gets raw `op`, other fields get `valueOp`, and `manaCost` ignores `op`
  (causing a `.loses` bundle with a cost to emit a mana-cost set). Every
  written value takes the value op (an addition sets it, a removal removes it);
  a removed cost is `manaCost none`.

### 1a. Retire the `unless` Marking (owner addition)

- "Unless C" is "if not C"; whether a negated condition is said with "unless"
  is a realisation choice read from the condition, not stored. Retire
  `CondMarking.unless_` and `markingOk` (`Refusal.markingOk`) with their pins.
  `onlyUnless spec c` stays as the template for the Oracle word order, sugar
  for `.conditional spec (.not c) .ifSo`, the same expansion `onlyIfSo spec
  (not c)` gives, so card sources read as printed. `asLongAs` vs `ifSo` stays:
  they are different conditions.

### 1b. "Enters … unless" on the Replacement (owner addition)

- A condition checked as a permanent would enter belongs to the replacement,
  not the static ability: "this land enters tapped unless C" (*Turbulent Fen*
  and its kin) is `entersTappedUnless subject c`, the template over
  `entersUnless subject riders c` =
  `replace (would subject enter) (doIf (not c) (withRiders riders))`
  `[CR#614.1d,614.12]`, replacing `onlyUnless (entersTapped …) c`. The bench
  has no other "enters … if/unless" conditioned at the static level. The
  Saga rule's read-ahead test stays on the static ability (it decides whether
  the ability exists, `[CR#714.3a]`).
- The stack keywords whose replacement is printed "if C, … instead" condition
  the replacement the same way, `replace … (doIf c (instead …))`: aftermath,
  buyback, flashback, harmonize, rebound `[CR#702.127a,702.27a,702.34a,702.180a,702.88a]`
  and mutate's merge `[CR#702.140c]`. Dredge's "as long as you have at least N
  cards in your library" `[CR#702.52a]` is an as-long-as condition on the
  static ability and stays there.

### 2. Reconcile Overlapping AST & Vocabulary Constructs (Opus §§4.3–4.6, §4.13–4.14)

- **`Amount` vs `Quantity` (§4.3):** Reconcile `Amount.upTo`/`moreThan`
  (`Abilities.lean:383-387`) vs `Quantity.upToOf`/`moreThanOf` (`:394,399`) to
  honor the Game Model glossary `_Avoid_` lines on `Amount` and `Quantity`.
  "Up to N" and "more than N" are count constraints, so `Amount.upTo` and
  `Amount.moreThan` are retired and their one host, `Action.draw`, takes a
  `Quantity` (`draw n` builds `exactlyOf n`; `drawCount (upToOf n)`, `drawCount
  (moreThan 1)`); "up to N" still publishes the ceiling shortfall. `anyAmount`
  is a pattern wildcard, not a constraint, and stays.
- **`HeadNoun` vs `NounWord` & Object-Class Split (§4.4):** Reconcile the shared
  `card`/`token`/`spell`/`permanent`/`ability`/`type` heads across `HeadNoun`
  (`Words.lean:479-482`) and `NounWord` (`Words.lean:283-302`), the 3-way
  object-class split (`Predicate.isCard`/`isToken`/`isEmblem`, `isPermanent`,
  `isSpell`), and subtype-headed self-references (`thisAura`, `thisEquipment`,
  `thisRoom`, `thisSiege`).
- **Per-property up/down vocabularies vs `Delta` (§4.5):** Align `CostShift`,
  `ShiftDir`, `LoyaltyCost`, `LifeMove`, and `CounterMove` with the glossary's
  `Delta` rule (`_Avoid_` per-property up/down enums). `CostShift` is
  `generic (Delta Amount) floor | bySymbols (Delta ManaCostTerm) coloredOnly`
  (templates `less`, `more`, `lessBySymbols`, `moreBySymbols`);
  `DamageScale.shifted`, `Cost.loyaltySymbol` (`[0]` is `up 0`, `[−X]` is
  `down X`) and `Action.shiftResult` (a list of changes, the performer's
  choice: "increase or decrease") take `Delta Amount`. A shift never sets
  (`Refusal.shiftSets`, `[CR#118.9,606.4]`). `ShiftDir`, `LoyaltyCost` and the
  dead `LifeMove` are retired; `CounterMove` only picked between two templates,
  now `countersPut`/`countersRemoved` and their bare forms. `QualityOp` stays:
  qualities are not numeric properties.
- **"One counter of each kind it already has" (§4.6):** Unify the three
  encodings across `proliferate` (`CounterKindSource.own`), `doubleCounters`
  (`doForEachKind ... .bound`), and `moveCounters` (`.sameAs`), plus
  `proliferate`'s permanent vs player counter test. "One of each kind on X" is
  `sameAs X`, the recipient itself included: proliferate is
  `doForEach (those …) (putCounters 1 (sameAs it) it)` and `own` is retired.
  Doubling stays a kind loop: it gives as many of each kind as there are, not
  one. Moving put `sameAs source` (one of *each* kind on the source); it now puts
  `those`, the kind removed this way `[CR#122.5]`. `hasCounters` and `sameAs`
  accept players `[CR#122.1]`, so proliferate's choice is one test,
  `and [or [isPermanent, isPlayer], hasCounters none]`.
- **Remaining redundant AST constructors & one-card slots (§4.13, §4.14):**
  - Retire `Action.doOnlyIf` constructor (`Abilities.lean:1059`), keeping
    `doOnlyIf` as a `Templates.lean` helper expanding to `.doIf` (and align
    *Galvanic Blast*, §4.7). Where the condition is printed is wording only:
    `doIf` announces its branch's targets before its condition (they are chosen
    on casting, `[CR#601.2c]`), so either order reads them, and the branch's
    output continues as `doIf`'s always did. *Galvanic Blast* already wrote
    `doIf`.
  - Retire `Condition.noHolder` (`Abilities.lean:428`) in favor of a template
    over `not (exists_ (hasDesignation ...))`. The template is `thereIsNo`
    (the Oracle wording); the duplicate `noHolder` template goes with the
    constructor.
  - Retire `GameEvent.statBecomes` (`Abilities.lean:459`, `[CR#603.8]`) in favor
    of `GameEvent.stateHolds`: "its power becomes 20" is
    `stateHolds (compareAmt (powerOf it) .eq (.lit 20))`.
  - Fold `Predicate.isManaAbility` (`Abilities.lean:262`, `[CR#605.1]`) into
    `Predicate.abilityHead`: `AbilityClass.mana`, with `isManaAbility` the
    template.
  - Reconcile `NounPhrase.resolvedPermanent` vs `NounPhrase.becomes`
    (`Abilities.lean:301,304`, `[CR#400.7,608.3a]`). One constructor,
    `becomes` ("the permanent this spell becomes"), with `resolvedPermanent`'s
    reading: a type view refining the spell's referent in place (ADR §1.4),
    since effects on the spell continue to the permanent `[CR#400.7a]`.
  - Replace `NounPhrase.theGame` (`Abilities.lean:309`, §4.14 — refused
    everywhere except `.matches .theGame (.hasDesignation ...)` and
    `.gainDesignation .theGame ...`, violating the glossary's Reference = Object
    or Player) with dedicated game-designation forms or a clean non-`NounPhrase`
    representation: `Condition.gameHas d` ("it's day") and `Action.gameGains d`
    ("it becomes night"), the `gameIs`/`setGameDesignation`/`gameBecomes`
    templates over them; `Refusal.gameReference` goes, the misuse it caught
    being unrepresentable.
  - Audit and clean up one-card `Ability` constructors (`mayBeginOnBattlefield`,
    `alsoForKeywords`, `thatAbility`), `Predicate.other` vs `otherThan`,
    duplicated qualifier slots (`Ability.activated` vs `Ability.qualified` vs
    `triggered` limit vs `Action.limited`), `Plurality`/`CounterBatch`/`DiceBatch`,
    and orphan `inCaller` constructors on `TokenSpec`/`MoveRider` (§4.13).

### 3. Authoring, Label Discipline, Layering & Dead Helpers (Opus §§4.2, 4.8–4.12, 5.4)

- **Fold remaining raw constructors & fix `inCaller` / `firstMention` discipline
  (§§4.2, 4.10):**
  - Fold remaining raw constructors in `Definitions/Actions.lean` and
    `Definitions/Keywords.lean` (`monstrosity`, `renown`, `levelUp`, `modular`,
    `regenerationApplication`, and the 72 raw `(.actor ...)` calls in
    `Keywords.lean`).
  - Normalize `inCaller`/`firstMention` nesting order and parameter tag names
    (`destroy.subject` vs `fight.0` vs `goad.arg0`), enable `firstMention` on
    `cast`/`play`/`activate`/`regeneration`, and add missing `inCaller` wrappers
    on `search` (`zone`), `dredge` (`amount`), `vote` (`ballot`), and
    `Ability.inCaller` for `boast`/`exhaust`/`solved`.
  - Align `.actor` vs `you` across `searchLibraryFor`/`searchLibraryOrGraveyard`
    and `mill`/`scry`/`surveil`.
- **Keyword variant labelling & `Definition`/`Template` boundary (§§4.8, 4.9):**
  - Establish and apply a single rule for keyword-variant labels (`emergeFrom`
    `[CR#702.119b]`, `bandsWithOther` `[CR#702.22b]`, `hexproofFrom`,
    `typecycling`, `devourQuality`, `equipQuality`, and `devour` self-nesting).
  - Reconcile the `Definition` vs `Template` classification for the 57 ability
    words (`[CR#207.2c]`), `flip`, and `regenerationApplication`.
- **Close string escape hatches (§§1.6, 4.11):**
  - Replace `creatureType (label : String)` (`Templates.lean:376-381`) and raw
    `Subtype.of host (label : String)` call sites with typed catalog `Subtype`
    definitions; tighten `KeywordActionLabel`, `AbilityWordLabel`,
    `FlavorWordLabel`, `VoteLabel`, and card-local string helpers
    (`nonbasicLandsAre`, `enchantedBecomesVanilla`, `basicLand`, `thrivingLand`);
    resolve the `ItalicWord` (`Words.lean:405`) and `RoomHalf` (`:505`) owner
    comments.
- **Layering & dead templates/helpers (§§4.7, 4.12, 5.4):**
  - Remove `Templates.lean`'s import of `Semantics.Check` (move `Location.sort`
    below `Check` or into `Primitives`), eliminate forward references from
    `Templates` to `Definitions`, rehome `TurnPart` entries and predefined tokens
    so `Templates` doesn't use raw `.named "haste"`/`.named "ward"`/`.untapStep`/`.mainPhase`,
    and resolve collision names (`DesignationLabel.solved`, `Subtype.army`,
    `Subtype.infinity`, `splice_`).
  - Delete dead/duplicate templates and types in `Templates.lean`, `Events.lean`,
    and `Words.lean` (`act`, `Quantity.upTo`, `ManaSymbol.generic`,
    `Ability.activatedBy`, `Timing.duringPart`, `aRandom`, `command`/`commandZone`,
    `getsPt`/`ptModification`, `noHolder`/`thereIsNo`, `allAmong`/`allFromAmong`,
    `lit`, `keywordOf`, `isPermanentCard`/`isPermanentType` duplicate list,
    `itAsToken` vs `thatToken`, `CounterMove`, `LifeMove`).
  - Clean up remaining card warts: manual `printedKind` (~100 sites in
    `Cards/Counters.lean`) and `printedName`, raw `theVerbed (.action "...")`
    strings, `(.named "bloodCounter")` on *Font of Agonies*, hand-expanded
    templates (*Ersatz Gnomes*, *Crimson Wisps*, *Blatant Thievery*), `copy` vs
    `copySpell` (*Fork* vs *Twincast* vs *Strionic Resonator*), trigger template
    symmetry (`whenIf`/`wheneverIf`/`atIf`), and `matches_` → `matches` / `is`.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
