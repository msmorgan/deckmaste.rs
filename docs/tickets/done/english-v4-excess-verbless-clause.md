---
needs: []
---
# *excess* must not head a verbless clause in *If excess damage was dealt this way*

Orbital Plunge reads "If excess damage was dealt this way, create a Lander
token." After
[`english-v4-spurious-ambiguity-multipliers`](../done/english-v4-spurious-ambiguity-multipliers.md)
gave `noun:excess` `distribution: HeadOnly`, *excess damage* has one premodifier
analysis. The unit still carries wrong Readings in which `noun:excess` heads a
verbless clause: *excess [damage was dealt this way]*.

## Change

- Find the production that lets a bare noun head take a following finite clause
  as its dependent here, and remove the analysis structurally or lexically. No
  consumer-side exclude naming *excess*. The intended analysis of *in excess of*
  (the reason `noun:excess` exists) must keep reading.

## Done when

- Orbital Plunge's line has no Reading in which *excess* is separated from
  *damage*, covered by a `+` Judgment on its real text.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

Census delta (vvqptltk): faces read 28,709 → 28,709; Readings 346,725 → 346,319; lost faces: none.
Judgment set (vvqptltk): `cargo test -p deckmaste_english_v4` passes, including the new `excess_count_noun.txt`.
