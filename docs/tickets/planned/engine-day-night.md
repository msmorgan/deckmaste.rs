---
needs: [engine-transform, engine-transform-byindex-reread-hardening, engine-transform-current-face-name-loyalty]
---
Day/night game state, daybound/nightbound transforms (~236 cards). The generic
grammar is now in place: effects set
`SetGameDesignation("DayNight", "Day"/"Night")`, and triggers use
`DesignationChanged(name: "DayNight", to: "Day"/"Night")`. Build the
day/night lifecycle and daybound/nightbound on top.

## Current state (2026-07-18 deep-dive)

Storage, mutation, and event routing are generic: `DesignationStore.game`
holds `DesignationValue::Mode("Day"/"Night")`; `SetGameDesignation` emits and
applies the transition; `DesignationChanged{name,to}` filters the fact. Zero
corpus cards, zero macros, no daybound/nightbound anywhere.

## Missing machinery (in dependency order)

1. A game-scope read path — `StatePredicate::Designated` eval covers
   objects/players only; "if it's day" has no expressible condition.
2. The untap-step turn-based flip check [CR#502.2]: if day and the previous
   turn's active player cast no spells, it becomes night; if night and they
   cast two or more, day. The predicate is already data-expressible
   (`EventCount(Cast{who: …}, Lookback::LastTurn)`); the firing point,
   previous-active-player binding, and the store invariant — starts neither and,
   once set, always exactly one [CR#731.1] — need engine wiring. (When it is
   neither, the untap-step check doesn't happen [CR#731.2c].)

The DayNight declaration itself becomes data: a game-scoped
`Enum(["Day","Night"])` designation, `PerGame`, permanent, neither-yet = key
absent. The untap-step check stays engine-intrinsic permanently; only the
vocabulary moves.

## Daybound/nightbound (second half, needs engine-transform)

Daybound's three statics [CR#702.145b] and nightbound's two [CR#702.145e]:
enters transformed if it is night; "As it becomes night/day" (an event static, not
a trigger) transforms it if the matching face is up; can't transform except due to
its daybound/nightbound ability. Plus the any-time rules, which are immediate and
not state-based actions: a front-face-up daybound permanent at night is
transformed [CR#702.145c], a back-face-up nightbound permanent during day likewise
[CR#702.145f]; with a daybound permanent and neither day nor night, it becomes day
[CR#702.145d]; with a nightbound permanent, neither designation, and no daybound
permanent on the battlefield, it becomes night [CR#702.145g]. Needs the transform
verb from `engine-transform` plus an immediate non-stack hook evaluated whenever
those conditions can become true (designation change, entry, control change).
(Corrected 2026-10-21: the earlier text glossed [CR#702.145c] as "transforms
when day/night flips", which is [CR#702.145b]'s "As it becomes", and omitted
[CR#702.145d,702.145f,702.145g];
`docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md` #17.)

This is the first consumer that makes back-up double-faced permanents
reachable in production, so the two latent gaps the `engine-transform`
whole-branch review deferred are hard prerequisites:
`engine-transform-byindex-reread-hardening` (a mid-resolution transform can
panic an un-hardened by-index ability re-read) and
`engine-transform-current-face-name-loyalty` (legend-rule name /
named-predicate / loyalty reads still take the front face).
