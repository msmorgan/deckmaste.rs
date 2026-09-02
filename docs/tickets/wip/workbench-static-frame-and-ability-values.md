---
needs: []
---
# Finish the static statement frame and buy abilities as values

Two halves of one region. The static line's own frame still owes a per-part
span, a second statement and a turn window; and the payloads that frame carries
— described ability sets, ability nouns, quoted abilities that name their own
grantor, marker-object self-ascription — are all the same abilities-as-values
axis. The frame's carriers and the axis's payloads are read by the same rows, so
they are one claimable unit.

## The static line's span and second statement

What the static coordination round left: the carrier LANDED (`AndAlso` over
`StaticParts`, 1,433 supported sentences over 1,407 cards, 36 signatures,
findings 609–612) with ONE envelope covering every part. Three counted classes
want more than that envelope, and all three are span-or-carrier questions in the
same region.

- **The per-part duration**, 45 lines needing two spans. 41 are "<grant> until
  end of turn and can't be blocked this turn" (Distortion Strike, Taigam's
  Strike, Teleportal, Marchesa's Smuggler…) and six write the same word twice
  (the Mimic cycle — "has base power and toughness 4/2 until end of turn and
  gains first strike until end of turn"). These are NOT counterexamples to the
  envelope: it is the span table refusing to let a grant and a restriction share
  a current-turn word (finding 613, `badCoordinatedSpanDisagree`). The shape
  wanted is a span slot per part, not a second carrier. **Measure the
  SAME-word-twice six first** — if those six are a spelling variant of the
  shared envelope, the family shrinks to one coherent grant-plus-restriction
  class and the design is a per-part `Maybe Duration` with the envelope kept as
  the elided form.
- **The multi-sentence static line**, 11 supported lines (finding 620). Two
  sentences on one static ability, the second reading the first's subject back:
  "Equipped creature gets +2/+0. It gets an additional +0/+2 and has first
  strike as long as …" (Bride's Gown, Groom's Finery), "Enchanted creature can't
  attack or block. It loses all abilities and has '{T}: …'" (Heliod's
  Punishment), Retro-Mutation, Spider-Man No More, Intercessor's Arrest, Lost in
  Thought, Volrath's Curse, Shuriken. `Static : StaticEffect [] -> Ability`
  holds ONE statement, so this is a carrier question and not a mention one — the
  announcement is already there; what is missing is a place for the second
  sentence to stand. Its shape is `Effects`' and `StaticParts`' a third time (a
  telescope of statements typed in what their predecessors announced), the
  difference from `AndAlso` being the SPELLING: a full stop and a written
  subject where the coordination writes "and" and elides. Measure whether the
  eleven are one class before choosing between a second carrier and a marking on
  this one.
- **The compound subject**, 3 lines: "Gogo and that creature each get +2/+0 and
  gain haste until end of turn", "this creature and those creatures get +1/+1
  and have vigilance", "the chosen creatures get +X/+X and gain trample". Found
  while measuring the coordination's subject elision (finding 611). The "and" is
  inside the NOUN PHRASE and the distributive "each" marks it, so this is a noun
  question belonging with the group vocabulary — recorded here so the next
  reader of finding 611's 1,432-of-1,433 does not mistake these for crossings.
- **The GAME-SPANNING duration**, and it is a SPELLING rather than a `Duration`
  row. "For the rest of the game" is 55 lines, 41 of them the reminder text of
  Ascend (26), Storied (9) and Epic (5). The 14 that remain are the
  no-maximum-hand-size clause (7), the goad rider (4), Cyclopean Tomb's delayed
  trigger, and Screaming Nemesis and Stigma Lasher. ALL FOURTEEN ARE RESOLUTION
  CLAUSES and no printed static line writes the phrase, [CR#611.3b] already
  making a line last as long as its source, so finding 275's covariance holds
  perfectly. Acting on it means flipping `absentOk` for whichever kind; that was
  declined once because neither carrier could be benched. Settle it for all four
  constructions at once.

## The static's described ability payload and its turn window

Three measured gaps in the statement frame: a grant whose payload is a
*described* set of abilities rather than a quoted one, a loss whose exception
names a *class* of abilities, and the temporal window a statement has no slot
for. The first two are the same question about abilities-as-values; the third is
the frame's missing third reader.

### The ability-sharing static — 2 cards, both blocked the same way

"Each other planeswalker you control has the loyalty abilities of Kasmina"
(Kasmina, Enigma Sage) and "Nicol Bolas has all loyalty abilities of all other
planeswalkers on the battlefield" (Nicol Bolas, Dragon-God). A `Gains` whose
payload is named **by description** rather than quoted — the ledgered by-name
direction ([CR#201.5a], the grantor named from inside the quotation) meeting the
loyalty frame from the other side. **Whichever round takes ability-as-a-value
must take both cards together; neither is writable alone.**

### The mana exception — 1 line, recorded and not scheduled

"lose all abilities except mana abilities" is Blood Sun alone, one line with no
neighbours: the exception **names a class**, where the already-landed "other"
named a position (the scope word is a derived spelling and no slot at all). One
line is under the minting bar, so this **waits for a second card rather than for
a design** — recorded at its count so it is not re-discovered.

Nothing else is owed on the ability loss's variants: the eleven "all other" lines
are spelled by the coordination's part order, and Darksteel Mutation is benched
whole.

### The static's turn window — 2 lines

"During your turn, this creature has first strike" — the gap that took Restless
Spire off the witness list once the widening had made its carrier writable. **A
statement carries no window slot**: `windowOk`'s grid is the activated ability's
timing and `TriggerWindow`'s is the trigger's, and the third reader has never
been asked. Restless Spire is the named witness (its mana ability elides as every
animated land's does); "During your turn, outlaws you control have first strike"
(At Knifepoint) is the same word on a non-animated line.

### Not this round's, and not work — the one-time boon, measured at its zero

"You get a one-time boon with '…'" is written by **27 cards and none of them is
supported**: every one is Alchemy ("perpetually", "seek a nonland card"), which
is the supported flag doing its job. Declined until the flag says otherwise, and
recorded here only so the zero is not re-discovered as a gap. Its shapes, for
whoever inherits a changed flag: one-time, two-time, three-time and bare
(Merfolk Tunnel-Guide, Tasteful Offering, Swiftspear's Teachings, Flaming Fist
Duskguard).

## The ability noun and the borrowed-ability grant

Two gaps over the same missing vocabulary. An ability on the stack is an object
([CR#109.1], [CR#113.7a]) that the noun vocabulary has no head word for, and the
grant payload "all activated abilities of [that card]" has no term either. This
is a NOUN round and not a verb one — both consuming verbs are already shaped.

- **Ability borrowing**, 2 cards one line from whole. CONSPICUOUS SNOOP ("as
  long as the top card of your library is a Goblin card, this creature has all
  activated abilities of that card") and SKILL BORROWER (the same sentence over
  an artifact or creature card). Chapter 116 landed both cards' other lines —
  the visibility rider and a top-of-library permission apiece — and the top-card
  condition probed green, so the gap is exactly "has all activated abilities of
  [that card]".
- **The ability on the stack**, 60 supported sentences over two verbs, ledgered
  on the `CounterSpell` noun since chapter 28: "counter target activated or
  triggered ability" 29, "copy target activated or triggered ability you
  control" 31. It is not a card, so `CardW` does not reach it, and it is not a
  spell, so the stack's carrier noun [CR#112.1] does not either. Both verbs'
  complement slots are already the ordinary stack noun, so nothing about either
  row changes when the word lands.
- **Measure the description vocabulary before minting anything.** The corpus
  writes only narrow descriptions: "activated or triggered", "activated",
  "triggered", plus source restrictions ("from an artifact source", "from a
  creature source").
- Cross-count from the prohibition side: the targeted ability is 35 lines
  ("counter target activated ability"), an ability on the stack and therefore an
  object [CR#109.1]. Chapter 122 minted `Kind.Ability` for the ability-CLASS
  subject and deliberately did not mint this object-sorted head; the 35 are this
  round's, not that one's.

## The quoted ability's carrier residues

Three residues of the ability-as-a-value axis, all in the quoted-payload
region: the word a token or emblem uses to call itself one, the reference a
quoted ability uses to name its own grantor, and one measured cell recorded
below the evidence bar. The first two are the axis's last unbought pieces.

### The marker object's self-ascription — one gap, two carriers

`TokenChars.abilities` and `GetsEmblem` both hold whole abilities already, and
both families keep most of their spans behind a word `AsType` cannot give:
`AsType` ascribes a card type or one of nine subtypes ([CR#109.2]'s "card type
or subtype"), while [CR#111.1] makes "token" the marker word and [CR#114.3]
leaves an emblem no types at all.

- THIS TOKEN: 193 of the 239 token-creation spans.
- THIS EMBLEM: 10 of the 90 emblem payloads (Chandra ×4, Karn, Koth, Narset,
  Ral).
- It is a THIRD axis on `AsType`, not a row on an existing one — which is why
  two prior chapters recorded it rather than buying it.

Whoever takes it inherits the whole creation carrier by name: Anax, Hardened in
the Forge; Mesmerizing Benthid; Harried Spearguard; Wolf's Quarry; Nesting
Dragon; Reef Worm — all waiting on this word and on nothing else in the
payload.

### The grantor named from inside the quotation

[CR#201.5a] is its rule. 47 quoted payloads name a card: Gutter Grime's "This
token's power and toughness are each equal to the number of slime counters on
Gutter Grime", and every Equipment that names itself in the ability it grants
(Leonin Bola, Heartseeker, Blazing Torch, Hankyu). The rule fixes the name to
"the specific object which is that first ability's source", where `Named`
describes a CLASS. A denotation gap, not a spelling one, so it wants a
reference row and not a predicate. Gutter Grime is the cheapest whole card.

### The granted ability's subject in a non-battlefield zone — RECORDED, do not build

FOUR supported spans, not the 40 an earlier sweep reported: that sweep read a
whole clause segment where it should have read the subject NP, so every
"Threshold — As long as there are seven or more cards in your graveyard, THIS
CREATURE has '…'" and every "{2}, Exile this card from your hand: TARGET LAND
gains '…'" was counted as a non-battlefield subject when the zone word sits in
the condition or the cost.

The real four: Case of the Uneaten Feast, Kethis, the Hidden Hand and The Grim
Captain's Locker grant to cards in a GRAVEYARD; Lukka to cards EXILED this way.
Hand and library are ZERO. That is under the corpus bar, so this waits for a
fifth card rather than for a design — do not mint a row for it in this round.
Recorded because three of the four payloads are play permissions `MayPlay`
already spells (it is the SUBJECT that refuses) and because [CR#113.6b] is what
those payloads say about themselves.

Counts here are a prior session's measurements; re-measure before building.

## The closure grid this family owns

Ranked below the workbench's fifteen top flip risks — the grid's many zeros are
independent measurements, so a printing moves one cell and the closure survives —
but every cell this family moves is one of them. A widening names the cell it
moved and re-reads the grid rather than defaulting it, and a cell left closed
says whether a rule or a count is closing it. Rows, evidence and widening costs:
[the closure tables](../../idris-workbench-closure-tables.md) §2.3.

`AndAlso`'s 1,433 sentences (`Experimental.idr:2888`) — the largest population
in the workbench with **no CR anchor at all**, so nothing but the count closes
it.

## Measured cell (2026-08-21)

- `attachHeadOk Equipped PermanentW = False` refuses an attested line: "equipped permanent" 1 of 627 "equipped" lines (Luxior, Giada's Gift); "equipped planeswalker" 0; "enchanted permanent" 76. Flip it or pin it as a deliberate single-witness refusal — today it is unmarked.

## Consumption boundary

`idris/src/Experimental.idr` (`Static`, `StaticParts`, `AndAlso`, the span table
and `absentOk`; `Gains` and its payload, the ability-loss exception, the
statement frame, `windowOk`, `TriggerWindow`; the grant payload; `AsType`,
`GetsEmblem`, the reference rows), `idris/src/Experimental/Words.idr` (the
compound subject's noun phrase; the ability head word and its description
vocabulary; the marker word and its gate),
`idris/src/Experimental/Events.idr` if the window shares the span vocabulary,
`idris/src/Experimental/Macros.idr` (`TokenChars`), the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- The same-word-twice six are measured and classed BEFORE the design is fixed;
  the verdict is recorded either way.
- `badCoordinatedSpanDisagree` still refuses genuine disagreement after a
  per-part span exists.
- The eleven multi-sentence lines write, subject readback intact.
- The described payload elaborates for both Kasmina and Nicol Bolas, or neither
  lands.
- The statement's window is a third reader of the timing vocabulary, not a copy
  of the activated ability's grid.
- The mana exception is left refused with its count recorded, unless a second
  card has appeared.
- The 60 stack-ability sentences write; Conspicuous Snoop and Skill Borrower
  bench whole.
- The description vocabulary admits the measured surfaces and nothing wider.
- The marker word is a third axis on `AsType`, and an emblem's type absence
  [CR#114.3] stays a measured fact rather than an ascription of nothing.
- The grantor reference denotes the ability's source object, and is not
  expressed as a `Named` class predicate.
- The four non-battlefield subject spans and the hand/library zeros are carried
  as measurements, with no row minted for them.
- The named cards above bench whole.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-anaphora-e-partitive-surfaces (close, 2026-08-27):** the "play with [slice] revealed" STATIC has no row ([CR#401.5,401.6] — the rules define the revealed-top state; Field of Dreams, Lantern of Insight, Wizened Snitches spell their slices and wait on it). A static row, so it lands here.

- **Routed from workbench-choice-c-chooser-positions (close, 2026-08-27):**
  Koh the Face Stealer's whole-ability-set grant (gaining all activated
  abilities of the chosen card) — an ability-values grant; the chooser side
  landed in choice-C.

- **Routed from workbench-counted-anaphora-narrowings (close, 2026-08-27):**
  no lose-one-NAMED-ability row exists (`LosesAllAbilities` is the only loss
  shape) — Blind Fury's last blocker. Ability-values machinery, so it lands
  here.

---

## As landed (2026-09-02)

Every count below was re-measured against `data/derived/cards.jsonl` under
`jq 'select(.supported)'`. Several of the ticket's numbers and premises were
stale; the corrections are stated at the cell.

### Built

- **The described ability-set grant** — `GainsAbilitiesOf`
  (`Effect.idr`). **29 supported lines** ("this creature has all activated
  abilities of that card", "Nicol Bolas has all loyalty abilities of all
  other planeswalkers on the battlefield"). The description vocabulary is
  `AbilityClass`' own, already minted for the ability on the stack: 26 write
  "all activated", 2 "all activated and triggered", 2 the loyalty class. The
  class slot is a LIST; the EXCEPTION slot is a `Predicate` over `Ability`,
  so Sharkey, Tyrant of the Shire's "except mana abilities" ([CR#605.1a]'s
  derived property, `IsManaAbility`) and Scheming Fence's "except for
  loyalty abilities" (`AbilityHead LoyaltyClass`) are one slot. No
  subject-zone demand ([CR#113.6b]).
  Bench: **Conspicuous Snoop and Skill Borrower whole**; Kasmina, Nicol
  Bolas Dragon-God, Myr Welder and Sharkey's lines.
- **`testSubjectOk` admits a library slice** as a condition's subject
  (`Phrase.idr`), for the `Definite` row's own stated reason — `nounDelta`
  mints its mention at `TheD` and [CR#401.2] keeps a library in one ordered
  pile. This is what let Snoop and Borrower bench whole ("as long as the top
  card of your library is a Goblin card, this creature has all activated
  abilities of THAT CARD"): the read is of what the condition's subject
  announced. **Flagged deviation** — a condition-vocabulary cell moved to
  buy two of this ticket's named cards.
- **The named-ability loss** — `LosesAbilities` (`Effect.idr`), [CR#613.1f]'s
  layer 6 and `Gains`' mirror. **76 supported lines over 76 cards**
  (measured; "loses flying" 32). The payload is a `List (AbilityAt bs)`, so
  one row carries the bare keyword, the parameterised one ("protection from
  black", Cephalid Snitch) and the quoted ability. Bench: **Blind Fury
  whole** (routed item), Shadowspear's line.
- **The ability-loss EXCEPTION** — `LosesAllAbilities` gained a `Maybe
  (Predicate … Ability)`. Blood Sun is still the corpus's **only** loss-side
  exception; the slot is bought by the GRANT's two lines and REUSED here
  rather than minted for one line. Bench: **Blood Sun whole**.
- **The marker object's self-ascription** — `AsMarker` (`Phrase.idr`) over
  `MarkerWord` (`Words.idr`), a THIRD ascription axis: [CR#111.1]'s token
  and [CR#114.3]'s emblem name no type, so neither can ride `ascriptionOk`,
  whose whole content is [CR#109.2]'s "card type or subtype". Zones are the
  rules' own ([CR#111.1] battlefield, [CR#114.2] command). **102 of the 206
  distinct quoted token-creation payloads write "this token"; 9 of the 90
  distinct emblem payloads write "this emblem"** (the ledger's 193/239 and
  10/90 were a different span count). Bench: Nesting Dragon's inner token
  payload, Chandra, Awakened Inferno's emblem.
- **The grantor named from inside the quotation** — `TheGrantor`
  (`Phrase.idr`), [CR#201.5a]'s reference. `This` cannot say it (inside a
  granted ability `This` is the granted-to object) and `Named` cannot
  (that describes a class). **49 quoted payloads over 48 supported cards.**
  Its zone is the battlefield ([CR#113.6]); an emblem grantor would be the
  widening and no supported line writes one. Bench: **Leonin Bola whole**.
- **The per-part span** — `SubjectVP`'s arms carry their own
  `Maybe Duration`, and a third arm `VPDeontic` spells `Deontic`'s slots
  minus the subject. **51 supported lines** coordinate a grant written
  "until end of turn" with a restriction written "this turn" (49 in that
  order, 2 reversed) — two different `Duration` values, which
  `Continuously`'s single envelope cannot write. The envelope stays as the
  elided form. `vpOk` gained the subject's `nounTy` so the arm can ask
  `Deontic`'s participant demands. Bench: Distortion Strike's line.

### Verdicts recorded, nothing built

- **The same-word-twice class is a spelling variant of the shared
  envelope** — measured BEFORE the design was fixed, as the acceptance
  requires. **Four** lines write the identical current-turn word twice on one
  coordination (the Mimic cycle's flying/first-strike/trample/wither
  members), plus Sylvan Awakening's land-copy line; the fifth Mimic
  (Riverfall) writes the can't-be-blocked variant and belongs to the
  DISAGREEING family. Both written spans being the same `Duration` value,
  the envelope already says what the line says. **Battlegate Mimic benches
  whole on the envelope, unchanged.** The family that actually needed a
  per-part span is therefore the 51 grant-plus-restriction lines alone.
- **The multi-sentence static line needs no carrier.** **11 supported
  lines** (re-measured; the 12 Zendikon-style "It's still a land" lines are
  excluded — `SetsType`'s retention slot spells them). They are **not one
  class**: 6 are a plain further statement, 3 write a REPLACING second
  statement ("It gets +3/+1 INSTEAD as long as …" — Mind Carver,
  Precipitous Drop, So Tiny), 2 give the subject's controller leave to
  ignore the effect (Lost in Thought, Volrath's Curse). `AndAlso` already
  coordinates whole statements with later ones reading an earlier back, so
  the full stop is SPELLING — Lithoform Blight decided the same question the
  same way a round earlier. Bench: **Retro-Mutation whole**. The
  instead/additional marking (3 lines) and the ignore-this-effect permission
  (2 lines) are their own cells, recorded at their counts.
- **`badCoordinatedSpanDisagree` no longer exists**, and neither do
  `CoordSpanOk`/`coordSpanOk`/`partsAdmit`. The pins round retired them
  before this one. The acceptance line "still refuses genuine disagreement"
  has no gate left to hold; the disagreement is now WRITABLE, which is what
  the printed lines wanted.
- **The GAME-SPANNING duration is settled at zero and wanted no flip.**
  `absentOk` does not exist; `SpanUnstated` is unconditional in the
  `StaticKind` ([CR#611.2a]) and `RestOfGame` is an ordinary row. "For the
  rest of the game" is 55 supported lines, 41 of them Ascend/Storied/Epic
  reminder text; all 14 of the rest are resolution clauses and no printed
  static line writes the phrase, so [CR#611.3b]'s covariance holds. Recorded
  on `SpanOk`.
- **The static statement's TURN WINDOW already landed.** `OnlyDuring` is the
  third reader of the timing vocabulary and `windowOk` is the same gate the
  activated ability's `Timing` and the trigger's `TriggerWindow` get.
  Re-measured at **96 supported lines over 94 cards** (91 "during your
  turn"), not the two the ledger carried. Bench: **Ahn-Crop Invader whole**
  (the sentence Restless Spire quotes) and Bedrock Tortoise's line.
  At Knifepoint is blocked on "outlaws", the cover word for five creature
  types — not on the window.
- **The "play with [slice] revealed" static already landed** (routed item).
  `Visibility Reveal <who> TopOfLibrary` is the row, and Field of Dreams,
  Lantern of Insight and Wizened Snitches were already benched on it.
- **The ability on the stack already landed.** `AbilityClass` +
  `AbilityHead`, with `AbilityOf`/`ActivatedBy` for the possessor and the
  source restriction, and `Counterable`'s ability row for the verb.
  Re-measured: **40 supported lines** name a targeted ability there, not 60
  — 23 at "counter" (Squelch, Stifle, Disallow benched) and 15 at "copy".
  The description vocabulary the corpus writes is "activated", "triggered",
  "activated or triggered", "loyalty", plus a source restriction ("from an
  artifact source" 4, "from a noncreature source" 1) — all inside
  `AbilityClass`/`AbilityOf` and nothing wider.
- **The COMPOUND SUBJECT already landed** in the noun vocabulary:
  `EachOfBoth` over `BothOf`'s pair. Re-measured at **14 supported lines**,
  not three. Bench: Alluring Suitor's activated ability (a static subject).
- **`attachHeadOk Equipped PermanentW` — the ledger was stale.** The cell is
  True and the table's own docstring names Luxior, Giada's Gift. Re-measured:
  "equipped permanent" 1, "equipped planeswalker" 0, "enchanted permanent"
  105 (the ledger said 76). No change needed; the ledger line is corrected
  here.
- **The non-battlefield granted subject: FOUR spans, unchanged.** Case of the
  Uneaten Feast, Kethis and The Grim Captain's Locker (graveyard), Lukka
  (exiled this way); hand and library ZERO. Under the bar, no row minted.
- **The one-time boon: still ZERO supported.** 27 cards write it and every
  one is Alchemy.

### Routed items

- **Blind Fury (lose-one-named-ability)** — LANDED and benched whole; the
  row is `LosesAbilities` at 76 supported lines.
- **Koh, the Face Stealer's whole-ability-set grant** — the GRANT is built
  and its exact shape is `GainsAbilitiesOf This [AnyActivated, AnyTriggered]
  <source> Nothing`. Koh still does not bench: its source is "the last
  chosen card", the object-sorted MARKED READ, which no row writes. Idris,
  Soul of the TARDIS writes the same class pair over "the exiled card" — a
  participle definite its own earlier ability stamped — and no static line
  reads across an ability boundary. So the class slot's LIST arm has no
  bench entry: **both** supported two-class lines are held up by their
  source, and the blocker is the chooser/anaphora family's, not this one's.
- **"Play with [slice] revealed"** — already landed and benched (above).

### Remainders (explicit)

1. **Copying an ability** — 15 supported lines. `CopyStack` takes a
   `Noun bs Object` under `OnStack` where `CounterSpell` is kind-indexed
   under `Counterable`; widening it wants `Counterable`'s twin AND a second
   move at the rider, since every one of the 15 goes on to say "you may
   choose new targets for the copy" and `ChooseNewTargets` is Object-kinded
   too (`wordReaches CopyW AbilityP = False`). Two widenings in the COPY
   verb's own family. The ledger's "both verbs' complement slots are already
   the ordinary stack noun" is wrong about copy.
2. **The keyword-term arm of the ability loss** — Shay Cormac's bare
   "protection"/"ward" and Tolaria's "all 'bands with other' abilities" want
   a `KeywordTerm` element beside the `AbilityAt` one. 2 lines.
3. **The "instead"/"additional" second statement** (3 lines) and the
   **ignore-this-effect permission** (2 lines), from the multi-sentence
   measurement.
4. **Distortion Strike does not bench whole**: its second line is the
   keyword Rebound, absent from the keyword-facts catalog.
5. **"This token can't block"** (Harried Spearguard, Anax) still does not
   write: the deed table gives "Block" a Creature-typed agent and refuses a
   bare subject ([CR#509.1a]), and the marker word ascribes no type. The
   deed vocabulary's cell.
6. **Gutter Grime does not bench**: its token payload counts *slime*
   counters and `CounterKind` is closed by enumeration without that word.
   The grantor reference it wanted is built; the counter word is not.
7. **At Knifepoint** wants "outlaws", the cover word for five creature
   types.
