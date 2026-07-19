---
needs: [engine-transform, engine-transform-byindex-reread-hardening, engine-transform-current-face-name-loyalty]
---
Day/night game state, daybound/nightbound transforms (~236 cards). Goal: retire
the four special-cased variants — `Action::BecomeDay`/`BecomeNight`,
`EventFilter::BecameDay`/`BecameNight` — by riding the generic designation
machinery, then build daybound/nightbound on top.

## Current state (2026-07-18 deep-dive)

Half-graduated already. Storage is generic: `DesignationStore.game` holds
`DesignationValue::Mode("Day"/"Night")`. Event routing is generic:
`eval.rs` matches `BecameDay` against the `DesignationChanged{name:"DayNight",
becomes:"Day"}` fact. The grant path is a loud seam
(`resolve/action.rs` `todo!("engine seam: day/night designations")`). Zero
corpus cards, zero macros, no daybound/nightbound anywhere. Only the
*vocabulary* is special-cased; nothing blocks the restructure.

## Missing machinery (in dependency order)

1. Idris `Scope` gains `Game` (`Core.idr` has Object/Player only) — without it
   day/night never crosses the soundness gate. Tracked in the
   `idris-mirror-enum-gaps` cluster; resolve that entry with this ticket.
2. A game-scope grant verb (`GetDesignation` is player-scope v1).
3. A `to:` value discriminator on `EventFilter::DesignationChanged` — today it
   is `{name, of}` only, so one Enum designation's Day-vs-Night transitions are
   not distinguishable generically (the exact reason `BecameDay`/`BecameNight`
   exist as variants).
4. A game-scope read path — `StatePredicate::Designated` eval covers
   objects/players only; "if it's day" has no expressible condition.
5. The untap-step turn-based flip check [CR#502.2]: if day and the previous
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
