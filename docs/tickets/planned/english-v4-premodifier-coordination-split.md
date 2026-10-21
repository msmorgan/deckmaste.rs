---
needs: []
---
# *instant or sorcery spell* is coordinated Premodifiers, never [*instant*] or [*sorcery spell*]

Owner ruling (cost-audit smaller defect): mint this as a defect. A coordinated
Type Word before a carrier noun splits into a Nominal coordination
[*instant*] or [*sorcery spell*]. That is the Noun Phrase split against the
Premodifier analysis.

## The defect

*instant or sorcery spell* has two analyses today:

- the intended one: [*instant or sorcery*] *spell*, a coordination of Noun
  Premodifiers;
- a Nominal coordination [*instant*] or [*sorcery spell*], whose first
  coordinate is a bare Type Word.

The second analysis is one node smaller, so it is also the cheaper one. Other
texts with the same split:

- *Counter target instant or sorcery spell*;
- *Return target artifact or creature card from your graveyard*;
- *you may put an Elf or Tyvar card from your graveyard onto the battlefield*;
- *reveal a creature or land card*;
- *a Bat, Bird, Mouse, or Rabbit card*;
- *copy the next instant or sorcery spell you cast this turn*;
- *Return target artifact or non-Aura enchantment card*. This text also shows
  the related split [*artifact or non-Aura*] *enchantment card*.

## Authority

- `docs/oracle-style-guide.md` §7, "Card types are common nouns and
  attributive modifiers":
  - "Coordinate alternatives explicitly: … `instant or sorcery card`".
  - "A final carrier noun may distribute over the whole coordinated phrase …
    Repeat the carrier when the alternatives have different domains or
    modifiers."
  - So [*instant*] or [*sorcery spell*] would be written with a repeated
    carrier if it were meant.
- `docs/oracle-style-guide.md`, shortest useful checklist, item 6: a bare type
  word names a permanent on the battlefield. In *Return target artifact or
  creature card from your graveyard*, the split would therefore pair a
  permanent with a graveyard card. No Oracle text repeats a type with and
  without a carrier (*X or X card*): there are 0 matches in the snapshot.
- CGEL, Ch. 15 §1.3.4, pp. 1285–1286 (*long poems and essays*): with a
  dependent next to a coordination, the dependent applies either to one
  coordinate or to the whole coordination. Here the dependent is the head that
  follows. Oracle's register (OSG §7) fixes the distributive structure.

## Change

- Exclude a Nominal coordination whose last coordinate carries a noun
  Premodifier when its first coordinate is a bare noun that the same Noun
  Premodifier coordination would take.
- Express this through the coordination's features (Premodifier presence or
  Conversion), never by naming Type Words.
- Before landing, confirm against the census that no face reads only through
  the split.

## Acceptance set

All figures are from the cost-audit evaluation data, measured on `loxuknun`.

- **Corpus sample:** 14 Segments lose 1,605 Readings, out of a combined Surplus
  of 3,079.
- **Judgment set:**
  - 17 entries lose 164 Readings, out of a Surplus of 274.
  - No entry's endorsed Readings all use the split.
- **Corpus reach:**
  - 638 ambiguous units contain *X or Y spell/card/permanent*, with 22,525
    Readings between them.
  - Expected reduction: roughly half of the Surplus of the affected Segments,
    as in the sample.

## Done when

- The examples above have only the Premodifier coordination, each pinned by a
  `+` Judgment on real text.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
