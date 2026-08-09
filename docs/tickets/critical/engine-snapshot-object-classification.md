---
needs: [engine-snapshot-predicate-breadth]
---
**Engine: evaluate object-kind and subtype predicates against zone-change
snapshots.** `trigger.rs::filter_matches_snapshot` falls through its loud
catch-all for `Predicate::Kind` and `CharacteristicPredicate::Subtype`, even
though a snapshot's card spine can recover the `CardInstance` token flag and
printed face/subtypes.

Implement these leaves from the pre-move snapshot, not from the reminted
destination object. Preserve token identity even though tokens use synthesized
card definitions internally. If derived subtype changes must be captured for
correct LKI, store the derived value at snapshot time instead of silently
substituting the printed face.

Foundations witnesses: Arahbo, the First Fang; Crossway Troublemakers; Gate
Colossus; Gateway Sneak; High-Society Hunter; Infernal Vessel; Kalastria
Highborn; Lathliss, Dragon Queen; Midnight Reaper; Spinner of Souls; Valkyrie’s
Call; Wildborn Preserver; Youthful Valkyrie.

Tests must cover positive and negative subtype matches, nontoken-vs-token dies
filters, and another controlled token entering while a subtype-filtered trigger
watches. None may reach the snapshot catch-all.
