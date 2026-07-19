---
needs: [core-copy-grammar]
---
Make `ObjectKind` the substrate axis only; express spell and permanent as
zone-derived predicates. Today `ObjectKind` (`deckmaste_core/src/filter.rs:26`)
conflates two axes: the SUBSTRATE (card, copy-of-a-card, token, ability, emblem) and
a ZONE-ROLE. "Spell" is enumerated as an `ObjectKind` but is really derived from zone
— `object_kind` returns `Spell` for a card on the stack
(`deckmaste_engine/src/target.rs:52`, `ObjectSource::Card(_) if zone == Stack =>
Spell`). Meanwhile the parallel battlefield-role, "permanent", is NOT an `ObjectKind`
at all — it is a zone predicate (`Predicate::State(InZone(Battlefield))`,
`filter.rs:383,558`) plus a `Permanent` filter atom (`filter.rs:144`). So the two
zone-roles are modeled inconsistently, and [CR#109.1] lists spell and permanent as
distinct kinds that overlap the substrate (a battlefield token is a token AND a
permanent; a card on the stack is a card AND a spell — a flat mutually-exclusive enum
can't hold both).

Rework so `ObjectKind` = substrate (`Card`, `CardCopy`, `Token`, `Ability`, `Emblem`,
`Player`), and "spell" / "permanent" are uniformly zone-derived predicates (matching
how "permanent" already works). "Counter target spell" becomes substrate + on-stack.
This also de-conflates the copy classification `core-copy-grammar` added: a copy is
`CardCopy` substrate regardless of zone, and `object_kind` no longer flips
`Spell`↔`CardCopy` by zone (`target.rs:52,65`) — the copy-cease SBA scopes on
`CardCopy` + not-on-stack instead of a zone-conditional kind.

Scope: ~20 `ObjectKind::Spell` use sites plus heavy render-layer dependence
(`deckmaste_cards/src/render/ability.rs` prints "spell" from
`Predicate::Kind(ObjectKind::Spell)` — needs a spell-predicate render rule). Surfaced
by (not caused by) `core-copy-grammar`; sequenced after it so it can simplify that
ticket's `CardCopy` classifier rather than collide with it.
