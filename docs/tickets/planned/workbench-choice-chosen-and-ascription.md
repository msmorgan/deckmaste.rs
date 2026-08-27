---
needs: []
---
# Write the chooser positions, the chosen-value reads, and the ascription rows' last payloads

**SPLIT 2026-08-27 into five sub-tickets — claim those, not this.** This file
is the umbrella and stays authoritative for section content, measurements,
pins and acceptance: [A obligation face law](workbench-choice-a-obligation-face-law.md),
[B sorts and reads](workbench-choice-b-sorts-and-reads.md),
[C chooser positions](workbench-choice-c-chooser-positions.md),
[D ascription payloads](workbench-choice-d-ascription-payloads.md),
[E subtype words and linkage](workbench-choice-e-subtype-words-and-linkage.md).
Order: A first (architectural); B before C's counter-kind chooser reads; D and
E free after A.

One family across the choice container, the chosen-value read side, and the
ascription rows that consume those reads. The chooser positions decide what a
choice may bind; the read sorts decide what may be read back; and the
ascription/type-payload cells are where nearly every one of those reads is
spent. They share `EntersChoice`/`Choose`, `QualitySort`/`countQuality`/
`ChosenQualityRead`, and the `SetsType`/`BecomesAlso`/`AsType` region, so a
round that opens one has to answer the others.

## The chooser positions the as-enters rider does not reach

The indexed choice container landed with one grammar row, the as-enters chooser.
Four other chooser positions are measured and unwritable, and a fifth shape is
recorded as deliberately out of scope. They share one container and one
`Choose`/`EntersChoice` region.

### The attach-triggered chooser — 3 cards

Sanctuary Blade, Dinosaur Headdress, Psychic Paper: "As this Equipment becomes
attached to a creature, choose …". `EntersChoice`'s `ZoneFits (nounZone n) (Just
Battlefield)` gate is keyed to entering the battlefield, and the attachment
happens after the permanent is already there — a different entry-replacement
timing, **not** a widening of that gate.

It is the marked read's largest single blocker and the price is probed, not
guessed: Sanctuary Blade written with a stand-in as-enters chooser elaborates
whole (its second line is `Gains … (KeywordAbility Protection {param = Just
(ParamQuality OfLastChosenColor)})` and its Equip line writes), so the attach
trigger is that card's only blocker.

### The non-entry chooser — 5 cards

Beckoning Will-o'-Wisp and Triarch Stalker at a combat trigger, Teyo at a
loyalty ability, Koh the Face Stealer at an activated ability, Mystic Barrier at
a fused "when this enchantment enters and at the beginning of your upkeep"
header. Chromatic Armor's and Shapeshifter's second choosers sit here too. The
effect-level `Choose` row already exists, so what these want is the
**container**, not the choice clause.

### The chooser that chooses an object — 4 cards

Koh the Face Stealer, Dinosaur Headdress, Forgotten Lore, Shrouded Lore: "Choose
a creature card exiled with Koh", "Target opponent chooses a card in your
graveyard". The read is "the last chosen card" and the mention is an **object**,
not a `Quality` — `countQuality`/`qualityB` do not reach it. The two Lores add an
opponent as chooser and a repeated process ("repeat this process except that
opponent can't choose a card already chosen for Forgotten Lore") on top.

### The compound chooser — 3 supported lines

"Choose a color and a creature type" (Riptide Replicator, Volrath's Laboratory),
"choose a color and an opponent" (Call to Arms). The composition is already free
— two `EntersChoice` lines of different sorts thread two mentions and both reads
elaborate (probed) — so what is missing is only the **spelling** of two choices
in one sentence. A coordination question, not a discourse one.

### The plural read — 5 cards, and it splits by chooser shape

Its three chooser configurations are inventoried in
[workbench-name-match-family](workbench-name-match-family.md); they are
chooser shapes, so measure them against whatever container this round lands
before assuming one construction covers them.

### Recorded as not this shape — the each-player chooser, 3 lines

"A type chosen this way" (Harsh Mercy, Patriarch's Bidding, Lydari Druid): a
distributive choice with as many values as there are players and no single
binding to read. Do not fold it into the container.

### Chromatic Armor's remaining 2 lines

The `{X}` activation cost — `SimpleManaSymbol` is `Generic Nat | Specific
ColorOrColorless` and has no variable symbol, with the wrinkle that the card's
defining rider is read by the **ability's cost** rather than by its body — and
the `sleight` counter kind. Both deliberately unbought while the card cannot land
whole; everything else on that ability already writes.

## The chosen-value sorts and carriers the reads cannot reach

The chosen-value reads landed colour-fixed. What is left is the read side at the
sorts that have no representation and at the two carriers no row covers, plus the
`QualitySort` values whose absence blocks three families at once. One region:
`QualitySort`, `countQuality`, `ChosenQualityRead`.

Authority: [The kind index joins; union marking is
spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md) — the
player and direction sorts are decided here on lowerability and algebra, not on
which surface marks them, and the overgeneration below is tolerated rather than
closed.

### The marked read's other carriers — 2 lines

Psychic Paper's name-and-type **setting** clause ("its name and creature type are
the last chosen name and creature type") reads two qualities in one clause, at
sorts `ChosenQualityRead` calls unreadable for the unmarked read. Shapeshifter's
**amount** ("power is equal to the last chosen number").

### The chosen-number amount — one construction serving two entries

Shapeshifter's line above and Sanctum Prelate/Talion's chosen-number read are the
same missing `Amount`, not a missing comparison relation; the equality relation
those cards also waited on is landed, so the amount is their remaining blocker.

### The sorts with no representation

- **Direction** has no `Kind` constructor at all (Mystic Barrier, Teyo — "the
  nearest opponent in the last chosen direction").
- **Player** has a `Kind` and a payload but is unreachable through
  `countQuality`, which is hard-wired to `Quality QualitySort` (Beckoning
  Will-o'-Wisp, Triarch Stalker — note the read stands in `Attacks`' defender
  slot).
- Polis the Planeshifter's **plane** is unsupported and needs no row.

### The card-type and land-type `QualitySort` values

11 card-type chooser lines against one card-type reader (Pippin, Guard of the
Citadel), and 20 land-type chooser lines. `QualitySort` has neither value. The
chooser row takes all 31 the day the values exist, with no further container
work.

Behind the land-type value sit the **5 chosen basic land type** lines
(Realmwright, Thran Portal, Convincing Mirage, Phantasmal Terrain, Multiversal
Passage). They are neither reachable nor absorbable today: the your-choice
basic-type setting row covers only the setting case at that sort, so the two rows
are disjoint in fact. Mint the value and they fall out of the landed ascription
rows.

### The overgeneration the marked read carries — recorded, not bought

All 12 carriers of the marked read have a chooser that can fire more than once,
and **no card** writes the marked read against a once-only chooser — but nothing
here represents a chooser's repeatability, so the row admits a marked read after
a single non-repeating chooser. Closing it wants a provenance mechanism of the
kind the anaphor work queued, it is worth nothing on its own, and the direction
above tolerates it. Do not buy provenance inside this round.

### Unprobed, from the exact-value sweep

Two exiled-with-X carriers were routed forward untested rather than claimed:
Ashiok, Nightmare Weaver (a loyalty ability and a type-addition rider) and
Bronzebeak Foragers (an until-leaves exile and a per-opponent distributive).
Probe before scoping either.

## The literal colour change and the ascription rows' last payloads

The chosen-quality ascription rows landed and spell a colour change at layer 5
when the colour is a **read**. The literal half — a colour word where those write
a read — still has no row, and three more payloads sit one row away from whole
cards on the same carriers.

### Colour-only "becomes [color]" — 42 supported lines

Layer 5 [CR#613.1e], under the "becomes" verb alone: "becomes blue until end of
turn", "All creatures become black". The type-setting row's payload carries
colors, but only as part of a type line, and a colour change that names no type
is a different layer and a different sentence.

It is no longer designed from nothing: the chosen-quality setting and adding rows
write exactly this sentence over a read ("All nonland permanents are the chosen
color", Shifting Sky; Painter's Servant's adding form), **20 lines**. Model the
literal half on them; the entry's size is unchanged by their landing.

### Celestial Dawn's literal colour setting, and Maskwood Nexus's "every creature type"

Both are ascriptions the landed off-battlefield extension would accept the day
their rows exist. Celestial Dawn is additionally blocked on an unrelated second
line, as are Biotransference and Roshan.

### Ashes of the Fallen — 1 line

It ascribes to creature cards in a **graveyard**, where both ascription rows
demand a battlefield subject. The demand was kept rather than carved out; decide
it deliberately here.

### Leyline of Transformation — 1 card, 1 construction

Its ascription line writes; what is left is the Leyline play permission ("If this
card is in your opening hand, you may begin the game with it on the
battlefield").

## The ascription's last five subtype words, its self-antecedent pronoun, and linkage

`AsType` is already general over [CR#109.2]'s whole phrase ("a card type or
subtype"); what is left is cells. Five measured words, the pronoun that cannot
read an ascription back, and the linkage cell whose blocking word now exists —
all on the same declarations.

### Five unrowed subtype words

Each is one `ascribesAsSubtype` True cell plus one `subtypeType` card type, grown
on the round that benches a card writing it. NO CROSS-LINKS LEFT, no design
question open:

- Vehicle — 166 occurrences / 123 cards. Cheapest of the five; its cards want crew.
- Class — 27/22.
- Spacecraft — 20/16.
- Case — 13/9.
- Room — 4/4.

Already rowed and not to be re-proposed: `Aura`, `Curse`, `Equipment`, `Siege`
(landed with the `Battle` card type, [CR#205.3q] making it the only battle type)
and `Saga` (landed with the chapter-ability construction; 92 own-line occurrences
over 80 cards, against the 2 lines on a Saga that write "this enchantment"
instead — Day of the Moon, Death in Heaven).

### The self-antecedent pronoun after an ascription — 5 occurrences / 4 cards

Soul Ransom's "This Aura's controller sacrifices IT, then draws two cards" does
not land, and the block is exactly one thing: an unmoved ascription announces no
mention, so `It` has no antecedent to read. The possessive itself probed green
and needs nothing (`ControllerOf` over the ascription). The whole possessive
family is 5 occurrences over 4 cards (Aura 1, Equipment 2, Vehicle 2).

The question this round answers: whether a self-reference should announce a
readable mention at all — the "this card" / "this creature" split asked from the
pronoun's side.

### Linkage under a subtype word — 6 lines the delta

`SortedSelfLinked` covers the card-type ascription only, measured: "cards exiled
with this Saga" is six supported lines, "this Vehicle" three, "this Class" one,
while the two rowed subtypes write it ZERO times apiece. The cell opens with
those rows and not before them — and the Saga row now exists, so its six lines
are writable the moment the cell opens.

## The type addition's payload and surface cells

The arrival ascription landed its payload — `BecomesAlso` now takes `SetsType`'s
`TokenChars` bundle, so the colour, the P/T glyph and the with-clause ability
are writable at both readings (finding 1121), family remeasured at 59 lines over
58 cards (finding 1117). Three residues remain, all in the same bundle-and-gate
region.

- **The face-down arrival rider**, 7 of the 59 lines place their object FACE
  DOWN before ascribing to it: Yedora Grave Gardener, Magar of the Magic
  Strings, Tezzeret Cruel Machinist, Missy, Cybership, Death in Heaven, The
  Cyber-Controller. `TokenRider` is `EntersTapped | EntersAttacking` and nothing
  else. This is the flip verb's gap counted from the arrival side, and the
  design note that closes both at once is: index the rider vocabulary over
  `StatusVal`. Four of the seven want a `Cyberman` catalog word as well. Yedora
  is the cheapest — one line, one rider, and a type line the catalog already
  spells.
- **The colour-only type addition**, 1 line: "Target permanent becomes blue in
  addition to its other colors" (Indigo Faerie). `BecomesAlso` now carries the
  colour, but every one of its gates asks the TYPE LINE, so a bundle with a
  colour and no type word cannot pass `LineNonEmpty`. The other line of the
  shape is the CHOSEN colour that `AddsChosenQuality` already spells (Painter's
  Servant), so the cell is one card wide. What it costs is a bundle-level "says
  something" gate where three line-level ones stand (finding 1119).
- **The type-ORDERING half of `tokenCanonical`** is not asked of the addition
  row and never has been, so an unordered addition line is writable where the
  setting's is not. Whether that is real free variation or an unmeasured hole is
  one sweep's work.
- **The "base power and toughness 4/3" RIDER phrase** — 78 lines at the setting,
  19 more inside addition clauses. It is `HasBasePt`'s and still wants the
  coordination of two statics inside one sentence that no construction spells,
  the same ledger entry `SetsType` carries. The 19 are worth their own count:
  most coordinate the rider with the addition in one sentence ("Equipped
  creature has base power and toughness 5/5, has menace, and is a black Demon in
  addition to its other colors and types", Blade of the Oni), so closing the
  coordination closes them at both rows at once.

Both surface cells were deliberately left by the payload widening and their
reasons are recorded in `BecomesAlso`'s docstring (finding 1123) — read it
before redesigning either.

## The type quantifier and the ascription rows' leftovers

The type-space quantifier landed at the ascription position (`TypeSpace`, the
third payload shape beside the enumerated line and the chosen quality, and a
sibling row to `SetsChosenBasicType` for its stated reason — no `Subtype` value
denotes a space), carrying the GRANT frame with it (finding 935: the verb and
the quantifier word are both the frame's), so 22 of that family's 33 sentences
write. The type-line ascription rows landed too — two rows, because [CR#205.1a]
and [CR#205.1b] tell setting from adding exactly as they do for
`SetsType`/`BecomesAlso`, one row per operation, the payload carrying the source
axis (`OfChosen q`, `OfYourChoice q`, gated by `QualityRead`). What both left is
one round's worth of cells at the same position.

### The quantifier's cells

- **The LOSS cell**, 6 sentences: "loses all creature types" (Curse of
  Conformity, Ego Erasure, Nameless Inversion, Amoeboid Changeling) and "loses
  all land types and abilities" (Alpine Moon, Lithoform Blight, Ultima, Origin
  of Oblivion). The negative pole of `AddsEveryType`. The three land-type lines
  COORDINATE the type loss with an ability loss inside one phrase, which
  `LosesAllAbilities` writes as its own statement today — decide whether the
  coordination is one row or two before minting either.
- **The becomes-a-creature-WITH cell**, 3: "This land becomes a 2/2 creature
  with all creature types until end of turn" (Mutavault, Faceless Haven,
  Soulstone Sanctuary). The quantifier rides `SetsType`'s `TokenChars` there
  rather than the ascription position, so it is a payload question inside the
  token characteristics and not this row's.
- **"Is all colors"**, 1 (Leyline of the Guildpact): a COLOUR payload at the
  same position, quantifying a different space with its own carrier. Measure
  against the colour-setting family before scheduling; that card additionally
  wants the opening-hand permission.
- **The LAND space's other carriers**: Omo, Queen of Vesuva writes the only "is
  every land type" ascription and wants an everything counter — the space's row
  is minted and witnessed by nothing yet.
- **Still open from the enumerated side**: the choose-two lines ("As this
  enchantment enters, choose two basic land types"; "Choose two basic land
  types. Create the appropriate Shockland token").
- **One card away**: ENERGYBENDING (the basic-land grant cell) needs only the
  Lesson subtype; MASKWOOD NEXUS needs "the same is true for creature spells you
  control and creature cards you own that aren't on the battlefield", an
  `AlsoOffBattlefield` whose subject is TWO FURTHER NOUNS rather than the same
  one restated.

### The chosen basic type's own leftover

- **The chosen type ADDED**, 1 line: "becomes the basic land type of your choice
  in addition to its other types" (Navigator's Compass), [CR#305.7]'s last
  sentence at the chosen subtype. `BecomesAlso`'s payload is a bare `TypeLine`,
  bs-free like the token bundle, so the choice cannot ride it; one line does not
  buy a second row. Wait for a second carrier — recorded so the absence is not
  re-read as an oversight.

### Facts to keep, not re-derive

- `BasicLandType : Subtype -> Type` with `PlainsBasic | IslandBasic | SwampBasic
  | MountainBasic | ForestBasic` sits beside `subtypeType`; [CR#205.4c] keeps
  Basic-the-supertype independent of the subtype (Wastes proves the axes come
  apart). The family's arity is [CR#305.7]'s "ONE OR MORE", so a list witness
  `BasicLandTypes` rides beside it (Lush Growth writes three types in one line
  and is benched). No Bool table was copied for it: [CR#305.6] closes this set
  from outside the catalog, where the keyword-counter idiom exists because
  [CR#122.1b]'s list is open at an open catalog's end.
- `SetsChosenBasicType` has NO domain slot and NO chooser slot, both measured
  zeros (finding 802); the choice half is 12 lines, not 19 (finding 797). The
  bundle half needed nothing (finding 798): `HasSupertype` landed with Winter
  Moon's backfill and seven probes elaborated first attempt (Blood Moon,
  Conversion, Celestial Dawn, Kavu Recluse, Contaminated Ground, Lush Growth,
  Swampbenders).
- **A NON-BASIC land type**: the `Subtype` catalog holds only the five, so
  `basicLandLine`'s gate currently refuses nothing the card-type check would not
  (finding 801 — a refusal by vocabulary, not pinned). The first card to bench
  Desert, Gate, Cave, Locus or Urza's turns that gate live and earns the pin.
- **Free witnesses** for any round wanting a cheap land line: the named "becomes"
  settings, 17 lines (Kavu Recluse, Streambed Aquitects, Nightcreep, Thelonite
  Monk, Cyclopean Giant), writable today and deliberately unbenched — the
  copular and inchoative frames are the same term and both are already benched.
  SONG OF THE DRYADS ("Enchanted permanent is a colorless Forest land") is
  writable on `SetsType` and unbenched; it is the reason NO gate was added
  against subtype-only basic land lines, since a basic land type beside a colour
  and a card type is a sentence no basic-only payload could hold. The two rows
  do not overlap — `SetsChosenBasicType` carries no subtype at all.

## The closure grid this family owns

Ranked below the workbench's fifteen top flip risks — the grid's many zeros are
independent measurements, so a printing moves one cell and the closure survives —
but every cell this family moves is one of them. A widening names the cell it
moved and re-reads the grid rather than defaulting it, and a cell left closed
says whether a rule or a count is closing it. Rows, evidence and widening costs:
[the closure tables](../../idris-workbench-closure-tables.md) §2.1.

Two grids. The `Subtype` catalog's ~130 rows (`Words.idr:1501`), grown one
witnessed card at a time; and `ascribesAsSubtype`'s nine-self-naming-words census
against every creature type (`Words.idr:1866`).

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **`CountedGroup` carries no `ChoiceMode` at all.** That blocks ~30
  counted-plural at-random lines ("discards two cards at random", "discard X
  cards at random") and the same slot unblocks 35 plural "… of their choice"
  lines; `SomeOf` (7 lines) and the modal list (5) want it too. 51 call sites —
  `docs/tickets/done/workbench-randomness-vocabulary.md`.
- **The plural chosen-quality read.** `Static.EntersChoice` takes a bare
  `QualitySort` with no `Quantity`, and `countQuality` counts only `OneOf`
  bindings, so a `ManyOf` quality binding is invisible to every chosen-quality
  read (Seal of the Guildpact, Tablet of the Guilds) —
  `docs/tickets/done/workbench-name-match-family.md`.
- **Aisling Leprechaun's "that creature becomes green".** `ColorIs` exists as a
  predicate only and the `Becomes*` effect rows set types or copy; this is the
  colour-only "becomes [color]" section's carrier —
  `docs/tickets/done/workbench-event-algebra.md`.
- **Devotion's chosen-colour read.** Nykthos, Shrine to Nyx and Nyx Lotus write
  "your devotion to that color" over a chosen quality while `Devotion`'s slot is
  a literal `Chroma.Color`; Altar of the Pantheon modifies devotion across every
  colour combination at once —
  `docs/tickets/done/workbench-amount-comparison-and-quantity.md`.
- **Subtype rows named by closed rounds and still unrowed:** `Fungus`, `Kor`,
  `Wolf`, `Arlinn`
  (`docs/tickets/done/workbench-counter-family-residues.md`), `Drake`
  (`docs/tickets/done/workbench-event-algebra.md`), `Garruk`
  (`docs/tickets/done/workbench-multiface-cards.md`).
- **The joint-typing container for cross-line choices** (consolidated open gap
  15) would move the five [CR#608.2c] pins together —
  `docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md`.
- **Master Biomancer's entry-time type addition** coordinated with its counter
  clause — `docs/tickets/done/workbench-counter-family-residues.md`.

## Consumption boundary

`idris/src/Experimental.idr` (`EntersChoice`, `Choose`, `ZoneFits`/`nounZone`,
the choice container's index; `ChosenQualityRead`, `QualityRead`, the `Amount`
catalog, `Attacks`' defender slot; `SetsType`, `BecomesAlso`, the
chosen-quality setting/adding rows and the off-battlefield wrapper; `AsType`,
`SortedSelfLinked`, `ControllerOf`, the `Subtype` rows and the pronoun's
antecedent machinery; `TokenChars`, `TokenRider`, `LineNonEmpty`,
`tokenCanonical`, `HasBasePt`; `AddsEveryType`, `TypeSpace`,
`SetsChosenBasicType`, `LosesAllAbilities`, `AlsoOffBattlefield`),
`idris/src/Experimental/Words.idr` (`countQuality`, `qualityB`,
`SimpleManaSymbol`, `CounterKind`; `QualitySort`, `Kind`; the colour
vocabulary; `ascribesAsSubtype`, `subtypeType`; the `Cyberman` catalog word;
`TypeLine`, `Subtype`, `basicLandLine`), pins in
`idris/src/Experimental/ProofsF.idr` and the other pin modules
`idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- `badTwoChoosersOneSortRead` still holds: three cards in the corpus write two
  same-sort choosers and none of them then writes the plain read, because English
  marks a re-choice with a different word ("the last chosen color") [CR#607.2d].
  No append and no re-reading of `countQuality`.
- The attach timing is a distinct entry-replacement row, not a relaxed
  battlefield gate; Sanctuary Blade benches whole.
- The each-player distributive is still refused and still recorded as a
  different reading.
- Each sort minted is minted with its reader, or the round records the sort's
  reader count as a measured zero rather than leaving it implied.
- The chosen-number amount serves Shapeshifter and Sanctum Prelate/Talion from
  one row.
- The repeatability overgeneration is still recorded as accepted, with no
  provenance machinery added for it.
- The literal colour change is one row per operation at the same layer as the
  chosen-colour rows, not a second payload on the type line.
- The battlefield-subject demand ends the round either kept with Ashes of the
  Fallen recorded against it, or widened on stated evidence.
- Every subtype word added is paid for by a benched card writing it; the two
  cells per word are the whole cost and the ascription itself is not reshaped.
- The self-reference's mention question is answered explicitly — announced or
  refused — with Soul Ransom benched or the refusal recorded.
- The linkage cell opens only for subtypes whose rows exist, and the zero cells
  for the rowed subtypes that never write it stay zero.
- Yedora Grave Gardener benches; the 7 face-down lines write through one rider
  vocabulary shared with the flip verb.
- Indigo Faerie writes without dropping what the three line-level gates refuse.
- The ordering sweep returns a verdict — free variation or hole — recorded, not
  deferred.
- The 6 loss sentences write, with the type-loss/ability-loss coordination
  decided one way and the decision recorded.
- Energybending benches on the Lesson subtype; the choose-two lines are either
  written or refused with a counted reason.
- Every measured zero above (the domain and chooser slots, the non-basic gate's
  vocabulary-only refusal) survives the round explicitly.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## Ruling (user, 2026-08-26): cross-ability chosen reads are a forward obligation, discharged at the card boundary

The `EntersChoice → OfChosen` cross-ability linkage stops being telescope
threading (`staticChoiceIntro`'s export through `abIntro` retires; `abIntro`
collapses toward constant). Instead: the choice ability mints a FORWARD
OBLIGATION — one that is nonsense unless a LATER ability of the same card
discharges it by reading — and a new face law (sibling of
`CardText`/`CardChapters`/`CardBox`, on `textDefines`' whole-sequence fold
precedent) checks the pairing: every cross-ability chosen-read of sort q
finds a chooser of sort q in an EARLIER ability, and every chooser finds a
later reader. That is [CR#607.2d]'s linkage as a card law, running the same
forward direction the binder contract already runs, one level up. NO second
index on Effect — the obligation lives in one `So`-gate at the boundary
where the whole text is visible. Fragment-level bare chosen-reads become
tolerated overgeneration, refused at the face law. Default pin: HYBRID —
intra-ability chooser→read stays anaphoric under `bs` (ProofsAnaphora
untouched for those); only cross-ability reads move to the face law.
Unified with the named-memory ruling (done/workbench-named-memory-channels):
no cross-ability discourse anywhere — state for notes, forward-obligation
card linkage for choices.

- **Routed from workbench-amount-ceiling-read (close, 2026-08-26):** Truce / Temporary Truce's tail — "for each card less than two a player draws this way" reads the number NOT chosen against the ceiling; nothing announces it. A chosen-value read, so it lands here.

- **Routed from workbench-chosen-counter-kind (close, 2026-08-26):** the BOARD-READ kind chooser (~13 lines — "choose a kind of counter on target permanent", Bribe Taker's second arm, Crystalline Giant's at-random + exclusion domain) — a chooser POSITION over a counter-kind sort, the biggest thing behind minting one; plus Grimdancer's plural pick with distinctness. Chooser positions are this ticket's, so they land here.

- **Routed from workbench-axis-distributive-reads (close, 2026-08-26):** the numeric-characteristic chosen read — `chosenQualityReadOk Number = False` blocks Celestial Judgment's "a creature with that power" after the `ValueAxis Power` pass binds the value; Hurkyl's "a card of that type" is the card-type twin. Chosen-value reads, so they land here.

- **Routed from workbench-anaphora-mentions-and-creation (split into five sub-tickets, 2026-08-27):** cross-ability memory — Phyrexian Ingester's static line reading "the exiled creature card's power" off its own imprint-style trigger, and Drach'Nyen's twin. §1.3's no-cross-ability-discourse ruling (carried by every anaphora sub-ticket) forecloses threading a mention between abilities; whatever survives is a card-scope FORWARD OBLIGATION under this ticket's own 2026-08-26 ruling (the `EntersChoice → OfChosen` forward-obligation face law). Cross-check: confirm that ruling's shape actually covers this same-card, cross-ability pair before treating either card as delivered by it.

- **Routed from workbench-quality-sort-gaps (close, 2026-08-27):** the chosen-quality consolidation — `SetsChosenBasicType` is now REDUNDANT machinery (minted only because no sort carried the land type; fold it into the general chosen-quality statics — 3 witnesses + 1 ProofsF pin move with it); the `CounterKindSource` bound-kind arm (the single node holding all 13 `CounterKindQ` lines); and the counter-event reader blockers recorded on the `Events.idr` rows (`lookbackSubjectOk` shut at both kinds; no noun names ONE kind). All chosen-value machinery, so they land here.

- **Routed from workbench-anaphora-b-verb-provenance (close, 2026-08-27):** finding 1030's chooser repeatability — recorded on `ChoiceStands` at 12 occ/12 cards, not gated; the marked-read/payer rider question is chooser-position machinery, so it lands here.
