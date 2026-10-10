---
needs: []
design: true
---
**Write the four rules-defined subtype conferrals in v2 syntax.** Four subtype
declarations carry an EMPTY `rules` list because the syntax to write their type
rules does not exist yet. `plugins-v2-subtypes-macro-only` derived every subtype
declaration's `Definition::Subtype(subtype:, rules:)` body from its meta and
converted the corpus, so the `rules` list is the only thing left: this ticket
teaches the syntax the three gaps below need, then fills the four lists. Each
declaration keeps its v1 core type-rule record verbatim as a comment under its
STOP, which is what those lists must reproduce in v2 terms. Standard
constraints apply.

## The four records, and where they are

| declaration | file |
| --- | --- |
| `equipment` | `plugins_v2/builtin/macros/subtypes/artifact/equipment.ron` |
| `fortification` | `plugins_v2/builtin/macros/subtypes/artifact/fortification.ron` |
| `aura` | `plugins_v2/builtin/macros/subtypes/enchantment/aura.ron` |
| `saga` | `plugins_v2/builtin/macros/subtypes/enchantment/saga.ron` |

Each is pinned by
`crates/deckmaste_construction_core/tests/builtin_v2_noncreature_subtypes.rs::rules_defined_conferrals_stay_on_their_subtype_declarations`,
which asserts the preserved record text and the derived body; the guard
`crates/xtask/src/facts/lean.rs::subtype_definitions_read` carries an empty
exception list, so nothing may go back to a non-`Definition` body.

## The three syntax gaps

Recorded verbatim by `semantics-v2-definition-bodies` (2026-09-08):

| declaration | what it needs |
| --- | --- |
| `equipment`, `fortification` | the [CR#301.5,301.6] host rule is a deontic over the `Attach` deed [CR#701.3a], whose `actFacts` row declares no patient role, so "…to a creature" has no role to bind |
| `aura` | [CR#704.5m] fires on "attached to an illegal object or player, OR is not attached"; `Predicate.attachment` spells only the second half |
| `saga` | [CR#714.4] reads the final chapter number off the Saga's own chapter abilities [CR#714.2d]; `Amount` has no aggregate over a card's chapter marks |
