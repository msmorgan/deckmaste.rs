---
needs: [ability-conferral-without-innate, card-form-characteristics-model, construction-dsl-node, decision-terminology, engine-combatant-role, english-v2-frame-selected-prepositions, english-v2-grammatical-relations, english-v2-person-number-agreement, english-v2-targeting-marker, semantic-query-domains, type-def-permanent-type-flag]
---
**Adopt the candidate root [context map](../../../CONTEXT-MAP.md),
[Game Model glossary](../../contexts/game-model/CONTEXT.md), and
[Oracle English glossary](../../contexts/oracle-english/CONTEXT.md) after the
nomenclature correction family lands.** This is the conformance and activation
ticket, not another compatibility pass.

Audit public names and explanatory prose across semantic, core, engine, Idris,
construction, English-v2, data, RON, generated fixtures, and contributor docs.
The semantic, core, engine, and Idris ASTs should use the same Game Model term
for the same concept; Oracle English keeps standard linguistic terms even where
a spelling such as Object or Predicate legitimately means something different
in the Game Model. Fix remaining contradictory aliases, stale comments, and
generated vocabulary. Update the owning context's `CONTEXT.md` only where
implementation evidence exposed a real flaw in a definition, not to accommodate
an old identifier.

Only after that audit, add the stable instruction in `CLAUDE.md` to read the
root `CONTEXT-MAP.md` before terminology-sensitive repository work, follow it
to each relevant glossary, and keep those documents current through the
domain-modeling workflow. Until this ticket lands, the map and glossaries
remain unactivated proposals referenced by this ticket family only.
