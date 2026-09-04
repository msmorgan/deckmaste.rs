---
needs: [core-entity-object-classes]
---
**Separate singular reference, collection, criterion, and filtering concepts
throughout the semantic ASTs.** Use the [`Reference`, `Selection`, `Predicate`,
`Filter`, and `Source` definitions](../../contexts/game-model/CONTEXT.md). A `Reference` denotes
one Object or Player; a `Selection` denotes a collection; a `Predicate` states
a truth criterion over candidates from a known domain; a `Filter` is the
operation that applies one.

Audit the semantic, core, and Idris ASTs and their RON, lowering, rendering,
and evaluation paths for values that violate those boundaries. Move set-valued
`Reference::Source` behavior into a Selection or a specially named predicate,
because Source is a contextual relation—source of an Ability, damage, or
mana—not an Object class. Preserve singular Object-or-Player References so
targeting Players, attaching Auras to Players, and placing counters on Players
do not require duplicate languages.

Make every query constructor's candidate domain explicit enough that a
Player-only predicate cannot silently run over Objects and an Object predicate
cannot silently admit Players. This ticket does not decide whether Object and
Player identities share physical storage; it consumes the public Entity
boundary from `core-entity-object-classes`.

Split the current broad `Sort` vocabulary as needed: `ReferentSort` applies to
Entity-valued References and Selections, while `Amount`, `Pile`, and pile-valued
Selections need an explicitly named value or collection domain. Do not broaden
Reference to amounts or piles merely to preserve the current enum. Acceptance
includes the existing `PilesOf` and `Them(Pile)` paths as well as Entity-valued
queries.
