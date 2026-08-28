---
needs: []
---
# Close the mana region: persistence and pool, the production reads, and the "as though" payment permission

The mana region's three open surfaces, taken as one claimable unit: the mana a
sentence keeps talking about (plus the unspent pool), the tapped-for-mana trigger
family and the productions that read another object, and the permission widening
what already-made mana may pay for (with the "as though" oddments that ride with
it). They are one round because they all bind or read the same `AddMana` region —
its riders, its produced run, its mentions and its spend vocabulary.

## The mana a sentence keeps talking about, and the unspent pool

Three families all bind the same missing thing — the mana already added, and the
spell it was spent on — and a fourth reads a player's unspent mana. They are one
region and land together.

### The effect on the PAID-FOR OBJECT — 11 cells

9 sentences write "If that mana is spent on a creature spell, it gains haste"; 2
write the restriction and the effect in one sentence (Cavern of Souls, Delighted
Halfling — "and that spell can't be countered").

### The DELAYED TRIGGER — 3, [CR#603.7a]

Path of Ancestry, Primal Amulet, Pyromancer's Goggles.

Both families bind the spell the mana was spent on, and the frozen module already
names the shape this grammar has no sort for: `bindIt PaidSpellAnte`.

### PERSISTENCE — 25 lines

"Until end of turn, you don't lose this mana as steps and phases end." Core
carries this as a fourth rider (`Persistent(TurnMarker)`) and [CR#106.4] is the
rule it overrides. It is a separate SENTENCE with a duration, so it may want the
`Continuously` envelope rather than a rider slot — THAT IS THE DESIGN QUESTION of
this sub-area.

Sizing that bounds the three: 219 "this mana" / "that mana" lines were measured in
all, and every one belongs to one of these three families or to the spend
restriction that already landed.

### The unspent-mana pool — 14 lines over 13 cards

Doubling Cube, Kruphix, Omnath Locus of All, Mana Short, Upwelling, Yurlok. ZERO
lines write "mana pool", which is [CR#106.4]'s own errata note visible in the
corpus. Drain Power is [CR#106.13]'s named single card and rides here.

The pool is engine state deliberately kept out of the grammar; these 14 lines are
what would make it a sentence, and the round decides whether they do.

## The tapped-for-mana trigger family and the productions that read another object

The referential productions all wait on one header family, and the header family
is measured and reconciled. Landing them together also clears the last freedom
cells on the production run and the four other-object readers the chosen-colour
row could not reach alone.

### The tapped-for-mana trigger family — 56 lines over 55 cards

Two narrower readings of one family, reconciled: 23 lines write the passive
"Whenever [X] is tapped for mana", 33 write the active "Whenever a player taps
[X] for mana", and the union is 56 lines over 55 cards, of which 42 have an
effect that adds mana.

[CR#106.12] defines the phrase ("to tap a permanent for mana is to activate a
mana ability of that permanent that includes the {T} symbol"), [CR#106.12a] makes
the trigger fire on the RESOLUTION, and [CR#605.1b] makes the resulting ability a
mana ability itself.

### `ProducedByEvent` — 17 add-sentences over 17 cards

"Add one mana of any type that land produced." Wants the tap-for-mana event above
plus the capability gate the frozen module carries (`producesMana (eventCaps b)`).
ALL SEVENTEEN sit inside one of the family's headers — a total covariance, and
the warrant for that capability gate.

### The COULD-PRODUCE read — 18 lines over 18 cards, [CR#106.7]

15 of the 18 are add-payloads over 15 cards (Exotic Orchard, Fellwar Stone,
Reflecting Pool). Its blocker is a hypothetical query over another permanent's
abilities that nothing here spells — [CR#106.7] defines it by what an ability
"would produce if the ability were to resolve at that time".

KEEP THE TWO ROWS APART WHEN RE-MEASURING: the past tense and the modal share the
phrase "mana of any type that", and a regex over it merges them — that is how a
first pass put `ProducedByEvent` at 24.

### `AmongColorsOf` — 4 sentences

"One mana of any of the exiled card's colors" (Chrome Mox and its neighbours).
Wants a colour-set read off a mentioned object.

### The four OTHER-OBJECT readers of the chosen colour

Caged Sun, Gauntlet of Power, Utopia Sprawl, Shimmerwilds Growth. These need the
landed chosen-colour row PLUS the `ProducedByEvent` machinery `AddMana`'s
docstring disclaims, so they belong to this round and not to the chooser's. Caged
Sun's middle line ("Creatures you control of the chosen color get +1/+1") was
buildable all along.

### The remaining freedom cells — three, small

- "Add [N] mana in any combination of {R} and/or {G}" — 12 sentences. A
  combination over a WRITTEN colour set, where `EachColor` ranges over all five.
- "Add two mana of different colors" — 4. A third value on the freedom axis, all
  units distinct.
- The five-way alternative list is written ZERO times. That zero is why a
  canonicality gate against `Runs [[W],[U],[B],[R],[G]]` was RECORDED rather than
  minted: minting it would refuse a term denoting attested English. Do not mint
  it now either.

## Widening what already-made mana may pay for, and the "as though" oddments

The mana half of the counterfactual "as though" surface is a PERMISSION on a
carrier the grammar does not have, and it has a sibling phrasing with its own
rule. The zone row and the six unclustered sentences are the rest of the
surface, small enough to ride along and specific enough to lose if they do not.

### The mana half — 52 sentences, and it is NOT `ManaRider`/`SpendOnly`

Checked, not assumed. `ManaRider`/`SpendOnly` is a RESTRICTION attached to mana
as it is produced ("Spend this mana only to cast an Assassin spell", 164
sentences). This family is a PERMISSION widening what already-made mana may pay
for, on a different carrier.

- [CR#609.4b] states its semantics separately: "this affects only how the
  player may pay a cost. It doesn't change that cost, and it doesn't change what
  mana was actually spent".
- [CR#118.14] gives the sibling phrasing "mana of any type can be spent" its own
  rule. Two surfaces — **52 "as though" sentences and 33 "can be spent" ones** —
  which the round that takes this should measure against each other BEFORE
  minting either.
- VIZIER OF THE MENAGERIE is one "can be spent" line from whole; its first line
  already landed and its second already wrote.
- The as-though half names a PURPOSE in 46 of its 52 ("to cast that spell" and
  its kin 39, "to activate" 7) and writes none in 6 — that is `SpendPurpose`'s
  own axis, and the strongest hint that the two families share a vocabulary even
  though they do not share a carrier.

### The zone counterfactual — 1 sentence

Shaman's Trance's "You may play lands and cast spells from other players'
graveyards this turn as though those cards were in your graveyard". It is the
second row of `PlayAsThough`, deliberately unminted, and it is the ONE attested
clause writing a place and an "as though" together — which is why
`badZonedFlashPermission` is row-sensitive rather than slot-sensitive. Its card
additionally wants a play-from-OTHER-PLAYERS'-graveyards permission and a
coordinated "play lands and cast spells", so the row alone lands nothing.

### The six unclustered sentences — each its own thing

- Everlasting Torment and Phyrexian Unlife counterfactualise the damage
  SOURCE's keyword ("as though its source had wither/infect").
- The Chain Veil and Elvish Refueler counterfactualise an ACTIVATION history
  ("as though none of its loyalty abilities have been activated").
- Biomancer's Familiar counterfactualises an object's COUNTERS.
- Shaman's Trance is the zone row above.

Counts here are a prior session's measurements; re-measure before building.

## Consumption boundary

`idris/src/Experimental.idr` (`AddMana` and its riders, `Continuously`, the
duration envelope, the mana mention and its readers; `ProducedRun`,
`ProducedByEvent`, `AmongColorsOf`, `EachColor`, the chosen-colour row;
`PlayAsThough`, `ManaRider`, `SpendPurpose`, the new payment-permission carrier),
`idris/src/Experimental/Events.idr` (the delayed trigger's carrier; the
tap-for-mana header and its capability gate `producesMana`; `PlayAsThough`'s
reader), `idris/src/Experimental/Words.idr` (mana-mention spelling, production
spelling), the pin modules `idris/src/Experimental/Proofs*.idr` (including
`badZonedFlashPermission`), evidence bench `idris/src/Experimental/Cards.idr`.
No Rust crate.

## Acceptance

- One mention of already-added mana serves the paid-for-object effect, the
  delayed trigger and the persistence sentence; the paid-for SPELL is bound once.
- Persistence is settled as envelope or rider on stated evidence, not left
  implicit.
- The zero for "mana pool" survives whatever the pool decision is.
- Cavern of Souls or Delighted Halfling benches the one-sentence form.
- The tap-for-mana header lands once and serves both spellings (passive and
  active); the 56/42 split is re-derivable from the benched witnesses.
- `ProducedByEvent` lands under the header with the capability gate, and the
  could-produce read is measured separately — the merged 24 is not reproduced.
- The five-way alternative list stays unminted and its zero stays recorded.
- At least one of the four other-object readers benches.
- The two mana surfaces ("as though" and "can be spent") are measured against
  each other before either is minted, and the result is recorded whichever way it
  comes out.
- The permission does not land on `ManaRider`/`SpendOnly`; if it does, the
  reason it may is written down against the restriction/permission distinction
  above.
- `badZonedFlashPermission` remains row-sensitive, or its change is argued from
  the single attested sentence.
- Each of the six unclustered sentences is either written or left with its own
  named gap; none are folded into a general counterfactual by accident.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-payment-event-residues (close, 2026-08-26):** mana named by what it was spent on — "for each {B} or {R} spent this way" (Balduvian Fallen's body). A payment-surface read, so it lands here.

- **Routed from workbench-verbed-event-overlap (close, 2026-08-27):** the "for mana" adjunct on the tap act — 35 of 39 active-tap headers write "taps [what] for mana" and no slot spells the adjunct. A mana-production trigger surface, so it lands here.

- **Routed from workbench-choice-e-subtype-words-and-linkage (close, 2026-08-27):**
  "Mana of any type/colour can be spent to cast that spell" has no carrier
  (`PlayAsThough = HadFlash` and nothing else) — the clause beside the linkage
  read on Rogue Class, King Narfi's Betrayal, Summon: Esper Valigarmanda.

## Ruling cross-reference (2026-08-27)

The "as though" payment permission is RULED to be the deontic carrier's
as-though premise slot at the SPEND deed, with a mana-symbol-matcher premise
sort ([CR#609.4b]) — one mechanism with the prohibition region's rider, not a
sibling `SpendAsThough` row. The ruling text lives in
workbench-prohibition-and-permission-acts; this round builds the spend deed's
premise sort against it.

- **Routed from workbench-coordination-1-condition-disjunction (close,
  2026-08-27):** the MANA-SPENT-TO-CAST condition — "if no mana was spent to
  cast it" / "if mana was spent to cast it" has no `Condition` row; the last
  blocker on Mythos of Nethroi and Primeval Spawn (their disjunctions now
  write). A spent-mana read, this region's machinery.

- **Decision from prohibition-2 (close, 2026-08-28):** the MANA RESTRICTION
  ("can't be spent to cast …", 9 real lines — 31 of the naive 40 are token
  reminder text) is RULED into this vocabulary: `Kind` has no mana row, so
  the restriction rides the mana-adding effect and reads
  `DeonticCounterpart`'s noun. Note the umbrella's quoted example ("mana
  value 3 or greater") is 0 supported lines — build against the real 9.

- **Routed from workbench-cost-3 (close, 2026-08-28):** Ice Cauldron's NOTED
  mana — "add this artifact's last noted type and amount of mana" names a
  note of mana SPENT (not a printed cost; `AsPrintedCost` landed the other
  line). A mana note channel, this region's machinery (state-for-notes per
  the named-memory ruling).

## As landed (2026-08-28)

All counts below are re-measured from supported cards only
(`jq 'select(.supported)'`, 32,568 cards / 60,133 text lines). Where they
differ from the prose above, the prior session's figure is named.

### [CR#106.6] is the region's spine, and it was carrying one third of its load

The rule enumerates what a mana production may say about its mana — it
"restrict[s] how that mana can be spent, [has] an additional effect that
affects the spell or ability that mana is spent on, or create[s] a delayed
triggered ability … that triggers when that mana is spent" [CR#603.7a]. The
grammar carried the first and neither other. `ManaRider` now has all three:

- `SpendOnly` (unchanged, 164 sentences) and its negative twin `SpendNotOn`
  — **9 real lines**, exactly the prohibition-2 decision's figure. The naive
  sweep returns 40; **31 are the Powerstone token's reminder text** inside
  parentheses on cards that make one.
- `OnSpent`, which carries [CR#106.6]'s second and third things over ONE
  mention of the paid-for object — **14 cells**: 11 additional effects and 3
  delayed triggers (Path of Ancestry, Primal Amulet, Pyromancer's Goggles),
  both figures as the ticket had them.

**The paid-for spell is bound once**, by `OnSpent`'s own `Noun` slot, and
every reading comes off it: "it", "that spell", "that creature", "copy that
spell". `only` is the Cavern of Souls / Delighted Halfling cell — one
sentence doing [CR#106.6]'s first and second things over a single mention.
Two riders could not have said it: the rider list is a `List` whose members
announce nothing to one another.

**The restriction reads `SpendPurpose`, not `DeonticCounterpart`'s noun** —
a deliberate deviation from the prohibition-2 decision's letter, taken
because that decision's own witness refuses the noun. Jegantha's "can't be
spent to pay generic mana costs" names a COST, and [CR#107.4b] makes a
generic symbol a component of a cost rather than an object, so no noun
describes it. The decision's substance holds: the restriction rides the
mana-adding effect and is not a `Deontic`, because no `Kind` names mana.

### The mention: an OUTCOME, not a noun

`AddMana` now announces `outcomeB ManaAdded` on `DealDamage`'s exact
channels (`effIntro` and `deedDelta`, and neither pre-resolution channel —
nothing is in a pool until the clause resolves). `ManaHeld.ThisMana` reads
it. Mana is not an object and no `Kind` names it, so `itReaches` can never
reach the mention — which is right: no line writes "it" for mana.

### PERSISTENCE: the envelope, on stated evidence

**32 lines** carry the body, not 25. 25 say it of mana a preceding sentence
added ("this mana") and **7 say it of a player's unspent mana** — a family
the prose above did not separate out. The decision is settled by those 7,
three ways:

1. **6 of the 7 write no span at all** (Omnath Locus of Mana, Leyline
   Tyrant, Ashling, Electro, Fangorn, Upwelling), so the 25's "Until end of
   turn" is a duration the sentence takes, not one a rider must invent.
2. **Those same 6 are static abilities of permanents that add no mana
   anywhere on the card.** No attachment to a production could have carried
   them; a rider slot would have written 25 of 32 lines.
3. **[CR#106.6] does not list it.** The rule enumerates what a production
   may say about its mana and persistence is not among the three.

So: `KeepsUnspentMana` is its own `StaticEffect` under the ordinary
`Continuously` envelope, at a new `StaticKind.ManaPersistence`. The frozen
`Semantics.idr` reached the same answer independently — its `ManaRider`
doc excludes "doesn't empty" as "a separate static/replacement over all your
mana", naming Omnath and Upwelling.

### The two mana surfaces, measured against each other before either minted

- **"as though": 50** sentences (the ticket said 52).
- **"can be spent": 33** — exactly as measured before.

**Result: ONE carrier, two spellings, and the rules say so in as many
words.** [CR#118.14] glosses "mana of any type can be spent" as "players may
spend mana as though it were colorless mana or mana of any color to pay that
cost", and [CR#609.4b] closes with "the same is true for effects that say
'mana of any type can be spent'". The passive is the counterfactual with its
agent unwritten — spelling, exactly as the tapped-for-mana header's two
voices are. **83 sentences, one row.** Vizier of the Menagerie's third line
is a third spelling of the same thing ("You can spend mana of any type to
cast creature spells") and benches on the same row.

**It did not land on `ManaRider`/`SpendOnly`**, and the two are now
documented against each other at the row: [CR#106.6]'s restriction attaches
to mana AS IT IS PRODUCED and narrows; [CR#609.4b]'s permission is over mana
already made, by whatever produced it, and widens.

**Built as ruled.** `deedCounterfactual` widened from `Bool` to
`Maybe PremiseSort`, so which payload a permission may carry is the deed's
own fact. A new `"Spend"` deed (agent a player [CR#106.1], patient `noRole`
— what is spent is mana, and no `Kind` names it) carries `ManaPremise`; the
five deeds that already admitted a counterfactual carry `ObjectPremise`. The
purpose rides the premise, not the carrier, because [CR#609.4b] scopes the
permission to "a cost" and naming that cost is part of what the
counterfactual says.

### The tap-for-mana header: ONE header, both spellings

**58 lines over 57 cards** (the ticket said 56/55) — **23 passive** and **35
active**, and the union splits as the routed verbed-event item predicted.
**42 add mana**, exactly as measured before. The active half is 19 "a player
taps" plus 16 "you tap"; a regex on "taps" alone misses the second-person
half, which is how the prior 23/33 split arose.

`VerbedEvent` gains a `forMana : Bool` slot gated by `verbForManaOk`, True
for `"Tap"` alone — [CR#106.12] defines the phrase for one act and defines
it as a second act. `VerbedVoice` already made the voice a spelling, so no
second row was needed. [CR#106.12a] fires the trigger on the ability
resolving and producing mana, so the narrowed event mints `ManaProduced`.

`ProducedByEvent` is gated on that mention. **17 sentences over 17 cards**,
and **all 17 sit inside such a header** — the total covariance the ticket
predicted, and the warrant for gating the reader rather than trusting the
noun.

**The could-produce read was measured separately and the merged 24 is not
reproduced**: **18 lines over 18 cards, 15 of them add payloads**, exactly
the ticket's figures. `CouldProduce` is ungated where `ProducedByEvent` is
gated, and [CR#106.7] is why — a hypothetical needs no event, only a
permanent to ask about.

### Remaining productions and the freedom cells

- `AmongColorsOf` — **3 lines** read a mentioned object's colours (Chrome
  Mox, Pit of Offerings, Omnath Locus of All). The ticket's 4th match is
  **Cryptic Spires' "either of the circled colors"**, which is a chosen-value
  read and not an object's colours; recorded so the 4 is not re-derived.
- `AmongWritten` — the combination over a written colour set, **12
  sentences** (the ticket's figure), against **34** that leave the set at
  all five.
- `ColorFreedom.DistinctColors` — "two mana of different colors", **4
  lines**, a value on the freedom axis rather than a rider (a rider could be
  written beside `SameColor`, which is a contradiction no rule resolves).
- **The five-way alternative list stays 0 and stays unminted.** No gate
  against `Runs [[W],[U],[B],[R],[G]]` was minted. Tolerated
  overgeneration recorded: `AmongWritten` admits a written 5-set, which says
  what `AnyColor EachColor` says.
- **"mana pool" is 0 lines**, and [CR#106.4] states the reason in the rule
  itself — the errata "to no longer explicitly refer to the mana pool". No
  row here spells a pool. The zero survives.

### Ice Cauldron: the note channel DID reduce cheaply, on the read side

Per the named-memory ruling (Option B, 2026-08-26): persistent notes are
STATE on the holder with anchored ungated reads, `GreatestStoredMatch` the
landed precedent. `ProducedMana.LastNotedMana` is exactly that shape — a
singular holder, no key, no gate. **[CR#607.2e] is the rule the ruling's
obligation (2) asked for**, and it is the direct twin of [CR#607.2d]'s
chosen-value link the same ruling already leaned on: "if an object has an
ability … that allows some information to be noted and another ability which
refers to information noted for that object, those abilities are linked".

The **noting side is the named remaining gap**: no effect here records state
on a permanent, so Ice Cauldron's first ability still does not write.

### The spent-mana test

`Condition.ManaSpentToCast` — **8 lines over 8 cards** (Boromir, Lavinia,
Nix, Roiling Vortex, Vexing Bauble, Void Mirror, plus Primeval Spawn and
Freestrider Commando coordinating it with a cast test). An EXISTENCE test,
never the amount reads it resembles (Adamant, Converge, Sunburst). Polarity
is `NotCond`'s: all 8 write the negative. Mythos of Nethroi and Primeval
Spawn's disjunctions have their blocker cleared.

### Benched (all under a clean `idris/scripts/build`, 23/23)

`delightedHalflingMana` (the one-sentence form, `only = True`),
`boseijuMana` (the conditional spelling), `pyromancersGogglesMana` (the
delayed trigger), `thranTurbineMana` (`SpendNotOn`), `suChiCaveGuardDies`
("this mana" after an add), `omnathLocusOfManaPersistence` and `upwelling`
(the description arm, no span), `manaFlare` (active tap header +
`ProducedByEvent`), `shimmerwildsGrowth` (passive tap header + an
other-object chosen-colour reader), `chromeMoxMana`, `fellwarStone`,
`iceCauldronNotedMana`, `firemindVesselMana`, `goblinClearcutterMana`,
`vizierOfTheMenagerieSpend`, `vexingBaubleTrigger`, `voidMirror`; and
`rogueClassLevelThree` and `summonEsperValigarmandaCast` extended with the
clause each was waiting on.

Five pins: `badManaPremiseAtAttack`, `badObjectPremiseAtSpend` (the sort law
both directions), `badForManaOnNontap`, `badProducedByEventWithoutEvent`,
`badThisManaWithoutAdd`.

## Deviations from the ticket's premises

- **`badZonedFlashPermission` does not exist** anywhere under
  `idris/src/Experimental/`. It retired with `PlayAsThough = HadFlash` into
  the deontic carrier's general premise slot, and `playSourceOk` now reads
  only the PRESENCE of a counterfactual, not its value. The acceptance line
  naming it is stale; nothing in this round disturbs the surviving
  mechanism, and Shaman's Trance's one attested sentence stays unwritten
  (below).
- **`producesMana (eventCaps b)` is the frozen `Semantics.idr`'s**, not this
  grammar's. Experimental has no capability record on events at all; its
  idiom is the `outcomeB <sort>` mention minted by `eventAfter`, which is
  what `ManaProduced` is. Same law, house shape.
- **The restriction's complement is `SpendPurpose`, not a noun** — argued
  above from Jegantha.

## Remainders (explicit)

1. **9 of the 11 paid-for-object cells wait on a spell-to-permanent read.**
   `OnSpent` carries all 11; what the 9 need is a mention of what the spell
   resolves INTO — "it gains haste until end of turn" (Arena of Glory,
   Generator Servant, Hall of the Bandit Lord, Domri, Carnelian Orb),
   "that creature enters with an additional +1/+1 counter on it" (Animal
   Attendant, Biophagus, Guildmages' Forum). The mention is a spell on the
   stack [CR#601.2a] and the grant lands on the permanent; the grammar
   refuses granting haste to a stack object, correctly. Boseiju and the two
   `only` cells speak of the spell and write today.
2. **Ice Cauldron's noting side** — no effect records state on a permanent.
3. **`SpendPurpose` has no anaphoric arm.** `ToCast` takes a `Predicate`,
   which describes and points at nothing, so "to cast THAT SPELL" / "those
   spells" is written as the description again. Most of the 83 payment
   sentences write the anaphor. Rogue Class and Summon: Esper Valigarmanda
   bench with the description.
4. **The unspent-mana POOL EFFECTS did not land — 14 lines over 13 cards**
   (the ticket's figures, confirmed). The MENTION landed
   (`ManaHeld.UnspentMana`) because persistence's own 7 lines needed it, and
   it is what Doubling Cube's and Glissa's amount reads would take. What did
   not land: "loses all unspent mana" (Mana Short, Power Sink, Pygmy Hippo,
   Worldpurge), the becomes-instead replacement (Horizon Stone, Kruphix,
   Omnath Locus of All, Ozai), Doubling Cube's doubling, Yurlok's
   loss-causes-life-loss, and Drain Power — [CR#106.13]'s named single card,
   which the rule describes as emptying one pool into another. Each is an
   effect row over the mention, not a mention problem.
5. **Pyromancer's Goggles benches its copy but not "you may choose new
   targets for the copy"** — `ChooseNewTargets (ItVerbed "Copy")` is refused;
   the copy stamp does not reach. A copy-family gap, not this region's.
6. **Balduvian Fallen** ("for each {B} or {R} spent this way") — **1 line**,
   plus Emblazoned Golem's "no more than one mana of each color may be spent
   this way" as its only neighbour. An amount read over a payment's mana; no
   row here.
7. **The six unclustered "as though" sentences each keep their own named
   gap, and none was folded into a general counterfactual.** The object-arm
   payload is unchanged; the second arm is gated to the spend deed by sort.
   - Everlasting Torment, Phyrexian Unlife — the DAMAGE's source
     counterfactualised, with no permission and no agent; not a `Deontic` at
     all.
   - The Chain Veil, Elvish Refueler — an ACTIVATION HISTORY. `"Activate"`
     carries no premise sort; opening one needs a premise about the
     abilities rather than the object.
   - Biomancer's Familiar — an object's COUNTERS, under "the next time
     target creature adapts", which is neither a permission nor a deed here.
   - Shaman's Trance — the zone counterfactual, still the one attested
     sentence writing a place and an "as though" together. It additionally
     wants a play-from-other-players'-graveyards permission and a
     coordinated "play lands and cast spells", so the row alone lands
     nothing, as the ticket said.
