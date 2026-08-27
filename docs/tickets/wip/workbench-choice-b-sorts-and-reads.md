# choice-B: the chosen-value sorts and reads

Sub-round B of [workbench-choice-chosen-and-ascription](workbench-choice-chosen-and-ascription.md)
(the umbrella). Owns its sections "The chosen-value sorts and carriers the
reads cannot reach" and "The chosen basic type's own leftover", plus these
routed items from the umbrella's tail: the numeric-characteristic chosen read
(Celestial Judgment, Hurkyl's), Truce's ceiling read, the chosen-quality
consolidation (fold `SetsChosenBasicType` into the general chosen-quality
statics — 3 witnesses + 1 ProofsF pin move), the `CounterKindSource`
bound-kind arm and its counter-event reader blockers, Devotion's
chosen-colour read, and the plural chosen-quality read (`EntersChoice` gains
a `Quantity`; `countQuality` sees `ManyOf`).

Pins:
- Every sort minted is minted WITH its reader, or the reader count is a
  recorded measured zero — never implied.
- The chosen-number `Amount` is ONE row serving Shapeshifter and Sanctum
  Prelate/Talion.
- `countQuality`'s hard-wiring to `Quality QualitySort` is generalized to
  reach the Player kind, not duplicated.
- The repeatability overgeneration stays recorded; NO provenance machinery.
- Card-type and land-type `QualitySort` values land with their 31 chooser
  lines and the 5 chosen-basic-type lines falling out of landed rows — no new
  container work.

Acceptance: the umbrella's acceptance lines for sorts, the chosen-number
amount, and measured zeros. `idris/scripts/build` PASS. Standard constraints
apply.

## As landed

Eight files, +450 / −94. `idris/scripts/build` PASS (23/23 from a clean
`build/`, 0 errors, 0 warnings). Cites: 0 non-compliant, 18,199 checked / 0
stale, 25 audited sites read against their rule text, lock unchanged (every
rule already blessed). No design artifact: the docstrings plus this section
are the record.

### Premise corrections, up front

- **The umbrella's "card-type and land-type `QualitySort` values" section is
  STALE.** Both values landed in `done/workbench-quality-sort-gaps` —
  `CardTypeQ`, `SubtypeQ : CardType -> QualitySort`, and the
  `BasicTypesOnly`/`NonbasicTypesOnly` domains with them. Nothing to mint;
  what was left was the reads and benches falling out of them, which is what
  this round did.
- **`CounterKindSource`'s bound-kind arm is NOT "the single node holding all
  13 `CounterKindQ` lines".** Every one of the 13 carries a SECOND blocker.
  Measured below.
- **Teyo, Aegis Adept is unsupported.** The direction sort's supported
  carriers are Mystic Barrier, Pramikon, Order of Succession, Aminatou and
  Teyo, *Geometric Tactician* — five, not the umbrella's two.

### The chosen basic type's own leftover — `SetsChosenBasicType` folded

`SetsChosenBasicType` is gone. `OfYourChoice` gains `QualityNoun`'s domain
slot, so "the basic land type of your choice" is
`SetsChosenQuality n (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly))`,
and the bespoke row's `LandSubject` gate is replaced by the general
`HostedRead`: a chosen-quality ascription's subject must be able to CARRY the
sort read, [CR#205.3d]'s refusal of a subtype that corresponds to none of the
object's types. `LandSubject` is deleted (it had no other caller). All three
witnesses move (Reef Shaman, Grixis Illusionist, Unstable Frontier) and the
`ProofsF` pin `badChosenBasicTypeOnCreature` moves with them, now refused by
`hr` rather than `ls`. Every pre-existing chosen-quality witness satisfies the
new law unchanged.

`sameChoiceDomain`/`sameDomainOpt` lost their shared index — a read at
`Object` carries its sort in a slot, not in its kind, so `predEq` compares
two domains that need not be at one sort.

**The 5 basic-type lines:** Realmwright landed last round; **Convincing
Mirage** benches here (whole card, the SET case on the attach host);
Phantasmal Terrain is the same two rows at a different cost, recorded not
duplicated. Thran Portal (conditional enters-tapped, mana-ability cost rider)
and Multiversal Passage ("Then you may pay 2 life. If you don't, it enters
tapped") stay blocked on machinery outside this family.

### The chosen-number amount — one row, both spellings

`ChosenNumber : {ChoiceStands (countChoice (QSort Number) bs)} -> Amount bs`.
ONE row for "the chosen number" and "the last chosen number" because
[CR#607.2d] writes one linkage over "the chosen [value]", "the last chosen
[value]," or similar; the gate is existence, so which word a card prints
follows from its chooser's repeatability and no rule tells the readings apart.
`readAmount ChosenNumber = False` on `UpTo`'s ground ([CR#608.2d] announces
it; no game state carries it).

`ChoiceDomain` also gains **`NumberBetween lo hi`** — 7 supported lines write
a printed range and `NumberAbove` is a floor alone. Bounds gated on
`lo <= hi`: [CR#608.2d] forbids choosing an impossible option and an empty
range leaves none.

**Benches:** `sanctumPrelate` (whole card — the plainest read: one chooser,
one comparison bound, no domain) and `expelTheInterlopers` (whole card — the
read as a comparison bound plus the printed range).

- **Shapeshifter does NOT bench** and the amount is not why: its second
  chooser is an upkeep trigger, which is sub-round C's non-entry chooser.
- **Talion does NOT bench**, and this is a finding worth routing: its body
  writes "a spell with mana value, power, or toughness equal to the chosen
  number", and `Or [Compare ManaValue …, Compare Power …, Compare Toughness …]`
  is refused by `parallelDisjuncts`' `seedsUniform` — `comparedType` answers
  `Nothing` for mana value and `Just Creature` for power and toughness. The
  honest shape is a characteristic LIST on `Compare`, not an `Or` of three
  predicates. 1 supported line; recorded, not bought.

### The sorts and carriers — the player sort

`countQuality` is **generalized, not duplicated**:
`countChoice : ChoiceSort -> Bindings -> Nat` over
`ChoiceSort = QSort QualitySort | PlayerC`. The quality arm still reduces to
`kindLte (Quality q) k`, so `ProofsAnaphora`'s `countQualityIsCountOnes` /
`countQualityIsFold` survive verbatim and every chosen-read gate still reads
`bs` unchanged (the HYBRID pin from sub-round A is untouched).

A player is deliberately not a `QualitySort`: that catalog ranges over
[CR#109.3]'s characteristics, which is what lets `chosenQualityReadOk` ask
whether an object matches; a player is no characteristic of anything. What
the two share is [CR#607.2d], and that is what `countChoice` counts.

- `Payload` gains **`ChosenPlayerP`** — the one mark the choice vocabulary
  needs, because `Player` is the only kind a chooser shares with ordinary
  mentions ("you", "each opponent"). It echoes and describes exactly as
  `PlayerP` does (`wordReaches PlayerW`, `halfReaches`, `setZone`,
  `markTyKeeps*`).
- `EntersChoice` takes a `ChoiceSort`; `staticChoiceIntro` mints `choiceB`.
  The 33 `Cards.idr` chooser sites needed NO edit — `entersChoosing` /
  `entersChoosingFrom` keep their `QualitySort` surface and wrap. New macro
  `entersChoosingPlayer`.
- `ChoiceDomain` is re-indexed to `ChoiceSort` and gains **`OpponentsOnly`**
  ([CR#102.1] / [CR#102.2]; the team form [CR#102.3] stays deferred as the
  `Opponent` head noun already defers it). 29 supported cards write a
  chosen-player read: 18 narrow the chooser to an opponent, 7 leave it at "a
  player".
- `Predicate` gains **`ChosenPlayer`**, read under `Definite` ("the chosen
  player"). At this kind the read IS the head noun, where `OfChosen` is a
  description matching a value against a characteristic. ONE row for both
  spellings on `ChosenNumber`'s ground, and the gates agree over the whole
  corpus: **no supported card writes two singular player choosers** (measured
  zero), so nothing here weakens `badTwoChoosersOneSortRead`.
  `uniquifies ChosenPlayer = True` — [CR#607.2d]'s linkage names exactly one.

**Benches:** `nyxathid` (whole card — narrowed chooser + read inside a count)
and `stuffyDoll` (whole card — bare "choose a player" + the read as a damage
recipient).

**Still blocked, and none of it on the sort:** Beckoning Will-o'-Wisp and
Triarch Stalker write "the last chosen player" behind a COMBAT-trigger
chooser, which is sub-round C's non-entry chooser position. Bitter Feud and
Sower of Discord write "choose two players" and read it partitively ("one of
the chosen players … the other chosen player"), which wants the plural
chooser below plus a partitive over a chosen set.

### The marked read's other carriers — declined, measured

The marked read is 12 lines over 12 cards, by sort: object 4 ("the last
chosen card" — Forgotten Lore, Shrouded Lore, Koh, Paleontologist's Pick-Axe),
player 2, direction 2, colour 2, number 1, name-and-creature-type 1.

- The **number** carrier is served: `ChosenNumber`'s gate is `ChoiceStands`.
- The **player** carriers are served: `ChosenPlayer`'s gate is `ChoiceStands`.
- Generalizing `OfLastChosenColor` to `OfLastChosen q` would buy exactly
  **Psychic Paper** (its two sorts, `CardName` and `SubtypeQ Creature`), whose
  chooser is the ATTACH-triggered position — sub-round C's. **Declined this
  round** rather than landed unwitnessed; it is a five-line change the day C's
  chooser lands.
- Shapeshifter's amount carrier is the `ChosenNumber` row above.

### The chosen-quality consolidation's other routed items

- **Devotion's chosen-colour read — LANDED.** `Devotion`'s two
  `Chroma.Color` slots take a `ColorTerm bs` (`LitColor` | `ThatColor`,
  gated on a unique chosen colour), rather than a second devotion row:
  [CR#700.5]'s count is the same either way and only the slot's filler
  differs. **`nykthosShrineToNyx` benches whole**; Nyx Lotus is the same
  ability without the {2}. Altar of the Pantheon ("devotion to each color and
  each combination of colors is increased by one") is a devotion-MODIFYING
  static, a different cell, ledgered.
- **The numeric-characteristic chosen read — LANDED, and it needed no new
  node.** Once the chosen number has an amount seat, Celestial Judgment's "a
  creature with that power" is `Compare Power Eq ChosenNumber` over the value
  the `ValueAxis Power` pass bound. `chosenQualityReadOk Number = False`
  stands untouched beside it, and the two are consistent for the rule's own
  reason. Benched as `celestialJudgmentPass` (the pass; "Destroy each creature
  not chosen this way" wants a negated verb-stamped read and is not this
  cell). Hurkyl's card-type twin stays blocked on "from among the revealed
  cards", exactly as `workbench-quality-sort-gaps` recorded.
- **The `CounterKindSource` bound-kind arm — LANDED with a MEASURED ZERO of
  benched carriers**, and the arm is never what blocks them.
  `CounterKindSource` is now indexed by `Bindings`; `BoundKind` reads a kind
  an earlier chooser or distributive pass bound [CR#607.2d].
  `counterSourceScope BoundKind k = True` at every kind — recorded
  overgeneration, since a bound kind's scope is whatever the binder drew it
  from and no slot carries that. **The 13 lines' real blockers:** 4 give the
  counter to "that permanent or player" and `PutCounters`' recipient is an
  object (Animation Module, Maulfist Revolutionary, Powerful Broker, Skyship
  Plunderer); 4 write a chooser this grammar cannot spell — "choose a COUNTER
  on [n]" naming one kind, or "at random … from among [menu]" (Aven Courier,
  Contractual Safeguard, Crystalline Giant, The Caves of Androzani); 2 write
  "put another counter of that kind on IT" where the entering creature and the
  target permanent are both in scope and the pronoun refuses (Quarry Hauler,
  Dramatist's Puppet — probed, `countOnes Object` is 2 and `ItAt PermanentSlot`
  is 2 as well); 1 puts the pass inside a static (Blue, Loyal Raptor); 1 mixes
  a printed kind and a bound one in one menu (Bribe Taker); 1 needs a
  partitive holder, "either of those tokens" (Exotic Pets).
  **Deliberately NOT widened:** `RemoveCounters`' `Maybe CounterKind` slot.
  Quarry Hauler's "or remove one from it" elides the bound kind and so wants
  it, but the pronoun blocks that card anyway, so widening it now would add an
  unprinted `ChosenKind`-on-removal overgeneration for no witness. Probed and
  reverted.
- **The counter-event reader blockers** (`lookbackSubjectOk` shut at
  `CounterPlacement`/`CounterRemoval`, and no noun naming ONE kind) are
  unchanged and stay recorded on the `Events.idr` rows. Neither is a
  chosen-value cell: the first is a lookback-subject table, the second a
  predicate at `Quality CounterKindQ`. Routed on.

### Declined with a written verdict

- **The direction sort.** 5 supported carriers (Mystic Barrier, Pramikon,
  Order of Succession, Aminatou, Teyo Geometric Tactician). The sort itself is
  cheap and its shape is settled: a direction is `Number`'s twin, a
  `QualitySort` row with `chosenQualityReadOk Direction = False`, since a
  direction is not one of [CR#109.3]'s characteristics — NOT a `Kind`
  constructor, which the umbrella assumed and which the player sort needed
  only because a chosen player has to be usable as a `Noun bs Player`.
  Its READER is a player-locating phrase — "the nearest opponent in the
  (last) chosen direction" (3) and "the next player in the chosen direction"
  (2) — and all five sentences containing it are blocked elsewhere: the
  attack-only restriction (3), "starting with you and proceeding in the chosen
  direction" (1), and a loyalty ability plus group control transfer (1). So
  the sort would land with an unwritable reader, which is dead structure by
  this repo's own rule. **Mint it beside its first reader**, not before.
- **The plural chosen-quality read.** Measured: the plural chooser is 4
  supported lines — "choose two colors" (Seal of the Guildpact, Tablet of the
  Guilds) and "choose two players" (Bitter Feud, Sower of Discord). The
  chooser slot is cheap (a count on `EntersChoice`, `choiceB` at `ManyOf`, a
  plural counter beside `countChoice`). Every one of the four READS is not:
  Seal and Tablet want "for each of the chosen colors IT IS", an intersect-
  count of the object's colours with the chosen set; Bitter Feud and Sower
  want a partitive plus "the other" over a chosen pair. Landing the chooser
  alone is half a bridge, so it is declined here with the shape written down.
- **Truce / Temporary Truce's ceiling read.** "For each card less than two a
  player draws this way" is the SHORTFALL of a ceilinged draw — the ceiling
  `UpTo (Lit 2)` announces nothing, and no row reads the number not taken.
  The shape it wants is an announcement on the ceilinged amount (what was
  drawn this way) so the tail can subtract; that is the `UpTo` row's business,
  not the chosen-value vocabulary's. 2 cards, identical text. Routed on.

### Pins

`badChosenBasicTypeOnCreature` moves to the general row and still refuses, now
on `HostedRead` and [CR#205.3d]. `badYourChoiceNumber`,
`badReaderBeforeChooser`, `badTwoChoosersOneSortRead`,
`badNonCreatureTypeExclusion`, `badPluralDevotion`, `badRemoveCountersDead`
and `badAxisValueCrossing` all still fail as they must. No new pin: the two
candidates this round produced (an empty number range; a chosen-player read
with no chooser) are refused by gates that already exist, and a player-sort
twin of `badReaderBeforeChooser` would be a near-duplicate of a landed pin.
