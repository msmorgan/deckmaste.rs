# choice-D: the literal colour, the payload cells, and the quantifier

Sub-round D of [workbench-choice-chosen-and-ascription](workbench-choice-chosen-and-ascription.md)
(the umbrella). Owns its sections "The literal colour change and the
ascription rows' last payloads", "The type addition's payload and surface
cells", and "The type quantifier and the ascription rows' leftovers", plus
routed: Aisling Leprechaun ("that creature becomes green") and Master
Biomancer's coordinated entry-time addition.

Pins:
- The literal colour change is one row per operation at layer 5, modeled on
  the chosen-quality setting/adding rows — never a second payload on the type
  line.
- The battlefield-subject demand is decided deliberately: kept with Ashes of
  the Fallen recorded against it, or widened on stated evidence.
- `TokenRider` is indexed over `StatusVal` (closing the flip verb's gap from
  the arrival side); Yedora Grave Gardener benches.
- Indigo Faerie lands via a bundle-level "says something" gate WITHOUT
  dropping what the three line-level gates refuse.
- The type-loss/ability-loss coordination is decided ONE way and recorded
  before either row is minted.
- The ordering sweep returns a recorded verdict — free variation or hole.
- Read `BecomesAlso`'s docstring (finding 1123) before touching either
  deliberately-left surface cell; Navigator's Compass stays a recorded
  absence.

Acceptance: the umbrella's lines for the colour change, Ashes, Yedora, Indigo
Faerie, the ordering sweep, the 6 loss sentences, Energybending and the
choose-two lines. `idris/scripts/build` PASS. Standard constraints apply.

Routed from choice-C (close, 2026-08-27): the TOKEN SPEC's chosen-quality
cell — `TokenChars` cannot carry a chosen quality where a literal one stands;
payload territory, so it lands here.

## As landed

Nine files, +792 / -20. `idris/scripts/build` PASS (23/23 from a clean
`build/`, 0 errors, 0 warnings). Cites: 0 non-compliant, 18,358
checked / 0 stale (18,375 after this section), 50 audited sites read
against their rule text, 1 rule
newly blessed ([CR#708.3]). No design artifact: the docstrings plus this
section are the record.

### Premise corrections, up front

- **`BecomesAlso` has no docstring.** Finding 1123's record is not there and
  never was; what carries it is the closure table's row
  (`idris-workbench-closure-tables.md` §2.5, `Experimental.idr:2780`), which
  names both surface cells — "the colour-only addition (2 lines, 1 ledgered
  residue = Indigo Faerie) is unmodelled" and "TokenPt/TokenTyped/
  type-ordering do NOT ride". Read before either cell was touched; the row
  now carries a docstring of its own.
- **There is no type-ORDERING demand anywhere in the grammar.**
  `tokenCanonical` is `colorsDistinct && typesDistinct` — DISTINCTNESS, not
  order. The umbrella's "type-ORDERING half" does not exist. What was really
  missing on the addition row was the distinctness half: it asked
  `ColorsDistinct added.colors` where the setting asks the whole of
  `TokenCanonical`. Verdict below.
- **The LOSS cell is 7 sentences over 7 cards, not 6.** "Ultima, Origin of
  Oblivion" is ONE card, which the umbrella reads as two names. Measured:
  4 creature-type lines (Curse of Conformity, Ego Erasure, Nameless
  Inversion, Amoeboid Changeling) + 3 land-type (Alpine Moon, Lithoform
  Blight, Ultima Origin of Oblivion).
- **"Is all colors" is 8 supported lines over 8 cards, not 1.** Leyline of
  the Guildpact is one of them; the others are Transguild Courier, Sphinx of
  the Guildpact, Fallaji Wayfarer, Invasion of Alara, The Kami War,
  Scrapbasket and Tam, Mindful First-Year. Measured against the colour
  family as instructed, and 8 lines is a payload arm's worth — landed.
- **The choose-two lines are 1 supported line, not 2** (Illusionary Terrain).
- **Maskwood Nexus's first line is already benched** (`maskwoodNexusTypes`).
  The `AlsoOffBattlefield` "two further nouns" subject is not a gap at all:
  Arcane Adaptation and Conspiracy already bench the identical printed
  sentence. Maskwood's whole card waits on the Changeling keyword, which
  `KnownKeyword` does not carry.
- **The `Cyberman` catalog word costs nothing.** Subtypes have been labels
  since `workbench-subtype-labels`; `creatureType "Cyberman"` is writable
  today and there is no cell to open (sub-round E found the same premise
  stale).
- **The base-P/T rider coordination LANDS FREE**, so the ledger entry closes
  rather than surviving. `AndAlso` already spells two statics inside one
  sentence — Darksteel Mutation writes exactly that shape at the setting —
  and both carriers elaborated first attempt (below).
- **The literal colour-setting family is 52 supported lines over 51 cards**
  by this round's sweep, against the umbrella's 42.

### The literal colour change — `SetsColor`, one row for the setting

`SetsColor : (n : Noun bs Object) -> (cs : ColorSpec) -> …`, [CR#613.1e]'s
layer 5, modeled on `SetsChosenQuality`. ONE row per operation, and the
operations divide as the pin asked: the SETTING gets the new row, the ADDING
one is `BecomesAlso`, which already exists and needed only its gate widened
(next section). The asymmetry is the operations' own and is written in the
row's docstring — a setting that named no type would say the subject's types
are replaced by none, where an addition of no type adds none and the sentence
still means what it says. That is also why the colour-only addition, at ONE
printed line (Indigo Faerie), buys no second row: the same rule that keeps
Navigator's Compass unrowed.

`ColorSpec = SomeColors (List Color) | EveryColor`. The list is the token
bundle's own colour payload, so the empty list is "colorless" [CR#105.2c]
exactly as it is on a written token, and `EveryColor` is the printed
quantifier "all colors" — two arms rather than a five-way enumeration, for
`AddsEveryType`'s own reason, even though [CR#105.1] closes the space where a
subtype set is open.

**No subject-zone demand**, and the evidence is not one line but eight: the
lace cycle and Ersatz Gnomes set the colour of a SPELL. Same ground as the
chosen-quality widening below.

**Benched (11 carriers):** `aislingLeprechaun` (whole — the routed carrier,
on Inferno Elemental's coordinated header, the tail reading the creature the
block paired it with), `darkestHour` (whole), `thranLens` (whole — the empty
list as "colorless"), `ghostflameSliver` (whole), `sinisterStrength` (whole —
the copular spelling coordinated with a pump on the attach host),
`crimsonWisps` (whole — the inchoative with a duration, coordinated with a
keyword grant on one target), `nightcreep` (whole — the colour setting and a
type setting under one duration, two layers in one sentence),
`ersatzGnomes` (whole — the stack subject), `celestialDawnAscriptions` (its
two ascription lines, the literal colour setting under the off-battlefield
extension, which is what that extension was waiting for),
`transguildCourier` and `scrapbasket` (whole — `EveryColor`, standing and
durated).

**Zeros, each with its own cause:** the lace cycle's six "target spell or
permanent" lines are refused by `parallelDisjuncts` — probed, `So
(parallelDisjuncts [spell, Permanent])` has no implementation, the two seeds
sitting in different zones. That is a UNION-subject cell and not this row's.
Mycosynth Lattice and Painter's Servant want the everywhere subject ("All
cards that aren't on the battlefield, spells, and permanents"), the same
cell one step wider. Celestial Dawn's whole card wants the two
mana-spending permissions (`PlayAsThough` is `HadFlash` and nothing else,
sub-round E's remainder).

### `BecomesAlso`'s bundle gate, and the ordering sweep's verdict

`AdditionSaysSomething` replaces `LineNonEmpty` + `AddsSomething` and is
asked of the BUNDLE. Nothing the three line-level gates refused is dropped:
`AddedFits` stands unchanged, a line that is WRITTEN must still add a type
the subject lacks, and the only bundle admitted without one is a bundle that
writes a COLOUR. A P/T or a with-clause ability alone is still refused —
[CR#613.4b]'s base P/T is `HasBasePt`'s statement and a granted ability is
`Gains`', and neither is printed without a type word in an addition sentence.
`badBecomesNothing` moves to the new gate and still refuses; it is the pin
that proves the widening added one admission and no more.

**Ordering sweep — VERDICT: a HOLE, and closed.** No ordering demand exists
to sweep. The distinctness one does, and the addition row was not asked it:
`TokenCanonical` now stands where `ColorsDistinct` did, which is a strictly
stronger gate and one fewer argument. Measured zero: no supported addition
line repeats a card type or a colour. New pin `badRepeatedAdditionType`
("becomes an artifact artifact"); `badRepeatedAdditionColor` moves to the
same gate.

**Benched:** `indigoFaerie` (whole — the colour-only addition),
`ghoulflesh` (whole) and `bladeOfTheOniStatic` (the base-P/T rider, a keyword
grant and the addition coordinated in one sentence — the 19 lines' shape,
and the ledger entry it closes).

### `TokenRider` over `StatusVal`

    data TokenRider : Type where
      EntersAs : {0 c : StatusCat} -> (v : StatusVal c) ->
                 {auto 0 at : StatusEffectVal v} -> TokenRider
      EntersAttacking : TokenRider

with `EntersTapped = EntersAs Tapped`, so every existing rider site is
unchanged (7 direct, plus the four `Macros` helpers that write them). The gate is `SetStatus`' own (`StatusEffectVal`, [CR#710.4]
refusing `Unflipped`), which is what makes this the flip verb's vocabulary
rather than a second one. [CR#708.3] turns an object face down BEFORE it
enters, so the word is a rider on the arrival and not a second instruction;
`EntersAttacking` stays its own row because attacking is a combat position no
`StatusVal` denotes ([CR#506.3a,508.4d]).

**Benched:** `yedoraGraveGardener`, whole card, first attempt — the arrival
rider `EntersAs FaceDown` plus the type line [CR#708.2a]'s "unless otherwise
specified" licenses. And `missyFaceDownReturn`, which is the vocabulary's
point: TWO riders in one arrival, `[EntersAs FaceDown, EntersTapped]`, with
the `Cyberman` word costing nothing. The remaining five lines all write the
same rider and are blocked past it — Magar notes a card name and grants two
quoted abilities; Tezzeret's is a loyalty ability over "any number of cards
from your hand"; Cybership needs crew, Death in Heaven a Saga chapter over
"creature cards exiled with this enchantment", The Cyber-Controller a milled
group read. None of the five is blocked at the rider any more.

### The type-loss pole — `LosesEveryType`, and the coordination decided

`LosesEveryType : (n : Noun bs Object) -> (space : TypeSpace) -> …`, the
negative pole of `AddsEveryType` with the same payload and the same host
gate.

**The type-loss/ability-loss coordination is TWO rows, coordinated by
`AndAlso`** — decided before either row was minted, and the reason is the
layers: [CR#613.1d] applies the type loss at layer 4 and [CR#613.1f] the
ability loss at layer 6, so "loses all land types and abilities" is two
statements in one sentence and not one statement with a rider.
`LosesAllAbilities` already writes the second and needed nothing. The
alternative — a Bool rider on the loss row — would have duplicated a landed
row at a different layer. Measured covariance: all 3 land-type lines
coordinate, all 4 creature-type lines write the loss alone, so neither
spelling is the other's default.

**Benched:** `amoeboidChangelingTypeAbilities` (the quantifier's two poles
printed on one card — the minimal pair the row is paid for by),
`namelessInversionBody`, `egoErasureBody`, `curseOfConformity` (whole card —
the standing, undurated loss at the enchanted player's creatures), and
`lithoformBlightLoss` (the coordination itself). The three Changeling cards
do not bench whole: `KnownKeyword` has no Changeling. Alpine Moon's line
additionally wants a granted mana ability and a chosen-name-scoped land
filter.

### The battlefield-subject demand — WIDENED, on stated evidence

Dropped from `AddsChosenQuality` and `SetsChosenQuality` only. The layers
apply to an OBJECT's characteristics [CR#613.1] and [CR#109.1] makes a card
an object, naming no zone, so nothing makes a graveyard card's type
unchangeable and the demand refused printed text. What carries the meaning is
`HostedRead` ([CR#205.3d], landed in sub-round B), and it asks the subject's
TYPE, which a zone does not decide. Recorded overgeneration: a library or
hand subject, neither printed. The type-line rows (`SetsType`,
`BecomesAlso`, `AddsEveryType`, `LosesAllAbilities`) KEEP the demand — no
printed line asks them to drop it, and the evidence is specific to the two
rows that lost it.

**Benched:** `ashesOfTheFallen`, whole card, first attempt.

### Recorded, not built

- **Navigator's Compass** stays a recorded absence: 1 line, and one line does
  not buy a second row. Re-measured at 1.
- **Omo, Queen of Vesuva** stays witnessed by nothing: "every land type" is
  still 1 supported line and the everything counter has not fallen.
- **The choose-two lines are REFUSED, with the count.** 1 supported line —
  Illusionary Terrain, "As this enchantment enters, choose two basic land
  types. / Basic lands of the first chosen type are the second chosen type."
  The chooser slot is cheap (sub-round B priced it); the READ is an ORDINAL
  over a chosen pair ("the first chosen type", "the second chosen type"),
  which is the class B declined with the shape written down (Bitter Feud's
  "one of the chosen players … the other chosen player"). One line does not
  buy an ordinal partitive.
- **The becomes-a-creature-WITH cell** (Mutavault, Faceless Haven, Soulstone
  Sanctuary, 3 lines) is not built and the cost is why: the quantifier would
  be a SIXTH field on `TokenChars`, and `MkToken` is written positionally at
  every construction site in the grammar. The coordination workaround
  (`AndAlso [SetsType …, AddsEveryType It CreatureSpace]`) moves a printed
  with-clause constituent out of the bundle it is printed in, which is a
  spelling deviation, not a saving. Ledgered with that price.
- **The token spec's chosen-quality cell** (routed from C; Volrath's
  Laboratory, Riptide Replicator, 2 lines) is the SAME payload question at
  the same bundle — `TokenChars.colors` is a literal `List Color` and
  `.line` a literal `TypeLine`, with nowhere for a read to sit — and it
  carries the same price. Recorded together with the cell above: they land
  or fail together, and neither is worth a positional-field break at 3 and 2
  lines.
- **Master Biomancer** is not built. "Each other creature you control enters
  with … +1/+1 counters on it AND as a Mutant in addition to its other
  types" wants an entry-time type ascription, and no row writes one:
  `EntersRider` carries a `TokenRider` and nothing else, and `BecomesAlso`
  is not an entry replacement. That is a new row at 1 line, not a payload
  cell. Routed.
- **Leyline of Transformation's opening-hand permission is ROUTED, not
  built.** 17 supported cards write "If this card is in your opening hand,
  you may begin the game with it on the battlefield" — real machinery (a
  pregame procedure whose subject is a card in the opening hand, before any
  zone this grammar's nouns range over), not a small carrier. Its ascription
  line already writes.

### Closure cells this round moved

Named as the umbrella asks, so the grid is re-read rather than defaulted:
`BecomesAlso`'s widening column ("the colour-only addition … is unmodelled"
— now modelled, 1 line) and its distinctness gate; `AddsChosenQuality`/
`SetsChosenQuality`'s subject-zone demand; `TokenRider`'s two-value catalog.
Two new rows (`SetsColor`, `LosesEveryType`) and one new catalog
(`ColorSpec`, 2 rows) want rows of their own. The tables' §2.5 line anchors
are stale independently of this round (they point into the pre-split
`Experimental.idr`), and sub-round B's `SetsChosenBasicType` row is stale
there too.
