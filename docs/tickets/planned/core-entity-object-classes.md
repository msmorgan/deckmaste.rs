---
needs: [core-copy-grammar]
---
**Replace the exclusive `ObjectKind` axis with the Entity/Object boundary and
nonexclusive CR Object classes.** The target vocabulary is defined in the
[Game Model glossary](../../contexts/game-model/CONTEXT.md): a Player and an Object are both project
Entities, but a Player is not a CR Object; Card, copy of a card, Token, Spell,
Permanent, Emblem, and ability on the stack are overlapping classifications
from [CR#109.1].

Today `ObjectKind` conflates three questions: whether a candidate is a Player
or Object, what represents an Object, and which current rules roles it has. It
also makes the false claim that Players are Objects. An exclusive enum cannot
say that a card on the stack is both a Card and Spell or that a battlefield
Token is both a Token and Permanent.

Change the semantic and core predicate languages to make their candidate
domain explicit at the Entity boundary. Replace `Predicate::Kind(ObjectKind)`
with independently testable Object classes plus a Player classification at the
Entity level. Carry the same distinction through Idris, lowering, rendering,
candidate enumeration, snapshots, and the engine classifier. The spell and
permanent tests may be derived from the relevant state, but their public names
and behavior remain the CR classes rather than storage tags.

This ticket deliberately does **not** choose whether Players and Objects occupy
one slotmap or separate stores. Existing player proxies may remain behind an
adapter until that representation is relitigated; they must no longer leak the
claim that a Player is an Object into public types, filters, or prose.

Acceptance: a single candidate can satisfy every applicable Object class;
Player-only predicates do not enter the Object domain; representative Card +
Spell and Token + Permanent conjunctions work in Rust and Idris; and repository
prose contains no affirmative "players are objects" claim.
