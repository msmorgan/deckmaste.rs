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
  into `CharacteristicEdit`. A defined power or toughness is a `stat .sets`
  edit, written under `StaticSpec.characteristicDefining` (§1c). The switch is
  `CharacteristicEdit.ptSwitch` under `characteristicChange`.
  `ptDefinition`/`ptSwitch` remain as templates, `DefinedSlots` as
  `ptDefinition`'s input only.
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

### 1c. Characteristic-Defining Abilities as a Form (owner addition)

- Whether a static ability is a characteristic-defining ability is a rules
  classification (all zones `[CR#604.3]`, first in its layer, 7a for power and
  toughness `[CR#613.3,613.4a]`), so it is a form, not inferred from where a
  set stands: `StaticSpec.characteristicDefining subject edits`. The subject
  stays written (it carries the printed head: "this creature's power",
  changeling's "this object"), and the checker enforces `[CR#604.3a]` where
  it is structurally visible: an edit other than a color, subtype, power or
  toughness definition (`cdaEdit`, criterion 1; a stat sets, a color or
  subtype adds or sets), a subject other than the object itself (`cdaSubject`,
  3), the form written inside a grant or an established effect (`cdaGranted`,
  2 and 4; a granted keyword whose definition is one, as changeling, is an
  ordinary grant), and a condition on it (`cdaConditional`, 5). Criterion 2
  otherwise holds by where it stands: a face, a token's definition, a copy or
  a text change.
- Templates `ptDefinition`, `ptDefinitions` (power and toughness apart, one
  ability; the toughness reads the power's number back: Tarmogoyf, Lhurgoyf,
  Consuming Blob, Shapeshifter) and `qualityDefinition`. Changeling ("This
  object is every creature type", subject `this_`, `[CR#702.73a]`), devoid
  `[CR#702.114a]`, Transguild Courier and Mistform Ultimus are written with
  it. `definedStats` reads the form.

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
  `thisRoom`, `thisSiege`). `HeadNoun` is retired: `headed` and `theGrantor`
  take a `NounWord`, which gains `emblem` and `subtype s` (read as its card
  type, `[CR#205.3]`), so "this creature" and "that creature" share one word
  and `thisAura`, `thisEquipment`, `thisRoom`, `thisSiege` (and the other
  subtype heads) carry their subtype. A word that is no Head Noun (player,
  pile, stack, join, copy) is refused as a head (`Refusal.headNoun`). The
  object-class predicates stay as they are: a permanent and a spell are
  defined by their zone (`isPermanent`, `isSpell` read it,
  `[CR#110.1,112.1]`), a card, a token and an emblem by what represents the
  object (`isCard`, `isToken`, `isEmblem`, `[CR#111.1,114.1]`).
- **Per-property up/down vocabularies vs `Delta` (§4.5):** Align `CostShift`,
  `ShiftDir`, `LoyaltyCost`, `LifeMove`, and `CounterMove` with the glossary's
  `Delta` rule (`_Avoid_` per-property up/down enums). `CostShift` is
  `generic (Delta Amount) floor | bySymbols (Delta ManaCostTerm) coloredOnly`
  (templates `less`, `more`, `lessBySymbols`, `moreBySymbols`);
  `DamageScale.shifted`, `Cost.loyaltySymbol` (`[0]` is `up 0`, `[−X]` is
  `down X`) and `Action.shiftResult` (a list of changes, the performer's
  choice: "increase or decrease") take `Delta Amount`. A shift never sets
  (`Refusal.shiftSets`; a cost set instead is an alternative cost `[CR#118.9]`,
  and a loyalty symbol puts on or removes counters `[CR#606.4]`). `ShiftDir`, `LoyaltyCost` and the
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
    `doIf`. The anaphora is part of `doIf`'s contract for both word orders:
    in "Create a token if you control a creature. Put a counter on it." (the
    postposed conditional) "it" is the token, as in the preposed form. The
    old Idris-era refusal of that reading is retired on purpose;
    `conditionalArmAntecedentIsTheToken` pins it.
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
    Audit: `DiceBatch` duplicated `Plurality` and is retired; `TokenSpec` and
    `MoveRider` get their `CallerScope` instances (`create` already wraps
    both); `Predicate.other` (no anchor: other than the targets already
    chosen of its kind) and `otherThan` (a written anchor) are documented, two
    meanings. Kept: `mayBeginOnBattlefield` is the opening-hand action of
    `[CR#103.6a]`, not a static, activated or triggered ability;
    `alsoForKeywords` is a Text construct six cards print ("the same is true
    for …"); `thatAbility` reads a chosen ability back as an Ability value;
    `CounterBatch.emptying` is part of the event `[CR#310.12b]`, so it is no
    plurality. An activated ability's own timing, limit and guard are the
    printed restriction on its use `[CR#602.5b]`, `qualified` the one a keyword
    or a grant adds to the ability it carries (ruling of 2026-10-09);
    `triggered`'s limit bounds the triggering; `Action.limited` counts the
    action taken ("do this only once each turn"), and the ability carrying it
    triggers only if its controller has not yet taken that action within the
    limit `[CR#603.2h]`.

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
- **Answers:**
  - Definitions write templates, not raw constructors (`enact`, `putCounters`,
    `gainDesignation`, `removeFromCombat`, `clearDamage`; a subject by
    coercion, a counter kind by coercion, so cards write `p1p1Counter`, not
    `printedKind`). A Binding Identity is named `<declaration>.<parameter>`
    (`fight.other`, `goad.subject`). `cast`, `play`, `activate` and
    `regeneration` bind their subject with `firstMention`. `search` reads its
    zone, `dredge` its amount, `vote` its ballot (`Ballot.inCaller`) in the
    caller; the abilities boast, exhaust, solved (and forecast, infinity, max
    speed, power-up) qualify are `Ability.inCaller`. Search, like mill, scry
    and surveil, reads the performer's own library (`.actor`), as the
    handoff names it `[CR#701.23]`.
  - A mention in a Definition's body never changes a role the caller can
    see: leaving the body (`leaveOwnScope`) restores the caller's pending
    roles as they were when it was entered, with each argument read in the
    caller pending as the call's object. With that, the nesting is one rule,
    identity outermost (`firstMention id (inCaller arg)`, `laterMention id
    (inCaller arg)`), at every Definition site; "Target creature fights
    target creature. Tap it." still reads the fight's source.
  - A keyword variant carries its base keyword's label when the CR calls it
    a variant of that keyword ("is a variant of", "is a hexproof ability",
    "typecycling abilities are cycling abilities"): hexproof from, typecycling,
    devour [quality], equip [quality], and now emerge from
    `[CR#702.11d,702.29e,702.82c,702.6c,702.119b]`. Bands with other keeps its
    own: the banding rules count a creature with it as one without banding
    `[CR#702.22c]`. Devour's definition is one entry (`devourEntry`), not a
    devour nested in devour.
  - `flip` is a Template: flipping is no keyword action `[CR#701.1,710.2]`.
  - Ability words are Templates over `abilityWord`: they have no rules
    meaning `[CR#207.2c]`. `AbilityWordLabel` is a label checked against the
    CR's list (`knownAbilityWord`), and cards call the ability word's
    template.
  - `regenerationApplication` merges into the one `regeneration`
    Definition: a shield's application (remove damage, tap, remove from
    combat, `[CR#701.19a]`), the deed "can't be regenerated" forbids and the
    shield `regenerate` installs applies.
  - The string subtype templates (`creatureType` …) are retired for the
    catalog's subtypes (*Mishra's Factory* now names Assembly-Worker
    `[CR#205.3m]`); card-local helpers take a `Subtype`. A keyword action
    label stays a `String`, refused unless declared (`knownAct`); flavor words
    and vote options are the card's own words `[CR#207.2d,701.38a]`.
    `ItalicWord` and `RoomHalf` are documented: the CR tells ability and
    flavor words apart `[CR#207.2c,207.2d]`, and a split permanent has a left
    and a right half `[CR#709.5c]`, closed as a card's sides are.
  - Templates import none of the checker (`Location.sort` lives with the Core
    AST; the selection-evidence pins moved to `Proofs/`); `crew`/`saddle`
    dispatch where the Definitions are declared; the named `TurnPart`
    aliases were dead and are retired; `gainHaste` is a template over the
    `haste` Definition; ward stays a bare label in `faceDownWardedCreature`,
    since cloak (below the keyword abilities) reads it. A declaration whose
    bare name is taken is named in its sort (`Subtype.army`,
    `DesignationLabel.solved`, `Ability.splice`).
  - Retired: `act`, `lit`, `aRandom`, `command`, `ptModification`,
    `allAmong`, the namespaced aliases (`Quantity.upTo`, `ManaSymbol.generic`,
    `Ability.activatedBy`, `Timing.duringPart`), `itAsToken`/`themAsTokens`
    and `Reach.tokenBorn` ("that token" is `that .token`), and the dead
    checker helpers of §5.4; `isPermanentCard` reads `isPermanentType`.
  - Card warts: printed names by coercion, *Font of Agonies*'
    `bloodCounter`, *Ersatz Gnomes*, *Crimson Wisps* and *Blatant Thievery*
    through their templates, one `copyOnStack` for every copy of a spell or
    ability on the stack (Fork's exception included, `[CR#707.10]`), worded
    `whenIf`/`wheneverIf`/`atIf` over `triggeredIf` `[CR#603.4]` at every
    site (the word read from the Oracle text, from the keyword's CR rule, or
    "at" for the beginning of a step or phase `[CR#603.2b]`), and `is` for
    `matches_`.
  - Adaptive Automaton's and Metallic Mimic's "this creature is the chosen
    type in addition to its other types" are Characteristic-Defining
    Abilities: they depend on a choice as Shapeshifter's power does, and
    depending on a choice is no condition under `[CR#604.3a]`(5). Their other
    abilities, which affect other creatures, stay separate. Shapeshifter's
    printed `*`/`7-*` box is filled by one `[CR#208.2a]`.
  - `doIf`'s other branch reads only what the branch announces: the targets
    and the amounts it names, never a participant the branch would choose as
    it resolves; the branch keeps the ids it announced its targets under, so
    a later line reads the target the condition read, and takes fresh ids
    instead should it need more than were reserved for it.
  - A Definition's first mention that binds nothing is refused once, by its
    own phrase; the later mentions of that identity add no refusal.
  - Left for follow-ups: typed deeds for participles
    (`lean-v3-typed-verbed-deeds`) and a check refusing an undeclared
    subtype (`lean-v3-known-subtype-check`).

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
