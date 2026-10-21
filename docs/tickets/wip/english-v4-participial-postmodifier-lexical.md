---
needs: []
---
# A participial postmodifier is never headed by the auxiliary *been*

## Witness

**Stalwart Successor:** *Whenever one or more counters are put on a creature
you control, if it's the first time counters have been put on that creature
this turn, put a +1/+1 counter on that creature.* 6 of its 12 Readings (on the
default line after `english-v4-multiword-term-split`) read *[first time
counters have] been put on that creature*: a bare relative *counters have*,
then *been put on that creature* as a participial postmodifier.

## Owner's ruling (verbatim)

"A bare past-participial postmodifier (reduced relative) can never be headed by
the perfect auxiliary participle *been* (`*a creature been put on the
battlefield`; CGEL Ch. 14 §9). Exclude `VerbKind: Auxiliary` on
`ParticipialPostmodifier` and `SingleParticipialPostmodifier` (the same
existing `Exclude("….VerbKind", "Auxiliary")` equation that `Passive` already
uses)."

Citation check: the section is right. CGEL, Ch. 14 §9, pp. 1264–1265: "Past-participial
modifiers are bare passives (Ch. 16, §10.1.1)" (*a letter written by my
great-grandfather*). Ch. 16 §10.1.1, pp. 1429–1430, defines bare passives as
passives that contain neither *be* nor *get* (*The guy mauled by our neighbour's
dog is in intensive care*). *Been put …* contains *be*, so it is no bare passive
and no past-participial modifier.

## Change

In `english.ron`, add `Exclude("head.VerbKind", "Auxiliary")` to
`ParticipialPostmodifier` and `Exclude("modifier.VerbKind", "Auxiliary")` to
`SingleParticipialPostmodifier`. These are existing features and values, and
they match `Passive`. Cite CGEL Ch. 14 §9, pp. 1264–1265, and Ch. 16 §10.1.1,
pp. 1429–1430, in the comment.

## Done when

- No Reading has a participial postmodifier headed by *been*.
- A `+` judgment for Stalwart Successor asserts the intended analysis: *if
  it's the first time counters have been put on that creature this turn* as
  Adjunct of *put* [CR#603.4], and that no Reading has *been put on that
  creature* as a postmodifier (a `!` assertion).
- No new feature in `english.ron`.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta** (`mmzmnxwo`, against its claim parent): faces read 28,706 →
   28,706; Readings (product) 218,547 → 218,535; Surplus 91,328 → 91,316;
   cheapest tier singleton in 14,544 of 17,220 ambiguous units, unchanged. Only
   Stalwart Successor and Botanical Brawler fell, 12 → 6 each; no unit rose;
   lost faces none; `strata --check` passes.
2. **Judgment set** (`mmzmnxwo`): 3,010 enforced, 200 pending, all hold; the new
   Stalwart Successor entry in `participial_uses.txt` fails on the claim parent.
