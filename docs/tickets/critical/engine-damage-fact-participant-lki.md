---
needs: []
---
**A damage fact's participants are bare ids, so a history read cannot name a
creature that has since left the battlefield.** Found by
`core-regions-witness-fixtures`; it is the blocker on Predator Ooze. Standard
constraints apply.

## The gap

`FactView::of`'s `GameEvent::DamageDealt` arm records `source` and `patient`
through the `part` helper, which yields `Part::Obj(id)` for any card-backed
participant — no last-known-information snapshot, unlike the past-form
`ZoneChange` arm, which carries `Part::Gone(snapshot)`. Once that object has
moved, `GameState::part_matches` finds `self.objects.get(id)` empty and falls
back to matching only `Predicate::Any`, so an identity filter over the
participant is false.

"Whenever a creature dealt damage by this creature this turn dies" is exactly
the case where the damaged creature is gone at read time: the candidate is the
dying creature's snapshot, the pattern is
`Where(Happened(Damage(source: Ref(This), to: Ref(It)), ThisTurn))`, and the
recipient half can never match. [CR#608.2i] is explicit that a look-back read
does not require the object to still be where it was.

The source half already works (the Ooze is alive), which localizes the defect
to the recorded participant rather than to `Where`, `Happened`, or the
candidate binding.

## Acceptance

`crates/deckmaste_engine/tests/region_witnesses.rs`'s
`predator_ooze_counts_only_creatures_it_damaged_this_turn` loses its
`#[ignore]` and passes unchanged, including its control (a creature that dies
the same turn undamaged adds no counter).
