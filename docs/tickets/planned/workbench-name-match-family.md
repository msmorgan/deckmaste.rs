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
