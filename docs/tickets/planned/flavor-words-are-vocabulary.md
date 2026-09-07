---
needs: []
---
**The v2 reader does not register flavor words as macros.** Ruling (user,
2026-09-07). `plugins_v2/builtin/macros/flavor_words/` holds 630 bodyless
declarations whose only content is a spelling and a `FixedTerm` grammar;
they are english_v2's recognition vocabulary and stay exactly as they are.
Editing english_v2 is out of scope. The semantics never invokes one: a
flavor word is an open slot, `flavorWord (word : FlavorWordLabel)
(ability : Ability)` in `Macros.lean`, ported as the builtin `flavorWord`
helper, so a card writes `flavorWord("Kowabunga", ability)` today with
the label as a string.

Scope, semantics_v2 only: the v2 reader (`deckmaste_semantics_v2::ron`,
`reader.rs`) skips the `flavor_words` family so no flavor-word declaration
enters the macro table or the corpus count; the `FlavorWord` meta and
construction_core's reading of the family are untouched. The corpus test
states the new count. Whether english_v2 later replaces the per-word list
with an open-slot term is that lane's decision and is not scheduled here.
Standard constraints apply.
