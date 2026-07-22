---
needs: []
design: true
---
**[design] Drive English predicate recognition and lowering from shared
declarative lexical frames.** The current grammar derives most predicate
behavior from a few `Vocab` name checks and otherwise offers nearly every
dependent shape to every verb. Replace that with one frame interface shared by
named/irregular vocabulary, regular vocabulary, and catalog-derived keyword
actions. A lexical identity may expose multiple frames.

Frames must describe forbidden, optional, and required direct and indirect
objects; adjective, prepositional, infinitival, ability, scalar/statistic, and
other supported complements; selected prepositions versus free PP adjuncts;
licensed temporal/manner bare nominals; licensed verb-particle pairs; and
proform behavior. Recognition and lowering must carry the same selected frame,
without reclassifying a lowered predicate by matching `Vocab` variants.

Migrate the existing surface patches into the shared metadata path:

- remove the duplicated `combat | time | turn` and `way` nominal checks and the
  `attack | block` attachment check;
- replace globally attachable `in | out` particles with licensed pairs;
- make indirect-object and selected-PP complements reachable and render them in
  source order; and
- remove the `be`, `have`, and `do` frame-control checks from grammar flow.

Add causal positive and negative tests proving that an intransitive frame
cannot acquire an object, selected PPs remain complements, indirect objects are
reachable, particles require a licensed pair, multiple frames can coexist, and
all lexical sources use the same interface. Source-independent rendering must
round-trip each positive fixture.

Use `cargo xtask english recovery --json` only as an ephemeral before/after
check. Temporary regressions are acceptable during refactoring, but this ticket
is complete only when its coherent frame migration leaves structural recovery
no worse than its starting census. Do not commit corpus text or a baseline.

## Completion

- Predicate valency, complement/adjunct roles, bare nominal adjuncts, particles,
  and proforms flow through shared frame metadata.
- The audited spelling/name branches are gone or documented as genuine
  closed-class or irregular morphology.
- Positive and negative frame tests, the English parser suite, source-free
  rendering, and a non-regressing live corpus census pass.
