---
needs: [english-structural-recovery-zero]
---
**Nominal-coordination shared-determiner elevation and attachment repair.**
Sibling of `english-predicate-generics` and `english-surface-fact-diet`;
adjudicated from an external review 2026-07-23 and extended by the Bonfire
bracket audit.

- Split coordination of nominal material under one determiner (`target
  artifact or enchantment` — one selection) from coordination of complete noun
  phrases (`target artifact and target enchantment` — two). The shared
  determiner lives above every conjunct; no first conjunct carries it as an
  accidental special case.
- Preserve the distinction in the grammar provenance as well as the lowered
  AST. `target instant or sorcery card` remains coordinated modifier material,
  not two target heads.
- Correct the larger recipient/source and repeated-theme ambiguities exposed by
  Bonfire and Hail Storm. Bonfire is one `damage` object with one `to` PP; that
  PP coordinates shared-target `player or planeswalker` with independently
  determined `each creature ...`, and the zero relative belongs only to
  `creature`.
- Survey `CoordinatedPredicateObject` for the same defect. Its members are all
  complete predicate objects and it owns no shared determiner/material, so it
  needs no analogous split.

Adjudication notes (no action here): typed activation-cost components
(replacing `Cost::Components(Vec<Phrase>)`, an 18-variant catch-all bag)
belong to the campaign's existing cost slice, not this ticket. The partitive
critique was partially wrong — `any number of X` currently recovers rather
than parsing as a PP complement; when that family lands, pick its shape
against the real three-way split: partitive selection (`each of X`) vs
measure (`the number of X`) vs container (`a deck of cards`, barely
oracle-relevant).

Recovery census byte-identical and round-trip stays at zero. Standard
constraints apply.

## Completion

- Added `CoordinatedNominalPhrase`, whose single determiner scopes over bare
  nominal members, while `CoordinatedNounPhrase` remains coordination of
  complete noun phrases. Renderer and syntax traversal support both shapes.
- Added categorical grammar constituents for shared target heads, coordinated
  recipient/source PPs, and repeated `damage` themes. Attachment preference is
  carried by grammar features; there are no card-name or suffix checks.
- Bonfire, Hail Storm, Disenchant, and the passive `dealt to target player or
  planeswalker` ambiguity have typed structural regression tests, with
  repeated-target and coordinated-modifier controls.
- The supported-card bracket audit changes 763 of 55,983 ability rows, all in
  the intended shared-target, recipient/source, repeated-damage, or directly
  nested coordination families. The recovery census is byte-for-byte identical
  to the parent (3,480 structural spans / 64,217 source tokens), and all 31,685
  supported faces round-trip with zero mismatches or render errors.
