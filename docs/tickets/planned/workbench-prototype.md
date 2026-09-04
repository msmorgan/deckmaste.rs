---
needs: [workbench-levelers]
---
**Give the `Card` sort a prototype wrapper: `Prototype (inner : CardFace)
(alt : (cost, box))` [CR#702.160a].** Fresh workbench review 2026-09-03, R3,
resolved by ruling.

**Ruling (settled 2026-09-03): prototype is an inner-face wrapper, the same
idiom as the leveler.** A prototype card is one face with a second, smaller
mana cost and printed box; it is not two faces, so `Transforming`/`ModalDfc`
are the wrong shape and `CardFace.cost : Maybe ManaCost` /
`CardFace.box : Maybe PrintedBox` are one each. The constructor wraps one
`CardFace` and carries the alternative pair. The lowering shape is
`Layout::Prototype { inner, alt }`.

19 supported cards print Prototype (e.g. Arcane Proxy).

Depends on `workbench-levelers`: that ticket lands the inner-face wrapper
idiom — how a wrapper interacts with `Card.FaceLaws`, what the wrapped face
owes, and how the lowering names it — and this one follows it rather than
inventing a second answer.

The alternative's type line is *not* a slot: prototype changes cost, colour,
and the printed box, and the round records which of those the pair carries
against [CR#702.160a] rather than assuming. Every slot is positional and
required, per `docs/decisions/card-authoring-binds-no-implicits.md`.

Size: S–M once the leveler wrapper exists.

Done when: Arcane Proxy is a typechecking bench witness with both costs and
both boxes; a pin refuses a prototype whose alternative cost is absent or
whose alternative box contradicts the CR's prototype rules, probed
non-vacuous; the wrapper reads through the same face laws the leveler wrapper
established; the build is 44/44 with 0 errors and 0 warnings. Standard
constraints apply, plus the RON-shaped constraint: a core constructor is
admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
