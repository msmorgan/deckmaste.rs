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
