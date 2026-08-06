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
   previous-active-player binding, and the store invariant — starts neither
   [CR#731.2c], once set always exactly one [CR#731.1] — need engine wiring.

The DayNight declaration itself becomes data: a game-scoped
`Enum(["Day","Night"])` designation, `PerGame`, permanent, neither-yet = key
absent. The untap-step check stays engine-intrinsic permanently; only the
vocabulary moves.

## Daybound/nightbound (second half, needs engine-transform)

[CR#702.145b..702.145c]: enters transformed if night; transforms immediately
(not an SBA) when day/night flips; can't transform except via its
daybound/nightbound ability. Needs the transform verb from `engine-transform`
plus an immediate non-stack transform hook on the designation-change fact.

This is the first consumer that makes back-up double-faced permanents
reachable in production, so the two latent gaps the `engine-transform`
whole-branch review deferred are hard prerequisites:
`engine-transform-byindex-reread-hardening` (a mid-resolution transform can
panic an un-hardened by-index ability re-read) and
`engine-transform-current-face-name-loyalty` (legend-rule name /
named-predicate / loyalty reads still take the front face).
