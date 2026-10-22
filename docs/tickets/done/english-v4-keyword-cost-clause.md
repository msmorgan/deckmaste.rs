---
needs: []
---
# A keyword before *abilities* or *costs* in a sentence is a premodifier, not a keyword ability with a cost

In running text, *Equip*, *Blitz* and *Dash* before a noun can also parse as
a Keyword Line whose Cost parameter swallows the rest of the sentence:
- *Equip [abilities you activate cost {2} less to activate]*;
- *Blitz [costs you pay cost {1} less …]*;
- *Dash [costs you pay cost {2} less]*.

Each such Reading has the whole declarative clause as the keyword's cost.
Under Right Association the swallowing Reading is the cheapest, because the
relative clause's host becomes *abilities* (or *costs*) rather than the
premodified *Equip abilities*. Right Association measures the whole host
phrase (owner's ruling).

## Owner's ruling (round 4, verbatim)

> Keyword cost parameters swallowing a clause (Equip [abilities you activate
> cost …], Blitz, Dash): inside a sentence, Equip / Blitz / Dash before
> `abilities` or `costs` is a bare keyword premodifier (KeywordPremodifier),
> not a keyword ability taking a cost parameter.

## Witnesses

Three judgments; each endorses only the premodifier Reading, and each lost
its all-endorsed top tier when Right Association landed:
- `keyword_mention_without_parameter.txt:23`: *Equip abilities you activate
  cost {2} less to activate.* (Fighter Class);
- `keyword_mention_without_parameter.txt:24`: *Blitz costs you pay cost {1}
  less for each time you've cast your commander from the command zone this
  game.* (Henzie "Toolbox" Torre);
- `stratum_zero.txt:47`: *Dash costs you pay cost {2} less.* (Warbringer).

## Change

- A keyword's Cost parameter takes the cost forms of a Keyword Line or an
  activation cost [CR#118.1]. It never takes a finite declarative Clause, and
  never one inside a sentence.
- Restrict this through existing categories and features. The keyword's
  catalog-declared parameter frame and the Cost categories already exist;
  name no keyword in the grammar.
- **No new features in `english.ron`.** If the fix cannot be done without
  one, STOP and return to the owner.

## Done when

- The three witnesses have no Reading in which the keyword takes the rest of
  the sentence as its cost.
- Their cheapest Reading is the `KeywordPremodifier` Reading.
- Keyword Lines with real costs (*Equip {2}*, *Equip—Pay 3 life*, *Dash
  {2}{R}*) keep their Readings.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta** (measured on `lsolvqym`): faces read 28,799 → 28,799;
   total Readings 214,300 → 214,182 (Surplus 88,560 → 88,449, over 19 units,
   none rising); lost faces: none.
2. **Judgment set**: 3,028 enforced at `COMPLETED_STRATUM` 6, all hold; 207
   pending.
