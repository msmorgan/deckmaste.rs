# workbench-cost-tail

The cost umbrella's survivors, gathered at its close (all three sub-rounds
done 2026-08-28; details in done/workbench-cost-{1,2,3}-*.md):

- **The cost-action stamp across the ability boundary** — Burn at the Stake's
  damage line + 13 for-each reduction lines; `staticChoiceDelta` has `bs`
  erased where `effDelta` needs it.
- **`AltCost`'s two recorded populations** — the subject slot (13 generic
  grants: As Foretold, Fist of Suns) and the value-valued declined cost (11
  non-mana lines); plus `PlayPayment`'s written-alternative arm (23
  pronoun-tail lines).
- **Modal linkage** for entwine/escalate + escalate's per-mode multiplier.
- **Disturb and Morph `keywordFacts` cost rows** — Unblinking Observer and
  Qarsi Deceiver bench the moment they exist (witnesses confirmed).
- **The disjoined defender** — "you or planeswalkers you control" at the gate
  (8 lines); a noun-level disjunction at the defender seat.
- **Small recorded leftovers**: the coloured-mana-only reduction rider (2);
  Nils' per-member counter read inside a gate cost; Verrak's ability-anchored
  life readback; Yidaro's cross-copy payment count; Jegantha's symbol-class
  restriction ([CR#107.4b], 1 line); `CounterKind.Storage` (11 storage
  lands).

Re-measure at claim.

## As landed (2026-09-02)

Every count below was re-measured this round, line-level, over
`jq 'select(.supported)'` with reminder parentheticals stripped. Where a
count differs from the umbrella's or a sub-round's, the re-measure is
authoritative and the correction is written into the grammar beside the
row it belongs to.

### The cost-action stamp across the ability boundary — BUILT

`staticChoiceDelta` takes `bs` un-erased and the `AddedCost` arm reads a
new `costDelta` (`effDelta`'s move at the cost seat), so an additional
cost exports its WHOLE delta and not merely its chooser. [CR#118.1]
makes a cost an action a player carries out, so what the action did to
the objects it named is a fact the next sentence may read; the as-enters
rows still export the choice alone, because a chooser position has
nothing else to give.

**13 supported lines write "costs [N] less to cast for each ... this
way"; 12 of them pair it with an additional cost** (Dargo, Gorex,
Explosive Singularity, Rottenmouth Viper, Torgaar, Extus, Hierophant
Bio-Titan and the five Marches). The 13th, Mutated Cultist, takes its
stamp from a trigger's own clause and is not this row's.

**Benched: Burn at the Stake WHOLE** (the additional cost is
`Do (Macros.tap ...)`, a labelled keyword action that stamps what it
tapped, and the damage line reads `Times 3 GroupSize`) and **Explosive
Singularity WHOLE**.

**One gate had to move with it**: `staticOnSpellCardOk` now admits
`CostsToCast This` — [CR#113.6e] functions off the battlefield for "an
object's ability that restricts or modifies how THAT PARTICULAR OBJECT
can be played or cast", and 6 of the 12 lines are instants or sorceries.
The subject is asked there, so a reduction stated of a CLASS of other
spells stays a battlefield static. `staticOnSpellCardOk (AltCost ...)`
was tightened the same way.

### `AltCost`'s subject slot — BUILT, and it paid two populations

`AltCost` gains `CostsToCast`'s subject seat: a `Noun bs k` under
`CostSubject`, first and at `bs`, with the cost at `selfSubjIntro n`.
All 21 existing sites write `This`.

- **12 generic grants** (not 13, not 14): As Foretold, Fist of Suns,
  Jodah, Rooftop Storm, Conspiracy Unraveler, Darksteel Monolith,
  Kentaro, Leyline of Mutation, Nissa Worldsoul Speaker, Runeforge
  Champion, Charred Foyer, Tlincalli Hunter.
  **Benched: Fist of Suns WHOLE, Rooftop Storm WHOLE.**
- **10 non-mana declined costs** (not 11), and they needed NO second
  decline: they name an ABILITY and decline ITS cost, which is
  `CostSubject`'s `AbilityCostSubject` arm, open since prohibition-2.
  **3 write today** — **benched: Heart of Kiran's crew alt-cost, New
  Perspectives' cycling line, Thick-Skinned Goblin's echo line.**
  **5 wait on one thing that is no part of this row**: an
  ordinal-per-turn restrictor over ACTIVATIONS ("the first equip ability
  you activate each turn" — Bruenor Battlehammer, Forge Anew, Kili,
  Advancing the Spirit, Gavi). `NthCastBy` is that restrictor at the
  casting history; the activation history has no twin.
  **2 are not this slot's**: K'rrik and Heirloom Epic replace a SYMBOL
  inside a cost, which [CR#107.4b] makes a component of a cost rather
  than a cost — Jegantha's gap in the other voice.
- **As Foretold still stays short** of a whole-card bench: "Once each
  turn" over the statement and a where-clause letter inside the
  subject's description, neither of them the subject slot's.

### The written-alternative payment arm — BUILT

`PlayPayment` moves out of `Words.idr` into `Effect.idr`'s mutual block,
indexed by `Bindings`, and gains `PayingInstead : Cost bs`. The index is
what the arm costs and what it buys: the cost is written where the
permission's complement has already been announced ("paying life equal to
ITS mana value"). The cost is `costOffBattlefield`'s, as `AltCost`'s and
`AddedCost`'s are.

**Re-measured: the 23 lines are TWO shapes, not one.** 16 write the
rider — one sentence, the cost inside the permission (Worldheart Phoenix,
Squee, Raffine's Guidance, Scourge of Nel Toth, Glimpse the Cosmos, The
Infamous Cruelclaw, Anrakyr, Amped Raptor, ...). The other 7 write a
SECOND SENTENCE keyed by "this way" (Bolas's Citadel, Eye of Duskmantle,
Gwenom, Inside Information, Nashi, Valgavoth, Xander's Pact); those are a
statement about a cast an earlier sentence licensed, not a rider, and are
recorded on the type rather than folded in.

New macro `mayCastFromPaying`. **Benched: Raffine's Guidance WHOLE,
Scourge of Nel Toth WHOLE** (the second with a compound cost, mana plus
an action). **Pin: `badPlayPaymentTapSymbol`.**

### Modal linkage for entwine/escalate — BUILT; the multiplier is a zero

`Card.idr` gains `modalFrameOk`, folded into `CardText`: a card printing
a word whose own rule confines it to a modal spell must also write its
modes. Written as a FRAME law beside `chapterLineOk` and not as a
`keywordFacts` column, because it is a demand one line makes on the rest
of the card rather than a fact about the word.
Re-measured: **30 entwine keyword lines and 7 escalate ones** (the
sub-round said 32 and 9), every carrier writing its modes.
**Pins: `badEntwineWithoutModes`, `badEscalateWithoutModes`**, both
non-vacuous (Borrowed Malevolence still elaborates).

**Escalate's per-mode multiplier is a MEASURED ZERO**, recorded and not
minted: ZERO supported lines write "for each mode" outside the word's own
reminder text. [CR#702.120a] states the multiplier and no card prints it;
a keyword ability is a label plus its parameter and the rule expands it.

### Disturb and Morph `keywordFacts` rows — BUILT

Two `CostParam` rows with no stack regime (each carries its own zone, as
madness and mayhem do) and neither in [CR#122.1b]'s keyword-counter list.
Bought by the SPEND-RESTRICTION channel (`CostNamed.OfKeyword` through
`keywordCosts`), not the readback one — both still write zero readbacks,
so the catalog's twenty-word list stands.
The card classes are the RULES' and not the corpus's: neither
[CR#702.146a] nor [CR#702.37a] names a card type, and [CR#708.4] applies
a face-down cast's prohibitions to the face-down characteristics whatever
the card was. Measured: **25 disturb lines and 172 morph/megamorph
lines**, every one on a creature or artifact-creature card.
**Megamorph got no row**: [CR#702.37b] says "a megamorph cost is a morph
cost".

**Benched: Unblinking Observer WHOLE, Ainok Tracker WHOLE** (the morph
keyword line), **`qarsiDeceiverMorphSpend`** — Qarsi Deceiver's
restriction minus its first purpose.
**COST, recorded: Qarsi Deceiver does not bench whole.** "Cast a
face-down creature spell" describes a SPELL as face down, and
[CR#110.5d] says only permanents have status — "cards not on the
battlefield do not" — so `HasStatus FaceDown`, whose seed zone is the
battlefield for that reason, can never describe one. What the line means
is [CR#708.4]'s face-down CAST. **3 supported lines** write "face-down
creature spell"; its own gap, and no part of the keyword catalog's.

### The disjoined defender — BUILT, and the population is six times the
### ticket's

`EitherJoined` completes the coordination grid the `EitherOf` docstring
names: `Both` is the cross-kind conjunction, `Joined` the cross-kind
head, `EitherOf` and `BothOf` the same-kind pair, and the cross-kind
DISJUNCTION was the missing cell. It is `EitherOf`'s meaning at `Both`'s
kind — both arms read in the same context, no joint binding — with
`nounTys` joining and NO agreement gate, because the printed arms
disagree in number ("YOU or PLANESWALKERS you control"); the phrase's own
number is its arms' where they agree and `ManyOf` where they do not.

It is not `Both` under another spelling: a defender is ONE object or
player [CR#506.3,508.1b], so the line names the alternatives an attack
may not pick where a conjunction would name a pair no attack can be
declared against.

**48 supported lines**, not 8: **25** with the plural right arm and
**23** with the singular indefinite, **16 of those the attack trigger**
("Whenever a creature attacks you or a planeswalker you control"). Only 8
pair it with a cost, which is why the gate round counted 8.
`DeonticPatient.DefendingPlayer` needed nothing — the seat took a joined
kind already.
**Benched: Archangel of Tithes' attack toll, Archon of Absolution
WHOLE.**

### The small recorded leftovers

- **The coloured-mana-only reduction rider — BUILT.** `CostShiftRun`
  gains `coloredOnly`. **4 lines, not 2**: Bard Class, Edgewalker,
  Morophon, Ragemonger. It is a rider though it is printed as a second
  sentence — it states nothing on its own, and it says something no rule
  says ([CR#601.2f] subtracts reductions from the total cost and stops
  only at {0}; nothing confines a coloured reduction to the coloured
  part). `False` is exercised by every rise, the 25 strive lines
  included. **Benched: Edgewalker WHOLE.**
- **`CounterKind.Storage` — BUILT.** **18 supported LANDS write 36
  lines**, not 11 lands; 6 of them the "Remove X storage counters ...:
  Add X mana in any combination" family. Its own kind and not `Charge`'s:
  [CR#122.1] gives a counter's kind no meaning beyond the abilities that
  name it. **Benched: Mercadian Bazaar WHOLE.**
- **Nils, Discipline Enforcer — HALF paid, recorded.** Its defender now
  writes with `EitherJoined`; what still blocks the line is the
  distributive read inside the gate cost ("pays {X}, where X is the
  number of counters on that creature"), a per-member read over the
  subject's members plus a where-clause letter opened by a gate.
- **Verrak, Warped Sengir — REFUSED, unchanged.** Three faults at once
  (no cost NAME to sort by, an ABILITY anchor where `PaidCost` takes an
  object, an AMOUNT read where `TimesPaid` counts payments).
- **Yidaro, Wandering Monster — REFUSED, unchanged.** The count runs over
  payment events across every copy of a named card this game.
- **Jegantha, the Wellspring — REFUSED, unchanged, 1 line.** [CR#107.4b]
  makes a numerical symbol a component of a cost, so "can't be spent to
  pay generic mana costs" restricts a cost's PART. K'rrik and Heirloom
  Epic are now recorded beside it as the same gap in the other voice.

### Gates

`idris/scripts/build` 23/23 from a wiped `build/ttc`, 0 errors, 0
warnings. `cargo xtask cite check --list-noncompliant` 0 (a quoted cross-reference
inside a rule excerpt was reworded out of a docstring); `cite check` 0 stale
over 20,206 citations; `cite bless` newly registered [CR#702.37a] and
[CR#708.4], each read against its citing claim;
`jj diff --git | cargo xtask cite audit --diff` read over **52** citation
sites, and one was withdrawn in the pass ([CR#608.2c] was cited for the
participial stamp across a COST boundary, where the rule speaks of a
spell's own instructions in resolution order).

### Deviations

- **Qarsi Deceiver benches as a fragment, not whole** — the face-down
  SPELL description is a real gap ([CR#110.5d]) and was recorded rather
  than worked around; Ainok Tracker carries the Morph row's keyword line
  instead.
- **The generic-grant subject went in FIRST**, `CostsToCast`'s order,
  where the printed line writes the cost first. A cost reading the
  subject is therefore a forward reference; no supported line writes one,
  and the width is the tolerated kind.
- **`EitherJoined` touches `nounTys`**, which the parallel choice round
  was warned off around; the arm is an append and conflicts, if any, keep
  both sides.
