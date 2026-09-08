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

## Landing record (2026-09-08)

The ticket's premise was short. The derivation `bare + "ed"` was wrong for
every `-e` stem, not just the doubling ones: `assembleed`, `conniveed`,
`createed`, `doubleed`, `endureed`, `exchangeed`, `exploreed`, `forageed`,
`incubateed`, `investigateed`, `populateed`, `proliferateed`, `shuffleed`,
`tripleed`, `voteed`, plus `collect evidenceed` and `face a villainous
choiceed`. The five declared overrides (`activate`, `exile`, `regenerate`,
`sacrifice`, `cast`) hid the rule's failure on the stems someone had noticed.

**PROVE.** `english_participle` now applies the `-e` rule (`d` after a final
`e`). The `RedundantOverride` validation then refused the four `-e` overrides
that became equal to the derived surface, so they are removed from
`activate`, `exile`, `regenerate` and `sacrifice` (and from the same four
synthetic fixtures in `english_v2`'s `predicate_grammar` and the `frindle`
fixture in `open_declaration_verbs`); `cast` keeps its irregular override.
`tap` and `untap` declare `participle: Unavailable`: english_v2's lexical
ownership gives `tapped` to the vocabulary literal `Status::Tapped`, and
declaring the string is refused as a `LiteralLexiconCollision`, which is why
neither had ever declared one. Nothing is lost: no consumer had a participle
for those two before either (it was `taped`).

**DISCLOSE.** Added
`keyword_actions_with_doubling_stems_declare_their_participle`
(construction_core, `builtin_v2_keyword_actions`): a one-syllable stem in a
single vowel and consonant whose participle is still the derived form fails.
Assurance: restored 0, re-spelled 0, ignored 0, added 1, removed 0.
Deviation: the fix is the derivation rule rather than two declared strings,
because the test written for the ticket exposed the `-e` failures.

**REPORT.** Gate: `cargo test -p deckmaste_construction_core -p
deckmaste_construction -p deckmaste_english_v2 -p deckmaste_lexical_source
-p deckmaste_semantics_v2 -p xtask` green; clippy clean on
construction_core and english_v2; cite check 0 non-compliant, 0 stale, no
citations changed. The Lean checker's `ActFacts.participle` column is
untouched (`facts-participle-column-reconciled` established it as the
checker's own fact).
