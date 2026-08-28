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
