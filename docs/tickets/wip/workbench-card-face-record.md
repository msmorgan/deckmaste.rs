---
needs: []
---
**Make `Characteristics` its own record; a `CardFace` has characteristics
and whatever its layout adds.** Ruling 2026-09-04 on the alias caveat of
`card-form-characteristics-model`: `Experimental.Card.Characteristics` is
`Characteristics = CardFace`, a transparent alias, so a flip card's
alternative characteristics and an adventure's typecheck as a face and
`FaceLaws`/`Macros.backFace` accept them.

Fix: `record Characteristics` holds the characteristic set [CR#109.3];
`record CardFace` holds a `Characteristics` plus the face-only data a layout
adds (nothing for a single-faced card today; keep the slot positional and
RON-shaped). `FlipCard`'s `alternative` and `Adventurer`'s `adventure` are
typed `Characteristics`; `FaceLaws` is over `CardFace`; a pin refuses
`Macros.backFace` on a flip card, probed non-vacuous. Mirror the split in
core if `CardFace` is still an alias there (`crates/deckmaste_core`), with
the drift guard re-run.

Size: S. Done when: no alias remains; the pin refutes; every face witness
typechecks; workspace tests green; build at its module count. Standard
constraints apply.
