---
needs: []
---
**Replace the catch-all `Normal | TwoFaced { layout, front, back }` model with
CR-faithful Card Form and Characteristics vocabulary.** The target-vocabulary
definitions are [`Card Face`, `Characteristic`, and `Alternative
Characteristics`](../../contexts/game-model/CONTEXT.md). `CardFace` must denote only something
the CR calls a face; `Characteristics` is the shared value used wherever a
face or rules-defined alternative supplies characteristics.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Represent these forms distinctly rather than treating all of them as
two-faced cards:

- nonmeld double-faced cards have front and back Card Faces; the meld
  oversized-face exception must not be forced into the same pair
  ([CR#712.1]);
- split cards have two card faces on one side ([CR#709.1]);
- flip cards have one card face with normal and alternative characteristics
  used according to flipped status ([CR#710.1]); and
- adventurer cards have normal characteristics plus alternative Adventure
  characteristics, not an Adventure face ([CR#715.2]).

Choose the smallest sum and product shapes that preserve those distinctions in
`deckmaste_core` and the Idris workbench (`idris/src/Experimental/`) and in the
core-facing RON. Migrate lowering's core-facing output, card lookup, generated
fixtures, and all layout consumers.

Overlaps the workbench card-shape tickets: `workbench-levelers` and
`workbench-prototype` add inner-face wrappers beside
`SingleFaced`/`Transforming`/`Adventurer`, and `workbench-supplements-out`
trims the workbench card-type set; whichever runs second reconciles the names.

Acceptance includes one double-faced, split, flip, and Adventure witness in
`deckmaste_core` and the workbench, with `cd idris && ./scripts/build` green at
its module count; each witness exposes exactly the faces and alternative
Characteristics its CR form permits. Do not preserve the old model through
compatibility aliases.
