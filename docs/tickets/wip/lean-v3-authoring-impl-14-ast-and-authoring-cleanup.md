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

### 2. Reconcile Overlapping AST & Vocabulary Constructs (Opus §§4.3–4.6, §4.13–4.14)

- **`Amount` vs `Quantity` (§4.3):** Reconcile `Amount.upTo`/`moreThan`
  (`Abilities.lean:383-387`) vs `Quantity.upToOf`/`moreThanOf` (`:394,399`) to
  honor the Game Model glossary `_Avoid_` lines on `Amount` and `Quantity`.
- **`HeadNoun` vs `NounWord` & Object-Class Split (§4.4):** Reconcile the shared
  `card`/`token`/`spell`/`permanent`/`ability`/`type` heads across `HeadNoun`
  (`Words.lean:479-482`) and `NounWord` (`Words.lean:283-302`), the 3-way
  object-class split (`Predicate.isCard`/`isToken`/`isEmblem`, `isPermanent`,
  `isSpell`), and subtype-headed self-references (`thisAura`, `thisEquipment`,
  `thisRoom`, `thisSiege`).
- **Per-property up/down vocabularies vs `Delta` (§4.5):** Align `CostShift`,
  `ShiftDir`, `LoyaltyCost`, `LifeMove`, and `CounterMove` with the glossary's
  `Delta` rule (`_Avoid_` per-property up/down enums).
- **"One counter of each kind it already has" (§4.6):** Unify the three
  encodings across `proliferate` (`CounterKindSource.own`), `doubleCounters`
  (`doForEachKind ... .bound`), and `moveCounters` (`.sameAs`), plus
  `proliferate`'s permanent vs player counter test.
- **Remaining redundant AST constructors & one-card slots (§4.13, §4.14):**
  - Retire `Action.doOnlyIf` constructor (`Abilities.lean:1059`), keeping
    `doOnlyIf` as a `Templates.lean` helper expanding to `.doIf` (and align
    *Galvanic Blast*, §4.7).
  - Retire `Condition.noHolder` (`Abilities.lean:428`) in favor of a template
    over `not (exists_ (hasDesignation ...))`.
  - Retire `GameEvent.statBecomes` (`Abilities.lean:459`, `[CR#603.8]`) in favor
    of `GameEvent.stateHolds`.
  - Fold `Predicate.isManaAbility` (`Abilities.lean:262`, `[CR#605.1]`) into
    `Predicate.abilityHead`.
  - Reconcile `NounPhrase.resolvedPermanent` vs `NounPhrase.becomes`
    (`Abilities.lean:301,304`, `[CR#400.7,608.3a]`).
  - Replace `NounPhrase.theGame` (`Abilities.lean:309`, §4.14 — refused
    everywhere except `.matches .theGame (.hasDesignation ...)` and
    `.gainDesignation .theGame ...`, violating the glossary's Reference = Object
    or Player) with dedicated game-designation forms or a clean non-`NounPhrase`
    representation.
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
