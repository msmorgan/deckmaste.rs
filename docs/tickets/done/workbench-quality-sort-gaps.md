---
needs: []
---
# Quality-sort gaps and the domainless axis pass

Routed from `workbench-axis-distributive-reads` (close, 2026-08-26), which
landed `ForEachKindOf` (value bound as a quality via `kindValueIntro`) and
`ColorPairAxis`/`ExactlyColors`. Its measured remainders, one region:

1. **`QualitySort`'s three gaps** — card type ("choose a card type", 10
   supported), non-creature-host subtype ("choose a land type" 5 + "choose a
   basic land type" 8), counter kind (2). The honest subtype shape is
   `SubtypeQ : CardType -> QualitySort` mirroring `SubtypeAxis`, and
   `sameQEq` then needs a 15×15 `CardType` equality-reflection lemma — the
   recorded cost; weigh it against the 25 supported lines.
2. **`kindAxisSort`'s corresponding `Nothing` cells** become `Just` as each
   sort lands (Magnigoth's `SubtypeAxis Land` value read; Hurkyl's card-type
   read).
3. **The DOMAINLESS distributive pass** (Niv-Mizzet Reborn): "For each color
   pair, choose …" runs over all ten of [CR#105.5]'s pairs, not the values
   present in a group — `ForEachKindOf`'s mandatory domain slot does not
   reach it. An 8th distributive line of a different shape; land it or name
   it at its one-line size with the rule.
4. Celestial Judgment's body read is refused by the standing rule-backed
   `chosenQualityReadOk Number = False` — that read's fate is
   `workbench-choice-chosen-and-ascription`'s, not this ticket's; recorded
   here only so the pass's witness is found when it flips.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`QualitySort`, `kindAxisSort`,
`sameQEq`), `Phrase.idr`, `Cards.idr` bench, `Proofs*.idr`. No Rust crate.

## Acceptance

- Each numbered item lands or ends in a written rule-backed verdict; the
  15×15 lemma cost is paid knowingly or the sort declined on it.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-event-zone-4-zone-catalog-and-reader-payload (close, 2026-08-26):** the counter-pair KIND complement (3 measured lines) is blocked on SORT, not count — the payload is a `CounterKind` and no quality sort carries it. The counter-kind sort is this ticket's item, so the complement lands here with it.

## As-landed

Three sorts landed, one declined; the domainless pass landed; the counter
complement's cells stay shut and the reason is recorded on the table.
Nine files, +355 / −127. `idris/scripts/build` PASS (23/23, 0 errors, 0
warnings). Cites: 0 non-compliant, 0 stale, 26 audited sites read against
their rule text. No design artifact: the docstrings plus this section are
the record.

### The measurement (supported faces only, 2026-08-12 snapshot)

The ticket's counts all survive re-measurement — "choose a card type" 10,
"choose a land type" 5, "choose a basic land type" 8, "choose a kind of
counter" 2 — and three of them were short:

- **"choose a nonbasic land type" 1** (March from Velis Vel) and **"choose
  a planeswalker type" 2** (Deification, Leori). The non-creature subtype
  choice is therefore **16 lines at two hosts**, not 13 at one, which is
  what decided the host parameter.
- **"for each kind of counter" 10** — ~8 of them true distributive passes
  ("for each kind of counter on target permanent, put another counter of
  that kind on it"). The counter-kind sort's motivation is 13 lines, not
  the 2 the ticket carried.
- **The domainless pass is 6 lines, not 1.** Niv-Mizzet Reborn is the only
  "For each color pair"; "For each color," is 3 (All Suns' Dawn, Rogues'
  Gallery, Call the Spirit Dragons) and "For each card type," is 2 (Atraxa,
  Portent of Calamity).

### Item 1 — the three gaps

`QualitySort` is now a GADT: `Color | SubtypeQ CardType | CardName |
Number | CardTypeQ | CounterKindQ`. The `Q` suffix is worn by exactly the
sorts whose bare name is already the type of value they range over.

- **`SubtypeQ : CardType -> QualitySort` — LANDED, and it REPLACES
  `CreatureType`** (71 sites renamed; `SubtypeQ Creature` is the only
  spelling now). Keeping both would have been two names for one thing:
  `TypeOtherThan` already gates its `Subtype` to host Creature, so the old
  nullary sort always meant "a subtype at the creature host".
- **The 15×15 lemma cost was paid, at 15 rows instead of 225.** The
  investigation: equality reflection on an ENUMERATED `Eq` is irreducibly
  quadratic — `(==) Creature b` is stuck on a variable `b`, so every one
  of the fifteen first-argument cases must split the second. The cheaper
  payment re-bases the equality itself: `cardTypeIx : CardType -> Nat`,
  `cardTypeAt : Nat -> Maybe CardType`, the round trip
  `cardTypeAtIx : cardTypeAt (cardTypeIx t) = Just t`, and
  `Eq CardType` as `cardTypeIx a == cardTypeIx b`. `sameCardTypeEq` is
  then five lines over `natEqSo`. Net +26 lines where the hand-written
  table would have been +225, one equality notion rather than two, and the
  instance's meaning is unchanged (concrete arguments reduce as before).
  `natEqSo` moved from `ProofsAnaphora.idr`, where it was unused, rather
  than being duplicated.
- **`CardTypeQ` — LANDED**, 10 lines, `chosenQualityReadOk = True`: a card
  type is one of [CR#109.3]'s characteristics.
- **`CounterKindQ` — LANDED** on 13 lines, with
  `chosenQualityReadOk CounterKindQ = False` and its rule: [CR#122.1]
  makes a counter a marker ON an object and [CR#109.3]'s characteristics
  do not include the markers an object carries, so the object-side read
  matches against nothing. "Of that kind" is the counter slot's read and
  that node is unminted — the sort's 13 lines all wait on it (ledger).
- **The permanent-type sort — DECLINED at 1 supported line** ("choose a
  permanent type"). [CR#110.4]'s six are [CR#205.2a]'s own words under a
  restriction, so the sort would be a second vocabulary over the same
  values. `kindAxisSort PermanentTypeAxis` stays `Nothing`.
- **The card-type choice DOMAIN — DECLINED at 2 lines.** "Choose a card
  type other than creature" (Arachne) and "other than creature or land"
  (Stenn) are both blocked outside the domain anyway, on cost-modification
  statics the tree does not have; and Stenn's needs a LIST where
  `ColorOtherThan`'s shape carries one value. Ledgered.

`ChoiceDomain` gains `BasicTypesOnly` and `NonbasicTypesOnly`, both at
`SubtypeQ Land` — the index fixes the host because [CR#305.6] gives the
basic/nonbasic split to that host alone, the same ground `subtypeScopeOk`
states for `SubtypeAxis`. An unnarrowed "choose a land type" writes no
domain at all.

### Item 2 — `kindAxisSort`'s cells

`CardTypeAxis → Just CardTypeQ`, `CounterKindAxis → Just CounterKindQ`,
and the two subtype rows collapse into one total row,
`SubtypeAxis host _ → Just (SubtypeQ host)` — the host parameter is what
makes Magnigoth's `SubtypeAxis Land` cell open for free. Only
`PermanentTypeAxis` and `ColorPairAxis` stay `Nothing`, each declined
above on its own ground.

**No new WHOLE distributive line benches from these cells, and the reason
is not the sort.** All seven "For each [axis] among [domain]" lines were
inventoried by the parent round: Magnigoth needs landwalk, which the tree
has none of; Hurkyl and Master Wizard need "from among the revealed cards"
and "put the rest … in a random order"; Celestial Judgment's read is the
standing `chosenQualityReadOk Number = False`. The pass fragment was not
benched because there is no printed body to bench it with — substituting
one would be inventing text, not benching it.

### Item 3 — the domainless pass: LANDED

`ForEachKindOf`'s domain became `Maybe (Noun bs Object)` rather than
growing a sibling row: the nine companion tables all match the head with
wildcards, so the optional slot cost no rows at all, and one head with an
optional domain is what the two readings actually are. `kindValueIntro`
gains its `Nothing` arm (the value alone, over the ambient bindings).

Absence is gated by `kindAxisClosed` (`Words.idr`) through
`kindDomainOk` (`Phrase.idr`): a pass with no group to draw values from
runs over the axis's whole value set, so a RULE must close that set.
[CR#105.1] closes the colours at five, [CR#205.2a] enumerates the card
types, [CR#110.4] the permanent types, [CR#105.5] the ten pairs and
[CR#305.6] the five basic land types; nothing closes the other subtype
sets or [CR#122.1]'s counter kinds, and a characteristic's values are
unbounded. Written domainless: colours 3, card types 2. The basic-land,
permanent-type and pair arms stand open at their zero.

**Niv-Mizzet Reborn still does not write**, and the domain was never its
blocker: `kindAxisSort ColorPairAxis = Nothing` because the value is a
PAIR and no quality sort carries one, and the line further needs the
pair-valued read ("a card that's exactly those colors") and "from among
them … in a random order". One line; ledgered.

### Item 5 — the counter-pair complement: cells stay shut, recorded

The routed append had it "blocked on SORT, not count". Re-measured: the
sort was **not the only blocker**. With `CounterKindQ` in hand the cells
still stand shut on two further counts, now written on the table rows in
`Events.idr`: `lookbackSubjectOk` refuses `CounterPlacement` and
`CounterRemoval` at both kinds, so no reader reaches either event; and a
noun at the kind sort spells only "a kind of counter" — `QualityNoun` and
`ChoiceDomain` have no row naming ONE kind, which is exactly what the
three lines write ("a +1/+1 counter was put on…", "an oil counter was
removed from…").

### Bench (`Cards.idr`), oracle text verified against local card data

- **`realmwright`** — whole card. "As this creature enters, choose a basic
  land type. Lands you control are the chosen type in addition to their
  other types." Xenograft's sentence at the land host, benched against the
  already-landed `xenograft` beside it: same two rows, and the only
  difference is the host the sort now carries plus [CR#305.6]'s narrowing
  written as the choice's domain. This is the whole case for the host
  parameter, in one card.
- **`roguesGallery`** — whole card. "For each color, return up to one
  target creature card of that color from your graveyard to your hand."
  The domainless pass's witness; All Suns' Dawn writes the same sentence
  over cards rather than creature cards and wants a bare card noun.
- **`worldQuellerChoice`** — the upkeep trigger's body, "you may choose a
  card type. If you do, each player sacrifices a permanent of their choice
  of that type." The card-type sort's CHOICE and READ in one line.

### Pins

- **`badDomainlessOpenAxis`** (`ProofsG`) — "For each creature type, …":
  the domainless pass at an unclosed axis.
- **`badChosenCounterKindRead`** (`Proofs`) — "a counter of that kind"
  read off the object, [CR#122.1] against [CR#109.3].

`badAxisValueCrossing` still fails as it must: `ColorAxis` cannot bind a
`SubtypeQ Creature`.

## Ledger — needs routing to live planned tickets

1. **`SetsChosenBasicType` is now redundant machinery.** It is the bespoke
   static the missing sort forced — "target land becomes the basic land
   type of your choice" with the sort and the choice both baked in. With
   `SubtypeQ Land` plus `BasicTypesOnly` it should fold into the general
   chosen-quality statics. 3 witnesses (Reef Shaman, Grixis Illusionist,
   Unstable Frontier) and one pin (`ProofsF`) move with it.
2. **The counter kind's bound read.** `CounterKindSource` has no arm
   reading a chosen or pass-bound kind, which is the single node holding
   up all 13 of `CounterKindQ`'s lines: ~8 distributive ("for each kind of
   counter on target permanent, put another counter of that kind on it" —
   Quarry Hauler, Dramatist's Puppet, Powerful Broker), 2 choice
   (Contractual Safeguard, Crystalline Giant, the latter also wanting an
   at-random choice from a printed menu), 3 complement.
3. **The counter-pair complement's other two blockers** (item 5 above):
   `lookbackSubjectOk CounterPlacement/CounterRemoval` at Object, and a
   predicate naming ONE counter kind at `Quality CounterKindQ`.
4. **The colour-pair quality sort** — Niv-Mizzet Reborn, 1 line, plus the
   pair-valued read and "from among them … in a random order".
5. **The card-type choice domain** — Arachne and Stenn, 2 lines, needing a
   list-valued exclusion and cost-modification statics.
6. **Magnigoth Treefolk** — its axis cell is open now; landwalk is not.
7. **Hurkyl, Master Wizard / Atraxa, Grand Unifier / Portent of
   Calamity** — the card-type cells are open; "from among the revealed
   cards" and "the rest … in a random order" are not.
8. **All Suns' Dawn** — the domainless pass's second whole line; wants a
   bare card noun in a graveyard and "Exile ~".
