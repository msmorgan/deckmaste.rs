---
needs: []
---
**Replace the catch-all `Normal | TwoFaced { layout, front, back }` model with
CR-faithful Card Form and Characteristics vocabulary.** The target-vocabulary
definitions are [`Card Face`, `Characteristic`, and `Alternative
Characteristics`](../../contexts/game-model/CONTEXT.md). `CardFace` must denote only something
the CR calls a face; `Characteristics` is the shared value used wherever a
face or rules-defined alternative supplies characteristics.

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
semantic, core, and Idris models and in authored RON. Migrate extraction,
lowering, card lookup, generated fixtures, and all layout consumers.

Acceptance includes one double-faced, split, flip, and Adventure witness; each
witness exposes exactly the faces and alternative Characteristics its CR form
permits. Do not preserve the old model through compatibility aliases.
