---
needs: []
---
**Give every pin its positive twin in the same module.** Cleanroom review 3,
2026-09-04, §2d census: 84 of 628 pins are twinless.

- `VERIFY.md` requires each `Unspellable` pin to carry its positive twin in
  the same module. 84 pins carry their evidence in another module, in a bench
  file, or nowhere. None is vacuous — every twinless pin's positive form was
  probed and admitted — so this is a discipline defect, not a soundness one.
- Distribution over the pin families: Description and Anaphora 6; Trigger,
  Damage and Zone 4; Keyword 9; Counters and Mana 10; Deontic, Choice and
  Static 25; Cost, Faces, Turn, Copy and Piles 30. `ProofsStatic` is fully
  twinned and is the reference shape — adjacent pairs, every refusal on the
  named implicit.
- Write each twin in the pin's own module next to it: the same sentence with
  the refused part corrected, and a synthetic rules-meaningful sentence where
  no printed card supplies one. Pin modules import no other `Proofs*` module,
  so a twin is never borrowed across modules.
- Wire the check into `idris/scripts/build` beside the bench brace lint, so a
  new twinless pin fails the build.

Size: M. Done when: every `Unspellable` declaration has an adjacent positive
twin in its own module; the check is wired and green; the added count is
reported; build at its module count. Standard constraints apply, including the
RON-shaped constraint.
