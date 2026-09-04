---
needs: [core-copy-grammar]
---
**Replace the exclusive `ObjectKind` axis with the Entity/Object boundary and
nonexclusive CR Object classes.** The target vocabulary is defined in the
[Game Model glossary](../../contexts/game-model/CONTEXT.md): a Player and an Object are both project
Entities, but a Player is not a CR Object; Card, copy of a card, Token, Spell,
Permanent, Emblem, and ability on the stack are overlapping classifications
from [CR#109.1].

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Today `ObjectKind` conflates three questions: whether a candidate is a Player
or Object, what represents an Object, and which current rules roles it has. It
also makes the false claim that Players are Objects. An exclusive enum cannot
say that a card on the stack is both a Card and Spell or that a battlefield
Token is both a Token and Permanent.

Change core's predicate language to make its candidate domain explicit at the
Entity boundary. Replace `Predicate::Kind(ObjectKind)` with independently
testable Object classes plus a Player classification at the Entity level. Carry
the same distinction through the Idris workbench (`idris/src/Experimental/`),
lowering's core-facing output, candidate enumeration, snapshots, and the engine
classifier. The spell and permanent tests may be derived from the relevant
state, but their public names and behavior remain the CR classes rather than
storage tags.

This ticket deliberately does **not** choose whether Players and Objects occupy
one slotmap or separate stores. Existing player proxies may remain behind an
adapter until that representation is relitigated; they must no longer leak the
claim that a Player is an Object into public types, filters, or prose.

Overlaps `workbench-ability-kind-fold`, which folds the workbench's `Kind`
family so an ability on the stack is an `Object` with an `AbilityP` payload;
whichever runs second reconciles the names.

Acceptance: a single candidate can satisfy every applicable Object class;
Player-only predicates do not enter the Object domain; representative Card +
Spell and Token + Permanent conjunctions work in `deckmaste_core` and in the
workbench, with `cd idris && ./scripts/build` green at its module count; and
repository prose contains no affirmative "players are objects" claim.
