---
needs: []
---
**An activated ability that functions from the graveyard emits an `Activated`
frame with no `from: Graveyard` whenever it carries no "Activate only …" rider —
so the engine gates its activation to the wrong zone.** Latent, silent, and the
only test covers the case that works.

The sole site that infers the graveyard zone is
`crates/deckmaste_migrations/src/parsers/activated_ability.rs:88-90`:

```rust
if body.contains("~ from your graveyard") {
    riders.from = Some("Graveyard");
}
```

But it sits **inside `peel_activation_riders`**, which early-returns at `:74-75`
when the effect clause has no `. Activate only ` delimiter:

```rust
let Some((body, clause)) = rest.rsplit_once(". Activate only ") else {
    return Ok((effect_clause.to_owned(), None));   // riders = None → from never set
};
```

So the zone inference is reachable **only** on the rider-bearing path. A
graveyard-functioning activated ability with no rider —
`{2}{R}{R}{R}: Return ~ from your graveyard to your hand.` — parses its effect
fine (`effect.rs:1239` matches `~ from your graveyard to your hand` and emits the
`Move(_, Hand)` product) but renders `Activated(cost: […], effect: …)` with no
`from: Graveyard`. The ability functions from the graveyard ([CR#113.6,602.1]);
without the frame's zone declaration the engine treats it as functioning on the
battlefield only — it can't be activated from the graveyard at all, or is
mis-gated. The existing test (`activated_ability.rs:433`) exercises only the
*with*-rider form (`… Activate only during your upkeep.`), which is why this
never surfaced.

This is the same defect class as [[parse-positional-target-reads]]: a structural
fact (the ability's functional zone) inferred as a side effect of an unrelated
parsing step (rider peeling) instead of owned where the fact is known.

## Fix

Move the self-return-from-graveyard zone inference **out of the rider peeler** so
it runs regardless of whether an "Activate only …" rider is present. The effect
production that consumes `~ from your graveyard …` is where the fact actually
lives (`parse_return_to_hand`, and the reanimation sibling at `effect.rs:1562-1567`
`Return ~ from your graveyard to the battlefield`); carry the functional zone on
`ParsedEffect` and let `render` (activated_ability.rs:138) emit `from:`
unconditionally. Keep the self-vs-object discrimination that comment `:83-87`
documents — a `target … from your graveyard` OBJECT description (a *different*
card the effect reaches for) must still NOT set `from: Graveyard`; only the
self-referential `~ from your graveyard` does (the reanimation path
`effect.rs:1574-1578` targets another card and must stay battlefield-functional).

Check whether reanimation-from-graveyard self-return (`Return ~ from your
graveyard to the battlefield`, `effect.rs:1567`) has the same missing-zone gap
and fold it into the same fix.

Verify: a rider-less fixture `{2}{R}{R}{R}: Return ~ from your graveyard to your
hand.` now emits `Activated(cost: […], from: Graveyard, effect: …)`; the existing
with-rider test (`:433`) still emits `from: Graveyard` (now via the effect, not
the peeler); the `target … from your graveyard` object form (`:445`) still emits
**no** `from:`; `cargo xtask generate plugins/wizards` re-emits affected cards
with the zone; `cargo test --workspace` green.
