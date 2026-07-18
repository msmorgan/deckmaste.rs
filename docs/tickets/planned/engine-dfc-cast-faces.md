---
needs: [engine-transform]
---
Casting a chosen face of a two-faced card — the cast-time face mechanic, distinct
from transform-in-place (`engine-transform`, which flips an already-on-battlefield
permanent). Two populations:

- **Modal DFC play-either-face** (~93 `modal_dfc` cards; grammar + extraction
  already done). At cast/play time the player chooses which face to put on the
  stack; only that face is evaluated for legality and cost, and only it is put on
  the stack [CR#712.11b,712.11c]. A resolving DFC spell that becomes a permanent
  enters with the same face that was up on the stack [CR#712.13]. Playing an MDFC
  as a land chooses one land face before it enters [CR#712.12].
- **Back-face / "transformed" casting** — casting a card's back face as a
  consequence of an effect (Siege "cast the back face", disturb, etc.), including
  casting a nonmodal DFC "transformed"/"converted" so the spell is put on the
  stack with its back face up and only the back face's characteristics, its mana
  value still calculated from the front face's mana cost [CR#712.8c,712.11a]. The
  nonmodal (Siege/TDFC) subset needs those cards extracted first
  (`pipeline-layout-extraction`).

Builds on the runtime face-state substrate from `engine-transform` (which face an
object presents; face-aware base characteristics). Adds the cast-subsystem face
selection that `cast.rs` lacks today — every cast site reads the front face via
`derive::face`, so a two-faced card is only ever castable as its front face. This
is the census `modal_dfc` engine line ("play-either-face + back-face casting
rules") and the third battles dependency for back-face casting. `[design]`.
