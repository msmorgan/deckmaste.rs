---
needs: []
---
# "As though its source had <keyword>" has no by-reference form

Standard constraints apply. Proposed, not ruled.

## Problem

*Everlasting Torment* ("All damage is dealt as though its source had wither.") and *Phyrexian
Unlife* ("As long as you have 0 or less life, all damage is dealt to you as though its source had
infect.") are written out in full over existing primitives in `lean/Semantics/Cards/Damage.lean`
(`everlastingTorment`, `phyrexianUnlife`): wither's row (a replacement of `markDamage` putting
−1/−1 counters) and infect's player row (a replacement of `loseLife` putting poison counters) are
restated for every source. The cards do not refer to the keyword, so they do not compose with the
keyword's own rows in `Definitions/Keywords.lean`: a change to wither's or infect's definition
would not reach them.

`AsThough` (`lean/Semantics/Abilities.lean`) attaches only to the deontics `cant`, `can`, `must`
and `gatedBy`, with the arms `condition`, `mana` and `greater`. None says "this source is treated
as having keyword K".

## Rules

"As though" effects treat the game exactly as if the stated condition were true, for that effect
only [CR#609.4].
([CR#609.4b] is the mana case only, so it is not the rule for this ticket.) The keyword rows being
referenced: wither makes damage to a creature put −1/−1 counters instead of marking damage
[CR#702.80a]; infect makes damage to a player give poison counters rather than life loss
[CR#702.90b]; both through [CR#120.3].

## Proposal (proposed, not ruled)

A static rule carrying "as though its source had <keyword>", applying to the damage (all damage, or
damage to a player) and resolving to the keyword's own rows, so these cards compose with wither's
and infect's definitions. The form must decide: what the keyword's rows are applied to (sources,
not permanents), how it interacts with a source that really has the keyword (once, not twice), and
whether it is a new `AsThough` arm or a new `StaticSpec`.

## Tests

- `everlastingTorment` and `phyrexianUnlife` re-authored by reference, checking clean.
- A source that really has wither or infect gets the keyword's effect once.
- Both cards' other lines (`cant … gainLife`, the damage-prevention prohibition, the
  loses-the-game exception) are unchanged.
