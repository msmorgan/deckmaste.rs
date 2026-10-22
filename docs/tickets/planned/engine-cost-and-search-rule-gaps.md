---
needs: []
---
**Engine: four latent cost and search rules the engine gets wrong: unmatched
colored cost reductions, gain-life costs under "can't gain life", the [CR#601.2h]
payment tiers, and the stated-quality search floor in public zones.** All latent at
head (no live card reaches them); each will be hit by the first card of its kind.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #4,
#29, #34, #35.

## 1. A colored or {C} reduction with no matching pip reduces generic (#4)

`crates/deckmaste_engine/src/cast.rs:91-92` (doc: "a colored pip removes ONE
matching colored pip and nothing else") and `reduce_symbol` `:131-135` (a
`Specific(_)` reduction removes a matching pip; no else arm, so an unmatched one is
dropped). Core `CostChange::Reduce(Arc<[CostComponent]>)`
(`crates/deckmaste_core/src/continuous.rs:203-206`) has no colored-only flag; Lean
`CostShift.bySymbols … (coloredOnly : Bool)` and semantics_v2 `BySymbols {
colored_only }` already carry one, but no lowering maps it into core.

[CR#118.7b]: "If a cost is reduced by an amount of colored or colorless mana, but
the cost doesn't require mana of that type, the cost is reduced by that amount of
generic mana." [CR#118.7c] and [CR#118.7d] reduce generic by the excess. Khalni
Hydra ruling (2010-06-15): the reduction also cuts an imposed generic tax, after
the green requirement. So {W}{U}{B}{R}{G} off a {3}{U} spell leaves {0}; the engine
leaves {3}.

**Change:** add a colored-only flag to core `CostChange::Reduce`; when no matching
pip remains and the flag is false, fall through to the generic arm. Tests: the
{3}{U} case; a Khalni-shaped {G} reduction under a generic tax.

## 2. A gain-life cost under "can't gain life" (#29)

`activate.rs:616-618`: "Gaining/setting life as a cost ([CR#119.7]; Invigorate,
Skyshroud Cutter) needs no prior resource — always payable." →
`Action::ChangeLife(_, LifeOp::Up(_) | LifeOp::Set(_)) => true`. The sibling
[CR#119.8] lock has a seam TODO (`:548-557`) and a pin test
(`cant_pay_life_lock_is_a_documented_seam`, `:1438-1451`); [CR#119.7] has neither.
[CR#119.7]: under a can't-gain-life effect "a cost that involves having that player
gain life can't be paid"; [CR#119.5] makes a raising set a gain. Invigorate ruling
(2020-08-07): a replacement of the gain (Sulfuric Vortex's) does not block
payment ([CR#118.11]).

**Change:** replace the comment with a [CR#119.7,119.5] seam and add a pin test
beside the [CR#119.8] one now; when the deontic layer gains a gain-life action, a
gain-life cost or a raising `Set` is unpayable while the player who would gain
(possibly an opponent, not the payer) can't gain life.

## 3. [CR#601.2h] payment tiers (#34)

`payment/fulfill.rs:81-83` defers any library `Search`; `:96-104` defers `DrawCard`;
`:107-109` defers any library-origin `Move` to a non-library zone, hand included.
[CR#601.2h]: "First, they pay all costs that don't involve random elements or moving
objects from the library to a public zone … Then they pay all remaining costs."
[CR#400.2]: library and hand are hidden zones. A library→hand move with no random
element is first-tier; a search that includes a shuffle has a random element.

**Change:** defer a library-origin `Move` or `Search` only when its destination is a
public zone or the cost includes a shuffle or other randomness. Register `DrawCard`
(is an unknown top card a "random element"?) in the UD register
(`docs/engine-adrs.md`).

## 4. Stated-quality search floor in public zones (#35)

`resolve/effect.rs:345-375`: "A STATED quality ([CR#701.23b]) never compels one,
even with a match sitting right there"; `min` is floored to 0 for every non-bare
filter across every zone in `search.from` (`:371-375`); same claim in
`resolve/query.rs:505-516` and the test doc at `:5077-5079`.
`crates/deckmaste_semantics/src/binder.rs:63` calls the zones "hidden
libraries/graveyards". [CR#701.23b] exempts only "searching a hidden zone"; its
example: "Splinter's controller must find the Howling Mine in the graveyard."
[CR#400.2]: the graveyard is public. Eradicate ruling: "The copies must be found if
they are in publicly viewable zones."

**Change:** compute the lower bound per zone, flooring to 0 only for stated-quality
candidates in hidden zones; public-zone candidates use the quantity's own bound
(check whether `Quantity::Range(Some(CountOf(..)), None)` expresses a mandatory
"all" before adding a variant). Correct the `binder.rs:63` doc comment.
