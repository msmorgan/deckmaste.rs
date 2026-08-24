---
needs: []
---
# Finish the name match, its counted residues and its group-level constraints

One family: `Predicate.Named` and its chooser residues, plus the group-level
constraints ("different names", "the same name as one another") that ride the
counted mention instead of the member description. They meet at the same pin
(`badGroupNameRelatum`), the same `NameSource`, and the same six elliptical
lines, so the co-referential row and the group row have to be decided together.

> **Ruling 2026-08-22:** the group relatum is admitted. "Cards with the same
> name as those creatures" is rules-meaningful (names are strings; the relation
> lifts pointwise), so `badGroupNameRelatum` is deleted under
> `workbench-pins-refuse-rules-impossibility-only`'s doctrine and the internal
> disagreement below dissolves: both shapes are rows.

## The name match's counted residues

`Predicate.Named` takes a `NameSource bs` (`PrintedName` / `ChosenName`), one
row over the source with the binding demand on the payload and the spelling
derived from it; the population is 70 supported lines over 63 cards in three
frames (this count corrects an earlier "72 over 65"), and 43 of the 70 make the
choice and read it in the SAME line (finding 854). The narrowing landed too:
`ChoiceDomain : QualitySort -> Type` on both `QualityNoun` (20 effect-level
lines) and `EntersChoice` (10 as-enters), cross-sort rather than a card-name
gadget (46 lines over four sorts), with the card-name chooser corrected to 66
lines — 26 prenominal narrowings + 4 postnominal exceptions, not 28 (finding
868: the earlier sweep dropped the narrowings containing a comma). Four
residues, each counted.

- **The postnominal exception**, 4 lines: "a card name other than a basic land
  card name" (Booby Trap, Desperate Research, Necromentia, Null Chamber). Its
  content is `NameOfCard (Not (And [HasSupertype Basic, HasType Land]))` and
  what refuses it is `negatable (And _) = False` — a standing refusal with
  nothing to do with names or choosers.
- **Garth One-Eye's chooser**, 1 line and the same slot's most extreme cell: "a
  card name that hasn't been chosen from among Disenchant, Braingeyser, Terror,
  Shivan Dragon, Regrowth, and Black Lotus" wants a literal-name LIST plus a
  not-yet-chosen memory. ("The card with the chosen name" is writable today; the
  rest of that card is the copy-a-card verb's.)
- **The plural read**, 5 cards and not 1 line (finding 1028 — ENLARGED, and it
  splits by CHOOSER shape rather than by surface). One plural surface over three
  chooser shapes: two choosers made by one distributive clause (Null Chamber,
  "you and an opponent each choose a card name" … "the chosen names"); one
  chooser making a PLURAL choice (Seal of the Guildpact, Tablet of the Guilds —
  "choose two colors" … "for each of the chosen colors it is"); and three
  choosers written as three clauses with a union read (Paliano the High City and
  Regicide — "The player to your right chooses a color, you choose another
  color, then the player to your left chooses a third color" … "one or more of
  the colors chosen as you drafted cards named Regicide", which also wants
  draft-time choosers and a card-name-scoped memory). **Measure which shape the
  round takes before assuming one construction**: the recency family shares the
  two-chooser configuration and writes a SINGULAR marked read instead.

### Pins that must survive

`badNameMatchWrongSort`'s sibling refusal at two bindings is not a pin about
English and stands as it is. `badTwoChoosersOneSortRead` stands untouched: three
cards in the whole corpus write two same-sort choosers and none of them then
writes the plain read, because English marks a re-choice with a different word
("the LAST chosen color", Chromatic Armor) that [CR#607.2d] names in the same
sentence.

## The group-level name constraint, both polarities

Thirty supported lines say something about a *group* of chosen members that no
predicate in this vocabulary can say: that no two of them share a name, or that
all of them do. Both poles are one construction, and it rides the **counted
mention** rather than the mention's description — every predicate here is a test
on one member.

### Different names — 24 supported lines

"Search your library for up to three artifact cards with different names"
(Saheeli Rai); "Return any number of permanent cards with different names from
your graveyard to the battlefield" (Eerie Ultimatum); Deathbellow War Cry; Behold
the Sinister Six!; Begin the Invasion; Ecological Appreciation.

It is **not** a `Predicate`: the constraint is on the group — no member may share
a name with another. `DistinctDisjuncts` is the nearest existing shape and asks a
different question (two *written alternatives*, not n *chosen members*). Saheeli
Rai is the named witness and the only reason that card is not the bench's second
planeswalker.

The co-referential name match does **not** reach it, in either direction: the
count re-derives unchanged at **24**, and `NameSource`'s three sources all answer
"does *this* object have that name", where this asks whether no two of n share
one. The rule states it in the same shape, as one predicate over a group —
[CR#201.2b], "those objects have different names only if each of them has at
least one name and no two objects in that group have a name in common".

### The same name — 6 supported lines

"two or more nonland, nontoken permanents with the same name as one another"
(Chrome Replicator) and "eight or more artifacts with the same name as one
another" (Mechanized Production), plus four that write **no relatum at all**:
"three or more lands with the same name" (Endless Atlas, Sceptre of Eternal
Glory), "two nonland cards with the same name" (Sphinx of the Chimes), and
Tainted Pact's "you exile two cards with the same name".

Same shape, opposite polarity, same reason neither is a `Predicate`.
`badGroupNameRelatum` is the pin that keeps the co-referential row from being
mistaken for this one and must keep holding.

## The same-name group relatum, decided against its pin

The co-referential name landed with one demand — the relatum is an ordinary
singular object noun, [CR#201.2a] comparing an object to an object — satisfied by
**111 of 113** lines and pinned for the other shape as `badGroupNameRelatum`.
Six measured lines **are** that other shape (the six above: Chrome Replicator,
Mechanized Production, and the four elliptical lines — Endless Atlas, Sceptre of
Eternal Glory, Sphinx of the Chimes, Tainted Pact), so the pin's fate has to be
decided rather than inherited. [CR#201.2b] is the negative pole's rule and states
the constraint in the same shape.

Two questions, and they are not the same one: whether a group-internal comparison
is the same relation as an object-to-object one, and whether the four elliptical
lines write a relatum at all or read one from their own subject.

**Disagreement carried from the merged entries, unresolved here.** The
group-constraint entry says `badGroupNameRelatum` "must keep holding"; the
relatum entry says its fate must be decided — narrowed to what it still refuses,
or intact with the group carried by its own row. The round must settle which,
not assume either.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`Named`, `NameSource`, `ChoiceDomain`,
`NameOfCard`, `negatable`), `idris/src/Experimental.idr` (the plural read's
binding; the counted mention, `NameSource`, `DistinctDisjuncts`,
`Predicate.Named`), pins in `idris/src/Experimental/ProofsF.idr`
(`badGroupNameRelatum`), evidence bench `idris/src/Experimental/Cards.idr`.

## Acceptance

- The 4 postnominal exceptions write, and `negatable (And _)` is answered
  deliberately rather than special-cased for names.
- The plural read's three chooser shapes are measured and the round's coverage
  stated per shape.
- One construction carries both group polarities and attaches to the counted
  mention, not to the member description.
- The relatum-less four elaborate without inventing a relatum, and are answered
  explicitly rather than left to the general relatum.
- `badGroupNameRelatum` ends the round in a stated relation to the 6 lines:
  narrowed to what it still refuses, or intact with the group carried by its own
  row; the co-referential name row is unchanged either way.
- Saheeli Rai is re-checked as a whole-card witness.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

**The carried disagreement, resolved.** The header ruling governs: where the
group-constraint section says `badGroupNameRelatum` "must keep holding", it is
superseded. The pin refused `Named (SameNameAs (AllOf creature))` on the ground
that [CR#201.2a] "compares an object to an object", but that is not what the
rule says: [CR#201.2a] states the same-name relation itself over "two or more
objects", and [CR#201.2c] shows the CR comparing an object against "a second
object **or group of objects**" outright (for the different-name pole). So the
term the pin refused is rules-meaningful and the pin was a defect under
`docs/memory/rulings/measurements-live-in-pins.md`. The pin is deleted and
`SameNameAs`'s `nounPlur n = OneOf` gate with it. The co-referential name row
is otherwise unchanged: one `NameSource`, three sources, same spelling.

**Re-measured before building** (mtg-rules `corpus`, default *supported* scope
— the earlier 28/33 figures came from `--all`, which is where the drift was):

| claim | ticket | re-measured |
|---|---|---|
| "with different names" on a counted mention | 24 | **24** (28 over all "different names" phrasings; 33 with `--all`) |
| group same-name lines | 6 | **6** (a 7th under `--all` is reminder text quoting the legend rule) |
| postnominal exception | 4 | **4** (Booby Trap, Desperate Research, Necromentia, Null Chamber) |
| plural read | 5 cards / 3 chooser shapes | **5 cards** (Paliano and Regicide are in supported scope; their draft clause is what is not) |

### One construction, both polarities, on the counted mention

`NameAgreement` (`DifferentNames` / `SameName`) in `Words.idr`, carried by a new
`Noun` constructor `NamesAgree agr grp` gated by `CountedMention grp` (the
mentions that write their own headcount: `CountedGroup`, `TargetGroup`) and
`nounPlur grp = ManyOf`. It **wraps**: `nounDelta` delegates, so the phrase is
one mention of one referent with the wrapped mention's count, zone, head type
and bindings. Not a `Predicate`, as the ticket required — every predicate here
tests one member. `DistinctDisjuncts` was left alone; it asks about written
alternatives, not chosen members.

The relatum-less four are answered by having **no relatum slot at all**:
"with the same name as one another" and the elliptical "with the same name"
are one `SameName` construction differing only in whether English writes the
reciprocal. Nothing was invented for them.

A sibling `Condition` constructor `ExistsGroup n` was needed to land the
positive pole's own witnesses: `Exists` takes a `Predicate` and a description
carries no count, so a group-level constraint had nowhere to be tested in a
condition. Its gate `CountedExistential` admits ONLY a counted mention under a
group-agreement wrapper — never a bare one. A bare counted mention is the
comparison's spelling ("three or more artifacts" as a headcount comparison),
and a bare TARGETED one would be an announcement hole: a comparison carries its
amounts' bindings out to the clause it governs, where [CR#601.2c] has every
target announced, while `ExistsGroup` introduces nothing. Refused structurally
by the gate, not by a pin — no rule makes the term meaningless, so under
`measurements-live-in-pins` there is nothing to pin.

Macros: `withDifferentNames`, `withTheSameName`.

Landed witnesses (all text-verified against `card`):

- **Eerie Ultimatum** — whole card, negative pole on `CountedGroup anyNumber`.
- **Sphinx of the Chimes** — whole card, positive pole with no relatum, inside
  an activation cost.
- **Chrome Replicator** — whole card, positive pole with the reciprocal, under
  `ExistsGroup` in an intervening-if.
- **Endless Atlas** — whole card, elliptical positive pole under `ExistsGroup`
  in an activation guard.

**Saheeli Rai, re-checked whole.** The +1 lands (`saheeliRaiPlusOne`) and the −2
already did (`saheelisCopy`). The card still stays off the bench, and no longer
for anything in this family: `Effect.Search` takes a bare
`Predicate bs Object` with no `Quantity`, so "search your library for **up to
three** artifact cards with different names" has no counted mention for the
constraint to ride. Deathbellow War Cry is blocked identically. That is the
counted-search gap, not a name gap — see the ledger.

### The postnominal exception

The stated blocker was stale: `negatable` reads `negatable _ = True` after three
explicit `False` cells, so `And` was already negatable and no widening was
needed. The deliberate answer is recorded on `negatable` instead of assumed: by
De Morgan a conjunction's complement is the disjunction of the conjuncts'
complements, and the rules read object properties one at a time — [CR#205.2b]
has an object satisfy the criteria for any of its card types, [CR#205.4c] makes
every land without the supertype a nonbasic land — so "other than a basic land
card" leaves a populated remainder. That is a property of the conjunction, not
of any modifier inside it, which is why no arm-by-arm test was added: an arm
whose own complement is empty contributes an empty disjunct and takes nothing
from the others. Nothing was special-cased for names.

`NameOfCard (Not (And [HasSupertype Basic, HasType Land]))` writes in both
chooser slots — `necromentiaChoice` (resolution-time; covers Necromentia and
Desperate Research) and `boobyTrapNameChoice` (as-enters). All 4 lines' name
phrases write; Booby Trap's co-chosen opponent and Null Chamber's distributive
two-chooser are separate clause-level gaps, ledgered below.

### Pins

- `badGroupNameRelatum` — **deleted** (ruling above).
- `badNameMatchWrongSort` — **survives, still bites** (ProofsF compiles; an
  `impossible` clause that stopped being impossible would fail to elaborate).
  Untouched by this round: it is a sort mismatch, not an English claim.
- `badTwoChoosersOneSortRead` — **survives, still bites**. Untouched, and the
  round did not write the plain read it refuses. [CR#607.2d] backs it: a linked
  ability refers to the choice its first ability made, and English marks a
  re-choice with a different word ("the LAST chosen color").
- `badSingularNameAgreement` — **new**, covering the new plurality gate:
  [CR#201.2b] states the constraint over two or more objects in a group, so one
  mention of one object has no two members to compare. Rules-grounded, not a
  count.

### Ledger

Ordered by what each wants, with the exact missing constructor.

**Counted search — 2 named lines (Saheeli Rai −7, Deathbellow War Cry), plus
most of the 24.** `Effect.Search : (who) -> (sc) -> (p : Predicate bs Object)`
has no `Quantity` and no mention, so neither the count nor the group constraint
has anywhere to attach. Of the 24 "with different names" lines, 13 write the
constraint on a search's own object. This is a search gap, wholly outside the name family; the
construction is ready for it the day `Search` takes a mention.

**Counted-mention conditions beyond `ExistsGroup`'s reach — Mechanized
Production, Tainted Pact.** Mechanized Production's line composes as far as its
Aura frame allows; Tainted Pact's "you exile two cards with the same name" is
the terminating condition of a `Repeat`-until with a "whichever comes first"
disjunction, which is a loop-control gap.

**The plural read — 3 chooser shapes, measured card by card.** Shape by shape:

1. *One chooser, plural choice* (Seal of the Guildpact, Tablet of the Guilds).
   Measured, not assumed: the plural **choice** already composes at resolution
   time as `CountedGroup (exactly 2) (quality Color)` — `choosable` is `True`
   for a counted mention and `Phrasal (Quality q)` holds. Two things are
   missing. (a) `Static.EntersChoice` takes a bare `QualitySort` with no
   `Quantity`, so the as-enters "choose two colors" cannot be written at all.
   (b) The plural **read** is refused: `OfChosen q` demands
   `countQuality q bs = 1`, and `countQuality` counts only `OneOf` bindings, so
   a `ManyOf` quality binding is invisible to every chosen-quality read
   (verified: the probe fails on exactly that constraint). A third piece,
   "costs {1} less for each of the chosen colors **it is**", additionally wants
   an `Amount` counting the members of a chosen-quality group that the subject
   also is; `CountOf` takes a `Predicate`, and no constructor projects a group
   of chosen qualities.
2. *Two choosers from one distributive clause* (Null Chamber). Wants an
   as-enters chooser with a chooser slot at all — `EntersChoice`'s chooser is
   always you — and a distributive one ("you and an opponent **each** choose"),
   plus the plural read of two separately-made choices. `badTwoChoosersOneSortRead`
   pins the *singular* read of two same-sort choices; the plural read is the
   other half of that cell and has no constructor. Null Chamber's name phrase
   itself now writes.
3. *Three choosers in three clauses with a union read* (Paliano, the High City;
   Regicide). Out of scope, and not narrowly: it wants a draft-time chooser and
   a card-name-scoped memory ("one or more of the colors chosen as you drafted
   cards named Regicide"), which is not a chosen-quality read at all but a
   lookup into a per-name record kept across a draft.

The round's coverage of the plural read is therefore: nothing landed, shape 1
reduced to two named constructors, shape 2 to two, shape 3 declared out of
scope.

**Garth One-Eye — half of it is already writable.** The literal-name LIST needs
nothing new: `NameOfCard (Or [Named (PrintedName "Disenchant"), …])` elaborates
today over all six names (probed and compiled; `DistinctDisjuncts` is satisfied
by six distinct printed names). What is missing is only the **not-yet-chosen
memory**: "a card name that hasn't been chosen" wants a per-permanent record of
which names this ability's earlier activations chose, and quality bindings are
per-clause, not persistent across activations. Not built, per the round's brief.

**Group relatum, now admitted but unwritten.** No supported corpus line writes
`SameNameAs` with a plural relatum. It is admitted because the rules admit it
([CR#201.2a] for the same-name pole over two or more objects, [CR#201.2c] for
the object-against-group shape), not because a card asks for it — which is the
whole point of the ruling.
