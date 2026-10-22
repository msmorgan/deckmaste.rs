---
needs: []
---
**Engine: a continuous effect that has started applying keeps applying in later
layers even if its source ability is removed during the pass ([CR#613.6]), and the
layer fixpoint can't oscillate.** Latent: no live card authors `LoseAllAbilities`
or `LoseAbility` (Humility, Opalescence, Dryad of the Ilysian Grove, Blood Moon are
`.todo`), but every ability-removing static will hit it.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #6.

## What is wrong at head (crates/deckmaste_engine/src/layer.rs)

- `gather` (`:502`, `:543-564`): after iteration 0, static effects are re-gathered
  from each object's DERIVED ability list, so an effect whose source ability is
  removed later in the pass (layer 6) is dropped from every layer on the next
  iteration.
- `layers()` (`:2333`, `:2369-2378`) stops only when the new signature equals the
  previous one; two alternating states never converge, the loop runs to its cap of
  16, and the result depends on parity; `:2383` `debug_assert!(converged, …)`
  trips. `:2346` "The effect set grows monotonically toward the fixpoint" is false
  once an effect can remove ability sources.
- `depends_on` (`:2117-2131`) probes only the affected set; it has no existence
  edge.

Failures: (a) a creature with a layer-4 static (Dryad of the Ilysian Grove) under
Humility loses the layer-4 effect from iteration 1 on, and the pass converges there;
(b) Humility plus Opalescence, or two "other creatures lose all abilities" creatures,
alternate between iterations.

[CR#613.6]: an effect that starts to apply "will continue to be applied to the same
set of objects in each other applicable layer … even if the ability generating the
effect is removed during this process". [CR#613.8a]: an effect depends on another
if applying the other "would change the text or the existence of the first effect".
[CR#613.8b]: loop members "are applied in timestamp order". Humility ruling
(2009-10-01): "The rest of it will apply even if the permanent loses its ability
before it's finished applying." [CR#613.8b] does not itself say whether both of two
mutually ability-removing effects apply; the Opalescence/Humility rulings
(2006-02-01) give the timestamp-order results to test against.

## Change

Decide each static effect's existence per layer during the pass: once an effect has
applied in a layer, it continues in every later layer ([CR#613.6]); add
[CR#613.8a] existence edges (applying d removes e's source ability) to
`depends_on`. Keep the re-gather only for newly granted statics, and never drop an
effect that already applied. Fix the `:2346` comment.

## Tests

- Dryad of the Ilysian Grove under Humility keeps its layer-4 effect.
- Humility + Opalescence in both timestamp orders match the 2006-02-01 rulings.
- Two "other creatures lose all abilities" creatures converge.
