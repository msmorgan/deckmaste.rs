---
needs: [workbench-attack-agent-role]
---
**Put supertypes on the type line, in printed order.** Ruling 2026-09-04.
`Words.TypeLine` is `MkTypeLine subs tys` and supertypes live apart as
`Characteristics.supers` (with a second separate slot on
`Card.SharedLineSplit`), while the type line by definition carries card
types, supertypes and subtypes [CR#205.1] and the printed order is
supertypes, card types, subtypes [CR#205.4a,205.3a].

Fix: `MkTypeLine (supers : List Supertype) (tys : List CardType) (subs :
List Subtype)`; delete `Characteristics.supers` and `SharedLineSplit`'s
separate `supers`; move the `CardSupers` frame law onto the line; retype
`lineNonEmpty`, `addedFits` and every projection; re-spell every witness
(the `Macros.card` line argument and the type-changing effects that build
a `TypeLine`) and every pin; no supertype-versus-type pin: a supertype is independent of card type
[CR#205.4b], so no such pairing is rules-meaningless. Mechanical after the
record change; the build's face laws catch the rest.

Size: S–M. Done when: no supertype field exists outside `TypeLine`; every
witness spells its printed line in printed order; build at its module
count. Standard constraints apply, including the RON-shaped constraint.
