---
needs: []
---
**Give the `Card` sort a leveler wrapper: `Leveler (inner : CardFace) (bands :
List LevelBand)` [CR#711.2a].** Fresh workbench review 2026-09-03, R3,
resolved by ruling.

**Ruling (settled 2026-09-03): layout features are in the workbench's remit,
and the leveler shape is an inner-face wrapper.** `Card.Card` (`Card.idr:365`)
gains a constructor beside `SingleFaced`/`Transforming`/`Adventurer` that
wraps one `CardFace` rather than pairing two, because a leveler has one face
whose printed box and text are replaced band by band. A band is a level range,
a printed box, and the abilities that band grants — `LevelBand (range :
LevelRange) (box : PrintedBox) (text : AbilitySeq …)`. The lowering shape is
`Layout::Leveler { inner, bands }`.

25 supported cards print a level-up box (e.g. Brimstone Mage); today none of
them has a spelling, because `CardFace.box : Maybe PrintedBox` is one box and
`CardFace.text` one sequence.

Design points to settle in the round, not before: whether the range is
`(from, Maybe to)` or a closed pair with an open last band; whether the
level-up activated ability is a band-level ability or an ability on `inner`
(it is printed on the base face, so `inner`); and which face laws
(`Card.FaceLaws`) apply to the bands — a band has no name, cost or type line
of its own.

Every slot is positional and required, and any optional content arrives with
the wrapping macro that spells it, per
`docs/decisions/card-authoring-binds-no-implicits.md`.

Size: M.

Done when: Brimstone Mage is a typechecking bench witness with all its bands,
its level-up ability reading from `inner`; a second leveler with a different
band count is a second witness; a pin refuses a band whose range overlaps
another, or whose box contradicts the CR's band rules, probed non-vacuous; the
build is 44/44 with 0 errors and 0 warnings. Standard constraints apply, plus
the RON-shaped constraint: a core constructor is admissible only if the RON
re-emitter can produce it from a RON node, and a macro only if it names a RON
macro (`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
