---
needs: []
---
# "As though its source had <keyword>" has no by-reference form

Standard constraints apply. Ruled by the owner, 2026-10-24 (below).

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

## Ruling (owner, 2026-10-24)

- "<subject> <verbs> as though …" is an indicative, mandatory statement [CR#609.4], not a
  "can"/"may" permission [CR#101.2].
- New `StaticSpec.asThough (subj : Subject) (action : Action) (asThough : AsThough)`, stepped like
  `can` but not recorded as permitted. It reuses `AsThough` and `AsThough.condition`; no new
  `AsThough` arm. `StaticSpec.can` stays strictly for "can"/"may" permissions.
- *Everlasting Torment*: an indicative `asThough` over `dealDamage` dealt by any source, with the
  condition `holds (is <its source> (hasKeyword wither))`. *Phyrexian Unlife*: the same over
  damage dealt to you, with infect, under
  `onlyWhile (compareAmt (lifeTotalOf you) .atMost 0)`. The hand-written replacements go.
- Wither and infect apply to the source treated as having them [CR#702.80c,702.90d], and a
  source that really has the keyword gets the effect once [CR#120.3d]. The rows' subjects change
  only if the CR requires it.
  (Infect's any-zone rule, the counterpart of [CR#702.80c], is [CR#702.90e]; [CR#702.90d] is its
  last-known-information rule, the counterpart of [CR#702.80b]. Neither changes the rows: both
  are written of "an object with" the keyword, which the source treated as having it is
  [CR#609.4], so `this_` stays their subject.)
- *Hotshot Mechanic* and *Cloudspire Captain* ("crews Vehicles / saddles Mounts as though its
  power were 2 greater") move from `can` to the new form, as does every other indicative
  "as though" static the bench spells with `can`.
- The raw `asThough := some (.condition …)` sites in `Definitions/Keywords.lean` (haste) are
  fixed here if they become this form or `holds` (authoring-surface §2.5).

## Tests

- `everlastingTorment` and `phyrexianUnlife` re-authored by reference, checking clean.
- The indicative form is not recorded as a permission.
- *Hotshot Mechanic* still checks.
- A source that really has wither or infect gets the keyword's effect once.
- Both cards' other lines (`cant … gainLife`, the damage-prevention prohibition, the
  loses-the-game exception) are unchanged.
