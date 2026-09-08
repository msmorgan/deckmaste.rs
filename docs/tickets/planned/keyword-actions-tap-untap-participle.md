---
needs: []
---
**`tap` and `untap` derive a wrong English participle.** Found by
`facts-participle-column-reconciled`: a keyword-action declaration that omits
`grammar: Verb(participle: …)` gets `english_participle`'s derived form
(`crates/deckmaste_construction_core/src/macro_def.rs`, bare form plus `ed`),
which yields `taped` and `untaped` for
`plugins_v2/builtin/macros/keyword_actions/{tap,untap}.ron`. The five
declarations that do declare a participle are exactly the ones whose derived
form would be wrong (`activate`, `cast`, `exile`, `regenerate`, `sacrifice`),
so these two are the remaining holes.

Either declare the participle on both (`tapped`, `untapped`), or teach the
derivation the final-consonant doubling rule and pin `tap`/`untap`; the
derivation already covers `-e` stems only by override, so the first is the
consistent fix unless the rule is wanted generally. Add a test that every
keyword action's effective participle is a real word where the corpus uses
one. Standard constraints apply.
