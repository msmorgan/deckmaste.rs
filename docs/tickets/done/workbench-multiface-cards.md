---
needs: []
---
# Give the card record its faces, its layouts and its missing printed boxes

The workbench's `Card` is one face. Every multi-face layout the rules define is
undescribable, and the card record is also missing two printed boxes that are
not face questions but sit in the same record.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 20 (2026-08-24) — graded **MISSING**, and "Not recorded", which is why this
is a ticket. Delta only: the card-level laws (`CardLine`, `CardText`,
`CardChapters`, `CardPt`, `CardCost`, `Experimental.idr:5367-5389`) already
exist and are the frame a face layout has to fit into.

## From the v1 comparison (2026-08-24)

> Crate: `Card::TwoFaced { layout: FaceLayout, front: CardFace, back: CardFace }`
> (`card.rs:87`) with `FaceLayout { Transforming, ModalDfc, Split, Adventure,
> Flip }` (`card.rs:59`), each CR-cited, and `CardFace` (`card.rs:18`) as a full
> characteristic set including `defense`.
>
> Workbench: `record Card` (`Experimental.idr:5391`) is
> `{ name, cost, supers, line, text, pt }` — one face, no `loyalty`, no
> `defense`, no layout, no second face. `Action::Transform` has a workbench
> counterpart only as `StatusVal` face-change; the *card shape* does not exist.
> Not recorded.

Verified in tree at `Experimental.idr:5391-5398`: the six fields are exactly as
described. `Characteristic` carries `Loyalty` (`Words.idr:50`) and
`LoyaltyCost` exists (`Words.idr:1905`), so loyalty is readable and payable but
has no printed box on the record to be read *from*.

## The five layouts, each its own rule

The crate's five layout values are five different rules, not one shape with a
flag, and each names what a "face" is for it:

- non-modal double-faced cards, which transform or convert [CR#712.2];
- modal double-faced cards, whose faces are usually independent [CR#712.3];
- split cards, two faces on one card with a normal back [CR#709.1];
- adventurer cards, a two-part frame with a smaller frame inset in the text box
  [CR#715.1];
- flip cards, a two-part frame where the upside-down text gives alternative
  characteristics [CR#710.1].

Decide whether these are one record with a layout tag (the crate's shape) or
five, and decide it against what the *text* of each face may say about the
other. The flip verb is already partly here:
[workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)
records `SetStatus`/`statusEffectOk Flipped` open at 16 measured occurrences and
that flipping is one-way, so the flip layout's transition half is landed and its
*card shape* is not.

## The two missing printed boxes

Not faces, but the same record, and each already has a named payer:

- **Starting loyalty.** `Card` has no slot for it.
  [workbench-copy-family-residues](workbench-copy-family-residues.md) records
  the starting-loyalty readback twice over — "finding 639's unmodeled box a
  second time" (Ob Nixilis, the Adversary) — so the box is what both readbacks
  wait on.
- **Defense.** The crate's `CardFace` carries it; the workbench has no
  counterpart at all.

## The named payer already on the board

[workbench-description-and-player-sorted-reads](workbench-description-and-player-sorted-reads.md)
counts, among eleven supported cards for one reading, ten stopped elsewhere —
one of them "on an Adventure face". Adventure is the cheapest layout to name a
blocked bench line for; re-measure before scoping, and prefer a layout with a
whole card behind it over one with only a shape.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental.idr` (`record Card` and the
card-level laws `CardLine`, `CardText`, `CardChapters`, `CardPt`, `CardCost`;
any face-scoped re-statement of them), `idris/src/Experimental/Words.idr`
(`PrintedStat` and a defense/loyalty box if either lands; a layout catalog),
the pin modules `idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr` — every existing entry is a single-face term
and must keep elaborating unchanged. No Rust crate.

## Acceptance

- The layout decision (one tagged record or five) is recorded where `Card` is
  defined, with the rule each layout answers.
- Every card-level law is re-stated per face or explicitly scoped to the whole
  card; none is left silently applying to one face of two.
- All 329 existing bench entries elaborate unchanged after the record grows.
- At least one multi-face card benches, named before the round starts; a layout
  that lands no card says so.
- Starting loyalty and defense land as printed boxes or are recorded as
  deliberately deferred with the readbacks that wait on them named.
- The flip layout reuses the landed `Flipped` status transition rather than
  minting a second one.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` 19/19 PASS on a clean
rebuild (`rm -rf build`), and again after a reviewer round applied F1, F2, F3
and F6. Five pins added, two migrated, one written then retired as a clone;
all five survivors perturbed 5 of 5. `Experimental/Cards.idr` binds no implicits and writes no
`{default`; neither do `Experimental.idr`, `Experimental/Macros.idr` or
`Experimental/ProofsG.idr`. `cargo xtask cite check --list-noncompliant`
empty, `cite check` 0 stale, `cite bless` re-pinned 1474 rules, and all 133
citation sites in the diff (74 of them in `idris/`) were read against their
rule text.

### The layout decision: five constructors, two face records

`Card` became a `data` type with **six constructors** — `SingleFaced`,
`Transforming`, `ModalDfc`, `SplitCard`, `Adventurer`, `FlipCard` — not one
record with a `FaceLayout` tag. The decision is recorded in the `data Card`
docstring where `Card` is defined, with the rule each layout answers
([CR#712.2], [CR#712.3], [CR#709.1], [CR#715.1], [CR#710.1]), and it was made
against **which printed boxes each layout's faces actually have**, not against
convenience:

- A nonmodal back face writes no mana cost — [CR#202.3a] and [CR#202.3b] read
  its mana value off the *front* face precisely because the back has none.
- A flip card's upside-down half writes no mana cost either: [CR#710.1c] keeps
  the one printed cost with the card however it is turned, and [CR#710.1b]
  enumerates what the half *does* print (name, text box, type line, power and
  toughness) with no cost among them.
- Modal faces [CR#712.3], split halves [CR#709.4b] and both parts of an
  adventurer frame [CR#715.2] each have a cost slot of their own — filled, or
  empty at a land face, but never read off the other face [CR#202.3b].

So a uniform `CardFace` with a `Maybe ManaCost` would have *stated the
disagreement away* — it would let a flip half or a transformed back carry a
cost neither prints. Two records instead: `CardFace` (name, cost, supers,
line, text, box) and `AltFace` (the same without a cost field). The costless
half is unrepresentable rather than refused, which is why there is no pin for
it — there is no term to pin.

**Cross-face text was the other axis, and it settled the same way.** Each
face carries its own `AbilitySeq []`, so no face's clause can read a binding
another face introduced. That is not hygiene: a face's characteristics exist
only while that face is the one in play ([CR#712.8f], [CR#709.3b],
[CR#715.3b]), so there is no moment at which one face's anaphor could resolve
against the other's antecedent. A single shared text field would have been the
wrong shape independently of the box question.

Meld [CR#712.4] is deliberately **not** a sixth layout and the `data Card`
docstring says so: a meld pair's combined back face belongs to two cards at
once [CR#712.4b], so it is not a second face of one card.

### Card-level laws, re-stated per face

Every one of `CardLine`, `CardSupers`, `CardText`, `CardChapters`,
`CardBox` (was `CardPt`) and `CardCost` is re-stated at each face by
`FaceLaws : CardFace -> Type`. `AltFaceLaws : AltFace -> Type` re-states four
of them unchanged, **drops** `CardCost` (the face writes no cost, so the
land-cost gate has no cost to read) and **restates the corner-box law as
`AltCardBox`** — see the loyalty note below. **No law is scoped whole-card and
none is left silently applying to one face of two.** Two layout gates sit on
top of the face laws, each stating only what `CardLine` does not already force:

- `AdventureInset` — the inset frame names the `Adventure` subtype. A player
  plays the card "as an Adventure" [CR#715.3] and [CR#205.3k] makes Adventure a
  spell type. That its line is then an instant or a sorcery is **not** restated:
  `subsFitLine` fits a spell type to no other card type, so the conjunct would
  be unreachable.
- `FlipHalf` — each half names a permanent type, the six [CR#110.4] lists,
  because [CR#710.2] reads the alternative characteristics only on the
  battlefield. Excluding spell types is **not** restated: `typesCombinable`
  already refuses a line mixing the two.

### The two missing printed boxes: one corner, three boxes

Starting loyalty and defense landed **as printed boxes**, and they landed by
generalising the P/T slot rather than sitting beside it. [CR#208.1],
[CR#209.1] and [CR#210.1] each name *the lower right corner*, and [CR#200.1]
lists power/toughness, loyalty and defense as three separate parts of a card.
So the record has one box, not three optional ones:

```
data PrintedBox = PtBox PrintedStat PrintedStat
                | LoyaltyBox PrintedStat
                | DefenseBox PrintedStat
```

`Card.pt : Maybe (PrintedStat, PrintedStat)` became `CardFace.box :
Maybe PrintedBox`. All three boxes take `PrintedStat`, so the starred forms
(`PrintedStar`, `PrintedStarPlus`) are available at loyalty and defense on the
same terms as at P/T; only `PtBox` has slots a characteristic-defining line can
star, and `boxPt`/`definedSlotsStarred` keep [CR#208.2]'s starred-print gate
reading exactly the pair it always read. `cardPtOk` became `cardBoxOk`, whose
new half is `boxFitsLine`/`boxSuitsType`: Creature demands a `PtBox`,
Planeswalker a `LoyaltyBox`, Battle a `DefenseBox`, and every other type leaves
the corner to the rest of the line. A line naming two of the three demands two
numbers in one corner and has no box that fits — that falls out of the table
rather than being asserted, and `badPlaneswalkerPtBox` witnesses it.

Migrated: the four planeswalker witnesses that had no loyalty box
(`jaceBeleren` 3, `elspethSunsChampion` 4, `saheeliFiligreeMaster` 3,
`gideonAllyOfZendikar` 4) and the one battle (`invasionOfDominaria`, defense
5). All five loyalty/defense numbers were verified with `card`.

**The loyalty demand is a card demand, not a face demand.** Applying
`boxSuitsType` through `AltFaceLaws` made two printed cards unrepresentable —
Arlinn, Embraced by the Moon and Garruk, the Veil-Cursed are legendary
planeswalker **back faces with the box empty** (both verified with `card`),
while Jace, Telepath Unbound prints loyalty 5 on the same kind of face. So the
costless face answers its own `AltCardBox`/`altBoxSuitsType`: the creature P/T
demand stands ([CR#710.1b] enumerates power and toughness among what such a
half prints, and nonmodal creature backs print them), the battle demand stands,
and the planeswalker box becomes **optional** — [CR#209.1] puts the number on
each planeswalker *card*, and [CR#712.8a] reads a double-faced card's
characteristics off its front face everywhere but the battlefield and the
stack, so a back face has a loyalty to read without printing one. Evidence is
`Cards.planeswalkerBackWithoutLoyaltyOk`, an `AltFaceLaws` probe rather than a
bench entry, because **neither whole card benches**: both print "Transform
[this]" in their loyalty abilities and the transform verb does not exist
(Ledger). `Arlinn` joined the subtype catalog for the probe's type line.

### Per-layout witness outcomes: five for five

Every layout landed a **whole card**. Named before building, each text
verified with `card`:

| Layout | Witness | Note |
|---|---|---|
| `Transforming` [CR#712.2] | Invasion of Dominaria // Serra Faithkeeper | see below |
| `ModalDfc` [CR#712.3] | Branchloft Pathway // Boulderloft Pathway | two land faces, one mana ability each |
| `SplitCard` [CR#709.1] | Wax // Wane | two instants, one cost each |
| `Adventurer` [CR#715.1] | Merfolk Secretkeeper // Venture Deeper | vanilla 0/4 front, `Sorcery — Adventure` inset |
| `FlipCard` [CR#710.1] | Orochi Eggwatcher // Shidako, Broodmistress | the flip is spelled `SetStatus Flipped` |

The transform layout was expected to land a shape and no card, because the
workbench has no transform verb (`GameEvent`/`Effect` carry the face-down
`FaceUp`/`FaceDown` status of [CR#708], which is a different mechanic). It
landed a whole card anyway: **[CR#310.12b] makes the turning-over intrinsic to
every Siege** — "When the last defense counter is removed from this permanent,
exile it, then you may cast it transformed without paying its mana cost" is not
printed text — so a Siege battle's printed text is only what each face says for
itself. `invasionOfDominaria` was already on the bench as a front face alone;
it is now the whole card, and the same entry pays for the defense box.

The flip layout reuses the landed `Flipped` status transition and mints
nothing: Orochi Eggwatcher's own activated ability writes
`Macros.ifThen (CompareAmt (CountOf creatureYouControl) AtLeast (Lit 10))
(SetStatus Flipped thisCreature)`. `FlipCard`'s docstring records that the
transition is one-way by [CR#710.4] and that the constructor adds no second
verb for it.

### The bench held

433 `Card` definitions in `Experimental/Cards.idr`, up from 429 (four new
multi-face entries; `invasionOfDominaria` was rewritten in place, not
replaced). **No witness lost.** The 425 `Macros.card` sites are byte-identical:
`card`'s signature is unchanged apart from its `pts` implicit becoming `bx`,
which no bench entry names. `Macros.cardOf` (8 sites) took the mechanical
change — its `stats` argument is now `Maybe PrintedBox`, so the four starred-
P/T entries write `Just (PtBox PrintedStar PrintedStar)` and the four
planeswalkers write `Macros.loyaltyBox n`. New helpers beside `printedBox`:
`loyaltyBox` [CR#209.1] and `defenseBox` [CR#210.1].

### A catalog repair the round needed

`subsFitLine` fitted a subtype to a line by `subtypeType`'s single answer,
which under-reports the spell types: [CR#205.3k] makes Adventure, Arcane,
Lesson, Omen and Trap one list that instants **and** sorceries share, so
`Adventure` on `Sorcery — Adventure` (the common adventurer inset) did not fit.
Added `spellSubtype` and a third disjunct to `subsFitLine`, in the same idiom
as the existing Kindred special case [CR#308.2]. This also repairs `Arcane`,
which could previously only sit on an `Instant` line. `spellSubtype` reads
`subtypeType s == Instant` rather than listing the spell types behind a
`_ = False` catch-all, so it is exactly [CR#205.3k]'s shared list and a future
Omen or Lesson mint answers it without a second edit. Subtype catalog gained
`Adventure`, `Snake` and `Arlinn`.

### Pins ±

Five added, all in `ProofsG`, each naming a rule:

| Pin | Rule |
|---|---|
| `badPlaneswalkerNoLoyalty` | [CR#209.1] — the loyalty number is printed on the card |
| `badPlaneswalkerPtBox` | [CR#209.1] + [CR#208.1] — same corner, one box |
| `badBattleNoDefense` | [CR#210.1] — the defense number is printed |
| `badUnnamedAdventure` | [CR#715.3] + [CR#205.3k] — the spell type is what it's played as |
| `badSpellFlipHalf` | [CR#710.2] + [CR#110.4] — a flip half is a permanent face |

A sixth, `badPermanentAdventure` (inset typed `Enchantment`), was written and
then **retired**: once `adventureInsetOk`'s unreachable conjunct went, its
refusal was `badUnnamedAdventure`'s clause verbatim — a cloned pin, not a
second fact.

**Perturbation, 5 of 5 as expected.** Each pin's term was flipped to its legal
twin (loyalty box supplied, defense box supplied, P/T box swapped for loyalty,
`Adventure` added to the inset line, the flip half made a creature) and the
build re-run: every one lifted the refusal, so no pin passes vacuously or for a
neighbouring law's reason.

Two migrated with the law rename (`MkCardPt` → `MkCardBox`, `{pts}` →
`{bx}`): `ProofsD.badCreatureCardNoPt` and `ProofsE.badStarlessDefinedPt`.
Both re-elaborated against the generalised `cardBoxOk` and still refuse.

**No pin here is justified by a count** — every refusal names the rule that
makes the term meaningless, per
`docs/memory/rulings/measurements-live-in-pins.md`. Deliberately *not* pinned,
as tolerated overgeneration: a non-creature face may still print a `PtBox`
(Vehicles), a spell face may print any box at all, and a split half may be a
permanent card ([CR#709.5] Rooms are the printed case).

### Ledger

Named blockers, one line apiece. None is a refusal.

| Item | State after this round | Exact missing piece |
|---|---|---|
| Transform as a verb | not built; the Siege witness does not need it | `Action::Transform`'s counterpart — a `transform`/`convert` effect and its trigger event [CR#701.27,701.28]; every non-Siege TDFC's front face writes one in printed text |
| Arlinn Kord // Arlinn, Embraced by the Moon | back face's box law probed (`planeswalkerBackWithoutLoyaltyOk`); whole card blocked | the transform verb (both faces print "Transform [this]"), plus "up to one target", an emblem body and a `Wolf` subtype row |
| Garruk Relentless // Garruk, the Veil-Cursed | same box law, same block | the transform verb (the front's state trigger writes it), plus a `Garruk` subtype row |
| Meld [CR#712.4] | out of scope, recorded in the `Card` docstring | a two-card object; [CR#712.4b] and [CR#712.21] make it a different shape from a face layout |
| Split cards with a shared type line (Rooms, [CR#709.5]) | `SplitCard` admits them; the unlock designations do not exist | the "left/right half unlocked" designations [CR#709.5c] and the unlock special action [CR#709.5e] |
| Fuse [CR#702.102] | out | a keyword row; chose Wax // Wane over Wear // Tear to avoid it |
| The Adventure-face blocked line in `workbench-description-and-player-sorted-reads` | the *shape* is now here | re-measure that ticket's eleven-card reading against the landed `Adventurer` and `Transforming` constructors — the two Sieges it also counts are likewise unblocked at the card shape |
| Adventure exile-and-recast [CR#715.3d] | out | the "then exile this card, you may cast the creature later from exile" rider is reminder text on the printed card and has no row |
| `Snake`, `Adventure` subtypes | minted this round | — |

Measurements recorded here, not in source: 433 `Card` definitions (was 429),
425 unchanged `Macros.card` sites, 8 `Macros.cardOf` sites, 137 audited
citation sites, 1477 rules in the lock, 5 of 5 pin perturbations lifting.
